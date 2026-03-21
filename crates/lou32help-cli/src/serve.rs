use anyhow::{Context, Result, anyhow};
use lou32help_core::CONTENT_SECURITY_POLICY;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Method, Response, ResponseBox, Server, StatusCode};
use tracing::{error, info, warn};

pub fn serve_preview(site_dir: &Path, port: u16) -> Result<()> {
    let site_dir = fs::canonicalize(site_dir)
        .with_context(|| format!("failed to canonicalize {}", site_dir.display()))?;
    let address = format!("127.0.0.1:{port}");
    let server =
        Server::http(&address).map_err(|error| anyhow!("failed to bind {address}: {error}"))?;

    println!("Serving {} at http://{address}", site_dir.display());
    for request in server.incoming_requests() {
        let method = request.method().clone();
        let url = request.url().to_string();

        let prepared = match prepare_response(&site_dir, &method, &url) {
            Ok(prepared) => prepared,
            Err(error) => {
                error!(method = %method.as_str(), url, error = %format!("{error:#}"), "failed to prepare preview response");
                PreparedResponse::text(StatusCode(500), "Internal Server Error", false, false)
            }
        };
        let status = prepared.status.0;
        let response = prepared.into_response();

        match request.respond(response) {
            Ok(()) => info!(method = %method.as_str(), url, status, "served preview request"),
            Err(error) => warn!(
                method = %method.as_str(),
                url,
                status,
                error = %error,
                "failed to send preview response"
            ),
        }
    }

    Ok(())
}

#[derive(Debug)]
struct PreparedResponse {
    status: StatusCode,
    body: Vec<u8>,
    headers: Vec<Header>,
    head_only: bool,
}

impl PreparedResponse {
    fn text(status: StatusCode, body: &str, head_only: bool, allow_header: bool) -> Self {
        let mut headers = security_headers();
        headers.push(
            Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=utf-8"[..])
                .expect("valid content type header"),
        );
        if allow_header {
            headers.push(
                Header::from_bytes(&b"Allow"[..], &b"GET, HEAD"[..]).expect("valid allow header"),
            );
        }

        Self {
            status,
            body: body.as_bytes().to_vec(),
            headers,
            head_only,
        }
    }

    fn file(status: StatusCode, body: Vec<u8>, content_type: Header, head_only: bool) -> Self {
        let mut headers = security_headers();
        headers.push(content_type);

        Self {
            status,
            body,
            headers,
            head_only,
        }
    }

    fn into_response(self) -> ResponseBox {
        let mut response = if self.head_only {
            Response::empty(self.status).boxed()
        } else {
            Response::from_data(self.body)
                .with_status_code(self.status)
                .boxed()
        };

        for header in self.headers {
            response = response.with_header(header);
        }

        response
    }
}

fn prepare_response(site_dir: &Path, method: &Method, url: &str) -> Result<PreparedResponse> {
    let head_only = matches!(method, Method::Head);
    if !matches!(method, Method::Get | Method::Head) {
        return Ok(PreparedResponse::text(
            StatusCode(405),
            "Method Not Allowed",
            false,
            true,
        ));
    }

    let request_path = url.split('?').next().unwrap_or("/");
    let candidate = match resolve_request_path(site_dir, request_path) {
        Ok(candidate) => candidate,
        Err(_) => {
            return Ok(PreparedResponse::text(
                StatusCode(400),
                "Bad Request",
                head_only,
                false,
            ));
        }
    };

    if !(candidate.exists() && candidate.is_file()) {
        return Ok(PreparedResponse::text(
            StatusCode(404),
            "Not Found",
            head_only,
            false,
        ));
    }

    let content_type = content_type_header(&candidate).unwrap_or_else(|| {
        Header::from_bytes(&b"Content-Type"[..], &b"application/octet-stream"[..])
            .expect("valid default content type")
    });
    let bytes =
        fs::read(&candidate).with_context(|| format!("failed to read {}", candidate.display()))?;

    Ok(PreparedResponse::file(
        StatusCode(200),
        bytes,
        content_type,
        head_only,
    ))
}

fn resolve_request_path(site_dir: &Path, request_path: &str) -> Result<PathBuf> {
    let decoded = decode_request_path(request_path)?;
    if decoded.contains('\\') {
        return Err(bad_path("mixed path separators are not allowed"));
    }

    let trimmed = decoded.trim_start_matches('/');
    let mut segments = Vec::new();
    for segment in trimmed.split('/') {
        if segment.is_empty() {
            continue;
        }
        if segment == "." || segment == ".." {
            return Err(bad_path("relative path segments are not allowed"));
        }
        segments.push(segment);
    }

    let mut candidate = site_dir.to_path_buf();
    if segments.is_empty() {
        candidate.push("index.html");
    } else {
        for segment in &segments {
            candidate.push(segment);
        }
        let wants_directory = request_path.ends_with('/');
        if wants_directory || candidate.extension().is_none() {
            candidate.push("index.html");
        }
    }

    if candidate.exists() {
        let canonical = fs::canonicalize(&candidate)
            .with_context(|| format!("failed to canonicalize {}", candidate.display()))?;
        if !canonical.starts_with(site_dir) {
            return Err(bad_path("resolved path escaped the site root"));
        }
        Ok(canonical)
    } else {
        Ok(candidate)
    }
}

fn decode_request_path(path: &str) -> Result<String> {
    let bytes = path.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(bad_path("incomplete percent encoding"));
            }

            let high = decode_hex(bytes[index + 1])?;
            let low = decode_hex(bytes[index + 2])?;
            decoded.push((high << 4) | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }

    String::from_utf8(decoded).map_err(|_| anyhow!("request path is not valid UTF-8"))
}

fn decode_hex(value: u8) -> Result<u8> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(bad_path("invalid percent encoding")),
    }
}

fn bad_path(message: &str) -> anyhow::Error {
    anyhow!(message.to_string())
}

fn security_headers() -> Vec<Header> {
    vec![
        Header::from_bytes(
            &b"Content-Security-Policy"[..],
            CONTENT_SECURITY_POLICY.as_bytes(),
        )
        .expect("valid csp header"),
        Header::from_bytes(&b"X-Content-Type-Options"[..], &b"nosniff"[..])
            .expect("valid nosniff header"),
    ]
}

fn content_type_header(path: &Path) -> Option<Header> {
    let value = match path.extension().and_then(|ext| ext.to_str())? {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    };
    Header::from_bytes(&b"Content-Type"[..], value.as_bytes()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn site_fixture() -> TempDir {
        let temp = TempDir::new().expect("tempdir");
        fs::create_dir_all(temp.path().join("docs/page")).expect("site dirs");
        fs::write(temp.path().join("index.html"), "home").expect("home");
        fs::write(temp.path().join("docs/page/index.html"), "page").expect("page");
        temp
    }

    #[test]
    fn resolves_extensionless_routes() {
        let temp = site_fixture();
        let root = fs::canonicalize(temp.path()).expect("root");
        let resolved = resolve_request_path(&root, "/docs/page").expect("resolved path");

        assert_eq!(resolved, root.join("docs/page/index.html"));
    }

    #[test]
    fn rejects_percent_encoded_traversal() {
        let temp = site_fixture();
        let root = fs::canonicalize(temp.path()).expect("root");
        assert!(resolve_request_path(&root, "/%2e%2e/Cargo.toml").is_err());
    }

    #[test]
    fn rejects_mixed_separators() {
        let temp = site_fixture();
        let root = fs::canonicalize(temp.path()).expect("root");
        assert!(resolve_request_path(&root, "/docs\\page").is_err());
    }

    #[test]
    fn rejects_invalid_percent_encoding() {
        let temp = site_fixture();
        let root = fs::canonicalize(temp.path()).expect("root");
        assert!(resolve_request_path(&root, "/docs/%zz").is_err());
    }

    #[test]
    fn rejects_unsupported_methods() {
        let temp = site_fixture();
        let root = fs::canonicalize(temp.path()).expect("root");
        let prepared = prepare_response(&root, &Method::Post, "/").expect("prepared");

        assert_eq!(prepared.status, StatusCode(405));
    }

    #[test]
    fn adds_security_headers() {
        let headers = security_headers();
        assert!(
            headers
                .iter()
                .any(|header| header.field.equiv("Content-Security-Policy"))
        );
        assert!(
            headers
                .iter()
                .any(|header| header.field.equiv("X-Content-Type-Options"))
        );
    }
}

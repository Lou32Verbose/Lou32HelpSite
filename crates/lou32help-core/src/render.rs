use ammonia::Builder;
use pulldown_cmark::{Options, Parser, html};

/// Content Security Policy header value used in generated HTML pages.
pub const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'; \
     img-src 'self' data:; object-src 'none'; script-src 'self'; style-src 'self'";

/// Render Markdown to sanitized HTML.
pub fn markdown_to_html(body: &str) -> String {
    let parser = Parser::new_ext(body, Options::all());
    let mut output = String::new();
    html::push_html(&mut output, parser);

    Builder::default().clean(&output).to_string()
}

/// Render Markdown to plain text suitable for terminal display.
pub fn markdown_to_terminal(body: &str) -> String {
    let mut output = String::new();

    for line in body.lines() {
        let trimmed = line.trim_start();
        let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
        if hashes > 0 {
            let heading = trimmed[hashes..].trim();
            if !heading.is_empty() {
                output.push_str(&heading.to_uppercase());
                output.push('\n');
                output.push_str(&"-".repeat(heading.len().min(80)));
                output.push_str("\n\n");
                continue;
            }
        }
        output.push_str(line);
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::markdown_to_html;

    #[test]
    fn sanitizes_script_tags_and_dangerous_links() {
        let rendered = markdown_to_html(
            r#"
<script>alert("x")</script>

<a href="javascript:alert('x')" onclick="alert('x')">bad</a>
"#,
        );

        assert!(!rendered.contains("<script"));
        assert!(!rendered.contains("javascript:"));
        assert!(!rendered.contains("onclick="));
    }

    #[test]
    fn keeps_normal_markdown_output() {
        let rendered = markdown_to_html(
            r#"
## Heading

[Safe link](/docs/)

```powershell
Write-Host "ok"
```
"#,
        );

        assert!(rendered.contains("<h2>Heading</h2>"));
        assert!(rendered.contains(r#"<a href="/docs/""#));
        assert!(rendered.contains(">Safe link</a>"));
        assert!(rendered.contains("<pre><code"));
    }
}

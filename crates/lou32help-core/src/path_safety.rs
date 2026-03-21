use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

/// A route or filesystem safety issue discovered during validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSafetyIssue {
    /// Machine-readable issue code.
    pub code: &'static str,
    /// Human-readable description.
    pub message: String,
}

impl PathSafetyIssue {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Supported route kinds for metadata validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutePathKind {
    /// A document slug such as `/powershell/networking/bits-transfer/`.
    Slug,
    /// A topic path such as `powershell/networking`.
    Topic,
    /// A related slug entry such as `/cli-tools/wget/recursive-download/`.
    RelatedSlug,
    /// A tag that becomes `/tags/<tag>/`.
    Tag,
    /// A top-level topic key from configuration.
    TopicKey,
}

/// Inspect a metadata value that influences generated routes.
pub fn inspect_route_path(value: &str, kind: RoutePathKind) -> Vec<PathSafetyIssue> {
    let mut issues = Vec::new();
    let label = match kind {
        RoutePathKind::Slug => "slug",
        RoutePathKind::Topic => "topic",
        RoutePathKind::RelatedSlug => "related slug",
        RoutePathKind::Tag => "tag",
        RoutePathKind::TopicKey => "topic key",
    };

    if value.contains('\0') {
        issues.push(PathSafetyIssue::new(
            route_code(kind),
            format!("{label} '{value}' contains a NUL byte"),
        ));
    }

    if value.contains('\\') {
        issues.push(PathSafetyIssue::new(
            route_code(kind),
            format!("{label} '{value}' contains backslashes"),
        ));
    }

    match kind {
        RoutePathKind::Slug | RoutePathKind::RelatedSlug => {
            if !value.starts_with('/') || !value.ends_with('/') {
                issues.push(PathSafetyIssue::new(
                    route_code(kind),
                    format!("{label} '{value}' must start and end with '/'"),
                ));
            }
            inspect_segments(
                value.trim_matches('/').split('/'),
                value,
                label,
                route_code(kind),
                &mut issues,
            );
        }
        RoutePathKind::Topic => {
            if value.starts_with('/') || value.ends_with('/') {
                issues.push(PathSafetyIssue::new(
                    route_code(kind),
                    format!("{label} '{value}' must not start or end with '/'"),
                ));
            }
            inspect_segments(
                value.split('/'),
                value,
                label,
                route_code(kind),
                &mut issues,
            );
        }
        RoutePathKind::Tag | RoutePathKind::TopicKey => {
            if value.contains('/') {
                issues.push(PathSafetyIssue::new(
                    route_code(kind),
                    format!("{label} '{value}' must be a single route segment"),
                ));
            }
            inspect_segment(value, value, label, route_code(kind), &mut issues);
        }
    }

    dedupe_issues(issues)
}

/// Ensure a relative output path cannot escape the site root.
pub fn validate_output_relative_path(path: &Path) -> Result<(), PathSafetyIssue> {
    if path.as_os_str().is_empty() {
        return Err(PathSafetyIssue::new(
            "unsafe-output-path",
            "output path must not be empty",
        ));
    }

    let mut rebuilt = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                validate_output_component(part)?;
                rebuilt.push(part);
            }
            Component::CurDir => {
                return Err(PathSafetyIssue::new(
                    "unsafe-output-path",
                    format!("output path '{}' contains '.'", path.display()),
                ));
            }
            Component::ParentDir => {
                return Err(PathSafetyIssue::new(
                    "unsafe-output-path",
                    format!("output path '{}' contains '..'", path.display()),
                ));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(PathSafetyIssue::new(
                    "unsafe-output-path",
                    format!("output path '{}' must be relative", path.display()),
                ));
            }
        }
    }

    if rebuilt.as_os_str().is_empty() {
        return Err(PathSafetyIssue::new(
            "unsafe-output-path",
            format!("output path '{}' produced no safe segments", path.display()),
        ));
    }

    Ok(())
}

fn inspect_segments<'a>(
    segments: impl Iterator<Item = &'a str>,
    raw_value: &str,
    label: &str,
    code: &'static str,
    issues: &mut Vec<PathSafetyIssue>,
) {
    let mut saw_segment = false;
    for segment in segments {
        saw_segment = true;
        inspect_segment(segment, raw_value, label, code, issues);
    }

    if !saw_segment {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' must contain at least one segment"),
        ));
    }
}

fn inspect_segment(
    segment: &str,
    raw_value: &str,
    label: &str,
    code: &'static str,
    issues: &mut Vec<PathSafetyIssue>,
) {
    if segment.is_empty() {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' contains an empty path segment"),
        ));
        return;
    }

    if segment == "." || segment == ".." {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' contains forbidden segment '{segment}'"),
        ));
    }

    if is_drive_letter_segment(segment) {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' contains drive-letter segment '{segment}'"),
        ));
    }

    if segment.ends_with(' ') || segment.ends_with('.') {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' contains segment '{segment}' with trailing dot/space"),
        ));
    }

    if segment
        .chars()
        .any(|ch| matches!(ch, '<' | '>' | ':' | '"' | '|' | '?' | '*'))
    {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' contains Windows-reserved filename characters"),
        ));
    }

    if is_windows_reserved_name(segment) {
        issues.push(PathSafetyIssue::new(
            code,
            format!("{label} '{raw_value}' contains reserved Windows name '{segment}'"),
        ));
    }
}

fn validate_output_component(part: &OsStr) -> Result<(), PathSafetyIssue> {
    let value = part.to_string_lossy();
    let mut issues = Vec::new();
    inspect_segment(
        &value,
        &value,
        "output path",
        "unsafe-output-path",
        &mut issues,
    );
    issues.into_iter().next().map_or(Ok(()), Err)
}

fn route_code(kind: RoutePathKind) -> &'static str {
    match kind {
        RoutePathKind::Slug => "invalid-slug-path",
        RoutePathKind::Topic => "invalid-topic-path",
        RoutePathKind::RelatedSlug => "invalid-related-path",
        RoutePathKind::Tag => "invalid-tag-path",
        RoutePathKind::TopicKey => "invalid-topic-key",
    }
}

fn is_drive_letter_segment(segment: &str) -> bool {
    segment.len() == 2
        && segment.as_bytes()[0].is_ascii_alphabetic()
        && segment.as_bytes()[1] == b':'
}

fn is_windows_reserved_name(segment: &str) -> bool {
    let stem = segment.split('.').next().unwrap_or(segment);
    matches!(
        stem.to_ascii_uppercase().as_str(),
        "CON"
            | "PRN"
            | "AUX"
            | "NUL"
            | "COM1"
            | "COM2"
            | "COM3"
            | "COM4"
            | "COM5"
            | "COM6"
            | "COM7"
            | "COM8"
            | "COM9"
            | "LPT1"
            | "LPT2"
            | "LPT3"
            | "LPT4"
            | "LPT5"
            | "LPT6"
            | "LPT7"
            | "LPT8"
            | "LPT9"
    )
}

fn dedupe_issues(issues: Vec<PathSafetyIssue>) -> Vec<PathSafetyIssue> {
    let mut deduped = Vec::new();
    for issue in issues {
        if !deduped.iter().any(|existing| existing == &issue) {
            deduped.push(issue);
        }
    }
    deduped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_invalid_slug_segments() {
        let issues = inspect_route_path("/powershell/../bits./", RoutePathKind::Slug);
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("forbidden segment"))
        );
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("trailing dot/space"))
        );
    }

    #[test]
    fn rejects_backslashes_and_reserved_names() {
        let issues = inspect_route_path(r"powershell\con", RoutePathKind::Topic);
        assert!(
            issues
                .iter()
                .any(|issue| issue.message.contains("backslashes"))
        );

        let reserved_name_issues = inspect_route_path("con", RoutePathKind::Tag);
        assert!(
            reserved_name_issues
                .iter()
                .any(|issue| issue.message.contains("reserved Windows name"))
        );
    }

    #[test]
    fn output_path_rejects_escape_components() {
        let issue = validate_output_relative_path(Path::new("tags/../index.html")).unwrap_err();
        assert_eq!(issue.code, "unsafe-output-path");
    }
}

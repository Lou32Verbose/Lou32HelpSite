use anyhow::{Context, Result, anyhow, bail};
use chrono::NaiveDate;
use pulldown_cmark::{Event, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

/// The structural type of a document page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PageType {
    /// A reference page describing a tool, command, or concept.
    Reference,
    /// A step-by-step recipe for accomplishing a task.
    Recipe,
    /// A troubleshooting guide for diagnosing and fixing problems.
    Troubleshooting,
    /// A reusable template with placeholder variables.
    Template,
}

impl PageType {
    /// Return the lowercase string representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Recipe => "recipe",
            Self::Troubleshooting => "troubleshooting",
            Self::Template => "template",
        }
    }
}

impl Display for PageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for PageType {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "reference" => Ok(Self::Reference),
            "recipe" => Ok(Self::Recipe),
            "troubleshooting" => Ok(Self::Troubleshooting),
            "template" => Ok(Self::Template),
            other => bail!("unsupported page type '{other}'"),
        }
    }
}

/// Publication status of a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocStatus {
    /// Not yet ready for public builds.
    Draft,
    /// Included in public site builds.
    Published,
}

impl Display for DocStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => f.write_str("draft"),
            Self::Published => f.write_str("published"),
        }
    }
}

/// YAML frontmatter fields parsed from a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Human-readable title.
    pub title: String,
    /// Canonical URL slug (e.g. `/powershell/networking/bits-transfer/`).
    pub slug: String,
    /// One-line summary of the document.
    pub summary: String,
    /// Topic path (e.g. `powershell/networking`).
    pub topic: String,
    /// Structural page type.
    #[serde(rename = "type")]
    pub page_type: PageType,
    /// Freeform tags for cross-cutting categorization.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Alternative names that resolve to this document.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Platforms this document applies to.
    #[serde(default)]
    pub platforms: Vec<String>,
    /// Slugs of explicitly related documents.
    #[serde(default)]
    pub related: Vec<String>,
    /// Publication status.
    pub status: DocStatus,
    /// Date the document was last updated.
    pub updated: NaiveDate,
}

/// A parsed Markdown document with frontmatter metadata.
#[derive(Debug, Clone)]
pub struct Document {
    /// Path to the source `.md` file.
    pub source_path: PathBuf,
    /// Parsed YAML frontmatter.
    pub metadata: DocumentMetadata,
    /// Markdown body content (everything after the closing `---`).
    pub body: String,
    /// Heading texts extracted from the body.
    pub headings: Vec<String>,
}

impl Document {
    /// Parse a Markdown file with YAML frontmatter into a `Document`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read document {}", path.display()))?;
        let normalized = raw.replace("\r\n", "\n");

        let stripped = normalized
            .strip_prefix("---\n")
            .ok_or_else(|| anyhow!("document {} is missing YAML frontmatter", path.display()))?;
        let marker = "\n---\n";
        let Some(frontmatter_end) = stripped.find(marker) else {
            bail!(
                "document {} is missing the closing YAML frontmatter fence",
                path.display()
            );
        };

        let frontmatter_raw = &stripped[..frontmatter_end];
        let body = stripped[frontmatter_end + marker.len()..]
            .trim()
            .to_string();
        if body.is_empty() {
            bail!("document {} has no body content", path.display());
        }

        let mut metadata: DocumentMetadata = serde_norway::from_str(frontmatter_raw)
            .with_context(|| format!("failed to parse frontmatter in {}", path.display()))?;
        normalize_metadata(&mut metadata);
        let headings = extract_headings(&body);

        Ok(Self {
            source_path: path.to_path_buf(),
            metadata,
            body,
            headings,
        })
    }

    /// Returns `true` if this document has `status: published`.
    pub fn is_published(&self) -> bool {
        self.metadata.status == DocStatus::Published
    }

    /// Return the first segment of the topic path (the top-level topic key).
    pub fn top_level_topic(&self) -> &str {
        self.metadata.topic.split('/').next().unwrap_or_default()
    }
}

/// Normalize a slug to lowercase with leading and trailing slashes.
pub fn normalize_slug(input: &str) -> String {
    let lowered = input.trim().replace('\\', "/").to_ascii_lowercase();
    let mut value = lowered.trim_matches('/').to_string();
    if value.is_empty() {
        return "/".to_string();
    }
    value.insert(0, '/');
    value.push('/');
    value
}

/// Normalize a topic path to lowercase without leading/trailing slashes.
pub fn normalize_topic(input: &str) -> String {
    input
        .trim()
        .replace('\\', "/")
        .trim_matches('/')
        .to_ascii_lowercase()
}

/// Normalize an alias to trimmed lowercase.
pub fn normalize_alias(input: &str) -> String {
    input.trim().to_ascii_lowercase()
}

/// Return the expected heading sections for a given page type.
pub fn scaffold_sections(page_type: PageType) -> &'static [&'static str] {
    match page_type {
        PageType::Reference => &[
            "Synopsis",
            "Syntax",
            "Parameters/Flags",
            "Examples",
            "Related",
        ],
        PageType::Recipe => &[
            "Goal",
            "Prerequisites",
            "Steps",
            "Commands",
            "Verification",
            "Related",
        ],
        PageType::Troubleshooting => {
            &["Symptoms", "Cause", "Resolution", "Verification", "Related"]
        }
        PageType::Template => &["Use Case", "Template", "Variables", "Examples", "Related"],
    }
}

/// Extract heading texts from Markdown body content.
pub fn extract_headings(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let hashes = trimmed.chars().take_while(|ch| *ch == '#').count();
            if hashes == 0 {
                return None;
            }

            let rest = trimmed[hashes..].trim_start();
            if rest.is_empty() {
                return None;
            }
            Some(rest.to_string())
        })
        .collect()
}

/// Check whether Markdown body contains raw HTML elements.
pub fn contains_raw_html(body: &str) -> bool {
    Parser::new_ext(body, Options::all())
        .any(|event| matches!(event, Event::Html(_) | Event::InlineHtml(_)))
}

/// Derive a human-readable title from the last segment of a slug.
pub fn title_from_slug_leaf(value: &str) -> String {
    value
        .trim_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or(value)
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert a slug to a relative filesystem path ending in `index.html`.
pub fn slug_to_output_path(slug: &str) -> PathBuf {
    let cleaned = slug.trim_matches('/');
    if cleaned.is_empty() {
        return PathBuf::from("index.html");
    }
    let mut path = PathBuf::from(cleaned);
    path.push("index.html");
    path
}

fn normalize_metadata(metadata: &mut DocumentMetadata) {
    metadata.slug = normalize_slug(&metadata.slug);
    metadata.topic = normalize_topic(&metadata.topic);
    metadata.tags = metadata
        .tags
        .iter()
        .map(|value| normalize_alias(value))
        .collect();
    metadata.aliases = metadata
        .aliases
        .iter()
        .map(|value| normalize_alias(value))
        .collect();
    metadata.platforms = metadata
        .platforms
        .iter()
        .map(|value| normalize_alias(value))
        .collect();
    metadata.related = metadata
        .related
        .iter()
        .map(|value| normalize_slug(value))
        .collect();
    metadata.summary = metadata.summary.trim().to_string();
    metadata.title = metadata.title.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_frontmatter_fence() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("bad.md");
        fs::write(&path, "no frontmatter here\n\n## Body\n\nContent.").unwrap();
        let err = Document::from_file(&path).unwrap_err();
        assert!(
            format!("{err:#}").contains("missing YAML frontmatter"),
            "expected missing frontmatter error, got: {err:#}"
        );
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("unclosed.md");
        fs::write(&path, "---\ntitle: Test\nslug: /test/\n").unwrap();
        let err = Document::from_file(&path).unwrap_err();
        assert!(
            format!("{err:#}").contains("closing YAML frontmatter fence"),
            "expected unclosed frontmatter error, got: {err:#}"
        );
    }

    #[test]
    fn rejects_malformed_yaml() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("bad_yaml.md");
        fs::write(
            &path,
            "---\n: : : not valid yaml\n---\n\n## Body\n\nContent.",
        )
        .unwrap();
        let err = Document::from_file(&path).unwrap_err();
        assert!(
            format!("{err:#}").contains("failed to parse frontmatter"),
            "expected parse error, got: {err:#}"
        );
    }

    #[test]
    fn rejects_invalid_page_type() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("bad_type.md");
        fs::write(
            &path,
            "---\ntitle: Test\nslug: /test/\nsummary: s\ntopic: t\ntype: foobar\ntags: []\naliases: []\nplatforms: []\nrelated: []\nstatus: published\nupdated: 2026-03-20\n---\n\n## Body\n\nContent.",
        )
        .unwrap();
        let err = Document::from_file(&path).unwrap_err();
        assert!(
            format!("{err:#}").contains("failed to parse frontmatter"),
            "expected type error, got: {err:#}"
        );
    }

    #[test]
    fn normalize_slug_edge_cases() {
        assert_eq!(normalize_slug(""), "/");
        assert_eq!(normalize_slug("   "), "/");
        assert_eq!(normalize_slug("foo\\bar"), "/foo/bar/");
        assert_eq!(normalize_slug("/foo/bar/"), "/foo/bar/");
        assert_eq!(normalize_slug("///"), "/");
    }
}

//! Core library for the LOU32HELP knowledge base system.
//!
//! Provides document parsing, workspace loading, search indexing,
//! Markdown rendering, and validation for a terminal and static-site
//! documentation library.

#![warn(missing_docs)]

/// Configuration loading and structure.
pub mod config;
/// Document parsing, metadata, and normalization.
pub mod document;
/// Workspace loading and pre-computed views.
pub mod library;
mod projection;
/// Markdown rendering to HTML and terminal output.
pub mod render;
/// Full-text search index and query engine.
pub mod search;
/// Workspace validation rules.
pub mod validation;

pub use config::{Lou32HelpConfig, PathConfig, SearchConfig, SiteConfig, TopicConfig};
pub use document::{
    DocStatus, Document, DocumentMetadata, PageType, normalize_alias, normalize_slug,
    normalize_topic, scaffold_sections, slug_to_output_path, title_from_slug_leaf,
};
pub use library::{TopicNode, Workspace, WorkspaceView};
pub use render::{CONTENT_SECURITY_POLICY, markdown_to_html, markdown_to_terminal};
pub use search::{SearchIndex, SearchQuery, SearchResult, build_search_index, search_index};
pub use validation::{Severity, ValidationIssue, ValidationMode, validate_workspace};

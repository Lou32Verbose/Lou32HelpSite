use crate::document::{Document, PageType, normalize_alias, normalize_slug, normalize_topic};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A search index containing pre-normalized entries for all visible documents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndex {
    /// Timestamp when the index was generated.
    pub generated_at: String,
    /// Indexed entries, one per document.
    pub entries: Vec<SearchEntry>,
}

/// A single indexed document with pre-normalized search fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntry {
    /// Canonical document slug.
    pub slug: String,
    /// Document title.
    pub title: String,
    /// Document summary.
    pub summary: String,
    /// Topic path.
    pub topic: String,
    /// Page type.
    #[serde(rename = "type")]
    pub page_type: PageType,
    /// Normalized slug for matching.
    pub slug_search: String,
    /// Normalized title for matching.
    pub title_search: String,
    /// Normalized summary for matching.
    pub summary_search: String,
    /// Normalized topic for matching.
    pub topic_search: String,
    /// Normalized tags for matching.
    pub tag_search: Vec<String>,
    /// Normalized aliases for matching.
    pub alias_search: Vec<String>,
    /// Normalized platforms for matching.
    pub platform_search: Vec<String>,
    /// Normalized headings for matching.
    pub heading_search: Vec<String>,
    /// Normalized body text for matching.
    pub body_search: String,
}

/// Parameters for a search query, with optional filters.
#[derive(Debug, Clone, Default)]
pub struct SearchQuery {
    /// The search text.
    pub query: String,
    /// Filter to documents under this topic.
    pub topic: Option<String>,
    /// Filter to this page type.
    pub page_type: Option<PageType>,
    /// Filter to this platform.
    pub platform: Option<String>,
    /// Maximum number of results to return.
    pub max_results: Option<usize>,
}

/// A scored search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Canonical document slug.
    pub slug: String,
    /// Document title.
    pub title: String,
    /// Document summary.
    pub summary: String,
    /// Topic path.
    pub topic: String,
    /// Page type.
    #[serde(rename = "type")]
    pub page_type: PageType,
    /// Relevance score (higher is better).
    pub score: i32,
    /// Which fields the query matched on.
    pub matched_on: Vec<String>,
}

impl SearchQuery {
    /// Return the normalized topic filter, if set.
    pub fn normalized_topic(&self) -> Option<String> {
        self.topic.as_ref().map(|topic| normalize_topic(topic))
    }

    /// Return the normalized platform filter, if set.
    pub fn normalized_platform(&self) -> Option<String> {
        self.platform
            .as_ref()
            .map(|platform| normalize_alias(platform))
    }
}

/// Build a search index from an iterator of documents.
pub fn build_search_index<'a>(docs: impl IntoIterator<Item = &'a Document>) -> SearchIndex {
    let entries = docs
        .into_iter()
        .map(|doc| SearchEntry {
            slug: doc.metadata.slug.clone(),
            title: doc.metadata.title.clone(),
            summary: doc.metadata.summary.clone(),
            topic: doc.metadata.topic.clone(),
            page_type: doc.metadata.page_type,
            slug_search: normalize_slug(&doc.metadata.slug),
            title_search: normalize_text(&doc.metadata.title),
            summary_search: normalize_text(&doc.metadata.summary),
            topic_search: normalize_topic(&doc.metadata.topic),
            tag_search: doc
                .metadata
                .tags
                .iter()
                .map(|value| normalize_alias(value))
                .collect(),
            alias_search: doc
                .metadata
                .aliases
                .iter()
                .map(|value| normalize_alias(value))
                .collect(),
            platform_search: doc
                .metadata
                .platforms
                .iter()
                .map(|value| normalize_alias(value))
                .collect(),
            heading_search: doc
                .headings
                .iter()
                .map(|value| normalize_text(value))
                .collect(),
            body_search: normalize_text(&doc.body),
        })
        .collect();

    SearchIndex {
        generated_at: Utc::now().to_rfc3339(),
        entries,
    }
}

/// Search the index with the given query and return scored results.
pub fn search_index(index: &SearchIndex, query: &SearchQuery) -> Vec<SearchResult> {
    let normalized_query = normalize_text(&query.query);
    if normalized_query.is_empty() {
        return Vec::new();
    }

    let tokens = split_tokens(&normalized_query);
    let normalized_topic = query.normalized_topic();
    let normalized_platform = query.normalized_platform();

    let mut results = index
        .entries
        .iter()
        .filter(|entry| {
            filter_entry(
                entry,
                normalized_topic.as_deref(),
                query.page_type,
                normalized_platform.as_deref(),
            )
        })
        .filter_map(|entry| score_entry(entry, &normalized_query, &tokens))
        .collect::<Vec<_>>();

    results.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.title.cmp(&right.title))
    });

    let limit = query.max_results.unwrap_or(results.len());
    results.truncate(limit);
    results
}

fn filter_entry(
    entry: &SearchEntry,
    topic: Option<&str>,
    page_type: Option<PageType>,
    platform: Option<&str>,
) -> bool {
    let topic_matches = topic
        .map(|topic| {
            entry.topic_search == topic || entry.topic_search.starts_with(&format!("{topic}/"))
        })
        .unwrap_or(true);
    let type_matches = page_type
        .map(|kind| entry.page_type == kind)
        .unwrap_or(true);
    let platform_matches = platform
        .map(|platform| entry.platform_search.iter().any(|value| value == platform))
        .unwrap_or(true);

    topic_matches && type_matches && platform_matches
}

fn score_entry(
    entry: &SearchEntry,
    normalized_query: &str,
    tokens: &[String],
) -> Option<SearchResult> {
    let mut score = 0;
    let mut matched = BTreeSet::new();

    if entry.slug_search == normalize_slug(normalized_query) {
        score += 240;
        matched.insert("slug".to_string());
    } else if entry.slug_search.contains(normalized_query) {
        score += 120;
        matched.insert("slug".to_string());
    }

    if entry
        .alias_search
        .iter()
        .any(|alias| alias == normalized_query)
    {
        score += 220;
        matched.insert("alias".to_string());
    } else if entry
        .alias_search
        .iter()
        .any(|alias| alias.contains(normalized_query))
    {
        score += 110;
        matched.insert("alias".to_string());
    }

    if entry.title_search == normalized_query {
        score += 200;
        matched.insert("title".to_string());
    } else if entry.title_search.contains(normalized_query) {
        score += 140;
        matched.insert("title".to_string());
    }

    if entry.summary_search.contains(normalized_query) {
        score += 80;
        matched.insert("summary".to_string());
    }

    if entry.topic_search == normalized_query
        || entry
            .topic_search
            .starts_with(&format!("{normalized_query}/"))
    {
        score += 75;
        matched.insert("topic".to_string());
    }

    for token in tokens {
        if entry
            .title_search
            .split_whitespace()
            .any(|word| word == token)
        {
            score += 35;
            matched.insert("title".to_string());
        } else if entry.title_search.contains(token) {
            score += 18;
            matched.insert("title".to_string());
        }

        if entry.tag_search.iter().any(|tag| tag == token) {
            score += 28;
            matched.insert("tags".to_string());
        } else if entry.tag_search.iter().any(|tag| tag.contains(token)) {
            score += 15;
            matched.insert("tags".to_string());
        }

        if entry
            .heading_search
            .iter()
            .any(|heading| heading.contains(token))
        {
            score += 16;
            matched.insert("headings".to_string());
        }

        if entry
            .platform_search
            .iter()
            .any(|platform| platform == token)
        {
            score += 14;
            matched.insert("platforms".to_string());
        }

        if entry.body_search.contains(token) {
            score += 8;
            matched.insert("body".to_string());
        }
    }

    if score == 0 {
        return None;
    }

    Some(SearchResult {
        slug: entry.slug.clone(),
        title: entry.title.clone(),
        summary: entry.summary.clone(),
        topic: entry.topic.clone(),
        page_type: entry.page_type,
        score,
        matched_on: matched.into_iter().collect(),
    })
}

/// Normalize text for search: lowercase, keep only ASCII alphanumeric, collapse whitespace.
pub fn normalize_text(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut last_was_space = true;

    for ch in value.chars().flat_map(|ch| ch.to_lowercase()) {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch);
            last_was_space = false;
        } else if !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }
    }

    normalized.trim().to_string()
}

fn split_tokens(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .filter(|token| !token.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_text_strips_non_ascii() {
        assert_eq!(normalize_text("café"), "caf");
        assert_eq!(normalize_text("hello world"), "hello world");
        assert_eq!(normalize_text("test!!!value"), "test value");
    }

    #[test]
    fn search_empty_query_returns_nothing() {
        let index = SearchIndex {
            generated_at: "2026-03-20T00:00:00+00:00".to_string(),
            entries: vec![SearchEntry {
                slug: "/test/".to_string(),
                title: "Test".to_string(),
                summary: "A test".to_string(),
                topic: "test".to_string(),
                page_type: PageType::Reference,
                slug_search: "/test/".to_string(),
                title_search: "test".to_string(),
                summary_search: "a test".to_string(),
                topic_search: "test".to_string(),
                tag_search: vec![],
                alias_search: vec![],
                platform_search: vec![],
                heading_search: vec![],
                body_search: "test body".to_string(),
            }],
        };

        let results = search_index(
            &index,
            &SearchQuery {
                query: "".to_string(),
                ..Default::default()
            },
        );
        assert!(results.is_empty());

        let results = search_index(
            &index,
            &SearchQuery {
                query: "   ".to_string(),
                ..Default::default()
            },
        );
        assert!(results.is_empty());
    }
}

//! WASM module that exposes the LOU32HELP search engine for browser-side use.
//!
//! Compiled to `wasm32-unknown-unknown` and loaded by `search.js` in the
//! generated static site.

use lou32help_core::{SearchIndex, SearchQuery, search_index as core_search_index};
use wasm_bindgen::prelude::*;

/// Run a search query against a JSON search index and return JSON results.
#[wasm_bindgen]
pub fn search_index(
    index_json: &str,
    query: &str,
    topic: &str,
    page_type: &str,
    platform: &str,
    max_results: usize,
) -> String {
    let index: SearchIndex = match serde_json::from_str(index_json) {
        Ok(index) => index,
        Err(error) => {
            return serde_json::json!({
                "error": format!("failed to parse search index: {error}")
            })
            .to_string();
        }
    };

    let results = core_search_index(
        &index,
        &SearchQuery {
            query: query.to_string(),
            topic: (!topic.trim().is_empty()).then(|| topic.to_string()),
            page_type: parse_page_type(page_type),
            platform: (!platform.trim().is_empty()).then(|| platform.to_string()),
            max_results: Some(max_results),
        },
    );

    serde_json::to_string(&results).unwrap_or_else(|error| {
        serde_json::json!({
            "error": format!("failed to serialize search results: {error}")
        })
        .to_string()
    })
}

#[wasm_bindgen(start)]
pub fn start() {}

fn parse_page_type(input: &str) -> Option<lou32help_core::PageType> {
    input.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::search_index;

    #[test]
    fn returns_error_for_invalid_index_json() {
        let payload = search_index("{not json}", "bits", "", "", "", 10);
        assert!(payload.contains("\"error\""));
    }

    #[test]
    fn returns_stable_results_payload() {
        let index_json = r#"{
  "generated_at": "2026-03-20T00:00:00+00:00",
  "entries": [
    {
      "slug": "/powershell/networking/bits-transfer/",
      "title": "Bits Transfer",
      "summary": "Download with BITS",
      "topic": "powershell/networking",
      "type": "recipe",
      "slug_search": "/powershell/networking/bits-transfer/",
      "title_search": "bits transfer",
      "summary_search": "download with bits",
      "topic_search": "powershell/networking",
      "tag_search": ["powershell", "bits"],
      "alias_search": ["start-bitstransfer"],
      "platform_search": ["windows"],
      "heading_search": ["goal", "commands"],
      "body_search": "start bitstransfer"
    }
  ]
}"#;

        let payload = search_index(index_json, "bits", "", "recipe", "windows", 10);

        assert!(payload.contains("\"slug\":\"/powershell/networking/bits-transfer/\""));
        assert!(payload.contains("\"matched_on\""));
    }
}

//! Static site generator for the LOU32HELP knowledge base.
//!
//! Builds a fully static HTML site from a [`lou32help_core::Workspace`],
//! including topic pages, tag pages, search infrastructure, and document pages.

#![warn(missing_docs)]

mod builder;
mod layout;
mod pages;

pub use builder::{BuildReport, build_site, build_site_from_view};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::finalize_html;
    use crate::pages::{render_document_page, render_home_page};
    use lou32help_core::Workspace;
    use lou32help_test_fixtures::write_two_doc_workspace as write_workspace;
    use std::fs;

    fn compact(value: &str) -> String {
        value.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn builds_site_outputs() {
        let temp = write_workspace();
        let workspace = Workspace::load(temp.path()).expect("load workspace");
        let view = workspace.view(false);
        let out_dir = temp.path().join("dist/site");
        let report = build_site_from_view(&view, &out_dir).expect("build site");
        assert!(report.page_count >= 8);
        assert!(out_dir.join("index.html").exists());
        let search_index = out_dir.join("assets/search-index.json");
        assert!(search_index.exists());
        assert!(
            out_dir
                .join("powershell/networking/bits-transfer/index.html")
                .exists()
        );
        let search_index_raw = fs::read_to_string(search_index).expect("search index");
        assert!(!search_index_raw.contains(r#""body":"#));
        assert!(!search_index_raw.contains(r#""headings":"#));

        let home_page = fs::read_to_string(out_dir.join("index.html")).expect("home page");
        assert!(home_page.contains(r#"href="search/index.html""#));
        assert!(home_page.contains(r#"href="topics/index.html""#));
        assert!(!home_page.contains(r#"href="/search/""#));
        assert!(!home_page.contains(r#"src="/assets/search.js""#));

        let doc_page =
            fs::read_to_string(out_dir.join("powershell/networking/bits-transfer/index.html"))
                .expect("doc page");
        assert!(doc_page.contains(r#"href="../../../search/index.html?platform=windows""#));
        assert!(
            doc_page.contains(r#"href="../../../cli-tools/wget/recursive-download/index.html""#)
        );

        assert!(out_dir.join("topics/index.html").exists());
        assert!(out_dir.join("tags/index.html").exists());

        let search_js = fs::read_to_string(out_dir.join("assets/search.js")).expect("search js");
        assert!(
            search_js
                .contains(r#"import init, { search_index } from "./lou32help_web_search.js";"#)
        );
        assert!(search_js.contains(r#"fetch(new URL("./search-index.json", import.meta.url))"#));
    }

    #[test]
    fn home_page_snapshot() {
        let temp = write_workspace();
        let workspace = Workspace::load(temp.path()).expect("load workspace");
        let view = workspace.view(false);
        let rendered = finalize_html("/", render_home_page(&view).into_string());
        let compacted = compact(&rendered);
        assert!(compacted.contains("Browse Topics"));
        assert!(compacted.contains("topics/index.html"));
        assert!(compacted.contains("Content-Security-Policy"));
        insta::assert_snapshot!(compacted);
    }

    #[test]
    fn document_page_snapshot() {
        let temp = write_workspace();
        let workspace = Workspace::load(temp.path()).expect("load workspace");
        let view = workspace.view(false);
        let doc = view
            .find_document("/powershell/networking/bits-transfer/")
            .expect("doc");
        let rendered = finalize_html(
            &doc.metadata.slug,
            render_document_page(&view, doc).into_string(),
        );
        let compacted = compact(&rendered);
        assert!(compacted.contains("Declared"));
        assert!(compacted.contains("See Also"));
        assert!(compacted.contains("canonical"));
        insta::assert_snapshot!(compacted);
    }

    #[test]
    fn rejects_unsafe_output_paths_even_without_prior_validation() {
        let temp = lou32help_test_fixtures::write_workspace(&[(
            "content/powershell/networking/unsafe.md",
            r#"---
title: Unsafe Output
slug: /powershell/../escape/
summary: Should not build
topic: powershell/networking
type: recipe
tags: [ok]
aliases: []
platforms: [windows]
related: []
status: published
updated: 2026-03-20
---

## Goal

Goal text.

## Prerequisites

Need PowerShell.

## Steps

1. Do thing.

## Commands

```powershell
Write-Host "ok"
```

## Verification

Check file exists.

## Related

- Nothing
"#,
        )]);
        let workspace = Workspace::load(temp.path()).expect("load workspace");
        let view = workspace.view(false);
        let out_dir = temp.path().join("dist/site");
        let err = build_site_from_view(&view, &out_dir).unwrap_err();

        assert!(format!("{err:#}").contains("unsafe site output path"));
    }
}

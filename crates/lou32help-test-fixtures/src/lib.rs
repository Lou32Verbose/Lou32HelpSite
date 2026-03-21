use std::fs;
use tempfile::TempDir;

pub const RECIPE_DOC: &str = r#"---
title: Bits Transfer
slug: /powershell/networking/bits-transfer/
summary: Download with bits
topic: powershell/networking
type: recipe
tags: [powershell, bits]
aliases: [start-bitstransfer]
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
Start-BitsTransfer -Source foo -Destination bar
```

## Verification

Check file exists.

## Related

- Nothing
"#;

pub const REFERENCE_DOC: &str = r#"---
title: Wget Recursive Download
slug: /cli-tools/wget/recursive-download/
summary: Mirror a site with wget
topic: cli-tools/wget
type: reference
tags: [wget]
aliases: [wget mirror]
platforms: [windows, linux]
related:
  - /powershell/networking/bits-transfer/
status: published
updated: 2026-03-20
---

## Synopsis

Mirror a site.

## Syntax

```text
wget -m -p -k -np https://example.com/
```

## Parameters/Flags

Flags text.

## Examples

Example text.

## Related

- [`Bits`](/powershell/networking/bits-transfer/)
"#;

pub fn default_config_toml() -> &'static str {
    r#"
[site]
title = "LOU32HELP"
tagline = "tag"
description = "desc"
base_url = "https://example.com"
copyright = "me"

[paths]
content_dir = "content"
site_dir = "dist/site"
assets_dir = "assets"

[search]
min_query_length = 2
max_results = 25
related_limit = 4
featured_limit = 8
wasm_module = "lou32help_web_search"

[[topics]]
key = "powershell"
title = "PowerShell"
description = "PowerShell docs"
order = 10
"#
}

pub fn two_topic_config_toml() -> &'static str {
    r#"
[site]
title = "LOU32HELP"
tagline = "A terminal and site library."
description = "Dense docs."
base_url = "https://example.com"
copyright = "me"

[paths]
content_dir = "content"
site_dir = "dist/site"
assets_dir = "assets"

[search]
min_query_length = 2
max_results = 25
related_limit = 4
featured_limit = 8
wasm_module = "lou32help_web_search"

[[topics]]
key = "powershell"
title = "PowerShell"
description = "PowerShell docs"
order = 10

[[topics]]
key = "cli-tools"
title = "CLI Tools"
description = "CLI docs"
order = 20
"#
}

/// Create a temporary workspace with the given config TOML and document files.
pub fn write_workspace(docs: &[(&str, &str)]) -> TempDir {
    write_workspace_with_config(default_config_toml(), docs)
}

/// Create a temporary workspace with a specific config and document files.
pub fn write_workspace_with_config(config_toml: &str, docs: &[(&str, &str)]) -> TempDir {
    let temp = TempDir::new().expect("tempdir");
    fs::write(temp.path().join("lou32help.toml"), config_toml).expect("write config");

    fs::create_dir_all(temp.path().join("content/powershell/networking")).expect("mkdirs");
    for (path, content) in docs {
        let full = temp.path().join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).expect("mkdir parent");
        }
        fs::write(full, content).expect("write doc");
    }
    temp
}

/// Single-document workspace with the recipe doc.
pub fn write_default_workspace() -> TempDir {
    write_workspace(&[("content/powershell/networking/bits.md", RECIPE_DOC)])
}

/// Recipe doc variant with a cross-reference to the wget doc.
pub const RECIPE_DOC_WITH_RELATED: &str = r#"---
title: Bits Transfer
slug: /powershell/networking/bits-transfer/
summary: Download with bits
topic: powershell/networking
type: recipe
tags: [powershell, bits]
aliases: [start-bitstransfer]
platforms: [windows]
related:
  - /cli-tools/wget/recursive-download/
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
Start-BitsTransfer -Source foo -Destination bar
```

## Verification

Check file exists.

## Related

- [`Wget`](/cli-tools/wget/recursive-download/)
"#;

/// Two-document workspace with cross-references between recipe and reference docs.
pub fn write_two_doc_workspace() -> TempDir {
    write_workspace_with_config(
        two_topic_config_toml(),
        &[
            (
                "content/powershell/networking/bits.md",
                RECIPE_DOC_WITH_RELATED,
            ),
            ("content/cli-tools/wget/recursive.md", REFERENCE_DOC),
        ],
    )
}

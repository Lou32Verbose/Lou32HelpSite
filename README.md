# LOU32HELP

LOU32HELP is a Rust-powered personal knowledge base that turns one set of
Markdown source files into:

- a terminal-first help system with exact lookup and search
- a static HTML reference site with topic indexes, tag indexes, and related docs
- a shared prebuilt search index used by the CLI and browser

## Quick Start

```powershell
cargo run -p lou32help-cli -- check
cargo run -p lou32help-cli -- show /powershell/networking/bits-transfer/
cargo run -p lou32help-cli -- search bits transfer
cargo run -p lou32help-cli -- build
```

## Verification

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p lou32help-cli -- build
pwsh -File scripts/run-audit.ps1
```

Set `RUST_LOG=info` to see workspace load timing, validation summaries, build timing,
and preview-server request logs.

## Layout

- `content/`: canonical Markdown documents with YAML frontmatter
- `docs/`: archived legacy notes kept for reference only and not read by the app
- `migration/`: legacy migration inventory and supporting notes
- `lou32help.toml`: topic registry and site configuration
- `crates/lou32help-core`: shared parsing, validation, indexing, rendering, and cached workspace views
- `crates/lou32help-cli`: terminal commands, build flow, and local preview server
- `crates/lou32help-site`: static HTML generation
- `crates/lou32help-web-search`: Rust-to-WASM browser search bridge

## Legacy Migration

- Run `pwsh -File scripts/generate-legacy-docs-manifest.ps1` to rebuild the full `docs/` inventory.
- Review `migration/legacy-docs-manifest.csv` before moving files out of the archive.
- Keep `docs/` untouched until a source file has an explicit manifest disposition and its replacement page lives under `content/`.

## Security Notes

- Markdown is sanitized before being embedded into generated HTML.
- The preview server only serves files under `dist/site` and only accepts `GET` and `HEAD`.
- Search result cards in the browser are rendered with DOM APIs instead of `innerHTML`.

# Contributing

## Local Setup

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p lou32help-cli -- check
cargo run -p lou32help-cli -- build
pwsh -File scripts/run-audit.ps1
```

Set `RUST_LOG=info` to see validation counts, build timing, browser bundle cache decisions, and search index size logs.

## Content Authoring Rules

- Every Markdown page under `content/` must include valid YAML frontmatter and the required headings for its page type.
- Slugs and related slugs must start and end with `/` and must not contain empty segments, `.`, `..`, backslashes, drive letters, NULs, Windows-reserved filename characters, or trailing dots/spaces.
- Topic paths must not start or end with `/` and must follow the same safe-segment rules.
- Tags become `/tags/<tag>/` routes, so they must be a single safe segment.
- Avoid Windows reserved names such as `con`, `prn`, `aux`, `nul`, `com1`, or `lpt1` in any route-affecting metadata.

## Build Notes

- Browser search assets are cached under `target/lou32help/browser-search/`.
- Warm builds reuse cached browser assets when the bundle fingerprint is unchanged.
- If browser bundling fails, confirm `wasm32-unknown-unknown` is installed via `rustup` and `wasm-bindgen-cli` is available on `PATH`.

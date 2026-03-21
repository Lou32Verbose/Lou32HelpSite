# Architecture

## Workspace Layout

- `crates/lou32help-core`: document parsing, normalization, validation, search indexing, and workspace projection
- `crates/lou32help-site`: static HTML rendering, output-path enforcement, and browser search asset emission
- `crates/lou32help-cli`: command dispatch, validation/build orchestration, preview serving, and cached browser bundling
- `crates/lou32help-web-search`: WASM adapter that runs browser-side search against the browser search index

## Validation Flow

1. `Document::from_file` parses frontmatter and records raw route/path issues before normalization.
2. `Workspace::validate` combines document issues, config topic-key checks, schema/content checks, and link validation.
3. `build` and `serve` call validation before site generation and stop on any error-level issue.
4. `lou32help-site` performs a second output-path safety check before writing any dynamic page.

## Search Flow

- `lou32help-core` builds a full in-memory `SearchIndex` for CLI lookups.
- `lou32help-site` serializes a dedicated `BrowserSearchIndex` with bounded body text into `dist/site/assets/search-index.json`.
- `lou32help-web-search` deserializes the browser index in WASM and applies the same field-aware ranking model used by the core search implementation where those fields still exist.

## Browser Bundle Pipeline

1. `lou32help-site` writes `search.js` with the integrity placeholder and the browser index JSON.
2. `lou32help-cli` fingerprints the bundle inputs and checks `target/lou32help/browser-search/`.
3. On a cache miss, the CLI verifies the wasm target and `wasm-bindgen`, builds `lou32help-web-search`, and stores generated JS/WASM plus manifest metadata under `target/`.
4. On a cache hit, the CLI copies the cached JS/WASM into `dist/site/assets/` and injects the cached SRI hash into `search.js`.

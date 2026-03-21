# Operations

## Production Build Prerequisites

- Rust toolchain with the workspace `rust-version`
- `wasm32-unknown-unknown` target installed
- `wasm-bindgen-cli` installed and available on `PATH`

## Standard Verification Sequence

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p lou32help-cli -- check
cargo run -p lou32help-cli -- build
pwsh -File scripts/run-audit.ps1
```

## Browser Bundle Recovery

- If the build reports a missing wasm target, run `rustup target add wasm32-unknown-unknown`.
- If the build reports a missing `wasm-bindgen`, run `cargo install wasm-bindgen-cli`.
- If cached browser assets appear stale or corrupted, delete `target/lou32help/browser-search/` and rebuild.
- The browser bundle cache is disposable and should never be checked into source control.

## Release Expectations

- `cargo run -p lou32help-cli -- check` must be clean against repo content before release.
- A successful build must emit `dist/site/assets/search-index.json`, `<module>.js`, and `<module>_bg.wasm`.
- Browser search should load without console errors and route-safe validation should reject unsafe metadata before content ships.

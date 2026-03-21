use anyhow::{Context, Result, anyhow, bail};
use base64::Engine;
use sha2::{Digest, Sha256, Sha384};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tracing::info;
use walkdir::WalkDir;

const WASM_TARGET: &str = "wasm32-unknown-unknown";

#[derive(Debug, Clone)]
pub struct BundleReport {
    pub cache_hit: bool,
    pub asset_bytes: u64,
}

#[derive(Debug, Clone)]
struct CacheManifest {
    fingerprint: String,
    module_name: String,
    sri: String,
}

#[derive(Debug, Clone)]
struct CommandOutput {
    success: bool,
    stdout: String,
}

trait CommandRunner {
    fn run(&self, program: &str, args: &[String], cwd: &Path) -> Result<CommandOutput>;
}

struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, program: &str, args: &[String], cwd: &Path) -> Result<CommandOutput> {
        let output = Command::new(program)
            .current_dir(cwd)
            .args(args)
            .output()
            .with_context(|| format!("failed to run {program}"))?;

        Ok(CommandOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        })
    }
}

pub fn bundle_browser_search(
    root: &Path,
    module_name: &str,
    assets_dir: &Path,
) -> Result<BundleReport> {
    bundle_browser_search_with_runner(&SystemCommandRunner, root, module_name, assets_dir)
}

fn bundle_browser_search_with_runner(
    runner: &dyn CommandRunner,
    root: &Path,
    module_name: &str,
    assets_dir: &Path,
) -> Result<BundleReport> {
    let start = Instant::now();
    let cache_dir = root.join("target").join("lou32help").join("browser-search");
    let manifest_path = cache_dir.join("manifest.txt");
    let generated_dir = cache_dir.join("generated");
    let fingerprint = compute_bundle_fingerprint(root, module_name)?;

    fs::create_dir_all(&generated_dir)
        .with_context(|| format!("failed to create {}", generated_dir.display()))?;

    let (manifest, cache_hit) = match read_manifest(&manifest_path) {
        Ok(Some(manifest))
            if manifest.fingerprint == fingerprint
                && manifest.module_name == module_name
                && cached_assets_exist(&generated_dir, module_name) =>
        {
            info!(
                cache_hit = true,
                module_name,
                elapsed_ms = start.elapsed().as_millis(),
                "browser bundle cache hit"
            );
            (manifest, true)
        }
        Ok(_) | Err(_) => {
            let reason = if !cached_assets_exist(&generated_dir, module_name) {
                "missing cached assets"
            } else {
                "input fingerprint changed"
            };
            info!(
                cache_hit = false,
                module_name, reason, "browser bundle cache miss"
            );
            (
                rebuild_bundle_cache(runner, root, module_name, &generated_dir, &fingerprint)?,
                false,
            )
        }
    };

    let asset_bytes = copy_cached_assets(&generated_dir, module_name, assets_dir)?;
    inject_wasm_integrity(&assets_dir.join("search.js"), &manifest.sri)?;

    fs::write(
        &manifest_path,
        format!(
            "fingerprint={}\nmodule_name={}\nsri={}\n",
            manifest.fingerprint, manifest.module_name, manifest.sri
        ),
    )
    .with_context(|| format!("failed to write {}", manifest_path.display()))?;

    info!(
        cache_hit,
        module_name,
        asset_bytes,
        elapsed_ms = start.elapsed().as_millis(),
        "bundled browser search"
    );

    Ok(BundleReport {
        cache_hit,
        asset_bytes,
    })
}

fn rebuild_bundle_cache(
    runner: &dyn CommandRunner,
    root: &Path,
    module_name: &str,
    generated_dir: &Path,
    fingerprint: &str,
) -> Result<CacheManifest> {
    ensure_wasm_target(runner, root)?;
    ensure_wasm_bindgen(runner, root)?;

    let cargo_args = vec![
        "build".to_string(),
        "-p".to_string(),
        "lou32help-web-search".to_string(),
        "--target".to_string(),
        WASM_TARGET.to_string(),
        "--release".to_string(),
    ];
    let cargo_output = runner.run("cargo", &cargo_args, root)?;
    if !cargo_output.success {
        bail!("failed to build lou32help-web-search");
    }

    let wasm_path = root
        .join("target")
        .join(WASM_TARGET)
        .join("release")
        .join("lou32help_web_search.wasm");
    let bindgen_args = vec![
        "--target".to_string(),
        "web".to_string(),
        "--no-typescript".to_string(),
        "--out-dir".to_string(),
        generated_dir.display().to_string(),
        "--out-name".to_string(),
        module_name.to_string(),
        wasm_path.display().to_string(),
    ];
    let bindgen_output = runner.run("wasm-bindgen", &bindgen_args, root)?;
    if !bindgen_output.success {
        bail!("wasm-bindgen failed. Install it with `cargo install wasm-bindgen-cli`.");
    }

    let wasm_output = generated_dir.join(format!("{module_name}_bg.wasm"));
    let wasm_bytes = fs::read(&wasm_output)
        .with_context(|| format!("failed to read {}", wasm_output.display()))?;
    let sri = format!(
        "sha384-{}",
        base64::engine::general_purpose::STANDARD.encode(Sha384::digest(&wasm_bytes))
    );

    Ok(CacheManifest {
        fingerprint: fingerprint.to_string(),
        module_name: module_name.to_string(),
        sri,
    })
}

fn ensure_wasm_target(runner: &dyn CommandRunner, root: &Path) -> Result<()> {
    let args = vec![
        "target".to_string(),
        "list".to_string(),
        "--installed".to_string(),
    ];
    let output = runner.run("rustup", &args, root)?;
    if output.success && output.stdout.lines().any(|line| line.trim() == WASM_TARGET) {
        return Ok(());
    }

    bail!(
        "wasm target '{WASM_TARGET}' is missing. Install it with `rustup target add {WASM_TARGET}`."
    );
}

fn ensure_wasm_bindgen(runner: &dyn CommandRunner, root: &Path) -> Result<()> {
    let args = vec!["--version".to_string()];
    let output = runner.run("wasm-bindgen", &args, root)?;
    if output.success {
        return Ok(());
    }

    bail!("wasm-bindgen is missing. Install it with `cargo install wasm-bindgen-cli`.");
}

fn cached_assets_exist(generated_dir: &Path, module_name: &str) -> bool {
    generated_dir.join(format!("{module_name}.js")).exists()
        && generated_dir
            .join(format!("{module_name}_bg.wasm"))
            .exists()
}

fn copy_cached_assets(generated_dir: &Path, module_name: &str, assets_dir: &Path) -> Result<u64> {
    fs::create_dir_all(assets_dir)
        .with_context(|| format!("failed to create {}", assets_dir.display()))?;

    let mut bytes = 0u64;
    for file_name in [
        format!("{module_name}.js"),
        format!("{module_name}_bg.wasm"),
    ] {
        let source = generated_dir.join(&file_name);
        let destination = assets_dir.join(&file_name);
        fs::copy(&source, &destination).with_context(|| {
            format!(
                "failed to copy cached browser asset {} to {}",
                source.display(),
                destination.display()
            )
        })?;
        bytes += fs::metadata(&destination)
            .with_context(|| format!("failed to stat {}", destination.display()))?
            .len();
    }
    Ok(bytes)
}

fn inject_wasm_integrity(search_js_path: &Path, sri: &str) -> Result<()> {
    let js_content = fs::read_to_string(search_js_path)
        .with_context(|| format!("failed to read {}", search_js_path.display()))?;
    if !js_content.contains("__WASM_INTEGRITY__") {
        return Err(anyhow!(
            "search.js is missing the __WASM_INTEGRITY__ placeholder"
        ));
    }

    fs::write(
        search_js_path,
        js_content.replace("__WASM_INTEGRITY__", sri),
    )
    .with_context(|| format!("failed to write {}", search_js_path.display()))
}

fn read_manifest(path: &Path) -> Result<Option<CacheManifest>> {
    if !path.exists() {
        return Ok(None);
    }

    let raw =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    let values = raw
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect::<HashMap<_, _>>();

    let Some(fingerprint) = values.get("fingerprint") else {
        return Ok(None);
    };
    let Some(module_name) = values.get("module_name") else {
        return Ok(None);
    };
    let Some(sri) = values.get("sri") else {
        return Ok(None);
    };

    Ok(Some(CacheManifest {
        fingerprint: fingerprint.clone(),
        module_name: module_name.clone(),
        sri: sri.clone(),
    }))
}

fn compute_bundle_fingerprint(root: &Path, module_name: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(module_name.as_bytes());

    for relative in [
        Path::new("Cargo.toml"),
        Path::new("Cargo.lock"),
        Path::new("crates/lou32help-core"),
        Path::new("crates/lou32help-web-search"),
        Path::new("crates/lou32help-site/src/assets/search.js.tpl"),
    ] {
        hash_path(root, relative, &mut hasher)?;
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn hash_path(root: &Path, relative: &Path, hasher: &mut Sha256) -> Result<()> {
    let full_path = root.join(relative);
    if !full_path.exists() {
        bail!("bundle input '{}' does not exist", full_path.display());
    }

    if full_path.is_file() {
        hasher.update(relative.display().to_string().as_bytes());
        hasher.update(
            fs::read(&full_path)
                .with_context(|| format!("failed to read {}", full_path.display()))?,
        );
        return Ok(());
    }

    let mut files = WalkDir::new(&full_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    files.sort();

    for file in files {
        let relative_file = file
            .strip_prefix(root)
            .with_context(|| format!("failed to strip prefix from {}", file.display()))?;
        hasher.update(relative_file.display().to_string().as_bytes());
        hasher
            .update(fs::read(&file).with_context(|| format!("failed to read {}", file.display()))?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;
    use tempfile::TempDir;

    #[derive(Clone)]
    struct FakeRunner {
        calls: Rc<RefCell<Vec<String>>>,
        failures: Rc<RefCell<HashMap<String, CommandOutput>>>,
    }

    impl FakeRunner {
        fn new() -> Self {
            Self {
                calls: Rc::new(RefCell::new(Vec::new())),
                failures: Rc::new(RefCell::new(HashMap::new())),
            }
        }

        fn fail(&self, program: &str, output: CommandOutput) {
            self.failures
                .borrow_mut()
                .insert(program.to_string(), output);
        }
    }

    impl CommandRunner for FakeRunner {
        fn run(&self, program: &str, args: &[String], _cwd: &Path) -> Result<CommandOutput> {
            self.calls
                .borrow_mut()
                .push(format!("{program} {}", args.join(" ")));

            if let Some(output) = self.failures.borrow().get(program).cloned() {
                return Ok(output);
            }

            if program == "wasm-bindgen" && args.iter().any(|arg| arg == "--out-dir") {
                let out_dir = args
                    .windows(2)
                    .find(|pair| pair[0] == "--out-dir")
                    .map(|pair| PathBuf::from(&pair[1]))
                    .expect("out dir");
                let out_name = args
                    .windows(2)
                    .find(|pair| pair[0] == "--out-name")
                    .map(|pair| pair[1].clone())
                    .expect("out name");
                fs::create_dir_all(&out_dir).expect("create bindgen out dir");
                fs::write(out_dir.join(format!("{out_name}.js")), "// bindgen output")
                    .expect("write bindgen js");
                fs::write(out_dir.join(format!("{out_name}_bg.wasm")), b"wasm-bytes")
                    .expect("write bindgen wasm");
            }

            let stdout = if program == "rustup" {
                format!("{WASM_TARGET}\n")
            } else {
                String::new()
            };

            Ok(CommandOutput {
                success: true,
                stdout,
            })
        }
    }

    fn workspace_fixture() -> TempDir {
        let temp = TempDir::new().expect("tempdir");
        fs::create_dir_all(temp.path().join("crates/lou32help-core/src")).expect("core src");
        fs::create_dir_all(temp.path().join("crates/lou32help-web-search/src")).expect("web src");
        fs::create_dir_all(temp.path().join("crates/lou32help-site/src/assets"))
            .expect("site assets");
        fs::create_dir_all(temp.path().join("dist/site/assets")).expect("dist assets");
        fs::write(temp.path().join("Cargo.toml"), "[workspace]\nmembers=[]\n")
            .expect("write cargo");
        fs::write(temp.path().join("Cargo.lock"), "# lock\n").expect("write lock");
        fs::write(
            temp.path().join("crates/lou32help-core/src/lib.rs"),
            "pub fn core() {}\n",
        )
        .expect("write core");
        fs::write(
            temp.path().join("crates/lou32help-web-search/src/lib.rs"),
            "pub fn wasm() {}\n",
        )
        .expect("write wasm");
        fs::write(
            temp.path()
                .join("crates/lou32help-site/src/assets/search.js.tpl"),
            "template\n",
        )
        .expect("write template");
        fs::write(
            temp.path().join("dist/site/assets/search.js"),
            "const WASM_INTEGRITY = \"__WASM_INTEGRITY__\";\n",
        )
        .expect("write search js");
        temp
    }

    #[test]
    fn cache_miss_builds_and_populates_cache() {
        let temp = workspace_fixture();
        let runner = FakeRunner::new();
        let report = bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .expect("bundle");

        assert!(!report.cache_hit);
        assert!(report.asset_bytes > 0);
        assert!(
            runner
                .calls
                .borrow()
                .iter()
                .any(|call| call.starts_with("cargo build"))
        );
        assert!(
            temp.path()
                .join("target/lou32help/browser-search/manifest.txt")
                .exists()
        );
    }

    #[test]
    fn cache_hit_skips_rebuild_commands() {
        let temp = workspace_fixture();
        let runner = FakeRunner::new();
        bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .expect("first bundle");
        let first_call_count = runner.calls.borrow().len();
        fs::write(
            temp.path().join("dist/site/assets/search.js"),
            "const WASM_INTEGRITY = \"__WASM_INTEGRITY__\";\n",
        )
        .expect("reset search js");

        let report = bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .expect("second bundle");

        assert!(report.cache_hit);
        assert_eq!(runner.calls.borrow().len(), first_call_count);
    }

    #[test]
    fn cache_invalidates_when_inputs_change() {
        let temp = workspace_fixture();
        let runner = FakeRunner::new();
        bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .expect("first bundle");
        let first_call_count = runner.calls.borrow().len();

        fs::write(
            temp.path().join("crates/lou32help-web-search/src/lib.rs"),
            "pub fn wasm() { println!(\"changed\"); }\n",
        )
        .expect("update input");
        fs::write(
            temp.path().join("dist/site/assets/search.js"),
            "const WASM_INTEGRITY = \"__WASM_INTEGRITY__\";\n",
        )
        .expect("reset search js");

        let report = bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .expect("second bundle");

        assert!(!report.cache_hit);
        assert!(runner.calls.borrow().len() > first_call_count);
    }

    #[test]
    fn missing_wasm_bindgen_returns_actionable_error() {
        let temp = workspace_fixture();
        let runner = FakeRunner::new();
        runner.fail(
            "wasm-bindgen",
            CommandOutput {
                success: false,
                stdout: String::new(),
            },
        );

        let error = bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .unwrap_err();

        assert!(format!("{error:#}").contains("cargo install wasm-bindgen-cli"));
    }

    #[test]
    fn missing_wasm_target_returns_actionable_error() {
        let temp = workspace_fixture();
        let runner = FakeRunner::new();
        runner.fail(
            "rustup",
            CommandOutput {
                success: true,
                stdout: String::new(),
            },
        );

        let error = bundle_browser_search_with_runner(
            &runner,
            temp.path(),
            "lou32help_web_search",
            &temp.path().join("dist/site/assets"),
        )
        .unwrap_err();

        assert!(format!("{error:#}").contains("rustup target add"));
    }
}

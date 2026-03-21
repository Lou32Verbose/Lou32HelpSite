$ErrorActionPreference = "Stop"

$dbPath = Join-Path ([System.IO.Path]::GetTempPath()) ("rustsec-advisory-db-" + [guid]::NewGuid().ToString("N"))

try {
    git clone --depth 1 https://github.com/RustSec/advisory-db.git $dbPath | Out-Host
    cargo audit --db $dbPath --no-fetch
}
finally {
    if (Test-Path $dbPath) {
        Remove-Item -Recurse -Force $dbPath
    }
}

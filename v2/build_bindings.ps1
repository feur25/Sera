$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $Root

function Invoke-Step {
    param(
        [string] $Name,
        [string[]] $Command
    )
    Write-Host "==> $Name"
    & $Command[0] @($Command[1..($Command.Length - 1)])
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

function Has-RustTarget {
    param([string] $Name)
    $targets = rustup target list --installed
    foreach ($target in $targets) {
        if ($target.Trim() -eq $Name) {
            return $true
        }
    }
    return $false
}

$CoreFeatures = "gui,sysmon,telemetry-http"
$FfiFeatures = "ffi,gui,sysmon,telemetry-http"
$WasmFeatures = "js"

Invoke-Step "core" @("cargo", "build", "-p", "seraplot", "--features", $CoreFeatures)
Invoke-Step "ffi" @("cargo", "build", "-p", "seraplot", "--features", $FfiFeatures)

$wasmReady = $env:SERAPLOT_BUILD_WASM -eq "1" -and (Has-RustTarget "wasm32-unknown-unknown")
if ($env:SERAPLOT_BUILD_WASM -eq "1" -and -not $wasmReady) {
    Write-Host "==> wasm skipped: target wasm32-unknown-unknown is not installed"
} elseif ($env:SERAPLOT_BUILD_WASM -ne "1") {
    Write-Host "==> wasm skipped: set SERAPLOT_BUILD_WASM=1 to force wasm compilation"
} else {
    Invoke-Step "wasm" @("cargo", "build", "-p", "seraplot", "--release", "--no-default-features", "--features", $WasmFeatures, "--target", "wasm32-unknown-unknown")

    $wasmBindgen = Get-Command wasm-bindgen -ErrorAction SilentlyContinue
    if (-not $wasmBindgen) {
        Write-Host "==> wasm-bindgen CLI not found on PATH -- skipping glue/docs update. Install a version matching Cargo.lock's wasm-bindgen crate: cargo install wasm-bindgen-cli --version <version>"
    } else {
        $wasmOut = Join-Path $Root "src\target\wasm-bindgen-out"
        if (Test-Path $wasmOut) { Remove-Item -Recurse -Force $wasmOut }
        New-Item -ItemType Directory -Path $wasmOut | Out-Null

        Invoke-Step "wasm-bindgen" @(
            "wasm-bindgen", "--target", "no-modules", "--out-dir", $wasmOut, "--out-name", "seraplot",
            (Join-Path $Root "src\target\wasm32-unknown-unknown\release\seraplot.wasm")
        )

        $themeDir = Join-Path $Root "src\docs\theme"
        Copy-Item (Join-Path $wasmOut "seraplot_bg.wasm") (Join-Path $themeDir "seraplot_bg.wasm") -Force
        Copy-Item (Join-Path $wasmOut "seraplot_bg.wasm.d.ts") (Join-Path $themeDir "seraplot_bg.wasm.d.ts") -Force
        Copy-Item (Join-Path $wasmOut "seraplot.d.ts") (Join-Path $themeDir "seraplot.d.ts") -Force

        # wasm-bindgen's raw --target no-modules output only defines the
        # `wasm_bindgen` global -- it never assigns it to window.SeraplotWASM
        # or exposes the __init(path) promise wrapper the docs theme JS
        # (params-panel.js, playground.js) expects. Append that shim on
        # every regeneration instead of hand-patching it after the fact.
        $glueDest = Join-Path $themeDir ("seraplot-web." + (Get-Date -Format "yyyyMMdd") + ".js")
        Copy-Item (Join-Path $wasmOut "seraplot.js") $glueDest -Force
        Add-Content -Path $glueDest -Value "`r`nwindow.SeraplotWASM = wasm_bindgen;`r`nwindow.SeraplotWASM.__init = function(path){return wasm_bindgen(path).then(function(){wasm_bindgen.__ready=true;});};"

        Write-Host "==> wrote $glueDest -- update book.toml's additional-js entry and docs/theme/params-panel.js's SP_WASM_BUILD default to match its date stamp if it changed"
    }
}

Write-Host "==> generated runtime adapters"
Write-Host "target build output contains the auto-discovered FFI and WASM surfaces"

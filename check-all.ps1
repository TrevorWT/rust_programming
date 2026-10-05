<#
.SYNOPSIS
    Runs cargo fmt --check, cargo test, and cargo clippy against every
    exercise project in this directory and reports a pass/fail summary.

.DESCRIPTION
    Each Exercism exercise here is its own standalone Cargo project (no
    shared workspace), so this script auto-discovers every directory
    (at any depth under this script's location) containing a Cargo.toml
    and runs the three checks against each one.

.PARAMETER Fix
    If set, runs `cargo fmt` (without --check) instead, reformatting any
    misformatted files in place.

.EXAMPLE
    .\check-all.ps1
    Check formatting, run tests, and run clippy for every project.

.EXAMPLE
    .\check-all.ps1 -Fix
    Reformat every project in place, then run tests and clippy.
#>
param(
    [switch]$Fix
)

$ErrorActionPreference = "Continue"
$root = $PSScriptRoot

$projects = Get-ChildItem -Path $root -Directory -Recurse |
    Where-Object {
        (Test-Path (Join-Path $_.FullName "Cargo.toml")) -and
        ($_.FullName -notmatch '\\target(\\|$)')
    } |
    Sort-Object FullName

if (-not $projects) {
    Write-Host "No Cargo projects found under $root" -ForegroundColor Yellow
    exit 1
}

$results = @()

foreach ($project in $projects) {
    $name = $project.FullName.Substring($root.Length).TrimStart('\', '/')
    $manifest = Join-Path $project.FullName "Cargo.toml"
    Write-Host "`n==================== $name ====================" -ForegroundColor Cyan

    Write-Host "--- cargo fmt ---" -ForegroundColor DarkCyan
    if ($Fix) {
        cargo fmt --manifest-path $manifest
        $fmtOk = $LASTEXITCODE -eq 0
    } else {
        cargo fmt --manifest-path $manifest --check
        $fmtOk = $LASTEXITCODE -eq 0
    }

    Write-Host "--- cargo test ---" -ForegroundColor DarkCyan
    cargo test --manifest-path $manifest --quiet
    $testOk = $LASTEXITCODE -eq 0

    Write-Host "--- cargo clippy ---" -ForegroundColor DarkCyan
    cargo clippy --manifest-path $manifest --all-targets -- -D warnings
    $clippyOk = $LASTEXITCODE -eq 0

    $results += [pscustomobject]@{
        Project = $name
        Fmt     = $fmtOk
        Test    = $testOk
        Clippy  = $clippyOk
    }
}

Write-Host "`n==================== Summary ====================" -ForegroundColor Cyan
$results | Format-Table -AutoSize

$failed = $results | Where-Object { -not ($_.Fmt -and $_.Test -and $_.Clippy) }
if ($failed) {
    Write-Host "`nFAILED: $($failed.Project -join ', ')" -ForegroundColor Red
    exit 1
} else {
    Write-Host "`nAll projects passed fmt, test, and clippy." -ForegroundColor Green
    exit 0
}

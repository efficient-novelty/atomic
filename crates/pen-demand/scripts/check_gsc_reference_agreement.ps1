param(
    [string]$WorkspaceRoot
)

$ErrorActionPreference = 'Stop'

if ([string]::IsNullOrWhiteSpace($WorkspaceRoot)) {
    $WorkspaceRoot = [System.IO.Path]::GetFullPath(
        (Join-Path $PSScriptRoot '..\..\..')
    )
} else {
    $WorkspaceRoot = [System.IO.Path]::GetFullPath($WorkspaceRoot)
}

Push-Location $WorkspaceRoot
try {
    cargo test --locked -p pen-demand `
        gsc::tests::live_reference_agreement_mints_pinned_capability_and_verifier_manifest `
        -- --exact --ignored
    if ($LASTEXITCODE -ne 0) {
        throw 'Authoritative pinned Rust/Agda GSC reference-agreement gate failed'
    }
} finally {
    Pop-Location
}

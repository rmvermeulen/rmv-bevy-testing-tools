#!/usr/bin/env pwsh

$feature_groups = @( 
    @("insta"),
    @("rstest"),
    @("speculoos"),
    @("iter_tools"),
    @("trait_manage_state"),
    @("insta", "rstest"),
    @("insta", "speculoos"),
    @("insta", "iter_tools"),
    @("insta", "trait_manage_state"),
    @("rstest", "speculoos"),
    @("rstest", "iter_tools"),
    @("rstest", "trait_manage_state"),
    @("speculoos", "iter_tools"),
    @("speculoos", "trait_manage_state"),
    @("insta", "rstest", "speculoos"),
    @("insta", "rstest", "iter_tools"),
    @("insta", "rstest", "trait_manage_state"),
    @("insta", "speculoos", "iter_tools"),
    @("insta", "speculoos", "trait_manage_state"),
    @("rstest", "speculoos", "iter_tools"),
    @("rstest", "speculoos", "trait_manage_state"),
    @("rstest", "iter_tools", "trait_manage_state"),
    @("insta", "rstest", "speculoos", "iter_tools"),
    @("insta", "rstest", "speculoos", "trait_manage_state"),
    @("insta", "rstest", "iter_tools", "trait_manage_state"),
    @("insta", "speculoos", "iter_tools", "trait_manage_state"),
    @("rstest", "speculoos", "iter_tools", "trait_manage_state"),
    @("insta", "rstest", "speculoos", "iter_tools", "trait_manage_state")
)

foreach ($items in $feature_groups)
{
    $features = $items -join ","
    Write-Host "Running check+test for features: '$features'"
    cargo check --no-default-features --features $features 2>&1 1>$null
    if (! $?)
    {
        Write-Error "check failed for '$features'"
        exit 1
    }
    cargo test --no-default-features --features $features 2>&1 1>$null
    if (! $?)
    {
        Write-Error "test failed for '$features'"
        exit 1
    }
}

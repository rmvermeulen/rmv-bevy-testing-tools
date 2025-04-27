#!/usr/bin/env pwsh

# features: insta, rstest, speculoos, iter_tools, trait_manage_state

param(
    [switch] $Check,
    [switch] $Test
)

if (!$Check -and !$Test)
{
    Write-Host "Pass '-Check' and/or '-Test' as an argument"
    exit 1
}

$feature_groups = @( 
    # 1
    @("insta"),
    @("rstest"),
    @("speculoos"),
    @("iter_tools"),
    @("trait_manage_state"),
    # 2
    @("insta", "rstest"),
    @("insta", "speculoos"),
    @("insta", "iter_tools"),
    @("insta", "trait_manage_state"),
    @("rstest", "speculoos"),
    @("rstest", "iter_tools"),
    @("rstest", "trait_manage_state"),
    @("speculoos", "iter_tools"),
    @("speculoos", "trait_manage_state"),
    @("iter_tools", "trait_manage_state"),
    # 3
    @("insta", "rstest", "speculoos"),
    @("insta", "rstest", "iter_tools"),
    @("insta", "rstest", "trait_manage_state"),
    @("insta", "speculoos", "iter_tools"),
    @("insta", "speculoos", "trait_manage_state"),
    @("insta", "iter_tools", "trait_manage_state"),
    @("rstest", "speculoos", "iter_tools"),
    @("rstest", "speculoos", "trait_manage_state"),
    @("rstest", "iter_tools", "trait_manage_state"),
    # 4
    @("insta", "rstest", "speculoos", "iter_tools"),
    @("insta", "rstest", "speculoos", "trait_manage_state"),
    @("insta", "rstest", "iter_tools", "trait_manage_state"),
    @("insta", "speculoos", "iter_tools", "trait_manage_state"),
    @("rstest", "speculoos", "iter_tools", "trait_manage_state"),
    # 5
    @("insta", "rstest", "speculoos", "iter_tools", "trait_manage_state")
)

foreach ($items in $feature_groups)
{
    $features = $items -join ","
    if ($Check)
    {
        Write-Host "Running check for features: '$features'"
        $output = cargo check --no-default-features --features $features 2>&1
        if (! $?)
        {
            Write-Output $output
            Write-Error "check failed for '$features'"
            exit 1
        }
    }
    if ($Test)
    {
        Write-Host "Running test for features: '$features'"
        $output = cargo test --no-default-features --features $features 2>&1
        if (! $?)
        {
            Write-Output $output
            Write-Error "test failed for '$features'"
            exit 1
        }
    }
}

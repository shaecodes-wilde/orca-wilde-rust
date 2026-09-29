param([string]$Toolchain='stable',[switch]$WindowsChild,[switch]$WindowsScene)
if ($Toolchain -notin @('stable','1.94.0')) { throw 'Compiler must be stable or pinned 1.94.0' }
$toolchainSelector='+'+$Toolchain
$ErrorActionPreference = 'Stop'
$projectRoot = Split-Path -Parent $PSScriptRoot
$gitCommon = & git -c "safe.directory=$projectRoot" -C $projectRoot rev-parse --git-common-dir
if ($LASTEXITCODE -ne 0) { throw 'Cannot establish shared repository build authority' }
$commonPath = if ([System.IO.Path]::IsPathRooted($gitCommon)) { $gitCommon } else { Join-Path $projectRoot $gitCommon }
$commonPath = (Resolve-Path -LiteralPath $commonPath).Path
$primaryRoot = Split-Path -Parent $commonPath
$expectedBuildRoot = [System.IO.Path]::GetFullPath((Join-Path (Split-Path -Parent $primaryRoot) 'wr-build'))
if ($env:WILDE_BUILD_ROOT -and ((-not [System.IO.Path]::IsPathRooted($env:WILDE_BUILD_ROOT)) -or [System.IO.Path]::GetFullPath($env:WILDE_BUILD_ROOT) -ne $expectedBuildRoot)) { throw 'Build override must match the canonical shared build root' }
New-Item -ItemType Directory -Force -Path $expectedBuildRoot | Out-Null
$buildRoot = (Resolve-Path -LiteralPath $expectedBuildRoot).Path
if ((Get-Item -LiteralPath $buildRoot).Attributes -band [System.IO.FileAttributes]::ReparsePoint) { throw 'Build root cannot be a link/reparse point' }
$leasePath = Join-Path $commonPath 'native-build-lease.json'
$lease = [System.IO.File]::Open($leasePath,[System.IO.FileMode]::CreateNew,[System.IO.FileAccess]::Write,[System.IO.FileShare]::Read)
$leaseWriter = [System.IO.StreamWriter]::new($lease)
$ownedStepExited = $true
try {
    $leaseWriter.WriteLine((@{pid=$PID;started=(Get-Process -Id $PID).StartTime.ToUniversalTime().ToString('o');root=$projectRoot;rule='Retain uncertain lease; verify exact owner and child completion before recovery.'} | ConvertTo-Json -Compress)); $leaseWriter.Flush()
    $env:CARGO_BUILD_JOBS='2'; $env:CARGO_INCREMENTAL='0'; $env:RUST_TEST_THREADS='1'; $env:ORCA_BACKGROUND_LAUNCH='1'
    $env:CARGO_TARGET_DIR=Join-Path $buildRoot 'a01'
    foreach ($role in @('DOMAIN','BROWSER','RUNTIME','IDE')) {
        $scratch=Join-Path $buildRoot ($role.ToLowerInvariant()+'-tests')
        New-Item -ItemType Directory -Force -Path $scratch | Out-Null
        [Environment]::SetEnvironmentVariable(('WILDE_'+$role+'_TEST_ROOT'),$scratch,'Process')
    }
    Push-Location $projectRoot
    try {
        $evidenceRoot=Join-Path $projectRoot 'evidence'
        $steps=@(
            @{name='format';arguments=@($toolchainSelector,'fmt','--all','--','--check')},
            @{name='check';arguments=@($toolchainSelector,'check','--workspace','--offline','--locked')},
            @{name='test-build';arguments=@($toolchainSelector,'test','--workspace','--offline','--locked','--no-run','--message-format=json')},
            @{name='guarded-tests';arguments=@()},
            @{name='clippy';arguments=@($toolchainSelector,'clippy','--workspace','--all-targets','--offline','--locked','--','-D','warnings')}
        )
        if ($WindowsChild) { $steps+=@{name='windows-child';arguments=@($toolchainSelector,'check','-p','wilde-browser','--features','windows-child','--offline','--locked')} }
        if ($WindowsScene) { $steps+=@{name='windows-scene';arguments=@($toolchainSelector,'check','-p','wilde-desktop','--features','windows-scene','--offline','--locked')} }
        $results=@(); $guardRunId=[Guid]::NewGuid().ToString('N')
        foreach ($step in $steps) {
            $logPath=Join-Path $evidenceRoot ('integrated-'+$step.name+'.log')
            $started=[DateTime]::UtcNow; $ownedStepExited=$false
            if ($step.name -eq 'test-build') {
                $cargoArguments=$step.arguments
                & cargo @cargoArguments > (Join-Path $evidenceRoot 'cargo-test-artifacts.jsonl') 2> $logPath
            } elseif ($step.name -eq 'guarded-tests') {
                & python (Join-Path $PSScriptRoot 'guarded_tests.py') --run-id $guardRunId --artifacts (Join-Path $evidenceRoot 'cargo-test-artifacts.jsonl') --target-root $env:CARGO_TARGET_DIR --logs (Join-Path $evidenceRoot 'guarded-tests') --timeout 45 *> $logPath
            } else { $cargoArguments=$step.arguments; & cargo @cargoArguments *> $logPath }
            $code=$LASTEXITCODE; $ownedStepExited=$true
            if ($step.name -eq 'guarded-tests') {
                $ownedStepExited=$false
                $guardReceipt=Join-Path $evidenceRoot 'guarded-tests/results.json'
                if (-not (Test-Path -LiteralPath $guardReceipt)) { $ownedStepExited=$false } else {
                    $guardResults=Get-Content -LiteralPath $guardReceipt -Raw | ConvertFrom-Json
                    if ($guardResults.run_id -eq $guardRunId -and $guardResults.completed -is [bool] -and $guardResults.completed -eq $true -and @($guardResults.results).Count -gt 0 -and @($guardResults.results | Where-Object { $_.owned_job_empty -isnot [bool] -or $_.owned_job_empty -ne $true }).Count -eq 0) { $ownedStepExited=$true }
                }
            }
            $results+=@{step=$step.name;exit_code=$code;started_utc=$started.ToString('o');elapsed_seconds=([DateTime]::UtcNow-$started).TotalSeconds;log=$logPath}
            $results | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $evidenceRoot 'verification-steps.json') -Encoding utf8
            if ($code -ne 0) { Get-Content -LiteralPath $logPath -Tail 45; throw ('Verification failed: '+$step.name) }
            Write-Output ($step.name+': passed')
        }
    } finally { Pop-Location }
} finally {
    $leaseWriter.Dispose()
    if ($ownedStepExited) { Remove-Item -LiteralPath $leasePath } else { Write-Warning ('Cancellation left uncertain process completion; build lease retained at '+$leasePath) }
}

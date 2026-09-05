# gate.ps1 - GooseClaw definition-of-done gate (Windows PowerShell 5.1+)
# Runs every proof line from RECIPE.md. Prints PASS/FAIL per subsystem,
# ends with "X/Y subsystems PASS", exits 0 only if all pass.
# -SelfTest plants a failing proof in a temp copy of RECIPE.md, confirms
# the FAIL fires and the exit code is nonzero, restores, prints evidence.
# ASCII only. No bash, no POSIX, no && chaining.

param([switch]$SelfTest)

$ErrorActionPreference = 'Stop'
$Root = $PSScriptRoot
$Spec = Join-Path $Root '..\specification.md'   # assembled canon - deliberately NOT committed
$ClaudeMd = Join-Path $Root 'CLAUDE.md'
$Ci = Join-Path $Root 'ci.yml'
$VendorGate = Join-Path $Root 'vendor_gate.py'
$Recipe = Join-Path $Root 'RECIPE.md'
$Gate = Join-Path $Root 'gate.ps1'
$AuditDir = Join-Path $Root 'docs\audit'

function Read-Text($path) {
    if (-not (Test-Path $path)) { return $null }
    return [System.IO.File]::ReadAllText($path)
}

function Count-Hits($content, [string[]]$needles) {
    if ($null -eq $content) { return 0 }
    $hits = 0
    foreach ($n in $needles) { if ($content.Contains($n)) { $hits++ } }
    return $hits
}

function New-Result([bool]$pass, [string]$detail) {
    return [pscustomobject]@{ Pass = $pass; Detail = $detail }
}

# One check per RECIPE.md subsystem. A row with no check here FAILs -
# the gate cannot silently pass a growing graph.
$script:Checks = [ordered]@{

    'Foundation spec (assembled canon)' = {
        $c = Read-Text $Spec
        $need = @('fiduciary agent runtime', 'AGPL-3.0-or-later')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in ..\specification.md (canon, uncommitted)')
    }

    'Seed gate files (94f0658)' = {
        $h = 0
        foreach ($f in @($ClaudeMd, $Ci, $VendorGate)) { if (Test-Path $f) { $h++ } }
        $cm = Read-Text $ClaudeMd
        if ($null -ne $cm -and $cm.Contains('TIER 1')) { $h++ }
        New-Result ($h -eq 4) "$h/4 (CLAUDE.md + ci.yml + vendor_gate.py exist; CLAUDE.md has TIER 1)"
    }

    'Seven-crate design (spec)' = {
        $c = Read-Text $ClaudeMd
        $need = @('`gooseclaw`', '-channels', '-skills', '-nostr', '-knowledge', '-payments', '-safety')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' crate names in CLAUDE.md')
    }

    'Rule-9 gate (core-owned)' = {
        $cm = Read-Text $ClaudeMd
        $c = Read-Text $Spec
        $h = (Count-Hits $cm @('Rule 9', 'go-ahead')) + (Count-Hits $c @('core-owned', 'one enforcement point'))
        New-Result ($h -eq 4) "$h/4 (CLAUDE.md: Rule 9 + go-ahead; canon: core-owned + one enforcement point)"
    }

    'ACP contract (Approve mode, protocolVersion 1)' = {
        $c = Read-Text $Spec
        $need = @('protocolVersion = 1', 'Approve')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in canon section 3.1')
    }

    'MCP-vault federation (Alfred sole writer)' = {
        $c = Read-Text $Spec
        $need = @('Single-writer', 'system-of-record')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in canon section 3.2')
    }

    'Persistence (libSQL/FTS5 + native vector)' = {
        $c = Read-Text $Spec
        $need = @('libSQL', 'FTS5', 'native vector')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in canon')
    }

    'Nostr channel layer' = {
        $c = Read-Text $Spec
        $need = @('NIP-17', 'NIP-44', 'NIP-46')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' NIPs in canon')
    }

    'Channel priority (RULED 2026-09-01)' = {
        $c = Read-Text $Spec
        $need = @('Signal', 'White Noise', 'marmot-protocol/mdk')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in canon section 7.2(b)')
    }

    'Payments (CDK/Lightning)' = {
        $c = Read-Text $Spec
        $need = @('Cashu', 'P2PK', 'NIP-57')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in canon')
    }

    'Skillsmith scope (RULED 2026-09-01)' = {
        $c = Read-Text $Spec
        $need = @('Skillsmith', 'ELv2', '2026-09-01')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in canon section 7.2(a)')
    }

    'Vendor denylist enforcement (Meta/OpenAI/xAI)' = {
        $h = 0
        $notes = @()
        $ci = Read-Text $Ci
        $vg = Read-Text $VendorGate
        if ($null -ne $ci -and $ci.Contains('--self-test')) { $h++ } else { $notes += 'ci.yml missing --self-test wiring' }
        if ($null -ne $vg -and $vg.Contains('EXCEPTIONS: dict[str, str] = {}')) { $h++ } else { $notes += 'EXCEPTIONS map not empty or gate unreadable' }
        if ($null -ne $ci -and [regex]::Matches($ci, 'PIN_RESOLVE_IN_PHASE_0').Count -eq 9) { $h++ } else { $notes += 'PIN_RESOLVE_IN_PHASE_0 count drifted from 9' }
        $py = Get-Command python -ErrorAction SilentlyContinue
        if ($null -ne $py) {
            & python $VendorGate --self-test | Out-Null
            if ($LASTEXITCODE -eq 0) { $h++ } else { $notes += "vendor_gate.py --self-test exit $LASTEXITCODE" }
        } else { $notes += 'python not on PATH' }
        $d = "$h/4 (vendor_gate.py --self-test exit 0; ci.yml wires --self-test; EXCEPTIONS empty; 9 PIN placeholders)"
        if ($notes.Count -gt 0) { $d = $d + ' :: ' + ($notes -join '; ') }
        New-Result ($h -eq 4) $d
    }

    'Graph-engineering convention' = {
        $h = 0
        foreach ($f in @($Recipe, $Gate)) { if (Test-Path $f) { $h++ } }
        if (Test-Path $AuditDir) { $h++ }
        $cm = Read-Text $ClaudeMd
        if ($null -ne $cm -and $cm.Contains('Project graph')) { $h++ }
        New-Result ($h -eq 4) "$h/4 (RECIPE.md + gate.ps1 + docs/audit/ exist; CLAUDE.md has Project graph)"
    }

    # SelfTest canary: proof demands a string that does not exist. Must FAIL.
    'ZZ-SelfTest-Canary' = {
        $c = Read-Text $Spec
        $h = Count-Hits $c @('SELFTEST-CANARY-ABSENT-STRING-7F3A')
        New-Result ($h -eq 1) "$h/1 (planted-absent string in canon)"
    }
}

function Invoke-Gate([string]$recipePath, [switch]$IncludeCanary) {
    $lines = [System.IO.File]::ReadAllLines($recipePath)
    $rows = @()
    foreach ($line in $lines) {
        if ($line -match '^\s*\|(.+)\|(.+)\|(.+)\|\s*$') {
            $name = $Matches[1].Trim()
            if ($name -eq 'Subsystem' -or $name -match '^[-\s]+$') { continue }
            $rows += $name
        }
    }
    $pass = 0
    $total = 0
    $out = @()
    foreach ($name in $rows) {
        if (-not $IncludeCanary -and $name -eq 'ZZ-SelfTest-Canary') { continue }
        $total++
        if ($script:Checks.Contains($name)) {
            $r = & $script:Checks[$name]
        } else {
            $r = New-Result $false 'no gate check implemented for this subsystem'
        }
        if ($r.Pass) { $pass++; $out += "PASS $name -- $($r.Detail)" }
        else { $out += "FAIL $name -- $($r.Detail)" }
    }
    $out += "$pass/$total subsystems PASS"
    $code = 0
    if ($pass -ne $total) { $code = 1 }
    return [pscustomobject]@{ Lines = $out; ExitCode = $code }
}

if ($SelfTest) {
    Write-Output 'SELFTEST: planting a failing proof in a temp copy of RECIPE.md'
    $tmp = Join-Path $env:TEMP ('RECIPE.selftest-' + [guid]::NewGuid().ToString('N') + '.md')
    $canaryRow = '| ZZ-SelfTest-Canary | none | canon contains planted-absent string - 1/1 |'
    $body = [System.IO.File]::ReadAllText($Recipe)
    [System.IO.File]::WriteAllText($tmp, $body + "`r`n" + $canaryRow + "`r`n")
    try {
        $r = Invoke-Gate $tmp -IncludeCanary
        Write-Output '--- selftest gate output (temp copy, canary planted) ---'
        foreach ($l in $r.Lines) { Write-Output $l }
        Write-Output ("--- exit code with canary planted: " + $r.ExitCode + ' ---')
        $canaryLine = $r.Lines | Where-Object { $_ -like 'FAIL ZZ-SelfTest-Canary*' }
        if ($null -ne $canaryLine -and $r.ExitCode -eq 1) {
            Write-Output 'SELFTEST PASS -- gate correctly FAILed on the planted proof and exited 1'
            Write-Output 'SELFTEST evidence: canary FAIL line observed; non-passing summary observed; real RECIPE.md untouched'
            exit 0
        } else {
            Write-Output 'SELFTEST FAIL -- gate did NOT fail on a planted bad proof; the gate is not trustworthy'
            exit 1
        }
    } finally {
        if (Test-Path $tmp) { Remove-Item $tmp -Force }
        Write-Output 'SELFTEST: temp copy removed; RECIPE.md restored (never modified)'
    }
}

$result = Invoke-Gate $Recipe
foreach ($l in $result.Lines) { Write-Output $l }
exit $result.ExitCode

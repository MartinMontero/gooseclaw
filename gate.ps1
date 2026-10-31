# gate.ps1 - GooseClaw definition-of-done gate (Windows PowerShell 5.1+)
# Runs every proof line from RECIPE.md. Prints PASS/FAIL per subsystem,
# ends with "X/Y subsystems PASS", exits 0 only if all pass.
# -SelfTest plants a failing proof in a temp copy of RECIPE.md, confirms
# the FAIL fires and the exit code is nonzero, restores, prints evidence.
# ASCII only. No bash, no POSIX, no && chaining.

param([switch]$SelfTest)

$ErrorActionPreference = 'Stop'
$Root = $PSScriptRoot
$Spec = Join-Path $Root 'specification.md'   # assembled canon - committed at repo root (homecoming 2026-09-07)
$ClaudeMd = Join-Path $Root 'CLAUDE.md'
$Ci = Join-Path $Root '.github\workflows\ci.yml'   # moved 2026-10-27 (approved)
$VendorGate = Join-Path $Root 'scripts\vendor_gate.py'   # moved 2026-10-27 (approved)
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

function Invoke-CargoTest([string]$Package) {
    # Windows PowerShell 5.1 turns native stderr into a fatal error under
    # $ErrorActionPreference='Stop'. Capture output and exit code without
    # hiding either, then restore the caller's preference in every path.
    if ($null -eq (Get-Command cargo -ErrorAction SilentlyContinue)) {
        return [pscustomobject]@{ Output = 'cargo not on PATH'; ExitCode = 1 }
    }
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $out = & cargo test -p $Package 2>&1 | Out-String
        $code = $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $previous
    }
    return [pscustomobject]@{ Output = $out; ExitCode = $code }
}

# One check per RECIPE.md subsystem. A row with no check here FAILs -
# the gate cannot silently pass a growing graph.
$script:Checks = [ordered]@{

    'Foundation spec (assembled canon)' = {
        $c = Read-Text $Spec
        $need = @('fiduciary agent runtime', 'AGPL-3.0-or-later')
        $h = Count-Hits $c $need
        New-Result ($h -eq $need.Count) ("$h/" + $need.Count + ' tokens in specification.md (canon, committed at repo root)')
    }

    'Seed gate files (94f0658)' = {
        $h = 0
        foreach ($f in @($ClaudeMd, $Ci, $VendorGate)) { if (Test-Path $f) { $h++ } }
        $cm = Read-Text $ClaudeMd
        if ($null -ne $cm -and $cm.Contains('TIER 1')) { $h++ }
        New-Result ($h -eq 4) "$h/4 (CLAUDE.md + .github/workflows/ci.yml + scripts/vendor_gate.py exist; CLAUDE.md has TIER 1)"
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

    'MCP-vault federation client (Phase 2: expectedVersion + offline semantics)' = {
        # Phase 2 proof: cargo test -p gooseclaw-knowledge must sum to 8 passed
        # (RRF's 2 + 6 federation tests in tests/federation.rs). A missing
        # toolchain or a failing suite is a FAIL, never a skip (DRIFT RULE).
        $test = Invoke-CargoTest 'gooseclaw-knowledge'
        $out = $test.Output
        $code = $test.ExitCode
        $total = 0
        foreach ($m in [regex]::Matches($out, 'test result: ok\.\s+(\d+) passed')) {
            $total += [int]$m.Groups[1].Value
        }
        $src = Read-Text (Join-Path $Root 'crates\gooseclaw-knowledge\tests\federation.rs')
        $names = @('era_probe_and_legacy_handshake', 'conflict_human_edit_wins', 'offline_mute_and_stale_reads', 'reconnect_revalidates_before_write', 'ttl_serves_fresh_reads_from_cache', 'era_is_detected_as_dual_era_aware')
        $present = 0
        if ($null -ne $src) {
            foreach ($n in $names) { if ($src.Contains($n)) { $present++ } }
        }
        $ok = ($code -eq 0) -and ($total -eq 8) -and ($present -eq 6)
        New-Result $ok ("cargo test -p gooseclaw-knowledge exit=$code; summed passed=$total (expected 8 = RRF 2 + federation 6); named proofs present $present/6")
    }

    'Nostr channel red-team suite and relay seam (Phase 3)' = {
        # Phase 3 proof: cargo test -p gooseclaw-channels must sum to 18 passed
        # (4 baseline + 14 adversarial/transport tests: relay drain x2, relay chunking,
        # relay signer-offline, relay oversized). Source must carry moderate() and
        # chunk_text() — the two functions the suite leans on.
        $test = Invoke-CargoTest 'gooseclaw-channels'
        $out = $test.Output
        $code = $test.ExitCode
        $total = 0
        foreach ($m in [regex]::Matches($out, 'test result: ok\.\s+(\d+) passed')) {
            $total += [int]$m.Groups[1].Value
        }
        $src = Read-Text (Join-Path $Root 'crates\gooseclaw-channels\src\moderation.rs')
        $names = @('moderate', 'chunk_text')
        $present = 0
        if ($null -ne $src) {
            foreach ($n in $names) { if ($src.Contains($n)) { $present++ } }
        }
        $ok = ($code -eq 0) -and ($total -eq 18) -and ($present -eq 2)
        New-Result $ok ("cargo test -p gooseclaw-channels exit=$code; summed passed=$total (expected 18 = 4 baseline + 14 adversarial/transport); moderate()/chunk_text() present $present/2")
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
        if ($null -ne $ci -and [regex]::Matches($ci, 'PIN_RESOLVE_IN_PHASE_0').Count -eq 0 -and $ci.Contains('actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1')) { $h++ } else { $notes += 'pins unresolved or checkout SHA drifted' }
        $py = Get-Command python -ErrorAction SilentlyContinue
        if ($null -ne $py) {
            & python $VendorGate --self-test | Out-Null
            if ($LASTEXITCODE -eq 0) { $h++ } else { $notes += "vendor_gate.py --self-test exit $LASTEXITCODE" }
        } else { $notes += 'python not on PATH' }
        $d = "$h/4 (vendor_gate.py --self-test exit 0; ci.yml wires --self-test; EXCEPTIONS empty; 0 PIN placeholders + checkout SHA pinned)"
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

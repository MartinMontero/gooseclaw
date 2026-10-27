#!/usr/bin/env python3
"""GooseClaw pin-drift check — compares a dated pin snapshot against LIVE upstream.

The snapshot JSON (docs/pin-register/pin-snapshot-YYYY-MM-DD.json) records what a
dated pin-register refresh verified. This script re-resolves every entry against
the live source and exits 1 if any entry has DRIFTED or cannot be resolved
(a proof that cannot run is a FAIL, not a skip — RECIPE.md DRIFT RULE).

This script does NOT re-pin anything. It flags drift loudly; re-pin decisions
are Martin's, recorded in 03_DECISION_LOG.md (pin-register convention).

Usage:
    python scripts/pin_drift_check.py docs/pin-register/pin-snapshot-2026-10-27.json

Exit codes: 0 = all entries match live upstream; 1 = drift or unresolvable entry.
No third-party dependencies: stdlib only (same supply-chain rule as vendor_gate.py).
"""

import json
import re
import sys
import urllib.request

UA = {"User-Agent": "gooseclaw-pin-drift-check (github.com/MartinMontero/gooseclaw)"}


def fetch_json(url: str):
    req = urllib.request.Request(url, headers=UA)
    with urllib.request.urlopen(req, timeout=30) as r:
        return json.loads(r.read().decode())


def fetch_text(url: str) -> str:
    req = urllib.request.Request(url, headers=UA)
    with urllib.request.urlopen(req, timeout=30) as r:
        return r.read().decode()


def resolve(entry: dict):
    """Return (live_value, error). live_value may be str or bool."""
    kind = entry["kind"]
    try:
        if kind == "github_latest_release":
            d = fetch_json(f"https://api.github.com/repos/{entry['repo']}/releases/latest")
            return d["tag_name"], None
        if kind == "github_first_tag":
            d = fetch_json(f"https://api.github.com/repos/{entry['repo']}/tags?per_page=1")
            return d[0]["name"], None
        if kind == "github_repo_exists":
            try:
                fetch_json(f"https://api.github.com/repos/{entry['repo']}")
                return True, None
            except urllib.error.HTTPError as e:
                if e.code == 404:
                    return False, None
                raise
        if kind == "github_file_regex":
            text = fetch_text(
                f"https://raw.githubusercontent.com/{entry['repo']}/{entry.get('ref', 'main')}/{entry['path']}"
            )
            m = re.search(entry["pattern"], text)
            if not m:
                return None, f"pattern not found in {entry['path']}"
            return m.group(1), None
        if kind == "crates_io_max_stable":
            d = fetch_json(f"https://crates.io/api/v1/crates/{entry['crate']}")
            return d["crate"]["max_stable_version"], None
        return None, f"unknown kind '{kind}'"
    except Exception as e:  # network/parse failure = UNVERIFIABLE = FAIL
        return None, f"{type(e).__name__}: {e}"


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__)
        return 1
    with open(sys.argv[1], encoding="utf-8") as f:
        snapshot = json.load(f)

    print(f"pin-drift check against snapshot as-of {snapshot.get('as_of', '?')}")
    drifted = 0
    for e in snapshot["entries"]:
        live, err = resolve(e)
        if err is not None:
            drifted += 1
            print(f"  ERROR    {e['name']}: could not resolve live value ({err})")
            continue
        if live == e["expect"]:
            print(f"  CURRENT  {e['name']}: {live}")
        else:
            drifted += 1
            print(f"  DRIFTED  {e['name']}: snapshot={e['expect']} live={live}")

    total = len(snapshot["entries"])
    if drifted:
        print(f"\nDRIFT: {drifted}/{total} entries drifted or unresolvable.")
        print("Flag it, don't silently fix it. Re-pin decisions are Martin's (03_DECISION_LOG.md).")
        return 1
    print(f"\n{total}/{total} entries match live upstream.")
    return 0


if __name__ == "__main__":
    sys.exit(main())

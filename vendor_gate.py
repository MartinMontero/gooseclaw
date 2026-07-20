#!/usr/bin/env python3
"""GooseClaw vendor-exclusion gate — LAYER 1 (build-time dependencies).

Fails the build if any resolved Cargo dependency originates from a denied
vendor. Policy (Constitution 7): exclude ONLY Meta, OpenAI, xAI.
Google, Mistral, Ollama, MiniMax, and Anthropic are PERMITTED.

This is layer 1 of a three-layer engine. Layers 2 (runtime provider config)
and 3 (model filtering in the setup wizard) are application code, not CI.

Usage:
    python3 scripts/vendor_gate.py              # scan the resolved graph
    python3 scripts/vendor_gate.py --self-test  # prove the matcher works

Exit codes: 0 = clean, 1 = violations found (or the gate itself is broken).
No third-party dependencies: stdlib only, by design (supply-chain surface).
"""

import json
import re
import subprocess
import sys
from pathlib import Path

# --------------------------------------------------------------------------
# Denylist. Additions are a HUMAN decision — surface, don't silently expand.
# --------------------------------------------------------------------------

DENIED_ORGS = {
    "Meta": [
        "github.com/facebook/",
        "github.com/facebookresearch/",
        "github.com/facebookincubator/",
        "github.com/meta-llama/",
    ],
    "OpenAI": [
        "github.com/openai/",
    ],
    "xAI": [
        "github.com/xai-org/",
    ],
}

# Matched as: exact name, or name followed by '-' / '_'.
DENIED_CRATES = {
    "OpenAI": ["openai", "async-openai", "openai-api-rs", "openai_dive", "tiktoken", "tiktoken-rs"],
    "xAI": ["xai", "grok"],
    "Meta": ["llama-api"],
}

DENIED_AUTHOR_DOMAINS = {
    "Meta": ["@meta.com", "@fb.com"],
    "OpenAI": ["@openai.com"],
    "xAI": ["@x.ai"],
}

# --------------------------------------------------------------------------
# EXCEPTIONS — deliberately EMPTY.
#
# Some infrastructure libraries originate from a denied org but have nothing
# to do with AI-provider lock-in (e.g. zstd, rocksdb — both github.com/facebook/).
# The policy as written blocks them. Granting an exception is a RULE-9-CLASS
# DECISION for the human, not a default. To grant one, add:
#
#     "zstd-sys": "Compression only; no AI/provider surface. Approved <date>.",
#
# Every entry needs a rationale and an approval date. Do not add entries to
# make CI green.
# --------------------------------------------------------------------------
EXCEPTIONS: dict[str, str] = {}


def _name_hit(name: str, denied: list[str]) -> str | None:
    n = name.lower()
    for d in denied:
        if n == d or n.startswith(d + "-") or n.startswith(d + "_"):
            return d
    return None


def scan(packages: list[dict]) -> list[dict]:
    """Return a list of violations. Pure function — testable without cargo."""
    violations = []
    for pkg in packages:
        name = pkg.get("name", "")
        version = pkg.get("version", "")
        urls = " ".join(
            str(pkg.get(k) or "") for k in ("repository", "homepage", "documentation")
        ).lower()
        authors = " ".join(pkg.get("authors") or []).lower()

        if name in EXCEPTIONS:
            continue

        hit = None

        for vendor, denied in DENIED_CRATES.items():
            d = _name_hit(name, denied)
            if d:
                hit = (vendor, f"crate name matches denied '{d}'", name)
                break

        if not hit:
            for vendor, orgs in DENIED_ORGS.items():
                for org in orgs:
                    if org in urls:
                        hit = (vendor, f"repository under denied org '{org}'", urls.strip())
                        break
                if hit:
                    break

        if not hit:
            for vendor, domains in DENIED_AUTHOR_DOMAINS.items():
                for dom in domains:
                    if dom in authors:
                        hit = (vendor, f"author domain '{dom}'", authors.strip())
                        break
                if hit:
                    break

        if hit:
            vendor, reason, evidence = hit
            violations.append(
                {
                    "vendor": vendor,
                    "crate": name,
                    "version": version,
                    "reason": reason,
                    "evidence": evidence,
                }
            )
    return violations


def cargo_packages() -> list[dict] | None:
    """Resolved dependency graph, or None if the workspace isn't scaffolded yet."""
    if not Path("Cargo.toml").exists():
        return None
    cmd = ["cargo", "metadata", "--format-version", "1", "--all-features"]
    if Path("Cargo.lock").exists():
        cmd.append("--locked")
    out = subprocess.run(cmd, capture_output=True, text=True, check=True)
    return json.loads(out.stdout).get("packages", [])


def report(violations: list[dict]) -> None:
    print(f"\n  VENDOR GATE FAILED — {len(violations)} denied dependency(ies)\n")
    for v in violations:
        print(f"  [{v['vendor']}] {v['crate']} {v['version']}")
        print(f"      reason:   {v['reason']}")
        print(f"      evidence: {v['evidence'][:120]}")
    print(
        "\n  Policy: Meta / OpenAI / xAI are excluded as dependencies AND model\n"
        "  providers (Constitution 7). Google, Mistral, Ollama, MiniMax and\n"
        "  Anthropic are permitted.\n"
        "  Remove the dependency, or — if it is infrastructure with no AI or\n"
        "  provider surface — take an explicit human decision and add it to\n"
        "  EXCEPTIONS with a rationale and date. Do NOT add entries just to go green.\n"
    )


# --------------------------------------------------------------------------
# Self-test: the deliberately-failing fixture required by the build spec.
# Proves BOTH matchers fire and that real deps do not false-positive.
# --------------------------------------------------------------------------

FIXTURE = [
    # must be caught — crate-name matcher
    {"name": "async-openai", "version": "0.1.0",
     "repository": "https://github.com/64bit/async-openai", "authors": []},
    # must be caught — org-URL matcher (this is the real zstd/rocksdb collision)
    {"name": "zstd-sys", "version": "2.0.0",
     "repository": "https://github.com/facebook/zstd", "authors": []},
    # must pass — real GooseClaw deps
    {"name": "tokio", "version": "1.0.0",
     "repository": "https://github.com/tokio-rs/tokio", "authors": []},
    {"name": "nostr-sdk", "version": "0.4.0",
     "repository": "https://github.com/rust-nostr/nostr", "authors": []},
    {"name": "wasmtime", "version": "43.0.2",
     "repository": "https://github.com/bytecodealliance/wasmtime", "authors": []},
]


def self_test() -> int:
    got = {v["crate"] for v in scan(FIXTURE)}
    must_catch = {"async-openai", "zstd-sys"}
    must_pass = {"tokio", "nostr-sdk", "wasmtime"}

    missed = must_catch - got
    false_pos = got & must_pass

    if missed or false_pos:
        print("  SELF-TEST FAILED — the gate itself is broken.")
        if missed:
            print(f"      failed to catch: {sorted(missed)}")
        if false_pos:
            print(f"      false positives: {sorted(false_pos)}")
        return 1

    print(f"  Self-test OK — caught {sorted(must_catch)}, cleared {sorted(must_pass)}.")
    return 0


def main() -> int:
    if "--self-test" in sys.argv:
        return self_test()

    packages = cargo_packages()
    if packages is None:
        print("  No Cargo.toml yet — workspace not scaffolded (Phase 0). Nothing to scan.")
        return 0

    violations = scan(packages)
    if violations:
        report(violations)
        return 1

    print(f"  Vendor gate clean — {len(packages)} packages, 0 denied.")
    if EXCEPTIONS:
        print(f"  ({len(EXCEPTIONS)} human-approved exception(s) in force.)")
    return 0


if __name__ == "__main__":
    sys.exit(main())

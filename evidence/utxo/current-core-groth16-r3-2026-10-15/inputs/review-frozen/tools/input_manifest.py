#!/usr/bin/env python3
"""Reviewer-owned: exact input manifest (sha256/bytes) of every file this review read
in full, read in part, or only hashed/listed. The private provider file is listed by
path only and is never opened. Usage: python3 -I input_manifest.py OUT_JSON"""
import resource, sys, os, json, hashlib
resource.setrlimit(resource.RLIMIT_CPU, (60, 60))
resource.setrlimit(resource.RLIMIT_AS, (1 << 30, 1 << 30))
V = "/home/charl/moriarty-eutxo-validation-2026-10-12"
CAND = V + "/current-core-groth16-compiler-source-r2-20261015"
CO = "/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015"
REG = "/home/charl/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f"
CACHE = "/home/charl/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f"
PLUG = "/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.1/skills/develop"
SELF = V + "/current-core-groth16-compiler-independent-source-review-20261015"

groups = {
    "review_task": [(SELF + "/PROMPT.md", "full"), (SELF + "/run-worker.py", "full"), (SELF + "/PROCESS.json", "full (state at start)")],
    "skill": [(PLUG + "/SKILL.md", "full"), (PLUG + "/references/execution-focus.md", "full")],
    "checkout_governance": [(CO + "/AGENTS.md", "full"), (CO + "/docs/MORIARTY-PRODUCT-CONTRACT.md", "full"), (CO + "/ROADMAP.md", "full"),
                            (CO + "/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md", "full"),
                            (CO + "/openspec/changes/consolidated-language-kernel/requirements.md", "full"),
                            (CO + "/openspec/changes/consolidated-language-kernel/security-reconciliation-2026-10-14.md", "full"),
                            (CO + "/docs/audits/2026-10-14/SECURITY-ROUTING.md", "full"),
                            (CO + "/openspec/changes/consolidated-language-kernel/proposal.md", "hashed only"),
                            (CO + "/openspec/changes/consolidated-language-kernel/traceability.md", "hashed only"),
                            (CO + "/docs/audits/2026-10-14/SECURITY-REVIEW.md", "hashed only"),
                            (CO + "/docs/audits/2026-10-14/AUDIT-PLAN.md", "hashed only")],
    "checkout_core_semantics": [(CO + "/crates/mori-core/src/" + f, m) for f, m in [
        ("ir.rs", "full"), ("lib.rs", "full"), ("logical_cost.rs", "full"), ("runtime.rs", "full"), ("parser.rs", "full"),
        ("policy.rs", "full"), ("intent.rs", "lines 60-94 and 252-350, plus signature grep"), ("stage/mod.rs", "full"),
        ("stage/profile.rs", "full"), ("stage/plan.rs", "full")]] + [
        (CO + "/crates/mori-lexer/src/number.rs", "full"), (CO + "/crates/mori-lexer/src/token.rs", "grep only"),
        (CO + "/Cargo.toml", "full"), (CO + "/rust-toolchain.toml", "full"), (CO + "/crates/mori-core/Cargo.toml", "full"),
        (CO + "/crates/mori-diagnostics/Cargo.toml", "full"), (CO + "/crates/mori-lexer/Cargo.toml", "full"),
        (CO + "/crates/mori-cardano/tests/metadata-consumer/Cargo.toml", "full"),
        (CO + "/crates/mori-cardano/tests/metadata-consumer/Cargo.lock", "parsed (tomllib)")],
    "validation_root_context": [(V + "/guarded-status-20261015-continuation-1000.json", "full"),
                                (V + "/claim-utxo-abi-v1-source-20261015/VOTES.md", "lines 115-205 (P3/P4 literal texts)"),
                                (V + "/claim-utxo-prototype-source-20261015/implementation-attempt-2/source/x7/Cargo.toml", "full"),
                                (V + "/claim-utxo-prototype-source-20261015/implementation-attempt-2/source/x7/Cargo.lock", "parsed (tomllib)")],
    "arkworks_registry_sources": [(REG + "/" + f, m) for f, m in [
        ("ark-relations-0.4.0/src/r1cs/constraint_system.rs", "regions 80-115, 150-270, 505-670, 795-960 + grep"),
        ("ark-relations-0.4.0/src/r1cs/mod.rs", "lines 1-60 and Ord impl"), ("ark-relations-0.4.0/Cargo.toml", "full"),
        ("ark-ff-0.4.2/Cargo.toml", "full"), ("ark-ff-0.4.2/src/fields/prime.rs", "grep"), ("ark-ff-0.4.2/src/fields/models/fp/mod.rs", "grep"),
        ("ark-ff-0.4.2/src/biginteger/mod.rs", "grep"), ("ark-groth16-0.4.0/Cargo.toml", "full"), ("ark-groth16-0.4.0/src/generator.rs", "grep"),
        ("ark-groth16-0.4.0/src/prover.rs", "grep"), ("ark-groth16-0.4.0/src/lib.rs", "grep"), ("ark-groth16-0.4.0/src/verifier.rs", "grep"),
        ("ark-groth16-0.4.0/src/data_structures.rs", "grep"), ("ark-snark-0.4.0/src/lib.rs", "grep"), ("ark-bls12-381-0.4.0/Cargo.toml", "full")]] + [
        (CACHE + "/" + c + ".crate", "tar member listing only") for c in ("ark-ff-0.4.2", "ark-ff-asm-0.4.2", "ark-ff-macros-0.4.2", "ark-serialize-derive-0.4.2")],
}
# The complete candidate (every file except the private provider body).
cand = []
for dp, dns, fns in os.walk(CAND):
    dns[:] = sorted(dns)
    for fn in sorted(fns):
        p = os.path.join(dp, fn)
        rel = os.path.relpath(p, CAND)
        if rel == "provider-result.private.json":
            continue
        big_fixture = rel.startswith("fixtures/reused/") and rel.endswith(".json") and "expected-program" not in rel
        summarized = {"STATIC-CHECKS.json": "all check names, all_ok and failures read",
                      "static-passes/STATIC-CHECKS-PASS-1.json": "all check names, all_ok and failures read",
                      "ROOT-PARTIAL-SOURCE-COPY.json": "top-level fields and every row (truncated display)",
                      "fixtures/staged/reserve.expected-program.json": "parsed; statement kinds per action",
                      "fixtures/staged/delivery.expected-program.json": "parsed; statement kinds per action",
                      "EXPECTED-PACKET/AUTHOR-EXPECTATIONS.json": "parsed; constants, vector ids, P1 used by reviewer tool"}
        cand.append((p, "parsed as data by reviewer tools (structure, scenario, expected execution and cost compared)" if big_fixture else summarized.get(rel, "full")))
groups["candidate_complete"] = cand

out = {"format": "reviewer-input-manifest/1", "not_opened": [CAND + "/provider-result.private.json", SELF + "/provider-result.private.json"],
       "groups": {}}
for g, items in groups.items():
    rows = []
    for p, mode in items:
        if os.path.isfile(p):
            b = open(p, "rb").read()
            rows.append({"path": p, "sha256": hashlib.sha256(b).hexdigest(), "bytes": len(b), "read": mode})
        else:
            rows.append({"path": p, "missing": True, "read": mode})
    out["groups"][g] = rows
with open(sys.argv[1], "w") as f:
    json.dump(out, f, indent=2)
    f.write("\n")
print({g: len(r) for g, r in out["groups"].items()}, "missing:", [r["path"] for rs in out["groups"].values() for r in rs if r.get("missing")])

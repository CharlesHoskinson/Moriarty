# W-D2H image codec initial audit disposition

**Decision, 2026-09-30:** changes required. The original 72/72 test result and nine synthetic byte vectors are retained as observations, but the original codec is not accepted for closed-record admission or coherent multi-image production.

The frozen [packet](w-d2h-image-codec-candidate-packet.md) has SHA256 `34e34595aaab4094ee4735ce81b511b873f82fe3ad7f89648ebcfecb7757d332`. GPT-6.1 Sol high [recomputed all 15 manifest hashes, reran tests/vectors and reproduced a high Proxy defect](w-d2h-image-codec-gpt-6.1-sol-high-audit.md). Grok was requested as 4.7 xhigh; its [raw static result](w-d2h-image-codec-grok-4.7-xhigh-raw.json) reports `grok-4.7-build`, bounded acceptance and no high/medium defect. Grok did not run code or recompute hashes and explicitly noted that `policyValue` shares nested caller objects. The two reviews disagree; the executable counterexamples control this disposition.

The defect permits an invalid selected-action/kind pair and inconsistent Source/policy asset links when Proxy `get` values change after descriptor validation. The original codec is preserved under `hash-image-codec/history/pre-proxy-repair-01/`. A repair is in progress to retain owned descriptor snapshots through all encoding and composition phases, with new hostile tests. Fresh audits must review those new bytes.

No provider authentication, loaded-artifact correspondence, B05–B07 adoption, W-D2 or Sprint 1 closure follows from either audit.

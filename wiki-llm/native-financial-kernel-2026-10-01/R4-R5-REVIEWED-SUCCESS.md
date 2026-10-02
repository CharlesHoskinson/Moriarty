# Reviewed native financial proof and full verification

Experiment observation, SRC-0119, 2026-10-01/02, S3: the fixed trusted public financial fixture produced a finalized 6,336-byte native proof. R4 applied the signed transaction with default Real proof verification enabled and returned Success. R5 independently verified the retained proof and completed the full native fault suite. [Portable evidence manifest](evidence/native-financial-r4-r5/archive-manifest.json) maps unchanged original absolute paths to repository copies.

R4’s native acceptance path enforced escrow 8990, recipient A1 outputs 1000 and 10, storage effects and conservation of the 1e12 NIGHT fixture supply before emitting its receipt. Consumed protocol DUST was 1117490000000001; remainder was 4999998882509999999999 out of 5e21 available. The fee stayed within allowance 1e20. Replaying the accepted transaction refused IntentAlreadyExists and preserved the accepted state. Application fee 10 and protocol DUST are separate quantities.

R4 proof generation exited 0 in 287.614 seconds. R5 verification exited 0 in 67.036 seconds, with sampled process-group CPU 58.18 seconds and RSS 716906496 bytes. Sampling can miss peaks and is not a hard cgroup bound. Both pristine verification passes checked all 1035 public-input mutation indices plus the remaining cryptographic, strict decoder and owner-signature refusals. All six history/claim controls refused natively with no output state: suffixes, truncations and independently reconstructed canonical alternates. Initial and final pristine financial outputs matched the original R4 outputs byte for byte. The canonical alternate fixture also changes operation registration; this is not an isolated timestamp-only control.

[Root acceptance](evidence/native-financial-r4-r5/NATIVE-FINANCIAL-V10-R5-ROOT-ACTUAL-ACCEPTANCE.json) pins actual-result freeze `32305c38817649e1d6bca2d51a93360e8dc065584a5bf7c8fe8bf2c764fd3c26` and records three fresh actual-result approvals: GPT-6.1 Sol high, Claude Opus 5.5 high and Grok 4.7 xhigh. Original reports, terminal receipts and source/resource votes remain unchanged. Historical Opus R4 CPU dissent and the 2–1 decision remain visible.

R4's first independent verification stopped after the first correctly refused history suffix because the helper expected an obsolete decoder message. That partial suite remains a failure. R5 repaired only the verification helper and case expectations, reused the same proof and executable, and completed the suite. Earlier disk stops, signal interruption, unsorted offers and NightBalance failures remain historical failures; none is retroactively successful.

The root acceptance also qualifies two report errors: Sol's earlier cache measurement omitted file symlinks, so no cache reduction is established; Grok prefixed one historical digest with its entry count. The native receipt's 293 operations and 47 reads carry the gates enforced during R4 construction; R5 did not independently recount them. Truncation helper policy is generic, while retained raw logs establish UnexpectedEof.

Only generated `.lake/build` directories in two already-trashed repositories were removed to make room. The observed free-space increase was 14917431296 bytes, subject to concurrent host activity and hardlinks. Current inputs and sources were preserved.

This is a manually specialized kernel with a fixed production Beta/Core handoff and trusted public development genesis. Funding-history checks do not authenticate real chain history or establish recursive PCD. Generic source-to-proof correspondence, source-owner/native-payer correspondence, authenticated funding/deployment, general contract/intent/transition/history proofs and Preview financial settlement remain open. No public transaction was submitted.

The archive includes every one of the 165 actual-result frozen file bodies, all 22 proof files, all 30 final verification files, additional original proof-gate history, the native Rust consumer source used by R4/R5, and the final three reviews and acceptance record. Large ELF, PK, SRS and dependency caches remain externally hash-bound; nested input manifests preserve their identities. Historical configurations and wrappers retain absolute paths and are evidence, not portable execution commands. A fresh clone can verify copied artifact integrity without native execution; it cannot independently rebuild and replay the whole native experiment from this packet alone.

From the repository root:

```sh
python3 wiki-llm/native-financial-kernel-2026-10-01/evidence/native-financial-r4-r5/check-archive.py
```

This command checks stored bytes and result invariants; it does not invoke a native prover, verifier or network service.

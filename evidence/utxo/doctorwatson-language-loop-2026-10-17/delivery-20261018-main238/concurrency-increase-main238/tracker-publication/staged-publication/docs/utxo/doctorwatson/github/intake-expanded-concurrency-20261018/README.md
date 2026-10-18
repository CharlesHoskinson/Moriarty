# Expanded current GitHub intake

Final readback through 2026-10-18T13:08:00.336646+00:00: new PRs**224,225 and227 are OPEN drafts** at the exact heads below, all based on main `e4d7a2b5709caa18a94f5256d03a0302364b8235`. Current utxo: `05ed1521e24608d898f7c78d0d0b3f04def0c6d9`. Earlier drafts220/222/223 remain open.

| Draft | Exact head / tree / membership | Observed result; remaining predicate |
|---|---|---|
| [224](pr-224-intake.json) | `ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75` / `3b5d75a115712b155b68398d83ce18cef78a3d87` /4paths | Capability query:2no-default tests53.793330s;1ordinary-opt-in test5.595689s. Exact source/result review remains pending; catalogue base revision is not binary/target attestation. |
| [225](pr-225-intake.json) | `97c9b1565d71404413aa35e9f95fe7661b8a09b1` / `e20de64d1e0ada6872b80ddd941857145305dfe0` /15paths | One integral absent-rate UMP01 test25.857098s,3filtered; complete12prefixes/seven effects/seven quantities.32logical units remain specified, not measured. Exact source/result review and full ACTUS predicates remain open. |
| [227](pr-227-intake.json) | `823ce081c00786a0c55b92af81009f9e7c5e6a16` / `aa593783a8717ac838ae556e24c187997ab3547e` /21paths | Production shared checker:8actual CLI tests22.639858s and8direct whole-stdout/exit matches against real baseline. Compatibility is not an independent financial/semantic oracle; exact source/result review and MCP remain separate. |

Source integration facts requiring explicit guards:224 and227 both change `crates/mori/src/commands/mod.rs`. The224 catalogue labels LSP/MCP as stubs at its actual base; integrating220’s LSP implementation must refresh that catalogue/tests. These are factual integration predicates, not source/security/merge approvals.

Append-only tracker proposal: [frozen20-item before](work-items-before.json), [complete23-item after](work-items-after.json), [exact append patch](work-items-append.patch). All original20 item fields remain byte-for-byte JSON values; only operational counts and the additive snapshot/new three items change. All**148 retained EARS (119+11+18)**, original acceptance fields, all13children and previous review failures remain. No operational PR identifier is a new product requirement.

Canonical before at captured utxo is Git blob `2f488c5928e66a342547846ccd8b4506e100f2b2`, SHA256 `38abab37fca328582fe9b00d98dfdec1c7cfed5b0df929d613c854898e6b3d47`; proposed after SHA256 `113a8506ea5c7b053aa34b2c69d94a0c7e4b119b7f54feeb553dfea81bd94533`. It is identical to the earlier frozen20-item proposal. Root must recheck live source/head/path guards and review before integrating.

[EXPANDED-INTAKE.json](EXPANDED-INTAKE.json) carries full predicates and guards; [API capture manifest](CAPTURE-MANIFEST.json) preserves full public PR/issue bodies and complete source/commit membership; [receipt manifest](RECEIPT-MANIFEST.json) binds15public receipts. Historical pre-run UNRUN snapshots, the227 guard failure/correction and prior DTO/review errors remain distinct from later observed results. No record is treated as an approval.

This lane performed no provider calls, web retrieval, product tests, Git mutation, comments, remote writes or shared/repository modifications. Root alone integrates.

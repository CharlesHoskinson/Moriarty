The frozen candidate adds supplied duty-frame reads and strengthens the existing escrow consumer.
All requested local checks passed. Fresh non-author Sol and Grok audits remain required.

Base: `97e77ea86d3832c642ebade4678f195b4e178775`.
Historical composed baseline tree: `4882c45fc4cf75604ac0c3a70f723a75d193a6d2`.
Current real and isolated candidate index trees: `1bca8244f7b9e5541afb3e863d2bb5ea6254ae8c`.
Full patch SHA-256: `a11a388a9dfbb49ebb4d6ffd28ece51e2316141227b4962b5581a4da5f519bdd`.
Owned patch SHA-256: `514e4d561cf4ddf5a48fc4e29d97dfe19f7f301bd8c808d1ce97904fc6ec2f76`.

The candidate contains 30 changed files and a 17-file duty-read delta.
This continuation changed no source bytes. Root staged the exact candidate files after the initial baseline verification.
Raw original index byte preservation is not claimed. The historical baseline tree remains recorded.
The report files are untracked evidence and are outside the candidate index and patches.

Core reads require one quoted name with an unambiguous source-wide identity.
Both reads inspect the current statement snapshot. Missing rows return false/0, while present-zero rows return true/0.
The ordinary compiler uses current constrained duty wires. Escrow guards refuse mismatched duties before financial effects.
The four CLI fixtures retain their complete post-state and ordered effects.
Ledger-call admission explicitly refuses these new reads.

| Fresh command | Result | Actual log |
|---|---|---|
| `cargo test --offline --locked -j 2 -p mori-backend --test replay --test replay_challenge -- --test-threads=1 --nocapture` | 16 tests passed (exit0) | [duty-reads-final-backend-replay.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-backend-replay.log) |
| `cargo test --offline --locked -j 2 -p mori-ledger --lib layout::duty_read_tests::ledger_call_admission_explicitly_refuses_duty_reads_before_native_work -- --exact --test-threads=1 --nocapture` | 1 tests passed (exit0) | [duty-reads-final-ledger-refusal.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-ledger-refusal.log) |
| `cargo test --offline --locked -j 2 -p mori --test enforcement_report --test replay_challenge -- --test-threads=1 --nocapture` | 14 tests passed (exit0) | [duty-reads-final-cli-integration.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-cli-integration.log) |
| `cargo run --offline --locked -j 2 -p mori -- test examples/core/escrow-recovery` | 4 manifest cases passed (exit0) | [duty-reads-final-cli-escrow-manifest.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-cli-escrow-manifest.log) |
| `cargo check --offline --locked -j 2 --workspace --all-targets` | check passed (exit0) | [duty-reads-final-workspace-check.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-workspace-check.log) |
| `cargo fmt --all -- --check` | check passed (exit0) | [duty-reads-final-fmt.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-fmt.log) |
| `git diff --check` | check passed (exit0) | [duty-reads-final-diff-unstaged.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-diff-unstaged.log) |
| `git diff --cached --check` | check passed (exit0) | [duty-reads-final-diff-staged.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-final-diff-staged.log) |
| `cargo test --offline --locked -j 2 -p mori-core -- --test-threads=1 --nocapture` | 60 tests passed (exit0) | [duty-reads-provenance-core.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-provenance-core.log) |
| `cargo test --offline --locked -j 2 -p mori-backend --test compile --test escrow_recovery --test liabilities -- --test-threads=1 --nocapture` | 30 tests passed (exit0) | [duty-reads-provenance-backend-affected.log](/home/charl/Moriarty/.moriarty-dev/runtime/language-wave1-20261014/duty-reads-provenance-backend-affected.log) |

Current total: 121 Rust tests passed, zero failed. Four escrow CLI manifest cases passed, zero failed.
Core contributes 60 tests. Backend compile/escrow/liabilities contribute 30, while backend replay/challenge contributes 16.
The ledger refusal unit contributes one test. CLI enforcement/replay contributes 14.
The ledger unit filtered 81 unrelated tests. No ignored or native suites ran.

The earlier RED logs retain the accepted mismatch and missing intrinsic failures.
The first final-check log retains one incorrect test diagnostic expectation. The final source corrects that expectation.
The second final-check log stops during backend replay. That interrupted aggregate receives no pass credit.
Fresh exact commands replace its missing command and exit provenance. Historical counts do not increase the current total.
Core and backend property tests emitted four proptest persistence-path warnings. Their actual test results passed.

| Measured ordinary artifact resource | Before guards | After guards |
|---|---:|---:|
| instructions | 1650 | 1798 |
| zkir_bytes | 101715 | 110759 |
| artifact_bytes | 208158 | 219157 |
| public_values | 271 | 271 |
| max_replay_steps | 25 | 29 |
| liability_registry_entries | 2 | 2 |
| max_liability_registry_entries | 16 | 16 |

Both artifacts report cost_measured=false, circuit_k=null and fraud_max_k=null.
These counters describe compilation and replay shape. They do not measure native proving or fraud cost.

Recorded limitations:

- Fresh non-author GPT-6.1 Sol high and Grok4.7 high exact-candidate source/result audits remain pending. This report does not grant approval.
- The prior Grok audit blocked on missing reports, interrupted test evidence and unstaged source. This report and current index address those evidence gaps. Fresh review remains required.
- Backend README line35 says effect descriptors do not change. Added guard statements shift effect statement ordinals. Public slot count and descriptor shapes remain stable, but descriptor identities do not remain identical.
- The compiled coherent amount mutation helper skips negative deltas when a valid closing duty amount is zero. It therefore omits those completion and refund-recovery shortfall mutations.
- The compiled coherent mutation helper exercises amount changes. Its name does not establish complete coherent absence mutation coverage.
- Scenario state, incoming duties, documents, recipient and clock remain supplied local premises. These reads do not authenticate ledger history or external facts.
- The ledger-call path explicitly refuses the new read leaves. This candidate does not implement native ledger-call support for duty reads.
- No proof generation, proving-degree discovery, SRS initialization, key generation, native proof verification, ledger apply, deployment or network dispatch ran in these checks.
- Core/backend/ledger/CLI source changes affect existing closure inputs. Historical native material and reviews do not qualify this candidate. No native source-closure recomputation or material reuse is claimed.
- Workspace validation is a typecheck of all targets. Unrequested broad workspace runtime tests, ignored/native tests and Quint correspondence were not executed by this continuation.
- No U/M/MC/SP milestone, native qualification, financial ledger settlement, PCD correspondence or release acceptance follows from these local results.

Every Cargo command used the shared build lock, offline locked dependencies, two jobs, and the same source checkout and target.
Rust test suites used one thread. This continuation ran no package clean or source checkout switch.
Cargo.toml and Cargo.lock retain their HEAD bytes and pinned revisions.
The prior author host identity is unavailable in this continuation. This report grants no source or result approval.

Full candidate file digests:

| Path | SHA-256 | Bytes | Duty-read delta |
|---|---|---:|---|
| `crates/mori-backend/README.md` | `eae59c7f15c0ef22abe1cbcd764c46be6b4e3b57a441109ad0cb53091038501f` | 5636 | yes |
| `crates/mori-backend/src/liabilities.rs` | `5706d938f5d7a0afe9fc58adb4ec192683d18c07f0924a70c7c1777e6814e10c` | 3905 | yes |
| `crates/mori-backend/src/lib.rs` | `4485b650e29e4c726721a54082fcf133b62891478f964e6780aa981dff2c61e0` | 44074 | yes |
| `crates/mori-backend/src/replay.rs` | `bd0356519e70bb68730afe1e63cab574f5c46a907b7fa4e8295f66f51296e126` | 14986 | no |
| `crates/mori-backend/tests/escrow_recovery.rs` | `7fc630d810bced200fbc4032ef40753fef1ae28aedd948fca56533470a0ecabc` | 20285 | yes |
| `crates/mori-backend/tests/replay_challenge.rs` | `a8c544db40a05afd836b5d56d773a754c1a73a45831c0743f8c97b089cb7de43` | 8304 | no |
| `crates/mori-core/README.md` | `831d97c015d29ff6ab3821f8d758e07e7302162a8645e1681d14a8b735f74461` | 5939 | yes |
| `crates/mori-core/src/ir.rs` | `9bc79d6446fcdbeb10f4cefbdf581ad95bd1460e7a2dd5af6081fd5e95197549` | 6982 | yes |
| `crates/mori-core/src/parser.rs` | `62f07ce1b28d119b64712e120bfd370c5fd10b7a96e9fe6b77c87dc47179c90a` | 12453 | yes |
| `crates/mori-core/src/runtime.rs` | `148f3937874119e3cbe066adbbab31b22124c64676f2d8f43fb181cbe5370b40` | 32703 | yes |
| `crates/mori-core/tests/escrow_recovery.rs` | `1deb72e26bd9766bf5b70986938cab25bdf72646be32bbb5dee796c0d430746c` | 15436 | yes |
| `crates/mori-core/tests/language.rs` | `50a5ee441a4f71c1c86ad5c5ce18e5dfe06d056bb2d82d2d6bd3e17dd359a8ad` | 13255 | no |
| `crates/mori-core/tests/liability_reads.rs` | `c4a084774e52129fee31a02f1860e8fb1ae59597483744241105dbd74eb45cc0` | 7120 | yes |
| `crates/mori-core/tests/liability_registry.rs` | `fd9c6755ab481a27e6780bf626efbaaacfec0e05f1d73127d5fb3a82b4a04d7e` | 9707 | no |
| `crates/mori-ledger/src/layout.rs` | `3623052b7bb15fcd6938c38d657ae745c1fd09afdfee4d205ebf7cc2a5ad0005` | 23877 | yes |
| `crates/mori-ledger/src/ops.rs` | `c93d938223e2a9647f223a670285a8941ccfcff1c1bb5f87136cc11644c170a7` | 26900 | yes |
| `crates/mori-ledger/src/relation/compiler.rs` | `b7805ead687bb015dbd6b5391348f86e11f778ba5b310632f23c90341d6b88dc` | 26093 | yes |
| `crates/mori/src/commands/inspect.rs` | `134486a956869ce26810f1e87a9eaa5c90c279e6752b5c24e60005df7d6d5127` | 18683 | yes |
| `crates/mori/src/commands/replay.rs` | `41c907d748aaff6619c4087af1c4f16622cf3f4e6df6d51fa18d57f7d56e04d0` | 6664 | no |
| `crates/mori/tests/enforcement_report.rs` | `14b5bec3ff14cd3e99a1741e4f753ba1f06d95f7a75919d0e1136a4916dc7c0d` | 14989 | yes |
| `crates/mori/tests/replay_challenge.rs` | `925aa4b1a2faefdb530ee412dbb0c8278d8d6b1ebed9c57928285bce44bd277a` | 13402 | no |
| `docs/NATIVE-CORE.md` | `c64192feb61b15fd52d9acf4d6225a4939a5ad1f2297f52b70632f0e3d97ef65` | 27053 | yes |
| `examples/core/enforcement-report/source.mori` | `0db5c8082782f426fe91f4cae913eda1a2c3893c8d9d75e8613d347395df55ce` | 922 | no |
| `examples/core/enforcement-report/target.json` | `6640aff4f6003524f1cb6d0f951a92005e673a49ad750ee8883692498888add1` | 251 | no |
| `examples/core/escrow-recovery/complete.json` | `bfc067ef2044b862bd3c4fafb58ef1f8430f716f16836ae46a537816a44f5b22` | 3156 | no |
| `examples/core/escrow-recovery/escrow.mori` | `2d7bfc32f9568cc673eabbcf43dfb750426e4f7c5bedab9eebd67ce4dce773c1` | 6431 | yes |
| `examples/core/escrow-recovery/fund.json` | `a8b01269282364afc13420c1e4047dfcaf6945749d9c5deaa33b75b395205519` | 2059 | no |
| `examples/core/escrow-recovery/mori-tests.json` | `129dfbd7efbd35a2e441128e8250da9754ee569162d892182e29d2c61e6c1a35` | 29990 | no |
| `examples/core/escrow-recovery/partial.json` | `49aef829b12b8c5086990e2f4264d9ee53d2bdc8afeaa0c8d1d3c700df986fdf` | 3221 | no |
| `examples/core/escrow-recovery/recover.json` | `d50bb3b23105b83d7c1d52a5352bec528fdb871f97cbff00265fa51b452fc553` | 3155 | no |

The JSON report contains complete command records, observed suite counts, historical results, input digests and artifact digests.
The runtime and worktree report copies have identical bytes. Neither report is staged.
No commit, push, milestone closure or native qualification is claimed.

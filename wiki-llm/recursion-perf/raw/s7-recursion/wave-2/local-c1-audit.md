# C1 local implementation and recorded evidence audit
> Source: file:///home/charl/code/Moriarty-perf
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Checkout HEAD: `865ad2dcfa1fc2e0e8c40ad0809b16d55764fbdd`. File hashes pin inspected working-tree contents; no fresh synthesis or timing was run.

| Source file | SHA-256 |
|---|---|
| `crates/mori-backend/src/sherlock_machine/literal_bridge.rs` | `fd3385a267d4b386e39c7242d5082482d9c3ec67a08daccdb10f7879d08faec5` |
| `crates/mori-backend/src/sherlock_link.rs` | `48e3aa08eb7c48b2e640ae88ce6ca7865d546b58754337811e6b6e46c5217501` |
| `crates/mori-backend/src/sherlock_child.rs` | `34865218c90faffb0f0820e289800056d6f905bb1bf611a234e0d13a2416cb89` |
| `crates/mori-recursion/src/parent.rs` | `552d089556dad9e4480848910369c69cb1177785b10bd9e34b6434c3ebf09fb4` |
| `crates/mori-recursion/src/decider.rs` | `f7733c6439c66b39dd594349438f24550cd6776d7366b17e053849fb057aae32` |
| `crates/mori-recursion/tests/sherlock_parent.rs` | `0d30d09520a642eabb80cfc30f7d3f7f965031e08dbdfa4e7834cf64052759e1` |
| `crates/mori-recursion/tests/sherlock_terminal.rs` | `7d29f699b1261adab51b4850d0f145bd972f85dcf9bbb77afd73f1be79a77415` |
| `crates/mori-recursion/tests/fixtures/stagenet-verifier.json` | `1a253a8988fd0d22406ed26858e0210500221e71a40ef0f84a14b4273e245798` |
| `crates/mori-recursion/tests/fixtures/accept-template.json` | `d7a9f218630cfc1f25e4f474927a671755b929501171e3fe3121351f5bd29207` |
| `docs/results/sherlock-native-a102-a80-child-producer-20261021/actual/RUN-REPORT.json` | `656c5a0f71e3bd19f49eb16312be138dd2e7894160acc232e58d663cde07b18a` |
| `qa/sherlock/runs.jsonl` | `d872f07534775c7b5009f0bde9aa5b75858b85fa0edab039f18fa2247b5a2b36` |
| `models/doctorwatson/factory/cli.py` | `d2104f8a8c148a966ee3ba9cd4c0ecfc4ac571be714e8cd62a603e95222287b3` |

SOURCE — The recorded 2026-10-21 RUN-REPORT has A102 k13, A80 k12, 3,440 proof bytes and 253 instance lanes per child, P=86 and S=81. It does not record advice rows, table rows, columns or degree. Its qualification is only the scoped literal/return producer-consumer run; it is not full interpreter correctness. Its capacities differ from the current A3 test (that test builds Literal(UInt(7)) with explicit larger capacity arrays and stack_limit=1). Reproduction must pin one identical fixture in both candidate and baseline rather than silently transfer the old geometry.

SOURCE — literal_bridge.rs:69–160 and sherlock_link.rs:390–480 assign separate public operands, constrain context limbs, derive a rule selector, force that Boolean selector to one, invoke the original semantic fragment and emit an Output terminator. Direct composition needs a private shared carrier around the original fragments; concatenating these two emitters retains two public interfaces/terminators. sherlock_child.rs:240 onward constructs the fixture Preprocessed carrier from the known statement lanes. This is not evidence that arbitrary private wallet witnesses are available.

SOURCE — parent.rs:86–108 defines SHK_KEYS/SHK_PROG/SHK_STATE/SHK_TRACE/SHK_STMT and 16 statement fields, in order: keys_id, a102_ctx_lo, a102_ctx_hi, a80_ctx_lo, a80_ctx_hi, program_c, pre_root, h_pre, mid_root, post_root, h_post, bind_a102, bind_a80, n_steps, work, trace_c. parent.rs:454 derives keys_id from both old child VK digests and shape and assigns it as a fixed circuit constant. This identity can name the source semantics in a replacement, but does not assert existence of the old proofs.

SOURCE — parent.rs:569–724 aliases the A80 program to A102 program and A80 pre to A102 post, pins contexts/envelopes (3,1)/(0,0), prepares two proofs, accumulates once, resolves fixed bases and collapses once. Its comment explicitly defers pairing. It computes one domain-prefixed program hash and three state hashes, bounds h_pre and h_post below 2^64, derives h_mid=h_pre+1 and h_post=h_pre+2, fixes n_steps=2 and work=1, and hashes 21 ordered trace fields. Binding lanes are exposed, not recomputed as SHA-256 source claims. The parent exposes 16 fields plus 12 accumulator lanes.

SOURCE — decider.rs:172–270 verifies the parent and its carried accumulator, reassigns subgroup-checked final points, binds their encodings and both unit scalars, and invokes PairingGadget. It hashes [DOM_STMT, instance_id, batch_no, selected 16 fields], then constrains the communication commitment as Poseidon(comm_rand, stmt) in BoundPoseidonStmtV1 mode. The pinned parent constructor uses bound mode. Public slots are 15: binding_input at 0, communication_commitment at 1, statement at 10, fixed constants elsewhere. The binding input itself is a public call value, not a substitute for the communication opening or anchor state checks.

SOURCE — sherlock_terminal.rs chooses `prove::<DeciderRelation, blake2b_simd::State>` and verifies with the same host transcript. It calls ledger VerifierKey::verify with embedded PARAMS_VERIFIER and round-trips tagged verifier bytes. The circuit architecture enables BLS and Poseidon, not a Blake2b chip for the outer transcript. stdlib 2.3.5 public source lines 1984–2039 independently shows transcript generic H is passed to host BlstPLONK proving/verification.

SOURCE — Installed A4 mode reads the on-chain contract operation acceptValidity and requires raw verifier hash equality with the fixture. Before SRS/model/keygen it checks the supplied parent VK against parent_vk_sha256; before keygen it checks relation_sha256; it later requires generated VK bytes to equal the installed bytes. Offline `prepare_terminal_proof_for_deployment` deliberately does not establish installed-key equality. The fixture schema denies unknown fields and currently requires address, vk_sha256, parent_vk_sha256 and relation_sha256. A direct terminal cannot silently reinterpret parent_vk_sha256 as a fused-relation hash.

SOURCE — The local fixture records address ac93b80e252f9cde794d22f76b2eb7d727f855f07ad77a0904afe5bac855388b; VK hash 3f3b7e131e83101258ef080820362941218adf6d59cf6574c682267a01ff0083; parent VK hash 801467c825c79e063c0603b94c49eef63fd527ff8b3a6d42aa24022b108849a8; relation hash 86dfecc6d4a09d10cea0508341bb435eeff32265b52b623fe84289aa91577905. These are file contents, not new live-chain readback.

SOURCE — A3 authenticates official bls_midnight_2p20 before RawBytesUnchecked parsing: length 201326980 bytes, SHA-256 1cc62978558fdc1e445cd70cfd9a86ec3c2e2151b6d74811232d37faf9133ff1. Its k20 assertion makes it a baseline test, not a candidate harness. Candidate smaller-k parameter conversion must preserve that setup provenance and pass the ledger's independent verifier.

SOURCE — factory/cli.py accepts `heavy --id ID --budget KEY --max-wall SECONDS [--hypothesis TEXT] -- COMMAND`. Heavy/check write qa state/traces outside this research task's permitted paths, so neither was executed. Future experiments need their own measured budgets.

SOURCE — Selected existing run records below are observations, not new measurements. Hypotheses in the original records are hypotheses, not diagnoses proven by this audit.

```json
{"run": "20261025T000955Z-A3", "id": "A3", "exit": 0, "wallSeconds": 315.88, "maxRssBytes": 15760023552, "head": "865f064023351c5b4da448cc953747edc1cba908", "hypothesis": "The ported parent and native test compile on stock0.8.3; explicit BLS12 SRS typing fixes the build error, and preflight is now clear.", "trace": "qa/sherlock/traces/20261025T000955Z-A3.log"}
```

```json
{"run": "20261025T012355Z-A4", "id": "A4", "exit": 101, "wallSeconds": 15.338, "maxRssBytes": 784191488, "head": "f19f77e83de5b6f7f3517543f93ee1ce28bf3121", "hypothesis": "The archived installed parent is afc4ec3a while current A3 is801467c8; reject that exact mismatch before keygen and preserve the installed verifier requirement", "trace": "qa/sherlock/traces/20261025T012355Z-A4.log"}
```

```json
{"run": "20261025T012424Z-A4prepare-corrected", "id": "A4prepare-corrected", "exit": 0, "wallSeconds": 290.552, "maxRssBytes": 16587382784, "head": "f19f77e83de5b6f7f3517543f93ee1ce28bf3121", "hypothesis": "Offline compiled SherlockAnchor capture proves slot5 must be0; corrected relation86dfecc6 has been compared before keygen and now needs a fresh matching key and real proof", "trace": "qa/sherlock/traces/20261025T012424Z-A4prepare-corrected.log"}
```

```json
{"run": "20261025T024457Z-A4", "id": "A4", "exit": 0, "wallSeconds": 324.739, "maxRssBytes": 16487976960, "head": "6d752e7fbd30da16e5db7e9b78520121b623b107", "hypothesis": "The owner-authorized replacement verifier now installs the current parent/template decider; independent indexer state and Rust raw-key readback match the new pins.", "trace": "qa/sherlock/traces/20261025T024457Z-A4.log"}
```

```json
{"run": "20261026T044131Z-A3", "id": "A3", "exit": 0, "wallSeconds": 382.278, "maxRssBytes": 15723253760, "head": "c7d3c48a3d998a16112cab5f4a6aa621a47fa79e", "hypothesis": null, "trace": "qa/sherlock/traces/20261026T044131Z-A3.log"}
```

```json
{"run": "20261026T044937Z-A4", "id": "A4", "exit": 101, "wallSeconds": 2.201, "maxRssBytes": 88272896, "head": "dbffef95ac7995ad9471e1daaa892a28e4183f1f", "hypothesis": null, "trace": "qa/sherlock/traces/20261026T044937Z-A4.log"}
```

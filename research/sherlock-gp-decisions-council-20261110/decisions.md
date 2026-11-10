# Council decisions: Sherlock general-purpose recursion, D1-D4

Run `sherlock-gp-decisions-council-20261110`, 2026-11-10. Advisory under the council protocol. The owner keeps the final call. Nothing here is a test result, an audit verdict or a merge decision.

## Summary

| Decision | Selected option (short) | Status |
|---|---|---|
| D1 Genesis | In-circuit `Init(prog_d, input_d)` check over the committed register state on the first segment leaf (`h_pre == 0`), seal requires `h_lo == 0` | Blocked by dissent (1 `changes_requested` of 4) |
| D2 Leaf | Chiplet leaf `U` as a separate circuit: register trace plus PROG, memory, 128-bit and Poseidon chips in one proof, single-phase native `lookup_any`, first target `k_U <= 19` | Blocked by dissent (4 of 4 `changes_requested`) |
| D3 Wording | (i) tie 2-2 between an owner amendment and the literal reading; (ii) meet GP-3 as written, no narrowing; (iii) keyed Blake2b ledger bytes required, Poseidon re-proof is development evidence only; (iv) no depth-32 cap, variable or expandable depth | Blocked by dissent (4 of 4 `changes_requested`) |
| D4 Placement | Separate leaf plus lifts, core keys `{U, R, F, N, C, D}` | Blocked by dissent (3 substantive verdicts from 3 families on the decision, 2 `changes_requested`; only 2 substantive verdicts are for the selected option itself) |

No decision closed. Per the protocol, no second round was run. Every `changes_requested` finding below is a correction requirement for the owner or lead.

## Owner direction for D2 (recorded)

Owner, 2026-11-10, verbatim: "Alright really lean into the chip design and let's get it modular and concise". The owner's preferred direction for the leaf is a modular chiplet design: a small core trace with registers, and specialised chips on a lookup bus. The council's job on D2 was how to realise it on this stack and how it interacts with D1, D3 and D4, not whether to use it. This direction reached the council after the proposal prompts were sent. Proposals saw only the earlier owner addition (chiplets as a required candidate); the review prompts carried the direction. No reviewer argued against the chiplet direction. The only dissent concerns where the chips live (D2/D4).

## Round record

- Bundle: repository HEAD `33be3180aad268899a63aee01601f59bbf6a848e` at launch; HEAD moved to `6853d2588bcce2357751c6d13a396d6521ed157c` during the round by commits outside this council (the brief was committed). There is no base..head diff; the bundle is the brief plus the measurement commits `5782173bd` and `7fadf00c3` (branch `worktree-agent-acf90d07b6b60767c`; its locked worktree had moved to `79a3fad8c`, so lanes were told to cite commits). `BRIEF.md` sha256 `c2608ac00d5eb9105af060de43de3bf03f30b463304ad62e3e8a4adc55099330`. The working tree was dirty throughout (`qa/sherlock/status.json`, untracked `.checkpoint/`, `qa/sherlock/profiles/`) from other jobs; `git diff | sha256sum` stayed `fdf290ff…7274` before and after every lane, so no lane changed tracked files.
- Smoke: `codex` gpt-6-astra xhigh, answer `2`, exit 0. `grok` grok-4.7, `2`, exit 0. `muse` muse-wonder-full-1.0 (wire model confirmed), `2`, exit 0. `agy` one-shot `--print` failed (print timeout at 4 min with zero tokens); a 10 min retry exited 3 but is inconclusive, because its output files were lost when the lead moved the smoke folder during the run; five other agy processes from another project were running. An `argon-run` smoke wrote the correct answer with model `gemini-4-argon-high` confirmed in the log, but the turn did not return a result before the 15 min timeout. Classified as infrastructure (contention), not a verdict. The real Google lanes then completed through `argon-run`.
- Proposals (independent, read-only on the checkout): OpenAI (`codex`, gpt-6-astra, xhigh, `--search`, read-only sandbox, 11 min); xAI (`grok`, grok-4.7, xhigh, read-only sandbox, web on, 14 min); Meta (`muse`, muse-wonder-full-1.0, xhigh, `--disable-write`, workspace untrusted so AGENTS.md was not loaded, 4 min); Google (`argon-run`, gemini-4-argon-high, 31 min, wrote only its scratch output). Outputs went to a scratch folder and were copied into `proposals/` only after all reviews were classified.
- Deviation: the xAI proposal lane read `research/sherlock-processor-analogy-20261110/study.md` (a separate study, not a council input) because it was readable in the checkout; that path was added as an exclusion to the Meta and Google proposal prompts, which were not yet sent. It did not read any other lane's work.
- Deviation: the owner's D2 direction arrived after the OpenAI and xAI proposals had started; to keep proposal prompts identical in substance it went into the review prompts only.
- Blinding: transport preamble before `# Proposal` stripped (xAI only); no other text changed. Repository paths containing `.claude/worktrees` were left as they are data. Identity map: `_private/identity-map.json` (written after classification; never in a review input).

| Candidate | Family | Blinded sha256 |
|---|---|---|
| `cand-d600` | OpenAI | `b703ef3f443a691505d59a803b7fca1617fffc58ad2bdffb0a2997cb9ab35a40` |
| `cand-f429` | xAI | `4ad8f5791e1ec21af768eef5725e851fa838e2e3856cdf014a77d687be9b86ab` |
| `cand-fd2b` | Meta | `79f04a51d0cf09491a7ca343430f5b15d64c97914946d3fd1945c3a5cfdff847` |
| `cand-0c40` | Google | `7d481d1a0b4f2d1efdd362048fe8fb4aece1e90ef343385b97eb8f45bed03e09` |

| Review | Family | Candidates seen | Prompt sha256 |
|---|---|---|---|
| `rev-bfd3` | OpenAI | 0c40, f429, fd2b | `ccab45bb805890335d2ad9de587e7d362722b0bde5b9e4708fd8dac220bbcee3` |
| `rev-64d8` | xAI | 0c40, d600, fd2b | `617fbc22d49d60157285e41bb1ade98aa91c447a3db1d70d846ae53afd55dac0` |
| `rev-2756` | Meta | 0c40, d600, f429 | `0e2a3f19de16a37f4614b8ef469f08eb24e2c894cbbac68878786e7a4183ecc3` |
| `rev-2a3a` | Google | d600, f429, fd2b | `23a0ddc77b3bcc51b66421dcfd611ced9b625bf0946593591a081b86ba4a77cb` |

- Isolation check (after the round): every lane's tool log or session history was grepped for paths. The Google lanes (cwd in the lead's scratch folder, where other lanes' outputs and later the identity map lay) read and wrote only their own lane folder, the brief and repository files (tool calls: `read_file`, `run_command` for `git show` and `jq`, `write_file` to their own output). The OpenAI, xAI and Meta lanes touched no scratch-folder path and, inside the council folder, only `BRIEF.md`. No lane read `_private/`, `blinded/`, `review-prompts/` or another lane's proposal. HEAD at the end of the round: `0b7369c862292995c353c5bd6f16ad14f4dacdc9` (moved by other jobs); `git diff | sha256sum` still `fdf290ff…7274`.
- Transport classification: all four reviews completed (exit 0, terminal status completed or end_turn, one JSON object in the required shape, every finding with a location and a next action). The xAI review text had a 194-character narration before the JSON; the JSON itself was complete and is stored as `reviews/rev-64d8.json`. All four are completed-valid. No retries were needed.
- Tally rule: fixed in `tally-rule.md` (sha256 `71ed180f…543d`) before any review output existed. Picks are tallied by option, not by candidate id.

## D1 Genesis mechanism

**Selected option.** The brief's option (a), realised over the committed register state rather than over carriers: when `h_pre == 0`, the first segment leaf proves in circuit that its fixed register file and its commitment roots equal `Init(prog_d, input_d)`. `prog_d` commits the normalised program table (root) plus entry metadata and machine version; `input_d` commits the run input, the initial fuel (u128) and the stack limit. `Init` sets control to `Eval` at the entry opened from `prog_d`, initial and remaining fuel and the stack limit from `input_d`, a pending outcome, empty frame and helper-task links, canonical initial roots for the memory tables, and empty `deps_d`. The seal rule requires `h_lo == 0` and a terminal outcome. Equality with a supplied `init_d` alone is not the check. Off-chain recomputation (option b) is a diagnostic only. (cand-f429 labels the same construction "(c)".)

**Votes.**

| Review | Family | Pick | Option | Verdict |
|---|---|---|---|---|
| rev-bfd3 | OpenAI | cand-fd2b (Meta) | in-circuit committed-state Init | approved |
| rev-64d8 | xAI | synthesis cand-0c40 (Google) + cand-fd2b (Meta) | same | approved |
| rev-2756 | Meta | cand-f429 (xAI) | same | approved |
| rev-2a3a | Google | synthesis cand-fd2b (Meta) + cand-f429 (xAI) | same | changes_requested |

Not picked: cand-d600's (OpenAI) streaming `INIT` computation across chunks.

**Status: blocked by dissent.** Quorum met (4 substantive verdicts, 4 families). One admissible `changes_requested`.

**Dissent (rev-2a3a, Google), in substance:**
1. cand-fd2b and cand-f429 take only fuel and stack limit from `input_d` and set every memory root empty, so they never say how a non-empty run input (required by GP-1 acceptance, `general-purpose.md:11,16`) enters the initial environment or arena. Next action: define `input_d = H(IN, input_val_d, fuel, stack_limit)` and bind the input commitment into the initial environment/arena root or a dedicated input register at `h_pre == 0`. Evidence: `reference.rs:14-27` starts from an empty environment.
2. The genesis state omits three of the four append-only tables (`data`, `data_fields`, `env`, `value_slots`) and the helper (23-lane) and result (4-lane) records. Next action: include all four table roots and lengths, both hash-stack roots, the helper/result registers and `deps_d == EMPTY_DEPS` in `Init`. Evidence: `c6_phys_data.rs:11-18`, `owner_runtime.rs:3513-3564`, `c6_phys_helper.rs:62-70`.
3. cand-d600's multi-chunk streaming `INIT` adds parser and continuation surface; commit the normalised program as `prog_root` inside `prog_d` and check only the entry opening and O(1) register/root initialisation.

Related cautions from approving reviewers: rev-bfd3 asks that the `Init` mapping preserve u128 fuel, both fuel registers, empty environment/frame semantics and the Pending outcome; rev-64d8 and rev-2756 specify fuel as u128 (verified: `remaining_fuel: u128`, `initial_fuel: u128` at `crates/mori-core/src/sherlock/reference_types.rs:168-169`; cand-0c40's u64 is wrong).

**Kill criterion.** Reopen if a fresh witness can satisfy `h_pre == 0` with a non-`Init` register, a non-empty store, a fuel or stack limit outside `input_d`, or a Running outcome at seal; or if the check needs a scan whose rows grow with the program image or adds more than 5% (ESTIMATE) to the leaf rows. Only in that scan case does a multi-chunk `INIT` continuation on the same key, with owner-step height held at 0, replace the single-leaf gate.

**First experiment.** Host-only differential, no prover: for `literal`, `add` and fact(3), build the `Machine::new` snapshot and the proposed `Init` openings (including input value, all four tables, helper/result registers) and compare. Then `cost_model` the gated check in the D2 slice with the gate on and off, and refuse two malicious witnesses (non-canonical u128 fuel at `h_pre == 0`; an `Init` image reused at `h_pre > 0`). Do not read E3's whole-carrier number as this cost.

**Correction (lead, 2026-11-10, after the round; not council text).** The `changes_requested` findings 1 and 2 are addressed in `openspec/changes/sherlock-vm-consensus-20261107/design.md` §3.2(5), SRC-021. `Init(prog_d, input_d)` is defined field by field (39 fields) from `Machine::new` (`reference.rs:14-27`) and the helper-machine `genesis()` (`owner_prepare.rs:2760-2800`), mapped into the chip registers of `research/sherlock-processor-analogy-20261110/study.md` §1.3. Run input: `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)`, where `val_d` is the canonical value digest (the GP-1 result encoding). A `Main(in_fn, in_ty)` program starts at `Eval(ENTRY)`, which applies `in_fn` to `Literal(x)`; the cells of `x` are the initial DATA and FIELDS prefix; the reference side is `Machine::new(apply(prog, x), ...)`. All four tables, both stack links, the helper and result registers and `deps_d` are fixed. Finding 3 (no streaming `INIT`) is followed: `Init` is O(1) in the program and input size. Model: `models/sherlock/quint/init.qnt` (12 of 12 tests pass, 47 of 47 mutants caught). Host differential: task 4.15, row A37. The owner keeps the final call; the status above is unchanged.

Repository facts checked: the node forces `prev_d == init_d` only when `prev_h == 0` (`crates/mori-recursion/src/ivc.rs:498-501`); the contract constructor checks only non-zero digests (`tools/sherlock-sdk/sherlock-run.compact:29-35`); SRC-021 already forbids counting equality with a supplied `init_d` (`requirements.md:43`).

## D2 Leaf architecture

**Selected option (realisation of the owner's chiplet direction).** A separate universal chiplet leaf `U` under one fixed key: a small register trace of `1..=S` owner steps (S from {16, 8, 4}, the largest that measures) with opcode/rule transition constraints, and four chips in the same circuit (PROG ROM, memory, 128-bit arithmetic/range, Poseidon), joined by the pinned library's native single-phase `lookup_any` and permutation arguments. The bus closes inside `U`; only the register digest and the program and memory roots cross segment boundaries. The memory chip enforces address uniqueness, allocation freshness, ordering and read-after-write, and authenticates cold reads and boundary updates against the roots; frames and helper tasks are re-encoded as linked write-once entries or hash chains with tops in registers. `U` is a custom halo2 circuit (or a specified stdlib extension), not a ZKIR child, and is verified by the run lift `R`. Reviewers split on this point: rev-64d8 and rev-2a3a say "a `Relation`", rev-bfd3 and rev-2756 say a plain `Relation` cannot add the tables. The lead checked the pinned source: `MidnightCircuit<R: Relation>` configures through `ZkStdLib::configure(meta, params)` with the chip set from `used_chips()` (`midnight-zk-stdlib-2.4.0/src/lib.rs:1787-1853`), so custom chiplet lookups need a custom circuit or a stdlib extension. First target `k_U <= 19` within the current `MAX_K` and `S_MAX`. No LogUp in the first realisation. Expected `k_U` and per-step cost are ESTIMATE only (reviewer bands: 16-19); G-C is a later measurement, not met.

**Votes (by placement).**

| Review | Family | Pick | Placement | Verdict |
|---|---|---|---|---|
| rev-bfd3 | OpenAI | synthesis 0c40 (Google), f429 (xAI), fd2b (Meta) | separate `U`, native lookups, `k_U <= 19` | changes_requested |
| rev-64d8 | xAI | synthesis 0c40, d600 (OpenAI), fd2b | separate `U`, `k_U <= 19`, S in {16,8,4} | changes_requested |
| rev-2a3a | Google | synthesis fd2b, f429, d600 | separate `U`, `k_U` 19-20, T 64-256 | changes_requested |
| rev-2756 | Meta | synthesis d600 (OpenAI), f429 (xAI) | chips inline in node `N` at k=20 | changes_requested |

**Status: blocked by dissent.** Quorum met for the selected placement (3 substantive verdicts, 3 families). All four verdicts are `changes_requested`.

**Dissent and findings, in substance.**
- Option dissent (rev-2756, Meta): put the chiplet processor inside `N` at k=20 with a single `lookup_any` bus; separate leaf plus lifts only as the fallback if the cascade probe fails. Its findings: the 265,904-row headroom (996,147 − 730,243) comes from a lookup-free two-self probe; chiplet commitments enlarge `vk_N`, both in-node verifiers and the decider (already 929,166 rows), so the row total is an unmeasured fixed point; no candidate shows custom lookup tables are expressible in a stdlib `Relation`; set inclusion needs address uniqueness and per-row authentication against `prog_d` and the memory root (fixed columns would make the key per-program); G-C is claimed on unmeasured numbers, which SRC-011 forbids.
- rev-bfd3 (OpenAI): an ordinary stdlib `Relation` cannot install arbitrary chiplet gates or `lookup_any`; implement `U` as a custom halo2 circuit with an explicit constraint system verified by `R`, or specify the stdlib extension. LogUp needs its own challenge schedule and a recursive-verifier extension. Set inclusion does not give uniqueness, freshness or read-after-write; specify tagged request tuples and test conflicting duplicates, reads before allocation, omitted writes, substituted roots. Neither sizing estimate is established; raising `MAX_K` alone does not enable a k=20 leaf (`OfflineParameters` pins degrees 0..19 only); T=256 conflicts with `S_MAX = 16`. G-C is not demonstrated: 19 `U` + 19 `R` + 18 `N` + `C` + `D` is about 2,697 s serial (ESTIMATE) before setup and verification, and cand-0c40 counted only 19 recursive steps.
- rev-64d8 (xAI): cand-0c40's 66,552-row, k=17, 5-9 s budget uses a 73-row Poseidon figure and nonexistent vendor trees; the pinned chip is width 3, rate 2, 8 full and 60 partial rounds with `NB_SKIPS_CIRCUIT = 5` (18 round rows per permutation as a lower bound). cand-fd2b's T=256 exceeds `S_MAX = 16` and k=20 exceeds the leaf cap; raising either is a protocol change.
- rev-2a3a (Google): LogUp needs a second challenge phase the pinned verifier gadget does not have (it asserts one phase); use `lookup_any` keyed by a unique `(clk, slot_id)`. Inline-in-`N` placement inflates `N`'s VK commitments, adding variable-base MSM work twice in `N` and once in `D`; cand-f429 counted 19 node proofs where parallel base segments plus a binary join tree need 37. Enforce monotonic allocation counters on the four append-only tables; frames/tasks as hash chains. A k=20 leaf needs `PINNED_HASHES` extended with the `bls_midnight_2p20` hash, not only `MAX_K` raised.

Repository facts checked for this section: `MAX_K = 19` (`crates/mori-backend/src/proving.rs:22`); `PINNED_HASHES: [&str; 20]` for degrees 0..19 and the "No pinned SRS for the circuit degree" error (`crates/mori-backend/src/parameters.rs:13-16,44-48`); `S_MAX = 16` and the `steps - 1 < S_MAX` check (`crates/mori-recursion/src/ivc.rs:77,368-372`); `lookup_any` at `tools/profile/vendor/midnight-proofs-0.8.3/src/plonk/circuit.rs:1889`; the verifier gadget's one-phase assumption (`midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:16-20`, `assert_eq!(cs.phases().count(), 1)` at :308-310); Poseidon constants (`midnight-circuits-7.3.0/src/hash/poseidon/constants/mod.rs:16-26`, `poseidon_chip.rs:50`); pins `midnight-proofs =0.8.3`, `midnight-circuits =7.3.0`, `midnight-zk-stdlib =2.4.0` (`Cargo.toml:43-45`); only `midnight-proofs-0.8.3` is under `tools/profile/vendor/`; two-self probe 730,243 rows, three-slot 1,078,147 (`qa/sherlock/decisions/speed-levers.json:26-29`); decider 929,166 rows (`qa/sherlock/manifests/run-decider.json:20`).

**Kill criterion.** Abandon this packing if the S=16 slice, after the memory chip coalesces accesses, still has `k_U > 19` and S=8 and S=4 also miss `k_U <= 19` or the 48 GB process cap; or if a memory witness can break read-after-write or publish a root that disagrees with the net writes; or if `R` over `U`'s key exceeds 996,147 rows. If the separate placement fails and the same chiplet slice fits beside two hardened predecessor verifiers in `N` with `D` under 2^20 over the new node key, move it inline (Meta dissent). Per-rule keys and inline copies of today's fragments stay excluded by the measured k=23 rule rows and the 730,243-row two-verifier baseline.

**First experiment.** Model only, no SRS, no proving: (1) for the selected separate `U`: the gadget in `R` verifying a `U` key whose constraint system carries a `lookup_any` argument into a fixed table (for the inline dissent: the gadget and the `lookup_any` in the same circuit), to confirm the constraint system synthesises and the gadget accepts it; (2) host census of per-step table reads/writes and hash calls over the fact(3) (293 steps) and list123 (303 steps) reference traces; (3) `cost_model` of a minimal `U` with a 16-step register trace, one `lookup_any` bus, one permutation memory region with authenticated old-cell reads and fresh writes, one stack rewrite, one Poseidon permutation, one 128-bit operation and one PROG lookup, plus `R` verifying that `U`'s key. Then extend to the eight fact(3) rules (one match, one call/return) and extrapolate rows per owner step.

## D3 GP-3/GP-4 wording points

**Status: blocked by dissent.** Quorum met (4 substantive verdicts, 4 families), all `changes_requested`. Per point:

### (i) Two prior proofs plus new work

**Selected option: tie, 2-2.**
- Owner amendment (binary composition): rev-bfd3 (OpenAI) recommends an explicitly marked amendment for the separate-leaf design while the literal text stays in force until adopted; rev-2a3a (Google) adopts cand-fd2b's amendment text.
- Literal reading, no amendment: rev-64d8 (xAI) counts the node's compliance predicate (shape binding, seal, dependency discharge, claim-set union) as the new work, with VM segments entering through `R` and foreign proofs through `F`; rev-2756 (Meta) requires nonempty inline processor work beside two enabled predecessors, gated on the D2 cascade probe, with the amendment pre-drafted as a conditional fallback.
- Dissent in substance: rev-bfd3 holds that counting routine join/seal bookkeeping as "new work" is not established and that two slots holding prior proofs cannot also hold a new-work proof; rev-64d8 holds that cand-fd2b's replacement bullet narrows GP-4 and should not be adopted. These two positions are opposed; the owner must choose. The two literal votes are different constructions: Meta's needs VM work inline in `N`, which the selected D4 option does not have. Under the selected D4 option (separate leaf plus lifts) the split is therefore 2 for the amendment (OpenAI, Google) and 1 for the compliance-predicate reading (xAI).
- Facts: GP-4 bullet at `general-purpose.md:46`; the three-slot node is closed at k=21 (`general-purpose.md:52`; 1,078,147 rows at `speed-levers.json:27`).

### (ii) Circuits outside the Midnight stdlib configuration

**Selected option: meet GP-3 as written, no narrowing** (rev-bfd3, rev-64d8, rev-2756). Each foreign verifier class is admitted through the registry by an identity that binds key, constraint-system descriptor, degree, exact public-input layout, transcript, SRS and statement adapter; adapter keys are registry data and change no core key. Classes the gadget does not implement get an adapter (or an authenticated verifier program) without a new core key. An inner accumulator is folded only for the same SRS; otherwise the adapter discharges the pairing under its own parameters.
- Dissent: rev-2a3a (Google) accepts cand-fd2b's scope amendment as one option. All other reviewers say the stdlib-only amendments in cand-fd2b and cand-0c40 narrow GP-3 and may only stand as explicit owner amendments. rev-2756 adds that cand-d600's extension to other KZG parameters goes beyond "same curve and commitment scheme" and should not silently become a requirement; rev-bfd3 asks for an other-SRS path discharged inside the adapter. Open finding (rev-bfd3, rev-2756): the generic path for non-stdlib classes is asserted, not designed; specify the verifier representation, chunk schedule and accumulator handling and trace one small same-curve non-stdlib circuit.
- Fact correction (rev-bfd3): the stdlib configures two instance columns (`midnight-zk-stdlib-2.4.0/src/lib.rs:545-548`); the gadget's one-instance-column comment at `verifier_gadget.rs:16-20` is stale.

### (iii) Ledger-format (Blake2b) Compact proofs

**Selected option (unanimous on substance):** GP-3 completion requires verifying the supplied ledger-format proof bytes and exact public inputs under the registered key, without the witness, with the keyed 64-byte Blake2b transcript (key "Domain separator for transcript", absorb/squeeze prefixes), complete byte consumption, canonical scalar bounds and point/subgroup checks. Poseidon re-proving is development evidence only and does not close the GP-3 acceptance test. No amendment needed.
- Findings: cand-0c40 names the transcript Blake2b-256 (wrong: hash length 64); cand-fd2b claims the GP-3 acceptance test at the Poseidon stage (rejected by all four reviewers). The stdlib Blake2b helpers pass an empty key. The in-circuit transcript gadget documents missing trailing-byte and subgroup enforcement. Row estimates for an in-circuit Blake2b adapter are unmeasured.
- Facts checked: `TranscriptHash = blake2b_simd::State` at `repos/midnightntwrk/midnight-ledger/transient-crypto/src/proofs.rs:64`, used at :575 (working-tree copy; the checkout is not at a tag, so the tag line was not confirmed); `Params::new().hash_length(64).key(b"Domain separator for transcript")` with `BLAKE2B_PREFIX_COMMON` / `BLAKE2B_PREFIX_CHALLENGE` at `tools/profile/vendor/midnight-proofs-0.8.3/src/transcript/implementors.rs:13-29`.

### (iv) Fixed tree depth versus expandable storage

**Selected option: no fixed depth-32 cap; one key serves variable or expandable depth.** All four reviewers reject cand-fd2b's depth-32 amendment as a default. rev-64d8 and rev-2a3a: witnessed active depth `1..=64`, root extended by one compression when allocation grows, padded chip loop. rev-2756: word the bound as the 64-bit address word that already bounds height, with a path cursor bound into the state commitment and sealing refused while an operation is pending.
- Dissent (rev-bfd3, OpenAI): the 2^64 height bound does not establish a storage-address bound; a 64-bit ceiling is itself a fixed ceiling and must be an explicit OWNER AMENDMENT; use expandable addressing with authenticated continuations. It also notes that long-path continuations conflict with the current ban on zero-step segments (`ivc.rs:368-372` requires `1 <= steps <= S_MAX`), so owner-step height and proof-work progress must be separated.
- Fact: GP-1 bounds only run length by the height field (`general-purpose.md:13-14`); heights are checked below 2^64 (`ivc.rs:79,368-370`). cand-0c40's quotations of GP text ("No fixed upper bound on program size…", "Combine at least two prior proofs plus new work in one node", "any Midnight circuit whose verifying key is registered") do not appear anywhere under `openspec/` (grep, including `specs/`).

**D3 kill criteria.** (i) Withdraw the amendment recommendation if a hardened node with two prior proofs and useful inline chiplet work fits under 996,147 rows and meets throughput; reverse the literal-compliance-predicate reading if that node plus predicate exceeds 996,147 rows. (ii) Reverse if a same-curve Compact circuit cannot be admitted without a new core key. (iii) Reverse if challenges differ from native verification on a captured proof, or a split adapter still needs to drop consumption or subgroup checks. (iv) Reverse if a resumed path can be accepted with the cursor advanced and the root unchanged, or a larger valid store needs a second key.

**D3 first experiments.** `cost_model` of the hardened two-slot node with one-hot key selection, canonical disabled slots and the seal/dependency/shape constraints, against 996,147 and 1,048,576 rows; off-circuit replay of one stored Compact proof through the host `Blake2bState`, recording the full challenge sequence against native verification; one append plus one 64-level opening (with cursor resumption) modelled for rows.

## D4 Inline processor versus separate leaf plus lifts

**Selected option.** Separate leaf plus lifts: the chiplet leaf `U` (D2), run lift `R`, foreign lift `F` over per-class adapters held as registry data, a binary node `N` over the named keys `{N, R, F}` that also runs the composition predicate, closing proof `C` and decider `D`. Core keys `{U, R, F, N, C, D}`. Chips never become separate proofs; they stay inside `U`.

**Votes.**

| Review | Family | Pick | Option | Verdict |
|---|---|---|---|---|
| rev-64d8 | xAI | synthesis 0c40 (Google) + fd2b (Meta) | separate, `{U,R,F,N,C,D}` | approved |
| rev-2a3a | Google | cand-fd2b (Meta) | separate, `{U,R,F,N,C,D}` | changes_requested |
| rev-bfd3 | OpenAI | synthesis 0c40 + fd2b | separate, provisional | insufficient_evidence |
| rev-2756 | Meta | synthesis d600 (OpenAI) + f429 (xAI) | inline processor, `{N, D}` | changes_requested |

**Status: blocked by dissent.** Counted per decision, quorum is met (3 substantive verdicts from 3 families: xAI, Google, Meta; the OpenAI abstention does not count) and two admissible `changes_requested` are open. Counted per option under `tally-rule.md`, the selected option has only 2 substantive verdicts (xAI approved, Google changes_requested), so it would also fall short of quorum on its own.

**Dissent, in substance.**
- rev-2756 (Meta), option dissent: inline processor with core keys `{N, D}`; chips as selector-tagged regions of `N`, foreign verification as chunked work in `N`, unfinished continuations distinguished from completed claims in the statement; separate leaf plus lifts only as the fallback with the D3(i) amendment. Findings: price `D` over the new `vk_N` in the first cascade probe and treat `D` above 2^20 as a kill; arbitrary-depth PCD soundness for this accumulator is not argued by any candidate; the separate-leaf fallback in cand-0c40 cannot be adopted as written (LogUp, nonexistent vendor paths, u64 fuel, 19 folds priced for 37 proofs); every pure join pays a full k=20 node in the inline design, and the P=3 concurrency figure was measured on the lookup-free node.
- rev-2a3a (Google): the 730,243-row two-slot probe does a single final collapse; normalising each disabled slot with an in-circuit `collapse_self` (239,577 rows, `speed-levers.json:66`) would overflow 2^20, so `N` must select the canonical neutral accumulator by lane selection and collapse once. `GPR-NODE` must state that `C` folds `N`'s own and carried accumulators and resolves the fixed bases of `N`, `R`, `F`, and that `D` folds `C`'s own and carried accumulators, resolves `C`'s fixed bases and pairs (as `run_decider.rs:222-245` does today).
- rev-bfd3 (OpenAI), abstention with findings: the candidates never compare the same chiplet processor in both placements; cand-0c40's claim that about 90% of node time is MSM collapse is contradicted by the recorded profile (MSM share 0.256, quotient/NTT 0.57, `speed-levers.json:8-10`; the 78% figure concerns rows); matching commitment counts is not enough for named-key routing because fixed-base names are absent from the accumulator encoding, so the selector must also constrain coefficient destinations and statement types.

**Kill criterion.** Move the chiplet trace into `N` only if a measured node that verifies two predecessors and performs a checked owner operation fits under 996,147 rows, `D` over that key fits under 2^20, and the full pipeline (including all lifts and closing) beats the separate packing on a G-C run within 2 h and 48 GB per process. Reject the separate configuration if any of `R`, `F`, `N`, `C`, `D` exceeds 996,147 rows or the complete pipeline misses G-C.

**First experiment.** Paired `cost_model` of the same small chiplet slice in both placements: `U → R → N → C → D`, and inline in a binary `N` plus `D` over the resulting key, with final statement widths, canonical disabled slots (single collapse), fixed-base routing, and negative checks for a wrong-key slot, a malformed disabled-slot carry and an attempted finalisation of an unfinished continuation. No SRS, no proving.

## Top experiments (cheapest first, all model-only)

1. Recursive verifier gadget verifying a key that carries a `lookup_any` argument (gates every chiplet placement; for the selected option this is `R` over a `U` key).
2. Host census of table accesses and hash calls over fact(3) and list123, and host `Init` differential on `literal`, `add`, fact(3) (D1, D2 sizing).
3. Minimal `U` slice `cost_model` plus `R` over its key (D2).
4. Paired placement cascade: `U → R → N → C → D` against inline `N` plus `D` (D4, D3(i)).
5. Off-circuit replay of a stored Compact proof through `Blake2bState` against native verification (D3(iii)).

## Proposed EARS and design changes

Text only; nothing below has been applied to any file. Items marked OWNER AMENDMENT change the owner's GP text and need the owner's decision. Nothing else here narrows GP-1..GP-4.

1. SRC-021 mechanism (replaces the `[PENDING …]` clause; does not amend GP text): "When `h_pre = 0`, the first segment leaf SHALL prove in circuit that its register file and commitment roots equal `Init(prog_d, input_d)`, where `prog_d` commits the program table, entry metadata and machine version and `input_d` commits the run input, the initial fuel (u128) and the stack limit; `Init` SHALL bind the input into the initial state and SHALL fix every table root and length, both stack links, the helper and result registers and `deps_d`. The seal rule SHALL require `h_lo = 0` and a terminal outcome. Equality with a supplied `init_d` alone SHALL NOT count as this check. Off-chain recomputation MAY be recorded as a diagnostic and SHALL NOT be acceptance authority."
2. Chiplet leaf (replaces the SRC-013 option-U wording; the owner's D2 direction): "The universal leaf `U` SHALL be one fixed relation over a fixed register file and the rule inventory. It SHALL prove between 1 and S owner steps and SHALL route program reads, memory operations, 128-bit arithmetic and hashing to chips inside the same circuit through the pinned library's lookup and permutation arguments, which SHALL close inside the leaf. State that crosses a segment boundary SHALL be the register digest and the program and memory roots."
3. Memory integrity (new): "When the leaf issues a memory or program request, the proof SHALL bind operation, operands, result and table identity, and SHALL establish address uniqueness, allocation freshness, ordering and read-after-write consistency, authenticating each distinct cold row and every boundary update against `prog_d` or the memory root. Frames and helper tasks SHALL be stored as linked write-once entries or hash chains."
4. Continuations (new; needed by the expandable-storage and cursor readings of D3(iv) held by rev-bfd3 and rev-2756, and by any multi-chunk foreign verification; rev-64d8 holds that continuations are unnecessary while a 64-level path fits as a padded loop in one leaf): "While an operation spans proofs, each proof SHALL carry its complete continuation state, including any path cursor, in the state commitment; a boundary SHALL NOT consume semantic fuel, advance owner-step height without a completed owner step, or expose an unfinished continuation as a completed claim." Design note: this conflicts with the current `1 <= steps <= S_MAX` relation (`ivc.rs:368-372`) and requires separating owner-step height from proof-work progress.
5. Storage capacity (no amendment): "One key SHALL serve every memory size; authentication path length SHALL be a witness; no keyed depth, carrier-size or page-count limit SHALL impose a program, input or storage capacity bound." OWNER AMENDMENT (optional, flagged by the OpenAI reviewer as a ceiling GP-1 does not state): "Addresses and tree depths are bounded by the same 64-bit word that bounds height (active depth 1..=64)." Rejected by all reviewers as a default: cand-fd2b's depth-32 amendment.
6. Foreign proofs (no amendment): "When a foreign proof is admitted, the registry SHALL bind its verifier class (key, constraint-system descriptor, degree, exact public-input layout, transcript, SRS and statement adapter); adapter keys SHALL be registry data and admitting one SHALL NOT change the core key set. An inner accumulator SHALL be folded only when the foreign proof uses the outer SRS; otherwise the adapter SHALL discharge its pairing under its own parameters." Not adopted (would narrow GP-3; OWNER AMENDMENT only): the stdlib-only scope texts of cand-fd2b and cand-0c40.
7. Ledger bytes (no amendment): "When a claim states that ledger-format Compact proof bytes were verified, the adapter SHALL verify the supplied bytes and exact public inputs under the registered key without the witness, enforcing the keyed 64-byte Blake2b transcript with its prefixes, complete consumption, canonical scalars and prime-subgroup checks; the claim kind SHALL record `ledger`. A Poseidon re-proof SHALL be recorded as `reproved` and SHALL NOT satisfy the GP-3 acceptance test."
8. Node and closing (for the selected D4 option): "The node `N` SHALL verify up to two proofs under `{N, R, F}` with one constrained selector that also selects the statement type and every deferred fixed-base destination; a disabled slot SHALL take the canonical neutral accumulator by lane selection, with one collapse per node. `C` SHALL verify the root `N`, fold its own and carried accumulators and resolve the fixed bases of `N`, `R` and `F`; `D` SHALL verify `C`, pin its key, fold its own and carried accumulators, resolve `C`'s fixed bases and check the pairing."
9. Statement (new): "The decided statement SHALL carry the claim-set commitment and a shape commitment binding slot bits, continuation kind, segment bracketing and dependency closure; the final proof SHALL be one decider proof of equal size for every shape and depth."
10. G-C accounting (amends the projection rule only; caps unchanged): "A G-C projection SHALL count every leaf, lift, merge, closing and decider proof, setup, independent verification and call validation, with measured contention and both memory peaks; estimates SHALL NOT constitute a pass."
11. OWNER AMENDMENT (D3(i), tie; for the owner, not adopted by the council): replace the first GP-4 bullet (`general-purpose.md:46`) with "A node can take at least two proofs (arity >= 2), each either a prior proof or new work proved under a core lift key, so any combination of prior proofs and new work is reached by nesting under the same key set. Every node shape is served by the same key set; there is no per-depth or per-level key." The acceptance test stays unchanged. Alternative without amendment (xAI reviewer): an EARS line naming the node's compliance predicate as the new work. The OpenAI reviewer disputes that this alternative meets the current text.

## Files

- `BRIEF.md`, `tally-rule.md`, `review-record.json` (per-review verdicts, picks and hashes)
- `blinded/cand-*.md`, `review-prompts/rev-*.md`, `reviews/rev-*.json`
- `proposals/<lane>/` (prompts, raw proposals and lane logs, copied after classification); `reviews/transport/` (raw review transport files)
- `_private/identity-map.json` (candidate and reviewer identities)

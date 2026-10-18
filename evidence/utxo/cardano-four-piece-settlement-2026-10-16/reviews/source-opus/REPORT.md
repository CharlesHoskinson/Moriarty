# Fresh non-author source/effect review: M2 + O2 for one bounded local run

**Reviewer:** `claude-opus-5-5`, in a fresh context. I did not author M2, O2, the Muse diagnosis, the Muse operator completion, or Gemini `c611776c`. I made no tool calls, ran nothing, read no files beyond the inline packet, used no RNG, keys or ledger, and delegated nothing. This review is source-only. It grants zero execution authority, reports no result, and supplies no AUTH.

## 1. Verdict

**SOURCE_EFFECT_READY for ONE bounded ordinary 15-step local build/hash/size/147-selected-test run.** This is conditional on root actions that do not require patching source (§8). I found no defect that requires a source fix to M2 or O2. Every gap I found either:

- fails closed before T exists, or
- is documentary or a stale non-gating guard, or
- is a root/AUTH duty outside the 34 files.

Readiness is not a prediction that the run will be all-green.

- **Expected to stay FALSE:** RANGE-vs1, since vs1 was 6,859 B against the [7000, 10500] range and no validator source changed.
- **Not predicted either way:** PC4-4.
- **Required after the run:** separate root exact AUTH with a new target, then independent actual-result review.

## 2. M2 composition and the one repair

**Manifest.** M2 `ROOT-SOURCE-MANIFEST.json` is 14,506 B, `54e0fc56…`, with `parentManifestSha256` `dcdf544a…`. I counted 41 rows:

- 32 under `candidate/aiken/`: `aiken.toml`, 7 validators, 24 `lib/pvc2` modules.
- 9 documents.

**Cross-check against the operator data tables.** I compared all 32 package rows against `data/candidate-32.sha256` row by row, and they match. Examples: shard4 `7bac10be`, s2p `5960b769`, harness `0df2a7a7`, shard4_test `9310dab8`, cost `1d6cfb48`, fixtures `23d40ff2…ebc72c`. The 9 document rows match `data/m-docs-10.sha256`, where `EXPECTATIONS` is `80cf1601`; the 10th row of that table is the manifest itself.

**The diff.** `EXACT-HARNESS-DIFF.patch` (`22c3a892`) is one hunk. It changes `pub const vs2_t` from `#"c2…c2"` to `#"d2…d2"` (28 bytes). The current `shard4_harness.ak` bytes show `vs1_t c1×28`, `vs2_t d2×28`, `vs3_t c3×28`, `vs4_t c4×28`. Nothing else in the harness differs from what the diff implies.

**No other candidate change.** All other 40 rows keep their parent `original` paths in the shard-split M1 directory. `EXPECTATIONS` is byte-identical at `80cf1601`.

**The production anti-alias guard is intact** (`shard4.ak`, `identities_distinct`):

```
let ids = [c2_hash, vi_hash, policy, ..pieces]
list.length(pieces) == 4, all 28-byte, list.length(list.unique(ids)) == list.length(ids)
```

This is a 4-argument, `list.unique`-based guard. It is not the 7-argument, 21-inequality function Gemini described.

**Root cause, confirmed from source.** `fixtures_r1.ak` line 380 sets `c2_hash = c2×28`. The old `vs2_t` was also `c2×28`. So every call of `identities_distinct(fx.c2_hash, h.vi_t, s4.pieces_t(), …)` had a duplicate and returned False.

## 3. The 18 FAIL-affected paths and the masked negatives

### 3.1 The 18 tests

Each of these reaches `identities_distinct` with `fx.c2_hash` and `pieces_t()`, so the alias alone forced False:

1. **Ten `accepts1` tests** (`q_p01`–`p06`, `q_p14`–`p17`). The last conjunct, `s2p_on`, calls `shard4.delegated(c2, h.vi_t, pieces_t(), own_ref, t)`.
2. **`q_p07_accept`.** It calls `s2p_on` twice.
3. **`q_p08_genesis` and `q_p09_genesis`.** They go through `genesis4` → `shard4.genesis` → `identities_distinct`.
4. **`q_distinct_fixture_accepts`.**
5. **Four cost benches:** `s4c_p01_s2p`, `s4c_p04_s2p`, `s4c_p07_s2p_0`, `s4c_p07_s2p_1`.

That is 10 + 1 + 2 + 1 + 4 = 18, consistent with the frozen 129/147 + 18 FAIL.

**What the repair removes.** After it, `s2p_on` on an honest `sharded4` transaction goes through:

- `piece_lists` for k = 1..4;
- `shard_redeemer`, where `with_withdrawal` prepended `Pair(Script(h),0)` and `with_redeemer` appended exactly one `Withdraw` redeemer;
- `expect p: VsPieceV1`, `piece == k`, `thread_policy == own_hash`, and `list.has(threads, own_ref)`.

All of these hold by construction. The earlier `accepts1` conjuncts (`vi_count`, `vs1_on`…`vs4_on`) are not shown to hold for P02, P03, P05, P06 and P14–P17 by any prior observation. The `and` short-circuits, so the old FAIL does not reveal them. Whether these 18 now pass is exactly what the run measures.

### 3.2 Negatives that were masked and now refuse for their intended reason

I traced each:

| Test | Intended refusal |
|---|---|
| `q_s2p_missing_vs1..4` | skipped piece → `shard_redeemer` None |
| `vs2_redeemer_without_withdrawal` | `has_key` false |
| `vs4_withdrawal_without_redeemer` | `found = []` |
| `original_tx` | no withdrawals |
| `duplicate_vs3` | `found` has 2 entries |
| `swapped_piece_tags` | VS1 slot carries tag 2 |
| `foreign_policy_vs4` | policy ≠ own_hash |
| `uncovered_thread` | r1 not in VS1's threads |
| `q_s2p_piece_redeemer_wrong_type` (fail) | now reaches `expect p: VsPieceV1 = raw` on `AuthShardV1` data and halts; previously the `and` returned False before the decode. Both pass a `fail` test. |
| `q_h46`/`h47`/`h48`/`h58_genesis_false` | unchanged; `thread.genesis` is the first conjunct |

### 3.3 The seven actual negative alias tests (source bytes)

1. `q_alias_vs1_vs2`: `[vs1_t, vs1_t, vs3_t, vs4_t]`, a duplicate.
2. `q_alias_vs3_vi`: `[vs1_t, vs2_t, h.vi_t, vs4_t]`. Previously this also had a vs2 = c2 collision; now it isolates the VI alias.
3. `q_alias_vs4_c2`: `[vs1_t, vs2_t, vs3_t, fx.c2_hash]`. Previously c2 appeared three times; now it is a single alias.
4. `q_alias_vs2_policy`: `[vs1_t, p, vs3_t, vs4_t]`.
5. `q_alias_three_pieces`: `[vs1_t, vs2_t, vs3_t]`. The first conjunct, `length == 4`, decides it.
6. `q_alias_s2p_refuses`: `delegated` with `[vs1, vs1, vs3, vs4]`.
7. `q_alias_genesis_refuses`: `genesis4` with `[vs1, vs1, vs3, vs4]`.

Gemini's descriptions of #5 ("vs3 == vs4"), #6 ("vs4 == vs1") and #7 ("vs2 == c2") are wrong.

### 3.4 Constant uniqueness

From visible fixture text:

- `c2_hash` is `c2×28` (l. 380) and `c2_hash_foreign` is `cc×28` (l. 383).
- The P01–P05 statements carry `settle_hash` `a2×28` right after the claimant (`55×28`) and network byte, so the P01–P05 thread policy is `a2×28`.
- The harness constants are `b1`/`b2`/`b3` (shard_harness) and `c1`/`d2`/`c3`/`c4`.

These are pairwise distinct.

## 4. Partition (the "16") and the parameter DAG

**The "16 claim IDs" in Gemini's report do not exist in this source.** There is no `piece_1..4`, no `verify_claims_n`, and no ID range 0..15. The real 16 are the components the parent VS checked: `shard.thread_state` has 15 per-thread components, and `shard.state` adds `global_outputs_distinct`.

Their placement in `shard4.ak`:

| Piece | Components | Count |
|---|---|---|
| VS1 (`structure_thread` + `vs1_structure`) | shape_structure, shape_cells, own_mint, topup, reserve_floor, + `global_outputs_distinct(r.thread_policy, self)` once | 6 |
| VS2 (`accounting_thread`) | recomputed, bounds, intent_effects | 3 |
| VS3 (`identity_thread`) | claim_script, shape_digest, claim, statement_digest | 4 |
| VS4 (`custody_thread`) | custody_pre, continuation (via `claimed_next`), payouts | 3 |

That is 16 components, each in exactly one piece. VI keeps authority_shape, intent_window and intent_native, which brings the total to 19. This matches `DESIGN.md` §4 and the EXPECTATIONS `partition`.

**Use of `no_c2 = #""` in VS1, VS2 and VS4 is sound.** Only `settle_claim_script` reads `b.c2_hash`. `claimed_next` selects the claim at `Script(td.claim_hash)`, and VS3 requires `td.claim_hash == c2_hash`.

**The VI binding moved into VS3.** VS3 performs the old VS-side checks:

- `auth.thread_policy == r.thread_policy`;
- outref list equality, which makes the later `zip` lengths equal;
- the per-outref VI digest.

**N-3 (VS4 builds the next datum from the claim envelope digest).** This is sound under the conjunction. Both pieces use the same deterministic `settle_claim_input`, which halts unless exactly one claim exists, and VS3 checks `st.statement_digest == cl.env.statement_digest`.

**Hash chain.** I distinguish three things the review was asked to check:

- **The event chain.** `settle_next`'s `event_chain` is unchanged W1, with `#"01"`. VS4 feeds it the envelope digest.
- **FV-01 (the parameter DAG).** The DAG is C2 → VI(c2) → VS3(c2, vi) → S2′(c2, vi, vs1, vs2, vs3, vs4), with VS1, VS2 and VS4 unparameterized.
  - S2′ never requires VI directly. It relies on the VS3 hash it was parameterized with, and `identities_distinct` cannot check DAG consistency on-chain.
  - OPX4 enforces the DAG off-chain. It applies 9 parameters, one per apply, each from the previous read-only blueprint.
  - `hash_chain4.py` checks: `PARAMS` source-before-consumer (else a "cycle" problem); parameter bytes equal `581c` followed by the source hash; identical c2/vi bytes across all applications; blueprint hash fields equal the tool output; final re-derivation from B9; and 7 distinct 56-hex hashes.
  - **Carried, non-blocking:** deployment must bind S2′ to this exact DAG.
- **FV-02 (cross-policy single-transaction restriction).** Carried. VS1, VS2 and VS4 have one shared hash (and VI/VS3 are shared under the same C2). Each piece's single `Withdraw` redeemer names one `thread_policy`, and S2′ requires it equal to its own hash. So two S2′ deployments cannot settle in one transaction. This is a liveness limit, not an acceptance hole.

## 5. fixtures_r1.ak limitation (explicit)

The line view covers 446 lines. Literal lines over 2,000 B appear only as byte count and sha256. Examples:

- l.5 `ctx_p01`: 12,228 B, `54c7137f…`
- l.317 `ctx_p07_0`: 21,032 B
- l.329 `ctx_p08`: 3,700 B
- every `datum_post`/`msg` literal

I did not read those bytes. I make no all-bytes-read, CBOR or curve claim. As a result, I cannot confirm from source:

- the P06/P07/P08/P09/P14–P17 thread policies;
- that no catalogue `native` or `reserve_owner` credential is `Script(d2×28)`.

The latter governs `q_guard_accepts_fixture_catalogue` and `q_p08`/`q_p09`. It passed in the prior run with `c2×28` in `shards_t`. No visible line contains a `d2×28` run. This is a residual that the run itself measures.

## 6. O2: full read of all 34 files

**Pins.** The O2 manifest is 11,831 B, `a916a452`, with 34 rows equal to `OWN_FILES` and `OWN-OUTPUT-ALLOWLIST` (34). PRE-LAUNCH `74b248ed` has:

- `targetAbsent: true`;
- operator and candidate pins equal to `preflight.py` literals: `PRELAUNCH_FORMAT` and `OPERATOR_MANIFEST_FORMAT` harness-repair `/1`, and `CANDIDATE_MANIFEST {14506, 54e0fc56…}`;
- 15 step sha256s equal to the manifest rows: OPX0 `a775f6db`, OPX1 `ba2ae2e3`, OPX1t `dabd3b99`, OPX1p `be9eecb9`, OPX2a `f210687e`, OPX3 `e1b34a92`, OPX3p `29008b72`, OPX4 `9c39dd94`, OPX5 `3e91303f`, OPX6 `250a238b`, OPX2 `a6bdfdb6`, OPX2p `c600a503`, OPX2x `7db5aacb`, OPX7 `70504bb0`, OPX7f `3b7c8ffe`.

**Composition check by byte deltas.** I compared O2 step sizes with the parent AUTH sizes:

- 12 steps are +47 B (e.g. OPX0 4783 → 4830, OPX4 8378 → 8425). That equals the sum of the recorded edits: header +9, HOME +15, OWN +9, PINSTORE −4, M +3, T +15.
- OPX1 is +39 (header, HOME, T).
- OPX7f is +48 (no PINSTORE or M edit).
- OPX1p is +118, the extra being its comment edit.

Six tools keep their parent sha256 (freeze, tree_check, test_compare4, hash_chain4, label_search4, size_report4). preflight has 4 literal edits and evaluate4 has one LINEAGE edit. I did not have the parent bodies or RENDER-PLAN (`60b1a57f`); this cross-check uses the root's `OPERATOR-COMPOSITION.json`.

### 6.1 Steps (each script exits 0; its result is in `logs/ID.exit`)

All bounded steps run under `nice -n 10 timeout --kill-after=K W prlimit --as=34359738368 --core=0`.

| Step | Gate | Cap / kill / reserve | Effect |
|---|---|---|---|
| OPX0 | — (root STOP on nonzero) | 60 / 10, charged 70 | Writes nothing. Checks T absent, preflight `pre`, coreutils re-checks, stdlib pin, DATA-ONLY binary `4257cc8b` at mode 644 |
| OPX1 | root STOP if exit missing or ≠ 0 | unwrapped, charged 5 | atomic `mkdir` chain; empty `journal/elapsed.txt` |
| OPX1t | OPX1 | 120 / 10 / 136 | copies binary 0600 → sha → 0555; netns shows only `lo:`; `--version` = `aiken v1.1.24+bacbeb3`; 5 help files byte-equal to attempt 2's |
| OPX1p | OPX1 | 120 / 10 / 136 | 32 literal `cp --`; stdlib 38 + 77 B `packages.toml` (`42ca2389`); 71 files / 19 dirs; `tree_check --forbid-generated` |
| OPX2a | OPX1t & OPX1p | 300 / 30 / 136 | `aiken check --skip-tests --trace-filter all --trace-level silent` |
| OPX3 | OPX2a | 300 / 30 / 136 | `aiken build …`; requires `aiken.lock`; read-only `a2/plutus.unapplied.json` |
| OPX3p | OPX2a started | 60 / 10 / 136 | `--require-lock` only if OPX2a = 0; `--plutus` only if OPX3 = 0 |
| OPX4 | OPX3 & OPX3p | 240 / 10 / 136 | 9 applies + 13 hashes; then `hash_chain4` |
| OPX5 / OPX6 | OPX3 & OPX3p | 30 / 10 / 136 | labels / sizes; applied blueprint only if OPX4 = 0 |
| **OPX2** | **OPX2a = 0 & OPX3p = 0** | 1800 → aggregate / 60 / 246 | see argv below |
| OPX2p | OPX2 started | 60 / 10 / 176 | always `--require-lock` |
| OPX2x | OPX2 log exists | 30 / 10 / 136 | `test_compare4` |
| OPX7 | always run by root | 90 / 10 / 36 | integrity + preflight `post`; T top-level set |
| OPX7f | last | 30 + 5; needs ≥ 35 s left | `evaluate4`, `freeze`, `chmod` |

OPX2 argv is exactly:

```
aiken check --trace-filter all --trace-level silent --match-tests pvc2/shard4_test --match-tests pvc2/shard4_cost_test
```

There is no unfiltered, 9-module or 22-single-test campaign.

**Aggregate budget formula:** `W = 1800 − (75 + Σelapsed) − reserve − kill`, capped at the step cap. If `W < 5`, the step is not started and records exit 200.

**Gates, stated precisely:**

- Tests require a clean type check (OPX2a = 0) and clean post-build hygiene (OPX3p = 0).
- Tests do **not** require a successful build (OPX3) or application (OPX4). Nothing assumes all builds succeed.
- A failed OPX3 is labelled `FAILED(candidate build)`, `BLOCKED(pin-compat)` or `UNCLASSIFIED`, and skips OPX4–OPX6 with exit 201.
- OPX3p then does not require `plutus.json`.

**Tree hygiene (`tree_check.py`):**

- The 71 placed files must keep their pinned sha256.
- Only `aiken.lock`, `build/aiken-compile.lock` and `plutus.json` may appear.
- No non-regular entries are allowed.
- `build/packages` must contain exactly the stdlib and `packages.toml`; any extra there is "fetch".
- `aiken.lock` must name one package, stdlib v2.2.0, with no etag.
- Other new directories are recorded but not gated.

### 6.2 Tools

All 8 import stdlib only (`hashlib`, `json`, `os`, `re`, `stat`, `sys`, `collections`, `datetime`), with no subprocess and no sockets. Outputs are written with `open(…,"x")`; preflight writes nothing.

**preflight checks:**

- **P1–P4:** PRE-LAUNCH, operator manifest plus originals, the 41 candidate rows plus originals, and the tree (32 files, 3 directories, validators exactly as listed).
- **P5:** parses `TEST_DECL` and requires 125/9 and 22/0. Its module walk proves that only the two modules match the filter strings (stdlib names cannot contain `pvc2/`).
- **P6:** parent evidence and the R1 control.

**evaluate4:**

- keeps `allCommandsExit0` (13 steps) separate from `allPredictionsHeld`;
- labels each prediction TRUE, FALSE or UNEVALUATED;
- makes GREEN conditional on `OPX7f.exit = 0`;
- records LINEAGE: attempt1 14/90, attempt2 14/934, attempt3 15/446 at 129/147 with T-147/PC4-4/RANGE-vs1 FALSE, and `thisRun` 43 calls / 1,470 s kept separate.

**Effect boundary.** No step touches keys, wallets, the network (only `unshare --net`), SDKs or proofs. By call graph, none of the 147 tests reaches `intent.verify` (secp256k1) or `groth16.verify_digest` (BLS12-381):

- `vi_count` uses `authorize_gate`, not `authorize`.
- No test calls `claim.issue`.
- `q_pieces_on_revoke_tx_refuse` never calls `thread.revoke`.

Only CEK blake2b and sha2 hashing of fixture data runs. `aiken build` compiles the crypto builtins but does not execute them.

## 7. AUTH controls versus script enforcement

**What the scripts enforce:** per-step caps, kill-after, reserves, the 1,800 s ledger, the 32 GiB address-space limit, core 0, nice 10, self-gating with exits 200/201, pins, T-absent at OPX0, the T top-level set at OPX7, and no retry loop anywhere.

**What only the root can enforce:**

- STOP after a nonzero OPX0 or a missing/nonzero OPX1;
- invoking each script once in fixed order;
- rehashing all 34 files and the manifest before each call and after the final call;
- no outer timeout or restart;
- the lost-wrapper rule;
- one attempt and zero retries.

**Where EC-1/EC-2/EC-3 stand.** HR-CAP names them only by label. Their texts are not in O2: its `EXECUTION-PROPOSAL` is reduced to launch order, limits and preservation statements. The parent AUTH `stopRules` are the only supplied wording.

**The parent runner cannot be reused.** `run-reviewed-ordinary.py` (`c5407a02`) hard-codes the parent AUTH sha `76ce6440…` and writes to its own records directory. A successor runner is new root control code outside the 34 files, and it must be pinned in the new AUTH.

**Parent controls and counters stand unchanged:** per-step caps, kill, reserve, the 1,800 s aggregate, the 32 GiB address space, core 0, nice 10, one attempt and zero retries. The prior 43 calls and 1,470 s charged are not reset.

## 8. Findings

| ID | Severity | Blocking | Finding |
|---|---|---|---|
| OF-1 | MEDIUM | No (fail-closed) | The M2 harness row's `original` is `…actual-review-and-repair-muse-source-20261016/candidate/candidate/aiken/lib/pvc2/shard4_harness.ak` (note the doubled `candidate`). preflight P3 calls `file_is` on it. If that path is absent or differs, OPX0 exits 1 and the run stops before T exists. The root should confirm the path is present at sha `0df2a7a7` before AUTH. |
| OF-2 | LOW | No | `hash_chain4.py` `TEST_IDS` and the `notGenuine` text, plus EXPECTATIONS `testIdentifiers` (`"vs2_t": "#c2 x 28"`), are stale relative to `d2×28`. These are defence-in-depth and documentation only. Per its own authority clause, EXPECTATIONS is not refit. |
| OF-3 | LOW | No | HR-CLASS's "EXECUTION-PROPOSAL outcomes" resolves to M2's `EXECUTION-PROPOSAL` (`e3b749`) `outcomes` table, not O2's (`7a790992`), which has none. evaluate4's labels match the M2 table. HR-EXP's `BLOCKED(expectations-transcription)` is a root label; preflight emits problem text with exit 1. |
| OF-4 | INFO | No | Accepted residuals are carried verbatim: host FS, pathname UNIX sockets, no FSIZE limit or syscall trace, OPX1 unwrapped, OPX7f not tee-logged. |
| OF-5 | HIGH (gate) | Yes, for execution only | The next steps are a new root exact AUTH (O2 `a916a452`, M2 `54e0fc56`/14,506 B, PRE-LAUNCH `74b248ed`, the 15 argvs, the caps, EC/stop texts), a pinned successor runner, and a confirmed-absent T. Afterwards, independent actual-result review. |

## 9. Gemini `c611776c`: UNADOPTED

I independently confirmed the three root-recorded discrepancies (`d0f6223502…`). Gemini's eight clause sha256s do match `EXACT-OPERATOR-CLAUSES` (`4b5e9729`), but its rationales describe bytes that do not exist:

- an unfiltered OPX2, 9 per-module OPX2p runs and 22 single-test OPX2x runs;
- C2 = 13,989 B and VI = 1,565 B (actual 2,770 and 12,784);
- `prlimit --nproc/--nofile`, `--signal=TERM`, `tmp-home` paths, and Aiken v1.1.21+42b308e;
- `mkdir -p T` in OPX0;
- G1..G10 / F1..F8 and an 11,500 B piece bound;
- a 7-argument `identities_distinct`, `verify_claims_n` and claim IDs 0..15;
- wrong alias descriptions;
- a wrong fixtures sha (`23d40ff2324455…`) and a wrong M1 manifest sha.

None of Gemini's text is copied or adopted here.

## 10. Exact positions (8)

**Tally.** Each clause now has two agreeing substantive positions: the author's first FOR (`muse-wonder-1.0`) and this fresh AGREE. The prior Muse operator-only completion (all AGREE, operator scope) is preserved but not counted. Gemini's positions are not adopted.

| ID | Literal clause sha256 | Position | Rationale (actual bytes) |
|---|---|---|---|
| HR-ORD | `e1fd19ff49ef00e637e1688a14a514d79b909efdb58662368dccefa466469200` | AGREE | `LAUNCH_ORDER` equals PRE-LAUNCH order. `ok OPX2a` gates OPX3; `ok OPX3 OPX3p` gates OPX4–OPX6; `ok OPX2a OPX3p` gates OPX2, with no OPX3 dependency; OPX2p/OPX2x require a started OPX2; OPX7 always runs; OPX7f is last. |
| HR-FILT | `155f302905c8b2d1fa35f026fccfeaa886bffefe622b3549faf242db7195543a` | AGREE | OPX2 argv is the two repeated `--match-tests` module filters. preflight P5 requires the matched set to equal `MATCH_STRINGS`. `test_compare4` maps any membership deviation to `UNCLASSIFIED(match-filter)`. EXPECTATIONS `80cf1601` is only read, with pin check. RANGE-vs1 is in evaluate4's lineage and evaluated afresh. |
| HR-PIN | `5f1ba8cd270fd8aee56436d79a10a29d732c94ad75c8998c768ab1ebf26d7311` | AGREE | preflight P1/P2 refuse non-HEX64 or mismatched pins and require OWN files to equal the rendered-candidate originals. P3 pins M2 to 14,506 / `54e0fc56`. OPX0 and OPX1t pin the binary to `4257cc8b`. Byte deltas match the recorded edits. Limit: RENDER-PLAN body not read. OF-1 fails closed. |
| HR-EXP | `ea12995f781e9fa38784317bcb32b916673855705bff996e6832cc579fbd092e` | AGREE | `check_expectations` requires counts 125/9 and 22/0, `perModule`, `totals.new {147, 9}` and 147 unique names, plus sorted declarations equal to EXPECTATIONS, run in `pre` mode before T exists. No file references an alternate schema. Label nuance: OF-3. |
| HR-HYG | `92e18140191bc40fabd491354068e413b4bbd042543eb0ad97ba99716362ae1e` | AGREE | `tree_check.py`: 71 pinned files, the `GENERATED` allow-list, `--require-lock` passed only when OPX2a = 0 (OPX3p/OPX7) or always at OPX2p, `--plutus` only when OPX3 = 0, exact `build/packages` and stdlib interior, `extraDirectories` recorded. aiken always runs under `unshare --user --map-root-user --net` with `env -i`. |
| HR-CLASS | `f0d6fd7ff45f635a9cddfdcc7ba1996b2edbf2010f107203e4317e905b037656` | AGREE | evaluate4: `allCommandsExit0` is separate from `allPredictionsHeld`; `verdict()` returns TRUE, FALSE or UNEVALUATED; GREEN requires exits 0, `PREDICTIONS_HOLD` and a passing hash chain, and is labelled conditional on OPX7f; LINEAGE keeps attempt3 at 129/147; size and scope text exclude ledger claims. Doc nuance: OF-3. |
| HR-CAP | `28c8d2e62900f599b739c32641e5113d5df975997cb70645afb342d1aca65d21` | AGREE | `B_TOTAL=1800`, `B_FIXED=75`, `--as=34359738368 --core=0`, `nice -n 10` and the §6 cap table are in the bodies, and no file contains a retry loop. The new T differs from the old one, and OPX0/OPX1 refuse an existing T. One attempt, the stop rules and EC-1/2/3 are root controls whose text must appear verbatim in the new AUTH (§7). |
| HR-TOOLS | `3cdebcf3a1d31e398cf91a8173432588c752276a5f75fc9704fd00b190898184` | AGREE | 8 stdlib-only tools. Six are unchanged by sha; preflight and evaluate4 edits are literal pin and lineage changes only. There is no 35th file and no new EXPECTATIONS format. Stale `TEST_IDS` is a pre-existing non-gating guard (OF-2). |

## 11. Preserved and open

**Frozen facts:**

- Actual receipt sizes: C2 2,770 B, VI 12,784 B; pieces 6,859 / 9,940 / 10,832 / 10,825 B; S2′ 13,018 B.
- Sums: pieces 38,456 B; all seven 67,028 B; prior four-script total 48,413 B; increase +18,615 B.
- The size target is a working target only; no size refit.

**Old outcomes stay immutable:** 129/147 with 18 FAIL, and T-147 / PC4-4 / RANGE-vs1 FALSE.

**PC4-4 in the new run is unknown.** The old `s4c_*_s2p` cost (33,496 mem) came from the alias-refused path and is not a settlement cost.

**This review made zero calls.**

**Still open:** all 20 UTxO families, ACTUS, DeFi, general obligations, recursive PCD, private handoff, multi-parent composition, universal certificates, real Preprod, ledger ExUnits, transaction size, fees, FX1, X1, M-HASH, SR-06 and related items, and PB-01..PB-12.

current-cardano-four-way-vs-harness-repair-opus-full-source-effect-completion-20261016-FINAL

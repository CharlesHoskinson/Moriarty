# T1: the integration branch `tested`

Row T1 of `LOOP.md` (tested-design loop, 2026-11-13). One branch `tested` in each of the three repositories, built from fp8. Inputs: `research/sherlock-e-path-20261112/AUDIT-opus.md` section 2 and gap 2 (with its erratum), `AUDIT-codex.md` gap A, and the owner rulings in `research/sherlock-processor-final-20261111/DECISIONS-ASK.md` (O1 to O12 of 2026-11-13).

## 1. Trees

| Repository | Worktree | Branch base | Merged tips |
|---|---|---|---|
| Moriarty | `~/Projects/Moriarty-wt/tested` | fp8 `bcae0f185` | final-design-2 `98ecb75b2`, stark `37dbe2fec`, bridge-lean `ecbce035f`, e4-gpu `418b1f267`, e5-wrap `f6db2bbd5`, e6-int `ef70c4949`, x-exp `e53cdd956`, quint-x12 `bf916a205`; enc-fp3 `7d3d9074e` and e3-iface `eccfc7d86` were already ancestors of fp8 |
| openvm | `~/.cache/m7-wrap/src/openvm-tested` (a worktree of the `openvm-fp8` clone; the `openvm-fp8` checkout stays on fp8) | fp8 `5d8e158` | e5-wrap `2f1d8a4`, e4-gpu `6865752`, enc-fp3 `f31a843` |
| stark-backend | `~/.cache/m7-wrap/src/stark-backend-tested` (a worktree of the `openvm-org_stark-backend` clone that T0 exported) | fp8 `5f698b5` (equal to e2-lfr) | e4-gpu `0b779c7`, enc-fp3 `65d1a03` |

## 2. Conflicts and their resolution

### Moriarty

No merge stopped on a text conflict. Each merge was checked as follows.

- final-design-2 into fp8 (`fa76c3512`). The one commit of final-design-2 changes 16 design files that fp8 also changed (ARCHITECTURE, DECISIONS, EXPERIMENTS, PROPOSAL, FINAL-DESIGN .md and .html, 10 OpenSpec files). Git merged them without overlap. Every final-design-2 hunk is an owner ruling of 2026-11-12 that fp8 did not carry: the salted `input_d = P2(SHK_IN, salt, val_d, fuel_lo, fuel_hi, stack_limit)` in three places of ARCHITECTURE (the Codex audit's stale formula), AD-17 (b) and AD-18 (a) written as decided, `w_stack` of `L` frozen at 48, the SRC-021 amendment in `sherlock-vm-consensus-20261107`, and the OD-11 reading. No fp8 text was replaced.
- stark into the result (`956ff75e3`). After final-design-2, stark changes no file in `research/sherlock-final-design-20261111`, `research/sherlock-processor-final-20261111` or `openspec/`. The 12 design files that the Opus audit found different between stark and fp8 differed only by final-design-2 and by fp8's own later edits. The merge adds bridge-lean `6661325d4`, mca-lean, d10-heights, msm-rust, s10x-node, the openvm patch series and the T0 export (313 files).
- Files that two merged sides both changed after their merge base with fp8: the design files above (final-design-2 and stark, one lineage); the Lean files `clean/fork/Clean/Sherlock/{Bridge,BridgeAxioms,FrLog,PvBinding,UDBits}.lean` and `research/sherlock-stark-bls-bridge-20261111/LEAN.md` (stark carries bridge-lean `6661325d4`, bridge-lean adds 2 commits on top, one lineage); `crates/mori-gpu/src/lib.rs` (msm-rust adds `table_builds`, 11 lines; e4-gpu adds two `mod` lines; disjoint, no duplicated item).
- bridge-lean, e4-gpu, e5-wrap, e6-int, x-exp and quint-x12 share no file with fp8 after their merge bases except the one-lineage files above.
- One source of truth: the merged design files have no duplicated section heading outside the per-kit template headings of ARCHITECTURE, and no precedence note was added.

### openvm

| File | Conflict | Resolution |
|---|---|---|
| `crates/sherlock-wrap-export/examples/lfr_e3.rs` (add/add) | The exporter exists as two rewritten histories: fp8 has 983767b, f453d73, e147918 (via e6-int), then be96611 (every keygen site through `keygen_frozen`); e5-wrap has c1ddfc0, 8ce7774, 6ef8f47, 1f42114, then 2f1d8a4 (the port to E3 9539418: the J8 fixture, R, anchor, `claim_pub`, tail and lane-map cases, `lane_map_ok`) | The e5-wrap 2f1d8a4 text (newer: it carries the E3 9539418 rules, the owner-ruled `SPONGE(3)` lane map and P-1 tags) with fp8's one rule applied: `sherlock_lfr::frozen::keygen_frozen(&engine, &airs)` in place of `engine.keygen(&airs)` (FP-8 item 3) |
| `crates/sherlock-wrap-export/examples/lfr_real.rs` (add/add) | Only the keygen line differs | The same rule: `keygen_frozen` |
| `crates/sherlock-wrap-export/Cargo.toml` (add/add) and `Cargo.lock` | Reported as conflicts; git rerere replayed the resolution recorded at the earlier e6-int merge of e5-wrap | Checked: the result equals the fp8 file, a superset of the e5-wrap dependencies (the E6 `lfr_call` dependencies); `cargo metadata --locked --offline` passes on the merged tree |
| `Cargo.toml` `[patch."https://github.com/openvm-org/stark-backend.git"]` (e4-gpu merge) | fp8 pointed at `../stark-backend-fp8`, e4-gpu at `../stark-backend-e4` | Both replaced by `../stark-backend-tested`, the merged stark-backend branch |

Commits: `5362863` (e5-wrap), `7eb2679` (e4-gpu), `356ab28` (enc-fp3, no conflict), `b34cbc5` (`enc_wrap` default path, section 3).

Clean merges checked against the keygen rule: `grep '\.keygen('` on the merged `sherlock-lfr` and `sherlock-wrap-export` finds `frozen.rs` (the frozen wrappers themselves), `fp8_keyset.rs` (fp8's own key-set generator) and `tests/e4_profile.rs` (e4-gpu). `e4_profile` measures the GPU prover with a log blowup override, a parameter set that `keygen_frozen` refuses by design, so it stays on `engine.keygen`. Listed for the lead.

### stark-backend

No conflict in either merge (`52cb2f8` e4-gpu, `4b8562f` enc-fp3). `Cargo.lock` is ignored in this repository; the tests use a lock generated offline from the local registry cache.

## 3. Per-lane copies replaced

| Where | Old | New |
|---|---|---|
| stark-backend `crates/stark-sdk/Cargo.toml` (`gpu` feature) | `mori-gpu` from `~/Projects/Moriarty-wt/e4-gpu/crates/mori-gpu` | `~/Projects/Moriarty-wt/tested/crates/mori-gpu` (commit `0004791`) |
| openvm `Cargo.toml` patch of the stark-backend crates | `../stark-backend-fp8` (fp8), `../stark-backend-e4` (e4-gpu) | `../stark-backend-tested` |
| openvm `crates/sherlock-wrap-export/examples/enc_wrap.rs` default input | `~/Projects/Moriarty-wt/enc-fp3/research/sherlock-enc-fp3-20261112/VECTORS.txt` | `~/Projects/Moriarty-wt/tested/research/...` (commit `b34cbc5`) |
| Moriarty `tools/sherlock-wrap/tests/i9_enc.rs` | the enc-fp3 worktree | the tested worktree |
| Moriarty `tools/sherlock-wrap/scripts/lock-check.py` | `openvm-org_openvm/Cargo.lock`, `Moriarty-wt/e2-lfr`, `Moriarty-wt/e5-wrap` locks | `openvm-tested/Cargo.lock`, `Moriarty-wt/tested` locks |
| Moriarty runner scripts: `research/sherlock-e2-lfr-20261112/{i0,i9,i10}.sh`, `research/sherlock-m7-wrap-20261112/lfr/run.sh`, `research/sherlock-e5-wrap-20261112/export*.sh` | `~/.cache/m7-wrap/src/openvm-org_openvm` (e2-lfr), `openvm-e5` | `~/.cache/m7-wrap/src/openvm-tested` |
| Moriarty `research/sherlock-e5-wrap-20261112/{i0,i3,i4,i4b,i5,i5real,i6,i7,i8,i9,lever}.sh` | `~/Projects/Moriarty-wt/e5-wrap/tools/sherlock-wrap` | `~/Projects/Moriarty-wt/tested/tools/sherlock-wrap` |
| Moriarty `research/sherlock-m7-wrap-20261112/{optE,s1brk,s1d10,s1ud,split}.py` | `Moriarty-wt/d10-heights`, `Moriarty-wt/m7-wrap` | `Moriarty-wt/tested` (the same files, merged through stark) |

The T1 test runner does not call the lane runner scripts: they still use the lanes' target and log directories. It uses `~/.cache/t1/run.sh` (section 4).

## 4. Tests

Runner `~/.cache/t1/run.sh <chain>` in user units `t1-sb` and `t1-ovm` (the `wrap` chain starts from `t1-ovm` after the exports). Every step runs through `heavy-slot --mem SIZE --name t1-STAGE`; `RUSTUP_AUTO_INSTALL=0`, stable toolchains from each `rust-toolchain.toml` (1.90.0 stark-backend, 1.91.1 openvm, 1.99.0 Moriarty), `RUSTFLAGS=-C target-cpu=native`, target directories `~/.cache/t1/target-{sb,ovm,ovm-gpu,wrap}`. Logs `~/.cache/t1/logs/<stage>.log`, stage times and tree heads in `~/.cache/t1/logs/runs.log`.

Fixtures are regenerated from the tested trees, never read from a lane directory: `E2_FIXTURE_DIR=~/.cache/t1/fix/e2` (`l_fixture`), `E3_FIXTURE_DIR=~/.cache/t1/fix/e3` (`iface_fixture`), `E5_FIXTURES=~/.cache/t1/fix/e5` (`fixtures`, `enc_wrap`, `lfr_e3`, `lfr_real`).

Scope: the crates and commands that the E1 to E6, ENC-FP3 and FP8 RESULTS files name, not the whole upstream openvm workspace.

Results: section 5 (filled when the units finish).

## 5. Status at handback (2026-11-13 09:4x)

Late tips merged with no conflict: x-exp `1eda7a42e` (`35f9dbd71`), quint-x12 `f9af0da9e` (`7ebaeb6f7`), e5-wrap `5853f5ad0` (`258676028`). Every listed tip is an ancestor of `tested`.

MEASURED so far (trees: sb `0004791`, ovm `b34cbc5`, Moriarty `c8c718040`):

| Stage | Command | Result | Log |
|---|---|---|---|
| ovm-build | `cargo build --release --locked --offline -p sherlock-lfr --examples --tests` | pass (4 min 51 s) | `~/.cache/t1/logs/ovm-build.log` |
| ovm-build-exp | `cargo build --release --locked --offline -p sherlock-wrap-export --examples` | pass | `~/.cache/t1/logs/ovm-build-exp.log` |
| fix-l | `l_fixture` into `~/.cache/t1/fix/e2` | pass; L proof 430,073 bytes, verify from disk ok, wall 333 s | `~/.cache/t1/logs/fix-l.log` |
| fix-iface | `iface_fixture` into `~/.cache/t1/fix/e3` | pass (51 min, shared slots) | `~/.cache/t1/logs/fix-iface.log` |

Still running in units `t1-ovm` and `t1-sb` at handback: lfr-suite (i0, i5_l, i9 and the rest with `--include-ignored`), the exports, the wrap suite, i10 gate, j5 gate, m7-bls, lfr-gpu, and the stark-backend stages (waiting for a heavy slot). Results land in `~/.cache/t1/logs/runs.log`; the lead fills the remaining rows. The late merges touched only `qa/` and `research/` plus the files listed in the commit log; any code they changed needs the wrap suite rerun on the new tip.

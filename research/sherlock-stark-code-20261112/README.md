# STARK code patches for the Sherlock E path

The E path code lives in two upstream clones, not in this repository. This folder carries their commits as `git format-patch` series, so the branch `stark` holds the full workstream.

| Folder | Upstream | Base commit | Branch in the clone |
|---|---|---|---|
| `stark-backend/` | https://github.com/openvm-org/stark-backend | see `stark-backend/BASE` | `e2-lfr` (E1 Fr base field, M7b Poseidon config, E2 tags and height bound) |
| `openvm/` | https://github.com/openvm-org/openvm | see `openvm/BASE` | `e2-lfr` (M7 bench crate, `sherlock-lfr` crate: the L_Fr tape through I10, the E2 gate) |
| `openvm-e5/` | https://github.com/openvm-org/openvm | `c4bd312` (the top of `openvm/`) | `e5-wrap` (the `sherlock-wrap-export` crate for the E5 wrap fixtures) |

Apply with `git checkout $(cat BASE) && git am *.patch` in a clone of each upstream. The OpenVM workspace uses the backend through its path or git dependency; see the M7 and E2 RESULTS.md files for the build setup.

## Lane branch series (T0, 2026-11-13)

Each folder below holds one lane branch of a clone under `~/.cache/m7-wrap/src/`, exported against the upstream `main` that the branch forked from. `BASE` holds the full base commit and `TIP` the full branch tip. Every series is independent: apply it alone on its base.

| Folder | Branch | Base | Tip | Patches | Commits on the branch | Tip subject |
|---|---|---|---|---:|---|---|
| `openvm-e2-lfr/` | e2-lfr | `05f3b73e7` | `c4bd312be` | 9 | 9 | sherlock-lfr: I10 E2 gate at the L_Fr parameter set (PublicAir, D-2) |
| `openvm-e3-iface/` | e3-iface | `05f3b73e7` | `953941809` | 19 | 19 | sherlock-lfr: E3 D-9 lane map as apply_lane_map |
| `openvm-e3-iface-onion/` | e3-iface-onion | `05f3b73e7` | `f725b1594` | 15 | 15 | sherlock-lfr: E3 J8 guards |
| `openvm-e4-gpu/` | e4-gpu | `05f3b73e7` | `68657522e` | 12 | 12 | E4: GPU fractional sumcheck equality test; log blowup override in the profile |
| `openvm-e5-wrap/` | e5-wrap | `05f3b73e7` | `2f1d8a435` | 24 | 24 | sherlock-wrap-export: port to E3 9539418 |
| `openvm-e6-int/` | e6-int | `05f3b73e7` | `780db4ae1` | 15 | 20, 2 merges | Merge e5-wrap e147918 (lfr_e3 exporter) into e6-int |
| `openvm-enc-fp3/` | enc-fp3 | `05f3b73e7` | `f31a84381` | 10 | 10 | sherlock-lfr: ENC and FP-3 vectors on the L_Fr machine AIRs |
| `openvm-fp8/` | fp8 | `05f3b73e7` | `5d8e15883` | 21 | 33, 3 merges | fp8_keyset: univ mode (bound N1 to N3 and the L pins for two programs) |
| `openvm-fp8-units/` | fp8-units | `05f3b73e7` | `1ba98abf6` | 21 | 33, 3 merges | FP-8 units: N3 reference key in the bitcode check |
| `openvm-tested/` | tested (T1 integration, T3) | `05f3b73e7` | `b3fbaa5` | 27 | 48, 6 merges | T3: every recursion layer keyed with every AIR required |
| `openvm-t9-contract/` | t9-contract | `05f3b73e7` | `01eb63f` | 30 | 51, 6 merges | sherlock-lfr: T9 ENC-2 enc2.onion6 on the onion AIR |
| `openvm-t6/` | t6-leaf (T6 modular leaf slice) | `b3fbaa5` (the `tested` tip) | see `TIP` | see folder | linear | T6: `sherlock-leaf` slice crate, `KeyBind` app public values, `slice_chain`, `fp8_d3` |
| `openvm-t7/` | t7-bind (T7 `prog_d` and key route) | `8672651` (the `tested` merge of `t6-leaf` and `t9-contract`) | `662235f` | 3 | linear | T7: FP-2 statement AIR, `L` root statement and `RootSealAir`, `t7_chain` |
| `openvm-t8/` | t8-gp (T8 GP hooks) | `662235f` (the `t7-bind` tip) | `a645de3` | 4 | linear | T8: GP-2/GP-4 node mode block `gp.rs`, MERGE-WORK node (`Kind::Work`, split CORE, `MERGE_WORK` set), `t8_gp` (x7, gp2) |
| `openvm-t8b/` | t8b-gp (T8b GP hooks) | `a645de3` (the `t8-gp` tip, on `tested`) | `2325f63` | 4 | linear | T8b: chip set 2 value set (`value_set`), NW parent ladder (`keys_d` 7 rows), kernel CORE step words as columns, honest GP-2 SEAL-WITH-DEPS with twins, `x14_units` |
| `stark-backend-e1-fr-base/` | e1-fr-base | `a6b4761ad` | `30407617d` | 5 | 5 | E1: exact statistical distance of sample_bits at BLS12-381 Fr |
| `stark-backend-e2-lfr/` | e2-lfr | `a6b4761ad` | `5f698b56e` | 7 | 7 | E2: per-bus trace-height bound is the base field order capped at u32::MAX |
| `stark-backend-e4-gpu/` | e4-gpu | `a6b4761ad` | `0b779c74a` | 9 | 9 | E4: GPU fractional sumcheck hook for the Fr config |
| `stark-backend-enc-fp3/` | enc-fp3 | `a6b4761ad` | `65d1a03d4` | 10 | 10 | stark-sdk: FP-3 closure tests on the Rust reference |
| `stark-backend-fp8/` | fp8 | `a6b4761ad` | `5f698b56e` | 7 | 7 | same tip as `stark-backend-e2-lfr/` |

Branches without merges are plain `git format-patch --binary main..BRANCH`. A branch with merges (e6-int, fp8, fp8-units, tested, t9-contract) is exported along its first-parent history: each non-merge commit is one `format-patch` file, and each merge is one patch of its diff against its first parent, with the merge named in the message. So `git am` gives a linear history whose final tree equals the tip tree.

Apply: `git checkout $(cat BASE) && git am *.patch` in a clone of the upstream. The openvm clones are shallow, so fetch the base first (`git fetch --depth 1 origin $(cat BASE)`).

Round trip checked for all 14 series (2026-11-13, scratch clones under `~/.cache/t0-verify`, deleted after the check): the base plus `git am` of the series gives a tree equal to the tip tree (`git diff --stat` against the tip is empty and the tree ids match). The fp8 tree is `95e9c51`, and the stark-backend e2-lfr and fp8 tree is `eb17157`.

T3 (2026-11-13): `openvm-tested/` added the same way (first-parent export, merges flattened); round trip checked in a scratch clone: tree `39610c3` equals the `tested` tip tree. stark-backend `tested` has no T3 commit.
T9 round trip (2026-11-13): `openvm-t9-contract/` on its base gives tree `60db66a`, equal to the tip tree. The branch carries T3 (`b3fbaa5`) and the T9 commits `c7c0c2e` (ENC-4/5), `609c4f1` (decoder, `lfr_call`) and `01eb63f` (onion6 on the onion AIR).

T6 (2026-11-13): `openvm-t6/` and `stark-backend-t6/` are plain `git format-patch --binary BASE..t6-leaf` series. `openvm-t6` applies on the `tested` tip `b3fbaa5` (apply `openvm-tested/` first). `stark-backend-t6` applies on its `BASE` (the stark-backend `tested` tip) and adds `SB_MUTANT_PROVER` (test only). The openvm-t6 `Cargo.toml` patches point at `../stark-backend-t6/crates/...`.

T7 (2026-11-13): `openvm-t7/` is a plain `git format-patch --binary 8672651..t7-bind` series. It applies on the openvm `tested` commit `8672651` (the merge of `t6-leaf` 3160f59 into `tested`). The `Cargo.toml` patches still point at `../stark-backend-t6/crates/...`; stark-backend has no T7 change. Report: `research/sherlock-t7-bind-20261113/RESULTS.md`.

T8 (2026-11-14): `openvm-t8/` is a plain `git format-patch --binary 662235f..t8-gp` series. It applies on the openvm `t7-bind` tip `662235f` (series `openvm-t7/`). The `Cargo.toml` patches still point at `../stark-backend-t6/crates/...`; stark-backend has no T8 change. Report: `research/sherlock-t8-gp-20261113/RESULTS.md`.

T8b (2026-11-14): `openvm-t8b/` is a plain `git format-patch --binary a645de3..t8b-gp` series. It applies on the openvm `t8-gp` tip `a645de3` (series `openvm-t8/`). Patch 0001 points the `Cargo.toml` patches at `../stark-backend-t8b/crates/...`, a copy of stark-backend `deeeb0a` with no change. Report: `research/sherlock-t8b-gp-20261114/RESULTS.md`.

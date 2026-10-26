You work on Moriarty Sherlock U023 scenario 2 in this worktree (branch u023-witness, from sherlock-s10). Your part is gap G5, the host witness stream. Read first: `docs/sherlock/stagenet-gp-recursion.md` (sections 3 and 5), `docs/sherlock/design/c6-recursion-adapter.md`, `crates/mori-recursion/src/{ivc.rs,standin.rs,run_decider.rs}` (the frozen digests: `state_digest(tag, value, lanes_d)`, `DOM_STATE_D`, `state_digest_halted`), `crates/mori-backend/src/sherlock_c6_child.rs`, and how `crates/mori-backend/src/sherlock_machine/c6_dispatch_tests.rs` builds `run(source).steps` and calls `witness_from_host_snapshot`.

Implement a library API in `crates/mori-backend`, plus a small CLI or example, that takes a committed program source and a host config path. It runs the owner machine and emits, for every step: the C6 step witness (pre and post carriers, fired rule, h_pre), exactly as the dispatch tests construct it; prog_d and init_d; and the off-circuit pre_d and post_d, using the frozen digest definitions byte for byte. The output format is JSON lines, `moriarty-sherlock-c6-witness-stream/1`, holding public data only.

The leaf circuit (another worker) must compute lanes_d in-circuit over the carrier lanes. Find the intended definition (the docs above and `sherlock_c6_child.rs`) and use it. If none exists, define it once, in one place, with a doc comment that the leaf must match, and say so in your summary.

A sibling worker is creating `docs/sherlock/qualification/qualification-host-config.json`. Until it lands, make the config a parameter and test with a provisional copy of `qa/holdout/config/draft-host-config.json` kept inside your test fixtures.

Tests, for `entry 7;` and `entry 3 + 4;` (and fact3 if it stays under a few minutes in debug):
(a) post_d of step i equals pre_d of step i+1 for every step;
(b) the genesis pre_d equals init_d;
(c) the last post_d equals the halted state digest whose result is the `mori simulate` result;
(d) h increments by one per step;
(e) changing any carrier lane changes the digest.

Constraints: work only in this worktree. Do not commit, push, install or use the network. Build and test only with `RUSTC_WRAPPER= CARGO_TARGET_DIR=<this worktree>/target CARGO_BUILD_JOBS=4` (sccache is broken on this host), and keep any single command under about 20 GB RSS. Never run `scripts/doctorwatson heavy` or `check`, and never edit `qa/holdout/` (evaluator-owned). Write tests before behaviour changes. Match the surrounding code style and comment density. Reply DONE with a six-line summary including exact numbers and the test commands you ran.

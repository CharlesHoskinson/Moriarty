# REPORT: independent source, effect and resource review of the P3S1-R1 Rust R-1 request

## Lead: **REPAIRS_REQUIRED**

**Predicate.** Can these exact frozen bytes safely and reproducibly run only the 74 frozen test predictions
and the separate groth16 feature compilation, in the declared filesystem view and fixed bounds, with honest
result classification?

**Answer: not as frozen.** The containment design and the scope are sound at source. The bwrap view
removes host HOME, credentials, other tasks and Cargo ancestor config by construction. Every Cargo,
rustc, build-script, proc-macro, test and lock-gate process runs inside it. Process lifetime is bounded,
the lock is exact, and nothing runs a setup, key, proof or verification. Three defects block one-shot
authority:

1. **E-01 (HIGH):** the candidate's in-circuit BLAKE2b keeps XOR results as growing linear
   combinations. ark-relations 0.4 stores every one of them. Each SettlementHashV2 synthesis over the
   4–8-block statements is therefore predicted to exceed the fixed 8 GiB `RLIMIT_AS` and the R5 CPU
   budget. The `abi2` and `abi2_r1` binaries would end by abort or SIGXCPU, and up to 28 ABI2
   predictions would yield no result, so the single attempt for the ABI2 claim would be spent.
2. **E-02 (MEDIUM):** the classification has no class for a test binary or compiler that dies by
   signal or allocation failure. Cargo reports such a death as exit 101, so it would read as
   FAILED(compile) or stay unlabelled.
3. **E-03 (MEDIUM):** F1's host-side `sha256sum -c` over the sandbox-writable `cargo-home/registry`
   follows symlinks that a sandboxed build script could plant toward host private files.

The exact repair conditions are in §8 and `EXECUTION-ELIGIBILITY.json`. Bwrap capability stays UNKNOWN
until Q1/Q2, and no fallback is permitted.

## 0. Identity, routing, method

| Item | Statement |
|---|---|
| Reviewer | `claude-opus-5-5`, as the runtime states. Requested Opus 5.5 at high effort; actual effort is not observable. This is a fresh context. I authored none of the reviewed bytes, votes, candidate, helpers or precedents. Every seat in this lineage is the same model family, so independence is at the context level only. |
| Author identity | I take it from `TERMINAL-RECOVERY.json` (root public metadata): provider result `success`, `is_error=false`, canonical `claude-opus-5-5` firstParty. The original CLI numeric exit is **UNKNOWN** and is never inferred to be 0. That record does not substitute for this review. |
| Startup | I read the installed `develop/SKILL.md` and `references/execution-focus.md`, plus the full-integration `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/audits/2026-10-14/SECURITY-ROUTING.md`. The root refreshed the guarded CLI; the stale SP01 campaign is history and selects nothing here. Opus owns every security and crypto judgment below. Public developers remain permissionless. |
| Tools | Read, Write and Edit only. No Bash, Grep, Glob, listing, interpreter, parse, hash, Cargo, rustc, bwrap, network, probe or subagent. Every path came from the request, a manifest, a report or a source include/manifest reference. I verified **no hash**. Counts and lengths below are hand work over source text. |
| Not opened | Provider, stderr, PROMPT, PROCESS, FINAL and wrapper files; credentials and keys; RR03/W0/W1 drafts; binary bodies; the future target folder. |
| Writes | Only the files in `OWN-OUTPUT-ALLOWLIST.json`, inside this folder. No candidate, request, oracle, checkout, library or registry byte was edited. |
| Coverage | All 33 request files and all 60 candidate rust rows were read in full, including every embedded fixture, expectation and the BUILD manifest. Exact modes are in `READ-COVERAGE.json`. |

## 1. Containment and effect audit (judgment 1)

**Argv and invocation.** Each step runs as `env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC
/usr/bin/bash --noprofile --norc steps/<ID>.sh < /dev/null`. The wrapper is
`nice 10 → timeout --kill-after=60 W → prlimit --as=8 GiB --cpu=C --core=0 → bash -o pipefail -c "{cmd} | tee log"`
and its exit goes to `<ID>.exit`. Self-gating reads the prerequisites' `.exit` files. The order is
P1, P0, P2, Q1, Q2, R0, R0p, R1, R2, R3, R5, R4, F1, H1, F2. R4 needs R2, not R5, so it runs even after
an R5 failure.

**View (`sbx`).** `--unshare-user/-pid/-net/-ipc/-uts`, `--die-with-parent`, `--new-session`.
Read-only: `/usr /bin /lib /lib64 /etc/alternatives`, `--ro-bind-try` for the three ld.so paths,
`$TC` at its own path, and `/task/{r1cs,moriarty,gate,probe}`. `--dev /dev`, `--proc /proc`,
`--size 256 MiB --tmpfs /tmp`. Writable: `/task/{cargo-home,target,home,tmp}` → `<W>/…`. `--chdir /task`,
`--clearenv`, then 11 `--setenv`. Every option is in the captured 0.11.1 man page. `--clearenv` keeps
`PWD` (= `/` here); that is harmless. The eight `sbx` blocks read identically by my line comparison;
the root should confirm byte equality mechanically (E-13).

**Reads and writes.**

| Actor | Reads | Writes |
|---|---|---|
| Host P0, P2, F1 (coreutils, `freeze.py`) | 60 candidate rows, 70 Core files, 194 registry files, 12 absolute pins, own `data/` | P1/P2 under `<T>` only. F1 writes only its log. |
| Sandbox (Cargo, rustc, build scripts, proc-macros, tests, `lock_gate.py`) | `/usr` etc., the toolchain, the four read-only task binds, the writable binds | Only the four writable binds (host-backed), plus the in-RAM tmpfs root and `/tmp`, which vanish on exit |
| Host H1 and F2 | `R5.log`, the placed BUILD manifest, `logs/` and `receipts/` | `receipts/*`, `task-final.json`, `F2.exit` |

**Host ancestors and config.** Cwd and `/` are synthetic, so Cargo sees only `/task/.cargo` and
`/.cargo`. Both are absent, and Q2 checks them. `CARGO_HOME` has no config (P2, Q2 and F1 check).
The Core workspace's `[patch.crates-io]` and profiles are inert because `mori-r1cs` declares its own
empty `[workspace]`. The path dependency is resolved with workspace inheritance from
`/task/moriarty/Cargo.toml`, which is present among the 70 files. R3 used the same relative layout.

**Build scripts and proc-macros.** I read all 14 build scripts selected by the lock (the author's 13
plus `quote`), at public host-extracted paths. Their effects: env reads, `$RUSTC --version`, probe
compiles into `OUT_DIR`, writes to `OUT_DIR`, and zerocopy reading its own `Cargo.toml`. They make no
network or HOME access. The union analysis records `build_rs_in_crate=false` for ark-ff 0.4.2, so its
`rustc_version` build-dependency runs no script. I did not read the nine proc-macro crates; the view is
their containment. There is no `cc` crate.

**Lifetime.** `timeout` signals its own process group. That reaches bash, tee, `/usr/bin/time` and the
outer bwrap. `--new-session` moves only the sandbox child. When the outer bwrap or its parent dies,
`--die-with-parent` (PDEATHSIG SIGKILL) and the pid-namespace teardown end every descendant. This
closes R3's surviving-child residual.

**Escapes considered.** Read-only binds are inherited, locked mounts, so a nested user namespace cannot
remount them read-write. `/proc` is the sandbox pid namespace. `/sys`, pathname sockets, `/init`,
`/run/WSL` and `/mnt` are absent, so WSL interop is unreachable. Abstract sockets are per network
namespace, and Q2 gates loopback-only networking. The residual risk is the in-RAM writable root (E-07,
optional `--remount-ro /`).

**Probe (Q2).** It runs inside the identical `sbx` and uses only task-owned paths. The host canary is
the positive control: because `<T>` lies under `/home/charl`, its absence inside the view shows that
host HOME is not visible. The probe also checks CapEff 0, `lo` only, at most 16 pids, the exact
`/task` entries, no Cargo config, read-only inputs (including a write attempt into the read-only
`/usr` bind) and writable outputs, then compiles, links and runs `main.rs` with the pinned rustc. It
never names, stats or reads a private path. It does not exercise `python3 -I`/`tomllib` (E-06).

**Symlinks and path expansion.** `/bin`, `/lib` and `/lib64` resolve through bwrap's bind to their
`/usr` directories. P2 and F1 reject non-regular files in the read-only trees. Every `include!` path
resolves inside `/task` (§2). The one unsafe expansion is host-side: F1 `cd`s into and hashes the
sandbox-writable registry and follows links (**E-03**). `du`, `find` and `ls` as scripted do not follow.

**Pins.** P0 checks 60 + 70 + 194 rows and 12 absolute pins (R1 manifest, closure pins, OPTION-A plan,
`lock_gate.py`, union, R3 lock, `freeze.py`, bwrap, four toolchain files), the lock size, tool presence
and disk. F1 re-checks all of them after the sandboxed steps. The registry checklist matches
OPTION-A-PLAN on every row I compared. The R1 lock has 98 packages, 93 of them registry packages whose
checksums equal the `.crate` pins. Not pinned: python3 (which runs the gate and the freeze), bash,
coreutils, gcc/ld/glibc and `time` (E-08, precedent residual). bwrap's set-id bit is recorded but not
gated (E-09).

**Candidate, source and registry hygiene.** P0 and F1 assert no `Cargo.lock`, `target` or `.cargo` in
the packet. The lock lives only in the read-only `<W>/r1cs` copy. The checkout and registry are never
bound into the view, so the sandbox cannot write them.

## 2. Expected tests and include layout (judgment 2)

- **74 confirmed by my count from source:** accounting 3, arithmetic 8, assignment_shape 2,
  duties_frame 6, generic_pipeline 6, propose 3, refusals 9, tamper 4, abi2 18, abi2_r1 10,
  build_identity 5. Every name equals `data/expected-74-names.txt`, and the names are unique across
  targets, so H1's `comm` is sound. No `#[test]` or `#[cfg(test)]` appears in `src`, and the only doc
  fence is `text`. That gives 13 `test result:` lines.
- **Includes: 115 expansion sites and 115 distinct targets** (45 candidate files + 70 Core files):
  `build_identity.rs` has 19 `own!` + 70 `core_file!`, `tests/common/mod.rs` has 19 `packet!`, and
  `abi2.rs` 1, `abi2_r1.rs` 5, `tests/build_identity.rs` 1. That is 10 literal `include_*` tokens. The
  requested **116 is not reproducible from source**; I preserve the qualification that its counting
  rule is unknown (for example, counting `mod common;` or the path dependency). It is non-blocking
  because every row and every Core file is copied.
- **BUILD manifest:** I hand-summed the canonical `mori-r1cs-build-manifest/2` document:
  10 + 19×96 + Σpaths 256 + Σdigits 87 + 18 commas + 40 = **2,235 bytes**, which equals the frozen file
  size. So there is no trailing newline, and the prior review's risk is resolved at source; content
  equality rests on the root-verified hashes. The rows' sha and bytes equal the root freeze.
- **Statement sizes** (from the fixtures): P01 771 B, P02 721, P03 659, P04 952, P05 673. H1 is 838 by
  my body count, wide-carry 507. That is 4–8 BLAKE2b blocks each.
- **Never-compiled code:** I type-checked the R1-new paths in `abi2.rs`, `blake2b.rs`,
  `build_identity.rs` and `tests/abi2_r1.rs` by hand against Core types (`AssetId.scale: u8`,
  `Ord`; `Type: Copy`) and found no compile error. That is a prediction only.
- **Gaps:** none in the request or the rust rows. Partial reads (two Core files, the union analysis,
  the R3 lock window, ark-relations) are listed with their line ranges.

## 3. Resources (judgment 3)

**E-01 mechanism, verified in source.**

- `blake2b.rs:143-165` returns `a + b − 2p` and does not allocate the XOR output.
- In `g` (`:241-248`) the `b` and `d` words are only ever replaced by XOR results. At the block
  boundary `h[i] = h[i] ⊕ v[i] ⊕ v[i+8]` (`:272-275`), and the next block starts from `h` (`:254`).
- So per bit, a `b` word gains about 96 terms per compression, and `h[4..7]` carries its own previous
  value at two rotation offsets (12·2·(24+63) ≡ 40 mod 64).
- ark-relations 0.4.0 `new_ref()` defaults to `Prove{construct_matrices:true}` and stores every LC in
  `lc_map` (`constraint_system.rs:147-153, 242-275`).
- `compile()` always runs a Setup probe (`circuit.rs:275-307`): synthesis, then `finalize()` (which
  holds two full maps at once, `:309-393`), then `to_matrices()` (a third copy, `:526-567`), then
  per-term serialisation.

**Magnitude (hand estimate, unmeasured).** Stored terms are about 1.3×10^8 for 6 blocks, 1.9×10^8 for
7 and 2.8×10^8 for 8, at about 48 B each: roughly 6, 9 and 13 GB per prove-mode synthesis, before the
probe's 2.5–3.5× peak. Even if this is 3× too high, the 6-block probe still meets or exceeds 8 GiB,
and two test threads compound it. Debug-profile CPU is minutes per synthesis.

**Exposure.** `abi2` performs 11 V2 compiles, 5 proving syntheses and the 721-byte gadget vector.
`abi2_r1` performs 10 compiles and 14 syntheses. Cargo runs these two binaries before the 46 others.

**Envelope verdict (V-6: CHANGE).** The other values (8 GiB AS, 2 jobs, 2 threads, 6 GiB disk, walls,
kill-after 60, worst case 12,600 s) are proportionate for compile steps and for a bounded-term circuit.
With the allocated-XOR repair, a 6-block synthesis is about 2×10^6 terms (~100 MB), comfortably inside
R5 3600/3600. Two gaps remain even then:

- RLIMIT_AS is per process and not a host RAM bound on a shared WSL2 VM with live workers. E-04 adds a
  recorded MemAvailable ≥ 10 GiB gate.
- Child deaths need honest classes (E-02).

Replacement text **V-6-r** is in `VOTES.md`.

## 4. Votes (judgment 3)

The full literal table is in `VOTES.md`.

- **FOR:** V-1 scope, V-2 mechanism, V-3 mounts (qualified: PWD kept; RAM-unbounded synthetic root),
  V-4 lock, V-5 registry, V-7 probe, V-9 interpretation.
- **CHANGE:** V-6 (→ V-6-r: MemAvailable gate, a bounded-gadget precondition, child-death
  classification) and V-8 (→ V-8-r: accurate deletion wording, F1 link guard).

My CHANGE is not a FOR on the original text. V-6-r and V-8-r carry one vote each.

## 5. Scope and result handling (judgment 4)

- **Scope.** No setup, key, SRS, prove or verify path exists in any test. `groth16.rs` (verify only)
  compiles only in R4, and R4 can never reclassify R5. A genuine proof is still absent.
- **Distinct classes.** Forced-failure (hostile) tests are ordinary predictions graded HOSTILE-AUTH.
  Layout errors are BLOCKED(layout). Resource events are BLOCKED(resource), once E-02 adds the
  child-death rows. Lock, fetch, capability and hygiene each have their own class.
- **After failure.** F1 runs whenever P1 succeeded. H1 runs whenever `R5.log` exists and decides
  nothing. F2 runs last whenever `logs/` exists; it freezes logs and receipts with the byte-identical
  `freeze.py`, which hashes before it creates its output (`open "x"`), so `task-final.json` is not
  self-included. Journals stay unfrozen but publishable (E-12, record-only).
- **Deletion.** No operator `rm` beyond the probe markers. Tool-internal deletions inside the writable
  binds are inherent, so V-8's wording is corrected in V-8-r. `<W>` is retained until the result
  review. Publication excludes `<W>`, binaries and HOME.
- **Derived build digest.** H1 hashes the placed frozen manifest, then `printf`s the canonical
  `{compiler_source_manifest_sha256, core_closure_sha256, format}` with keys in Core's sorted order,
  compact, lowercase hex. This equals what the binary computes **only if**
  `embedded_build_manifest_equals_frozen_bytes` and `embedded_core_closure_equals_pinned_closure` both
  pass. It is sufficient for "a value derived from those two assertions" and for nothing more: it is
  not an observation of the binary, and not binary provenance.
- **Still open.** Arbitrary-witness soundness, arbitrary-program generality and full currentness
  (latest main, Sherlock, staged `/2`).

## 6. Oracle provenance, graded separately

| Grade | Rows | Independence |
|---|---|---|
| Non-author | Sol `/1` lifecycle vectors (six reused fixtures, staged plans) | Their own review is pending. Not independent of each other. |
| Non-author | H1-P1 held-out record (HELDOUT-VECTORS, frozen before that reviewer read the compiler) | Accounting comparison only. The R1 V2 context is synthetic. |
| AUTH via independent implementation | hashlib BLAKE2b-256 vectors | Python hashlib versus the crate's Rust code. Inputs chosen by the author. |
| AUTH | P01–P05 bodies, statements, public inputs, shape bytes, wide-carry, BUILD-MANIFEST-R2, AUTHOR-EXPECTATIONS/ADDENDUM | Same author lineage as the generator. Cross-language agreement, not an oracle. |
| Reviewer hand checks (this review) | 74-name membership, include count, manifest length, statement sizes, H1 debit cells (buyer USDX 250000/1250; seller PTS 5/0) agreeing with the frozen H1-P1 accounting | Hand arithmetic over source, not execution. |

No same-author expectation is treated as independent evidence of financial correctness.

## 7. Defects I could predict before authority

No deadlock found: every step is sequential, stdin is `/dev/null`, there is no TTY, a single Cargo runs
at a time, and a read-only lock is only read and flocked.

Predictable failures:

- **E-01**, the R5 ABI2 resource exhaustion (blocking).
- **E-06**: a python environment fault would be mislabelled BLOCKED(lock).

Otherwise I found no predictable compile, probe or layout failure at source. Q1/Q2 capability is
genuinely unknown; an AppArmor or userns restriction there would be an honest BLOCKED(capability).

## 8. Minimal concrete repair (judgment 5)

1. **RQ-1 (E-01, blocking).** Either:
   - **(a)** apply `proposals/BLAKE2B-XOR-ALLOCATION.md`: allocate the XOR output with
     `(2a)·b = a + b − c`. Constraint and witness counts and the digest are unchanged, and stored
     terms become linear in blocks. Re-freeze `blake2b.rs`, BUILD-MANIFEST-R2, the build-digest
     prediction and the R-1 data rows, and get a fresh Opus 5.5 review of the changed bytes. The
     optional linear-weight guard test is in the same file. Or:
   - **(b)** re-vote a 46-test R-1′ that compiles but does not run `abi2`/`abi2_r1`.
2. **RQ-2 (E-02, blocking):** child-death classification rows (`proposals/R1-STEP-REPAIRS.md`).
3. **RQ-3 (E-03, blocking):** F1 link guard before hashing `cargo-home`.
4. **RQ-4 (E-04) and RQ-5 (E-05):** the MemAvailable gate, and adopt V-6-r and V-8-r.
5. **Recommended:** the R5a/R5b/R5d split; RQ-6; RQ-9; optional `--remount-ro /`; tool-version
   records; root mechanical `sbx` equality.

No new admin framework is required. Changed bytes get one fresh review plus the votes.

**Next eligible demonstrated capability.** R-1 on the repaired candidate under the repaired request:

- the first in-view compile of the never-compiled ABI2 and BLAKE2b code,
- all 74 constraint-level predictions, including in-circuit BLAKE2b-256,
- the R1 byte-derived build identity,
- the groth16 compile-only step,

in one authorized attempt, followed by a fresh actual-result review. The fallback without new
candidate bytes is R-1′ (46 predictions, all compiles), which leaves every ABI2 claim unexercised.

## 9. Preserved obligations and non-claims

No security, target, proof, adoption or execution authority comes from this review. Native and SDK
authority stays 0. Still open:

- RR-03, all 20 pattern families and the original 42/52 gap;
- ACTUS 277/18/32/DS01–07, DeFi 72/DA01–24/held-outs/intents/regressions, and general financial
  obligations;
- recursion/PCD, private handoff and multi-parent composition;
- setup, VK provenance, proof and verification; Cardano, ledger, Preprod and Midnight;
- arbitrary-witness soundness and full Rust currentness;
- every U0–U7, M1–M17, MC01–MC08, SP01–SP12 and G01–G24 exit.

Public developers remain permissionless: this is a maintainer-operated check only.

## 10. Limits and disclosures

- E-01's magnitude is hand arithmetic. Its mechanism is source-verified in the candidate and in public
  ark-relations 0.4.0, read at the host-extracted path, which is not the pinned archive.
- I verified no hash. Core checkout bytes were read live and are not pin-verified by me.
- I did not read the proc-macro crates' sources.
- I could not observe my own effort or exit.

## 11. Files

`REPORT.md`, `FINDINGS.json`, `VOTES.md`, `READ-COVERAGE.json`, `EXECUTION-ELIGIBILITY.json`,
`proposals/BLAKE2B-XOR-ALLOCATION.md`, `proposals/R1-STEP-REPAIRS.md`, and `OWN-OUTPUT-ALLOWLIST.json`
(written last).

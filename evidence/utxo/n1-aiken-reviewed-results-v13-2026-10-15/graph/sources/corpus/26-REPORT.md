# REPORT: P3S1-R1 Rust R-1 execution readiness (source-only proposal)

**Predicate:** Can the existing P3S1-R1 compiler and ABI2 candidate (`mori-r1cs 0.0.2-p3s1.1`, 74 predicted tests) be compiled and tested under pinned Rust 1.99.0, the offline 102-crate union and a filesystem view that hides host HOME and private files from build scripts? Can this be done with exact pins and argv, reuse of existing mechanisms, and no new runner?

**Answer at source: yes.** This folder holds that exact proposal: 15 literal step scripts, the pins and checklists, the exact lock bytes, a bounded capability probe and graded expectations. The only capability not yet observed is the first bwrap execution. Steps Q1 and Q2 observe it, and if either fails the run stops as BLOCKED(capability), with no alternate mechanism and no retry. I found no source-level blocker. Nothing here has run, and none of it is authority.

## 1. Separate axes

| Axis | State |
|---|---|
| Implementation (this folder) | PROPOSED SOURCE: `EXECUTION-PROPOSAL.json`, `steps/*.sh` (15), `data/**` (checklists, exact lock, probe fixtures, 74 names), `LAYOUT-MAP.json`, `DEPENDENCY-COMPARISON.json`, `EXPECTED-TESTS.json`, `VOTES.md`. Unreviewed, unparsed and never executed. |
| Evidence | None new. The R3 run is historical-local evidence for its own tree only. The 74 tests, the BUILD/2 manifest `46454f9f…` and the build digest `472e6540…` remain predictions. |
| Acceptance | None. No product, runtime, crypto, security, ledger, Cardano, Preprod or Midnight acceptance. RR-03, all 20 families, the 42/52 gap, ACTUS, DeFi, recursion, private handoff, multi-parent composition, Rust full currentness and every U/M/MC/SP/G exit stay open. |
| Authority | Native/SDK authority stays 0. The run needs a fresh Opus review of these exact bytes, two substantive votes per clause V-1..V-9, root hashing and pre-launch re-verification, and root authorization (`EXECUTION-PROPOSAL.json` `root_prerequisites_before_P1`). |

## 2. Identity, routing, tools

- Author: `claude-opus-5-5`, as the runtime states. Requested Opus 5.5 at high effort; I cannot observe the actual effort, and the root verifies the identity. This is a fresh context. All security judgments here are mine (Opus routing). Every seat in this lineage is the same model family.
- Startup: I read the installed `develop/SKILL.md` and `references/execution-focus.md`, plus the full-integration `AGENTS.md`, `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/audits/2026-10-14/SECURITY-ROUTING.md`. The root refreshed the guarded status; I ran no CLI. The historical SP01 queue is not this task. Public language use remains permissionless.
- Tools: Read, Write and Edit only. I ran no Bash, Grep, Glob or listing, no interpreter, hash, Cargo, compiler, bwrap, Docker or network, and started no subagent. I located every path through a manifest, a report or the request. I opened no provider-result, process.stderr, prompt, process, final or wrapper file, no credential, key or private file, and no live Aiken, RR03 or C5 draft. Every hash is copied from a named source, not verified by me. My writes are only the files in `OWN-OUTPUT-ALLOWLIST.json`.

## 3. What the candidate actually does (my reading of all 98 rows' Rust part)

- **Tests.** I counted by hand: accounting 3, arithmetic 8, assignment_shape 2, duties_frame 6, generic_pipeline 6, propose 3, refusals 9 and tamper 4 give 41; abi2 has 18, abi2_r1 10 and build_identity 5, for **74**. No `src` file has `#[test]` or `#[cfg(test)]`. The only doc-comment fence (`src/abi2.rs`) is `text`. So there are 0 unit tests and 0 doctests, and 13 `test result:` lines are expected.
- **Effects.** None of the 18 `src` files or the 12 test files uses `std::fs`, `std::env::var`, `std::process` or `std::net` at run time. The tests read only bytes embedded at compile time. Inputs therefore enter only through `include_bytes!`/`include_str!`, which resolve at compile time inside the view.
- **Includes.** There are 115 macro sites and 115 distinct targets: 45 candidate files and 70 Core files (`LAYOUT-MAP.json`). The request says 116. My count over every source file gives 115, and the copied tree (all 60 rust rows plus all 70 Core files) covers either reading. I record this as a reconciliation item, not a blocker.
- **Statement size.** P02's statement is 343 + 378 = 721 bytes, so 6 BLAKE2b blocks. By my hand count that is about 300k constraints per V2 synthesis, and there are roughly 40 such syntheses. This sets the R5 CPU limit (§6).

## 4. Lock and dependencies (`DEPENDENCY-COMPARISON.json`)

The R1 `Cargo.toml` differs from the R3-executed manifest (`660d88a5…`) only in `version` and `description`. Dependencies, dev-dependencies and features are textually identical. The Core closure (`f47318a2…`) is the same. The new modules use only crates already declared. **No new crate.** The expected lock is the R3 actual `r1cs.Cargo.lock` (`6f0e757b…`, 23,064 B) with the `mori-r1cs` version line changed, giving 23,071 B (`data/r1cs-r1.Cargo.lock`). It uses 93 of the 102 union crates. The 9 union-only crates are materialized but never compiled. The lock is pre-reviewed as exact bytes, placed read-only, and gated three ways: R0 subset against the union, R0p pair against the R3 lock, and `--locked` in every Cargo step. No lock is generated in the run.

## 5. Containment (`EXECUTION-PROPOSAL.json` `mechanism`, `view_definition`)

- **Choice: bwrap 0.11.1** (`/usr/bin/bwrap` `523da3e7…`), wrapped in the executed Aiken `run_step` convention (nice, timeout, prlimit). Every option I use appears in the captured man page. Unprivileged user and net namespaces already work on this host for `/usr/bin/unshare` (R3, Aiken). bwrap needs the same kernel feature, but that is not assumed: Q1 and Q2 observe it.
- **View.** The root is an empty tmpfs. Read-only: `/usr /bin /lib /lib64 /etc/alternatives`, the toolchain directory, and the inputs `/task/{r1cs,moriarty,gate,probe}`. Writable: `/task/{cargo-home,target,home,tmp}`. The pid namespace gets its own `/proc`, and there are a minimal `/dev` and a 256 MiB `/tmp`. The flags are `--clearenv`, `--die-with-parent` and `--new-session`, with no capabilities. Host HOME, `~/.cargo` (apart from the 194 copied files), credentials and other task folders are simply absent. Cwd `/task` and `/` have no host ancestors, which closes the ancestor `.cargo/config` problem by construction. Inputs are read-only, so a build script cannot change the bytes that the build identity hashes.
- **Rejected.** Unshare-only isolation (R3's R-05 residual, refused for R1 by the review). Docker (unobserved daemon and image, possible pull or host mutation). Any fallback.
- **Probe (Q2, `data/probe/probe.sh`).** It uses only task-owned fixtures. A canary at `<T>/canary/CANARY.txt` must exist on the host (positive control) and must be invisible inside the view; the task logs and task root must be invisible too. The probe also checks CapEff 0, loopback only, a small pid set, the exact `/task` entries, absent Cargo config, read-only inputs and writable outputs, and that rustc can compile, link and run `main.rs`. It never names or stats a private path.
- **Build scripts I inspected** (host extracted sources; the run uses the pinned archives): ahash, libc, proc-macro2, num-traits, serde, serde_core, serde_json, zerocopy, generic-array, thiserror, paste, syn 1 and zmij. Their effects are limited to reading the environment, running `$RUSTC --version` or probe compiles into `OUT_DIR`, writing `OUT_DIR`, and, for zerocopy, reading its own `Cargo.toml`. None needs HOME or the network. Not read: quote's build script and the nine proc-macros; the view contains them. The crate has no `cc` dependency, so there is no C compilation.

## 6. Resources

The R3 values are kept: 8 GiB AS per process, core 0, nice 10, 2 jobs, 2 test threads, no retry. R5 gets CPU 3600 s and wall 3600 s, because RLIMIT_CPU counts the whole `abi2_r1` process, and its roughly 24 dev-profile BLAKE2b syntheses are unmeasured. My estimate of 2–10 s each could be off. A false BLOCKED(resource) would waste the single attempt. The worst case is 12,600 s; the expected run time is minutes. There is a 6 GiB disk gate.

## 7. Expectations (`EXPECTED-TESTS.json`)

Each of the 74 tests is graded by lineage:

- **ORACLE-NONAUTHOR:** Sol `/1` vectors, with their own review still pending.
- **HELDOUT-NONAUTHOR:** H1-P1. This is the one non-author ABI2 accounting comparison.
- **HASHLIB-AUTH:** an independent BLAKE2b implementation whose vectors the author chose.
- **AUTH, HOSTILE-AUTH, STRUCTURAL and PIN** cover the rest.

The pre-frozen financial and work facts are quoted, not computed. They are oracle gross/fees/W for the six lifecycles, the prior reviewer's partial(35) numbers, and the AUTH wide-carry value 2^128. No general financial-work oracle exists for ABI2 bytes beyond these. The P01–P05 bodies, statements and public inputs remain same-author. `compiler_build_digest` is recorded in H1 only as a value derived from the two build-identity assertions and Core's canonical rule (compact JSON, sorted keys). The binary does not print it.

## 8. Known risks that block rather than falsify

1. **The Core checkout is live.** If any of the 70 closure files changed since `6c7b1fda`, P0 stops with BLOCKED(identity). The R3 scratch copy of the same 70 files would need a new explicit decision; it is not a silent alternate.
2. **The lock is hand-transcribed.** The root's one-line diff and size check before review, R0, R0p and `--locked` each catch a defect.
3. **Tool availability.** P0 checks `prlimit`, `timeout`, `/lib64`, `/etc/alternatives` and `/usr/bin/cc`. Q2 proves the linker inside the view.
4. **BUILD-MANIFEST-R2 trailing newline** (prior review). A mismatch is a FAILED prediction to report, never an edit.

## 9. Residuals accepted for review (full list in the proposal)

- Only four toolchain files are pinned.
- `/usr` is visible read-only.
- `cargo-home` is writable, so F1 detects changes only after the fact.
- Limits apply per process.
- Proc-macros were not read.
- Hashes are copied, not verified by me.

## 10. Files

`REPORT.md`, `EXECUTION-PROPOSAL.json`, `LAYOUT-MAP.json`, `DEPENDENCY-COMPARISON.json`, `EXPECTED-TESTS.json`, `VOTES.md`, `steps/{P1,P0,P2,Q1,Q2,R0,R0p,R1,R2,R3,R5,R4,F1,H1,F2}.sh`, `data/{r1cs-r1.Cargo.lock, r1cs-60.sha256, core-70.sha256, option-a-194.sha256, inputs-abs.sha256, gate-3.sha256, expected-74-names.txt, probe/main.rs, probe/probe.sh, canary/CANARY.txt}`, `READ-COVERAGE.json`, and `OWN-OUTPUT-ALLOWLIST.json` (written last). The future target `current-cardano-p3-r1-rust-local-execution-20261015` was not created. After any run, a fresh independent actual-result review is required.

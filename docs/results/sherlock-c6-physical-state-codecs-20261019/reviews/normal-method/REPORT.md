# Method/API review: full normal Core rlib with `-C lto=no -C embed-bitcode=no`

This is a data-only review. It grants no execution authority and does not select an attempt. I used no tools.

## Verdict: APPROVE (method/API scope, conditional on mandatory issues M1–M6)

- **It is substantively different.** It changes how the rlib payload is generated: the default `yes` becomes explicit `no` for embedded bitcode. It is not an unchanged retry, a cap raise or a source subset.
- **Semantics are preserved under the stated premises.** Dropping the embedded LLVM bitcode does not remove anything a non-LTO Rust consumer needs. It does not touch:
  - public API metadata
  - generic or inline MIR
  - native object code
  - assertion behavior
  - cfg selection
  - the six externs
- **The one consumer-class restriction is explicit.** Consumers must not use cross-crate LTO (see P6/M3).

## Installed flag-support assessment

| Item | Pinned evidence | Is it enough? |
|---|---|---|
| `embed-bitcode` exists, applies to rlibs, default yes | `embed_bitcode_help` exact string | Yes, for existence, scope and default |
| `embed-bitcode=no` spelling | The diagnostic string literally contains `` `-C embed-bitcode=no` `` | Yes, for syntax |
| `lto=no` | `lto_help` lists `no` among the accepted booleans | Yes |
| `embed-bitcode=no` together with `lto=no` | The diagnostic only says "`-C embed-bitcode=no` and `-C lto` are incompatible" | **No.** The pinned text does not say whether `lto=no` counts as "`-C lto`" |
| `codegen-units` default | The help and man page give `N` but no default | Insufficient, so leaving it implicit is correct |
| What is retained without bitcode (metadata, objects) | Not stated in the one-line help | **Insufficient.** This rests on the trusted-compiler premise |

**On the pair-compatibility gap.** From general rustc knowledge (outside the pinned evidence), the incompatibility check rejects only the forms that enable LTO: bare, `yes`, `fat` and `thin`. Unspecified and `no` are accepted. The gap is not a reason to abstain, because the failure mode is fail-closed. If the pair is rejected, rustc stops early with a nonzero exit and produces no artifact, so nothing is silently weakened (see M2).

**On `lto=no` itself.** It is semantically redundant here. rustc does not perform LTO when it emits an rlib; LTO happens when the final artifact is linked. Implicit thin-local LTO is also inactive at opt-level 0. Keeping it records the intent and is fine. Removing it after this review would make it a different method.

**On codegen-units.** I do not propose an explicit N. The pinned material gives no default, so any N would be a guess. Leaving it implicit means the number of object members in the rlib is unpinned (see M1).

**On help sufficiency.** The help is sufficient for syntax and default. It is not sufficient for semantic proof, and I do not claim any. Semantic preservation falls inside the already-declared assumption that the Rust compiler and the qualified toolchain can be trusted.

## Premises for whole source, public ABI and non-LTO linking

- **P1 – What an rlib contains.** An rlib holds `lib.rmeta` plus native `.rcgu.o` objects.
  - `lib.rmeta` carries the crate metadata: public items, types, trait impls, exported symbols, and generic and inline MIR.
  - `embed-bitcode=no` removes only the extra LLVM bitcode copy, which is used solely as LTO input.
  - The metadata and the native code come from the same MIR either way.
- **P2 – Generics.** Downstream crates monomorphize from the MIR in metadata, not from bitcode. At opt0, shared generics may resolve to native symbols exported by this crate; those are still in the native objects.
- **P3 – Assertions.** Two settings are fixed when this crate compiles, independent of bitcode:
  - `debug_assertions` (`cfg!`/`debug_assert!`)
  - the MIR overflow `Assert`s
  
  Owner-admission and codec logic compile identically. Consumers' own flags must remain the approved ordinary ones.
- **P4 – cfg(test) stays false.** There is no `--test` and no `--cfg test`; this is unchanged from B32.
- **P5 – Externs.** All six externs use the same paths as B32. They are consumed through their metadata and objects. This crate's bitcode flag does not alter them, and whether they contain bitcode is irrelevant to a non-LTO build.
- **P6 – Consumer link class.** Every consumer must link without `-C lto` (fat or thin, including LTO'd staticlib or cdylib outputs).
  - Violating this fails at link time with a missing-bitcode error, so it is detectable rather than silent.
  - Background knowledge, not pinned evidence: Cargo itself passes `-C embed-bitcode=no` for non-LTO profiles. This artifact class therefore matches ordinary Cargo dev rlibs.
- **P7 – Symbol identity.** The new `-C metadata` value changes the SVH and symbol hashes relative to every earlier `mori_core`. Consumers must be compiled against this exact rlib.
- **P8 – Dep-info.** These flags do not change dep-info. The default `.d` file lists source inputs, not extern rlibs, so extern identity must come from the argv pins and the existing reused-dependency qualification.

## Mandatory issues

- **M1 – Archive qualification must fit a no-bitcode payload.**
  - It must accept `lib.rmeta` plus x86-64 ELF relocatable `.rcgu.o` members.
  - It must not require `.llvmbc` or bitcode content.
  - It must not pin object member count or names from any prior or default artifact, because codegen-units is implicit.
  - If the existing qualification assumes bitcode is present, that conflict goes to Root. It must not be edited silently.
- **M2 – Fail-closed on flag rejection.** If rustc rejects the pair, record it as a method failure with the full raw results. Do not silently drop `lto=no` or fall back to the default. Any change requires a new reviewed method.
- **M3 – Consumer non-LTO premise must be visible in the consumer record.** The consumer argv must show `-C lto` absent or `no`. If any approved consumer needs LTO, this method is invalid for that consumer.
- **M4 – Exact artifact binding.** The consumer's `--extern mori_core=` must point to the exact produced path and sha256. Reusing B32, old, partial, test-ELF or cargo-target `mori_core` artifacts is not allowed. The consumer-side `-L dependency=` directories (for example the `24-compiler-qa/.../debug/deps` directory) must not make a different `mori_core` resolvable.
- **M5 – Faithful diff statement.** Relative to the B32 argv, the delta is more than the two flags. It also covers:
  - the source root (3A snapshot instead of the 18host snapshot)
  - the `-C metadata` string
  - the output and dep-info paths
  - the cwd and TMPDIR
  
  "Add ONLY compiler choices" must be recorded as the compiler-option delta only. The source root delta is the separately approved 3A cut, not part of this method.
- **M6 – Interpreting a deadline.** A repeat 30s deadline says nothing about bitcode cost or root cause in either direction. It does not authorize an unchanged retry under this method.

## Limitations (all unobserved)

- **Fit is uncertain.** My judgment, not evidence: at opt0, embedding bitcode mainly means serializing modules LLVM has already built for each codegen unit. That is likely a small share of the cost and unlikely on its own to explain the gap between 7.1s and 30s or more. The method makes no performance claim, so this does not block approval, but Root should expect a low yield as a fit fix.
- **Cause is unknown.** Host pressure remains unproven, and child CPU and rusage counters are absent.
- **No artifact exists yet.** There is no actual rlib, dep-info, archive identity or member listing.
- **No tests have run.** All original and external 3A tests are unrun. A metadata-only check cannot substitute for them.
- **Out of scope:**
  - LTO, bitcode, SDK or native Phase F qualification.
  - 3B/3C sources and their union with this cut.
  - Any claim that the 179-member cut compiles unregistered modules.
  
  The method can be reused for those only after their own source/API approvals and an exact final union map.
- **Trusted assumptions remain.** The rustc 1.99.0 (b940084d7) semantics and the qualified toolchain are assumed, not proven.
# Prover Key Domain Implementation Plan

> Execute inline using superpowers:executing-plans. Root owns fresh audits,
> publication and every later native resource decision.

**Goal:** Transport a bounded selected PK independently from public bytes under
explicit work-limits v2, preserving exact v1 semantics.

**Architecture:** Internal typed limits version and shared pure keyset counts;
validated keyset-only check; existing generic/public checks remain unchanged.
Consumers forward the same counters and effective PK cap without new native or
serialization passes. Readers preserve immutable bounded acquisition.

**Tech Stack:** Rust 1.99.0, current serde/SDK dependencies, no new framework.

## Constraints

- Same DTO; v1 unchanged; v2 64 MiB PK /16 MiB public /1 MiB input /64 KiB VK.
- SRS lifecycle, default refusal deadlines, all-action program and phases unchanged.
- Tiny fixtures/pure counts only; no native keygen/prove/Mock/SRS access.

- [x] Add a callable tiny v2 proving-reader fixture exceeding public leaf/flat
  aggregate while fitting PK cap; capture Red against current source.
- [x] Add internal version/hard limits and effective PK cap; preserve all v1
  positive/duplicate/unknown/missing field oracles.
- [x] Add pure separate public/prover/physical accounting with checked arithmetic;
  apply it to selected generation, import and both reopen paths.
- [x] Add a private singleton keyset-export context/factory and check; recheck
  Signed descriptor replacement. Generic check never grants a PK exception.
- [x] Add meaningful profile/count/reader/export-context tests covering exact
  and exceeded domains, marker-first priorities, regular file/hash/descriptor
  failures, cold optional FIFO/symlink ignoring and legacy semantics.
- [x] Run focused ordinary suites, owner compile-fail and diagnostic controls,
  CLI early-refusal checks, fmt/build; retain actual terminal captures.
- [x] Supply exact Rust source functions to independent Quint QA for final pins.
- [x] Document source-only result and unchanged native obligations; commit a
  clean candidate and immutable producer/capture/closure digest record.

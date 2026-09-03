---
id: moriarty.research.journal
type: decision
title: Moriarty research journal
status: active
updated_at: 2026-09-03T09:07:57Z
sources:
  - SRC-0016
  - SRC-0017
  - SRC-0018
---

# Moriarty research journal

This journal records decision rationale, evidence changes, semantic scope, and
the next falsification test. It does not record private chain-of-thought. An
entry is complete only when its referenced evidence is preserved.

## Iteration S00.2: replace calendar progress with evidence gates

- Timestamp: 2026-09-03T07:23:21Z
- Repository base: `006c4d91ed09c0a89261861b6e7203b3efa3e2df`
- State: active

### Trigger

The user rejected calendar-based planning. The user required sprint-only
progress, an iteration-by-iteration semantic scope record, evidence that a
Compact DSL is possible, and a complete SDK specification.

### Decision rationale

Elapsed time is not evidence. Replace day windows with sprint gates. Retain the
same dependency order because proof, backend, and coverage packages still have
real input dependencies.

Treat E00 as a narrow feasibility witness. It proves that one finite
Marlowe-shaped Core slice can generate reviewable Compact and ZKIR artifacts.
It does not prove general compilation or production deployment.

Expand the SDK package from transaction verification to the complete authoring,
compilation, analysis, packaging, deployment, client-verification, and
operations toolchain.

### Active semantic scope

E00 contains `Close`, `Pay`, `If`, `When`, `Deposit`, bounded `Choice`, parties,
tokens, accounts, explicit timeout, payments, warnings, errors, and the atomic
swap specialization. All collections and contract paths are finite.

No later candidate construct is frozen. Attest, action sets, mandate,
conditional-token split or merge, external calls, minting, Merkleized
continuations, modules, and packages remain proposed or outside the E00 Core.

### Evidence state

- The positive Compact program compiled with the pinned toolchain.
- Four ZKIR 3.0 circuits passed the mock compiler.
- One thousand unique differential traces produced zero divergence.
- The negative disclosure fixture failed compilation as required.
- Real proving parameters, keys, proofs, ledger execution, and cost remain open.

### Next falsification test

Reproduce E00 from a fresh pinned environment. Then attempt a second canonical
application through the same generic surface-to-Core-to-Compact contracts. A
special-case generator that cannot support another shape is insufficient
evidence for a language.

## Iteration S00.3: reproduce E00 and close the SDK specification boundary

- Timestamp: 2026-09-03T08:21:33Z
- Reviewed instruction base: `f702692895e8be811fb9eac2a1c0bfafca5dea70`
- State: instruction revision verified; WP01 passed at S3

### Trigger

The user required evidence that a Compact DSL is possible and required the
entire development SDK to be specified. The three-seat Council requested changes
to all twelve initial work-package instruction sets.

### Council evidence

Grok, Sol, and exact Fable 5.1 each reviewed WP01 through WP12 in one frozen
bundle. All three returned `changes_requested`. The review was advisory, not a
security audit. The raw result digests and model terminal evidence are recorded
in `deliverables/moriarty-work-package-council-advisory-2026-09-03.md`.

The Sol result self-labels as `GPT-5`, but the authoritative Codex terminal
banner records `model: gpt-5.6-sol`. The Fable result uses exact
`claude-fable-5-1` and also discloses one 19-output-token Haiku helper call. The
project preserves both details. It does not silently upgrade model evidence.

The assurance scorecard was frozen at SHA-256
`a4557e72ebf50645d4b02c9d2ee3eb778fc9292bd754f71530927351ff6d6be2`.
The terminal decision scorecard was frozen at SHA-256
`42de140b02a7ed1e3982ec90ce27b2581486b938ccfe49b4b6e6dbb0429c279e`.

### Compact DSL evidence change

A fresh `git archive` of Moriarty commit
`006c4d91ed09c0a89261861b6e7203b3efa3e2df` created a new Python environment and
reproduced 43 focused tests. The run reproduced the 1,000-trace certificate,
Compact source, compiler manifest, four ZKIR files, and four binary ZKIR files.
All recorded digests matched. The undeclared-disclosure negative control also
reproduced with exit code 255 and the same full-diagnostic digest.

The run found a new reproducibility footgun. An absolute Compact input path
changed only the source-map and compiler-manifest digest. The recorded relative
command reproduced the expected digest. The SDK must build under a canonical
sandbox path or canonicalize source maps.

Moriarty now performs its own visibility-manifest check. A generated source that
omits `disclose(decision)` fails. A generated source that adds
`disclose(phase)` also fails. This check is independent of the third-party
compiler negative control.

### SDK scope change

WP09 now specifies 65 components. Seventeen form the minimum safety spine and 48
remain specified-only. The component inventory SHA-256 is
`b956125d8ac5e6f8de2cc1f22ca383152a0ac2a3c0969b0e2b2c1e0ca175458c`.

WP09 also specifies 28 canonical data and wire contracts. Their inventory
SHA-256 is
`180baf9647a73cd38ca680205ff9f12ce1b71d560aec9a1b636431446bac3195`.
Machine-readable component and data-contract schemas now bind both inventories.

The prover and proof-parameter provider are explicitly untrusted. Private
witnesses cannot enter an unapproved prover request. Proof parameters bind to
proof system, circuit, network, version, source, digest, and revocation state.
The client verifies proofs and transaction effects before it creates an
immutable signing request.

### Semantic scope transition

No semantic motion was accepted in this iteration.

- Previous version: `0.0.0-e00.1`
- Previous snapshot SHA-256:
  `edc69c1f857ab0465f867c03fe459464a3707529fa20a4fa7a6030caacf30f9a`
- New version: `0.0.0-e00.2`
- New snapshot SHA-256:
  `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
- Change kind: evidence-only.
- Reason: clean reproduction evidence changed. Core constructors and semantics did not.

### Decision rationale

E00 now gives reproducible evidence that one finite Moriarty fragment can target
Compact and ZKIR. The result remains S3 because it is one specialized prototype
and uses mock proof compilation. The clean run does not justify a general DSL
claim.

The SDK is complete as a specification boundary, not as an implementation. WP10
and WP11 cannot treat its 48 specified-only components as executable or audited.
They require the 17-component safety spine first.

### Next falsification test

Freeze WP04 semantic motions. Then rerun WP01 against the new scope digest and
compile a non-swap canonical application through shared compiler interfaces. If
the implementation needs another application-specific generator, the evidence
supports Compact libraries or templates, not a Moriarty language.

## Iteration S00.4: close evidence-integrity review findings

- Timestamp: 2026-09-03T08:48:41Z
- State: validator remediation verified; no semantic change

### Trigger

An independent review found four evidence-integrity defects. Compact disclosure
syntax with whitespace could bypass the source check. Compiler metadata was not
compared with the Moriarty manifest. The evidence manifest trusted package,
scope, self-hash, and gate fields too readily. The SDK inventory schemas and
component references were not enforced by the sprint validator.

### Corrections

- The Compact scanner now recognizes `disclose ( value )`. It excludes line
  comments, block comments, and string literals from executable disclosures.
  It rejects nested or compound disclosure expressions that the manifest does
  not name.
- Compiler metadata must match the manifest toolchain versions, exported
  circuits, circuit arguments and results, witnesses, and complete ledger
  schemas.
- The clean pinned-commit receipt and current-checkout validation now use two
  separate evidence streams. The current stream has no commit identity and no
  fresh-archive claim. It binds the remediated source files by SHA-256.
- The sprint validator now checks package identity, sprint identity, semantic
  scope, scope-index digest, output digests, manifest self-hash, and the WP01
  gate predicate. WP02 through WP12 fail closed until their package-specific
  gate validators exist.
- The SDK validator applies both Draft 2020-12 schemas. It requires exactly 65
  components and 28 data contracts, unique identifiers, valid specification
  paths, closed component references, and a fully recomputed SDK index.

The corrected data-contract inventory SHA-256 is
`180baf9647a73cd38ca680205ff9f12ce1b71d560aec9a1b636431446bac3195`.
The component inventory is unchanged at
`b956125d8ac5e6f8de2cc1f22ca383152a0ac2a3c0969b0e2b2c1e0ca175458c`.

### Semantic scope transition

No semantic motion was accepted. Version `0.0.0-e00.2` and snapshot SHA-256
`9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`
remain active. This is an evidence-integrity change only.

### Verification

- The 17 focused evidence and OpenSpec tests passed.
- All 12 OpenSpec changes passed strict validation.
- The instruction validator confirmed 12 packages, 65 components, and 28 data
  contracts.
- The WP01 validator recomputed the package gate and passed.
- The repository suite first passed 72 tests. Two graph tests could not find the
  ignored Marlowe Graphify analysis fixture in the isolated worktree. A
  read-only link to the preserved main-worktree fixture let the complete suite
  pass all 74 tests. The temporary link was then removed.
- The final independent re-audit returned `READY`. It reproduced seven focused
  mutation checks, both validators, and the fail-closed WP02 result.

### Decision rationale

A toolchain exit code is not sufficient evidence for a compiler boundary. The
Moriarty manifest must also agree with the compiler-observed interface. A JSON
`gate_passed` value is not sufficient evidence for a sprint boundary. The gate
must bind the package, semantic scope, source or commit identity, outputs, and
package-specific predicate.

### Next falsification test

Compile a second finite financial application through shared compiler
interfaces. Reject the general-DSL claim if it needs a new application-specific
lowerer. Do not expand the Core until an S03 semantic motion passes.

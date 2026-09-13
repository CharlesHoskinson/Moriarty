---
id: language.zk-survey
type: comparison
title: ZK surface language survey
status: active
updated_at: 2026-09-13T06:00:00Z
sources:
  - SRC-0114
  - SRC-0115
  - SRC-0116
  - SRC-0117
  - SRC-0118
  - SRC-0119
created: 2026-09-13
updated: 2026-09-13
tags:
  - moriarty
  - research
  - language
---

# ZK surface language survey

Six language designs read at source level to inform
[[wiki/language/surface-language-design|the Moriarty surface language]]. Corpus
registered in [[wiki/sources/zk-language-corpus|the ZK language corpus]].

## What to take

**CLM-0962 — Mina harvests annotations that already exist.** The `@method`
decorator reads TypeScript's `design:paramtypes` reflection metadata, so the
parameter types the author already wrote become the circuit's public input
signature, with no schema file and no second declaration. It then silently
injects the contract address and token id as witness arguments
(CLM-0962; SRC-0116 `src/lib/mina/v1/zkapp.ts:98-146`; source fact; reproduced; high; S4).

**CLM-0963 — Cairo makes mutability a type, not an annotation.** There is no
`#[view]` attribute anywhere in the compiler. Read-only is `self: @ContractState`
and mutable is `ref self: ContractState`; the entry-point wrapper generator reads
the signature. The annotation cannot drift from the body because there is no
annotation (CLM-0963; SRC-0117 `crates/cairo-lang-starknet/src/plugin/`; source fact;
reproduced; high; S4).

**CLM-0964 — Cairo pins generated source in goldens.** Attribute expansions are
fixed in `plugin_test_data/`, so the effect of every macro is reviewable as a
diff. For a project whose artifacts are hash-bound, sugar that cannot be diffed
is sugar that cannot be admitted (CLM-0964; SRC-0117 `crates/cairo-lang-plugins/src/test_data/`;
source fact; reproduced; high; S4).

**CLM-0965 — Noir guards off-circuit computation three ways.** Calling an
`unconstrained fn` needs an `unsafe { }` block; the parser emits
`MissingSafetyComment` if no `// Safety:` comment precedes it; and a dataflow
pass requires every returned value to participate in a later constraint,
reporting `Brillig function call isn't properly covered by a manual constraint`
otherwise. Three independent mechanisms for one hazard
(CLM-0965; SRC-0118 `compiler/noirc_frontend/src/parser/parser/expression.rs:524`;
source fact; reproduced; high; S4).

**CLM-0966 — SimplicityHL marks hazardous values in the token, not the context.**
Private inputs are a namespace, `witness::ORACLE_SIG`, and fast native paths are
a namespace, `jet::bip_0340_verify`. Neither can be confused with an ordinary
value because neither is spelled like one
(CLM-0966; SRC-0115 `examples/hodl_vault.simf`; source fact; reproduced; high; S4).

## What to refuse

**CLM-0967 — Mina makes the unsound read shorter than the sound one.** `get()`
does not prove the value matches on-chain state; `getAndRequireEquals()` does.
The API documents the hazard on `get()` itself and ships a `requireNothing()`
escape hatch labelled "DANGER ZONE". The v2 state module contains no
`requireEquals` at all, which indicates the pattern is being abandoned
(CLM-0967; SRC-0116 `src/lib/mina/v1/state.ts:30-86`; source fact; reproduced; high; S4).

Moriarty's `pre`/`next` split already makes the precondition-bound read the only
read. **This is now a design rule: no sugar may introduce a spelling that is both
shorter and less sound than what it replaces.**

**CLM-0968 — Noir's visibility does not propagate.** `Visibility` is attached to
parameter and return positions, never enters `Type`, never unifies, and never
taints downstream values; the compiler rejects it anywhere but an entry point.
It is an ABI annotation, not an information-flow type system. A financial
language wants propagation through joins with an explicit declassification
operator as the only escape
(CLM-0968; SRC-0118 `compiler/noirc_frontend/src/shared/visibility.rs`; source fact;
reproduced; high; S4).

**CLM-0969 — ZKsync kept the syntax and changed the semantics.** Solidity source
is byte-identical to Ethereum's while deployment, addressing, nonces, value
transfer and `tx.origin` all differ; EraVM-only operations are smuggled through
`call` with sentinel addresses that the compiler's own documentation calls
"temporary hacks". Developers get the illusion of portability: the code compiles
and the address is wrong
(CLM-0969; SRC-0119 `docs/src/specs/era_vm_specification/compiler/instructions/extensions/`;
source fact; reproduced; high; S4).

The inverse is the rule for Moriarty: a semantic difference belongs in the type
system where the compiler enforces it, never in build flags or conventions.

**CLM-0970 — Unverified purity attributes are a footgun.** Noir's
`#[no_predicates]` is a purity assertion the compiler does not check, where
violation "severely breaks the intended representation, proof generation, and
verification of the program". Contrast [[wiki/language/jet-discipline|the jet]],
whose semantics is the identity by construction
(CLM-0970; SRC-0118 `docs/`; source fact; not reproduced; medium; S3).

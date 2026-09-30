---
title: Modules, packages, testing and developer toolchain
status: research-complete-recommendation-unreviewed
created: 2026-09-30
updated: 2026-09-30
scope: authoring-beta-design
---

# Modules, packages, testing and developer toolchain

**Recommendation:** deliver one compiler driver and one reusable library from the existing TypeScript frontend. Use inert `.mori` source, explicit project metadata, deterministic local resolution and common diagnostics. The initial execution slice remains transfer and funded AccrualFirst repayment returning `PreparedUnqualified`. Broader examples can demonstrate authoring syntax while their semantic and execution status remains visible. A package release, test pass or content digest supplies no native proof or Midnight ledger acceptance.

## Evidence and comparisons

**Repository observations:** startup loaded `moriarty-dev:develop` and ran guarded `status --json` in the beta worktree. It reported no pending transactions and blocked the historical loan/swap campaign on stale bindings and missing accounting/live-resource evidence. This research did not dispatch it. Root AGENTS references two routing files absent from this checkout; the current beta user request controls this research. `wiki/index.md` was searched before acquisition. There is no root Graphify graph; no new graph was created. Product constraints come from the [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md), [mockup requirements](../../../docs/language/PROGRAMMER-FACING-MOCKUP-REQUIREMENTS-2026-09-30.md) and [meta-tool memo](08-meta-tools.md).

**Acquisition observations:** two fresh official captures through Scrapling 0.4.15, both HTTP 200, with `main` extraction and immutable response bytes. Both robots checks allowed the target paths. This was public reference acquisition without login, bypass, credentials or cookie storage. Existing static documentation patterns sufficed; no new site pattern or cookie record was needed. Page availability and robots permission do not settle licensing. [toolchain-sources.json](../toolchain-sources.json) preserves hashes, dates, scopes and limitations. Current package/security versions beyond these documents were not researched.

**Source facts — GO-01:** [Effective Go](https://go.dev/doc/effective_go) documents machine formatting through `gofmt`/`go fmt`, declaration comments as package/command documentation, concise package names used as accessors, and contextual error values. Its library examples illustrate idiomatic use. The page warns that it dates from 2009, is not actively updated, and omits ecosystem modules and generics. These sections support a convention comparison; this capture does not establish modern Go module resolution, testing APIs or adoption metrics.

**Source facts — CARGO-01:** the [Cargo package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) places `Cargo.toml` and `Cargo.lock` at the root, source in `src`, default library and executable entry points in separate files, and examples, integration tests and benchmarks in named directories. Its conventions aid navigation. Manifest semantics, lockfile guarantees and registry trust were not inspected in this page.

**Reused source facts:** Unison [hashes](https://www.unison-lang.org/docs/language-reference/hashes/) identify term/type structure independently of names. Its [big idea](https://www.unison-lang.org/docs/the-big-idea/) describes dependency hashes and reuse of compiled definitions; those benefits are the project's account, not a Moriarty replication. [Build Systems a la Carte](https://www.microsoft.com/en-us/research/wp-content/uploads/2018/03/build-systems.pdf) distinguishes scheduling and rebuilding in a common model. Elm [ports](https://guide.elm-lang.org/interop/ports.html) separate JavaScript interaction at application boundaries and exclude port modules from packages. Captures date to September 9, not this research. The [September 19 Simplicity study](../../../deliverables/simplicity-study-2026-09-19/note-explanation.md) supplies prior recommendations on bounds, commitments and certified primitives; its provenance and proof limits remain intact.

**Inference:** borrow predictable formatting, qualified names, contextual errors and navigable project conventions. Keep Moriarty's explicit semantic profiles and financial duties. Go is a useful general-purpose comparison, but no popularity ranking or causal usability advantage is established here. Elm's tool ergonomics were discussed in prior research; current `elm-format`, `elm-test` releases and installation behavior remain uninspected. No Go, Rust, Elm or Unison project was installed or benchmarked for this memo.

## Roles, namespaces and reusable operations

The following are **Moriarty recommendations**, not imported language facts or implemented grammar.

| Term | Role | Explicit boundary |
| --- | --- | --- |
| Module | Source namespace containing declarations and explicit exports | Resolves names; creates no authority or authenticated observation |
| Package | Versioned distribution of modules, manifest and documentation | Pins dependency content; does not grant deployment permission |
| Library | Reusable typed helpers or financial constructions | Declares effects, preconditions, resource requirements and supported profiles |
| Financial profile | Versioned semantic, arithmetic and work capability set | Determines supported operations and rejection rules; distinct from package version |
| Kernel interface | Closed typed request/evidence boundary for external execution | Names authority, evidence and complete effects; has no generic host-call escape |

Use descriptive qualified names such as `finance.transfer`, `loan.repayAccrualFirst` and `amm.swapExactIn` as proposed standard-library operations. A name is a human accessor; its resolved package/export identity and semantic profile are separately bound. Avoid wildcard imports in the beta; reject ambiguous exports and aliases shadowing reserved namespaces. Standard-library namespaces are versioned distribution conventions, not a list of individually approved developer programs. Anyone can compose supported definitions without maintainer registry admission, under the product's objective validity and resource rules.

Each financial operation needs a closed interface: nominal asset/domain/program identities, typed state reads, signed limits, required observations, ordered economic effects and residual duties. There is no `call("anything", arbitraryJson)` escape into a chain, JS runtime or verifier. An `amm` operation can be parseable and documented while lowering/simulation remains unavailable; recognition of its name cannot imply invariant checking or settlement.

Reusable pure helpers should accept explicit values, use checked arithmetic and declare applicable range/rounding contracts. Start with acyclic calls and statically bounded data; reject recursion or unbounded iteration until a defined admission rule exists. Account for helper expansion and transitive work, not only source length. Effectful operations declare complete footprints and duties; dead-code removal, unused bindings or failed continuations cannot discard a liability or pending duty. Jets require equivalence and target correspondence evidence as specified by the product contract. A helper hash cannot discharge those obligations.

**First-beta choice:** parse module/import declarations only if the grammar decision includes them; initially either resolve bounded project-local `.mori` modules or reject imports with `UNSUPPORTED_IMPORT`. Never silently ignore them. Defer remote dependency fetching and general multi-package linking. Resolution must reject cycles, root escape, unsupported extensions, duplicate identities and unknown profile versions. Bound total modules, bytes, edges, AST size, call depth and expansion size. Symlink policy and case-sensitive naming need explicit cross-platform tests before broader support.

## Project and manifest proposal

Use a small conventional tree. These names are proposed; the manifest is configuration, not a signed intent or executable hook.

```text
my-agreement/
  moriarty.json
  moriarty.lock
  src/main.mori
  src/math.mori
  examples/transfer.mori
  examples/repayment.mori
  tests/transfer.case.json
  fixtures/local-transfer.json
  docs/README.md
```

```json
{
  "schema": "moriarty-project/1",
  "name": "my-agreement",
  "version": "0.1.0",
  "sourceProfile": "moriarty-beta/1",
  "financialProfile": "mil4-s0/1",
  "entry": "src/main.mori",
  "modules": ["src/main.mori", "src/math.mori"],
  "dependencies": {},
  "limits": { "sourceBytes": 65536, "modules": 16 }
}
```

**Recommendation:** the profile spelling and numerical limits above are illustrative pending the shared grammar/bounds decision. Project limits may narrow compiler caps; they cannot raise protocol/profile limits. Unknown schema keys should fail clearly to catch misspellings. Require project-root relative paths. Prefer explicit module lists initially to uncontrolled recursive filesystem scanning. `init` writes a minimal supported example, manifest and test case, and refuses to overwrite existing work.

Package version communicates distribution compatibility; source profile selects grammar; semantic/arithmetic profile selects meaning; canonical image version selects encoding; target/compiler/verifier versions bind deployed artifacts. Lock records resolve dependency versions to exact content, including transitive resolution when introduced. Do not equate a semver range with frozen meaning. Record original-source digest separately from canonical elaborated-artifact digest and source map. Formatting changes original bytes; whether a canonical semantic image remains unchanged requires a defined encoding and invariance check. The lockfile and source/artifact manifest improve reproducibility, without proving economics, authorship, witness correctness or native acceptance.

## One CLI with honest result states

**Repository observations:** [package.json](../../../experiments/moriarty-language/package.json) is private ESM, exposes repository scripts and has no `bin` or public export map. `build` runs TypeScript with `noEmit`; the [lockfile](../../../experiments/moriarty-language/package-lock.json) has no pinned compiler dependency. Existing syntax and simulation CLIs already use bounded regular-file reads and explicit source-profile paths. They provide useful components but are not a demonstrated independent distribution.

| Proposed command | Developer result | Scope label |
| --- | --- | --- |
| `moriarty init DIRECTORY` | Minimal supported project and executable local example | Project creation |
| `moriarty check FILE --json` | Parse, implemented name/type/profile checks, diagnostics and coverage | Authoring checks; unsupported checks explicit |
| `moriarty fmt FILE --check` | Deterministic formatting or clear drift/error | Syntax tooling |
| `moriarty expand FILE --json` | Elaboration/Core request with origin map | Supported lowering only |
| `moriarty simulate FILE --fixture FILE --json` | Complete local result, ordered effects, duties, bindings | `PreparedUnqualified` or rejection |
| `moriarty test [CASE] --json` | Bounded declarative cases through actual command/library behavior | Local testing |
| `moriarty inspect FILE --json` | Profiles, bounds, imports, operation support and remaining obligations | Inspection |

Commands share a result envelope: schema version, command, source/semantic profile, capability status, diagnostics and result. A diagnostic carries stable code, phase, message, UTF-8 byte span, related declaration spans and an actionable explanation. Test spans on Unicode and invalid UTF-8; an editor adapter converts coordinates explicitly. Human output should explain unsupported execution in the same terms as JSON. Keep stdout as the requested artifact/JSON and stderr for human diagnostics when appropriate. Choose documented exits for success, rejected input, usage/IO and internal failure; prepared local success returns zero while reporting unqualified status. Do not equate a parse success with full type validation or a submitted transaction with confirmed finality.

`check` should report separate syntax/type/lowering/execution capabilities. If only structural checks exist, say so. For unsupported operations `expand` and `simulate` reject with a stable code and source span, without partial financial results. An operation-specific profile gate is an objective implementation boundary, not permission to author a new supported composition.

**Intent and fixtures:** source owns the declared promise, bounds, authority and holes. A local fixture supplies stipulated state, observations and invocation/completion values with explicit labels. Fixture defaults must never invent an omitted source constraint, widen authority, fill undeclared holes or erase signed limits. Completion values refine authored holes; they do not replace signed terms. Inspect and simulation output identify authored fields, solver choices, locally stipulated premises and missing authentication. Every success-shaped local result preserves complete effects and duties and carries `PreparedUnqualified`; no absent binding defaults to authenticated. No wallet/network access belongs in these authoring commands.

## Distribution choice and reproducibility

| Option | Fit | Decision |
| --- | --- | --- |
| Repository `.ts` scripts | Fast internal development; runtime/global-tool assumptions and checkout paths remain | Retain for development and regressions |
| Packaged JavaScript driver and library | Ordinary Node installation; same checker shared by CLI, editor and AI clients | First beta delivery target |
| Langium/Xtext workbench | More language services, generated artifacts and migration work | Conditional later trial under domain 08 |

**Recommendation:** emit ready-to-run ESM JavaScript and declarations, with a small `bin/moriarty.js` entry containing `#!/usr/bin/env node`, explicit `bin`, `files` and public `exports`. Publish only supported API surfaces; importing the library must not execute the CLI. Whether compilation uses emitted modules or a bundle is an implementation choice; require deterministic output and no missing external runtime modules. A single bundle reduces exposed file layout, while emitted modules simplify provenance and debugging. Neither changes financial semantics.

Pin TypeScript/build dependencies in `devDependencies` and the lockfile; `npm ci` must suffice to build from a clean checkout. Retain a tested Node minimum and engine declaration. Node 24.21.0 is the currently recorded baseline runtime; select that as the conservative initial tested minimum unless an actual older-version smoke justifies lowering it. This is a distribution recommendation, not a claim about general Node compatibility. End users install compiled JS without global `tsc`, TS source execution, repository credentials or campaign metadata. Runtime dependencies, if added, need explicit pinning and packed-install checks. No package install hooks, shell plugins or remote imports run during source parsing.

The release test packs the artifact, installs it into a fresh temporary project outside the checkout, invokes `moriarty --help`, imports the public API, and runs supported check/expand/simulate commands. Test with dependencies installed, then execute with network disabled and without the repository path available. Capture Node/package versions and artifact digest. A passing repo script cannot substitute for this cold-install observation. This experiment is specified-only here; no distributable was built by the researcher.

## Testing and executable documentation

**Recommendation:** define expected outcomes independently before changing the driver. Launch the public command as a subprocess and assert exit code, diagnostic schema/code/span, stdout/stderr discipline and complete result. Golden files should encode reviewed financial expectations, not snapshots automatically blessed from the implementation. Compare supported source lowering against separately constructed Core vectors with all signed bounds, identity fields, rounding, effect order and residual duties checked. Calling the same elaborator twice is not an independent oracle.

Include positive transfer/repayment cases and rejected underfunding, wrong asset/domain/program, changed gross debit/fees, stale evidence, missing intent, fixture-authority widening, dropped effects/duties, unknown profiles and unsupported families. Exercise invalid UTF-8, huge literals, nested syntax, excessive modules/expansion, path escape, cyclic imports, devices/pipes and truncated fixtures. Mutation cases delete each critical signed constraint and economic-effect field and must change the expected acceptance result. Seeded bounded fuzzing asserts termination, memory/work caps, valid spans, stable diagnostics and absence of crash/partial effects; record seed and bounds. Differential formatter checks preserve parse meaning within the implemented grammar and idempotence, with comments/source provenance accounted for. These are proposed gates, not tests executed in this memo.

Documentation needs a short start guide, syntax tour, operation/profile reference and explanatory source → intent → Core → local-result walkthrough. All eight DeFi families plus composition use one reviewed notation. Run every promised parseable example through the real CLI; separately declare whether type checking, lowering and simulation are supported, specified-only or open. A parseable bridge example cannot be reported as a bridge implementation. Transfer and repayment examples need field-by-field comparison to Source/6. Keep unsupported execution as explicit expected cases. Extract fenced examples or maintain linked source files so documentation drift is detected; do not hand-maintain competing syntax grammars.

## Exact next scope and open obligations

**Proposed next implementation:** in the existing TS package, add the agreed beta AST/checker, shared diagnostics and one driver for all seven commands; implement real S0 lowering/simulation, deterministic formatting, inert project initialization, explicit operation support inspection and a bounded declarative case runner. Produce compiled JS/public exports, pin the developer toolchain and demonstrate packed installation. Parse the agreed eight-family examples while labeling every unimplemented semantic capability. Local imports are optional in this first delivery; an explicit rejection is preferable to claiming unfinished linking. No new backend or remote registry is required.

Module elaboration must preserve nominal identities, complete footprints, ordered effects, signed intent and residual duties, with resource bounds across helper calls and composition. K/Quint obligations grow with actual semantics; syntax recognition does not discharge them. Target statement binding, authenticated evidence, native proofs, compiler correspondence and atomic Midnight ledger consumption retain separate open requirements. Current-version package licensing/security, cross-platform resolution, older Node compatibility and measured developer usability remain open evidence. The proposed toolchain is ready for the requested PL review, not an empirical usability verdict or release acceptance claim.

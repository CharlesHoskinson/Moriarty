# UMP01 public Core regression candidate

This is a complete **unapplied source candidate**, prepared against commit `e4d7a2b5709caa18a94f5256d03a0302364b8235` (tree `b44043fba64fa2bd396d18412adb3472434c70ac`). Root is the sole integrator. No product, Rust toolchain, provider or native execution occurred.

The falsifiable predicate is: the actual public `mori_core::parse`, `validate` and `simulate` path accepts the supplied UMP01 initial-exchange program/scenario, and its complete semantic pre/post, all twelve typed steps, ordered local/prefix effects and seven adapted ACTUS fields match independently adopted expectations. **That predicate is not established here.** The implemented test currently consumes explicitly author-derived provisional literals; independent oracle adoption and execution remain separate work.

## Concrete change

`candidate.patch` adds one test and its inline typed adapter to the existing `crates/mori-core/tests/financial_pair.rs`, plus nine files under `crates/fixtures/financial-pair/actus-ump01/`. `before/` contains exact-base source; `after/` contains every complete proposed changed/new file. The previous three financial test bodies remain byte-for-byte unchanged. No Cargo, dependency, feature, production parser/runtime or public interface changes are proposed.

The test calls the real public Core consumer. Its adapter observes actual Transfer, Liability, State and Emit results; it checks full local asset/party/duty identities, reads outstanding principal from the actual closing liability, and reads rate/accrued interest from actual typed state. It emits seven typed fields: eventDate, eventType, payoff, currency, notionalPrincipal, nominalInterestRate and accruedInterest. It does not return fixture results or a host success flag. The complete trace is compared separately so the seven-field projection cannot hide omitted fees, duties, state or effects.

The fixture is financially distinct from the equity case: an initial loan cashflow creates a nonzero persistent borrower-to-lender principal duty. The supplied balance provisioning, duty name, one-shot guards and timestamp encoding are explicit source/scenario stipulations. General ACTUS language support does not follow from them.

## Expectations and independence

`after/crates/fixtures/financial-pair/actus-ump01/author-provisional.json` supplies concrete full semantic cells so this is a runnable regression proposal rather than a stub. `AUTHOR-PROVISIONAL-FREEZE.json` pins the source, complete test and provisional cells before any independent result was read. Its origin remains **author-derived-provisional** forever. The author saw upstream reference results; no blindness or oracle independence is claimed. Source hashes qualify exact finite input bytes only, not ABI, deployment or build identity.

`two_program_financial_oracles` received only `INPUT-ONLY-MANIFEST.json` and its whitelisted paths. That input custody manifest was frozen before authored expectation/test/adapter files were added. Raw normative family JSON includes upstream `results`, prominently identified; the separate operands-only JSON excludes exactly that member. The independent lane must not read this README, the candidate test, adapters or author cells during derivation. Root must inspect its independently frozen result and any disagreements before adopting a NEW independently attributed fixture/test revision. Never relabel the author freeze or overwrite it as an independent golden.

Numeric ExprIds, complete Program/Execution wire equality, raw stdout, actual logical-meter report/cap, circuit work, native proof/ledger behavior and general scheduling remain unasserted. Every selected step's remaining statement members and its ExprId field presence/type are checked. The chosen test invokes ordinary `simulate` only, not `simulate_metered`.

## Normative basis and retained scope

The original nine-case UMP family is retained byte-for-byte both physically and in the proposed fixtures, with license and provenance. `UPSTREAM-BYTE-CHECKS.json` confirms exact bytes against pinned existing local upstream Git objects; no fetch occurred. The complete source specification, licenses and valid event/state dictionary are physically available in `normative/`. `retained-source-plan/PROVENANCE.json` is historical provenance and is not a claim that its older implementation base remains current; this packet's `PROVENANCE.json` controls the new candidate.

Exact supplied clauses: UMP schedule/state/functions lines1845–1923 delegate the IED schedule and state transition to PAM and payoff to CLM; PAM IED date lines777–782 and transition884–894; CLM IED payoff1814–1825; role-sign553–570 and settlement-FX621–630. This selected absence-of-interest branch uses explicit zero clauses; no day-count or interest formula is invented. The positive RPA principal is represented as an actual borrower liability, a finite typed correspondence to review independently.

`FIELD-MAP.json` covers every original selected input member and all seven present result fields. UInt/u64 and i64 conversions admit this exact integral selected case, not the general ACTUS signed/decimal Real domains. The raw notional string retains its two leading spaces; source literal1000 is declared manual reification, not a general importer. General 30E360 computation, P1M notice, redemption/call/later-interest behavior, external observations, schedule completeness, arbitrary roles and horizon completion remain unsupported. The horizon never discharges principal.

The full277 filename-qualified membership and ordered field census,18 executable types,32 taxonomy rows, DS01–07 and all U/M/MC/SP/G exits remain retained. No fixture is skipped, quarantined, downgraded or scored by this candidate. **Full ACTUS fixture passes claimed:0; UMP family passes:0; new seven-field observed passes:0.** Other UMP cases remain obligations, including nonzero and negative rates. No security assessment is supplied.

## Root integration recipe

1. Inspect `candidate.patch`, all complete `after/` files, exact `before/` consumer files, obligations, licenses and freeze. Check the root integration base against the exact commit/tree; a later base requires byte-current integration assessment.
2. Obtain and inspect the separate independently frozen financial/work oracle. Compare source/input hashes, profile, mapping premises, complete state/effect coverage and unsupported fields. Resolve any disagreement; preserve this provisional freeze. Prepare a new independently attributed test/fixture binding when justified. Do not treat the provisional test as independent evidence.
3. Root alone may apply the reviewed source candidate in its assigned integration worktree. No shared file has been changed by this author.
4. After root inspects source, independent expectations and the ordinary dependency/feature path, the precise proposed selected command is:

```sh
cargo test --frozen -j1 -p mori-core --no-default-features --test financial_pair actus_ump01_matches_frozen_core_trace_and_seven_financial_fields -- --exact --test-threads=1
```

The command above has **not run**. Its proposed success condition is one selected test passing with all assertions reached, not mere compilation or agreement with author cells. Root separately owns resource/cleanup receipt and any ordinary formatting/lint or retained-test checks. The source lane supplies no permission to run broader targets, SDK/native tests or providers. `cargo fmt --all --check`, Clippy and Cargo tests are all UNRUN here; source custody checks in `STATIC-CHECKS.json` are not compilation.

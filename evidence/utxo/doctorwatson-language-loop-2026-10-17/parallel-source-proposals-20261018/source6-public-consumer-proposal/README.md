# Explicit public Source/6 consumer proposal

Unadopted and unrun, authored by GPT-6.1 Sol high. This successor adds a real public CLI consumer of the frozen transfer library. It does not change the original prototype directory. `source6-public-consumer.delta.patch` applies after the original library patch; `source6-public-consumer.combined.patch` contains both against the read-only checkout. Use one route, not both.

Falsifiable source task: with the proposed non-default `source6-expansion` build feature, public Rust `mori check invoice.mori --json` accepts beta/1 and public `mori --quiet expand ... --emit-source6-transfer` calls the real library and prints its actual Source/6 text. The actual emitted bytes must later pass the real TypeScript Source/6 lowerer. No runtime result is claimed here.

Before/default behavior remains:

```sh
mori expand core.mori --action noop --scenario core.json
mori expand invoice.mori --action pay --scenario scenario.json
```

The first prints Core Program JSON after existing reference scenario checking. Rust beta in the second still reaches the existing Core parser and refuses. The separate Node beta package/default expansion output is unchanged.

New explicit behavior proposed for a build with `source6-expansion`:

```sh
mori check invoice.mori --json
mori --quiet expand invoice.mori --action pay --scenario scenario.json --emit-source6-transfer
```

The flag selects `commands/expand/source6.rs`, decodes explicit typed scenario cells through the existing strict reader/UInt decoder, calls `mori_source6::expand_transfer`, and writes `expansion.source6` directly. No hand-generated success or compiler DTO stands in for the helper. The current Core branch body is retained unchanged. Four real-binary tests cover check→explicit expansion, quiet raw text, default Core complete bytes and the default Rust beta refusal boundary.

| Proposed interface | Choice requiring the planned two independent design/interface votes |
|---|---|
| Selection | `--emit-source6-transfer`, enabled by non-default `source6-expansion`; unavailable-feature error before file reads |
| Scenario wire input | Reuse `moriarty-local-scenario/1` and `local-stipulation` for a closed transfer-only subset; canonical decimal strings via existing Core UInt; exact three balance cells; no obligation or candidate_effects override |
| Ordering | JSON/profile decoding precedes the real helper's source/check/selection; this new explicit option makes no Node diagnostic-precedence parity claim |
| Output | Raw Source/6 plus its existing final newline; status line only on stderr; quiet leaves stderr empty; source/schema failures print no Source/6 |
| API/identity | Internal borrowed typed local scenario; no full Node BetaExpansion/provenance/hash wire compatibility claim |

The wire/interface choices are proposals, not adopted language semantics. The original transfer-only library remains byte-frozen in the combined candidate. No Source/6/Core/5→Core/1/Core/2/Sherlock/native embedding, target policy, security judgment, stage success or proof is added.

Expectation and evidence gaps are explicit in `EXPECTATION-STATUS.json`. The Source/6 stdout literal is copied from the original author-prepared pre-code freeze; new scenario/status/Core output literals are manually specified. They are **not independently frozen or reviewed** and were not generated from execution. The complete default-beta stderr byte baseline is missing: its test checks only exit, empty stdout and the existing Core syntax category, not complete byte compatibility. No diagnostic golden was invented. Independent reviewers must freeze/adopt full required expectations before scoring actual results.

Root next steps, within separately inspected authority: obtain two design/interface votes; freeze independent expectations (including default-beta stderr and required refusals); review exact combined source and current library limitations; run allowed formatting/ordinary compilation/tests and fresh execution review; capture actual public stdout and feed those bytes into the unchanged real `parseAndLowerSource6`. A check of only the expected fixture is insufficient. No build, Cargo, Rust, CLI, Node, package, model, SDK, proof, VM, network or key/wallet execution ran here.

Static artifact checks only: original artifact/library hashes, existing Core branch body equality, TOML/lock wiring, expectation-literal/snapshot consistency, diff reconstruction and new file hashes. All U0–U7/M1–M17/MC/SP/G, financial/composition/native and separate Preview/Fenrir exits remain unchanged. This is source preparation, not adoption or observed execution.

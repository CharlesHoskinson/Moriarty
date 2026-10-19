# Delta review: revision02 docs and C1/C2 citations

**Reviewer:** Opus 5.5 HIGH, non-author. This is a delta review only.

**Method.** I read only the supplied text. I ran no commands, Git, compiler, tests, providers, native tools or SDK. I opened no expected or protected bodies. Every statement below is a static reading. Hashes and byte counts are author-attested; I cannot recompute them.

## Verdict: C1 satisfied, C2 satisfied (one binding note), D1/D2/D3 resolved

The revision02 source approval stands. The patch contains no Rust, test, fixture or capture hunks. The 19 non-doc pins match the retained set as supplied, so none of the code findings from the full review are reopened.

The byte deltas are consistent with a docs-only edit, with every other change being a whitespace rewrap:

| File | Byte change | Explanation |
|---|---|---|
| `nominal-text-source.md` | +43 | Exactly `observed nominal admitted image/MatchPlan, ` |
| `nominal-public-command.md` | +82 | The AD and Pass B phrase |
| `nominal-compile-command.md` | +89 | The AD and Pass B phrase, `calls` → `is designed to call`, `retains` → `retain`, and dropping `Those` |

## C1: `mori_core::json::export`

**Satisfied.**

- **Signature.** `json.rs:117` reads `pub fn export(value: &impl serde::Serialize) -> Result<String, Error>`.
- **`S = String`.** `&String` coerces to `&str` for `display::text(&diagnostic)`.
- **`E = crate::Error`.** This is `ir::Error`, re-exported through `pub use ir::*` (`lib.rs:40`). It is `#[derive(Debug, …, thiserror::Error)]` (`ir.rs:466`), so test 06's `.unwrap()` compiles.
- **Symmetry.** `export` emits ASCII with only `\n` as a control character. `display::text` then escapes that newline. The test applies the same `export` → `display::text` chain as `source_io::operation`, so the expected and actual strings stay symmetric.
- **Size limit.** The 1 MiB `export` limit is irrelevant at this payload size.

## C2: `mori_diagnostics::Report` Display

**Satisfied** for the cited configuration.

The display chain, from the cited sources:

1. `mori_diagnostics::Report` is a direct re-export of `miette::Report` 7.6.0 (`mori-diagnostics/src/lib.rs:16`).
2. `SourceIoError::Operation` goes through `.into()`, which is `From<E: Diagnostic>`, then `from_std`. That gives a `Some(handler)` from `capture_handler`.
3. `Display for Report` calls `ErrorImpl::display`, which calls `handler.display(error, f)`.
4. `DebugReportHandler` overrides only `debug`. The default trait `display` writes `{}` of the error, then walks the chain only when `f.alternate()`.
5. `to_string()` is non-alternate.

The result is exactly `"{code}: {message}"`, which is `mori::command: <display::text(...)>`.

`Operation` also has no `#[source]` and `impl Diagnostic for SourceIoError {}` adds no code or help. So even an alternate render would add nothing.

**Binding note (an op02/op09 receipt check, not a source change).**

- The `features=[]` evidence comes from the 24-compiler-qa `core-target` fingerprints (`miette-fc49…` and `mori_diagnostics-3fff…`).
- Op02 and op09 link the mori-side externs from the 882cba / `-L 299` recipe, which may be a different instance.
- If that instance enables `fancy`, the default printer becomes `MietteHandler`. Its `display` behaviour is not in the cited sources. To my knowledge it also does not override `display`, but that is not established by this citation.
- **Before op02:**
  - Record the features of the actually linked `miette` and `mori_diagnostics` instances.
  - If either has `fancy`, cite `MietteHandler`'s `ReportHandler` impl.
- **Hook claim.** The claim that no `set_hook` is installed on the test path rests on the unchanged `nominal_compile_handler.rs` (`4c548c36`), which was reviewed previously. The production `main` is not executed by the test.

## D1: restore the open item in `nominal-text-source.md`

**Resolved.** "observed nominal admitted image/MatchPlan" is back in the "remain open" list, worded as the observation gap rather than as missing implementation.

## D2: name the admission preconditions

**Resolved** in both changed docs. They now name the AD-0a/AD-1/AD-2/AD-3/AD-6 admission preconditions and the Pass B finance-syntax refusal under `admit`. `admit-run` is kept for genesis.

**Limitation.** `explanation/nominal-text-to-admission.md` (pin `0a50ef05`) is unchanged and was not supplied in this delta. I did not re-check whether it carries the same B/T, C/P, V-only phrasing.

## D3: line length and tense

**Resolved** for every cited item:

- The cited over-long lines are rewrapped.
- "retains complete published artifacts" now reads "is designed to call … and retain". "Capture operations and full comparisons remain unrun" is retained.

The rewrap left three over-long lines. These are optional nits and do not block merge:

| File | Line | Length |
|---|---|---|
| `nominal-compile-command.md` | "reports unstepped genesis admission failures. Selection and compile-only observation refusals use" | about 97 chars |
| `nominal-compile-command.md` | "now adds explicit `PreparationReportC` … every preexisting" | about 90 chars |
| `nominal-text-source.md` | "and full original034/refinements remain open. The source-only text-to-admission API" | about 83 chars |

**Out of scope, unchanged text.** `nominal-text-source.md` says "the executed complete fixtures are linked in the evidence archive." If that refers to the earlier parser evidence, it is acceptable. If a reader could take it to mean this cut's fixtures, it overstates, because they are UNRUN. Consider clarifying it in a later docs pass.

## Recorded known behaviour change (no new fix)

**Change.** In `simulate_selection.rs` (`4e052504`), take the nominal profile with a valid `--entry` on the already-selected Sherlock simulation:

- **Before:** a structured `selection_error: UnknownProfile` at stage `select`.
- **Now:** at stage `select`, `actual_error.message` reads "selected Sherlock simulation supports the explicit dedicated draft entry".

**Status.** This is source behaviour that was already in approved revision02, so it is not a delta.

- It is untested and not observed by any of the eleven operations.
- Nominal simulation is still refused before any source, config or runtime read.
- Downstream consumers keyed on `selection_error` will no longer see `UnknownProfile` for this input.

## Retained limitations and non-authorizations

**What remains unchanged:**

- All eleven operations remain **UNRUN**.
- The original revision02 operations and caps remain frozen, and op01 under the identical caps remains **not eligible**.
- This review approves no execution and no resources.
- The changed resource proposal is separate. It needs an independently owned, scoped disposition and Root selection.

**All preserved limitations from the full review still apply:**

- HostOnly representation only.
- Witness and constraint plans missing; export unsupported; native and target evidence unrun.
- `body_typechecking: not-performed`, and an ill-typed body can still be published as admitted.
- Source name and spans absent; semantics `null`.
- The mutual-generic `preparation` section has no expectation.
- Only wrapped-countdown has independent full-table equality, and it is unrun.
- Not certified:
  - body typing
  - old-profile byte compatibility
  - Cargo or DoctorWatson integration
  - the backend-linked CLI
  - native, target, EARS, B04 or H1 acceptance

No protected expectation was read.
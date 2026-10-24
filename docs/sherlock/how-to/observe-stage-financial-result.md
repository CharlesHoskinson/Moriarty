# Observe a Core2 stage financial result

Use the owning `start_observed`/`resume_observed` API below to retain a same-call
program/input/result record. `project_invocation` remains available for diagnostic
manual capture; it cannot establish that arbitrary supplied inputs produced a
supplied result. This guide uses the existing retained-payment fixture and its false
then true events. The [API reference](../reference/financial-observation-adapter.md)
defines the borrowed fields and accounting cases.

## Capture the actual calls

In a Rust application with `mori_core` and `serde_json` available, use this
complete example with the repository root as the working directory. It calls
the existing local Core/2 producer; the projection itself performs no stage
execution. The fixture's action is `pay` and its start/resume caps and CLI action
are null, represented below as `None`.

```rust
use mori_core::{stage, Scenario};
use mori_core::stage::{StageEvent, StageOutcome};
use mori_core::sherlock_financial_projection::{self as projection, InvocationInput};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = "crates/mori-core/tests/fixtures/financial-observation";
    let source = std::fs::read_to_string(format!("{base}/retained-payment.mori"))?;
    let scenario: Scenario = serde_json::from_str(
        &std::fs::read_to_string(format!("{base}/scenario.json"))?)?;
    let inputs: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(format!("{base}/invocations.json"))?)?;
    let events: Vec<StageEvent> = serde_json::from_value(inputs["events"].clone())?;
    let program = stage::load(&source)?;

    let start_result = stage::start(&program, "pay", scenario.clone(), None);
    let start_view = projection::project_invocation(
        &program,
        InvocationInput::Start { action: "pay", scenario: &scenario, cap: None },
        &start_result,
    );
    println!("{:?}\n{start_view:#?}", stage::STAGE_LABELS);

    let mut continuation = match &start_result {
        Ok(StageOutcome::Waiting { continuation, .. }) => continuation.clone(),
        _ => return Ok(()),
    };
    for event in &events {
        let result = stage::resume(
            &program, continuation.clone(), event.clone(), None, None);
        let view = projection::project_invocation(
            &program,
            InvocationInput::Resume {
                continuation: &continuation, event, cli_action: None, cli_cap: None,
            },
            &result,
        );
        println!("{:?}\n{view:#?}", stage::STAGE_LABELS);
        match &result {
            Ok(StageOutcome::Waiting { continuation: next, .. }) => {
                continuation = next.clone();
            }
            _ => break,
        }
    }
    Ok(())
}
```

Retain the result and captured values for as long as a view borrows them. Passing
an unrelated result to the projection does not validate its provenance. Keep
the original full result when exporting an observation; account aggregates
alone omit steps, nonmonetary effects and unaffected state.

## Inspect the published boundary

For the fixed fixture, the existing
[captured result](../../results/sherlock-financial-observation-20261018/README.md)
observed these three invocations. The quantities below summarize the result;
the canonical test compares complete raw values and ordered effects.

| Invocation | Returned boundary and duties | Work and publication |
| --- | --- | --- |
| Start | Reserve UNIT 100 to 68; beneficiary receives 30; fee recipient receives 2; payment duty 40 to 10; unrelated audit duty 5 retained | Attempted 7, committed 7; published segment, frame site 0 to 1 |
| False await | Same full continuation and 68/10 payment boundary; audit duty 5 retained | Attempted 2, committed 7; `segment: None`, no advanced segment |
| True resume | Reserve 68 to 57; beneficiary receives 10 more; fee recipient 1 more; payment duty remains present at zero; residual audit duty 5 | Attempted 8, committed 15; complete published segment, frame site 1 to 0 |

Inspect all snapshot entries, both frames, ordered steps/effects, continuation
captures, residual duties and attempted/committed counters. A missing segment
differs from a present segment whose effects happen to be empty. On rejection,
inspect the actual error/work and retain pre-call input separately; do not label
that input a returned rollback continuation or manufacture a tentative delta.

## Use the captured verification precisely

The canonical test's name still ends in `independent_r152`; R175 is the revised
independent payload. Eleven standalone expectation files are compared in full;
the large `EXPECTED.json` compares only program-hash and plan-hash cells.
The original admission failure is preserved alongside the later positive
receipt. R176 executed the positive test; R189 only compiled the canonical
test. This example has no separate execution receipt.

Use those existing receipts for this documented observation rather than treating
a broad Core test invocation as equivalent evidence. Core development
dependencies include the Midnight SDK. The local stage labels and objective
language rules apply to public developer use; project review receipts introduce
no maintainer permission step. The wider Sherlock financial producer,
UIREQ-017/018/019 completion and native settlement retain their separate scope.


## Use the owning call record

The [reviewed producer candidate and actual positive check](../../results/sherlock-owning-financial-producer-20261018/README.md)
use `financial-producer-integration` fixtures. Load the staged program, then call
`start_observed(&program, action_string, scenario, cap)`. It returns a
`ProducedInvocation` directly; inspect `raw_result()` for Waiting, Complete or
Rejected. Clone a returned Waiting continuation before passing it, the next event
and optional flags to `resume_observed`. The actual positive test feeds the false
resume from the real start continuation and the true resume from that returned
continuation; it does not refill actual values from expected JSON.

Read `observation()` for full borrowed inputs/result/accounting, or call
`publication()` for the shared JSON value. Neither invokes stage execution again.
Retain the produced record and its external program for the borrow's lifetime.
The publisher preserves local-stage qualification/authentication/consumption
labels and existing status/work fields alongside `financial_observation`.
Do not replace `segment: None` with an empty execution or use per-segment account
aggregates as cumulative prefix history.

The one exact positive test compared the complete values in memory. It saved no
financial JSON report and did not execute CLI dispatch, file IO or stdout. The
manual snippet above still has no separate execution receipt. The final source/result review recommends whole017 closure for this declared
producer/adapter/shared-publication domain; Root owns adoption. Native, proof,
settlement and later-failure claims remain separate.

Root has now [adopted whole UIREQ-017](../../results/sherlock-owning-financial-producer-20261018/ROOT-FINANCIAL-PRODUCER-ADOPTION.json). The [current requirement report](../../results/sherlock-owning-financial-producer-20261018/ROOT-CURRENTNESS-READBACK.json) confirms fourteen current original closures. The source-only enclosing CLI and separate lowering, failure, authentication, proof and settlement qualifications above remain.


## Read the returned await prefix

Keep the owning `ProducedInvocation` and its program alive, then inspect `produced.observation().await_prefix()`. For `Waiting`, read `committed_boundary`, `logical`, `residual` and the optional `published_segment` alongside action/site/captures and attempted work. A false guard can return the same boundary with no advanced segment. On `Complete` or `RejectedNoReturnedCheckpoint`, inspect the full original result/financial observation; do not manufacture a returned checkpoint.

Use `produced.publication()` for the shared JSON value; it includes top-level `await_prefix` and the complete existing financial observation. These accessors do not invoke another producer call. The [fixed ordinary observations](../../results/sherlock-financial-await-prefix-20261018/README.md) compare full publications in memory; they do not execute CLI dispatch or persist a financial report. Preserve authentication none/consumption untracked. Generic018/019, full013 and native settlement retain their own obligations.


## Distinguish a stage observation from a carrier-component result

For the separate financial-carrier increment, read [its current reference](../reference/financial-carrier-binding.md) and [actual qualification](../../results/sherlock-financial-carrier-unit02-source-20261020/README.md). Confirm the producing call and retained result before interpreting the pass: the sixteen checked tests exercise the sealed crate-private carrier, whereas this guide's `start_observed`/`resume_observed` route captures the existing Core/2 producer. Neither result can be renamed a completed universal financial owner pipeline or a ledger settlement.

When inspecting a new report, keep the original source/input/configuration, actual admitted pair, financial genesis and first owner event distinct. Require actual observation at each claimed link; don't fill a missing link with another wrapper around the same evaluator. This section adds no runnable command, replacement evaluator or new permission requirement. Existing observed examples retain their original source cuts.

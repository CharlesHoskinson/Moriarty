from __future__ import annotations

import json
from dataclasses import replace
from pathlib import Path

from moriarty.backend import BackendMachine, BackendResult
from moriarty.certificate import (
    generate_traces,
    validate_translation,
    write_experiment,
)
from moriarty.swap import SwapParameters


PARAMETERS = SwapParameters.example()


def test_trace_generator_produces_one_thousand_distinct_traces() -> None:
    traces = generate_traces(PARAMETERS, minimum=1_000)

    assert len(traces) == 1_000
    assert len({trace.steps for trace in traces}) == 1_000
    assert {len(trace.steps) for trace in traces} >= {1, 2, 3, 4}


def test_certificate_has_zero_divergence_and_complete_stop_test_coverage() -> None:
    traces = generate_traces(PARAMETERS, minimum=1_000)

    certificate = validate_translation(PARAMETERS, traces)

    assert certificate["trace_count"] == 1_000
    assert certificate["unique_trace_count"] == 1_000
    assert certificate["divergence_count"] == 0
    assert certificate["first_divergence"] is None
    assert certificate["invariant_failure_count"] == 0
    assert set(certificate["coverage"]["accepted_transitions"]) == {
        "WaitingAlice:deposit->WaitingBob",
        "WaitingAlice:expire->Refunded",
        "WaitingBob:deposit->WaitingDecision",
        "WaitingBob:expire->Refunded",
        "WaitingDecision:choice-0->Refunded",
        "WaitingDecision:choice-1->Settled",
        "WaitingDecision:expire->Refunded",
    }
    assert set(certificate["coverage"]["rejection_codes"]) == {
        "choice_out_of_bounds",
        "contract_closed",
        "input_required",
        "no_matching_input",
        "time_before_state",
    }
    assert certificate["coverage"]["terminal_phases"] == ["Refunded", "Settled"]
    assert certificate["coverage"]["deadline_offsets"] == [-1, 0, 1]
    assert {
        "Refunded:expire->contract_closed",
        "Settled:expire->contract_closed",
    } <= set(certificate["coverage"]["rejected_transitions"])
    assert len(certificate["required_coverage"]["boundary_cells"]) == 18
    assert set(certificate["producer_source_sha256"]) == {
        "moriarty/backend.py",
        "moriarty/bounds.py",
        "moriarty/certificate.py",
        "moriarty/compact.py",
        "moriarty/core.py",
        "moriarty/swap.py",
    }
    for key, required in certificate["required_coverage"].items():
        assert set(required) <= set(certificate["coverage"][key])
    assert len(certificate["certificate_sha256"]) == 64


def test_stop_test_rejects_a_large_but_incomplete_trace_corpus() -> None:
    candidates = generate_traces(PARAMETERS, minimum=20_000)
    traces = tuple(
        trace
        for trace in candidates
        if all(step.kind != "expire" for step in trace.steps)
    )[:1_000]

    assert len(traces) == 1_000
    certificate = validate_translation(PARAMETERS, traces)

    assert certificate["divergence_count"] == 0
    assert certificate["invariant_failure_count"] == 0
    assert certificate["stop_test_passed"] is False


def test_certificate_is_byte_stable_for_the_same_inputs() -> None:
    traces = generate_traces(PARAMETERS, minimum=1_000)

    first = validate_translation(PARAMETERS, traces)
    second = validate_translation(PARAMETERS, traces)

    assert json.dumps(first, sort_keys=True) == json.dumps(second, sort_keys=True)


def test_certificate_detects_a_corrupted_backend() -> None:
    class CorruptMachine(BackendMachine):
        def apply(self, *args, **kwargs):
            result = super().apply(*args, **kwargs)
            if result.accepted and result.state.phase == "WaitingBob":
                return BackendResult(
                    True,
                    replace(result.state, alice_balance=result.state.alice_balance + 1),
                    result.payments,
                    result.warnings,
                    result.error,
                )
            return result

    traces = generate_traces(PARAMETERS, minimum=20)

    certificate = validate_translation(
        PARAMETERS,
        traces,
        machine_factory=CorruptMachine,
    )

    assert certificate["divergence_count"] > 0
    assert certificate["first_divergence"] is not None


def test_experiment_writer_preserves_hashed_artifacts(tmp_path: Path) -> None:
    result = write_experiment(tmp_path, PARAMETERS, minimum_traces=1_000)

    source = (tmp_path / "swap.compact").read_text(encoding="utf-8")
    manifest = json.loads(
        (tmp_path / "artifact-manifest.json").read_text(encoding="utf-8")
    )
    certificate = json.loads(
        (tmp_path / "translation-certificate.json").read_text(encoding="utf-8")
    )

    assert result == certificate
    assert manifest["compact_sha256"] == certificate["compact_sha256"]
    assert source.endswith("\n")
    assert certificate["divergence_count"] == 0
    assert "/home/" not in json.dumps(manifest)


def test_backend_model_does_not_import_the_core_interpreter() -> None:
    backend_source = (
        Path(__file__).resolve().parents[1] / "moriarty" / "backend.py"
    ).read_text(encoding="utf-8")

    assert "moriarty.core" not in backend_source

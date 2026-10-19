"""Pure ordinary decoder for AGY's observed nested terminal wire path.

No file/process/network calls or recursive candidate search. The caller supplies
the terminal process status, immutable-input observation, schema and token.
This module neither attests backend identity nor evaluates review correctness.
"""
from copy import deepcopy
import json

from jsonschema import validators
from jsonschema.exceptions import SchemaError, ValidationError
from referencing import Registry
from referencing.exceptions import Unresolvable

POSITIVE_STATUSES = frozenset({"SUCCESS"})
_PARSE_FAILURE = object()


def select_terminal_review(
    events,
    *,
    schema,
    review_token,
    terminal_exit_code,
    input_pins_unchanged,
    capture_parse_errors=0,
):
    """Return EXPORT/full review or NO_EXPORT/None; never mutate inputs.

    ``events`` is an iterable of parsed top-level JSON values. The single
    terminal result must be the last parsed event. Only its exact nested
    result.structured_output dictionary is eligible. Unknown nonterminal
    dictionary events are ignored, never recursively searched.

    Input-pin truth and exit0 are externally supplied observations, not checks
    performed by this pure function. Schema validation uses the installed
    jsonschema validator for a known declared draft (2020-12 when omitted),
    with an empty, non-retrieving reference registry. Local schema references
    may resolve; missing external resources produce NO_EXPORT, never a fetch.
    """
    receipt = {
        "policy": "agy-single-nested-terminal-structured-output",
        "terminalExitCode": terminal_exit_code,
        "inputPinsUnchangedSupplied": input_pins_unchanged is True,
        "captureParseErrors": 0,
        "resultEvents": 0,
        "terminalStatusLiteral": None,
        "cliInitModelLabels": [],
        "actualBackendIdentityAttested": False,
        "noRecursiveOrTextFallback": True,
    }

    def outcome(reason, review=None):
        decision = "EXPORT" if review is not None else "NO_EXPORT"
        return {"decision": decision, "reason": reason,
                "review": deepcopy(review),
                "receipt": dict(receipt, decision=decision, reason=reason)}

    if type(capture_parse_errors) is not int or capture_parse_errors < 0:
        return outcome("INVALID_PARSE_ERROR_COUNT")
    parse_errors = capture_parse_errors
    event_not_object = False
    event_count = 0
    result_position = None
    payload = None
    for event in events:
        if event is _PARSE_FAILURE:
            parse_errors += 1
            continue
        event_count += 1
        if not isinstance(event, dict):
            event_not_object = True
            continue
        if event.get("event") == "init":
            init = event.get("init")
            if isinstance(init, dict) and isinstance(init.get("model"), str):
                label = init["model"]
                if label not in receipt["cliInitModelLabels"]:
                    receipt["cliInitModelLabels"].append(label)
        if event.get("event") == "result":
            receipt["resultEvents"] += 1
            result_position = event_count
            if receipt["resultEvents"] == 1:
                payload = event.get("result")

    receipt["captureParseErrors"] = parse_errors
    if parse_errors:
        return outcome("CAPTURE_PARSE_ERRORS")
    if event_not_object:
        return outcome("EVENT_NOT_OBJECT")
    if type(terminal_exit_code) is not int or terminal_exit_code != 0:
        return outcome("NONZERO_OR_MISSING_EXIT")
    if input_pins_unchanged is not True:
        return outcome("INPUT_PINS_NOT_UNCHANGED")
    if receipt["resultEvents"] == 0:
        return outcome("NO_RESULT_EVENT")
    if receipt["resultEvents"] != 1:
        return outcome("MULTIPLE_RESULT_EVENTS")
    if result_position != event_count:
        return outcome("RESULT_NOT_LAST")
    if not isinstance(payload, dict):
        return outcome("RESULT_PAYLOAD_NOT_OBJECT")
    if "status" not in payload:
        return outcome("MISSING_RESULT_STATUS")
    status = payload["status"]
    if not isinstance(status, str):
        return outcome("RESULT_STATUS_NOT_STRING")
    receipt["terminalStatusLiteral"] = status
    if status not in POSITIVE_STATUSES:
        return outcome("UNRECOGNIZED_RESULT_STATUS")
    if "structured_output" not in payload:
        return outcome("MISSING_STRUCTURED_OUTPUT")
    review = payload["structured_output"]
    if not isinstance(review, dict):
        return outcome("STRUCTURED_OUTPUT_NOT_OBJECT")
    if not isinstance(review_token, str) or not review_token or review.get("reviewToken") != review_token:
        return outcome("REVIEW_TOKEN_MISMATCH")

    if not isinstance(schema, (dict, bool)):
        return outcome("SCHEMA_INVALID")
    if isinstance(schema, dict) and "$schema" in schema:
        if not isinstance(schema["$schema"], str):
            return outcome("SCHEMA_INVALID")
        validator_class = validators.validator_for(schema, default=None)
        if validator_class is None:
            return outcome("SCHEMA_DIALECT_UNSUPPORTED")
    else:
        validator_class = validators.Draft202012Validator
    try:
        validator_class.check_schema(schema)
        validator_class(schema, registry=Registry()).validate(review)
    except SchemaError:
        return outcome("SCHEMA_INVALID")
    except ValidationError:
        return outcome("SCHEMA_VALIDATION_FAILED")
    except Unresolvable:
        return outcome("SCHEMA_REFERENCE_UNRESOLVED")
    return outcome("SINGLE_POSITIVE_TERMINAL_STRUCTURED_OUTPUT", review)


def decode_terminal_json_lines(lines, **conditions):
    """Consume supplied text lines once; malformed lines prevent any export.

    Blank lines are ignored. This adapter opens no files and launches no
    producer; root owns acquisition and the terminal/pin observations.
    """
    def events():
        for line in lines:
            if not isinstance(line, str):
                yield _PARSE_FAILURE
                continue
            if not line.strip():
                continue
            try:
                yield json.loads(line)
            except (ValueError, RecursionError):
                yield _PARSE_FAILURE

    return select_terminal_review(events(), **conditions)

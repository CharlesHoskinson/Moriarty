"""Eight ordinary wire-format regressions. Authored before decoder; UNRUN."""
from copy import deepcopy
import json
import unittest

from agy_terminal_selector import decode_terminal_json_lines, select_terminal_review

TOKEN = "decoder-normal-token"
SCHEMA = {
    "type": "object",
    "additionalProperties": False,
    "required": ["reviewToken", "nonceEcho", "kind"],
    "properties": {
        "reviewToken": {"type": "string"},
        "nonceEcho": {"type": "string", "const": TOKEN},
        "kind": {"type": "string", "const": "public-transport-probe-only"},
    },
}
GOOD = {"reviewToken": TOKEN, "nonceEcho": TOKEN, "kind": "public-transport-probe-only"}


def terminal(status="SUCCESS", output=None):
    return {"event": "result", "result": {
        "conversation_id": "format-fixture", "status": status,
        "response": "ordinary response text is not selected",
        "duration_seconds": 1.0, "num_turns": 1,
        "structured_output": deepcopy(GOOD if output is None else output),
        "json_schema": {"type": "object"}, "usage": {},
    }}


def select(events, **kwargs):
    args = {"schema": SCHEMA, "review_token": TOKEN,
            "terminal_exit_code": 0, "input_pins_unchanged": True}
    args.update(kwargs)
    return select_terminal_review(events, **args)


class AgYTerminalFormatTests(unittest.TestCase):
    def stopped(self, outcome, reason):
        self.assertEqual(outcome["decision"], "NO_EXPORT")
        self.assertEqual(outcome["reason"], reason)
        self.assertIsNone(outcome["review"])

    def test_01_actual_nested_wire_path_exports_full_object_only(self):
        for status in ("SUCCESS",):
            with self.subTest(status=status):
                events = [{"event": "init", "init": {"model": "gemini-4-argon-high"}},
                          {"event": "step_update", "step_update": {"step_type": "message"}},
                          terminal(status)]
                original = deepcopy(events)
                outcome = select(events)
                self.assertEqual(outcome["decision"], "EXPORT")
                self.assertEqual(outcome["review"], GOOD)
                self.assertEqual(outcome["receipt"]["terminalStatusLiteral"], status)
                self.assertEqual(outcome["receipt"]["cliInitModelLabels"], ["gemini-4-argon-high"])
                self.assertFalse(outcome["receipt"]["actualBackendIdentityAttested"])
                outcome["review"]["kind"] = "changed returned copy"
                self.assertEqual(events, original)

    def test_02_progress_tool_arguments_are_never_candidates(self):
        events = [{"event": "step_update", "step_update": {
            "step_type": "tool_call", "tool_info": {"parameters": {
                "event": "result", "result": {"status": "success", "structured_output": GOOD}
            }}, "structured_output": GOOD,
        }}]
        self.stopped(select(events), "NO_RESULT_EVENT")

    def test_03_response_text_or_response_object_is_never_selected(self):
        for response in (json.dumps(GOOD), {"structured_output": GOOD}):
            with self.subTest(response=response):
                event = terminal()
                del event["result"]["structured_output"]
                event["result"]["response"] = response
                self.stopped(select([event]), "MISSING_STRUCTURED_OUTPUT")

    def test_04_duplicate_terminal_is_not_deduplicated_into_success(self):
        event = terminal()
        self.stopped(select([event, deepcopy(event)]), "MULTIPLE_RESULT_EVENTS")
        second = terminal("partial")
        self.stopped(select([event, second]), "MULTIPLE_RESULT_EVENTS")

    def test_05_partial_unknown_missing_status_is_not_positive(self):
        for status in ("success", "completed", "partial", "running", "cancelled", "error", "unrecognized", ""):
            with self.subTest(status=status):
                self.stopped(select([terminal(status)]), "UNRECOGNIZED_RESULT_STATUS")
        missing = terminal()
        del missing["result"]["status"]
        self.stopped(select([missing]), "MISSING_RESULT_STATUS")
        self.stopped(select([terminal(None)]), "RESULT_STATUS_NOT_STRING")

    def test_06_missing_or_wrong_wire_shapes_do_not_fall_back(self):
        for event in ({"event": "result"}, {"event": "result", "result": "text"}):
            with self.subTest(event=event):
                self.stopped(select([event]), "RESULT_PAYLOAD_NOT_OBJECT")
        event = terminal()
        event["result"]["structured_output"] = [GOOD]
        self.stopped(select([event]), "STRUCTURED_OUTPUT_NOT_OBJECT")
        event = terminal()
        del event["result"]["structured_output"]
        event["structured_output"] = GOOD
        self.stopped(select([event]), "MISSING_STRUCTURED_OUTPUT")
        self.stopped(select([{"type": "result", "structured_output": GOOD}]), "NO_RESULT_EVENT")
        self.stopped(select([[], terminal()]), "EVENT_NOT_OBJECT")
        self.stopped(select([terminal(), {"event": "step_update", "step_update": {}}]), "RESULT_NOT_LAST")

    def test_07_external_token_schema_exit_and_pin_conditions_all_apply(self):
        wrong = dict(GOOD, reviewToken="different-token")
        self.stopped(select([terminal(output=wrong)]), "REVIEW_TOKEN_MISMATCH")
        invalid = dict(GOOD)
        del invalid["kind"]
        # Envelope json_schema is permissive; the externally supplied schema wins.
        self.stopped(select([terminal(output=invalid)]), "SCHEMA_VALIDATION_FAILED")
        self.stopped(select([terminal()], schema={"type": 42}), "SCHEMA_INVALID")
        self.stopped(select([terminal()], terminal_exit_code=124), "NONZERO_OR_MISSING_EXIT")
        self.stopped(select([terminal()], terminal_exit_code=None), "NONZERO_OR_MISSING_EXIT")
        self.stopped(select([terminal()], input_pins_unchanged=False), "INPUT_PINS_NOT_UNCHANGED")

    def test_08_json_line_parse_failure_cannot_export_a_later_full_result(self):
        args = {"schema": SCHEMA, "review_token": TOKEN,
                "terminal_exit_code": 0, "input_pins_unchanged": True}
        good_line = json.dumps(terminal("SUCCESS")) + "\n"
        outcome = decode_terminal_json_lines(iter(["\n", good_line, "  \n"]), **args)
        self.assertEqual(outcome["decision"], "EXPORT")
        self.assertEqual(outcome["review"], GOOD)
        for lines in (["{\n", good_line], [good_line, '{"event":']):
            with self.subTest(lines=lines):
                outcome = decode_terminal_json_lines(iter(lines), **args)
                self.stopped(outcome, "CAPTURE_PARSE_ERRORS")
                self.assertEqual(outcome["receipt"]["captureParseErrors"], 1)


if __name__ == "__main__":
    unittest.main()

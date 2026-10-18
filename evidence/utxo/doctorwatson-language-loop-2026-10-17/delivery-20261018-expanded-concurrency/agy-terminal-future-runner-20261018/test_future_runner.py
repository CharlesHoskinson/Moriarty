"""Focused ordinary format/custody tests; no subprocess/provider is launched."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from future_runner import LiveCapture, build_argv, finish_capture, load_task, pins_match

TOKEN = "future-format-token"
SCHEMA = {"type": "object", "additionalProperties": False,
          "required": ["reviewToken", "answer"],
          "properties": {"reviewToken": {"const": TOKEN}, "answer": {"const": "normal"}}}
GOOD = {"reviewToken": TOKEN, "answer": "normal"}


def result():
    return {"event": "result", "result": {"status": "SUCCESS", "structured_output": GOOD}}


def bundle():
    return {"task": {"reviewToken": TOKEN}, "schema": SCHEMA}


def observed(*values):
    capture = LiveCapture()
    for value in values:
        capture.feed((json.dumps(value) + "\n").encode())
    capture.finish()
    return capture


class FutureRunnerFormatTests(unittest.TestCase):
    def test_progress_lookalike_never_becomes_terminal_review(self):
        capture = observed({"event": "step_update", "step_update": {
            "tool_info": {"parameters": result()}, "structured_output": GOOD}})
        outcome = finish_capture(capture, bundle(), 0, True)
        self.assertEqual(outcome["reason"], "NO_RESULT_EVENT")
        self.assertIsNone(outcome["review"])

    def test_new_fragmented_stdout_selects_only_exact_full_terminal(self):
        wire = (json.dumps({"event": "init", "init": {"model": "gemini-4-argon-high"}})
                + "\n" + json.dumps(result()) + "\n").encode()
        capture = LiveCapture()
        for index in range(0, len(wire), 7):
            capture.feed(wire[index:index + 7])
        capture.finish()
        outcome = finish_capture(capture, bundle(), 0, True)
        self.assertEqual(outcome["decision"], "EXPORT")
        self.assertEqual(outcome["review"], GOOD)
        self.assertFalse(outcome["receipt"]["actualBackendIdentityAttested"])
        self.assertEqual(capture.byte_count, len(wire))

    def test_all_parsed_nonobjects_are_retained_and_rejected(self):
        for nonobject in (None, [], "ordinary-string", 3):
            with self.subTest(nonobject=nonobject):
                capture = observed(nonobject, result())
                self.assertEqual(len(capture.events), 2)
                self.assertEqual(capture.events[0], nonobject)
                self.assertEqual(finish_capture(capture, bundle(), 0, True)["reason"], "EVENT_NOT_OBJECT")

    def test_partial_json_or_incomplete_capture_never_exports(self):
        capture = observed(result())
        capture.feed(b'{"event":')
        capture.finish()
        self.assertEqual(finish_capture(capture, bundle(), 0, True)["reason"], "CAPTURE_PARSE_ERRORS")
        capture = observed(result())
        capture.complete = False
        self.assertEqual(finish_capture(capture, bundle(), 0, True)["reason"], "CAPTURE_NOT_COMPLETE")

    def test_duplicate_or_response_only_candidates_do_not_export(self):
        self.assertEqual(finish_capture(observed(result(), result()), bundle(), 0, True)["reason"], "MULTIPLE_RESULT_EVENTS")
        event = {"event": "result", "result": {"status": "SUCCESS", "response": json.dumps(GOOD)}}
        self.assertEqual(finish_capture(observed(event), bundle(), 0, True)["reason"], "MISSING_STRUCTURED_OUTPUT")

    def test_exit_and_postcapture_pin_conditions_are_required(self):
        capture = observed(result())
        self.assertEqual(finish_capture(capture, bundle(), 124, True)["reason"], "NONZERO_OR_MISSING_EXIT")
        self.assertEqual(finish_capture(capture, bundle(), 0, False)["reason"], "INPUT_PINS_NOT_UNCHANGED")

    def test_exact_one_task_argv_keeps_high_plan_sandbox_and_zero_inner_timeout(self):
        task = {"model": "gemini-4-argon-high", "wallSeconds": 420, "packet": "/public/packet",
                "schemaFile": "/public/schema.json"}
        argv = build_argv({"task": task, "prompt": "complete ordinary prompt"}, Path("/new/output"))
        self.assertEqual(argv[:5], ["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "420s", "/home/charl/.local/bin/agy"])
        for flag, value in [("--model", "gemini-4-argon-high"), ("--effort", "high"), ("--mode", "plan"),
                            ("--output-format", "stream-json"), ("--print-timeout", "0")]:
            self.assertEqual(argv[argv.index(flag) + 1], value)
        self.assertIn("--sandbox", argv)
        self.assertEqual(argv[-2:], ["--print", "complete ordinary prompt"])

    def test_frozen_public_task_prompt_schema_manifest_and_file_are_rechecked(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            packet = root / "packet"
            packet.mkdir()
            source = packet / "source.txt"
            source.write_text("ordinary public source\n")
            digest = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
            manifest = packet / "MANIFEST.json"
            manifest.write_text(json.dumps({"files": [{"path": "source.txt", "sha256": digest(source)}]}))
            prompt = root / "PROMPT.md"
            prompt.write_text("read ordinary public source")
            schema = root / "schema.json"
            schema.write_text(json.dumps(SCHEMA))
            task = root / "TASK.json"
            task.write_text(json.dumps({"id": "one-normal-task", "model": "gemini-4-argon-high",
                "packet": str(packet), "manifestSha256": digest(manifest), "reviewToken": TOKEN,
                "promptFile": str(prompt), "promptSha256": digest(prompt), "schemaFile": str(schema),
                "schemaSha256": digest(schema), "wallSeconds": 420}))
            loaded = load_task(task, digest(task))
            self.assertTrue(pins_match(loaded["pins"]))
            source.write_text("changed ordinary public source\n")
            self.assertFalse(pins_match(loaded["pins"]))


if __name__ == "__main__":
    unittest.main()

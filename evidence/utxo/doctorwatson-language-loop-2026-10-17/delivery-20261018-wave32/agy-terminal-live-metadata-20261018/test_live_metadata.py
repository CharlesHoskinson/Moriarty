"""Ordinary synthetic format checks; no provider, subprocess, or capture files."""
import json
import unittest

from future_runner import LiveCapture, finish_capture


class Clock:
    def __init__(self):
        self.now = 10.0
    def __call__(self):
        return self.now


class LiveMetadataTests(unittest.TestCase):
    def capture(self):
        self.clock = Clock()
        return LiveCapture(clock=self.clock)

    def feed(self, capture, value):
        capture.feed((json.dumps(value) + "\n").encode())

    def result(self, status="SUCCESS", output=None):
        return {"event": "result", "result": {"status": status,
                "structured_output": {} if output is None else output}}

    def test_timeout_keeps_observed_shape_without_export(self):
        c = self.capture()
        self.clock.now = 12
        self.feed(c, self.result())
        self.clock.now = 435
        c.observe_process_exit(-9)
        m = c.live_metadata()
        self.assertEqual(m["resultObservations"], [{"parsedValueIndex": 0,
            "observedAfterSeconds": 2.0, "resultType": "object",
            "statusClass": "ALLOWLISTED", "statusLiteral": "SUCCESS",
            "structuredOutputType": "object"}])
        self.assertEqual(m["processExit"], {"code": -9, "observedAfterSeconds": 425.0})
        self.assertIsNone(m["eofObservedAfterSeconds"])
        outcome = finish_capture(c, {"schema": {}, "task": {"reviewToken": "t"}}, -9, True)
        self.assertEqual((outcome["decision"], outcome["reason"], outcome["review"]),
                         ("NO_EXPORT", "CAPTURE_NOT_COMPLETE", None))

    def test_late_eof_has_independent_observation_time(self):
        c = self.capture()
        self.feed(c, self.result())
        self.clock.now = 13
        c.observe_process_exit(0)
        self.clock.now = 17
        c.finish()
        m = c.live_metadata()
        self.assertEqual(m["processExit"]["observedAfterSeconds"], 3.0)
        self.assertEqual(m["eofObservedAfterSeconds"], 7.0)
        self.assertTrue(m["captureReachedEOF"])

    def test_progress_response_and_tool_lookalikes_not_observed(self):
        c = self.capture()
        nested = self.result()
        self.feed(c, {"event": "step_update", "response": nested, "tool_params": nested})
        self.feed(c, {"event": "init", "init": {"model": "not-attestation"}})
        self.assertEqual(c.live_metadata()["eventCounts"],
                         {"init": 1, "step_update": 1, "result": 0, "otherObject": 0, "nonObject": 0})
        self.assertEqual(c.live_metadata()["resultObservations"], [])

    def test_only_exact_success_literal_is_exportable_metadata(self):
        c = self.capture()
        self.feed(c, self.result("success"))
        self.feed(c, self.result("arbitrary provider response text"))
        for observation in c.live_metadata()["resultObservations"]:
            self.assertEqual(observation["statusClass"], "OTHER_STRING")
            self.assertIsNone(observation["statusLiteral"])
        self.assertNotIn("arbitrary provider", json.dumps(c.live_metadata()))

    def test_missing_and_wrong_result_shapes_have_only_types(self):
        c = self.capture()
        self.feed(c, {"event": "result"})
        self.feed(c, {"event": "result", "result": {"status": ["secret"], "structured_output": "text"}})
        a, b = c.live_metadata()["resultObservations"]
        self.assertEqual((a["resultType"], a["statusClass"], a["structuredOutputType"]),
                         ("missing", "RESULT_NOT_OBJECT", "missing"))
        self.assertEqual((b["statusClass"], b["structuredOutputType"]), ("NON_STRING", "string"))
        self.assertNotIn("secret", json.dumps(c.live_metadata()))

    def test_duplicate_results_counted_observations_bounded(self):
        c = self.capture()
        for _ in range(5):
            self.feed(c, self.result())
        m = c.live_metadata()
        self.assertEqual(m["eventCounts"]["result"], 5)
        self.assertEqual(len(m["resultObservations"]), 2)
        self.assertEqual(m["resultObservationsOmitted"], 3)
        self.assertEqual(len(c.events), 5)

    def test_partial_parse_and_nonobjects_retained_not_normalized(self):
        c = self.capture()
        self.feed(c, ["private text"])
        c.feed(b'{"event":"result"')
        self.assertEqual(c.live_metadata()["eventCounts"]["nonObject"], 1)
        self.assertEqual(c.events, [["private text"]])
        c.finish()
        self.assertEqual(c.live_metadata()["captureParseErrors"], 1)
        self.assertTrue(c.complete)
        self.assertNotIn("private text", json.dumps(c.live_metadata()))

    def test_metadata_snapshot_cannot_mutate_capture_or_exit_observation(self):
        c = self.capture()
        self.feed(c, self.result())
        c.observe_process_exit(0)
        m = c.live_metadata()
        m["eventCounts"]["result"] = 999
        m["resultObservations"][0]["statusLiteral"] = "changed"
        m["processExit"]["code"] = 99
        self.clock.now = 99
        c.observe_process_exit(-9)
        n = c.live_metadata()
        self.assertEqual(n["eventCounts"]["result"], 1)
        self.assertEqual(n["resultObservations"][0]["statusLiteral"], "SUCCESS")
        self.assertEqual(n["processExit"], {"code": 0, "observedAfterSeconds": 0.0})


if __name__ == "__main__":
    unittest.main()

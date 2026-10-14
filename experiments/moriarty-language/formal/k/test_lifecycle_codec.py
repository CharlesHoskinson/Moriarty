"""Offline lifecycle K transport tests. No K subprocess."""
import json
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent

try:
    import lifecycle_codec as lc
except ModuleNotFoundError:
    lc = None


def named(kind, name):
    return {"node": kind, "name": name, "params": []}


def token(sort, value):
    return {"node": "KToken", "sort": named("KSort", sort), "token": value}


def app(label, args):
    return {"node": "KApply", "label": named("KLabel", label), "arity": len(args), "args": args}


def wire(out, cells=None):
    cells = cells or {}
    children = [
        app("<k>", [{"node": "KSequence", "arity": 0, "items": []}]),
        app("<out>", [out]),
        app("<generatedCounter>", [token("Int", "0")]),
    ]
    for name, value in cells.items():
        children.append(app(name, [value]))
    return json.dumps({"format": "KAST", "version": 4, "term": app("<generatedTop>", children)})


class LifecycleCodecTests(unittest.TestCase):
    def test_codec_exists(self):
        self.assertIsNotNone(lc, "lifecycle_codec must exist")

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_missing_context_uses_distinct_constructor(self):
        packet = lc.admit_packet({"schema": "{}", "request": "{}"})
        self.assertEqual(packet["constructor"], "lifecycleMissingContext")
        self.assertEqual(set(packet["fields"]), {"schema", "request"})
        encoded = lc.encode_packet(packet)
        self.assertIn("lifecycleMissingContext", json.dumps(encoded))
        self.assertNotIn("lifecycleRequest", json.dumps(encoded))

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_present_null_financial_is_transport_error_not_omission(self):
        with self.assertRaisesRegex(lc.CodecError, "LIFECYCLE_TRANSPORT"):
            lc.admit_packet({"schema": "{}", "request": "{}", "financialPreState": None})

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_financial_json_parse_keeps_last_duplicate_key_and_numbers(self):
        text = '{"schemaVersion":"moriarty-financial-lifecycle-state/1","a":1,"a":2,"n":1e2}'
        packet = lc.admit_packet({"schema": "{}", "request": "{}", "financialPreState": text})
        self.assertEqual(packet["constructor"], "lifecycleRequest")
        self.assertEqual(packet["fields"]["financialPreState"]["original"], text)
        parsed = packet["fields"]["financialPreState"]["parsed"]
        self.assertNotEqual(parsed.get("tag"), "parseFailure")
        self.assertEqual(parsed["a"], 2)
        self.assertEqual(parsed["n"], 100.0)

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_invalid_financial_json_is_parse_failure_not_host_verdict(self):
        text = '{"schemaVersion":'
        packet = lc.admit_packet({"schema": "{}", "request": "{}", "financialPreState": text})
        self.assertEqual(packet["fields"]["financialPreState"]["original"], text)
        self.assertEqual(packet["fields"]["financialPreState"]["parsed"], {"tag": "parseFailure"})

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_schema_request_remain_canonical_and_reject_duplicates(self):
        with self.assertRaises(lc.CodecError):
            lc.admit_packet({"schema": '{"a":1,"a":2}', "request": "{}", "financialPreState": "{}"})

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_byte_limits_precede_normalization(self):
        huge = "x" * 65537
        with self.assertRaisesRegex(lc.CodecError, "INPUT_BOUND"):
            lc.admit_packet({"schema": huge, "request": "{}", "financialPreState": "{}"})
        with self.assertRaisesRegex(lc.CodecError, "INPUT_BOUND"):
            lc.admit_packet({"schema": "{}", "request": "{}", "financialPreState": huge})

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_four_result_variants_and_null_action_index(self):
        schema = "{}"
        request = "{}"
        financial = "{}"
        packet = lc.admit_packet({"schema": schema, "request": request, "financialPreState": financial})
        schema_term = lc.encode_text(schema, canonical=True)
        request_term = lc.encode_text(request, canonical=True)
        financial_term = lc.encode_financial(financial)
        cells = {"<sigma>": schema_term, "<request>": request_term, "<financial>": financial_term}
        prepared = app("lcPrepared", [
            lc.encode_json_value({"phase": "1"}),
            lc.encode_json_value({"schemaVersion": "moriarty-financial-lifecycle-state/1"}),
            lc.encode_json_list([]),
            token("Int", "413"),
        ])
        result = lc.decode_result(wire(prepared, cells), packet)
        self.assertEqual(result["status"], "FundedExpressionPrepared")
        self.assertEqual(result["workRemaining"], "413")
        expr = app("lcExpressionRejected", [
            token("String", '"WORK_EXHAUSTED"'),
            lc.encode_json_value({"kind": "source", "start": "0", "end": "1"}),
            lc.encode_json_list(["0"]),
            token("Int", "36"),
        ])
        rejected = lc.decode_result(wire(expr, cells), packet)
        self.assertEqual(rejected, {"status": "Rejected", "code": "WORK_EXHAUSTED", "span": {"kind": "source", "start": "0", "end": "1"}, "nodePath": ["0"], "workUsed": "36"})
        kernel = app("lcKernelRejected", [token("String", '"TRANSFER_NOT_IN_STEP"'), app("lcActionIndexNull", [])])
        k = lc.decode_result(wire(kernel, cells), packet)
        self.assertEqual(k, {"status": "Rejected", "code": "TRANSFER_NOT_IN_STEP", "actionIndex": None})
        adapter = app("lcAdapterRejected", [token("String", '"WORK_MISMATCH"')])
        a = lc.decode_result(wire(adapter, cells), packet)
        self.assertEqual(a, {"status": "Rejected", "code": "WORK_MISMATCH"})

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_unknown_constructor_is_execution_boundary_not_rejected(self):
        packet = lc.admit_packet({"schema": "{}", "request": "{}", "financialPreState": "{}"})
        cells = {
            "<sigma>": lc.encode_text("{}", canonical=True),
            "<request>": lc.encode_text("{}", canonical=True),
            "<financial>": lc.encode_financial("{}"),
        }
        unknown = app("lcInvented", [token("String", '"DUPLICATE"')])
        with self.assertRaisesRegex(lc.CodecError, "K_OUTPUT"):
            lc.decode_result(wire(unknown, cells), packet)

    @unittest.skipIf(lc is None, "lifecycle_codec not yet implemented")
    def test_stuck_or_nonempty_continuation_is_execution_boundary(self):
        packet = lc.admit_packet({"schema": "{}", "request": "{}", "financialPreState": "{}"})
        pending = app("lcPending", [])
        cells = {
            "<sigma>": lc.encode_text("{}", canonical=True),
            "<request>": lc.encode_text("{}", canonical=True),
            "<financial>": lc.encode_financial("{}"),
        }
        with self.assertRaisesRegex(lc.CodecError, "K_OUTPUT"):
            lc.decode_result(wire(pending, cells), packet)
        raw = json.dumps({"format": "KAST", "version": 4, "term": app("<generatedTop>", [
            app("<k>", [app("lcStep", [])]),
            app("<out>", [app("lcAdapterRejected", [token("String", '"WORK_MISMATCH"')])]),
            app("<sigma>", [lc.encode_text("{}", canonical=True)]),
            app("<request>", [lc.encode_text("{}", canonical=True)]),
        ])})
        with self.assertRaisesRegex(lc.CodecError, "K_OUTPUT"):
            lc.decode_result(raw, packet)


if __name__ == "__main__":
    unittest.main()

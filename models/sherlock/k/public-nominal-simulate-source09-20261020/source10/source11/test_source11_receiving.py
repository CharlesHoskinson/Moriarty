#!/usr/bin/env python3
"""Positive and hostile unit tests for Source11 NIWindowObservation K query and receiver.

UNRUN in designated author session per execution boundary; prepared for Root
execution after fresh independent security review.
"""

from __future__ import annotations

import copy
from pathlib import Path
import sys
import unittest

THIS_DIR = Path(__file__).resolve().parent
if str(THIS_DIR) not in sys.path:
    sys.path.insert(0, str(THIS_DIR))

import source11_receiving as s11  # noqa: E402

INPUTS_DIR = Path(
    "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/"
    "doctorwatson-stagenet-four-milestone-audit-20261020/"
    "k-next-public-nominal-state-event-source11/SAFE"
)

INPUT_PINS = {
    "map-queue": {
        "path": str(INPUTS_DIR / "map-queue.json"),
        "bytes": 15399,
        "sha256": "70cac9aa9b43dddd6c8361ed909a04a22b9778d816ee20f7b874d3a14a59491d",
    },
    "mirror-arbor": {
        "path": str(INPUTS_DIR / "mirror-arbor.json"),
        "bytes": 17236,
        "sha256": "fda25dc7346264988cb9189246aa446ce2261c57f7435c00ff433e2b477d8052",
    },
}


def load_input(case_name: str) -> dict:
    raw = s11.checked(INPUT_PINS[case_name])
    return s11.loads(raw)


class Source11ReceivingSecurityTests(unittest.TestCase):
    def test_01_registry_211_validation_and_split(self) -> None:
        self.assertEqual(len(s11.ALL_SYMBOLS), 211)
        self.assertEqual(len(s11.DATA_REG), 134)
        self.assertEqual(len(s11.QUERY_REG), 135)
        self.assertIn("niRunWindow", s11.QUERY_REG)
        self.assertNotIn("niRunWindow", s11.DATA_REG)
        self.assertIn("niWindowObserved", s11.DATA_REG)
        self.assertIn("niWindowObserved", s11.QUERY_REG)
        self.assertIn("niRun", s11.ALL_SYMBOLS)
        self.assertNotIn("niRun", s11.DATA_REG)
        self.assertNotIn("niRun", s11.QUERY_REG)

        for old_law in (
            "niLawEventOrderRetained",
            "niLawRawPresence",
            "niLawSavedStateRetained",
        ):
            self.assertNotIn(old_law, s11.ALL_SYMBOLS)

        function_names = [
            name
            for name, meta in s11.ALL_SYMBOLS.items()
            if meta["kind"] == "function"
        ]
        self.assertEqual(len(function_names), 77)
        for fn_name in function_names:
            self.assertNotIn(fn_name, s11.DATA_REG)
            if fn_name != "niRunWindow":
                self.assertNotIn(fn_name, s11.QUERY_REG)

        self.assertEqual(len(s11.EXPECTED_NEW_SURFACE_NAMES), 32)
        new_ctors = []
        new_fns = []
        for name in s11.EXPECTED_NEW_SURFACE_NAMES:
            self.assertIn(name, s11.ALL_SYMBOLS)
            self.assertIn(
                s11.ALL_SYMBOLS[name]["declaring_module"],
                (
                    "SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS",
                    "SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11",
                    "SHERLOCK-PROGRAMVIEW-STATE-EVENT-SOURCE11-LAWS",
                ),
            )
            if s11.ALL_SYMBOLS[name]["kind"] == "constructor":
                new_ctors.append(name)
            else:
                new_fns.append(name)
        self.assertEqual(
            new_ctors,
            ["ngNoStates", "ngStateRow", "ngTape", "niWindowObserved"],
        )
        self.assertEqual(len(new_fns), 28)

        binding = s11.loads(s11.checked(s11.NEXT_RECEIVER_BINDING_PIN))
        self.assertEqual(
            s11.ALL_SYMBOLS["niRunWindow"]["compiled_klabel"],
            binding["root"]["compiled_klabel"],
        )
        self.assertEqual(
            s11.ALL_SYMBOLS["niRunWindow"]["raw_kore_label"],
            binding["root"]["raw_kore_label"],
        )
        self.assertEqual(
            s11.ALL_SYMBOLS["niRunWindow"]["definition_line"],
            binding["root"]["definition_line"],
        )
        self.assertEqual(
            s11.ALL_SYMBOLS["niWindowObserved"]["compiled_klabel"],
            binding["dataRoot"]["compiled_klabel"],
        )
        self.assertEqual(
            s11.ALL_SYMBOLS["niWindowObserved"]["raw_kore_label"],
            binding["dataRoot"]["raw_kore_label"],
        )
        self.assertEqual(
            s11.ALL_SYMBOLS["niWindowObserved"]["definition_line"],
            binding["dataRoot"]["definition_line"],
        )

        raw_reg = s11.loads(s11.checked(s11.ACTUAL_211_REGISTRY_PIN))
        def_lines = (
            s11.checked(s11.DEFINITION_KORE_PIN).decode("utf-8").splitlines()
        )
        validated = s11.validate_receiving_registry(
            raw_reg, definition_lines=def_lines
        )
        self.assertEqual(len(validated["all_symbols"]), 211)

        bad_count = copy.deepcopy(raw_reg)
        bad_count["count"] = 210
        with self.assertRaises(s11.ObservationFormatError):
            s11.validate_receiving_registry(bad_count, definition_lines=def_lines)

        bad_new_names = copy.deepcopy(raw_reg)
        bad_new_names["new_surface_names"] = list(
            s11.EXPECTED_NEW_SURFACE_NAMES[:-1]
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.validate_receiving_registry(bad_new_names, definition_lines=def_lines)

        bad_label = copy.deepcopy(raw_reg)
        bad_label["rows"][0][4] = "ngAdd(_,_,_)_WRONG_MODULE_NGExpr_Int_Int_Int"
        with self.assertRaises(s11.ObservationFormatError):
            s11.validate_receiving_registry(bad_label, definition_lines=def_lines)

        bad_line = copy.deepcopy(raw_reg)
        bad_line["rows"][0][9] += 1
        with self.assertRaises(s11.ObservationFormatError):
            s11.validate_receiving_registry(bad_line, definition_lines=def_lines)

        bad_kind = copy.deepcopy(raw_reg)
        bad_kind["rows"][0][3] = "function"
        with self.assertRaises(s11.ObservationFormatError):
            s11.validate_receiving_registry(bad_kind, definition_lines=def_lines)

    def test_02_render_source11_map_queue_and_mirror_arbor_roundtrips(self) -> None:
        for case_name, expected_type_ids, expected_type_arg_kinds in (
            ("map-queue", [4, 1], ["Nominal", "Bool"]),
            ("mirror-arbor", [4, 0], ["Nominal", "UInt"]),
        ):
            doc = load_input(case_name)
            saved = copy.deepcopy(doc)
            rendered = s11.render_source11(doc, 4)
            self.assertTrue(s11.same(doc, saved))
            self.assertEqual(rendered["schema"], s11.RENDER_SCHEMA)
            self.assertEqual(rendered["observation_cut"], 4)
            self.assertTrue(rendered["inverse_verified"])
            self.assertEqual(rendered["model_steps_evaluated"], 0)
            self.assertEqual(rendered["expected_documents_opened"], 0)
            self.assertEqual(
                [
                    x["runtime_type_id"]
                    for x in rendered["ported"]["type_rows"][-1]["type"]["arguments"]
                ],
                expected_type_ids,
            )
            self.assertEqual(
                [
                    arg["type"]["kind"]
                    for arg in rendered["ported"]["type_rows"][-1]["type"]["arguments"]
                ],
                expected_type_arg_kinds,
            )

            kast = rendered["kast_jsonv4"]
            self.assertEqual(kast["format"], "KAST")
            self.assertEqual(kast["version"], 4)
            self.assertEqual(kast["term"]["node"], "KApply")
            self.assertEqual(
                kast["term"]["label"]["name"],
                s11.LABELS["niRunWindow"],
            )
            self.assertEqual(kast["term"]["arity"], 3)

            query_term = s11.from_source11_ast(
                rendered["query_ast"], allow_root_query=True
            )
            self.assertEqual(query_term.sort, "NIWindowObservation")
            self.assertEqual(query_term.name, "niRunWindow")
            self.assertEqual(len(query_term.args), 3)
            self.assertEqual(
                s11.decode_query_document(kast, "NIWindowObservation"),
                query_term,
            )
            self.assertEqual(
                s11.parse_printed(
                    rendered["printed_query"],
                    "NIWindowObservation",
                    registry=s11.QUERY_REG,
                ),
                query_term,
            )
            self.assertTrue(
                s11.same(s11.WIRE.decode(query_term.args[0]), doc)
            )
            self.assertTrue(
                s11.same(
                    s11.WIRE.decode(query_term.args[1]),
                    rendered["ported"]["ng_program"],
                )
            )
            self.assertTrue(s11.same(s11.WIRE.decode(query_term.args[2]), 4))

        arbor_rendered = s11.render_source11(load_input("mirror-arbor"), 0)
        self.assertEqual(arbor_rendered["observation_cut"], 0)
        arbor_term = s11.from_source11_ast(
            arbor_rendered["query_ast"], allow_root_query=True
        )
        arbor_prog = s11.WIRE.decode(arbor_term.args[1])
        self.assertEqual(
            arbor_prog["expressions"][12],
            {"id": 12, "occurrence": 12, "kind": {"Not": {"operand": 13}}},
        )
        self.assertEqual(
            arbor_rendered["ported"]["ctor_rows"][5]["fields"],
            ["z_left", "a_right"],
        )

    def test_03_nijson_types_some_zero_null_empty_and_unicode(self) -> None:
        sample = {
            "null_field": None,
            "zero_int": 0,
            "zero_str": "0",
            "empty_str": "",
            "bool_true": True,
            "bool_false": False,
            "negative_int": -42,
            "empty_list": [],
            "empty_obj": {},
            "unicode": "αβ\n\"\\t",
            "slice": {"start": "5", "len": "0", "decl": "0", "none": None},
            "items": [None, False, True, 0, "0", [], {}],
        }
        ast_node = s11._port10.json_ast(sample)
        term = s11.from_source11_ast(ast_node)
        self.assertEqual(term.sort, "NIJSON")
        printed = s11.render_term(term)
        reparsed = s11.parse_printed(printed, "NIJSON")
        self.assertEqual(reparsed, term)
        kast_doc = s11.envelope(term)
        self.assertEqual(s11.decode_document(kast_doc, "NIJSON"), term)
        decoded = s11.WIRE.decode(term)
        self.assertTrue(s11.same(decoded, sample))
        self.assertFalse(s11.same(0, False))
        self.assertFalse(s11.same(1, True))
        self.assertFalse(s11.same("0", 0))
        self.assertFalse(s11.same(None, 0))

    def test_04_np_types_values_and_declared_field_order(self) -> None:
        printed_np = (
            'npNominal('
            'npNominalType("Stamp",npType(npNominalType("Arbor",npType(npUIntType(),npNoTypes())),npType(npBoolType(),npNoTypes()))),'
            '"Seal",'
            'npField("z_left",npUInt(0),npField("a_right",npBool(false),npNoFields()))'
            ')'
        )
        term = s11.parse_printed(printed_np, "NPValue")
        self.assertEqual(s11.render_term(term), printed_np)
        decoded = s11.WIRE.decode(term)
        self.assertEqual(
            decoded,
            {
                "Nominal": {
                    "type": {
                        "kind": "Nominal",
                        "name": "Stamp",
                        "arguments": [
                            {
                                "kind": "Nominal",
                                "name": "Arbor",
                                "arguments": [{"kind": "UInt"}],
                            },
                            {"kind": "Bool"},
                        ],
                    },
                    "constructor": "Seal",
                    "fields": [
                        {"name": "z_left", "value": {"UInt": 0}},
                        {"name": "a_right", "value": {"Bool": False}},
                    ],
                }
            },
        )

        dup_fields = (
            'npNominal(npUIntType(),"Bad",'
            'npField("dup",npUInt(1),npField("dup",npUInt(2),npNoFields())))'
        )
        dup_term = s11.parse_printed(dup_fields, "NPValue")
        with self.assertRaises(s11.ObservationFormatError):
            s11.WIRE.decode(dup_term)

    def test_05_decode_source11_synthetic_window_tape_and_altered_noninitial_visibility(
        self,
    ) -> None:
        doc = load_input("mirror-arbor")
        rendered = s11.render_source11(doc, 4)
        query_term = s11.from_source11_ast(
            rendered["query_ast"], allow_root_query=True
        )
        source_printed = s11.render_term(query_term.args[0])
        prog_printed = s11.render_term(query_term.args[1])

        # Public synthetic DATA-only states exercising initial + two chronological
        # after-states + duplicated final with distinct live/saved maps and events.
        initial_state_printed = (
            f"ngState({prog_printed},"
            "ngEval(0),"
            ".Map,"
            "ngNoFrames(),"
            "ngNoEvents())"
        )
        after1_state_printed = (
            f"ngState({prog_printed},"
            "ngEval(12),"
            "(.Map (11 |-> ngBool(true))),"
            "ngFrame(ngArmScope(9,9,0,(.Map (7 |-> ngUInt(2)))),ngNoFrames()),"
            "ngEvent(ngNotStarted(12,12,13,(.Map (11 |-> ngBool(true)))),ngNoEvents()))"
        )
        after2_state_printed = (
            f"ngState({prog_printed},"
            "ngReturn(ngBool(false)),"
            "(.Map (11 |-> ngBool(false)) (12 |-> ngUInt(41))),"
            "ngFrame(ngNotArgument(12,12,(.Map (11 |-> ngBool(true)))),"
            "ngFrame(ngArmScope(9,9,0,(.Map (7 |-> ngUInt(2)))),ngNoFrames())),"
            "ngEvent(ngNotStarted(12,12,13,(.Map (11 |-> ngBool(true)))),"
            "ngEvent(ngNotReturned(12,12,ngBool(true),ngBool(false),(.Map (11 |-> ngBool(true)))),"
            "ngNoEvents())))"
        )
        final_state_printed = after2_state_printed

        tape_printed = (
            f"ngTape({initial_state_printed},"
            f"ngStateRow({after1_state_printed},ngStateRow({after2_state_printed},ngNoStates())),"
            f"{final_state_printed})"
        )
        raw_kprint = (
            "<generatedTop>\n  <k>\n    "
            f"niWindowObserved({source_printed},{tape_printed})"
            " ~> .K\n  </k>\n</generatedTop>\n"
        )

        decoded = s11.decode_source11(doc, raw_kprint)
        self.assertEqual(decoded["schema"], s11.DECODE_SCHEMA)
        self.assertTrue(decoded["source_identity_verified"])
        self.assertTrue(s11.same(decoded["source_document"], doc))
        self.assertEqual(decoded["raw_typed_ast"]["sort"], "NIWindowObservation")
        self.assertEqual(decoded["raw_typed_ast"]["constructor"], "niWindowObserved")

        tape = decoded["tape"]
        self.assertEqual(set(tape), {"initial", "after_each_event", "final"})
        self.assertEqual(len(tape["after_each_event"]), 2)

        s0 = tape["initial"]
        s1 = tape["after_each_event"][0]
        s2 = tape["after_each_event"][1]
        sf = tape["final"]

        for state_item in (s0, s1, s2, sf):
            self.assertTrue(
                s11.same(state_item["program"], rendered["ported"]["ng_program"])
            )

        self.assertEqual(s0["control"], {"ngEval": {"expr_id": 0}})
        self.assertEqual(s0["environment"], [])
        self.assertEqual(s0["frames_top_first"], [])
        self.assertEqual(s0["events"], [])

        self.assertEqual(s1["control"], {"ngEval": {"expr_id": 12}})
        self.assertEqual(
            s1["environment"],
            [{"name_id": 11, "value": {"Bool": True}}],
        )
        self.assertEqual(
            s1["frames_top_first"],
            [
                {
                    "ngArmScope": {
                        "match_expr_id": 9,
                        "match_occurrence": 9,
                        "source_arm_id": 0,
                        "saved_environment": [
                            {"name_id": 7, "value": {"UInt": 2}}
                        ],
                    }
                }
            ],
        )
        self.assertEqual([next(iter(ev)) for ev in s1["events"]], ["ngNotStarted"])

        self.assertEqual(s2["control"], {"ngReturn": {"value": {"Bool": False}}})
        self.assertEqual(
            s2["environment"],
            [
                {"name_id": 11, "value": {"Bool": False}},
                {"name_id": 12, "value": {"UInt": 41}},
            ],
        )
        self.assertEqual(
            s2["frames_top_first"],
            [
                {
                    "ngNotArgument": {
                        "expr_id": 12,
                        "occurrence": 12,
                        "saved_environment": [
                            {"name_id": 11, "value": {"Bool": True}}
                        ],
                    }
                },
                {
                    "ngArmScope": {
                        "match_expr_id": 9,
                        "match_occurrence": 9,
                        "source_arm_id": 0,
                        "saved_environment": [
                            {"name_id": 7, "value": {"UInt": 2}}
                        ],
                    }
                },
            ],
        )
        self.assertEqual(
            [next(iter(ev)) for ev in s2["events"]],
            ["ngNotStarted", "ngNotReturned"],
        )
        self.assertTrue(s11.same(sf, s2))

        # Altered non-initial state (after_each_event[0]) must remain visible.
        altered_after1_printed = after1_state_printed.replace(
            "7 |-> ngUInt(2)", "7 |-> ngUInt(99)"
        )
        altered_tape_printed = (
            f"ngTape({initial_state_printed},"
            f"ngStateRow({altered_after1_printed},ngStateRow({after2_state_printed},ngNoStates())),"
            f"{final_state_printed})"
        )
        altered_kprint = f"<k> niWindowObserved({source_printed},{altered_tape_printed}) ~> .K </k>"
        decoded_altered = s11.decode_source11(doc, altered_kprint)
        self.assertFalse(s11.same(decoded_altered["tape"], decoded["tape"]))
        self.assertFalse(
            s11.same(
                decoded_altered["tape"]["after_each_event"][0],
                decoded["tape"]["after_each_event"][0],
            )
        )
        self.assertEqual(
            decoded_altered["tape"]["after_each_event"][0]["frames_top_first"][0][
                "ngArmScope"
            ]["saved_environment"],
            [{"name_id": 7, "value": {"UInt": 99}}],
        )
        self.assertTrue(
            s11.same(decoded_altered["tape"]["initial"], decoded["tape"]["initial"])
        )
        self.assertTrue(
            s11.same(
                decoded_altered["tape"]["after_each_event"][1],
                decoded["tape"]["after_each_event"][1],
            )
        )

        # Altered second after-state (after_each_event[1]) must also remain visible.
        altered_after2_printed = after2_state_printed.replace(
            "12 |-> ngUInt(41)", "12 |-> ngUInt(42)"
        )
        altered_tape2_printed = (
            f"ngTape({initial_state_printed},"
            f"ngStateRow({after1_state_printed},ngStateRow({altered_after2_printed},ngNoStates())),"
            f"{final_state_printed})"
        )
        decoded_altered2 = s11.decode_source11(
            doc,
            f"<k> niWindowObserved({source_printed},{altered_tape2_printed}) ~> .K </k>",
        )
        self.assertFalse(
            s11.same(
                decoded_altered2["tape"]["after_each_event"][1],
                decoded["tape"]["after_each_event"][1],
            )
        )

        # Tampering with bound source document must be rejected.
        tampered = copy.deepcopy(doc)
        tampered["source"]["name"] = "DifferentName"
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_source11(tampered, raw_kprint)

    def test_06_reject_residual_functions_and_unreduced_k_cells(self) -> None:
        doc = load_input("map-queue")
        rendered = s11.render_source11(doc, 4)
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_source11(doc, f"<k> {rendered['printed_query']} ~> .K </k>")

        query_term = s11.from_source11_ast(
            rendered["query_ast"], allow_root_query=True
        )
        source_printed = s11.render_term(query_term.args[0])
        prog_printed = s11.render_term(query_term.args[1])
        residual_window = (
            f"<k> niWindowObserved({source_printed},"
            f"ngObserve(ngInitial({prog_printed},.Map),4)) ~> .K </k>"
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_source11(doc, residual_window)

        for residual in (
            rendered["printed_query"],
            "ngObserve(ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngEval(0),.Map,ngNoFrames(),ngNoEvents()),4)",
            "ngObserveLoop(ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngEval(0),.Map,ngNoFrames(),ngNoEvents()),ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngEval(0),.Map,ngNoFrames(),ngNoEvents()),ngNoStates(),4)",
            "ngReverseStates(ngNoStates(),ngNoStates())",
            "nwLawCompleteFourSteps()",
            "nwExpectedTape(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0))",
            "niRun(niNull(),ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),npNoTypeRows(),npNoCtorRows(),4)",
            "niSource(niObserved(niNull(),npObserved(ngState(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),ngHalted(ngBool(true)),.Map,ngNoFrames(),ngNoEvents()),npBool(true))))",
            "npRunReference(ngProgram(ngNoDecls(),ngNoNodes(),ngNoFunctions(),0),npNoTypeRows(),npNoCtorRows(),1)",
        ):
            with self.assertRaises(s11.ObservationFormatError):
                s11.parse_printed(residual, registry=s11.DATA_REG)

        for bad_frame in (
            "",
            "<k> niNull()",
            "niNull() </k>",
            "<generatedTop> niNull() </generatedTop>",
            "<generatedTop> <k> niNull() </k> <extra> 1 </extra> </generatedTop>",
            "<k> ~> .K </k>",
        ):
            with self.assertRaises(s11.ObservationFormatError):
                s11.complete_term(bad_frame)

        with self.assertRaises(s11.ObservationFormatError):
            s11.parse_printed("niNull() ~> .K", "NIJSON")

    def test_07_reject_r2_value_ast_and_bare_or_parametric_kast_labels(self) -> None:
        with self.assertRaises(s11.ObservationFormatError):
            s11.from_source11_ast({"sort": "Int", "value": 1})

        with self.assertRaises(s11.ObservationFormatError):
            s11.from_source11_ast(
                {"sort": "Map", "entries": []}, allow_map=False
            )
        empty_map_term = s11.from_source11_ast(
            {"sort": "Map", "entries": []}, allow_map=True
        )
        self.assertEqual(s11.WIRE.decode(empty_map_term), [])

        valid_kast = s11.envelope(s11.make_term("niNull"))
        bare_kast = copy.deepcopy(valid_kast)
        bare_kast["term"]["label"]["name"] = "niNull"
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_document(bare_kast, "NIJSON")

        kore_kast = copy.deepcopy(valid_kast)
        kore_kast["term"]["label"]["name"] = s11.ALL_SYMBOLS["niNull"]["raw_kore_label"]
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_document(kore_kast, "NIJSON")

        param_kast = copy.deepcopy(valid_kast)
        param_kast["term"]["label"]["params"] = ["SortNIJSON"]
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_document(param_kast, "NIJSON")

        fn_kast = {
            "format": "KAST",
            "version": 4,
            "term": {
                "node": "KApply",
                "label": {
                    "node": "KLabel",
                    "name": s11.LABELS["nwLawCompleteFourSteps"],
                    "params": [],
                },
                "arity": 0,
                "args": [],
            },
        }
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_document(fn_kast, "Bool", allow_root_query=True)

        for bad_int_token in ("01", "+1", "-0", "1 ", "true"):
            bad_tok_doc = {
                "format": "KAST",
                "version": 4,
                "term": {
                    "node": "KToken",
                    "sort": {"node": "KSort", "name": "Int", "params": []},
                    "token": bad_int_token,
                },
            }
            with self.assertRaises(s11.ObservationFormatError):
                s11.decode_document(bad_tok_doc, "Int")

    def test_08_reject_bool_int_confusion_and_integer_ranges(self) -> None:
        with self.assertRaises(s11.ObservationFormatError):
            s11.from_source11_ast({"sort": "Int", "literal": True})
        with self.assertRaises(s11.ObservationFormatError):
            s11.from_source11_ast({"sort": "Bool", "literal": 1})

        doc = load_input("map-queue")
        for bad_cut in (-1, True, 1 << 64):
            with self.assertRaises(s11.ObservationFormatError):
                s11.render_source11(doc, bad_cut)  # type: ignore[arg-type]

        max_u128 = (1 << 128) - 1
        self.assertEqual(
            s11.WIRE.decode(s11.parse_printed(f"ngUInt({max_u128})", "NGValue")),
            {"UInt": max_u128},
        )
        self.assertEqual(
            s11.WIRE.decode(s11.parse_printed(f"npUInt({max_u128})", "NPValue")),
            {"UInt": max_u128},
        )
        for bad_u128 in (-1, 1 << 128):
            with self.assertRaises(s11.ObservationFormatError):
                s11.WIRE.decode(
                    s11.parse_printed(f"ngUInt({bad_u128})", "NGValue")
                )
            with self.assertRaises(s11.ObservationFormatError):
                s11.WIRE.decode(
                    s11.parse_printed(f"npUInt({bad_u128})", "NPValue")
                )

        for bad_idx in (-1, 1 << 64):
            with self.assertRaises(s11.ObservationFormatError):
                s11.WIRE.decode(
                    s11.parse_printed(f"ngEval({bad_idx})", "NGControl")
                )

        for bad_ni_int in (-(1 << 128), 1 << 128):
            with self.assertRaises(s11.ObservationFormatError):
                s11.parse_printed(f"niInt({bad_ni_int})", "NIJSON")

    def test_09_reject_duplicate_keys_and_enforced_decode_budgets(self) -> None:
        with self.assertRaises(s11.ObservationFormatError):
            s11.loads('{"dup": 1, "dup": 2}')
        with self.assertRaises(s11.ObservationFormatError):
            s11.loads('{"bad": NaN}')

        dup_nijson = s11.parse_printed(
            'niObject(niField("k",niNull(),niField("k",niBool(true),niNoFields())))',
            "NIJSON",
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.WIRE.decode(dup_nijson)

        with self.assertRaises(s11.ObservationFormatError):
            s11.parse_printed(
                "(.Map (4 |-> ngBool(true)) (4 |-> ngBool(false)))", "Map"
            )

        dup_type_rows = s11.parse_printed(
            "npTypeRow(0,npUIntType(),npTypeRow(0,npBoolType(),npNoTypeRows()))",
            "NPTypeRows",
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.WIRE.decode(dup_type_rows)

        dup_ctor_rows = s11.parse_printed(
            'npCtorRow(4,0,"A",npNoFieldNames(),npCtorRow(4,0,"B",npNoFieldNames(),npNoCtorRows()))',
            "NPCtorRows",
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.WIRE.decode(dup_ctor_rows)

        tiny_bytes = s11.DecodeBudget(max_bytes=8)
        with self.assertRaises(s11.ObservationFormatError):
            s11.loads('{"long_key": "value"}', budget=tiny_bytes)

        tiny_tokens = s11.DecodeBudget(max_tokens=4)
        with self.assertRaises(s11.ObservationFormatError):
            s11.parse_printed(
                "niArray(niItem(niNull(),niNoItems()))",
                "NIJSON",
                budget=tiny_tokens,
            )

        tiny_depth = s11.DecodeBudget(max_depth=3)
        deep_printed = (
            "niArray(niItem(niArray(niItem(niArray(niNoItems()),niNoItems())),niNoItems()))"
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.parse_printed(deep_printed, "NIJSON", budget=tiny_depth)

        tiny_rows = s11.DecodeBudget(max_rows=2)
        three_items = s11.parse_printed(
            "niArray(niItem(niNull(),niItem(niNull(),niItem(niNull(),niNoItems()))))",
            "NIJSON",
        )
        with self.assertRaises(s11.ObservationFormatError):
            s11.WIRE.decode(three_items, budget=tiny_rows)

        tiny_nodes = s11.DecodeBudget(max_nodes=3)
        with self.assertRaises(s11.ObservationFormatError):
            s11.WIRE.decode(three_items, budget=tiny_nodes)

    def test_10_reject_nested_nirunwindow_sort_arity_and_ng_duplicates(self) -> None:
        doc = load_input("map-queue")
        rendered = s11.render_source11(doc, 4)
        bad_query_ast = copy.deepcopy(rendered["query_ast"])
        bad_query_ast["arguments"][0] = copy.deepcopy(rendered["query_ast"])
        with self.assertRaises(s11.ObservationFormatError):
            s11.from_source11_ast(bad_query_ast, allow_root_query=True)

        old_nirun_ast = {
            "sort": "NIObservation",
            "constructor": "niRun",
            "arguments": [
                rendered["query_ast"]["arguments"][0],
                rendered["query_ast"]["arguments"][1],
                {"sort": "NPTypeRows", "constructor": "npNoTypeRows", "arguments": []},
                {"sort": "NPCtorRows", "constructor": "npNoCtorRows", "arguments": []},
                {"sort": "Int", "literal": 4},
            ],
        }
        with self.assertRaises(s11.ObservationFormatError):
            s11.from_source11_ast(old_nirun_ast, allow_root_query=True)

        null_term = s11.make_term("niNull")
        with self.assertRaises(s11.ObservationFormatError):
            s11.encode_query_envelope(null_term)
        with self.assertRaises(s11.ObservationFormatError):
            s11.decode_query_document(s11.envelope(null_term), "NIJSON")

        with self.assertRaises(s11.ObservationFormatError):
            s11.make_term("niBool")
        with self.assertRaises(s11.ObservationFormatError):
            s11.make_term("niBool", s11.atom("Int", 1))

        for sort_name, dup_printed in (
            (
                "NGDecls",
                "ngDecl(2,0,ngNoCtors(),ngDecl(2,0,ngNoCtors(),ngNoDecls()))",
            ),
            (
                "NGNodes",
                "ngNode(0,ngLiteral(0,ngBool(true)),ngNode(0,ngLiteral(1,ngBool(false)),ngNoNodes()))",
            ),
            (
                "NGParams",
                "ngParam(1,0,ngParam(1,1,ngNoParams()))",
            ),
            (
                "NGArms",
                "ngArm(0,ngWildcard(),1,ngArm(0,ngWildcard(),2,ngNoArms()))",
            ),
            (
                "NGBindings",
                "ngBinding(5,0,0,ngRoot(),ngUInt(1),ngBinding(5,0,0,ngRoot(),ngUInt(2),ngNoBindings()))",
            ),
        ):
            term = s11.parse_printed(dup_printed, sort_name)
            with self.assertRaises(s11.ObservationFormatError):
                s11.WIRE.decode(term)


if __name__ == "__main__":
    unittest.main()

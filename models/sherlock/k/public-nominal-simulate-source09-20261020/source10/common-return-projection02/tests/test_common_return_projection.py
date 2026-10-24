"""Synthetic model-conformance examples; no private oracle/capture input."""
import copy
import json
from pathlib import Path
import sys
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "source"))
from common_return_projection import ProjectionDomainError, project_return_pair


class CommonReturnProjectionTests(unittest.TestCase):
    def setUp(self):
        self.ported = json.loads((ROOT / "fixtures/PUBLIC-SYNTHETIC-BINDINGS.json").read_text())
        self.cases = json.loads((ROOT / "fixtures/INDEPENDENT-PRECODE-SYNTHETIC-EXPECTATIONS.json").read_text())["cases"]

    def case(self, name):
        return copy.deepcopy(next(c for c in self.cases if c["id"] == name))

    def project(self, case):
        return project_return_pair(case["np"], case["ng"], self.ported)

    def test_all_ten_precode_whole_returns(self):
        for case in self.cases:
            with self.subTest(case=case["id"]):
                self.assertEqual(self.project(case), case["expected"])

    def test_empty_variant_and_present_zero_are_distinct(self):
        absent = self.project(self.case("nominal-absent"))
        present = self.project(self.case("nominal-present-zero"))
        self.assertNotEqual(absent, present)
        self.assertEqual(present["fields"][0]["value"], "0")

    def test_same_names_do_not_replace_declaration_ids(self):
        six, seven = self.case("declaration-identity-six"), self.case("declaration-identity-seven")
        self.assertEqual(six["np"], seven["np"])
        self.assertNotEqual(self.project(six), self.project(seven))

    def test_order_is_not_alphabetical(self):
        case = self.case("nonalphabetical-equal-field-types")
        self.assertEqual([c["value"] for c in self.project(case)["fields"]], ["13", "29"])
        case["np"]["Nominal"]["fields"].reverse()
        with self.assertRaises(ProjectionDomainError):
            self.project(case)

    def test_np_ng_payload_disagreement_is_not_erased(self):
        case = self.case("uint-zero")
        case["np"]["UInt"] = 1
        with self.assertRaises(ProjectionDomainError):
            self.project(case)

    def test_declared_field_type_is_checked(self):
        case = self.case("nominal-present-zero")
        case["np"]["Nominal"]["fields"][0]["value"] = {"Bool": False}
        case["ng"]["Nominal"]["fields"][0] = {"Bool": False}
        with self.assertRaises(ProjectionDomainError):
            self.project(case)

    def test_full_generic_argument_order_is_checked(self):
        case = self.case("nested-generic-and-bool")
        case["np"]["Nominal"]["type"]["arguments"].reverse()
        with self.assertRaises(ProjectionDomainError):
            self.project(case)

    def test_source_constructor_owner_is_checked(self):
        case = self.case("declaration-identity-six")
        self.ported["normalized_program_view"]["constructors"][3]["owning_type"] = 7
        with self.assertRaises(ProjectionDomainError):
            self.project(case)

    def test_unsupported_value_constructor_is_explicit(self):
        with self.assertRaises(ProjectionDomainError):
            project_return_pair({"Unit": None}, {"Unit": None}, self.ported)

    def test_no_input_or_source_option_mutation(self):
        case = self.case("nested-generic-and-bool")
        original = copy.deepcopy((case, self.ported))
        self.project(case)
        self.assertEqual((case, self.ported), original)
        self.assertIsNone(self.ported["source_document"]["declaration_option"])
        self.assertEqual(self.ported["source_document"]["Some0"], 0)
        self.assertEqual(self.ported["source_document"]["raw_empty_slice"], {"start": 77, "len": 0})

    def test_argument_runtime_identity_link_is_checked(self):
        case = self.case("nominal-absent")
        self.ported["type_rows"][2]["type"]["arguments"][0]["runtime_type_id"] = 1
        with self.assertRaises(ProjectionDomainError):
            self.project(case)

    def test_bool_is_not_a_uint_payload(self):
        with self.assertRaises(ProjectionDomainError):
            project_return_pair({"UInt": False}, {"UInt": False}, self.ported)


if __name__ == "__main__":
    unittest.main()

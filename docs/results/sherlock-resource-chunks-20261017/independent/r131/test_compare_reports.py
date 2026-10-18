#!/usr/bin/env python3
"""Pure normal synthetic data checks, never an actual producer observation."""
import copy
import json
import unittest
from compare_reports import REPORTS, Difference, compare, exact

class NormalSyntheticData(unittest.TestCase):
    def setUp(self):
        self.gold = {name: json.loads(path.read_text()) for name, path in REPORTS.items()}

    def test_all_four_full_records_accept(self):
        for name, record in self.gold.items():
            with self.subTest(report=name):
                compare(name, copy.deepcopy(record))

    def test_same_cumulative_record_and_additive_columns(self):
        self.assertEqual(self.gold["whole"], self.gold["cumulative"])
        for field in ("reference_work", "recorded_reference_events"):
            self.assertEqual(int(self.gold["whole"][field]),
                int(self.gold["first"][field]) + int(self.gold["second"][field]))
        self.assertEqual(self.gold["whole"]["maximum_observed_logical_depth"],
            max(self.gold["first"]["maximum_observed_logical_depth"],
                self.gold["second"]["maximum_observed_logical_depth"]))

    def test_normal_changed_selected_fields_fail(self):
        edits = [
            (("reference_work",), "2"),
            (("recorded_reference_events",), "1"),
            (("maximum_observed_logical_depth",), 1),
            (("format",), "moriarty-sherlock-a-resource-report/2"),
            (("manifest", "format"), "moriarty-sherlock-resource-manifest/2"),
            (("manifest", "reference_schedule_id"), "sherlock-k-a/2"),
            (("manifest", "source_definition_sha256"), "different-source"),
            (("manifest", "profile_id"), 1),
            (("manifest", "layout_version"), ["1", "0"]),
            (("manifest", "bounds"), {}),
            (("manifest", "measurement_scope"), "whole-run"),
            (("manifest", "unavailable_metadata", "status"), "measured"),
            (("helper_work", "status"), "measured"),
            (("helper_visits", "reason"), "counted"),
        ]
        # Every separate unmeasured group is selected, including its reason.
        for group in self.gold["whole"]["unmeasured"]:
            edits.append((("unmeasured", group, "reason"), "different-reason"))
        for path, replacement in edits:
            with self.subTest(path=path):
                candidate = copy.deepcopy(self.gold["whole"])
                parent = candidate
                for key in path[:-1]:
                    parent = parent[key]
                parent[path[-1]] = replacement
                with self.assertRaises(Difference):
                    compare("whole", candidate)

    def test_types_presence_order_and_metric_multiplicity(self):
        candidates = []
        c = copy.deepcopy(self.gold["whole"]); c["reference_work"] = 1; candidates.append(c)
        c = copy.deepcopy(self.gold["whole"]); c["maximum_observed_logical_depth"] = False; candidates.append(c)
        c = copy.deepcopy(self.gold["whole"]); del c["manifest"]["bounds"]; candidates.append(c)
        c = copy.deepcopy(self.gold["whole"]); c["new_total"] = "1"; candidates.append(c)
        c = copy.deepcopy(self.gold["whole"]); c["unmeasured"]["physical"]["metrics"].reverse(); candidates.append(c)
        c = copy.deepcopy(self.gold["whole"]); c["unmeasured"]["overhead"]["metrics"].pop(); candidates.append(c)
        c = copy.deepcopy(self.gold["whole"]); c["unmeasured"]["proof"]["metrics"].append("key-bytes"); candidates.append(c)
        for index, candidate in enumerate(candidates):
            with self.subTest(edit=index):
                with self.assertRaises(Difference):
                    compare("whole", candidate)

    def test_free_publication_chunk_distinguished(self):
        for left, right in (("first", "second"), ("whole", "first"), ("whole", "second")):
            with self.subTest(left=left, right=right):
                with self.assertRaises(Difference):
                    compare(left, self.gold[right])

    def test_lossless_decimal_handling_does_not_use_float(self):
        # Comparator utility only: these are not invented resource observations.
        max_u128 = str((1 << 128) - 1)
        exact({"counter": max_u128}, {"counter": max_u128})
        with self.assertRaises(Difference):
            exact({"counter": max_u128}, {"counter": str((1 << 128) - 2)})
        with self.assertRaises(Difference):
            exact({"counter": max_u128}, {"counter": int(max_u128)})

if __name__ == "__main__":
    unittest.main(verbosity=2)

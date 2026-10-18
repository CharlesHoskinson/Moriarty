"""Independent synthetic reporting expectations; never product/ledger evidence.

Only standard-library temporary files and the reader's public CLI are used.
Expectations come from the October 17 implementation plan, not reader internals.
"""

import copy
import hashlib
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import unittest


CHECKER = pathlib.Path(__file__).resolve().with_name("check.py")
PROFILE = "synthetic-core/1"
TARGET = "synthetic-local"
BASELINE = "docs/utxo/doctorwatson/EARS-BASELINE.json"
EVIDENCE = "openspec/changes/utxo-doctorwatson/current-evidence.json"


def digest(data):
    return hashlib.sha256(data).hexdigest()


class ReportingExpectations(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="doctorwatson-report-fixture-")
        self.addCleanup(self.temp.cleanup)
        self.repo = pathlib.Path(self.temp.name)
        requirements = []
        self.rows = []
        for index, requirement_id in enumerate(("R-001", "R-002", "R-003", None)):
            heading = f"{requirement_id or 'Retained clause'} Fixture {index}"
            names = [f"Positive fixture {index}", f"Negative fixture {index}"]
            source_path = f"specs/requirement-{index}.md"
            markdown = (
                f"# Synthetic normative fixture {index}\n\n"
                f"### Requirement: {heading}\n\n"
                "WHEN a synthetic declaration is assessed, the reader SHALL retain its names.\n\n"
                f"#### Scenario: {names[0]}\n\n- **WHEN** the synthetic source is current\n"
                "- **THEN** the report retains that observation\n\n"
                f"#### Scenario: {names[1]}\n\n- **WHEN** the source differs\n"
                "- **THEN** the report records a gap\n"
            )
            self.write(source_path, markdown)
            source = {"path": source_path, "sha256": digest(markdown.encode())}
            key = source_path + "#" + heading
            requirements.append({
                "key": key, "id": requirement_id, "heading": heading,
                "source": copy.deepcopy(source),
                "clause": "WHEN assessed, exact synthetic names SHALL remain bound.",
                "scenarios": [{"name": name, "text": "Synthetic reporting expectation only."} for name in names],
                "originalScope": {"profile": PROFILE, "target": TARGET},
            })
            self.rows.append({
                "key": key, "id": requirement_id, "group": "synthetic",
                "source": copy.deepcopy(source), "heading": heading, "scenarioNames": names,
                "applicability": {"status": "unassessed", "profile": PROFILE, "target": TARGET,
                                  "reason": "Synthetic fixture; no product result."},
                "implementation": {"status": "unassessed", "detail": "No implementation asserted."},
                "validation": {"status": "unrun", "evidenceRefs": []},
                "acceptance": {"status": "open", "scope": "Synthetic reporting only.",
                               "reason": "No acceptance asserted.", "reviewRefs": []},
                "scenarioRecords": [{"name": name, "status": "unrun",
                                     "expectation": "Retain this synthetic reporting predicate.",
                                     "evidenceRefs": []} for name in names],
                "sourcePins": [copy.deepcopy(source)], "dependsOn": [],
                "nextAction": {"consumer": "synthetic-reader-consumer", "expected": "Explicit observed declaration.",
                               "owner": "fixture-owner", "blockedBy": [], "readiness": "ready"},
            })
        self.baseline = {
            "format": "doctorwatson-utxo-design-baseline/1",
            "groups": [{"group": "synthetic", "package": "synthetic-only", "requirements": requirements}],
            "families": [{"id": f"family-{i:02d}", "name": f"Synthetic family {i}"} for i in range(20)],
        }
        self.evidence = {
            "format": "doctorwatson-utxo.evidence/1", "rows": self.rows,
            "families": [dict(family, source="unassessed", admission="unassessed", compiled="unrun",
                              native="unrun", private="unrun") for family in self.baseline["families"]],
        }

    def write(self, relative_path, content):
        path = self.repo / relative_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        return path

    def ref(self, relative_path, **extra):
        self.write(relative_path, "Synthetic reporting fixture. No ledger/product evidence.\n")
        return dict(path=relative_path, sha256=digest((self.repo / relative_path).read_bytes()),
                    profile=PROFILE, target=TARGET, **extra)

    def close(self, index=0):
        row = self.rows[index]
        row["applicability"].update(status="applicable", profile=PROFILE, target=TARGET)
        row["implementation"] = {"status": "complete", "detail": "Synthetic declaration only."}
        evidence = self.ref(f"public/observations-{index}.txt")
        row["validation"] = {"status": "observed", "evidenceRefs": [evidence]}
        for record in row["scenarioRecords"]:
            record["status"] = "observed"
            record["evidenceRefs"] = [copy.deepcopy(evidence)]
        row["acceptance"] = {
            "status": "closed", "profile": PROFILE, "target": TARGET,
            "scope": "Synthetic declaration; no semantic authentication.", "reason": "Fixture only.",
            "reviewRefs": [self.ref(f"public/review-{index}-{role}.txt", role=role,
                                    independent=True, author="fixture-author", reviewer="fixture-reviewer")
                           for role in ("independent-source", "actual-result")],
        }
        return row

    def invoke(self, *args, expected=0):
        self.assertTrue(CHECKER.is_file(), "doctorwatson-utxo reporting entrypoint is not implemented")
        self.write(BASELINE, json.dumps(self.baseline))
        self.write(EVIDENCE, json.dumps(self.evidence))
        result = subprocess.run(
            [sys.executable, str(CHECKER), "--repo", str(self.repo), "--json", *args],
            capture_output=True, text=True, timeout=10,
        )
        self.assertEqual(expected, result.returncode, f"stdout={result.stdout}\nstderr={result.stderr}")
        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError as error:
            self.fail(f"--json must produce a JSON report: {error}; stdout={result.stdout}; stderr={result.stderr}")

    def selected(self, report, index=0):
        return next(row for row in report["requirements"] if row["key"] == self.rows[index]["key"])

    def test_entrypoint_exists(self):
        self.assertTrue(CHECKER.is_file(), "doctorwatson-utxo reporting entrypoint is not implemented")

    def test_valid_all_open_inventory_has_integrity_without_closure(self):
        report = self.invoke()
        self.assertEqual("ok", report["integrity"]["status"])
        self.assertEqual(4, len(report["requirements"]))
        self.assertEqual(20, len(report["families"]))
        self.assertEqual({row["key"] for row in self.rows}, {row["key"] for row in report["requirements"]})
        for row in report["requirements"]:
            self.assertEqual("open", row["gate"])
            self.assertEqual("current", row["currentness"])
            self.assertEqual("unassessed", row["implementation"]["status"])
            self.assertEqual("unrun", row["observations"]["status"])
            self.assertEqual("open", row["acceptance"]["status"])

    def test_open_gate_fails_while_exact_requirement_inspection_succeeds(self):
        self.invoke("--gate", "R-001", expected=1)
        report = self.invoke("--requirement", "R-001")
        self.assertEqual([self.rows[0]["key"]], [row["key"] for row in report["requirements"]])

    def test_requirement_inspection_retains_complete_clause_and_scenario_declarations(self):
        requirement = self.baseline["groups"][0]["requirements"][0]
        requirement["clause"] = (
            "WHEN a synthetic declaration is assessed, the reader SHALL retain the complete clause.\n\n"
            "Trace: synthetic rule A and synthetic rule B.\n\n"
            "Acceptance: synthetic clause detail remains an open reporting obligation."
        )
        row = self.rows[0]
        row["scenarioRecords"][1]["expectation"] = "Retain this exact named missing synthetic observation."
        row["scenarioRecords"][1]["gap"] = "Synthetic observation has not been run."
        report = self.invoke("--requirement", "R-001")
        selected = report["requirements"][0]
        self.assertIn("clause", selected, "Requirement inspection must expose its complete normative clause")
        self.assertEqual(requirement["clause"], selected["clause"])
        for field in ("source", "heading", "scenarioNames", "scenarioRecords", "applicability", "nextAction"):
            with self.subTest(field=field):
                self.assertIn(field, selected)
                self.assertEqual(row[field], selected[field])

    def test_null_original_id_is_preserved_and_selectable_by_key(self):
        report = self.invoke("--requirement", self.rows[3]["key"])
        self.assertEqual(1, len(report["requirements"]))
        self.assertIsNone(report["requirements"][0]["id"])

    def test_multiple_nullable_ids_are_distinct_by_source_and_heading(self):
        self.rows[2]["id"] = None
        self.baseline["groups"][0]["requirements"][2]["id"] = None
        report = self.invoke()
        self.assertEqual(2, sum(row["id"] is None for row in report["requirements"]))

    def test_gate_selection_does_not_match_substrings(self):
        self.close(0)
        self.invoke("--gate", "R-001")
        for selector in ("R-00", "missing-id"):
            report = self.invoke("--gate", selector, expected=1)
            self.assertEqual("failed", report["integrity"]["status"])
        self.invoke("--gate", "R-002", expected=1)

    def test_complete_current_direct_declarations_can_close_only_selected_gate(self):
        self.close(0)
        report = self.invoke("--gate", self.rows[0]["key"])
        self.assertEqual(1, len(report["requirements"]))
        self.assertEqual("closed", report["requirements"][0]["gate"])
        self.assertEqual("complete", report["requirements"][0]["implementation"]["status"])
        self.assertEqual("observed", report["requirements"][0]["observations"]["status"])
        self.assertEqual("closed", report["requirements"][0]["acceptance"]["status"])

    def test_missing_membership_at_same_total_is_rejected(self):
        self.rows[-1] = copy.deepcopy(self.rows[0])
        self.evidence["rows"] = self.rows
        self.invoke(expected=1)

    def test_same_count_unknown_requirement_replacement_is_rejected(self):
        self.rows[2]["key"] = "specs/invented.md#Invented"
        self.invoke(expected=1)

    def test_duplicate_nonnull_requirement_id_is_rejected(self):
        self.rows[1]["id"] = self.rows[0]["id"]
        self.invoke(expected=1)

    def test_baseline_duplicate_nonnull_id_cannot_redefine_membership(self):
        self.baseline["groups"][0]["requirements"][1]["id"] = "R-001"
        self.rows[1]["id"] = "R-001"
        self.invoke(expected=1)

    def test_baseline_and_evidence_cannot_relabel_the_actual_normative_id(self):
        self.baseline["groups"][0]["requirements"][0]["id"] = "Q-999"
        self.rows[0]["id"] = "Q-999"
        self.invoke(expected=1)

    def test_missing_row_is_rejected(self):
        self.rows.pop()
        self.invoke(expected=1)

    def test_unknown_extra_row_is_rejected(self):
        row = copy.deepcopy(self.rows[0])
        row["key"] = "specs/extra.md#Extra"
        row["id"] = "EXTRA"
        self.rows.append(row)
        self.invoke(expected=1)

    def test_row_group_must_equal_baseline_group(self):
        self.rows[0]["group"] = "another-group"
        self.invoke(expected=1)

    def test_row_heading_cannot_change_its_stable_key_binding(self):
        self.rows[0]["heading"] = "Changed heading"
        self.invoke(expected=1)

    def test_row_source_hash_must_equal_the_baseline_pin(self):
        self.rows[0]["source"]["sha256"] = "0" * 64
        self.invoke(expected=1)

    def test_duplicate_scenario_names_at_equal_total_are_rejected(self):
        self.rows[0]["scenarioNames"][1] = self.rows[0]["scenarioNames"][0]
        self.invoke(expected=1)

    def test_same_count_scenario_record_replacement_is_rejected(self):
        self.rows[0]["scenarioRecords"][1] = copy.deepcopy(self.rows[0]["scenarioRecords"][0])
        self.invoke(expected=1)

    def test_scenario_order_is_binding(self):
        self.rows[0]["scenarioNames"].reverse()
        self.rows[0]["scenarioRecords"].reverse()
        self.invoke(expected=1)

    def test_scenario_records_cannot_omit_a_scenario(self):
        self.rows[0]["scenarioRecords"].pop()
        self.invoke(expected=1)

    def test_normative_source_whole_file_hash_is_binding(self):
        self.write(self.rows[0]["source"]["path"], "Changed normative text.\n")
        report = self.invoke(expected=1)
        self.assertEqual("stale", self.selected(report)["currentness"])

    def test_actual_source_heading_is_checked_after_all_hashes_are_rebound(self):
        path = self.repo / self.rows[0]["source"]["path"]
        path.write_text(path.read_text().replace(self.rows[0]["heading"], "Different heading"))
        self.repin_source(0)
        self.invoke(expected=1)

    def test_actual_source_scenarios_are_checked_after_all_hashes_are_rebound(self):
        path = self.repo / self.rows[0]["source"]["path"]
        path.write_text(path.read_text().replace("Negative fixture 0", "Different fixture scenario"))
        self.repin_source(0)
        self.invoke(expected=1)

    def repin_source(self, index):
        source = self.rows[index]["source"]
        new_digest = digest((self.repo / source["path"]).read_bytes())
        source["sha256"] = new_digest
        self.rows[index]["sourcePins"][0]["sha256"] = new_digest
        self.baseline["groups"][0]["requirements"][index]["source"]["sha256"] = new_digest

    def test_source_pin_changed_or_missing_is_an_integrity_gap(self):
        self.rows[0]["sourcePins"] = [self.ref("public/source-pin.txt")]
        self.write("public/source-pin.txt", "Changed after pin.\n")
        self.invoke(expected=1)

    def test_observation_hash_mismatch_marks_closed_declaration_stale(self):
        row = self.close()
        self.write(row["validation"]["evidenceRefs"][0]["path"], "Changed after observation pin.\n")
        report = self.invoke(expected=1)
        self.assertEqual("stale", self.selected(report)["currentness"])
        self.invoke("--gate", "R-001", expected=1)

    def test_missing_observation_file_is_an_integrity_gap(self):
        row = self.close()
        (self.repo / row["validation"]["evidenceRefs"][0]["path"]).unlink()
        self.invoke(expected=1)

    def test_review_hash_mismatch_cannot_close(self):
        row = self.close()
        self.write(row["acceptance"]["reviewRefs"][0]["path"], "Changed review.\n")
        self.invoke("--gate", "R-001", expected=1)

    def test_transitive_staleness_reaches_every_dependent(self):
        self.rows[0]["dependsOn"] = [self.rows[1]["key"]]
        self.rows[1]["dependsOn"] = [self.rows[2]["key"]]
        self.write(self.rows[2]["source"]["path"], "Changed dependency.\n")
        report = self.invoke(expected=1)
        for index in range(3):
            self.assertEqual("stale", self.selected(report, index)["currentness"])
        self.assertEqual("current", self.selected(report, 3)["currentness"])

    def test_unknown_dependency_is_rejected(self):
        self.rows[0]["dependsOn"] = ["specs/unknown.md#Unknown"]
        self.invoke(expected=1)

    def test_dependency_cycles_are_rejected(self):
        self.rows[0]["dependsOn"] = [self.rows[1]["key"]]
        self.rows[1]["dependsOn"] = [self.rows[0]["key"]]
        self.invoke(expected=1)

    def test_self_dependency_is_rejected(self):
        self.rows[0]["dependsOn"] = [self.rows[0]["key"]]
        self.invoke(expected=1)

    def test_each_family_is_required(self):
        self.evidence["families"].pop()
        self.invoke(expected=1)

    def test_baseline_and_evidence_cannot_jointly_drop_one_of_twenty_families(self):
        self.baseline["families"].pop()
        self.evidence["families"].pop()
        self.invoke(expected=1)

    def test_family_duplicate_at_equal_count_is_rejected(self):
        self.evidence["families"][-1] = copy.deepcopy(self.evidence["families"][0])
        self.invoke(expected=1)

    def test_family_name_is_bound_and_unknown_family_is_rejected(self):
        self.evidence["families"][0]["id"] = "invented-family"
        self.invoke(expected=1)

    def test_family_name_cannot_be_replaced_under_the_same_id(self):
        self.evidence["families"][0]["name"] = "Another family at the same count"
        self.invoke(expected=1)

    def test_family_status_layers_remain_independent(self):
        family = self.evidence["families"][0]
        family["source"] = "observed"
        report = self.invoke()
        actual = next(item for item in report["families"] if item["id"] == family["id"])
        for layer in ("source", "admission", "compiled", "native", "private"):
            self.assertEqual(family[layer], actual[layer])

    def test_every_closure_component_is_required(self):
        cases = (
            ("implementation incomplete", lambda row: row["implementation"].update(status="partial")),
            ("validation unrun", lambda row: row["validation"].update(status="unrun")),
            ("missing validation references", lambda row: row["validation"].update(evidenceRefs=[])),
            ("missing scenario observation", lambda row: row["scenarioRecords"][1].update(status="unrun")),
            ("missing scenario evidence", lambda row: row["scenarioRecords"][1].update(evidenceRefs=[])),
            ("missing review", lambda row: row["acceptance"].update(reviewRefs=[])),
            ("source review only", lambda row: row["acceptance"]["reviewRefs"].pop()),
            ("result review only", lambda row: row["acceptance"]["reviewRefs"].pop(0)),
            ("self review", lambda row: row["acceptance"]["reviewRefs"][0].update(reviewer="fixture-author")),
            ("nonindependent review", lambda row: row["acceptance"]["reviewRefs"][0].update(independent=False)),
            ("missing independence declaration", lambda row: row["acceptance"]["reviewRefs"][0].pop("independent")),
            ("acceptance profile mismatch", lambda row: row["acceptance"].update(profile="synthetic-core/2")),
            ("acceptance target mismatch", lambda row: row["acceptance"].update(target="another-local")),
            ("unknown applicability", lambda row: row["applicability"].update(status="unknown")),
            ("inherited applicability", lambda row: row["applicability"].update(status="inherited")),
            ("empty profile", lambda row: row["applicability"].update(profile=""), 2),
            ("empty target", lambda row: row["applicability"].update(target=""), 2),
            ("missing acceptance profile", lambda row: row["acceptance"].pop("profile"), 2),
            ("missing acceptance target", lambda row: row["acceptance"].pop("target"), 2),
        )
        for case in cases:
            label, mutate = case[:2]
            expected = case[2] if len(case) == 3 else 1
            with self.subTest(label=label):
                row = self.close()
                mutate(row)
                self.invoke("--gate", "R-001", expected=expected)

    def test_profile_or_target_labels_cannot_migrate_original_scope(self):
        row = self.close()
        row["applicability"]["profile"] = "synthetic-core/2"
        row["acceptance"]["profile"] = "synthetic-core/2"
        for reference in row["validation"]["evidenceRefs"] + row["acceptance"]["reviewRefs"]:
            reference["profile"] = "synthetic-core/2"
        for scenario in row["scenarioRecords"]:
            scenario["evidenceRefs"][0]["profile"] = "synthetic-core/2"
        self.invoke("--gate", "R-001", expected=1)

    def test_actual_observation_and_review_scope_are_required(self):
        for location in ("validation", "scenario", "review"):
            for field in ("profile", "target"):
                with self.subTest(location=location, field=field):
                    row = self.close()
                    refs = (row["validation"]["evidenceRefs"] if location == "validation" else
                            row["scenarioRecords"][0]["evidenceRefs"] if location == "scenario" else
                            row["acceptance"]["reviewRefs"])
                    refs[0][field] = "different-scope"
                    self.invoke("--gate", "R-001", expected=1)

    def test_closed_dependency_must_itself_have_closed_gate(self):
        row = self.close()
        row["dependsOn"] = [self.rows[1]["key"]]
        self.invoke("--gate", "R-001", expected=1)

    def test_current_closed_dependencies_allow_parent_gate_closure(self):
        row = self.close()
        self.close(1)
        row["dependsOn"] = [self.rows[1]["key"]]
        self.invoke("--gate", "R-001")

    def test_complete_observations_cannot_automatically_close_open_acceptance(self):
        row = self.close()
        row["acceptance"]["status"] = "open"
        report = self.invoke()
        self.assertEqual("open", self.selected(report)["gate"])
        self.assertEqual("complete", self.selected(report)["implementation"]["status"])
        self.assertEqual("observed", self.selected(report)["observations"]["status"])
        self.invoke("--gate", "R-001", expected=1)

    def test_ready_next_actions_and_blocked_actions_remain_distinguishable(self):
        self.rows[1]["nextAction"].update(readiness="blocked", blockedBy=["missing actual result"])
        report = self.invoke("--next")
        actions = report["nextActions"]
        ready = next(item for item in actions if item["key"] == self.rows[0]["key"])
        blocked = next(item for item in actions if item["key"] == self.rows[1]["key"])
        self.assertEqual("ready", ready["readiness"])
        self.assertEqual([], ready["blockedBy"])
        self.assertEqual("blocked", blocked["readiness"])
        self.assertIn("missing actual result", blocked["blockedBy"])
        self.assertEqual(self.rows[0]["nextAction"]["consumer"], ready["consumer"])

    def test_next_places_actionable_consumers_before_blocked_consumers(self):
        self.rows[0]["nextAction"].update(readiness="blocked", blockedBy=["synthetic external observation"])
        self.rows[1]["nextAction"].update(readiness="ready", blockedBy=[])
        self.rows[2]["nextAction"].update(readiness="blocked", blockedBy=["synthetic dependency result"])
        self.rows[3]["nextAction"].update(readiness="ready", blockedBy=[])
        report = self.invoke("--next")
        actions = report["nextActions"]
        self.assertEqual({row["key"] for row in self.rows}, {action["key"] for action in actions})
        readiness = [action["readiness"] for action in actions]
        self.assertEqual(["ready", "ready", "blocked", "blocked"], readiness)
        for action in actions:
            original = next(row["nextAction"] for row in self.rows if row["key"] == action["key"])
            self.assertEqual(original["consumer"], action["consumer"])
            self.assertEqual(original["blockedBy"], action["blockedBy"])

    def test_next_omits_closed_gates(self):
        self.close()
        report = self.invoke("--next")
        self.assertNotIn(self.rows[0]["key"], {item["key"] for item in report["nextActions"]})

    def test_malformed_input_shape_exits_two(self):
        for section in ("applicability", "implementation", "validation", "acceptance", "scenarioRecords",
                        "sourcePins", "dependsOn", "nextAction", "source", "heading", "scenarioNames", "id", "group"):
            with self.subTest(section=section):
                original = self.rows[0].pop(section)
                self.invoke(expected=2)
                self.rows[0][section] = original

    def test_wrong_evidence_format_exits_two(self):
        self.evidence["format"] = "something-else/1"
        self.invoke(expected=2)

    def test_wrong_baseline_format_exits_two(self):
        self.baseline["format"] = "something-else/1"
        self.invoke(expected=2)

    def test_wrong_nested_field_types_exit_two(self):
        self.rows[0]["validation"]["evidenceRefs"] = "not-an-array"
        self.invoke(expected=2)

    def test_reference_path_type_is_checked_before_file_access(self):
        self.rows[0]["validation"]["evidenceRefs"] = [{"path": 17, "sha256": "0" * 64}]
        self.invoke(expected=2)

    def test_missing_evidence_containers_are_malformed(self):
        for section in ("format", "rows", "families"):
            with self.subTest(section=section):
                original = self.evidence.pop(section)
                self.invoke(expected=2)
                self.evidence[section] = original

    def test_malformed_json_exits_two_without_traceback(self):
        self.assertTrue(CHECKER.is_file(), "doctorwatson-utxo reporting entrypoint is not implemented")
        self.write(BASELINE, json.dumps(self.baseline))
        self.write(EVIDENCE, "{broken")
        result = subprocess.run([sys.executable, str(CHECKER), "--repo", str(self.repo), "--json"],
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(2, result.returncode)
        self.assertNotIn("Traceback", result.stderr + result.stdout)
        json.loads(result.stdout)

    def test_explicit_paths_and_repository_defaults_are_equivalent(self):
        default = self.invoke()
        explicit = self.invoke("--baseline", BASELINE, "--evidence", EVIDENCE)
        self.assertEqual(default["integrity"], explicit["integrity"])
        self.assertEqual(default["requirements"], explicit["requirements"])

    def test_selectors_are_mutually_exclusive(self):
        self.assertTrue(CHECKER.is_file(), "doctorwatson-utxo reporting entrypoint is not implemented")
        result = subprocess.run([sys.executable, str(CHECKER), "--repo", str(self.repo), "--json",
                                 "--gate", "R-001", "--next"],
                                capture_output=True, text=True, timeout=10)
        self.assertEqual(2, result.returncode)
        self.assertNotIn("Traceback", result.stderr + result.stdout)

    def test_reference_escape_absolute_private_and_symlink_are_rejected(self):
        outside = self.repo.parent / (self.repo.name + "-outside.txt")
        outside.write_text("Synthetic public outside fixture.\n")
        self.addCleanup(outside.unlink, missing_ok=True)
        link = self.repo / "public" / "escape-link.txt"
        link.parent.mkdir(parents=True, exist_ok=True)
        link.symlink_to(outside)
        private_ref = self.ref(".moriarty-dev/private/synthetic.txt")
        cases = [
            {"path": "../" + outside.name, "sha256": digest(outside.read_bytes())},
            {"path": str(outside), "sha256": digest(outside.read_bytes())},
            {"path": "public/escape-link.txt", "sha256": digest(outside.read_bytes())},
            private_ref,
        ]
        for ref in cases:
            with self.subTest(path=ref["path"]):
                self.rows[0]["validation"]["evidenceRefs"] = [ref]
                self.invoke(expected=2)

    def test_optional_command_strings_are_inert(self):
        marker = self.repo / "COMMAND_MUST_NOT_EXECUTE"
        command = f"touch {marker}"
        self.rows[0]["nextAction"].update(command=command, consumer=command)
        ref = self.ref("public/inert-command.txt", command=command)
        self.rows[0]["validation"]["evidenceRefs"] = [ref]
        self.rows[0]["implementation"]["detail"] = f"$( {command} )"
        self.invoke("--next")
        self.assertFalse(marker.exists(), "Reader executed an inert descriptive command")

    def test_reference_ancestor_symlink_is_rejected(self):
        with tempfile.TemporaryDirectory(prefix="doctorwatson-outside-") as outside_dir:
            outside = pathlib.Path(outside_dir)
            data = b"Synthetic public fixture outside declared repository.\n"
            (outside / "receipt.txt").write_bytes(data)
            (self.repo / "linked-directory").symlink_to(outside, target_is_directory=True)
            self.rows[0]["validation"]["evidenceRefs"] = [
                {"path": "linked-directory/receipt.txt", "sha256": digest(data)}]
            self.invoke(expected=2)

    def test_directory_reference_is_malformed(self):
        (self.repo / "public-directory").mkdir()
        self.rows[0]["validation"]["evidenceRefs"] = [
            {"path": "public-directory", "sha256": "0" * 64}]
        self.invoke(expected=2)

    @unittest.skipUnless(hasattr(os, "mkfifo"), "FIFO fixture requires POSIX")
    def test_fifo_reference_is_rejected_without_waiting_for_input(self):
        os.mkfifo(self.repo / "public-fifo")
        self.rows[0]["validation"]["evidenceRefs"] = [
            {"path": "public-fifo", "sha256": "0" * 64}]
        self.invoke(expected=2)


if __name__ == "__main__":
    unittest.main()

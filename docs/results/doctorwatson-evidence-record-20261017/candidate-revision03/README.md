# Evidence companion revision03: actual dependency elimination

Source-only candidate; author ran no GREEN/default/subject tests. Root's failed default-CI receipt remains unchanged. Sealed candidates01/02 and every frozen expectation are untouched.

The optional reporting function is now wholly in existing check.py, including local strict byte/JSON reads. The sibling compile_evidence_record.py is deleted. No module, dynamic load, path obfuscation, runner, import or resource control is added. CHECK_HELPERS, run_host.comparison_helpers, all guards and fixed-snapshot selection are unchanged. The complete reporting function is captured/checksummed as part of existing CHECK.

Public API/CLI selection and every output field/classification/qualification remain revision02 semantics. A source-only AST check verifies the former build_record body was transplanted without statement changes after two local helpers. This is structural source verification, not a runtime pass. Existing two full positive fixtures and Root125 portable test source remain unchanged. Root owns the two affected snapshot checks, two positive checks and then one affected default run.

Apply the check.py afterimage and DELETE the path in DELETIONS.json. Do not retain or register a sibling helper. Full025 remains OPEN; the compiler companion alone does not establish the original historical model-only reporting scenario.

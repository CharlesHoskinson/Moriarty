# DoctorWatson current EARS reporting

DoctorWatson now consumes the current Sherlock EARS evidence sidecar through
the existing local checker. It reports implementation, observed validation,
owner-declared acceptance and computed source/evidence currentness separately.
The inventories retain 35 original requirements/37 scenarios, 25 refinements/68
scenarios and nine linked retained-family catalogues. Catalogue coverage does
not mean every linked requirement is assessed or complete.

The baseline regression failed because the checker lacked the evidence input.
The corrected normal fixture and all ten final reporting regressions passed.
They include a real checker/CLI check that changes a supporting source, keeps
the old closure declaration visible, excludes it from the current count and
returns a mismatch. The original red fixture and its correction are preserved.

Root then ran the actual `./scripts/doctorwatson.sh` on the installed candidate:
exit 0, 251 tests passed in 10.261 seconds. All 35 original rows, 25 refinement
rows and nine retained catalogues had current declared references. The tested
ledger was candidate03, before semantic adoption, with all acceptance rows open.
This result validates the factory integration and report integrity; it does not
close those requirements. An earlier selected report failed on 25 missing
refinement evidence-class declarations; the corrected data and failure remain
separate records.

UIREQ-001, UIREQ-010 and UIREQ-011 are now closed on their supported typed A host
scope. Fresh review supports their exact original scenarios, unchanged
engine/producer/fixture dependencies, independent expectations and historical
actual observations. [Root adoption](acceptance/ROOT-ADOPTION.json) records the
scope; a subsequent selected checker run reports three current adopted closures.
The original pre-adoption factory snapshot remains unchanged. UIREQ-025 and
UIREQ-026 remain open. The historical full-fixture comparison used
its recorded checker/helper versions; this update does not rerun that capture
or claim that the current checker reproduces it.

The review's three schema-consistency corrections now have a preserved RED
regression and passing eleven-test focused check. The final default factory
passed 252 tests in 34.875 seconds, with all recorded input pins unchanged and
exactly three current adopted original closures. See the
[final receipts](factory/schema-followup/COPY-MANIFEST.json). The 25 refinement
requirements and 32 remaining original requirements are still open.

See the [current checklist](../../sherlock/requirements-status.md),
[canonical sidecar](../../../openspec/changes/sherlock-completion-convergence/current-evidence.json),
[implementation loop](../../sherlock/implementation-loop.md),
[factory receipts](factory/COPY-MANIFEST.json) and
[frozen assessment](candidate-03/README.md). The original candidate03 manifest
is preserved; `MANIFEST-FINAL.json` adds the later checklist/exclusions members.
Native, formal, circuit, financial and target obligations retain their exact
requirement scopes and missing evidence.

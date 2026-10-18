# Cumulative A resource reporting

The public ReferenceSlice reporter now records cumulative reference work, event
count and maximum observed logical depth under a named, versioned manifest.
Separate unknown physical, helper, host, preparation, circuit, proof and fee
observations remain explicitly unmeasured.

The normal production metadata/library and external test build passed. One exact
supplied-data test passed against all four independently frozen87-leaf report
expectations. Source and actual-result reviews accepted this bounded feature;
all396 input pins remained unchanged. The canonical test changes comments only,
with executed source retained separately.

[Actual result](actual/attempt-01/RESULT.json) · [Fresh result review](review/r155/REPORT.md)
· [Qualifications](review/r155/QUALIFICATIONS.md) · [Root adoption](ROOT-FEATURE-ADOPTION.json)

Original UIREQ-013 remains partial/open. The test supplied literal records rather
than collecting a live computation; complete manifest identities/profile domains
remain required. No native or complete-language acceptance follows. Earlier
clipped review and output-length deviations are preserved verbatim.

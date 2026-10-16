## Later Core/2 financial comparison

**Observation:** the existing production Core/2 library matched complete,
independently frozen observations for the new installment invoice and
conditional grant programs: eight invocations, including six resumes, five
successful segments and three ordinary false-await observations. Root's exact
once-only attempt succeeded with all six postflight checks true; result SHA256
`9f80e599e082b548c3974a1f8e6340c46a9b121f0baced4973daa362a256ca25`.

**Decision and reason:** retain both complete typed comparisons and a direct
comparison of actual JSON against the original independent JSON. The latter
passed 79 checks without using the typed translator, giving a separate check
on the transcription. Reuse the verified production Core library for this
bounded diagnostic because its Core/2 producer bytes matched; keep the cached
lexer and transitive source-provenance limits explicit.

**Evidence:** [captured local package](../../docs/audits/2026-10-16/sherlock-financial-core2-delivery/README.md).
The old freeze's five-resume metadata differs from the six complete resume
records; preserve that discrepancy and the original bytes.

**Next:** connect the reviewed financial control/compiler mapping to Sherlock
and compare the required complete outcomes there. This library-stage result
does not establish Sherlock lowering, rejection or non-null CLI coverage,
authenticated or consumed continuations, native proofs or target acceptance.

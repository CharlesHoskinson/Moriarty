# Local developer mock validation

Scope: S3 interaction prototype, not a DSL, real verifier or ACTUS conformance
result. Source: experiments/moriarty-developer-mock; full source hashes are in
source-manifest.json. Node model tests and TypeScript build passed; browser
checks are enumerated in browser-check.json. No native prover was executed.

- tests-and-build.txt: final Node model and TypeScript output.
- browser-check.json: ten browser smoke predicates and network/error results.
- desktop-loan.png, desktop-swap.png, mobile-swap.png: rendered screenshots with
  finite CSS animations completed for capture.
- mock-export.json: exported object retains mock markers and unavailable claims.
- browser-initial-failure.txt: two initial assertion failures preserved, followed
  by the diagnosed CSS text-transform mismatch; no application change was needed.

Independent review found three defects: input changes erased demo consumption,
unused controls were misleading, and invalid edits retained prepared evidence.
All were fixed. A model regression failed before invalidation was moved ahead
of input validation; the final suite includes it. Root browser verification
confirmed consumption preservation and corrected controls. This is a local
review, not a security audit or formal proof.

Remaining functional limits: imported schedules do not regenerate, calendar and
option arithmetic is source-only, lending health/PCD are unimplemented, and
configuration restore is not durable submission recovery. Report ingestion also
added static proposed claim descriptions; no claim codec/checker is implemented.

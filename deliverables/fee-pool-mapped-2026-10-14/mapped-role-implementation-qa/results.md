# Mapped-role static implementation QA

Implemented explicit unsigned-mapped-roles mode with distinct validated source and artifact types, per-action from-only mapped role inventory, independent qualifications, full pure source-closure hash, and fresh exact-byte reopen. Strict native/signed routes and native custody production files remain unchanged.

Actual current-byte checks (all exit 0; source hashes unchanged during each run):

- mapped-role-ledger: 8 passed; elapsed 157.825s including lock waits/build.
- legacy-static-regression: 19 passed; elapsed 10.342s including lock waits/build.
- native-all-policy-refusal: 1 passed; elapsed 87.499s including lock waits/build.
- capability-doctests: 18 passed; elapsed 43.884s including lock waits/build.
- legacy-escrow-native-refusal: 1 passed; elapsed 6.642s including lock waits/build.

All invocations use +1.99.0, --offline --locked -j2 and one test thread under /tmp/moriarty-language-wave1-build.lock. Exact argv/stdout/stderr/timings/source hashes are in each named receipt. No broad --lib or duty-read suite was executed: both native checks used their specific early-refusal filters.

Public compile-only capability checks independently observed three E0277 no-conversion errors (native Artifact, AdmittedSeed, DerivedLocalDeployment) and E0451 private-field construction, each rustc exit 1 as expected, with no other compiler error. See public-capability-compile-fail.json and raw compiler errors. These checks do not merely fail to name a private type.

The initial expected RED parser failure is preserved. The separate native-admission lane passed 3 tests/84 exact one-public-payer refusals; CLI lane reports 13 public tests green. No source/backend acceptance is inferred from those reports without the parent reviewing the linked actual receipts.

All five production changes and the new ledger test are frozen by owned_source_sha256 in results.json. No commits were made by this agent. Financial sources/bindings and protected native production files match the base commit. Full M11, custody/owner authorization, native proof and ledger settlement remain open.

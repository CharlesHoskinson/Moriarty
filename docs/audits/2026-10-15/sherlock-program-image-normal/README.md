# Flat image and interpreter normal observations — 2026-10-15

Implemented source: a deterministic whole-source table builder and a genuine
expression-ID interpreter for the existing strict K-A domain. Both modules are
private and compiled only under `cfg(test)`. Public source `/1`, `/2` and the
existing public host API are unchanged. This is preparation toward B-T02, not
completion of its bounded preparation/admission requirements.

The builder records every source occurrence, ordered children, all function
bodies in source order, parameters, arms, constants and source locators. Calls
remain named references. Four complete expected images were independently
specified from declarations and literal inputs before subject execution.
The first N07 comparison failed against a deliberately empty builder; the real
builder passed the 35-test closure, including all four expected images.

The new interpreter fetches expression IDs and ordered tables to execute. Its
separate AST reconstruction cache serves snapshots only. A common assertion
consumer checks both implementations against unchanged independent K literals,
rather than using either interpreter to generate the other's expectations.
All eight normal programs and 216 literal fuel prefixes passed, including every
complete state, saved frame/environment, exact rule/charge, free publication
and run/step final state. The interpreter closure passed 46 tests. Its deliberate
`step = None` RED fails the first N08 fuel-zero publication comparison; it does
not independently establish positive-recursion RED for every prefix.

The public `models/sherlock/reference/check-reference.py` now includes the five
new compiled sources in its before/after membership. Its separately recorded
bounded execution verified both frozen JSON transcriptions and passed all 46
tests. Actual Rust compiler SHA256 is in raw checker output. The raw compressed stdout/stderr are lossless captures.

Commands from the checkout root:

```text
python3 -I -B models/sherlock/reference/check-reference.py --rustc ABSOLUTE_EXISTING_RUSTC --timeout-seconds 30
```

Observed compiler: pinned Rust 1.99.0. Each direct compile/test had a 30-second
limit; the checker had a 90-second outer limit. All used 4 GB memory, swap off,
256 tasks, network disabled and read-only source. Only fresh scratch was writable.
Each receipt records elapsed time, exit and raw output digest. Membership binds
the complete compiled closure; unwired ongoing source files are excluded.

Fresh Muse high reviews cover normal source and actual results. These approvals
supply no Opus/security judgment. Helpers mirror the old reference and share
runtime types/display, so implementation agreement alone is not independent.
Arbitrary images are outside the domain; host usize indices, recursive Values,
growable vectors and unmetered helpers are not bounded circuit fields.

B schema/adoption, capacities/admission, helper metering, general ADTs, financial
continuations, source compiler lowering, K execution/refinement, Lean/Clean and
handwritten-circuit correspondence, constraints, native proofs and Midnight
stagenet compile/prove/verify/deploy/call/readback remain open. No full roadmap
milestone or retained U/M/MC/SP/G/ZR/MNR exit closes here. Earlier separate Quint
cohorts retain their own exact-source evidence.

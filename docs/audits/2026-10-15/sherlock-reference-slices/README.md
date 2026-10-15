# K-A event grouping normal observations — 2026-10-15

A private test-only `advance_m` groups genuine `Machine::step` records without
changing the reference relation. Every internal complete before/after state,
rule, charge and cumulative work remains materialized. Technical cuts retain
pending control, caller maps and frames; free publication remains an event with
zero work. No source suspension or financial commit is introduced. An event
budget of zero is an unchanged observation-only cut, not a new machine outcome.

The normal consumer compares each grouped record directly with unchanged
independent literal states, rather than with a second machine run. Sizes 1, 2,
4 and 8 each passed all eight normal programs and 216 literal fuel prefixes:
864 grouped prefix comparisons inside eight test functions. Five local normal
tests additionally check the N07 literal rule order, pending N07/N08 caller
returns, technical cuts and publication. The public std checker verifies the
frozen JSON transcriptions and includes both new files in its membership.

Behavioral RED: a separate copy retaining no events compiled, then the first
N08 group-size-one/fuel-zero case failed the next-event assertion. It does not
establish individual RED for all other cases. Genuine GREEN passed all 59 host
tests. The separately captured public checker also passed those 59 tests with
both oracle transcriptions exact. Original failed copies and captures remain
unchanged. No expected oracle value was regenerated from an implementation.

Actual Rust compiler SHA256 and complete source membership appear in raw checker
output. Direct stages had a 30-second limit each; the public checker had a
90-second outer limit. Both used 4 GB memory, swap disabled, task limit 256,
network disabled, read-only source and only new scratch writable. Receipt JSON
records each exit, elapsed time and raw stdout/stderr digest. Gzip captures are
lossless. Command from the checkout root:

```text
python3 -I -B models/sherlock/reference/check-reference.py --rustc ABSOLUTE_EXISTING_RUSTC --timeout-seconds 30
```

The grouping routine calls the existing interpreter and is not an independent
semantic implementation or an optimized/native circuit. All modules added here
are private under `cfg(test)` and leave public exports unchanged. Source/step
IDs beyond the existing K rule lines, canonical codecs, bounded B representation,
helper metering, financial continuation, compiler/circuit/Lean correspondence,
proofs and real Midnight stagenet acceptance remain open. No security review or
full B-T08/roadmap completion is claimed.

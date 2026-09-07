# R3 native financial IVC result

**The fixed loan circuit does not fit the planned k17 recursive circuit. No
native recursive proof was produced.** The executable harness and exact failed
runs are preserved; this is a backend feasibility result, not completed PCD.

The [summary](summary.json) records actual resource use. Three explicitly staged
runs used about 219 active seconds, charged 220 seconds after rounding, under the
original 1,200-second ceiling. Memory stayed below the 8 GiB cgroup ceiling; no k,
SRS degree, memory limit or financial relation was increased. The unused budget
was not spent after the concrete row-exhaustion result.

| Run | Result | Evidence |
| --- | --- | --- |
| 01 | Compiler E0689: genesis integer array needed an explicit UInt128 type. No harness runtime. | [log](attempt-01/native.stdout.txt), [terminal](attempt-01/result.json), original inputs in `attempt-01-inputs/`. |
| 02 | Type-corrected harness compiled; independent arithmetic and four local circuit controls passed; recursive VK setup failed with a generic synthesis error. | [log](attempt-02/native.stdout.txt), [terminal](attempt-02/result.json), [changed hypothesis](correction-02.json). |
| 03 | One diagnostic-only logging line exposed `NotEnoughRowsAvailable { current_k: 17 }`; recursive VK setup again failed. No proof. | [log](attempt-03/native.stdout.txt), [terminal](attempt-03/result.json), [diagnostic patch](diagnostic-error.patch), [changed hypothesis](correction-03.json). |

Source review preceded execution; [review](pre-run-review.md) and
[implementation scope](native-source-report.md) are preserved. Review did not
claim that the circuit would fit. Parent follow-up corrected source provenance:
the exporter now builds before loading generated modules and records hashes of
both TypeScript source and executed JavaScript. The runner rejects unreviewed
tracked-source changes; its diagnostic exception binds the exact logged patch.
The initial implementation report predates the explicit UInt128 annotation and
runtime result. Original input snapshots preserve each run's actual code.

`export-episode.mjs` exports actual R2 states/actions/effects and canonical hash
preimages. Independently written Rust arithmetic agrees on all 11 financial
fields across three states. The application circuit constrains 54 public limbs,
exactly genesis->accrued->settled, full fixed state/effect/authority identities,
64-bit limb ranges and closed-state rejection. It specializes one micro-USD loan
period with fixed authority; SHA256 and dynamic Ed25519 authorization are not
implemented inside that circuit. The remaining 4,500 USD notional is not discharged.

Passing application-only MockProver checks are not cryptographic proof evidence.
The planned two native proving steps, altered-proof/final-input rejection checks,
and final recursive accumulator verification were never reached. Different-VK,
injected dependency and standalone verifier persistence controls also remain
unimplemented due to the initially inspected API gaps. No full contract theorem,
compiler refinement, PCD HistoryCompliance or ledger acceptance is established.

The [SRS receipt](srs-receipt.json) binds the 25,166,212-byte k17 binary to the
pinned official catalog and SHA256. The ceremony's trust and point validity are
assumptions; no independent powers-of-tau consistency computation was performed.

Resource enforcement uses systemd/cgroup v2: 8 GiB memory, zero swap, group OOM
termination, two CPUs/build jobs, and per-attempt time reduced by previous runs.
[cgroup evidence](cgroup-confirm2.stdout.txt) records enforced values. A launch
outside those limits [rejected](no-cgroup-red.txt), and [reuse of an output directory](reuse-refused.txt)
rejected before execution. The first preflight's unsupported MemoryOOMGroup
property is retained; OOMPolicy=kill supplies the verified group setting instead.
The 256 MiB retained-output bound covers receipts/proof outputs; disposable Cargo
build and dependency caches are separate. Expected proof/PI outputs were also
bounded below 67 MiB in the harness.

The next native decision is whether to reduce the public-state representation
with a correctly checked commitment or explicitly revise k and its resource
ceiling. Neither change was made here. Docker and network transaction results are
[separate evidence](../moriarty-midnight-network-2026-09-07/README.md), and cannot
turn this native setup failure into a proof success.

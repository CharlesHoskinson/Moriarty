# Versioned selected-PK transport repair

The root adopted the corrected contract after two substantive independent design
votes, before authoring. The independent oracle SHA256 is
`014b8382d6721c26a4468c1dc7c633b3c5b6d1474bcc8d92e53b46aa62d91556`.
This is a source/ordinary transport repair. No native execution, PK size estimate,
new allocation, retry or financial acceptance follows from these checks.

`NativeWorkLimits` keeps exactly its current DTO fields. Explicit `/1` keeps
legacy unsigned field admission, Signed ordinary envelopes and PK/shared-total
semantics. Explicit `/2` caps selected PK at caller PK bytes (hard 64 MiB), public
leaf/aggregate at 16 MiB, input at 1 MiB and VK at 64 KiB. Existing input JSON/
marker classes remain 1 MiB; generated readback/verification/package output
classes retain their existing public bounds. No generated JSON output narrowing
or global codec change was made. Smaller caller budgets remain binding.
Fresh review identified retained source and descriptor export checks missing
under lowered v2 input limits. Shared helpers now check source/predecessor/
ledger-parameter lengths and bound deployment descriptor JSON by the current
input cap. Unsigned and Signed descriptor input serialization occurs before
copying export source/PK leaves, without an added serialization pass. V1 keeps
its prior input semantics and serialization position; markers and generated
output JSON classes retain their prior caps.

`prover_key_cap` keeps min(PK,leaf) only for v1. Shared `keyset_counts` checks
public bytes, PK bytes and their physical sum without wrapping. V1 physical
total still shares max_artifact_bytes; v2 physical ceiling is checked public
cap plus PK cap. Generation tracks source/all-action IR/manifest/VK separately
from its one selected PK. Export counts every non-PK leaf and both markers as
public. Signed descriptor replacement repeats the same keyset check. The
existing independent SRS acquisition/cache counter and its caps are unchanged.

The private singleton selected-PK context is created by `check_keyset_export`
from a real `DerivedCallMaterial`, called only by validated unsigned/Signed
export factories. The generic `ArtifactLeaves::check` is unchanged and grants
no filename exception to proof, package or arbitrary artifacts. Ordinary tests
exercise the private byte/count helper using tiny opaque data and lengths;
they construct no derived owner and prove no positive native export.

Cold public readers continue acquiring only source/all-action public leaves and
the public marker. Declared optional PK/proving/producer payload sizes and their
files are neither acquired nor charged, even with nonsensical large metadata,
absent files, dangling symlinks or FIFOs. The encoded declarations themselves
remain part of the acquired, charged public marker. Proving readers additionally charge their proving
marker as public and exactly one PK as prover. Those acquired-subset counts do
not describe the complete producer export. Declared PK domains and physical
addition are checked before open; nonblocking regular-file, exact declared
length, sentinel and exact hash checks remain. Imported PK stays opaque.

Both reopen consumers reject lowered current domains before source preflight.
On admitted inputs, their existing fresh original-source all-action regeneration,
public/VK comparison and exact selected-PK comparison still precede regenerated
provider use. Positive native export/reopen was not run. Signed v1 keeps its
old short-circuit import guard before validating unrelated fields in preflight;
v2 validates its new limits before applying separate-domain guards. Two
double-fault regression captures cover legacy refusal order.

Actual ordinary captures are under
`.moriarty-dev/runtime/installed-call-20261012/signed-installed-source-20261012/key-export-diagnostic-01/prover-key-domain-implementation-01`.
`red-reader-01.log` records the missing explicit v2 reader behavior before
production edits. `red-v1-priority-01.log` and `red-v1-priority-02.log` record
the two initial Signed legacy priority regressions before restoring that branch.
`red-lowered-input-01.log` records both missing v2 input guards after extracting
the existing checks into helpers used by the real factories, before fixing them.
Tiny opaque fixtures are at most 8193 PK bytes; real 16/32/64/80 MiB boundaries
and machine-width overflow use counts only, with no large buffers/files.

Final ordinary Rust checks use Rust 1.99.0, `--locked --offline`, the existing
shared target and `CARGO_INCREMENTAL=0`; no missing dependency was fetched.

| Check | Actual result | Capture |
| --- | --- | --- |
| call_artifacts / installed_seed | 13 / 3 passed after lowered-input repair | ledger-consumers-final-02.log |
| Pure domain/profile/export-context and input controls | 7 passed | ledger-domain-final-02.log |
| Existing diagnostic controls plus effective v2 writer cap | 7 passed | ledger-diagnostic-final-01.log |
| Private owner compile-fail doctests | 12 passed | ledger-owners-final-01.log |
| CLI units / unsigned / Signed installed consumers | 17 / 4 / 4 passed | cli-final-01.log |
| Exact aggregate acquisition controls | 2 passed | ledger-aggregate-final-01.log |
| Unsigned / Signed all-action source preflight | 1 / 1 passed | ledger-source-preflight-final-01.log / ledger-signed-preflight-final-01.log |
| Ordinary mori build / fmt / whitespace | exit 0 after lowered-input repair | build-final-02.log / fmt-final-02.log / freeze record |

These are 71 distinct focused ordinary tests across the applicable captures;
unchanged diagnostic/ownership/CLI/preflight scopes retain their final01 results.
No historical broad suite was rerun or counted as current verification.

The descriptor replacement test is shared-helper coverage, not a genuine Signed
export. Lowered-budget reopen tests invoke the real consumers on untrusted
reader snapshots and stop before their deliberately invalid preflight sentinel.
Generic filename controls separately isolate leaf and aggregate refusal; cold
reader no-access claims also depend on inspection of its unchanged acquisition
branch, not timing alone. Existing SDK default refusal behavior is unchanged.

The independent Quint slice records bounded model checks and focused source
correspondence separately. It does not establish Rust/native equivalence,
cryptographic validity or feasibility of a 32 MiB hypothetical caller cap.
The tracked model has 27 passing deterministic controls; 10000 sampled traces
with maximum eight steps and 500 with maximum 12 steps found no violation.
All 14 target witnesses were reached in the 10000 sample, and three deliberate
faults have retained counterexample ITFs. Current correspondence03/result02
bind provisional Rust02; result02 SHA256 is
`cf2d1121968c01be0bfdfa1d95fddd1f3225ef25e3c67d927a880bb1138ece3a`.
The root admitted that limited correspondence before final freeze; full-source
approval remains separate. See the tracked
`experiments/moriarty-language/formal/quint/language/prover_key_domain.md`.
The final freeze pins source closure and the unchanged SDK revision; changed
source closure invalidates prior exports for this new consumer. Earlier d4/05
producers, native negatives, audits and grants remain immutable. Fresh full-source
audits, publication and any later exact resource decision remain root-owned.

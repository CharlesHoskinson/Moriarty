# DoctorWatson improvement log

Started 2026-10-16 UTC while Sherlock implementation proceeds. This running log
collects lessons for a later factory update. It is a working record beside the
[Sherlock journal](sherlock-implementation-journal/README.md), separate from the
canonical `wiki/` claim ledger and [factory plan](../docs/DOCTORWATSON.md).

Status labels: **observed practice** means a recorded manual technique or result;
**implemented factory code** requires an exact source reference and scoped check;
**proposed** means backlog. None of the proposals below is implemented factory
code by virtue of this entry. Source review, compiler metadata, runtime results,
native results and roadmap acceptance remain separate.

After each material implementation iteration, append a dated entry or correction
with its stable ID, observed problem and evidence, proposed change, status,
validation still needed and next owner/action. Record applied source separately
from validated behavior. Preserve failures and dissent. Link published evidence;
if evidence remains unpublished, say so and add its public locator only after
publication is verified. Keep provider streams, private review contexts, private
paths, secrets and private-fork material out of this log.

## 2026-10-16 — first lessons

### DW-01 — Bound complete deliverables before increasing parallelism

**Observation/evidence:** the implementation coordinator reported an oversized
finance response truncated after 1,673 seconds. Four narrower, complete,
disjoint units then completed in 56–95 seconds each and supplied 63 parseable
finance patch operations. The retained mechanical join contains those 63 plus
63 ADT operations, 126 total, with disjoint owned pointers. This is a mechanical
join, not semantic adoption. The timing report and join evidence are unpublished.
Models and scopes changed; these numbers are not a controlled speedup benchmark.

**Proposed:** ask each owner for one complete bounded artifact with explicit
membership and unresolved dependencies; split an oversized deliverable by owned
domain before retrying. **Status:** observed practice; factory automation proposed.
**Validation needed:** publish a scoped evidence summary, check completeness and
dependency reconciliation, and measure comparable future tasks before making a
performance claim.

### DW-02 — Preserve provider failure and check factual assertions

**Observation/evidence:** coordinator reports retain a direct Claude
authentication failure before model work. A separate Argon report asserted
inaccurate module/CLI facts despite unchanged source. The
[Sherlock journal](sherlock-implementation-journal/2026-10-16.md#k-capture-adapter-actual-source05-result)
records the source-metadata discrepancy and a fresh authorized review context;
the underlying provider failures are unpublished.

**Proposed:** retain failure and dissent, record requested/returned identities and
terminal status, and cross-check concrete assertions against pinned source before
relying on a review. Use only the standing authorized independent fallback when
its condition applies. **Status:** observed practice; structured factory reporting
proposed. **Validation needed:** a future report must distinguish pre-model
failure, substantive review and checked factual corrections. This entry makes no
security judgment or policy change; such changes belong to the designated owner.

### DW-03 — Apply only exact owner-supplied edits

**Observation/evidence:** the retained 126-operation join checked JSON-pointer old
values or required absence, disjoint ownership and authored append order. A later
qualified-host source adoption replaced one exact encoded predicate and checked
deep equality of the remaining JSON. Both records remain unpublished.

**Proposed:** bind every patch to its exact base and validate the full old span or
pointer value before applying it; return mismatches to the owner instead of
inferring replacement text. **Status:** observed practice; reusable factory support
proposed. **Validation needed:** demonstrate rejection of stale bases, ambiguous
spans and overlapping edits, then separately review the composed meaning.

### DW-04 — Keep source and each result class independent

**Observation/evidence:** a retained SDK-free diagnostic accepted an exact
21-file crate as Rust metadata, with zero errors and six warnings; its receipt
explicitly records no linking, engine execution or tests. The subsequent source
adoption remained qualified host source only. These receipts are unpublished.
[Published B-core evidence](../docs/audits/2026-10-16/sherlock-b-core-host-source/README.md)
already illustrates the narrower metadata-only distinction on an earlier candidate.

**Proposed:** show candidate source, metadata compilation, runtime, native and
acceptance as separate fields, each with exact consumed membership and attempt.
**Status:** observed practice; any additional factory fields proposed.
**Validation needed:** verify that missing or changed inputs cannot inherit a
different candidate's result, and that unrun stages remain unrun.

### DW-05 — Give each audience only its independent inputs

**Observation/evidence:** parallel owners report using input-only and case-free
packets to preserve independent normal expectations. The
[published B EnvGet/Bind projection](../models/sherlock/quint/b-env-bind/README.md)
separates finite input witnesses from unrun model execution. Private packet
contents and expectation values are deliberately absent here.

**Proposed:** define an explicit audience and selected input membership for each
producer, oracle and reviewer packet. Keep frozen expected data separate from
production authors. **Status:** observed practice; factory packet checks proposed.
**Validation needed:** inspect actual recipient inputs and dependencies for oracle
leakage and circular derivation. A source audit cannot establish execution;
matching JSON cannot alone establish producer origin.

### DW-06 — Prepare consumers and operation dependencies together

**Observation/evidence:** parallel preparation exposed shared type, helper and
consumer dependencies before integration. The journal's
[source-closure discussion](sherlock-implementation-journal/2026-10-16.md#what-changed-our-implementation-approach)
records why narrow helper preparation did not establish the complete machine.
The later 21-file diagnostic above supplies an unpublished type-check observation.

**Proposed:** prepare dependent operation signatures and their real consumer in
parallel under one shared-interface integrator; request an early scoped type
diagnostic when eligible. **Status:** observed practice; scheduling improvements
proposed. **Validation needed:** compare the declared dependency closure with the
actual compiler read set, then demonstrate the consumer's behavior separately.

### DW-07 — Ground compiler advice in the actual pinned tool and types

**Observation/evidence:** coordinator reports identify review assertions about
compiler flags, edition and parser types that required checks against actual
compiler/source facts. The specific review corrections remain unpublished.

**Proposed:** attach exact tool/version and source-type locators to such advice;
do not invent nightly flags or replace actual parser types with remembered APIs.
**Status:** observed factual discrepancy; the fresh source-author correction is
in progress, with no corrected result claimed; factory checklist proposed.
**Validation needed:** check the proposed invocation and type signatures against
the pinned tool and consumed source; retain diagnostics and unresolved claims.

### DW-08 — Publish the exact reviewed selection and verify merged bytes

**Observation/evidence:** the retained integration receipt for
[PR #124](https://github.com/CharlesHoskinson/Moriarty/pull/124) binds five selected
files and records matching merged blob hashes at
`e008b10eb410ba75ba4a6d049c0fc3f7db80029c`. It also records actual default local CI:
230 tests passed, exit 0, 91.605 seconds. That default suite did not execute the
new Quint model. The receipt and complete CI logs remain unpublished here; the
PR is a source-publication locator, not a substitute for those logs.

**Proposed:** every handoff names selected paths, sizes and hashes; independently
read back selected merged blobs before reporting publication. Publish selected
evidence rather than a whole private working directory. **Status:** observed
practice; additional factory automation proposed. **Validation needed:** recheck
the selection at integration and merged readback; obtain fresh review for changed
bytes. This documentation change runs no CI and inherits no test result.

### DW-09 — Optimize time to usable behavior

**Observation/evidence:** the journal records real-format decoder gaps and a
manifest defect discovered on the actual consumer path. Parallel comparison work
also reports that an endpoint match does not establish all finite-fuel runs;
missing actual input remains unobserved. The latter work is unpublished.

**Proposed:** prioritize a usable complete comparison/consumer path, first
divergence and next runnable capability; leave administrative refinements in this
backlog. **Status:** observed practice; prioritization proposal, no new scheduler.
**Validation needed:** record time to the next demonstrated behavior and its
complete scoped comparison, with pending work held pending until actual terminal
receipts. Keep the existing factory stop rules and all acceptance requirements.

The next factory-update pass should select a small proposal from this log only
after the useful Sherlock increment is delivered. Any proposal touching execution
guards, resources, security policy or security source requires the designated
security owner. This log grants no execution authority and closes no parent exit.


## 2026-10-17 UTC — second edition

The [first edition](https://github.com/CharlesHoskinson/Moriarty/pull/125)
was published at `ef8a6636de12ded575e039fea5a80c1aa8d63320` with all four selected
blobs verified by the integrator. Earlier entries above retain their original
freeze and status; the following observations update only their stated scope.

### DW-07 update — Corrected invocation reached an actual host capture

**Observation/evidence:** the coordinator records a fresh designated Muse-authored
minimal invocation candidate and separate fresh security/nonsecurity operational
reviews, each with actual CLI exit 0 and scoped source conclusions. Root then
recorded the exact operation and ran it once. The retained independent result
records actual tool exit 0: compiler version check exit 0 in 0.229 seconds;
exact 21-file library compilation exit 0 in 5.873 seconds, retaining six warnings;
binary compilation/link exit 0 in 1.086 seconds; and the D01 host run exit 0 in
0.011 seconds, with empty run stderr. The capture contains seven parseable JSON
records, 30,046 bytes, SHA-256
`9231489b77a7cf4a2bffe05a197428b755f35e15117a43d6331214d1fe32d114`.
Root's receipt verifies the exact library/binary dependency read sets and source,
snapshot and artifact hashes. That receipt remains unpublished; no public result
locator is claimed here.

**Status:** observed correction, scoped source reviews and actual host capture.
Semantic comparison is unrun and fresh actual-result audits are pending at this
entry's freeze. This updates DW-07's earlier in-progress correction status; the
factory checklist remains **proposed**. No full-B, native or parent acceptance
follows from parsing a capture or from operational source review.

**Validation needed / next action:** the comparison owner must compare the actual
capture against independently frozen complete expectations; designated reviewers
must assess the actual result and its bindings. Publish only the separately
verified evidence selection. Record those terminal results in a later entry.

### DW-10 — Track input context and output headroom separately

**Observation/evidence:** the coordinator reports two Muse row-author calls
failed before a source report: CLI exit 1, zero report bytes and no tools. Their
provider-reported context-compaction figures were 473,027 and 473,183 tokens,
both above the reported 460,800 threshold. The failures are retained as failures
and supply no review vote. Underlying receipts remain unpublished. These figures
describe those provider attempts; they do not establish a universal model limit.

**Observed change of approach:** the coordinator retained full current typed
schema, rows, canonical requirements, security/interface/Core2 context, current
dissent and dependency contracts inline, while moving superseded historical
companions into an explicit inventory reserved for final composition. This
changes packet assembly without waiving requirements or discarding dissent.
Exact resulting row-artifact validation remains pending for this entry.

**Proposed:** track observed input-context/transport headroom separately from
output-artifact budget and completion time. Preserve failed raw transport
privately, require complete parseable owner artifacts, and retain final
composition dependencies when reducing a packet. **Status:** observed failures
and packet revision; factory automation and general performance conclusions
remain **proposed**. DW-01's changed-model/scope benchmark limitation still applies.

**Validation needed / next action:** the source coordinator must verify current
requirements and dissent membership in the revised packet, complete owner output
and exact application against its base, then reconcile the reserved dependencies
before composition. Keep later outputs pending until their actual terminal and
artifact checks. No user budget, concurrency limit, execution guard or security
policy is changed by this log.


## DW120 — Freeze complete declarations and qualify the exact observation

A source/API review needs the complete declaration body through its closing delimiter, not a line range that stops at a struct opening. Pin and provide the actual fields, enum payloads, signatures and source regions used by the consumer. A whole-file hash identifies the unseen bytes but does not make a clipped declaration reviewed. Keep that omission separate from a typecheck or runtime result; the K complete-image compile retains its overall/Rust API ABSTAIN rather than converting source clarification into a new verdict.

Preserve raw receipts and qualify their predicates. In the [Quint B22 batch](../docs/results/sherlock-quint-parametric-literal-images-20261019/README.md), all four operations and eight directed tests passed. Each phase had 4,678 before/after source/tool/symlink checks and exact three-file model-directory membership. These are snapshots, not continuous immutability or a complete host-access/extra-file monitor. The raw `sourceAndExpectationsUnchanged=true` remains a literal in its original outcome. The separate EXPECTATIONS JSON was not in the eight phase snapshots; its later match to the frozen pre-execution review pin is a narrower post-readback observation, recorded separately. Do not rewrite the raw outcome, backfill phase coverage or fit expected cells to observed output. The 6,800-byte capture-helper prefix is also a prefix, not a full execution closure; the separately pinned full helper and retained review limits matter.

The [new K definition](../docs/results/sherlock-k-literal-complete-images-compile-20261019/README.md) compiled in 14.951 seconds and produced a 1,068,521-byte `definition.kore` (`df05e8d632013956de8704c61aeef513b53c33c775736f11b7a3aeb1f7222ac9`). This is fresh definition generation, with no new complete-image runtime comparison. In Fenrir, [Clean B28](https://github.com/CharlesHoskinson/projectfenrir/blob/9250d9ed/clean/fork/docs/results/sherlock-clean-complete-literal-statement-20261019/README.md) checked six finite Nat facts in 3.379900 seconds: 174 postcheck pins matched and five outputs totaling 526,120 bytes are retained by pins. Complete represented Program/Init/middle/Done recovery and derived child views do not establish Fr/hash/crypto, production correspondence or native/target acceptance. The B22 expectations are finite, same-author and source-exposed; none of these three results supplies a new EARS closure.

A wire bump is connected implementation work. The unreviewed A80 source now declares `moriarty-sherlock-link/2`, including per-child qualifications/obligations, provenance and optional carrier identity. Its private verified-export seal and checked offsets are source mechanisms; intrinsic manifest validation neither reads sibling files nor authenticates keys or proofs. The reviewed parent still reads `/1`. The `/2` parent reader must be reconciled, independently reviewed and actually integrated; do not label a new version string, metadata map, silent downgrade or permissive parse as compatibility. Keep the exact 16-statement-plus-12-accumulator route distinct from the stock 17/29 CLI and the missing current anchor/Template integration.

Root reports Moriarty commit `34a8593b` and Fenrir commit `9250d9ed` pushing at this dated cut, with PR numbers pending; this entry infers no merge or target result. The [source facts](../docs/results/sherlock-dw120-design012-preparation-20261019/SOURCE-FACTS.json) pin the actual records and retain the uncompiled/unreviewed source boundaries. DESIGN011 remains byte-identical. DESIGN012 is a separate pending Opus 5.5 HIGH creative contribution, not Sol-final website copy. Documentation and concept preparation run asynchronously alongside the native/circuit work; no new factory, execution permission or build hold is introduced. Sherlock's recursive stagenet and Fenrir experiment remain the priority.


### DW120 later creative-publication observation — October 19

The actual Opus 5.5 HIGH DESIGN012 author returned successfully after 254.400744 seconds. Its complete note and README append are materialized verbatim, with explicit marker spans and an exact reconstruction proof; no Sol creative replacement is presented as Opus. The [separate status](sherlock-circuit-atlas/DESIGN-012-STATUS-AND-PROVENANCE.md) preserves the author's input cut while recording Root's later actual merged readbacks for Moriarty PR294 and Fenrir PR35. A later 6,106-byte uncompiled anchor source is also Root-reported, with fresh review pending; it does not imply target integration. Keep the original source-cut facts, raw review qualifications and all old concept notes intact. The site still launches only when the full Sherlock goal is complete and accepted, and this documentation adds no native/circuit build hold, recursive proof, deployment or EARS closure.

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


## 2026-10-17 UTC — third edition

The [second edition](https://github.com/CharlesHoskinson/Moriarty/pull/127)
was published at `750595ba668c4a4ddf9040658c0198af72fe1682`. Prior entries retain
their freeze-time status. The entries below distinguish applied consumer source,
observed practice and proposed factory improvements. This edition validates no
new factory automation.

### DW-11 — Exercise the real reader and retain incomplete results

**Observation/evidence:** actual JSONL exposed a missing top-level rule lookup
and a profile-carrier presentation mismatch in the previously source-reviewed
consumer. Successor source uses the existing closed rule map and the explicitly
adopted profile bijection, retaining raw integer lexemes and actual locators;
other identity atoms remain separately compared. Root's exact format/profile
adoption was source-only. Fresh full Muse HIGH source review qualified the
successor; source publication remains pending at this freeze.

Root independently read back an actual 13-case host attempt: all 16 children
exited 0 and the immutable captures total 10,135,483 bytes. The ordinary QA reader
then compared each supplied capture at its selected fuel. All 13 comparisons
returned exit 2, **GAP**, **NO_MAPPED_DIFFERENCES**, with no first mapped difference.
Complete carrier/reference/producer obligations remain open. These unpublished
receipts do not establish full semantic agreement or exhaustive finite-fuel
coverage; fresh actual-result reviews and Root acceptance remain pending here.

**Status:** applied consumer fixes, fresh source review and observed ordinary DATA
comparisons. **Proposed / validation needed:** retain raw capture and first
mismatch, review a separate successor, then replay unchanged DATA. Track mapped
field agreement and complete acceptance separately; finish the outstanding
carrier checks and actual-result reviews before any stronger claim.

### DW-12 — Obtain missing input from the producer

**Observation/evidence:** the current v1 capture omits the actual constructor
source-program carrier. New source-input capture/serialization work has been
assigned; finalized implementation and an actual v2 observation remain pending.
Independent expected observations cannot supply the missing actual input.

**Status:** observed carrier gap; source work in progress. **Proposed / validation
needed:** require the actual producer to emit and bind its genuine source input,
then check exact consumed bytes through the consumer. Preserve the independent
oracle and existing gaps while that work is incomplete. Expected-output equality
cannot establish the producer's input or its origin.

### DW-13 — Adopt only the premise actually recomputed and reviewed

**Observation/evidence:** the retained standalone exact-16 SDK premise result was
recomputed and received fresh actual-result reviews from two separate Argon
instances and Muse. Root recorded only A4 row **P / PASS-EVIDENCE** and
**R2-AMD-X16-01(ii)** for that single consumed execution. The two Argon instances
share a model/provider; they are not cross-provider diversity. R5 remains a
separate pending decision, P10 retains its historical blocked status, full native
R2 remains unsatisfied, native authority remains zero and C4 remains open. The
underlying adoption record is unpublished.

**Status:** observed scoped premise adoption, reported without substitute security
analysis. **Proposed / validation needed:** bind each adopted premise to the exact
result, recomputation, actual reviewer identities and narrow predicate. Preserve
separate downstream conditions; route further decisions to their designated owner.

### DW-14 — Correct provenance and refresh availability from observed work

**Observation/evidence:** [PR #128](https://github.com/CharlesHoskinson/Moriarty/pull/128),
merge `8174e52975d8db9bf59636fea0fa338c0f47c855`, published the D01 host capture
with a corrected input-only packet digest. The raw capture remained unchanged;
earlier dissent and the superseded selection were retained, and the corrected
selection received fresh reviews. This supplies the public locator that DW-07's
second-edition freeze lacked, with semantic acceptance still separate.

A completed request returned canonical Opus identity and new security assignments
returned to Opus under the standing instruction. Existing fallback work keeps its
actual identity and scope. The [restoration record](../raw/assignments/moriarty-opus-security-restored-2026-10-16.md)
states that a completed request does not establish provider-wide health.

**Status:** observed provenance correction, publication and provider recovery.
**Proposed / validation needed:** check concrete digest/source claims before
publication and use actual canonical terminal results to update availability.
Retain old failures and fallback authorship; obtain fresh review for changed bytes.

### DW-15 — Give creative authors current steering and preserve history

**Observation/evidence:** the website brief now asks for confident finished-launch
visitor copy alongside truthful internal cumulative source/evidence history.
The creative author received the combined steering and returned an exact revised
draft; earlier drafts and authorship remain preserved. Full-candidate non-author
review included the creative reply and provenance/navigation additions. A design
companion remains distinct from a frozen circuit iteration; website prose and
source publication do not establish measured circuit activity.

**Status:** observed author/reviewer practice; new website implementation remains
separate. **Proposed / validation needed:** forward changed steering to the actual
author, materialize its exact revision and add later factual evidence as dated
provenance. Keep source publication, host observations and measured circuit
activity independently bound. Integrate only the exact reviewed public selection.


## 2026-10-17 UTC — fourth edition

The [third edition](https://github.com/CharlesHoskinson/Moriarty/pull/131)
remains unchanged above. [PR #132](https://github.com/CharlesHoskinson/Moriarty/pull/132)
published the handwritten scalar-tag leaf source, and
[PR #133](https://github.com/CharlesHoskinson/Moriarty/pull/133) published the
independent K/Quint sources. Those new circuit/model sources remain uncompiled
and unexecuted. The following lessons record this iteration's frozen status;
they implement no new factory automation.

### DW-16 — Capture genuine typed inputs and inspect retained results passively

**Observation/evidence:** the v2 six-input carrier reached an actual SDK-free
library compile, binary link and one D01 host run, each exiting 0. Root read back
seven inert JSONL records, 31,436 bytes, SHA-256
`12820350e75c6e7d37be799671fb1c5f942eda57491629719325c59c6a9d59a3`.
The initial record contains genuine constructor inputs. Later R2/R3 factual
attachments inventory the attempt tree and current pins without re-executing the
subject or reusing artifacts. These receipts remain unpublished.

**Status:** observed host capture, passive readback and retained scoped actual-result
reviews. Root's retention decision, v2 QA comparison and complete acceptance
remain pending at this freeze.
**Proposed / validation needed:** retain typed inputs from the actual producer,
then bind passive inventories and consumer comparison to the same immutable
capture. File-tree and hash facts cannot supply semantic or format judgments.

### DW-17 — Count bodies actually supplied separately from pinned closure

**Observation/evidence:** Root's coverage correction records that an actual
operation-review packet supplied 22 literal Rust bodies and two reconstructed
bodies. Separate full-body addenda supplied the missing two literal bodies and
received scoped reviews; their combined coverage is not one original whole-body
audit. A complete membership manifest alone did not mean every body had been
supplied. The original packet and correction remain retained separately; their
underlying reports are unpublished.

A reported `PlanDelta: Copy` mismatch was also withdrawn after checking the
literal declarations: all three inspected `PlanDelta` declarations derive Copy;
the adjacent `PlannedWrites` does not. Both shared revisions already derive Copy.
The original incorrect note and factual correction remain preserved. No compile
occurred and no compiler failure or confirmed source mismatch was established.

**Status:** observed packet-coverage correction. **Proposed / validation needed:**
account separately for literal body, reconstructed body, excerpt and pin-only
coverage in each exact reviewer packet. Bind later addenda and their review to
their supplied bytes; preserve the original coverage limits and dissent. Do not
infer full-body reading from a manifest or a successful provider exit.

### DW-18 — Keep cross-package publication pins coherent

**Observation/evidence:** immutable links among raw capture, QA summary and
consumer source required narrow I5 metadata repins. The dated joint handoff
selects one atomic publication of 19 raw-package files, seven QA files and
11 consumer files. All 13 raw captures remain unchanged. The QA metadata
successor preserves historical consumed membership separately from intended
publication membership. Root subsequently merged the selection in
[PR #134](https://github.com/CharlesHoskinson/Moriarty/pull/134), commit
`e3a274c448a44ed29fd78bd1ca7d7314a4ab358c`. The original 13 comparisons remain
GAP, with no new run or semantic result from publication.

**Status:** observed metadata repair, exact selection and source/data publication.
**Proposed / validation needed:** update only the affected provenance/status
metadata, check all linked selections together and verify the exact committed
blobs. Preserve immutable consumed-result identities; unchanged capture bytes and
result rows do not become a fresh execution or a stronger comparison.

### DW-19 — Carry the owner's exact preimage contract into source handoff

**Observation/evidence:** the adopted source04 helper contract records that
`alloc_env.previous`, when present, must be a recorded preimage EnvId below
`pre.img.active.env`. Root adopted the exact corrected source dependency only.
That adoption does not
establish Stage2 acceptance, a compiler result, execution eligibility or full B
behavior; its receipt remains unpublished.

**Status:** applied source correction and scoped source adoption.
**Proposed / validation needed:** retain the exact owner-authored preimage/index
condition with the candidate and its consumers. Keep source adoption and later
actual behavior separately evidenced; further contract judgments remain with
the designated owner.

### DW-20 — Preserve an incomplete physical partition as a gap

**Observation/evidence:** independent normal physical QA maps 296 known lanes per
case, with the named helper/counter lanes recorded. The exact remaining physical
partition is still an explicit **GAP**; complete physical inputs are **NOT_READY**.
The missing partition and Operand construction/import declarations prevented a
complete typed fixture harness. The retained handoff records this without an
invented empty rest, zero lanes or harness. No expected tuples are published here.

**Status:** observed bounded normal-data preparation; complete physical fixture
and harness remain pending. **Proposed / validation needed:** obtain the missing
exact declarations from their owner, then validate the complete partition before
claiming a callable physical fixture. Known-lane coverage is a separate result.

### DW-10 update — Reduce duplicate context without dropping current obligations

**Observation/evidence:** another Muse request reported 472,889 context tokens
against 460,800; a FullB16 Opus request reported 1,081,472 against 1,000,000.
Both failed without a substantive vote. The failures remain preserved. The
coordinator's changed approach uses exact-SHA deduplication and companion-specific
packets retaining full current requirements and dependencies. These are observed
provider-attempt limits, not universal model or concurrency limits.

**Status:** observed failures and packet revision; any resulting review remains
pending unless separately recorded with its actual terminal and full scope.
**Proposed / validation needed:** verify the deduplicated membership, full current
companion requirements and reserved dependencies before retrying a bounded
packet. Preserve failures as nonvotes. This log changes no budget, execution
guard, security policy or acceptance requirement.

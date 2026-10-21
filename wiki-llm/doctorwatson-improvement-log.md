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


## 2026-10-17 UTC — fifth edition

The [fourth edition](https://github.com/CharlesHoskinson/Moriarty/pull/136)
remains byte-exact above. These entries distinguish published source, observed
execution, review coverage and admitted reliance. They add no factory automation,
security policy or execution-control implementation.

### DW-21 — Follow source delivery through its real consumer

**Observation/evidence:** [PR #137](https://github.com/CharlesHoskinson/Moriarty/pull/137),
merge `1576682d24c291a5b086fdbc09a6c4f9c76c5680`, published six ADT source/map/
metadata files and one dated journal append. The twelve pure Build/Plan helper
bodies are 15,393 bytes, SHA-256
`a73951c5be2484ffa2ade77f8b33cffbf5b5b7894c04ca9797d545c850094681`,
with the designated ABI2 carriers. The default local CI summary records
230 PASS / 9.701 seconds; it did not compile or execute the new ADT source.

A separate 25-library-file SDK-free composition applies only five exact
owner-authored namespace/module spans. It remains uncompiled preparation.
Production `helper_machine` landing is separate: 19 files / 394,518 bytes are
prepared, but are not Root-integrated or compiled. A missing unsafe-code
prohibition was identified by designated security review and corrected by a
separate designated owner; final source audits remain pending. Neither the
publication nor the diagnostic composition establishes a ready production path.

**Status:** published, reviewed source and prepared composition; actual ADT
compilation, production integration and behavior remain pending.
**Proposed / validation needed:** name the actual consumer and changed dependency
closure at handoff, then record its own scoped result. A pure payload proposal,
a namespace adaptation and a default QA pass answer different questions.

### DW-22 — Correct the declared format without activating its consumer

**Observation/evidence:** the strict QA09-r3 source corrects one declared
structural `RunIntNat` carrier to use `n`; the other four declared formats use
`value`. Fresh Muse source review agrees within scope. The v2 consumer remains
inactive and no actual v2 QA comparison is established by that review.

**Status:** applied source correction and scoped source review; activation and
actual comparison remain pending. **Proposed / validation needed:** bind each
carrier spelling to its exact declared format, preserve the earlier defect, and
keep source review, transport checks and authorized actual comparison distinct.
Do not generalize one format's repair across the other formats.

### DW-23 — Preserve a failed preflight even when the later process succeeds

**Observation/evidence:** Root recorded a failed ordered-list preflight assertion:
the unordered source-pin sets matched, but the ordered lists did not. The outer
tool call printed the failed command's output and then incorrectly awaited the
dependent compile/run instead of stopping. The required new Root preconditions
and grant had not been written. Root explicitly owns this dispatch error.

The separate planned source05/v2 host attempt nevertheless completed: actual
16 child exits were zero and its receipt reported success. Passive inventory
recorded 62 regular files and eight directories. Those completion facts do not
establish admission. The single ordinary attempt remains consumed, with no retry
and no retroactive grant. Reliance, QA, publication and artifact reuse remain on
hold; native authority remains zero and C4, limits and counters remain unchanged.

**Actual review disposition:** the designated authorized Muse security fallback
completed and confirmed **UNADMITTED / HOLD**, no retry and no retroactive grant.
Its disposition permits only limited prospective passive incident read, hash,
framing and inert preservation, and requires staged full-capture actual-result
security review before reliance. The independent Muse
nonsecurity review records factual completion and abstains from security
judgment. Both read their complete supplied source packet, but runtime stdout
was supplied as inventory only. No complete capture-content review, semantic
comparison or permission to use the result follows. Root's subsequent recorded
disposition adopts only the limited prospective passive preservation, read, hash,
inventory, structural JSONL framing and staged-review preparation. The attempt
remains **NOT ADMITTED / HOLD**, with no QA, result reliance, publication,
comparison, retry, retroactive grant or artifact reuse. The admission directory
remains empty. The earlier handoff's unadopted label remains historical metadata;
it is not the current Root disposition.

Both reviews also corrected an earlier materialization record: a recorded
ordered tuple-equality claim was false, although unordered set equality held.
The original record is preserved with its correction. The underlying incident
facts, Root disposition, full reviews and retained outputs remain unpublished.

**Status:** observed unadmitted execution and completed scoped incident reviews;
result reliance remains held. **Proposed documentation improvement / validation
needed:** report preflight failure, actual process terminal, packet coverage and
admission status separately. Preserve the failed precondition and corrected
facts alongside successful process exits. Any execution-control repair or further
security disposition belongs to the designated owner; this log supplies neither.

### DW-14 update — Preserve current provider failure and actual fallback identity

**Observation/evidence:** a new Opus storage-source request exited 1 with a
session-limit failure before a model identity or substantive result was returned.
The standing user-authorized Muse security fallback was used for the incident
review above. Existing in-flight Opus work retains its actual identity until its
own terminal result; historical Opus and fallback records are not relabelled.

**Status:** observed request failure and completed scoped Muse incident review.
A separate fallback storage-source author request is live with no code result
claimed here. **Proposed / validation needed:** update availability from each
actual terminal, retain failed requests as nonvotes, and record the actual
fallback identity and scope. One request's failure does not establish provider-wide
health or change the standing fallback policy, native limits or acceptance gates.


## 2026-10-17 UTC — sixth edition

The [fifth edition](https://github.com/CharlesHoskinson/Moriarty/pull/139),
merge `06f7ff03a245e0ab6fae271f09cd75bf813f0330`, remains unchanged above.
These observations record exact source and result scopes at this freeze;
no new factory automation, security recommendation or execution authority is
created by this log.

### DW-24 — Record prospective admission and the actual compiler read set

**Observation/evidence:** [PR #138](https://github.com/CharlesHoskinson/Moriarty/pull/138),
merge `64fc52ca4b323b2ea5b537bc88470601961a3612`, published the production B
library source. Root then inspected a structured preflight exit 0 with all
required checks true, wrote the prospective grant, and invoked the separate
metadata operation once. Both actual children exited 0: the pinned Rust 1.99.0
version query and library metadata/dep-info compilation. The compile child took
0.516 seconds and retained six warnings; this is a child duration, not an
aggregate timing or performance benchmark.

The normalized compiler read set was exactly 24 files: 23 real canonical A+B
production files plus a 67-byte public-module wrapper. The historical consumed
source commit was `06f7ff03a245e0ab6fae271f09cd75bf813f0330`. Non-test module,
name and type closure compiled; 18 cfg(test) modules were excluded. Fresh distinct
Muse security-fallback and nonsecurity actual-result reviews accepted only the
scoped metadata result. The curated result selection and its publication review
are separate from that acceptance; this entry records no result publication.

**Status:** published production source, properly admitted observed metadata
compilation and scoped actual-result reviews. Cargo/whole-crate build,
cfg(test) compilation, runtime, semantic correctness, correspondence, SDK/native,
circuit, proof and stagenet acceptance are not established.
**Proposed documentation improvement / validation needed:** retain structured
preflight status, prior grant, actual child terminals and normalized consumed-file
membership together. Keep wrapper files and excluded modules explicit; refresh
only the evidence affected by later source changes.

### DW-25 — Keep test-source counts separate from executed tests

**Observation/evidence:** [PR #140](https://github.com/CharlesHoskinson/Moriarty/pull/140),
merge `92ca233157fc320ed7c9f34750803a7437f4a057`, published the production
cfg(test) append and its coverage README. The exact two merged files and unchanged
production prefix were verified. The source has 21 nonignored cells, while the
mandatory N01 cell remains ignored and unimplemented. At that publication freeze, all new Rust test
cells were **UNCOMPILED / UNEXECUTED**. The default ordinary CI summary records
230 PASS / 9.379 seconds; it did not compile or execute those cells.

**Status:** published test preparation and retained mandatory gap. A separate
cfg(test) metadata proposal, actual source-caller work, N01 difference analysis
and the remaining circuit-rest layout work are ongoing; none is complete here.
**Proposed / validation needed:** report each cell's implemented, ignored,
compiled and executed status independently. Bind later checks to the exact test
source and caller; neither a test count nor ordinary QA closes an unrun cell or
waives the mandatory N01 obligation.

### DW-23 update — Lossless staging preserves evidence, not admission

**Observation/evidence:** Root independently reconstructed all 13 held incident
streams byte-for-byte from their staged representations. The recorded designated
security and independent nonsecurity dispositions support structural retention
only. Root adopted the inert hash, byte, framing, record and source-pin facts,
closing the stated reconstruction condition. Staged review coverage remains
explicit; this is not a claim that one context read all raw captures and the
complete engine together.

**Status:** observed exact roundtrip and adopted structural-retention facts. The
original attempt 60460 remains **UNADMITTED / HOLD**: no QA, comparison, result
reliance, publication, retry, retroactive grant, admission or artifact reuse.
The separately admitted production metadata check in DW-24 changes none of those
restrictions. Native authority remains zero and C4, limits and counters remain
unchanged. The underlying retention and roundtrip receipts are unpublished.

**Proposed documentation improvement / validation needed:** bind any staged
summary to its original immutable bytes and retain each review's coverage limits.
Report a successful reconstruction as reconstruction; further reliance or
comparison remains dependent on its separate designated disposition and exact
independent expectations. No expected observations or held captures are selected
for publication by this log.

## 2026-10-17 UTC — seventh edition: diagnostics and literal evidence

Earlier editions remain byte-for-byte unchanged. This bounded consolidation of
iteration notes records observed practice and proposed improvements; it implements
no factory automation, security design, guard, policy or execution authority.

### DW-26 — Exercise the real API and isolate the failure being tested

**Observation/evidence:** ordinary diagnostics supplied evidence that source
review could not: an exact 79-file metadata composition compiled; a separate
Core operation compiled/linked and passed 47 unit tests. Their scopes remain
distinct from independent semantic comparison. The published
[production codec roundtrip](../docs/audits/2026-10-17/sherlock-production-codec-roundtrip/result.json)
in [PR #153](https://github.com/CharlesHoskinson/Moriarty/pull/153), merge
`6c9c7cd3836e9b4bc0ffb0288e83666031393e29`, built the real Core library,
linked an external example and observed two canonical roundtrips. That internal
encoder/decoder agreement is not an independent oracle or native witness.

Source reviews then missed an incorrectly isolated producer capacity test:
its expectation selected the wrong K/KF failure order. Two owner-specified typed
assertions against real `prepare_a` passed, confirming production Core's correct
source-derived order. This did not execute the producer or change Core. The
capacity reproduction and earlier diagnostic receipts remain unpublished.

**Status:** observed scoped compiler/unit/API/reproduction results.
**Proposed / validation needed:** inspect real consumers and dependencies early;
use eligible ordinary diagnostics for type/composition questions and isolate the
intended failure before asserting its typed error. Retain separate source,
compiler, unit, independent complete-output/prefix and native/target predicates.
The implementation/result owners must validate later changed bytes and remaining
semantic obligations; these observations are not a controlled performance study.

### DW-27 — Check literal APIs and compute membership from actual files

**Observation/evidence:** a literal preflight of an unadopted native-adapter
candidate found unavailable type/import assumptions, inaccessible APIs and
model-authored provenance rows that disagreed with acquired public-source pins.
One claimed source was absent from the original packet; later acquisition also
disagreed with its claimed hash. These are source/provenance findings, not
compiler errors, a security verdict or an adopted adapter. The factual witness
and provenance checks are retained privately; no original review body is selected.

**Status:** observed factual preflight; candidate readiness remains qualified.
**Proposed / validation needed:** bind actual declarations, visibility and caller
signatures before integration. Compute membership and hashes mechanically from
immutable bytes; model-written hash rows are claims, not computed membership.
Use exact OLD/NEW spans for bounded corrections and preserve failed assertions.
The source owner must supply corrected bytes and the applicable fresh checks.

### DW-28 — Distinguish complete local packets from received review coverage

**Observation/evidence:** a locally complete 206,846-byte inline ordinary data
review returned a complete final answer but reported a 15,265-byte tail truncation.
Its qualified data judgment did not establish full generator/final-metadata review.
The coverage receipt retained that limitation; the underlying private data and
review body remain unpublished. Other recorded reviews completed their tool reads
yet misstated literal APIs, so read counts alone did not establish report fidelity.

**Status:** observed transport/assessment limitations, preserved with the results.
**Proposed / validation needed:** after repeated factual failures, change to
smaller complete lossless units, explicit numbered read scope and literal
signature witnesses. Recover missing permitted ranges and report any remaining
tail gaps. Verify report claims against source separately from tool-read receipts;
neither local prompt size nor a full final answer proves exhaustive assessment.
Review owners must retain actual terminal, read extent and dissent without
transferring a scoped judgment to unseen files or independent runtime comparison.

### DW-29 — Preserve partial terminals and correct false diagnoses before edits

**Observation/evidence:** one producer author exited CLI zero while its provider
ended with an output-limit error. Three complete blocks were extracted unchanged;
an incomplete duplicate was excluded. A separate three-file parser pass did not
establish typechecking or runtime behavior. In another retained operation, N07
reported individual `ok` while N08 had no terminal result before the aggregate
timeout; the clarification preserved both facts without a two-test/full-suite pass.
A suspected newline defect was independently disproved by actual AST codepoints
and a synthetic parsing control before any source mutation or extra owner call.
These historical receipts and explicit corrections remain unpublished.

**Status:** observed extraction, parser-only check, partial result and corrected
diagnosis; no current successor outcome is inferred here.
**Proposed / validation needed:** report process exit, provider terminal, complete
extraction, candidate readiness and aggregate completion separately. Inspect bytes
or codepoints before repairing rendered escapes. Preserve the original failure
and dated correction; future claims require their own exact successor results.

### DW-30 — Keep local proof code distinct from versioned ledger acceptance

**Observation/evidence:** the acquired official ledger source at
`04c9c5d9bcebb8d4427d8589fb54d58a55599c14`
[checks transcript presence](https://github.com/midnightntwrk/midnight-ledger/blob/04c9c5d9bcebb8d4427d8589fb54d58a55599c14/ledger/src/verify.rs#L1132-L1160)
and returns `CallHasEmptyTranscripts` when both guaranteed and fallible transcripts
are absent. This is a pinned public-source fact, not an observed submitted
transaction or a new security judgment. Local proof/verification code alone does
not supply a qualifying ledger call or actual settlement evidence.

**Status:** observed public-source interface constraint; transaction/settlement
acceptance is not established by this entry.
**Proposed / validation needed:** pin the actual provider/ledger version and
adapter interface, and keep local prove/verify, transaction acceptance and final
settlement readback separate. The designated source/target owners retain the
missing integration and actual-result work. The first real stagenet transaction
remains the immediate priority within the full Sherlock obligations; no acceptance
requirement, native limit, counter or prior held-attempt disposition changes here.


## 2026-10-17 — source audit lessons

### DW-31 — Make exact source witnesses a prerequisite for API advice

**Observation:** the current B consumer is public in `crates/mori-core/src/sherlock/helper_machine/mod.rs:15`; `core_action_consumer.rs:76` declares `run_scalar_action`, which calls real source preparation, AST bridging, admission and owner execution. Meanwhile `docs/SHERLOCK-B-HOST-API.md:72` still says there is no production B consumer. The separately published [Core47 result](../docs/audits/2026-10-17/sherlock-core47-resource-result/README.md) records real ordinary tests, while leaving independent complete-state and full financial/native acceptance open. A dated API absence statement is no longer a reliable map of current source.

**Proposed:** every API claim supplies the exact declaration, visibility, import/reexport and a real caller at one pinned commit. Resolve `#[path]` explicitly: `mori_core::sherlock_source` is declared through `lib.rs:27–28` and implemented in `sherlock/source_adapter.rs`, not a same-named file. Inspect source read-only before proposing a correction. Distinguish “not present,” “present but private,” “public diagnostic callable,” “called by production CLI,” and “observed in an exact run.” Mechanically derive source membership and hashes; a model-authored row is a claim until checked against bytes.

**Status / validation:** observed source/document mismatch; factory enforcement proposed. Update the dated document with a current scoped note, preserve its historical 24-file metadata result, and have the source owner review the exact declarations and affected caller closure. A source check does not establish a compiler or execution result.

### DW-32 — Deliver complete review inputs in explicit pages

**Observation:** [DW-28](../wiki-llm/doctorwatson-improvement-log.md#dw-28--distinguish-complete-local-packets-from-received-review-coverage) records a locally complete 206,846-byte packet whose review reported a 15,265-byte missing tail. Complete local assembly and a complete final answer did not establish complete received context. The same log records factual API mistakes after successful tool reads.

**Proposed:** after truncation, split a packet at whole source/document boundaries. Each page states page number/total, immutable membership, exact byte or line coverage and digest. Separate independent oracle inputs from implementation-author packets. Finish with a coverage matrix naming pages actually read, missing ranges and literal declaration witnesses used in each material claim. Recover permitted missing pages before making an exhaustive judgment. Record both the provider terminal and the review's declared read extent.

**Status / validation:** proposed delivery discipline built from a recorded transport failure. This audit used bounded source reads and generated source-located graph facts; it did not re-review held/private inputs. Successful page delivery still needs factual claim checking and cannot widen the review's semantic or execution scope.

### DW-33 — Record a result lattice, not a single completion flag

**Observation:** the [145-test result](../docs/audits/2026-10-17/sherlock-production-normal-tests/README.md) concerns a standalone 72-source composition. A [later attempt](../docs/audits/2026-10-17/sherlock-normal-n07-n08-timeout/README.md) reports N07 individually `ok`, N08 without a terminal result and aggregate timeout after 30.001 seconds. Core47 is another cohort. The [external codec result](../docs/audits/2026-10-17/sherlock-production-codec-roundtrip/README.md) establishes two trusted roundtrips with shared codec logic; it is not an independent semantic oracle.

**Proposed:** represent source present, metadata compiled, linked, selected test terminal, aggregate terminal, independently compared complete observations, layer correspondence, native proof, ledger acceptance and settlement readback separately. Attach each to its candidate, attempt, profile, exact selected membership and original evidence. A historical pass never transfers to changed source. Keep observed partial terminals and consumed failures visible; do not sum unrelated test cohorts into a parent percentage.

**Status / validation:** existing reports already preserve these distinctions; a consistent factory presentation remains proposed. Verify the new presentation against the literal selected result files and exact stdout, including explicit missing rows. Retain N08's missing terminal without inventing a retry or new budget. Complete-state/normal-O, full B/ADT/finance, formal/circuit and target obligations stay open.

### DW-34 — Refresh the graph by changed source closure and explicit meanings

**Observation:** `.obsidian/graph.json` contains only viewer settings. The existing substantive graph is the dated `docs/audits/2026-10-14/knowledge-graph.json.gz`, schema `moriarty-combined-context-graph/1`, pinned to `cb8280ef54082853dd8bb6c6a52050328d0fc6fa`. Its 46,421 nodes and 57,298 parallel-preserving links contain no Sherlock or DoctorWatson match. The semantic companion likewise has none. Large graph counts therefore do not establish current Sherlock coverage.

**Proposed:** add a bounded Sherlock overlay with source hashes and `source_location`; label direct extraction, inference and unresolved interpretation as `EXTRACTED`, `INFERRED` and `AMBIGUOUS`. Refresh module exports, `#[path]` inclusions, cfg(test) membership and real call sites structurally. Refresh profile meanings, input domains, independent oracles, exact historical result scopes, missing consumers and blocker-to-requirement links semantically. Preserve parallel relations and original pins. Do not stamp the untouched historical graph as current merely by adding a delta.

**Status / validation:** a proposed compatible delta and read-only integrity facts accompany this audit. No canonical graph was replaced, no global graph build ran and no external provider was invoked. Before applying, check source/base hashes and every endpoint; after applying, preserve scoped provenance and invalidate affected stale meanings. The delta is not a complete `graph-ready.json` bundle for `build_task_context.py`, whose inventory and digest gates must remain in force.

### DW-35 — Choose the next useful end-to-end path while preserving the full target

**Observation:** the public scalar B consumer now reaches the actual owner; the default CLI still calls Core `simulate` at `crates/mori/src/commands/simulate.rs:126`. Scalar source preparation explicitly does not execute ordered effects. The [correctness loop](../docs/SHERLOCK-CORRECTNESS-LOOP.md) requires independent complete states/prefixes across declared K, Quint, Lean/Clean and handwritten domains. A public-input copier, resource report or a finite normal test pass does not supply the missing financial/proof/settlement path.

**Proposed:** after reconciling the five audits, select one existing requirement and source-to-consumer slice with an independent input/oracle owner, a real consumer, a falsifiable full observation and the first concrete blocker. Source preparation for the transition/public-statement/native boundary can proceed within its authorized scope; execution stays separate. Compare complete prefixes with explicit profile/clock transport before optimizing. Retain the complete language, generic data, finance/lifecycle, composition, formal, resource, native and qualified-target requirements in the completion roadmap. Prioritize the already requested first real stagenet path where eligible without presenting its narrow first transition as full interpreter completion.

**Status / validation:** next-sprint recommendation, not implementation or a replacement queue. Root should combine source, normal evidence, requirements and designated security-owner findings, then name the next observable capability and dependencies. This audit changes no native authority, C4 disposition, counter, terminal attempt, security ownership or public developer permission.


## 2026-10-17 — actual consumer sprint lessons

### DW-36 — Finish the actual consumer increment once, then move downstream

**Observation:** S1 now has actual source-to-consumer evidence: the existing `run_host.py --realization stream` compiled the real `Machine::visit_steps` path, ran eight Rust tests and compared all eight normal programs/216 complete runs with independently frozen expectations. Its 26 source pins match before, snapshot-after and checkout-after; this audit rehashed the same 26 current W/snapshot pairs. The source observer retains records, so no memory/allocation improvement follows. The separate integrated ordinary CI reported 241 tests OK; those are Python regressions, not another Rust/model/native result.

**Evidence:** `checks/stream-actual-01/{result.json,receipt.json,comparison.json,logs/run-tests.stdout}` and `checks/integration-ordinary-01/{RESULT.json,SOURCE-MANIFEST.json,stderr}` under the October 17 review runtime. The seven QA afterimages match current W; they are additional/current inputs beyond the original graph audit's 39 pins. Only `docs/DOCTORWATSON.md` changed among those original 39. Base commit/tree remain `628615a94d8a74837a1af510293927bb74d5c2b5` / `6b7f5964f7de2a711b820af011f5df2be0c59ecc`; current dirty QA afterimages need their own hashes.

**Proposed:** mark the exact S1 and ordinary-regression predicates observed, preserve all higher-layer gaps, and advance the next real consumer dependency. Do not redispatch unchanged 241-test or 216-run work solely to refresh presentation. Reuse the existing runner; no new shell mode, daemon or scheduler is needed.

### DW-37 — Preserve a failed check and continue from its precise cause

**Observation:** S2's host compile exited 0. Its following literal dep-info spelling check failed on one retained `../../../../models/sherlock/literal-step/MAPPING.md` include path, leaving the initial RESULT as FAILED. A separately recorded continuation normalized paths to exactly the ten frozen members without changing source/membership or recompiling that host artifact. It then linked the client, captured the real A102→A80 producer and passed named and ordered comparisons of the SAME 333 noncontext carriers. The continuation records 117 unchanged pins. The independent ordinary result review approved this narrow publication scope.

**Evidence:** `checks/composed-actual-01/RESULT.json`, `continuation-01/RESULT.json`, both comparison stdout files, and `01-implementation/composed-actual-result-review.md`. Named/ordered counts are 86 Program +81 Init +81 intermediate +81 Done +4 envelope. The two context limbs were absent and unbound from the agreed interface; extraction did not trim them from a 335-lane statement.

**Proposed:** retain both result identities and distinguish compiler outcome, path-set postcheck and continuation terminal. Diagnose exact bytes/path normalization before rerunning a successful compiler. Record reused artifact identity explicitly. Report two views of 333 as 333, not 666; no checked335/context, constraint, native, proof or target result follows.

### DW-38 — Resolve stale blocker status with later evidence before proposing another run

**Observation:** the later optimized147 operation already compiled its standalone 72-source cohort and passed all 147 selected tests, including terminal `ok` for every N03–N08 all-frozen-prefixes-and-cuts test. The earlier unoptimized two-test attempt still timed out after N07 individually `ok` and no N08 terminal. These are different binaries/selections. The old graph/lesson's “N08 missing” correctly describes that older attempt but is insufficient for today's next-work selection.

**Evidence:** `sherlock-parallel-20261016/b-normal-optimized147-operation-preparation-01/ROOT-OBSERVED-RESULT-01.json` and its original `candidate/optimized147-test-attempt-01/logs/test-harness.stdout`. They record compile 2.774 seconds, selected harness 3.834 seconds, 147 passed/0 failed/0 ignored. These are cohort timings, not an isolated N08 benchmark or speedup ratio. The result does not qualify current Core/full B or downstream layers.

**Proposed:** query later exact-scope results before opening a new optimization or missing-test task. Keep the original failure unchanged and add a later-evidence relation. An already successful exact selection is not next work unless changed inputs, a reproduced defect or a new required predicate justifies a fresh check.

### DW-39 — Freeze the design/specification before rearming the cleared goal

**Observation:** Root reports PR #168 merged at `011284561884346412a7c7eaa36a195310924878`, 17:38 UTC. This is source publication, distinct from the later dirty QA afterimages and ordinary host observations. Five Astra HIGH loop reviews were conditional; their earlier source/result statuses require reconciliation against subsequent S1/S2/CI receipts. At this handoff, final design03 still awaits three Astra/Gemini integration outcomes. The user requests rearming AFTER final design and specification; this pass does not rearm it.

**Proposed:** integrate current result pins, retained dissent, exact finite next batch and requirement exits into the final design/specification, then record Root's actual adoption/rearm action separately. An unchanged blocked dependency must not become a recurring task. An empty ready queue is not full completion. Keep native authority 0, C4/open gates, consumed attempts and all language/financial/formal/proof/qualified-target/release requirements intact.

**Status:** source/evidence reconciliation and bounded next-path recommendation only. Root-supplied merge/pending facts are explicitly distinguished from independently inspected runtime receipts; this agent made no GitHub/provider query or runtime goal mutation.


### Current integration disposition — October17

The stream realization/helper closure/source-read reporting are applied; [actual241 ordinary checks and stream8/216](../docs/results/sherlock-stream-20261017/README.md) and [actual333 same-carrier comparisons](../docs/results/sherlock-composed333-20261017/README.md) have retained scoped results. The old unoptimized N08 timeout remains historical; the later Optimized147 cohort passed all selected suites. The [graph refresh](../docs/audits/2026-10-17/sherlock-knowledge/manifest.json) preserves the October14 baseline and adds bounded Sherlock observations, without global freshness/native qualification. Five AstraHIGH final reviews support ordinary implementation under the [revision03 loop](../docs/sherlock/implementation-loop.md), preserving native blockers. The next source increment is the real public compiler preparation/admission path; further factory rebuilding is not a prerequisite. These notes do not themselves arm a goal.

### DW-40 — Separate cold dependency build time from the compiler diagnostic

The first B04 Cargo check timed out at30seconds while compiling normal dependencies. The next production rlib build timed out at180seconds after dependencies completed and core compilation began. Both results remain intact. Root then reused only completed normal dependency artifacts: a focused full-core metadata check passed in28.7567seconds and direct rlib code generation passed in45.6020seconds. All113 source pins were unchanged. The external harness compiled in3.2433seconds and all seven positive tests passed, including two actual source-to-B compile-only preparation/admission cases. No SDK target or owner.run was selected. These are cohort timings, not a benchmark or speedup ratio. See the [exact result](../docs/results/sherlock-source-compile-20261017/README.md).

After repeated timeouts, isolate the unfinished phase and bind the artifacts that actually completed. Preserve failures, exact profiles and inputs; do not keep recompiling the cold dependency prefix to obtain the next diagnostic. Successful typing and normal tests remain distinct from guard acceptance: the focused owner review requested two concrete preflight/diagnostic corrections, now being authored before merge.

## DW41 — Ground source-proposal coverage names in actual definitions

Observed October17: the R33 native-prerequisite author proposal returned one README old-string that matched no pinned source paragraph and62 test-name tokens absent from the supplied definitions. The actual eight new test method names exist; source existence alone does not prove coverage. Mechanical readback withheld final materialization rather than guessing a replacement. Improvement: supply a closed definition catalog and exact source bodies to the designated owner, separate executed predicates from source-audit/unestablished rows, and require literal patch applicability. Status: one owner correction is in progress; no R2/native acceptance follows from these facts.

## DW42 — Bound provider calls and preserve transport failures honestly

Observed October17: the first compiler-guard author call encountered two internal AGY stream-idle retries and produced no source; the next timed out without source and its nominal600second outer bound overran. A narrowed inline-source diagnostic task later returned an exact1896byte owner edit response successfully. The A continuity reviewer also needed a corrected local duration argument/stderr transport after an initialCLI2 with no model initialization. Improvement: distinguish manual invocations from backend retries; use documented duration units, consistent text/binary capture and an independent OS timeout; preserve actual elapsed time, missing diagnostics and no-vote outcomes. Status: the changed inline method produced one exact diagnostic source correction; fresh audit/CLI execution and the separate preflight fix remain open. No native limit or counter changed.

## DW43 — Merge independent product slices separately

Observed October17: the A complete-state continuity comparator passed48 focused tests and production metadata checking, received a fresh ArgonHIGH scoped non-author PASS, and merged through PR171. B complete typed observation source and four test functions also typecheck in separate PR172; their runtime tests remain pending scope review. Improvement: isolate dependencies in Git so the compiler's open guard findings do not hold reviewed independent observer work. Status: A merged; B and compiler remain draft. None closes full SCON-007, formal, native or stagenet acceptance.


### DW-44 — Read test selectors from typed fields and retain local checker failures

**Observation:** the B observer binary listed exactly four approved tests. Root's metadata checker compared those names with argv index4, which was `--exact`, and stopped before any product test ran. The corrected check used the owner's explicit selector list and retained the existing list output. The same sealed binary then passed all four exact tests, without repeating compilation, listing or tests.

**Evidence:** [actual tests and the original checker erratum](../docs/results/sherlock-b-complete-observation-20261017/README.md). Source approval, the four-test execution classification and actual-result review remain distinct. This is a local metadata-reader defect; it did not change product source or the classified execution bounds.

**Improvement:** use named selector fields or parse the value after a known flag when command metadata is needed. Check original command exits separately from later local assertions, preserve both outcomes, and continue from the precise cause when the already produced artifact remains valid. Passing test counts retain their exact source and selected branch scope. No new runner or execution policy is proposed.


### DW-45 — Check imported namespaces as well as local module APIs

**Observation:** the repaired prerequisite test source passed a direct guard-export/arity scan, but still called `mock.ExitStack()`. A separate Python3.14 namespace probe found that `unittest.mock` has no `ExitStack` while `contextlib` does. The designated owner supplied the import/call correction in a small follow-up; the eight test names and assertion counts remained unchanged. These are source and namespace facts; the prerequisite tests have not run.

**Evidence:** runtime `R50-one-file-test-api-repair`, `53-r2-integrated-source-candidate/STDLIB-NAMESPACE-READBACK.json`, and `R50-namespace-api-fix-01`; the final test afterimage is `4a91f6ffec061c78564d2bfbf6a13c2b5750d690d4570577e1ebfa5909273e54`. The driver and test repairs remain separate from an execution classification, R2 adoption and native permission.

**Improvement:** enumerate imported modules used by a new test fixture alongside the local target module. Verify an unfamiliar namespace against its installed source or a read-only namespace probe before running the suite. Keep syntax/API scans, unchanged assertion inventories and actual behavior as separate evidence. This adds no new gate, runner or execution authority.
### DW-46 — Include module visibility in function-level source tasks

The compact preflight author returned applicable source edits, but the first
production metadata check failed with three private-module import errors.
Supplying the module declaration and public consumer exports let the owner
return a two-file import/re-export repair while preserving the private module
and guard bodies. Production metadata and all five new regression functions
then typechecked. These are authored/compiled source results, not fresh review
or executed regression results. [Exact attempts and limitations](../docs/results/sherlock-compiler-preflight-20261017/README.md).

Include the relevant module boundary with full changed function bodies. Save
each child result before subsequent collector assertions: the integrated-tree
check emitted metadata but lost its child exit/timing values after a lexical
path comparison failed. The normalized dep-info matches pinned files; the
missing values remain unrecorded and the unchanged compilation was not retried.

### DW-47 — Check review facts independently of model completion

The B pairwise result reviewer returned provider SUCCESS and claimed approval
after 101 recorded page reads, but invented material types, JSON fields, sizes
and hashes. Root rejected that approval and held the merge. The request carried
561,839 bytes across 72 units; that oversized context is an observed task
property, not a proven cause of the factual errors. [Sealed failed disposition](../docs/results/sherlock-compiler-preflight-20261017/64-b-pairwise-actual-result-review/REVIEW-DISPOSITION.json).

Keep raw reports and contradictions. Calculate literal membership, counts and
hashes with inspectable code, then ask focused questions about source behavior
and evidence limits. A smaller packet is a proposed improvement awaiting an
actual reliable review; it does not automatically rehabilitate the failed vote.
Provider success, read receipts, authored code, actual tests and accepted review
remain separate observations. No security gate or model assignment changes.

### DW-48 — Make current requirement closure a factory output

The ordinary factory checker reported corpus consistency and execution-layer
gaps, but did not consume current per-requirement acceptance evidence. The frozen
design traceability record still labelled all 35 original requirements open;
those historical labels were not a current implementation census. This made
passing source and test batches difficult to reconcile with actual EARS closure.

The user directs the existing loop to close retained requirements. Add a current
evidence sidecar to the existing checker, preserving the frozen design bindings.
Keep implementation, observed validation, owner-assessed acceptance and computed
source/evidence currentness separate. Include the original 35 clauses, 25 additive
features/68 scenarios and linked retained families. A changed supporting source
must remove a prior declaration from the current closed count; matching hashes
do not establish semantic acceptance.

The first ordinary regression ran on October17 at21:58:48UTC and failed because
the actual checker lacked the `requirement_evidence` interface. Its copied source
remained unchanged. The regression describes a synthetic closure followed by a
source edit; it is not evidence that an actual Sherlock requirement is closed.
Implementation, green verification and fresh review follow this preserved red
result. Select the next missing requirement behavior or exact acceptance scenario
after the repair; do not build a replacement scheduler or repeat passing cohorts.

### DW-49 — Close clauses from their own evidence and check report consistency

The updated default factory passed 251 local tests and verified the complete
declared requirement inventory. Fresh review and Root adoption then closed
UIREQ-001, UIREQ-010 and UIREQ-011 on their supported typed A host scope, using
unchanged generic machine source and independently checked historical fixture
observations. The selected report shows three current adopted closures; the
historical pre-adoption result still shows zero. Other original clauses,
refinements and linked retained requirements remain open.

Review found three schema checks missing from the reporter: a nonempty evidence
class array, the required adoption enum and a retained-family disposition string.
Root reproduced all five malformed-declaration cases, applied the three checks,
and observed all eleven focused regressions pass. The final default factory
passed 252 local tests and retained exactly three current closures. A reporting
pass remains distinct from acceptance of its open requirements.

The review also caught an imprecise pending-frame locator and an overstated
Clean-stage implementation label. Exact factorial states 22/36 and list-sum
states 91/92/93 now support the closed predicates; UIREQ-026 remains partial/open
until its actual private-record and per-slice evidence exists. Use clause-specific
dependencies rather than making every host clause wait for stagenet. Preserve
failed runs and historical consumer versions when reusing their valid evidence.

See the [closure record and actual factory results](../docs/results/sherlock-ears-closure-20261017/README.md).

### DW-50 — Advance from passed scenarios to missing behavior

Nine independently frozen normal host cases passed, then fresh review and Root
adoption closed UIREQ-003/005/006/008 on the unchanged typed A host relation.
The selected DoctorWatson report verifies seven current adopted clauses, while
28 originals and all25 refinements remain open. The earlier252-test pass stays
attached to its earlier three-closure sidecar; no passing cohort was repeated.

A scenario pass does not settle a broader clause. UIREQ-004's nested arguments
now return8 in the required order, but generic constructor-field evaluation is
missing, so its next action is implementation of that behavior. Likewise all
five compiler preflight bodies have exact preserved2+3 passes; the queue moves
to public compiler qualification and recursive lowering rather than re-running
the same five. Source analysis of checked operations is distinct from observed
boundary execution; the direct A overflow/underflow paths remain unobserved.

The fresh review's semantic descriptions were grounded, but its numerical source
ranges were inaccurate. Preserve raw reports and bind actual symbol anchors in
a separate mechanical readback. Do not silently correct a review or transfer its
scope to new native, generic, formal or financial requirements. See the
[closure and selected-report record](../docs/results/sherlock-ears-host-closure-20261017/README.md).

### DW-51 — Rebind compatible changes without rewriting execution history

Promoting the existing grouping API changed module/leaf source bytes while
preserving the entire advance_m body and machine relation. DoctorWatson correctly
rejected those changed dependencies and removed three closures from its current
count. Root adopted the independently reviewed compatible promotion explicitly,
kept the original consumed private module as an immutable archive, and bound the
current module separately. No old execution input was relabelled as the new one.

The first reconciliation also exposed five additive refinement locators that
still pointed at the old private leaf. Preserve that failed report, update the
complete35-original/25-refinement inventory, and check whole-file line spans
after removing source lines. The final selected report returned current with
seven adopted original clauses; all25 refinements remain open. No unchanged
nine-case or252-test cohort was rerun. The separate current production metadata
check passed after composition, while full current CLI/runtime claims stay open.

See the [composition and rebind evidence](../docs/results/sherlock-public-grouping-20261017/ROOT-COMPOSITION-ADOPTION.json).

### DW-52 — Distinguish provider delivery from actual review coverage

Three source-review requests accepted complete large inputs at the CLI boundary,
but each model reported omitted context and returned incomplete full-source
judgments. Preserve those abstentions; successful transport and provider status
cannot establish a full review. A changed delivery used a small source index and
62 actual bounded file reads of the same frozen public bodies. The reviewer then
attributed full coverage and issued a conditional source/policy opinion. Actual
tool metadata confirms the62 paths and summaries; it is not an independent byte
attestation of the model's internal context. Additional required votes and native
prerequisites remain open. Do not repeat unchanged oversized submissions or
convert one conditional review into execution authority.

### DW-53 — Adopt the clause without inflating the oracle

The original shared/distinct Leaf4 pair passed once, and a fresh functional peer
accepted original009. Root adopts it as the eighth typed A host clause. Both
visits are charged, but only31 of62 logical boundaries have complete independent
state literals. Preserve those gaps and the raw reviewer qualifications instead
of turning pairwise equality or source review into a universal theorem. Do not
repeat this unchanged passing case; select the next missing behavior. The
[adoption and exact result](../docs/results/sherlock-structural-sharing-20261017/ROOT-ADOPTION.json)
retain the historical consumed library separately from current source pins.

The selected DoctorWatson report is an integrity check over declared evidence.
It must verify all eight current adopted clauses and leave27 originals plus25
refinements open. Reporting gaps, compiler integration and native prerequisites
remain separate actionable work; an adopted host clause does not close them.

### DW-54 — Keep evidence joins honest and integration dependencies complete

The compiler companion reports historical source/library/artifact links; matching
a client file to a receipt does not verify its call route. A non-author source
read caught that distinction before the final implementation. Preserve the
original expectation freeze and explicitly freeze a qualified successor before
changing code, rather than editing expectations to fit observed output.

An optional sibling module then exposed an existing snapshot-membership failure:
two of254 local tests failed because the declared helper set omitted that file.
The repair moved the small pure function into the checker whose complete bytes
are already snapshotted. The detector and guard stayed unchanged. Four targeted
checks and then all254 local tests passed. Preserve the failed source and receipt;
this was an actual integration defect, not a reason to weaken dependency checks.

See the [source and actual CI records](../docs/results/doctorwatson-evidence-record-20261017/README.md).
The companion's synthetic fixtures and historical file reads do not execute an
interpreter or close the full original025 scenario. Model-only evidence and later
B observations still need their separate complete reporting record and review.

### DW-55 — Close reporting with complete records; keep capability acceptance separate

The complete historical C1–C4 model record supplies all seven reporting fields
and explicitly marks the nine new-B scenarios unexecuted by that historical cut.
A fresh review read all42 supplied pages and accepted the original reporting
scenario. Root combined it with the qualified compiler companion and its actual
local checks, then adopted original UIREQ-025. The selected requirement reader
now reports nine current adopted originals,26 open originals and25 open
refinements. The original clauses and scenario wording are unchanged.

The [reporting adoption](../docs/results/sherlock-model-only-evidence-record-20261017/ROOT-ADOPTION.json)
does not validate the capabilities named in those records. Historical source
identity stays historical; inferred consumer metadata and unchecked authenticity
stay explicit. The raw review exceeded its requested output length and is
preserved alongside the complete page-read evidence. No unchanged model or
254-test cohort was rerun to obtain this closure.

The [grouped recursive-return case](../docs/results/sherlock-grouped-return-20261017/README.md)
passed once across twelve fixed combinations and received fresh result review.
Full UIREQ-032 remains open. A postcheck incorrectly assumed dependency-target
order; the data-only correction preserved the original result and performed no
subject rerun. Reported dependency membership never becomes an I/O attestation.
Next work is the real recursive compiler and resource-report chunk behavior,
with complete independently frozen expectations and the existing consumers.

### DW-56 — Test the producer when the scenario requires a collected run

The A cumulative reporter compiled in the normal production library. Its fixed
supplied-data test passed all four independently frozen87-leaf reports, and a
fresh reviewer accepted the actual result and comment/documentation promotion.
The same review kept original013 open: literal supplied records are not a live
admitted computation collected as one trace and two chunks. Select that exact
missing producer check next; do not repeat the passed data-only test or claim
that four complete reports close the broader clause.

The [actual result and qualified adoption](../docs/results/sherlock-resource-chunks-20261017/ROOT-FEATURE-ADOPTION.json)
keep missing manifest identities/profile domains and unmeasured physical costs
separate. The snapshot contains116 source files; dep-info names63 normal source
dependencies. Neither number is a complete read-access attestation. New emitted
library/test hashes were collected only after execution, while input pins and
ordered invocation records bind the ordinary run operationally.

A220KB inline review abstained after reporting clipping. A smaller complete
request accepted the source; its successor reviewed the actual result. Preserve
that failure and stop assuming locally complete input implies complete provider
context. Verify delivered coverage, prefer scoped full call paths or verified
pages, and check factual citations mechanically. Do not use model-written
cryptographic strings as the authoritative source manifest.

### DW-57 — Count the selected set and validate financial fixture admission

The [live chunk result](../docs/results/sherlock-live-resource-chunks-20261018/README.md)
passed using two real Machine instances and complete independent states/reports.
The fresh result review accepted that limited observation and kept original013
open for manifest/profile gaps. It also reconciled a preparatory count error:
the named files enumerate460 pin occurrences and425 unique paths, exactly the
actual precheck set. The original483/442 assertion remains preserved. Compute
counts from the selected records, compare exact sets and preserve contradictory
summaries; never manufacture a missing-file list to fit a stated count.

The first financial-adapter test compiled successfully but failed before its
positive prefix. The source did not register either liability present in the
starting snapshot. Stage admission rejected auditDuty with attempted0/committed0.
The input repair adds an unselected declaration action; an independent author
must derive the changed complete Program/Plan and continuation identities before
another selected test. Keep the original failure, financial expectations and
source frozen. Do not weaken identity validation or copy actual output into an
expected positive result. This financial evidence remains unpublished at this
entry; no financial requirement is closed by it.

Two Root receipt-processing mistakes were separate from the product result:
relative names in different freezes collided, and include records used the
snapshot field rather than path. Both were diagnosed from actual metadata.
The latter interrupted processing after three successful compilations; those
compilations were preserved and not repeated. List and exact-test commands ran
once afterward, with input/generated-output pins rechecked. Inspect actual
record shapes and normalize declared relative paths against their own origins
before selecting children. These are observed workflow lessons, not new factory
code or a native authority change.

The selected requirement-report helper returns `current`; the CLI wrapper uses
`pass`. Root initially asserted the CLI word against the helper's valid report.
The report and post-read correction are retained without rerunning the reader;
no successful initial capture-cell exit or timing was invented. Read the actual
API/status contract before asserting wrapper metadata. The resulting report
keeps nine adopted originals current and all remaining requirements open.


### DW-58 — Adopt observed components and preserve the missing producer boundary

The [Core/2 financial component](../docs/results/sherlock-financial-observation-20261018/README.md)
now passes the independently revised start/false-await/true-resume comparison.
The exact original first-paid-prefix scenario is observed on that producer and
adapter. The general Sherlock financial extension still needs its own actual
segment producer and adapter relationship; UIREQ-017 remains open without adding
UIREQ-018 opcodes or a native prerequisite to its original clause. Distinguish a
useful implemented component from the complete consuming feature.

Canonical integration changed only include locations and module documentation.
The permanent integration test compiled once; exact assertion-body and all15
payload comparisons justify reusing the existing R176 runtime result. Its new
binary was not executed. Preserve the earlier invalid-input failure, original
expectations, corrected independent identities and each actual review. Eleven
standalone expectation files are compared completely; only two hash cells of the
large context fixture are consumed. File size is not observed-field coverage.

The public module registration adds three lines and changes no existing body.
Root explicitly adopts that compatibility, retains the original library input
under an immutable archive path, and updates five live dependency pins. The
selected DoctorWatson reader reports nine current adopted originals and no
findings. Rebinding is an explicit reviewed reuse decision, not a new run of the
old host scenarios or the unchanged254-test cohort.

### DW-59 — Bound review context and record authoring exceptions exactly

Repeated numerical pin inventories make reviews larger without exposing more
behavior. Supply complete changed functions, their actual consuming path,
independent expected values and actual command/output records. Factor repeated
inventories by exact identity and mechanically verified set equality; state which
source bodies were omitted. Keep delivery coverage separate from local packet
completeness. A locally complete prompt does not disprove a provider's clipping
report, and an empty or interrupted response supplies no approval.

The user separately approved Sol authoring for parser budgets and type-checker
depth limits, each with fresh Opus/Gemini review before candidate execution.
Record each narrow exception and retain other ownership. The bounded parser's
actual19 selected commands and14 passing tests are new evidence; checker source
and native provisioner source remain separate review tasks. No model quota,
missing response or author-time comment can change a native execution grant or
close an unobserved requirement. These lessons improve the existing factory;
they introduce no scheduler or new public developer permission.


### DW-60 — Separate public API compilation from feature consumption

The [typed word components](../docs/results/sherlock-word-components-20261018/README.md)
provide named32/64-bit carry, borrow, full-product and checked-result records.
Seventeen original external tests passed against their exact source. The new
normal Core export, external public consumer and canonical test location then
compiled separately. No passing case was rerun to change its provenance. An
exported API still needs an actual arithmetic-lowering consumer; original030
remains open with that missing edge named explicitly.

The existing evidence reader reports nine current adopted originals after Root's
explicit additive-export compatibility decision. The original execution inputs
and preceding financial adoption remain archived unchanged. A later library
registration does not rewrite earlier execution-time input checks into claims
about today's entire tree. All original and refinement status enums are retained.

Freeze review handoff files before Root binds them for execution. One checker
precheck stopped before launching any child when a literal-location record was
finalized after selection. Its historical selection and stop were retained; the
final metadata was frozen before the actual eighteen checker tests ran and passed.
Finalized records now use new revision paths for later corrections. Record a
metadata stop separately from a compiler failure or subject retry.


### DW-61 — Check the materialized public consumer and reuse observed dependencies

The [public draft compiler result](../docs/results/sherlock-public-draft-compiler-20261018/README.md)
records nine passing bounded-copy phases (seven exact tests) and five passing
public compiler phases. A normal current Core library compiled in 5.247679
seconds using the already observed R191 lexer; the passing lexer prefix was not
rebuilt. The canonical external public consumer then compiled and passed both
positive fixtures, 177-byte ChainTotal and 211-byte BranchTotal. Full equality
of all four sealed R115 fields includes the twelve program tables; the separate
26-obligation literal correspondence object and selected artifact metadata also
match. These consuming tests discriminate actual public integration from an
exported declaration or an isolated copy-helper pass.

An earlier source-only R207 basis did not itself establish that its six extra
copy extents and copy-error diagnostics had reached the materialized public
configuration/report path. Inspect that current path: all sixteen preprocessing
fields and the exact copy-error projection are now present. Preserve the older
source-only assessment and its integration gap instead of rewriting it as an
observed public result. The lost R219 response supplied no verdict. The fresh
current-source review read all 64 supplied pages/52 bodies and approved the
changed current source with retained coverage and host-resource limitations;
it is a new source judgment, not recovery of the missing vote.

Keep source review, observed result and requirement closure separate. Final
post-result review and original UIREQ-022 adoption remain pending at this cut.
Compile-only preparation observes no B runtime values 7/8, defined failure or
resource-prefix traces, CLI publication, semantic proof or native result. The
complete documentation file-writing example remains unrun. These receipts
improve the existing DoctorWatson workflow without another factory or a rerun
of unchanged passing prefixes.


### DW-62 — Verify review examples and close the exact requirement

The public draft compiler batch closed original UIREQ-022 after actual normal
Core compilation, seven new bounded-copy tests and two public preparation
comparisons. The final review had invented example obligation names and merged
owner-gap descriptions. Literal checking caught those errors; its same-owner
correction supplies the actual 22 construct mappings, four dedicated sorts and
all five retained owner gaps. Original reports remain verbatim. Adopt the actual
report contract and corrected judgment, never the inaccurate examples.

The existing requirement reader then reported ten current adopted original
closures, with all other 25 originals and 25 refinement features open. Exact
reviewed source and documentation compatibility retained earlier closures
without rerunning unchanged passing cohorts. Named semantic obligations are
still unproved. A report clause can close while the reported runtime, constraint,
proof and target work remains incomplete. No new factory or native authority
was needed for this reporting capability.


### DW-63 — Report and test the admitted call's actual configuration

A supported-profile leaf is useful only when the public artifact returns it and
it describes the config used for that admitted result. R254 wires
`DraftCompileArtifact.supported_profile` through
`draft_profile::report(&config.fields)` before moving that same config identity
into the artifact. It reports complete grammar/domain/schedule, all 22 logical
bounds, layout capacities, fuel/stack, all 16 preprocessing extents, extensions
and remaining owner admission work. Logical capacities do not measure physical
resource use or enforce public JSON size.

R256 actually passed five phases: normal Core compilation, public consumer
compilation, exact two-test listing and both new tests, with all 723 selected
inputs unchanged. Both calls use the same ProfileFragment source. One compares
the complete returned report with the frozen source-first literal contract; the
other changes all 22 bounds and selected layout/run/preprocessing fields before
calling the producer. Its full equality check discriminates this invocation's
projection from fixture constants or an unwired export. R257 approved the
supplied source and exact plan, retaining coverage/owner/H1 limitations. R263's
same-owner reply subsequently returned, but Root withheld its original UIREQ-002
and compatibility adoption. A successful provider terminal did not make its
fixture, grammar, format, 22-bound inventory, 26 mapping IDs, gap labels and
pointers faithful to the supplied source. The raw reply remains unchanged.

Check a bounded review reply against the actual returned JSON schema, exact
fixture, named mappings and owner gap text before adopting its recommendation.
Hashes, a familiar conversation and an approval-shaped conclusion cannot repair
invented fields or examples. Preserve and qualify the failed reply, then change
to a fresh small literal result review (R272), rather than asking the same owner
for a metadata rephrase or rerunning unchanged passing tests. This is a material
review-delivery correction, not a new public developer permission requirement.

The prior ten-of-35 adopted-original baseline includes original UIREQ-022.
Passing report tests are not a new UIREQ-002 adoption; only Root's later actual
adoption after result review would make eleven. Public developers need no
reviewer permission to inspect the report. The separate R258 runtime source,
R259 independent two-program expectations and R264 QA composition are READY /
UNRUN: intended values 7/8, recursive-call sequences and counter relations have
not become runtime observations. Complete independent B clocks/prefixes, H1,
semantic proof and native/target results remain separate gaps.

Tutorial/explanation behavior is unchanged because this batch adds metadata
without transition or circuit changes. The full guide's file-publication
example remains unrun; no new circuit or website iteration is claimed. Preserve
R261's earlier UNRUN cut and original receipts as history. These material
consumer checks improve the existing DoctorWatson workflow without rerunning
old passing cohorts or adding another framework.


### DW-64 — Preserve review coverage through compaction and rerun only the changed case

A compressed handoff or a continued conversation is not a fresh full-source
review. Preserve the exact candidate, complete relevant bodies, independent
expectations, actual results and delivered membership. When replacing a large
packet, show how any lossless edit map reconstructs the omitted source; do not
ask a fresh reviewer to infer code from earlier conversational summaries. The
[paired-checker review](../docs/results/sherlock-paired-model-qa-20261018/review/r253-revision02/REPORT.md)
used a fresh context with the complete current checker/test/driver and exact
old-checker reconstruction, then returned a new source/result/reader judgment.
A previous review's missed `uiReturn` versus canonical `uiRet` fixture literal
remains part of the record; compaction does not erase that mistake or promote a
prior summary into coverage of changed source.

The first actual ten-case QA run had nine passes and one failure. Correct only
the two mistaken fixture literals against unchanged K/oracle declarations.
Execute the formerly failing method once, and retain the other nine historical
passes with the fresh source-compatibility judgment. The corrected case passed;
there was no fresh ten-case run and no rerun of the unchanged 254-test cohort.
Keep the original failing output, correction, exact selector and resulting pass
separate. Never fill an absent producer result from matching supplied fixtures.

Preserve review findings through integration: missing-field error typing,
unavailable top-level raw context on a program-identity difference, and the
unexercised F03 loading branch remain NB-01/02/03. Actual F03 K/Quint execution
and original UIREQ-021 stay open. The two current checker-hash cells maintain
an existing reporting closure; they add no new interpreter or formal closure.
Root also restored the ledger's original byte encoding after a JSON serializer
changed unrelated Unicode escapes: equivalent decoded values do not satisfy an
exact two-literal byte-edit claim. Use the guarded pointer patch and additive
correction, not a whole-ledger replacement that could overwrite another lane.


### DW-65 — Check terminal status before adopting a review

CLI exit 0 and a complete visible approval-shaped JSON reply do not establish
a successful review. R265 returned terminal `ERROR` after a stream interruption.
Root initially missed that status and hardcoded `SUCCESS` in its selection;
[the additive correction](../docs/results/sherlock-supported-profile-20261018/ROOT-RUNTIME-REVIEW-STATUS-CORRECTION.json)
preserves that mistake and withholds adoption of the visible opinion. Existing
compilation receipts and the first pre-step capacity refusal remain actual
observations, with no successful runtime result inferred. Check the explicit
terminal status, errors, identity and readable coverage before selecting further
execution. A fresh independently classified input cut reuses the passed Core
build rather than rerunning the passing prefix.

R272 supplied a successful fresh literal-result review for the separate
supported-profile report. Root checked the exact fixture, format and all 22
bounds before adopting UIREQ-002. The [installed adoption](../docs/results/sherlock-supported-profile-20261018/ROOT-INSTALLED-ORIGINAL002.json)
and current evidence check record 11 accepted originals and 24 open. DW-63's
ten-original/pending cut remains historical. Report tests were not rerun to
repair a review reply, and no runtime, semantic proof or native closure follows.


### DW-66 — Preserve the failing input and verify the real public consumer

The public runtime path now has two observed host cases through its real source
producer and the fixed B owner. R279 passed consumer compilation, exact listing
and separate ChainTotal/BranchTotal tests, retaining the passed Core build.
The inspected independent tests assert actual UInt7/8, full ordered tables,
recursive call order/counts and relational callback/counter/resource conservation.
Their raw stdout records libtest status, not complete payload traces, exact B
clock totals or a semantic proof. R281/R286 successful scoped reviews and Root's
exact three-source/seven-test installation remain distinct from those receipts.
The result adds no original EARS adoption to the eleven-original baseline.

A first input refused before stepping because value-slot capacity1024 was below
fuel4096. Preserve that refusal and the unrun Branch case. Independently derive
and freeze the new input's value_slots4096 from the existing owner condition,
update its exact config-hash literal/authorship metadata and review that changed
operation. Do not fit semantic expectations to a desired output. Root also
preserved its mistaken ordered comparison of the alphabetic test list; correcting
set/count membership did not rerun the passing compilation/listing prefix.
R265's complete visible opinion still had terminalERROR and remains unadopted.
The fresh successful review and later result do not erase the old terminal fact.

Use actual consumer evidence to update living tasks without closing their parent
prematurely. B04's initial bounds/diagnostic gap is superseded by the installed
public path, but its real CLI/optional-observe and file publication remain unrun;
all B04 checkboxes remain open. The guide's complete I/O example remains unrun.
Fifteen synthetic SDK-free mocks are separate interface-informed evidence with
R2UNSATISFIED/native0 retained, not a provisioning or native result. No new
transition/circuit semantics or circuit/website iteration is claimed. Append a
new dated journal entry and preserve previous pending cuts instead of rewriting
history. These concrete corrections improve DoctorWatson without a new factory
or a rerun of unchanged passing cohorts.


### DW-67 — Keep synthetic decoding and mock results in their actual scope

The corrected K adapter preserves its default64 capture bound and admits up to95
only through an explicit keyword selection. Five exact offline cases passed,
with literal source hashes and all five inputs unchanged. The supplied F03 test
uses selected boundary states and placeholder intermediate states with empty
engine artifacts; comparison_ready stays false. Preserve that qualification
when installing the decoder. Neither that pass nor a ready command plan is an
actual K capture or paired model comparison. The unchanged21 N01 methods were
not rerun. See the [decoder result](../docs/results/sherlock-k-f03-offline-decoder-20261018/README.md).

The [native prerequisite mock result](../docs/results/sherlock-native-prerequisite-policy-20261017/285-sdkfree-mock15-source-result/README.md)
passed15 synthetic SDK-free tests after the designated reviewer checked the
complete source and missing packet tail. Its exact label remains
INTERFACE_INFORMED_NOT_BLIND_R2_UNSATISFIED. Root publishes co-located source/test
snapshots in the result archive, keeping them out of default test discovery.
Passive publication preserves the original execution cwd and does not select a
new invocation or establish provisioning, native build, R2 or C4 closure.

Source judgment, prospective operation classification, captured execution and
actual-result review remain separate records. Readable complete input and an
explicit successful terminal are checked before adoption. This iteration adds
no new original EARS acceptance, circuit/website iteration or native authority.


## DW-68 — Honor explicit beta automation overrides

The user removed human-only operation and approval requirements for beta work. Root recorded the override in AGENTS/the implementation loop, observed working noninteractive sudo, and completed the existing OP0..OP6 host setup without changing its source or resource limits. The [actual receipts](../docs/results/sherlock-native-prerequisite-policy-20261017/beta-automated-provisioning-20261018/README.md) replace the missing live provisioning evidence.

DoctorWatson must apply current user authorization before recovering historical operator/approval queues. Keep objective resource/correctness checks and distinguish live setup from final configuration, native compilation, proofs and target outcomes. Do not regenerate a human-only request after that requirement was removed. Preserve historical refusals and policy records rather than rewriting their past meaning.


### DW-69 — Distinguish the three real consumers and preserve negative currentness

R299 passed seventeen phases and one Rust test over24 shared production argument
rows; it did not execute the enclosing CLI/global handler. R301's normal Core
compiler exited zero; the original literal dep-info FAIL_STOP remains alongside
the normalized-path/hash correction without a compiler rerun. R310 passed three
phases and one new test looping two fixed programs through real run, public
report, Serialize-to-bytes and JSON, with457 pins unchanged. It checks independent
UInt7/8/source identities, complete same-run preparation, genesis/post-run
separation and callback/summary/image/resource bindings. Raw stdout is test
status; complete persisted reports/traces, exact independent B clocks and
semantic proofs are not supplied. These are distinct bounded consumers.

R300's successful source/operation review and the raw81-page LF-count discrepancy
remain beside Root's additive convention correction. Root installed fourteen
exact source files after R311 review. R315 then returned SUCCESS on the actual
report result, exact six-file promotion and reporting-document compatibility;
Root installed all six consumer files and verified the combined twenty-file set.
No unchanged passing prefix was rerun and earlier negative metadata is preserved.

A documentation-pin change temporarily made the existing reader CURRENT10 with
stale UIREQ025, despite eleven declared acceptances. That negative remains
immutable. Reviewed R316 compatibility plus two guarded reference operations
restored the actual existing reader to CURRENT11/35, no findings, 24 original
clauses and25 refinements open. This is restored reporting currentness, not a
new EARS adoption or a proof supplied by a matching hash. Review final reporting
docs with their final pin update instead of blindly rebinding an intermediate
body and immediately making it stale again.

Full CLI/global handler, input/output files and a command printout remain UNRUN;
B04 stays unchecked despite actual report serialization in the Rust consumer.
Public developer flows gain no approval gate. Preserve current DW68's actual
beta-automation override and all earlier simulate cuts. No new circuit/website
iteration, native result, security conclusion or replacement factory is claimed.


### DW-70 — Preserve startup diagnostics and separate volatile metadata

Automated beta provisioning passed OP0 through OP6 without a human operator. A first supervisor launch stopped before its one-shot marker because the Cargo home directory mtime changed after capture. A full descriptive comparison found identical tracked membership and only that directory timestamp changed. The changed-input capture and configuration were explicitly rebound; the old failure remains preserved.

The next launch created the one-shot marker and failed at A0 with `listener handoff failed`. Cargo and the compiler never started. Its captured A0 stdout/stderr were both empty: the source registers pipe draining after listener handoff, leaving no underlying startup error in these receipts. The designated security owner is diagnosing this boundary; no root cause, source fix or successful build is inferred. The existing evidence collection and privileged teardown both passed automatically, preserving the consumed marker and failure in the durable evidence directory.

For future DoctorWatson work, distinguish volatile ancestor metadata from pinned input changes and make pre-handoff child errors available within the existing log/resource bounds. These are backlog observations for designated-owner design and review, not permission to weaken checks or repeat the failed operation unchanged.


## DW-71 — Test the smallest failed dependency before another build

Two [existing-tool sandbox probes](../docs/results/sherlock-native-prerequisite-policy-20261017/beta-automated-provisioning-20261018/native-startup-20261018/ordinary-dependency-probes/README.md) each passed in about16 milliseconds with no output: basic namespace startup, then the six exact empty-directory masks. These checks used no SDK or native build. Their passing scope excludes the sealed-memfd trampoline and seccomp listener handoff; those remain the next discriminator. Preserve exact dependency boundaries rather than describing `/bin/true` as a successful compiler sandbox.

The work selector also still contained already-adopted UIREQ002. Root removed only that row, retained all24 remaining action objects and additional obligations, and aligned the checklist/loop with the existing11/35 current adopted report. The evidence ledger and acceptance decisions are unchanged. Do not spend another iteration requeueing an unchanged closed reporting clause.


### DW-72 — Verify provider-visible coverage before adopting a review

R333's prepared prompt was 319,384 bytes and locally hash-pinned, but the actual
reviewer reported 131,446 bytes clipped inside a repeated R324 contract. Both
complete handler outputs and actual receipts were visible: the reviewer verified
the scoped four-phase result and approved the exact seven source/fixture files.
All five later documentation bodies were missing, so it explicitly abstained on
documentation. Preserve that raw negative and its separate scoped approvals;
CLI exit zero, provider SUCCESS, local hashes and unchanged pins did not establish
complete delivery or complete reading.

Supply only the missing five-doc tail in the same non-author context; do not
rerun the passed handler or requeue approved source/result prefixes. Deduplicate
unchanged bodies and keep source-only inline requests empirically at or below
150 KB, with smaller packets when possible. That is a delivery practice, not a
semantic-approval gate or a guarantee against truncation. The reviewer must
explicitly identify unavailable content and withhold unsupported judgments.
Any later docs acceptance needs its own actual complete-reading verdict.

No subject retry, new requirement closure, public developer permission gate or
replacement orchestration framework follows from this delivery correction.


### DW-73 — Pin historical citations to immutable source copies

An append preserved all 312 old source lines, but changed the complete file hash.
The existing evidence reader correctly reported two historical citations stale
when they still named the live file at its old hash. All eleven adopted original
rows remained current. Preserve the negative; do not replace a historical hash
with the new implementation hash or change the reader to suppress the finding.

Copy the exact old source to the public result archive and retarget only those
two citation paths, preserving their hash, line span, claim and acceptance. After
independent compatibility review, the reader reported current with no findings.
UIREQ-035 and SCON-013 now record partial implementation and observed baseline
tests while their cache, streaming and complete-prefix acceptance stays OPEN.
The interpreter tests did not need to run again for this reference correction.


### DW-74 — Check actors and mutable cache inputs before native setup

Fresh beta setup passed all seven steps automatically. Reading the actual
operator caught an incorrect actor in the prepared commands: OP3 requires the
ordinary user; only OP2, OP4 and OP5 require root. The actual selection corrected
that metadata and preserved the operator and numeric caps. No human window was
needed. The guard's new launch tests passed all thirty cases after the exact
reviewer-prescribed fixture corrections.

The native launcher now reaches the authenticated A0 worker. It refused stale
staging metadata: thirteen Cargo registry-index files had grown since their
manifest was captured. Preserve that failure and cleanup; do not convert it to
a build result or repeat the unchanged attempt. DoctorWatson should compare
current cache descriptors with the selected manifest before provisioning and
retain a stable cache snapshot for the chosen dependency tuple. Input refresh
and code/archive identity checks belong in the next scoped reconciliation.


### DW-75 — Match the real staging schema and freeze mutable controls

The first control-refresh helper assumed a nonexistent total-count field and a uniform Cargo-root path rule. The actual manifest has six category-count keys and five already-bound external archive rows. Independent review prescribed four literal corrections; six selftests and the live 1,755-control refresh then passed. Thirteen control records changed, 1,742 did not, and 488 noncontrol row objects were preserved unopened. Keep wrong owner hash/API prose as an unadopted proposal, not an authority for copied source or refreshed inputs.

The changed-input native attempt completed A0_FREEZE and then refused background descendants; it produced no A0_RESULT or accepted native output. Durable evidence collection and teardown passed. DoctorWatson must distinguish schema reconciliation, current mutable control identities, freeze completion and phase success. Preserve the refusal and consumed marker, diagnose the next concrete boundary, and do not rerun the unchanged passing thirty-test suite or relabel setup as a Cargo/compiler result. The separate selected-compile artifact readback reused the saved first artifact before running only the five remaining cases.


### DW-76 — Derive wire adapters from declarations and retain produced artifacts

The selected compiler produced its first artifact and status line before the independent consumer panicked on its layout predicate. Config wire layout and typed admitted layout differ: the latter also contains the same frozen bounds. The independent author corrected that adapter from public type declarations, retaining the frozen semantic expectations. A read-only verification mode checked the saved quote artifact without invoking the handler again; only the remaining five cases ran afterward. Keep the original exit101, the corrected verification and new runs as separate evidence.

The production handler and publisher bytes were unchanged by this test correction. Reuse the unchanged Core library, compile only the changed consumer and preserve exact source/dependency/command/result identities. A post-publication test failure does not justify discarding output or repeating the completed producer prefix. Full CLI/default/refusal/native acceptance remains separate from these six scoped artifact comparisons.

### DW77 — Separate qualitative review from computed identities

The selected compile result review initially clipped the BranchTotal tail and misstated Core wire fields and identity suffixes. Retain that report, then supply a smaller complete corrective packet to the same reviewer. The corrective report read the missing tail and retracted the wrong facts. Actual source, artifact, command and stream hashes remain machine-computed identities; generated prose never overrides them. A scoped successful review does not close B04 or native/target requirements. [Published adoption](../docs/results/sherlock-selected-compile-handler-20261018/ROOT-FINAL-ADOPTION.json).

### DW78 — Mix zero and nonzero arities in public source acceptance

The new ProvenanceTwins consumer compiled and listed successfully, then the
public runtime rejected its valid source before evaluation with
`V4:params not contiguous`. Its unused zero-argument function follows a
parameterized function. The producer normalizes an empty parameter slice to
start zero, while V4 compares every slice start to the cumulative parameter
cursor. The frozen independent source and expected tables stay unchanged; the
repair belongs in the owning preparation/admission contract. Retain the failed
run and empty event file. A test that merely reordered the functions would hide
the defect.

### DW79 — Validate literal test edits against the actual harness

The native lifecycle source owner confirmed the one-line as-pid-1 candidate,
but returned test hunks using obsolete harness signatures and members. Root
withheld that proposed extra test. The two exact expectation-line replacements
were reviewed separately, and the actual current thirty mocked checks passed.
Keep the valid source disposition, rejected patch proposal and actual unit
result separate. Generated claims about host versions, paths or unchanged
source details do not override inspected bytes and machine receipts.

### DW80 — Keep compilation evidence separate from source review

Both source reviewers accepted the new five-test module, but the actual test build found seventeen result-extraction errors: Tooling has no Debug implementation, while Result unwrap/expect require one. The normal production library had already compiled successfully. Preserve both results and request test-local error extraction from its author. Compilation is a necessary check even after detailed source review. A normal library success does not predict a test build.

### DW81 — Preserve usable artifacts when independent checks fail

After the test-build failure stopped its sequence, Root separately selected the three previously unrun public-consumer phases, whose only producer dependency was the successful normal Core library. The frozen program and expectations were unchanged. The new public run passed all table and event-origin assertions; the five unit methods remained unrun. Keep that scoped success and the unresolved test-build failure together. Reuse the compiled producer and retain complete event output; do not replay a passing build to conceal a failed check.

### DW82 — Reconcile generated membership with the pinned tool's behavior

Native04 passed its earlier A0 failure boundary, staging and Cargo fetch. Raw verification then found that only `.package-cache-mutate` was missing from576 declared generated entries. Preserve the strict refusal, the actual575-present/one-absent metadata, and the completed cleanup. Send this precise producer/expected-membership disagreement to the owner before changing configuration or source; another unchanged build attempt would add no evidence.

### DW83 — Fix expected producer metadata without relaxing validation

Native04's raw check expected576 generated files but observed575; `.package-cache-mutate` alone was absent. The owner checked four official Cargo source files at the binary-reported commit, and a non-author reviewer confirmed the narrow next-attempt correction. Keep the guard unchanged and remove only that expected metadata row, with ordered equality for all remaining rows. Preserve the failed attempt rather than precreate a cache file or weaken membership checks.

Run only the new synthetic regression after source/operation review; its missing/extra/exact membership and freeze checks passed. Retain the unchanged30 prior checks without repeating them. The owner's raw32-test suggestion was wrong: static methods total31, and this batch executed exactly one new method. Source facts, actual test result, setup/prelaunch and live native state remain separate. [Published source/ordinary evidence](../docs/results/sherlock-native-prerequisite-policy-20261017/beta-automated-provisioning-20261018/native-startup-20261018/fresh05-metadata-remedy-20261018/README.md).

### DW84 — whole-clause adoption and exact next native boundary, 2026-10-18

Root adopted the entire original UIREQ-027 on eight explicit predicates after whole current-source review and the unchanged actual draft/Core1 traces plus five admission regressions. The canonical reader reports 12 adopted originals, 23 open originals and all 25 refinements open. Keep the raw reviewer opinion and its corrections together: machine trace pins override wrong prose hashes, and the generic-extension label is UIREQ-034, not UIREQ-020. Full-page delivery supports coverage; substantive per-clause reconciliation supplies acceptance.

Native05 then exposed a distinct C boundary: four extra generated members, zero missing, with no successful C or D/E/F result. Preserve the exact difference and durable evidence; do not turn removal of a previous blocker into a new native-success or membership-policy exception. Collection/teardown passed. The next designated-owner diagnosis must address that actual changed boundary. No circuit changed; no website/circuit iteration or full-v1/native/target completion is recorded.

### DW85 — Permit source rereads within the same allowlist

The first B06 nominal-source authoring call stopped after590.6seconds with no final source because its reused review wrapper rejected a second read of one permitted page. The call had already completed102 unique full reads, including all72 mandatory pages; no unauthorized path or execution was observed. Preserve the failed result and its exact source/read receipts.

The changed continuation allows repeated reads of the same177 permitted pages, checks every completed read for full byte/line coverage, and credits the unchanged prior completed reads. Model/CID/cwd, source pins, no execution policy and time limits remain enforced. Read delivery is not source approval. Do not replay an entire unchanged readset or force an author to reconstruct an API from memory after context compaction. No native guard, resource cap or acceptance requirement changed.

### DW86 — Distinguish line-count conventions from missing source

R393's source author returned a concrete validation test after eight completed
page reads. Every reported byte count matched; every reported line count used
LF+1, while the observer expected LF. The observer therefore incorrectly marked
all eight deliveries incomplete. Preserve that original result and a separate
computed correction identifying the actual convention, all exact byte counts
and the author's explicit complete-file declaration. Do not rerun the source
request to repair a counting convention. Delivery metadata does not establish
the test's correctness: the exact insertion still receives independent review
before execution.

### DW87 — Check a review's source facts before adopting its verdict

The first UIREQ-013 review reported approval but invented type names, schema
fields, source locations and frozen case counters. The actual source patch and
six successful ordinary phases are separate evidence. Preserve the incorrect
review; give its author the literal current source, complete actual reports and
specific contradictory facts for substantive correction. An approval label and
complete page reads cannot replace source comprehension.

The new attempt persisted whole-attempt pre/post checks for all499 inputs, not
per-phase checks or continuous source immutability. Record that exact scope;
do not invent stronger receipts or rerun passing phases to satisfy a later
review condition. Require an explicit source/result and affected closed-clause
applicability judgment before reconciling current source pins. No new EARS,
native or circuit completion follows from preparation of that follow-up.

### DW88 — Compare the complete launcher argv before execution

Root's native06 launch omitted the old outer shell's `-eu` flags while adapting
its paths and config digest. Preserve the actual command and explicit deviation;
do not describe its prefix as byte-identical to the old command. A direct
readback confirmed the supervisor entered its required cgroup. The existing
guard and numerical caps were unchanged. Those observations do not retroactively
restore the omitted flags or establish identical failure behavior. Derive future
argv from the recorded JSON and compare every permitted operand before launch.

### DW89 — Preserve compilation separately from its output audit

Native06 passed A0 through E and Cargo reported a completed test build in57.74s
with the backend test executable produced. The subsequent F audit refused a
non-normalized dependency path; there is no successful full F receipt or accepted
native output. Preserve both the successful compiler evidence and failed audit.
Evidence collection retained43 members; the target remains quarantined for
diagnosis, with cleanup still owed. No compiled executable was run.

The first observed dependency spelling contains `../../../../models` from an
`include_str!` fixture. It is metadata for the source owner to assess, not a
reason to suppress dependency checks. Request a narrowly reviewed correction
and a bounded audit-only route over the retained outputs. Reuse the completed
compilation; do not reset its history or rerun Cargo and the passing prefix to
repair an audit. Whole native/proof/target acceptance remains open.

### DW90 — Check every selected path and let independent names expose producer defects

Nominal QA preparation initially assumed a missing manifest key, then retained a
copied argv pointing at the wrong consumer path. Preserve those failed preflight
steps rather than treating their correction as a subject pass. Compare every
actual argv input operand against existing pinned files, distinguishing future
generated outputs, and compare the complete unchanged source snapshot with W.
The corrected candidate03 comparison covers151 files:143 non-overlay sources
match W, eight are intended overlays and no unexpected differences remain.

The independent frozen name oracle exposed an actual producer defect: candidate02
collected declaration type-parameter and type-use names beyond Pass-B policy.
Candidate03 removes only those extra collection steps; declaration type,
constructor and field names remain, while the program entry also collects real
Core program names. Keep the oracle unchanged. Both entry points are publicly
re-exported from shared; the child owner_prepare module is private, so a consumer
import must use the real public interface.

The corrected normal Core compile exited0 in3.987870814seconds. Reuse that exact
artifact and dep-info after source/output pin checks for the remaining consumer
compile/list/test operations; do not replay its unchanged passing prefix. Source
adoption and tests remain pending independent review at this log proposal cut.
No generic runtime, full004/007/034, circuit, native or target acceptance follows.


### DW91 — Nominal integration and observed parser refusal fixture

Nominal declaration source and its independent full-table positive observation are adopted in `docs/results/sherlock-nominal-declaration-preparation-20261018/ROOT-DECLARATION-ADOPTION.json`. The archive has 83 mapping rows and 82 unique original destinations; its duplicate is byte-identical. Integration initially used the wrong published source-review directory name after exact source/archive copying; the corrected reference completed adoption without recompilation. Check actual archive destinations before constructing adoption links. Model-written digest suffixes are untrusted metadata: retain the raw opinion and use host-computed pins plus the published factual qualification map. One unresolved reported prefix is excluded.

Two native dep-info parser positive methods passed. The negative method initially expected filesystem-parent refusal from a path that first underflowed the lexical stack. The reviewed fixture correction preserves the underflow case and adds the intended regular-file-parent case; that one corrected method passed on October18UTC. The guard was unchanged and the positive methods were not rerun. Native output/lifecycle accounting remains a separate review; parser observations establish no SDK proof or stagenet result.


### DW92 — Reject source reviews with contradicted API coverage

The native06 retained-audit four-method synthetic suite passed; the corrected parser negative method passed separately without rerunning its two unchanged positives. Results and the earlier fixture failure are published under `docs/results/sherlock-native06-retained-audit-regressions-20261018`. The R407 opinion claimed source approval but described absent helpers, incorrect signatures and a wrong full-source line count. Root did not adopt that production-source opinion. A fresh context now receives the exact source and actual tests with concise configuration facts, rather than another long polluted conversation.

Provider SUCCESS and declared full reading do not establish source coverage. Check concrete API statements against the supplied code before carrying a review into operation metadata. Preserve raw contradictions and actual test results separately. This changes review delivery, not any native resource cap or acceptance predicate; it creates no new factory framework.

### DW93 — A reviewed audit can still fail on the retained artifact

Fresh source review and passing synthetic tests did not make the real retained audit succeed. The once-selected audit refused a non-normalized dep-info path; preserve that failure and the original build rather than rerunning Cargo or the passing prefix. The before/after metadata/hash comparison establishes its recorded interval only, not retrospective source/OS I/O authenticity. Empty cgroups do not discharge cleanup.

Before another source correction, isolate the exact dep-info word and parser branch using retained data. A generic refusal string is insufficient to select a fix. Keep source/config adoption, synthetic tests, real audit, output acceptance and cleanup distinct. The audit is consumed; any next action needs its own concrete scope within existing authorization and remaining limits, not a hidden retry. [Actual result](../docs/results/sherlock-native06-retained-audit-regressions-20261018/RETAINED-AUDIT-RESULT-STATUS.md).

### DW94 — Check API types, event envelopes and real session identity

The word-error consumer required exactly three author type-only hunks: an event counter changed from usize to u128, and two panic messages stopped requiring absent Debug implementations. The mathematical cases and full intermediate assertions stayed unchanged. Independent test/operation review followed; the error cohort reused the successful Core library and did not replay the normal cohort. Read the actual public result and event-stream declarations before authoring a consumer.

A fresh reviewer request omits a preassigned conversation ID and binds the actual first init CID. A resumed request must require the historical requested CID and reject a different returned context; binding first init is not permission to silently replace a resumed reviewer. The original production review's terminal SUCCESS/process137 and later clean confirmation remain separate observations.

The independent error-test review returned provider0/SUCCESS while the local JSON-only observer rejected one enclosing JSON code fence. Preserve that observer failure and parse exactly the single full-response JSON fence with outside whitespace only. This formatting-only issue does not require another source audit or rerun. Reviewer29's constructor authorship and a3ce's error-test authorship remain separate from their independent review scopes. [Root word adoption](../docs/results/sherlock-selected-word-arithmetic-20261018/ROOT-WORD-ADOPTION.json) now closes original030; no circuit/native/proof result follows.

### DW95 — Match real artifact layouts as well as synthetic examples

The dot-segment parser's five selected SDK-free methods and the fixed-successor route's one new synthetic method passed, yet the real retained audit still refused: `run-unit lacks dep-info-associated compiled build script`. Keep that negative result, the successful original Cargo build and all unchanged passing prefixes. A synthetic happy path is not coverage of the vendor artifact's actual build-script associations.

Root's retained-artifact diagnosis identifies an aws-lc-sys custom build-script target named `build-script-main`; the complete34-unit association inventory is still being prepared at this cut. Use the actual retained unit/dep-info metadata to freeze a discriminating layout fixture before changing the matcher. This observation identifies neither the only gap nor an approved fix. The failed audit is consumed; no Cargo rebuild, prefix rerun or hidden audit retry follows from this lesson.

The R418 reviewer correctly hashed compact supplied operation JSON. Root initially compared it with formatted proposal hashes; the published correction preserves both representations and their equal decoded values. Do not call that a faulty review. Keep3,971 target stat records distinct from3,544 total regular-file hashes, and preserve one fresh context's two typed roles as one opinion. Outputs remain quarantined/unaccepted and cleanup owed. [Actual records](../docs/results/sherlock-native06-retained-audit-regressions-20261018/fixed-successor-r2-20261018/README.md).

DW95 follow-up, October18: the [frozen whole association metadata](../docs/results/sherlock-native06-retained-audit-regressions-20261018/fixed-successor-r2-20261018/actual/artifact-layout-source-facts/MECHANICAL-SUMMARY.json) is now available: 34 run units, 34 compiled build-script fingerprints, 68 build executables and 22 shared objects. It records the custom aws-lc-sys `build-script-main` target and both getrandom versions. The earlier cut above remains historical; these source facts do not approve a matcher correction or retry.

### DW96 — Close the actual producer relationship and retain source currentness

The [owning financial producer](../docs/results/sherlock-owning-financial-producer-20261018/ROOT-FINANCIAL-PRODUCER-ADOPTION.json) retains one real stage call’s program, consumed arguments and raw result in private fields. Its observation and shared publication borrow that record; they do not rerun an evaluator or pair arbitrary inputs with an unrelated output. Three actual start/false/true calls matched complete independently frozen values in memory. Final non-author source/result review found the complete original017 clause satisfied; Root adopted it while retaining separate universal lowering, failure, authentication and native obligations. No actual financial JSON file was persisted, and the enclosing CLI was source-audited only.

The requirement reader then correctly removed025 from the current count because the dated native-refusal documentation suffix changed a declared pin. Root retained the actual failed check, verified the complete previous document is an exact byte prefix, and recorded that limited documentation compatibility before updating one live hash. The next actual check reports14 current originals and no findings. Hash updates follow an explicit applicability judgment; matching bytes alone do not approve behavior. No unchanged passing product cohort was replayed.


### DW97 — Validate actual public reference objects before currentness

The generic preparation source, independent tests and fourteen-clause applicability review passed, but the first requirement reader still failed because three evidence-reference names resolved to absent public `actual/ordinary-four` aliases. The receipt bytes already existed under another archived directory. Root preserved the failure, added seven exact aliases covering the shared publication namespace and called the existing reader once more; that actual check reports current14 and no findings. Neither matching hashes nor passing Markdown-link checks established that the reader's exact referenced paths existed.

Before final metadata adoption, resolve each newly introduced source/evidence/review path against the actual copied public destinations and compare its bytes. Keep source installation, actual product results, reviewer applicability, publication completeness and currentness as separate facts. Repair a path-only publication mismatch without editing old receipts or rerunning a passed compiler/test prefix. Original034 is still open after its observed preparation increment; an advisory graph edge or a current hash supplies no missing runtime, semantic proof, native result or acceptance.

For the actual F03 K/Quint observation, retain all complete projected fields, charged/free-publication counts and omitted inputs beside the result. Whole original021 acceptance follows its normative predicate and Root's separate adoption, not a provider SUCCESS marker, graph membership or an unexplained total. No new runner or approval layer is introduced by this lesson.

DW97 follow-up, October18: the next documentation-currentness proposal scanned only its preferred `profiles-results.md` filename and missed the changed `explanation/evidence-and-acceptance.md` pin34. The actual reader correctly refused025 again. Root inspected all three changed document bodies and the complete reference set: exactly two live025source-pin objects were affected, while the how-to page had no live reference. Its [explicit compatibility addendum](../docs/results/sherlock-k-quint-f03-actual-20261018/ROOT-DOCUMENT-COMPATIBILITY-ADDENDUM.json) preserves the first failed check, the complete original text and all statuses; the [next actual reader](../docs/results/sherlock-k-quint-f03-actual-20261018/ROOT-FINAL-CURRENTNESS-READBACK.json) reports current15 and no findings. Scan **every changed path** recursively across the ledger's source, evidence, review, citation and currentness objects; distinguish exact current references from archived preimage suffix matches. Do not stop after the expected filename or first hit. A full changed-path inventory precedes the applicability judgment and guarded hash update; it does not supply that judgment itself. No passed product/model/native prefix was replayed.


## DW98 — retained audit, exact outer operation and completed preservation

October 18 observed lesson: the first R3 outer wrapper refused before Python because the supervisor could not write `cgroup.procs`. Review and restore the complete selected outer operation, including the existing noninteractive privilege/setpriv boundary, rather than inspecting only Python operands. Retain that failure; do not replay the Cargo build or passed native prefix. The corrected retained audit passed once. Its security and independent normal result reviewers accepted scoped compile provenance; same-CID followups are not additional independent votes.

The [public R3 archive](../docs/results/sherlock-native06-association-audit-20261018/README.md) preserves raw opinions, operation receipts and count qualifications. The complete preservation set contains 3,554 files: 3,498 target files, 55 packet files and one separate 9-byte marker. Root verified all hashes/sizes, fsynced the persistent quarantine, made files nonwritable/nonexecutable, and then actually removed the three empty cgroups and unmounted the three tmpfs mounts. The final receipt records `cleanupOwed:false`; post-cleanup checks cover file regularity, size and mode without rehashing 2.628 GB. These facts do not promote native outputs or establish target execution/proofs/stagenet.

Freeze publication bytes before volatile cleanup, retain historical source paths as provenance, and copy from frozen staged paths afterward. Preserve each earlier pending/owed state with its date and later actual resolution; never rewrite the old receipt or manufacture a scope-adoption record. Scan every changed documentation/publication path recursively across every ledger object before Root's separate descriptive compatibility judgment. No new runner, schema or approval step is introduced.


### DW99 — Check the identity that the owner consumes

The [canonical TypeId migration](../docs/results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json) passed three compiles and ten new tests. Builtin results must stay unchanged, so output-only checks could miss a disconnected conversion. The positive consumer follows actual signatures through task/control/frame/helper state, then compares bound values and results with independent expectations. Distinct nominal IDs test storage separately.

The capturer false-stopped after a multiline passing test09. Root preserved that receipt, accepted the same bytes after independent review and ran only the four remaining tests. Preserve failures and separate capture interpretation from subject execution. The [Root compatibility record](../docs/results/sherlock-canonical-runtime-types-20261018/ROOT-CANONICAL-TYPEID-COMPATIBILITY-ADOPTION.json) keeps current references and historical citations distinct. This host-only increment adds no original/refinement closure or circuit iteration; native DW98 retains its own evidence boundary.


## DW100 — Keep complete-publication consumers compatible

October18 observation: adding the consumed `await_prefix` field made an existing strict whole-publication consumer incompatible. Preserve that strict comparison. The compatibility change adds five lines and independently frozen expected prefix includes while leaving the test function, original assertions and producer-call sequence intact. It then compiles against the already successful Core rlib and runs only its changed selector. The [archive](../docs/results/sherlock-financial-await-prefix-20261018/README.md) retains both positive results, source identities and unchanged input checks; no Core/new-consumer passing prefix is repeated.

Scan every changed production/test/fixture path recursively across the entire ledger. Update only adopted current references after substantive result/compatibility review; preserve archived old-wire source hashes and receipts. A matching pin is not semantic approval. Keep actual argv/helper/test-readback authoritative when a reviewed descriptive field is stale, and retain the original metadata plus explicit qualification. Correct reviewer arithmetic15closed/20open without turning that correction into a new closure. These host observations do not close generic018/019, full013 or native financial/proof/target obligations; no new runner or approval framework is introduced.


Review fidelity lesson: compare cited helper names, plan hashes, source line ranges and callable names with the actual selected files before adopting a result. The final financial reviewer cited nonexistent methods and the wrong runner/plan. Root preserved that opinion, supplied the complete actual files, and obtained explicit withdrawals and a source-grounded acceptance. A model-recited hash catalogue does not replace the selected manifest.


## DW101 — Keep counterpart source and observed stages separate

October 18 instruction: maintain dedicated Clean, Quint and K counterparts with each primary batch through the existing loop. A ready source batch is not a model run. Independently freeze shared inputs and complete expected projections before execution; compare full state/event fields and declared clock/work mappings, retaining unsupported cases and omissions. Keep ordinary host tests, model frontend acceptance, model runs, complete comparisons and native/proof/target results separate. No new factory or denominator is needed.

The current nominal K sequence makes those boundaries concrete: after the earlier missing-Map failure, model-only compilation passed in 4.238 seconds, but the first `kast` failed on the Map argument surface and 21 later children stayed unrun. Prepare the smallest input-only correction and reuse the successful compiled definition; do not replay that passing compile or report corrected tail observations before they exist. The separate five-operation witness host pass does not supply those missing model results. Native source returned or reviewed likewise does not resolve B1–B5 or the measured Cargo global-cache mismatch.

Preserve each actual failure and later stage result with its exact source/input identity. Keep dedicated source lanes active, and ask the independent complete comparison what changed before adopting correspondence. The [loop contract](../docs/sherlock/implementation-loop.md#persistent-clean-quint-and-k-counterparts--october-18-user-instruction) carries the persistent instruction; website concepts remain illustrations for the planned completed product.


October 18 observed check follow-up: the fresh Quint source review approved the finite operations, but the first actual typecheck failed with QNT101 on local binders `fields` and `field`; the two later commands remain unrun. The corrected K queries parsed, reduced and printed with exit0 for all seven cases, but complete comparison rejected every unevaluated `ngObserve` term. This is an evaluation failure, not correspondence evidence. The dedicated agents are preparing source/operation corrections while preserving these receipts. Clean review found a reserved `prefix` identifier and a non-definitional `rfl` base case; kernel checks remain unrun pending corrected source.


## DW102 — Judge the complete result, then preserve the successful prefix

October 18 observation: K reduction/printing exits0 once produced an unevaluated `ngObserve` term. Only the full independent tape comparison exposed the failure. A later selected initial-simplification tail matched all seven complete tapes while reusing the successful model definition and query parses. Preserve the failed comparison and all raw captures; terminal values or frontend success cannot replace initial/every-returned/final state and event-prefix equality. Deterministic gzip may store a large raw record only with its original byte identity and verified exact decompression.

Quint's corrected identifiers passed three commands, eight author-fixture tests and a directed run. Those results have a different expectation basis from K's independently frozen tapes. Clean's corrected source passed four module checks without warnings after a record-layout failure; five successful CanonicalTypeId artifacts were reused. State the conditional theorem scope instead of claiming an absent real child evaluator or whole correspondence. Actual-result review, source installation and native/product acceptance remain separate. The [evidence archive](../docs/results/sherlock-formal-nominal-progress-20261018/README.md) preserves both the failures and the new passes; no unchanged passing-prefix replay or new factory is needed.


DW102 result-review follow-up: the K raw ACCEPT contained 23 factual catalog errors, including invented codec/frame names, keys, bounds and a wrong fixture description. Preserve the full raw report plus the exact correction map/facts, and adopt only the supported seven-positive-tape claim. Quint's three minor qualifications preserve its distinct author-fixture scope. A mechanical factual map is not a new semantic vote, and a provider's hash recital cannot replace the actual selected manifest.


October 18 qualified-result and new-actual follow-up: K's seven-case actual result now has scoped review support only with the full 23-entry factual map and exact manifests; raw review catalogs are not adopted. Quint B01's three finite commands have qualified review support with three factual qualifications; its eight tests/193-state trace are author-fixture observations, not independent complete-oracle agreement. Clean's four conditional module passes remain separate from real child evaluation or whole correspondence. Root owns source/publication adoption; this adds no EARS closure.

The new witness reference projector also passed five ordinary operations across three exact reference-prefix cases; its result review remains pending. The shared-four Quint batch passed four operations, and Root's data-only comparison found all four complete observations equal (15/27/10/28 states, 80 total; 17 snapshot fields). Its result review remains pending. Neither result broadens the existing native/proof/target or whole-language scope.


## DW103 — Qualify catalogs without discarding supported data

October 18 lesson: the shared-four report returned raw ACCEPT but recited wrong source digests, sizes and publication keys. Preserve the full 13-entry correction map and exact records alongside it. All 68 valid factor references and the actual four complete equal observations remain supported. A catalog error neither proves reading a fictional artifact nor cancels a separately supported data comparison. The mechanical map is not a new semantic vote.

Portability also has distinct evidence stages: historical five-operation runtime review for original bytes; fresh source approval for four include literals; one actual portable compile. Do not inflate that compile into three new runtime passes or replay a successful prefix. Root has explicitly adopted unchanged-body namespace compatibility for retained027 and rebound five current references; original035 and SCON013 remain open. See the [scoped records](../docs/results/sherlock-formal-nominal-progress-20261018/addenda/20261018-reviewed-shared-and-witness/README.md). No new framework or EARS closure follows.


## DW104 — List membership and a tiny reproducer have distinct meanings

October 18 lesson: a successful discovery compile and an exact three-name list execute zero test bodies. Preserve those receipts alongside the contrast consumer's actual two-body pass; do not combine them into five runtime tests. Where comparisons occur in memory, report that boundary instead of inventing a persisted full actual dump. Keep the independently specified B0 counters distinct from the reference cursor.

For a frontend failure, the tiny Clean reproducer retained an expected negative exit1 and a positive exit0. That confirms a narrow correction approach while the full JSON path remains unrun. Reuse the passing prefix, retain failures and require the next actual stage before broadening a claim. The [records](../docs/results/sherlock-stage-separation-20261018/README.md) add no EARS closure, factory, circuit or native acceptance. Pending cache-debt source likewise remains separate from its eventual reviewed and observed behavior.


## DW105 — Name the source class, preserve the stage, and keep writing circuits

A preserved entry count is not a source-only count: P5 matched30,660 unchanged entries consisting of30,090 source and 570 generated files. It then failed a different partition check before target copy/Cargo. Preserve both observations and actual cleanup. Likewise, the44-byte TAG fallback does not reproduce177-byte historical TAGs; the six-template/two-TAG live-hash qualification remains.

Keep all four permanent lanes concrete after compaction. Returned checkedAdd emitter/test source is reviewable progress, while live A/B reviews supply no verdict. The archived/unregistered Scalar.tag005 source is not an observed native circuit baseline. Formal passes cannot substitute for the next handwritten implementation.

Separate observations within each formal lane: Clean's complete five-case equality does not turn a later zero/rfl frontend failure into a theorem. K compile/kast success does not turn a timed-out reduction into a complete tape. Quint's complete finite comparison does not make Part 4 acknowledgement a final result review. The new decoded-metadata consumer's actual five-operation pass remains separate from whole035 adoption. Preserve the successful prefixes and failed stages without reruns or expected-data fitting. See the [records](../docs/results/sherlock-four-lane-progress-20261018/README.md). No new framework or closure follows.

Persist each returned child receipt before later bookkeeping. One Root postcheck used an absent recipe key after a failed Caller check; the streams were retained, but its exit/time remained explicitly unknown. The next check persisted its child result first. Actual Lean diagnostics also found Caller and Arm defects missed by the source reviewer; all four corrected B06 modules subsequently passed without changing theorem meanings. A model opinion does not replace compiler/kernel evidence.


October18: follow the existing acceptance schema exactly: acceptance.adoption
is the enum adopted; keep the actual decision path/hash in rootAdoption. A
current owner declaration can pass integrity while not entering the adopted
subtotal. Inspect both subtotals before publishing a closure count. The035
metadata correction retained the actual reviewed semantic adoption and all
source/result pins.


## DW106 — Require the consumer, preserve the build, and qualify the whole comparison

A complete-context primary Opus review supplied concrete F1–F12 findings instead of another empty paging acknowledgement. Its later exact five-hunk F1/F2/F4 source approval and the separate metadata pass are progress; neither is an admitted runtime. Keep the nine other findings open. Packet size and a successful compiler return do not substitute for source semantics or actual consumer behavior.

Inspect a new consumer's actual source-path whitelist and imported API before freezing it. Root's exact W2 allowlist/read-only assignment binding edits honored the two original conditional opinions; no postpatch vote was invented. Keep test-build import/type repairs on a separate clean checkout so frozen source and strong assertions survive. P8 then built successfully but failed its audit: preserve both outcomes. The whole diagnostic shows six build-script binary recompiles and eight run-unit reexecutions, not a harmless timestamp discrepancy. Repair the seed-order source under review rather than weaken the guard or rerun unchanged Cargo.

Compare whole formal traces and publish the review's actual read scope. Clean retains typed-plus-byte equality; K retains complete typed equality with different raw bytes. About 30 K states were decoded by the reviewer, about 425 were not individually decoded, and Root's full machine comparison covers all 455. Preserve the independent prefreeze and conditional README/hash-override history instead of promoting partial manual inspection to a full decode. Keep the four source lanes concrete; a combined circuit candidate pending audit supplies no SDK-test or proof result. See the [record](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261018-admission-and-complete-traces/README.md).


## DW107 — Provider success is not a complete source delivery

A provider process exited 0 and reported success, but its full 74,931-byte Sub response began inside a source string and failed strict JSON at character 0. The requested delivery was an explicit source/edit object with reconstructible files and exact sequential edits. Comparing the full public response against that schema found no materializable emitter or tests. Do not infer missing code from token metadata, normalize fragments into a candidate, or publish a complete trailing ABI/website-note field as evidence that Sub was written. Preserve the raw response and delivery diagnostic.

The separate composer response did yield four unique source hunks materialized across nine files. Its forty tests are authored source only and remain unrun; the M1 disabled-chain stutter and M2 namespace changes still require fresh independent audit. Materializable source, independent approval and executed checks are different stages. Likewise, the native literal-only request exited 0 but returned an explicit refusal: no edit, vote or accounting-source completion followed. Factual JSON arithmetic does not patch the protected guard/helper. No further rescope is proposed here.

Keep partial formal progress equally precise. Clean's R4 reached the later theorem and failed elaboration; its earlier concrete-history result is not whole-module kernel acceptance. The subsequent R5 receipt also records exit 1. R6 then passed kernel checking in 0.559749247 seconds with empty stdout/stderr; that accepts only nine literal-call/cursor statements, with no full refinement or new EARS/native claim. Quint's six boundary-wire checks passed and publication was committed and PR269 was created; merge completion is outside this cut. Continue all four mandatory lanes—Clean, Quint, K and handwritten circuits—without replacing implementation with delivery bookkeeping. The [dated records](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261018-provider-delivery/README.md) add no framework, circuit iteration or requirement closure.


### October 19 — Include reused constants and record the pretest correction

The first fresh P8 accounting security opinion approved the source with two context conditions: reused P7 marker constants and the seven-step cleanup sequence were absent from its received excerpt. Include reused constant definitions and their actual container types with complete affected functions. Root then confirmed the existing marker and ordered tuple against actual receipts. Its first static check compared that tuple to a JSON list; it corrected only the check's sequence normalization before importing or testing. Candidate and receipt bytes never changed. Record this as a pretest check correction, not a source defect or failed test.

The later normal-source opinion records the conditions discharged. Exactly three changed SDK-free methods passed in 0.247066901 outer seconds, with the exact three source files unchanged. These author-written checks are interface-informed and do not exercise the production non-None receipt branch. Preserve that limit, both full opinions and the minor freeze-hash shorthand correction. The [accounting record](../docs/results/sherlock-native-p8-accounting-20261019/README.md) retains failed P8 and the explicit narrow user author exception. Native build/proof/stagenet outcomes are unrun in that record's source cut; later Root work needs its own receipts. No requirement adoption or new framework follows.


## DW108 — Cover the consumers of a shared ABI

October 19 lesson: normal-Core compilation and eighteen selected ordinary passes do not compile every affected consumer. Ten full captures preserved the old C6 bytes, while a separate source inventory found 27 introduced retained-test compatibility sites and six older TypeIdC mismatches. A cfg(test) module is excluded when Core is a normal dependency; external integration/support serializers need their own selected compile checks. Before merging an ABI change or claiming broad compatibility, inventory the actual callers and validate the affected cfg(test) and external serializer targets. Do not treat a normal-library pass as evidence for those unselected paths. Preserve assertions, independent projections and partial target reports; keep introduced defects separate from pre-existing ones.

A changed, small H1 pure_uint test-source request returned after two broad timeouts. The exact LF correction preserved the existing format contract, and the two new checks passed. This supports the specific changed approach; it is not a promise that all short packets succeed. Clean's two mutual finite-source soundness proofs have an actual kernel pass, while their premises and missing source constructs remain explicit. Materialized Sub/typed-completion/ReturnRight source still has unrun SDK checks. Keep those source, evidence and acceptance stages separate across all four lanes.

The P8 accounting source is reviewed, tested in three SDK-free methods and merged; that does not erase P8's failed native result. P9's prelaunch stop and exact old public-cache input recovery do not establish an approved next operation. Three classifier-stopped review contexts supplied no verdict. Record that absence plainly, distinguish it from an outage, and do not infer approval or a reviewer substitution from it. The subsequent explicit user grant permits Gemini or Muse for only the two P9 review items. Root selected fresh Argon HIGH Q1 and Muse HIGH Q2 contexts; no verdict is present in this cut, and the grant alone permits no native execution. Core39's eighteen observations belong to the frozen Core368/PR273 cut, while the current WW uses the separate Core354 branch. Keep both source identities rather than treating a checkout change as evidence corruption. The [dated record](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-consumer-closure/README.md) adds no factory, runner or EARS closure.


## DW109 — Keep the guard result, restoration and actual-data binding distinct

October 19 P9 reached a successful guard terminal result after exact source-directed offline-cache recovery, with no guard waiver. The completion sequence took 120.323642509 seconds, phase F 95.320790359 seconds and Cargo's reported build 25.69 seconds. The postcheck recorded 34 build-script run units not reexecuted and 90 host outputs unchanged. Keep these scopes separate: a successful child compile alone is not the full guard result. The backend test executable was produced but not run, and its outputs remain unaccepted pending independent result review.

The durable restoration receipt records the original 20,024-byte cache restored, original bytes verified before final timestamp restoration and a final `lstat`-only check. Saved pre-read atime/mtime were restored; ctime changed. Empty cgroup process lists and completed cache restoration did not themselves establish preservation or teardown. A later separate receipt now records 3,566 exact preserved files totaling 2,847,459,477 bytes, successful collection/teardown and `cleanupOwed: false`. Keep the operation result distinct from its preparation and from native output acceptance.

Root caught a proposed P9 projector still pointing at P8 inventory and raw observations before execution. The owner corrected the two data literals. Before labeling a derived report actual, compare its input paths and identities with this attempt's saved records. Preserve historical observations and failed preflights under their original identities. The [dated evidence cut](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-p9-terminal-restoration/README.md) records observations without a new framework, security judgment or requirement closure.


## DW110 — Complete the owner changes and validate each executable consumer

October 19 Core recovery completed the actual cfg(test) and retained external-consumer work after the earlier normal-library-only pass. Nineteen selected operations passed: thirteen compilations, two listings and four exact owner tests. The full Core listing had 335 tests; each selected owner run left 334 filtered out. Eleven external targets compiled without running their tests. This is meaningful consumer recovery, not execution of the whole suite or new language acceptance. Preserve the earlier partial report and later exact results as separate dated observations.

Review and materialize complete owner function changes before selecting execution. Read the actual schema when separating seed/input rows from produced/output rows; an old inventory or a patch fragment cannot establish the new record's meaning. Check the affected cfg(test), external serializers and output projections rather than inferring compatibility from normal-library metadata. Retain independent expected values and existing assertions. Source approval, fixed-operation review, observed result and semantic adoption each need their own truthful scope.

Quint H2's two narrow Result/Waiting-advanced tests also passed; the literal MF1 `site=s` is not a canonical Core identity. Clean's successful finite Match check preserves its earlier-arm failure premises and selected-body derivation; it does not establish the separate all-arm-exhaustion case. B16 R1 failed at line 787 with `No goals to be solved` (exit 1 / 2.857547972 seconds); no new artifacts were recorded and all 55 original / 55 copied artifacts stayed unchanged. The subsequent minimal 32-byte redundant-`simp` deletion preserved all 24 statements, received fresh Muse approval reported by Root and passed the actual kernel check in 3.103331909 seconds with empty streams. All 55 originals and 55 copies stayed unchanged and five artifacts were generated. This adds sufficient no-matching-arm soundness, not completeness or production refinement. P9's adopted compile fact and completed restoration/cleanup do not execute the SDK test ELF or the 49 new circuit SDK tests. Keep Clean, Quint, K and handwritten circuits active. The [dated follow-up](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-executable-consumers/README.md) changes no requirement acceptance and makes no currentness claim from the last adopted counts.

The separate K source/operation reviewer reported a truncated `<USER_REQUEST>` window and seven mandatory sources unread, returning `SOURCE=REQUEST_CHANGES` and `EXACT6_OPERATIONS=INELIGIBLE`. No K children ran and no private oracle body was opened. Fix the delivery process so the mandatory bodies can actually be inspected; do not weaken the source request or call an unread review complete. The raw title incorrectly says Muse; Root identifies the configured route as Gemini4Argon HIGH via AGY. The report's asserted 128 KiB host-cap mechanism remains under Root's independent delivery investigation. This observation alone neither revokes unrelated Core reviews nor proves a universal transport limit.


## DW111 — Reproduce filesystem assumptions and keep result scopes separate

October 19 P10's eleven selected SDK-free methods produced ten passes and one failing P9 tamper method with seven PermissionErrors: the fixture tried to rewrite files it had made 0444. Preserve that raw failure. The owner supplied the narrow permission repair, and only the repaired method subsequently passed in 0.256110654 seconds. The unchanged-ten dependency proof supports retaining those earlier named passes; this is not a single eleven-test pass and establishes no SDK/native result. Source approval, operation eligibility and actual observation remain separate facts.

A later actual Opus5.5 high metadata review returned REQUEST_CHANGES/INELIGIBLE with B1–B4: derive returned identity/context from actual records, check exact argv substitutions, bind QA postchecks/source pins, and cover every operand/hash/placeholder. These are the reviewer's findings, not completed fixes. The reused prompt said Muse; actual command/modelUsage identify Opus, with no independent weights attestation. A separate bridge review found a mutable public preparation/pair association, duplicated implementation, discarded B-genesis content and out-of-bundle expectation qualification. Its three narrow ordinary operations were eligible, while source changes remained required. Do not turn eligibility into observed success or a same-author specification into a blind oracle.

Keep Clean, Quint, K and handwritten circuits active. K's six passing operations cover 17 complete K states, 17 fixed partial boundary pairs and 77 primary-record own checks; the expected body opened only in child06. Clean B17 kernel-checked a sufficient missing-function path, not a global iff/refinement theorem. Mul arithmetic/typed completion had source/static-API observations with SDK/tests/proof and downstream wiring unobserved at the author cut. The [dated records](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-k-prefix-mul-source/CURRENT-DELTA-20261019.json) preserve those scopes.

[DESIGN009](sherlock-circuit-atlas/DESIGN-009-FULL-PRODUCT-AND-TYPED-COMPLETION.md) is copied byte-for-byte from the actual Muse contributor/meta reply; factual status lives separately. No website code, new circuit iteration, acceptance count or native/proof/ledger approval is added. The website goes live only when the whole Sherlock goal is finished and accepted.


## DW112 — Validate the integrated consumer against the new shared build

October 19 Root found that the prepared five-consumer QA plan used Core104 and would become stale as soon as the new public Core105 API was integrated. Preserve that Core104 plan as UNSELECTED. Reuse the actual new Core105 rlib for the five relinks and eight checks, with its generated-artifact identity bound into the operation inputs. Do not build Core again merely to recover the same dependency. The revised consumer operations remain unobserved until their actual results are recorded.

An existing owner assertion imported a private implementation module despite an available public reexport. Correcting only that import is an ordinary integration change: retain the original and exact delta for fresh Opus review. It does not require another security-author copy cycle. This distinction does not extend to guard, policy or hostile-assertion logic; those retain their assigned ownership and review.

Clean B18's source-derived argument-binding-failure extension actually passed the kernel check in 4.729634353 seconds with empty streams. That observation is scoped to the checked theorem, not global production refinement or an EARS closure. Root's dated current-evidence report remains eight valid EARS and sixteen historical records, unchanged by this append. Keep Clean, Quint, K and handwritten circuits active through every recovery; carry the next source cut, independent expectation, actual observation and remaining counterpart gap for each lane.


## DW113 — Check the real consumer before launch and preserve executed-byte provenance

October 19 P10's changed W3 native guard passed in 97.064814823 seconds; its F helper took 82.368223282 seconds and Cargo reported 24.01 seconds. Thirteen handwritten modules now compile, while all 49 SDK tests remain UNRUN. The preserved quarantine has 3,574 files totaling 2,916,032,469 bytes; its 467,781,064-byte test ELF remains 0644 and unexecuted. The postcheck records 34 build-script run units not reexecuted and 90 host outputs unchanged. Actual root teardown discharged cleanup: all three mounts and the cgroup are absent, and the original camino cache is restored. Compilation, preservation/cleanup, test execution, proofs and target acceptance remain distinct facts.

Five extra data-header byte-count fields (94 encoded bytes) disagreed with the guard's actual pin_schema. Check metadata with the read-only real consumer before launching the guard. Use Python -B in frozen checkouts to prevent generated pycs. Correct an ordinary import through an existing public reexport without a redundant security-author copy cycle; this does not extend to protected guard, policy or hostile-assertion logic. Libtest pretty output with --nocapture interfered with the parser; revised operations15/16 use --show-output and remain pending. D1's delivered materializer lost its ROOTS reset after execution: preserve the delivered record, retract the false attribution, and do not fabricate the bytes that ran.

PR280's source-retained A102 route has three passing operations and merged at d372b7. K's two new steps and five operations passed; PR281 merged at f2e6562b. Clean B19 passed its kernel check in 3.533338 seconds (source F d8ef82), scoped to that theorem extension. Keep Clean, Quint, K and handwritten circuits active. The [dated status](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-p10-compiled-cleaned/STATUS.json) changes no acceptance: eight current valid EARS and sixteen historical records remain the reported cut. No world-language percentage, new EARS closure or website creative revision is claimed.


## DW114 — Reuse unchanged evidence and select the next real SDK check

October 19 P10's exact W3 compile prerequisite succeeded. The produced W3 executable subsequently listed 119 tests and passed all 49 selected tests: 47 host tests and two in-memory SDK serialization/load tests. These results do not execute SDK constraint checking, ProofPreimage.check, a proof or a ledger transaction. PR276 merged at 2d291117 with the tested source head229770 unchanged; PR282 merged at 28a296fc with the receipts, H5 and DW113. Preserve DW113's earlier unrun test cut as history.

The actual existing Core105 rlib was reused for all eighteen passing QA operations: five relinks, five listings and eight runtime checks, with zero second Core builds. The corrected revision02 argv and actual results govern; the earlier plan remains historical. Root is publishing eight source files and 166 documentation/evidence files, not yet merged at this cut. Eight current valid EARS and sixteen historical records remain the reported state until explicit factual adoption and the checker run.

For exact unchanged source, reuse applicable substantive audit coverage and review the actual changed operation/data instead of repeating the full source audit. Keep metadata, selector and output-parser corrections distinct from real source defects. Include fetch and every load-bearing called helper in the source closure. These lessons preserve independent review, protected ownership and objective acceptance; they do not turn data plumbing into a source waiver.

The next missing observation is the existing genuine A102 producer/emitter consumer reaching the actual SDK check under an explicit source-bound operation, rather than another host-only substitute. Its applicability and limits remain to be established; the 49 passes supply no native allowance. K22's five eligible operations remain UNRUN. The whole Core2 contract remains rejected with revision2 ready; the separately adopted narrow weights do not approve that whole contract. Keep Clean, Quint, K and handwritten circuits active. The [dated status](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-host49-core105/STATUS.json) adds no EARS closure, world-language percentage or website deployment.


### DW114 later factual cut — merged publication and distinct artifacts

The original DW114 publishing/UNRUN statements above retain their dated cut. Core105's eight source files and 166 documentation/evidence files, plus the 372-file host49 publication, subsequently merged in **PR283 at c30b403867c1e44c713d385ce08c89c2cfdf79ad**. PR276 names the W3 handwritten source cut, not the Core105 publication. The source/prospective-operation reviews selected Muse before the later Opus agreement and described an UNRUN cut; the later eighteen actual passes are separate observations. D1's delivered-generator defect and attribution retraction remain retained.

The fifty W3 operations are one listing plus 49 exact test runs. Core105 uses its own rlib; W3 uses its native test ELF. Their results share no claimed build-artifact identity. K22's five operations have now actually passed. PR284 is a draft SDK-check source change on Root's newer WW40d cut and remains uncompiled; it supplies no SDK-check result. The seven-clause currentness opinion and this factual correction adopt no EARS: **last reported current eight, historical sixteen** is advisory history until Root's explicit factual adoption and checker outcome. Preserve all existing formal/circuit lanes, source/result qualifications and pending SDK/proof/target boundaries. No website deployment or creative-copy change occurs here.


### DW114 later lesson — inspect the complete persisted metadata before requesting source

P11's returned owner source took 875.60186 seconds and 90,989 bytes but assumed every preserved source file was 0644. The real manifest and projection contain 3,558 files at 0644, fifteen at 0600 and one at 0444. Root's first copy refused before mkdir; no copy or native attempt occurred. The later exact copy preserved all 3,574 files and 2,916,032,469 bytes in 17.428 seconds, fsynced the copies at 0444 and left the original source stats stable. The complete receipt is pinned separately; this data copy is not guard acceptance or source eligibility.

The next six-hunk owner fix then assumed padded `%04o` strings, while every actual projection value is unpadded `644`, `600` or `444`. Inspect the complete on-disk mode set and exact serialization before the source-owner request. Derive pinned metadata from the real immutable manifest and provide exact literal exceptions; do not broaden a rule or rewrite records to fit an assumption. Preserve both failed assumptions and the unchanged manifest/projection. The narrow 4,022-byte owner correction is pending at this cut; its live request supplies no verdict or execution authority. This is a practical input-preparation lesson within DoctorWatson, not a new framework or design gate.

Separately, Root reports the real Core consumer linked in 0.4186487 seconds and its actual Start/false-Resume observation passed in 0.004426 seconds, with the frozen-before-expected evidence ordering retained. Formal PR28 merged at 6cb381d. Keep those observations, their source/oracle relationships and the permanent formal/handwritten lanes distinct from the unaccepted P11 candidate and later SDK checks. No new EARS adoption or current-count claim follows from this append.


### DW114 later lesson — inspect the actual runtime boundary before maintaining dated glue

Root found that the dated P10 receipt materializer and observed_review are outside the guard/helper runtime call path. The current guard consumes direct JSON in its existing schemas, populated from truthful actual facts. Inspect that real producer/consumer boundary before rewriting a dated one-off issuer: this route changes no guard, helper, SDK check, cap or schema. A fresh NORMAL_SOURCE plus DATA_CONFIG audit is live over the complete 838 KB source/data packet at this cut; no verdict, Root adoption or guarded-F result is inferred.

The earlier 169 KB test-author request took 1,008 seconds and returned only a 2,138-byte tail with no code. Three disjoint requests of 22/29/29 KB then returned in 109/197/122 seconds and yielded the exact 124,727-byte aggregate (SHA prefix203bc). Fresh Opus accepted that source and its twelve ordinary tests; the actual twelve-test operation passed in 0.275950345 seconds. Keep the original incomplete delivery. Narrow coherent code-authoring requests around real functions and actual schema inputs instead of asking for a giant regeneration; keep their aggregate identities and applicable independent review intact.

P11's read-only bootstrap refused the existing retired empty06 root. Root preserved that failed receipt and reused the exact P10 provisioning operations2–6, all of which passed, retaining config-directory history, resources and counters. The actual fresh QOBS is 3,367 bytes (SHA prefix8c4b); the stat capture is 1,044,329 bytes (SHA prefixd6eb), covering 3,306 paths and 1,063 directories with zero source-payload reads. Provisioning is live with cleanup OWED and no P11 native compile yet at this cut. A passed setup does not erase the refusal or establish an unrun build.

The finite Core public observation and Quint common-domain comparison now report all six comparisons true. The model's reserved action identifier was corrected lexically to actionName, preserving the independent expected semantics before capture. Where already authorized, typecheck the model before spending an expensive review on a lexical failure. Keep the actual source/result cuts, independent expectation and unsupported scope visible. These are concrete DoctorWatson improvements, not a new factory, design gate, approval or EARS-adoption claim.


### DW114 later failed cut — current stat equality is not historical content identity

The actual normal Opus review61246 approved source and conditional data. Root adopted the DJ/final-root-selected config (SHA prefixb0f599; projectioncb0 unchanged), and prelaunch passed in 0.05953107 seconds. One real P11 F operation then failed PRECHECK in 13.154616678 seconds, before Cargo. Preserve the literal F_precheck.stderr: `A0 source digest changed: registry/index/index.crates.io-1949cf8c6b5b557f/.cache/ca/mi/camino`. This source-content refusal supplies no ELF or SDK result, despite the preceding source/data review and successful live prelaunch.

Current stat-metadata equality does not establish the historical input's content identity. Inspect and reconcile the raw identities of all selected mutable registry-index inputs against their retained historical pins before selecting another expensive one-shot cache preparation. The current camino/historical-maintenance relationship is under Root's investigation; no fix or new eligibility is inferred here. Preserve the original content/source constraints, safety and accounting, the failed cut and consumed marker identities. Do not requeue unchanged markers or normalize the independent resource counters to fit a sum.

The actual raw E/S/P values are 12,906,928 / 9,308,089 / 3,598,838 microseconds; cumulative values are 1,092,254,321 / 252,227,979 / 840,026,337 microseconds, with retained cumulative residual five. Root then collected five evidence products successfully in 0.081456689 seconds and completed teardown in 0.059007273 seconds. All three mounts and groups are absent and cleanupOwed is false. This retired failed cut remains separate from the earlier successful builds and from any future changed-input operation.

Separately, the actual Quint whole-machine check passed two states, alongside the earlier six complete common-domain/owning comparisons. Retain each finite comparison's own scope; no whole-language, circuit, SDK, proof or EARS acceptance follows. This append records the concrete failed input boundary and completed cleanup, not a framework, approval gate or source-constraint change.


## DW115 — Split review from authorship and check the exact wire and input syntax

The fresh native A source/test request took 919.100969 seconds and returned ABSTAIN: the reviewer reported an earlier classifier interruption, made no read-coverage claim and issued no security opinion or test source. Retain that result as an abstention. The user explicitly authorized Gemini/Muse fallback for this task; it is not evidence of a global Opus outage, and a fresh Opus002 result review worked separately.

Root split the work into a source-only request of about 277 KB for Gemini and a test-author-only request of about 32 KB for Muse. Muse returned source in 51.003528 seconds, with static/fresh-B review still pending at this cut. The first Gemini dispatch failed with `error: stream input message is missing the "event" field`; Root records zero turns and zero tokens. Reuse the exact previously successful `event:user` and `message.content` transport form rather than guessing a similar envelope. The second fresh call is live at this cut. Neither the failed dispatch, returned author source nor a live request supplies an approval or test eligibility.

B18's actual consumer link/capture passed and three public outputs were frozen. A sum-constructor record-syntax problem in the next input seed was caught before the frontend; its independent syntax revision remains pending. Check the actual input's static syntax and type shape before execution, without deriving expected values from product output or altering their intended semantics. Keep source-opinion and test-authorship requests distinct, inspect their actual consumed contracts and retain each failed cut.

Original002's four operations actually passed. The fresh result review approves only the finite current105 supported_profile rebind subject to its existing mechanical C1–C5; Root adoption remains pending and the last reported current count stays nine. The [dated status](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-split-requests-wire-inputs/STATUS.json) preserves those boundaries. Keep Clean, Quint, K and handwritten circuits active. No new factory, design gate, security grant, native result, EARS adoption or website deployment is added.


## DW116 — Resolve the current build with exact inputs and keep the next SDK check separate

P12 cleared the native build prerequisite for current WW40d4186894c93d120cf32e2ca1cb372dce902b6b and ledger9 SDK04c9c5d9bcebb8d4427d8589fb54d58a55599c14: actual exit0 in 83.773031826 seconds, including the two current SDK-check leaves. The new 473,172,040-byte ELF (SHA5d97f479d886b1e4835531588643c5bf82182227d079a248b2cc17cef64bbfca) is saved0444 and unexecuted. SDK checking, proofs and stagenet have not passed; the next new-ELF one-check review is being prepared.

The held two-lock exact camino recovery and all 2,243 historical raw hashes passed, with fresh 3,306-path stats/QOBS and the existing finite review/data hash chain completed. Root reports the seven normal QA checks passed in 0.437225436 seconds. Inspect the exact historical mutable-cache bytes before the one-shot operation, not merely their current stats. Completing existing finite conditions with actual facts suffices; do not maintain unused dated issuer glue or add human requirements to authorized beta work.

The nonexecuted snapshot preserves 3,574 files totaling 2,973,358,011 bytes (manifestaf365573be5afb2d8d59783c4ae4a672bc4766a11304ed05bdca12f0c634d540). The first preservation pass stopped after fifteen files on an unnecessary source-single-link assertion. Root retained that refusal, recorded the 76 regular Cargo hardlink aliases and copied each path into an independent single-link0444 destination. There was no native rerun. All three mounts/groups are now absent, the original20,024-byte camino and its metadata are restored, and holder80712 is terminal0 with locks released. Distinguish source aliases from destination isolation instead of imposing an unsupported blanket source-link assumption.

Cumulative E/S/P remain 1,190,123,103 / 265,033,522 / 925,089,576 microseconds, with residual five retained. Remaining conservative whole/F/supervisor values are 5,837 / 1,937 / 10,534 and cumulative F stderr79,686 bytes. Record a future owner task to make the existing failed-attempt accounting ledger data-driven, binding raw failed receipts while preserving every cap and independent counter, so each attempt need not trigger a source-constant edit and full unchanged-source audits. Do not implement that accounting change or add a framework here.

B18's three Core publications and the complete six-state Quint comparison reported all five equalities true; Root's complete comparison took 0.072341473 seconds. A pretty CLI trace may end mid-format while the separately retained ITF is complete: inspect that machine-readable product. This remains a finite representation-versus-seed observation. Keep Clean, Quint, K and handwritten circuits active. The [dated status](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-current-native-build-cleaned/STATUS.json) adds no EARS adoption, proof/target result or website deployment; the last reported current count is ten.


## DW117 — Freeze the actual SDK artifacts before the independent complete comparison

After P12's build publication merged in PR287, the current ELF's first heavy A102 SDK check actually passed in 5.822740215 seconds. Its list contained 120 selectors; the one selected libtest passed with 119 filtered out. Inside it, one positive and seven negative non-proving SDK checks ran over the fixed fixture/preprocess. This is meaningful SDK checking, not proof generation, independent proof verification, stagenet settlement or an exposed685-value SDK return vector.

Root fsynced the three actual artifacts and their directory, then froze their exact identities in manifest73df707b33412918cdb666f51ef13485f6eb9fe97ab5154d4dde362847f3afb9 before independent expected unblind. A separate passive comparison passed in 0.116572213 seconds: all27 checks, all2,077 statement lanes and their metadata, and complete program/pre/post images agreed. Its complete1,246,537-byte report is SHA7dfdf369f8f2f8cceeb9f62abb87b27846605c50fe442402c88dd52834cf0d52. Preserve this exact freeze/order and full comparison scope rather than rerunning the SDK consumer to inspect its values. The source/result Opus review remains pending at this dated cut.

Original C1's whole-host wording was ambiguous. Preserve the reviewed explicit shared-host amendment instead of pretending the original literal already allowed it: the Root/Moriarty slot stayed strict, load<nproc−1 and MemAvailable≥4GiB were real selection gates, and they passed. Caps, one-shot failure behavior and no-retry obligations stayed unchanged. Root's staging argv-index error stopped before mkdir/copy/ELF; its metadata-only correction and original failure remain recorded. Correct the actual adapter boundary without rewriting source or refunding a consumed operation.

DraftPR288/head5a321beb contains139,011 bytes across three new Rust circuit sources, unregistered and uncompiled; this A102 result transfers no semantics to them. The current120-selector ELF has no A102prove consumer. The next concrete source task is one test through the actual SDK proof APIs, with independent statement/verification obligations, not new infrastructure or assumed proof eligibility. Keep CleanB24, Quint derived evidence, K Lt counterparts and handwritten circuits active, and preserve website design-log iteration without deployment.

The [dated status](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-first-sdk-check-independent-freeze/STATUS.json) adds no EARS closure: last reported current10/historical16. General interpreter/refinement and complete prefix/cost/financial correspondence, StepInv/uiValid/F4/G4/projectA/continuity, native proving/independent verification, full statement and financial binding, ACTUS/DeFi objectives, required Preview/Fenrir qualification, Midnight stagenet settlement and release/composition remain subject to their retained acceptance evidence. C4/R2 and stoppedAm10/T3/k17 boundaries and every cap/raw failed receipt remain intact. The website goes live only after the whole Sherlock goal is finished and accepted.

Root selection after the DW117 author freeze: Opus review7517 returned REVIEW_SCOPED_RESULT_APPROVE; R1–R7 were recorded without rerun. Seven rejected SDK invocations cover four distinct corruptions. PR284 merged at 912df19f8de393da06d4f39212ffc2f59a00d271. The full current WW manifest (44,969 file identities) matches the compiled cut. See the SDK archive result-review and Root adoption records. The next proving consumer is authored but not compiled or run.


## DW118 — Recover from the current graph cut and preserve the proof/verification boundary

P13 compiled the real proving consumer in95.383643714 seconds (Cargo24.32s). Its later combined proof-and-independent-verify test produced a real k18 proof of3,440 bytes in a31,390-byte bundle, SHA5ec12ca7495db2201e9fafcf4af50a2478794910b33c499412541f1ca728b4bc. The saved bundle is a subsequent call-return artifact from source that returns only after producer-VK verification: a source-consistent producer-return observation, with fresh result audit pending. The combined test ended SIGKILL/-9 after503.384773 seconds and596.847982 childCPU seconds, before the independent verification report. Near-CPU-cap timing is consistent with RLIMIT_CPU, not a direct kernel cause verdict. Do not call this a full test pass, independent verification or stagenet success, and do not retry the combined test.

Root's eighteen pure data checks compared every2,077 public value, full claim metadata, exact program and IR against the earlier independently accepted SDK artifact. This is qualified transitive data evidence, not independent cryptographic verification. Scope termination left zero PIDs. Current hybrid E/S/P are1,903,890,457 /280,862,180 /1,623,028,271 microseconds, residual six; appended waited-child/controller measurements are disclosed rather than relabelled as new raw cgroup counts. The next concrete task is an Opus-owned verify-only consumer of the saved proof, not reproving. Preserve all previous failed cuts, caps and no-refund accounting.

CleanB24R3/source16443B6e576b05 actually passed its kernel check in2.013854957 seconds with empty streams, after the retained R2 syntax failure; source F8c70090 is published with PR pending. K Lt's first compile passed in15.911 seconds without orphans, producing15 files including the1,903,957-byte Kore definition (SHA prefix456fdb3b); the old unused-O warning remains. There is no K runtime, distinct new oracle or formal-correspondence closure. B19's qualified six-state comparison and Quint derived evidence retain their finite predicates. Keep the next handwritten task beside them: Lt's three draftPR288 files are still unregistered/uncompiled, requiring Opus-owned registration/native checks; Mul17 remains unchanged/uncompiled. Keep website design-log iteration without deployment.

Actual Graphify71613ff40e2e212bd854ffa70087d740928caa5ac57f3bb1aea6c8edaede5e37 is installed:111,729 nodes,321,393 edges and206 source hashes rechecked. The canonical CLI query passed with19 route nodes/24 edges; its roughly2,848-token result illustrates a soft budget against the requested 1,400 tokens. Old graph rows and their dated context remain intact; the Graphify owner is preparing a separate small current-proof result append. On recovery, use the graph first, verify the current source pins and refresh only changed sources before following exact full bodies. Get liveness, terminals and resource counters from latest Root records and actual tools, never static graph status. The advisory Sherlock KG is not main Graphify.

The [dated source cut](../docs/results/sherlock-four-lane-progress-20261018/addenda/20261019-proof-consumer-compiled-graph-recovery/STATUS.json) preserves the earlier preparation snapshot separately. Last measured current10/historical16 is retained without remeasurement or EARS adoption. General interpreter/formal/financial correspondence, independent proof verification, full statement/financial binding, target settlement, C4/R2, stoppedAm10/T3/k17, resource and release obligations remain subject to actual acceptance evidence. This is compact DoctorWatson recovery, not another factory or gate.

### Graphify installation and recovery update — October 19, 10:50 UTC

The current graph is installed at `/home/charl/Moriarty/graphify-out/graph.json`: SHA-256 `833be6cfef8cb6cd67979f525b86bada5e327ad2f4607277b915236acc23e87c`, 111,730 nodes and 321,396 relationships. The latest result adds one node and three relationships; all historical ordered rows and existing metadata remain exact. A fresh source check found all 206 indexed source files unchanged. The dated proof-result node supersedes the earlier “proof not run” snapshot; independent verification remains unfinished. The actual query returned 12 nodes and 17 relationships. Its roughly 2,045-token output exceeded the requested 700-token budget; treat that option as a soft target.

On every recovery, start with `graphify-out/SHERLOCK-CURRENT.md` and `SHERLOCK-CONTEXT.md`, then query the relevant named symbol with its explicit graph and source root. Check the source manifest before relying on line references, extract changed files only, and read the selected source bodies. Use relation filters or single-node lookup for narrow context. A saved graph is not a live process or accounting probe. Preserve separate source roots for historical layers; never resolve all relative source paths against the main checkout.

Clean B24 is merged into the Fenrir fork in PR 31 at `1847c4e6db59c78e05e857941bd4db9581ece8dd`. The primary next task remains a verify-only consumer of the frozen proof, with no repeated proving. Keep Clean, Quint, K and handwritten circuit implementation active. The graph is an index for this work, not another execution framework.

## DW119 — Put recursive stagenet acceptance ahead of graph maintenance

The user makes actual Sherlock recursive-proof validation on Midnight stagenet the first outcome, followed by a deterministic Fenrir-derived synthetic rollup and k/performance experiment. The current loop, experiment and additive SRPOC requirements now name that order. Distinguish circuit degree k from K semantics, literal prerequisite evidence from recursion, and local verifier time from RPC inclusion/settlement latency. Keep graph refresh cheap and scoped; then return to the missing recursive circuit/consumer. Do not spend another iteration rebuilding infrastructure or reproducing an unchanged proof prefix. All experiment acceptance remains open.


### DW119 recovery addendum — revision04 complete cohort and source-delivery boundaries

The broad source response was partial despite provider exit 0. A bounded leaf later delivered the complete 905-line Moriarty A80/link module. Preserve both dated replies: the later source does not fill missing Fenrir parent/terminal/harness code or supply review, compilation or a recursive endpoint. Split provider work into complete source leaves and inspect the requested artifacts, not just transport status. C4/T1's five source files are installed and Root reports source/evidence pushed at `3dd96307`; their child binding and early public key/instance/claim persistence still require actual child checks.

The one-Camino proposal received source approval and four exact ordinary method passes. A subsequent source precheck visited all 2,243 rows and found two additional refreshes, `find-msvc-tools` and `cc`; full row output was not persisted before the assertion. That candidate is not eligible for the actual compile. It stopped before provisioning, ONE_SHOT or native execution, did not mutate the host cache, and its own flock holders exited with no cleanup/restoration owed. Passing a focused seam test does not establish the whole input cohort. Historical helper/A0/D pins stay historical. The designated source owner is preparing a separately frozen complete immutable current-index cohort, not extending per-file exceptions or rewriting old receipts.

The first general-doc review failed with `malformed_tool_use_exhausted`, exit 1 and provider error after two turns; it returned no verdict. Do not classify that as a security refusal or quota result. Replace the obsolete review context with bounded complete changed entries, exact surrounding lines and current receipts. The actual Opus website concept is copied verbatim; later engineering facts belong in its separate status record, not a forged author rewrite. The site still launches once the full Sherlock goal is complete and accepted.

P14's verify-only timeout remains consumed and has no verifier verdict; no unchanged replay is selected. K's ten phases and independent complete finite-state equality retain their own uncharged profile. Neither source delivery, prelaunch refusal nor model equality supplies a native/recursive proof, stagenet endpoint, performance measurement or EARS closure. [Source facts](sherlock-circuit-atlas/provenance/child-parent-source-frontier-20261019.json) preserve the cuts.


## DW120 — Freeze complete declarations and qualify the exact observation

A source/API review needs the complete declaration body through its closing delimiter, not a line range that stops at a struct opening. Pin and provide the actual fields, enum payloads, signatures and source regions used by the consumer. A whole-file hash identifies the unseen bytes but does not make a clipped declaration reviewed. Keep that omission separate from a typecheck or runtime result; the K complete-image compile retains its overall/Rust API ABSTAIN rather than converting source clarification into a new verdict.

Preserve raw receipts and qualify their predicates. In the [Quint B22 batch](../docs/results/sherlock-quint-parametric-literal-images-20261019/README.md), all four operations and eight directed tests passed. Each phase had 4,678 before/after source/tool/symlink checks and exact three-file model-directory membership. These are snapshots, not continuous immutability or a complete host-access/extra-file monitor. The raw `sourceAndExpectationsUnchanged=true` remains a literal in its original outcome. The separate EXPECTATIONS JSON was not in the eight phase snapshots; its later match to the frozen pre-execution review pin is a narrower post-readback observation, recorded separately. Do not rewrite the raw outcome, backfill phase coverage or fit expected cells to observed output. The 6,800-byte capture-helper prefix is also a prefix, not a full execution closure; the separately pinned full helper and retained review limits matter.

The [new K definition](../docs/results/sherlock-k-literal-complete-images-compile-20261019/README.md) compiled in 14.951 seconds and produced a 1,068,521-byte `definition.kore` (`df05e8d632013956de8704c61aeef513b53c33c775736f11b7a3aeb1f7222ac9`). This is fresh definition generation, with no new complete-image runtime comparison. In Fenrir, [Clean B28](https://github.com/CharlesHoskinson/projectfenrir/blob/9250d9ed/clean/fork/docs/results/sherlock-clean-complete-literal-statement-20261019/README.md) checked six finite Nat facts in 3.379900 seconds: 174 postcheck pins matched and five outputs totaling 526,120 bytes are retained by pins. Complete represented Program/Init/middle/Done recovery and derived child views do not establish Fr/hash/crypto, production correspondence or native/target acceptance. The B22 expectations are finite, same-author and source-exposed; none of these three results supplies a new EARS closure.

A wire bump is connected implementation work. The unreviewed A80 source now declares `moriarty-sherlock-link/2`, including per-child qualifications/obligations, provenance and optional carrier identity. Its private verified-export seal and checked offsets are source mechanisms; intrinsic manifest validation neither reads sibling files nor authenticates keys or proofs. The reviewed parent still reads `/1`. The `/2` parent reader must be reconciled, independently reviewed and actually integrated; do not label a new version string, metadata map, silent downgrade or permissive parse as compatibility. Keep the exact 16-statement-plus-12-accumulator route distinct from the stock 17/29 CLI and the missing current anchor/Template integration.

Root reports Moriarty commit `34a8593b` and Fenrir commit `9250d9ed` pushing at this dated cut, with PR numbers pending; this entry infers no merge or target result. The [source facts](../docs/results/sherlock-dw120-design012-preparation-20261019/SOURCE-FACTS.json) pin the actual records and retain the uncompiled/unreviewed source boundaries. DESIGN011 remains byte-identical. DESIGN012 is a separate pending Opus 5.5 HIGH creative contribution, not Sol-final website copy. Documentation and concept preparation run asynchronously alongside the native/circuit work; no new factory, execution permission or build hold is introduced. Sherlock's recursive stagenet and Fenrir experiment remain the priority.


### DW120 later creative-publication observation — October 19

The actual Opus 5.5 HIGH DESIGN012 author returned successfully after 254.400744 seconds. Its complete note and README append are materialized verbatim, with explicit marker spans and an exact reconstruction proof; no Sol creative replacement is presented as Opus. The [separate status](sherlock-circuit-atlas/DESIGN-012-STATUS-AND-PROVENANCE.md) preserves the author's input cut while recording Root's later actual merged readbacks for Moriarty PR294 and Fenrir PR35. A later 6,106-byte uncompiled anchor source is also Root-reported, with fresh review pending; it does not imply target integration. Keep the original source-cut facts, raw review qualifications and all old concept notes intact. The site still launches only when the full Sherlock goal is complete and accepted, and this documentation adds no native/circuit build hold, recursive proof, deployment or EARS closure.


## Proposed DoctorWatson improvement — Complete expectations and byte-bound observability

Status: documentation draft only; no new checker, source fix, execution or acceptance decision. Base: Sherlock `35931362` (PR297).

A successful author exit does not issue an independent expectation. The recorded327-lane enumeration failed the required335-lane geometry and remains rejected without repair or comparison. Keep that raw failure and the original contract; a fresh deterministic constructor is a changed approach, pending its own actual review/validation. Validate full shape before comparison without feeding actual values or old differences to the new author. Freeze the output before parsing and the actual before any independent expected read. No private expected/capture arrays belong in the public improvement log.

Treat observability as a source change when source bytes enter identity. Both current9-file A102 and13-file complete relation lists include `literal_bridge.rs`; contexts incorporate the relation hash. Logging in that file changes the identity inputs. The deferred diagnostic candidate therefore inherits no saved-proof/context result, and does not justify another observability build. Preserve the old proof and current source cut separately; semantic equivalence and exact source identity are separate questions.

Publish only newly observed predicates and eligible reviewed bytes. Do not rerun passing prefixes to refresh a dashboard or call unrun K/Quint/Clean/native stages passed. Keep the full Sherlock goal and all permanent lanes active; private Fenrir research stays private. A later outcome entry must cite actual new terminal records and state precisely what was compared before any currentness or acceptance change.

Later dated observation: B24's actual new frontend and one sample both exited0. Root's original `[ok]` classifier failure is retained; whole-output `ok` adjudication corrects classification without rerun or changing the model/test/expectation. Preserve exact CLI-envelope recognition and require the full named outcome, not invented punctuation. The output is121bytes; a parent shorthand126 is corrected by the retained raw bytes. K's new actual reduction passed, then decoding failed on the reported empty-continuation seam; freeze the59 actual-file pins and continue only the changed decoder/unit/tail once eligible. Its07/08 comparison stages remain UNRUN. These are scoped new observations, not whole-K/Rust correspondence or EARS closure.

Later K tail observation: all four newly selected operations passed (20 synthetic grammar cases; changed decode; complete335-lane audit; whole typed independent-prefrozen comparison). No engine/kast/print prefix was replayed. Retain the original decode failure/59-pin freeze and actual-before-expected order. This is typed equality with different raw hashes, under the declared shared mapping, not original main.k firing, context authentication, native proof/target or EARS evidence. Preserve source identity and result scope while continuing the new optimization EARS/OpenSpec work in its exclusive owner worktree.

Later host-to-K data observation: Root's prefrozen-field-mapping comparison reports2262 typed recursive checks/zero mismatches over all8machine positions,15Program/6State physical tables/padding,2envelopes and335lanes, after only canonical host decimal-string-to-UInt128 conversion. Neither43824-byte host nor6886-byte K observation changed. Host-only metadata/coordinate names remain unpaired, old `NOT_ISSUED` capture wording is historical, and context binding remainsfalse. This passive data comparison is not new SDK/circuit execution, original main.k trace/refinement, context authentication, native proof, target acceptance or EARS adoption.


## October19 lesson — Name the normal Core and selected-handler observation

The [body-typed nominal24-operation observation](../docs/sherlock/reference/nominal-body-typed-cli-observation-20261019.md) passed a full normal Core build and an external consumer with actual production Args, source reading, selected handler and publication modules. That explicit consumer scope is useful evidence; it does not stand in for a whole backend-linked `mori` executable or native/target journey. Report which real modules were linked and invoked beside each result.

Keep test listing as an independently checked membership predicate: the default-pretty listing contained exactly21 names and its21-test footer; each subsequent exact invocation passed1 with20 filtered. A zero exit alone does not identify the selected harness. Preserve the initial20 pre-wiring freezes, incremental21st before its later retention behavior, and post-wiring/pre-execution UInt/Bool corrections separately. Date earlier UNRUN records and append actual results without rewriting their original provenance. This is an observed documentation lesson, not a new approval policy, security judgment or EARS closure.


## October 20 actual-batch lessons — source05, cohort05 and repair03

Proposed documentation lessons only, grounded in the [dated receipts and pins](../docs/results/sherlock-source-workflow-batch-20261020/BATCH-SCOPE.md). These do not change reviewer ownership, caps, operation authority or product acceptance. Retained typed lesson 84 and earlier log entries remain unchanged.

1. **A source/API approval is not a parser result.** Lean05 received prospective source/API and one-operation eligibility approval, then the actual leaf failed at 32:72 and 148:70 on doc comments before `mutual`. The 89:12 unused-simp warning also violates the existing strict zero-output requirement. Keep the failed raw result. The source06 doc-placement and simp correction received its own fresh review and strict kernel pass (0.554462 seconds, zero streams, five products). This advances only the isolated logical-Value equality theorem and seven examples; it never relabels source05 or accepts the whole B55 Host/native bridge.
2. **Select tests by their declared run names.** `--match=.*` selected imported stateful declarations as tests. 34 authored tests passed, yet the command exited 1. Keep that original failure and report corrected 24+5+1 selectors separately. A passing subset never rewrites the enclosing command's result.
3. **Quantifiers have runtime cost.** Repair 03 contains nested universal checks over retained histories. The requested 10,000-sample run timed out after 30.032008 s with empty output, while the directed path passed. Treat quantifier cost as a hypothesis needing measurement; no profile proves it caused this timeout. Do not turn a timeout into coverage, an invariant failure, a silent bound increase or an invariant waiver.
4. **Recover the whole immutable cohort, not just familiar filenames.** Root's exact 1,735-file/87,569,159-byte check binds membership, contents, read-only modes and manifest SHA256. It does not approve dispatch, execute native code or establish continuous immutability after that check. Preserve the manifest and original receipt when selecting a separately eligible operation.
5. **Public fixture hashing is not SDK execution.** ScalarConstruct05 has five complete source files and 26 test functions, all uncompiled/unrun; literal raw count 14 supersedes the owner's stated 11. Source presence, syntax/name inventories and hashes establish different predicates from Rust typing, preprocessing, synthesis or proof. Draft PR361's native test source adds no native result.
6. **Bind recovery to the original process and result.** The finite workflow model checks task/token/attempt/fingerprint/original-result dependencies; it does not authenticate a real executor. Keep unknown originals immutable and require evidence for the same original operation before resolution, rather than borrowing another process's output or resending an already-consumed identity. Lesson 84's format-decoder and unverified-settlement warning still applies; parseable JSON or a coherent owner flag is not settlement or proof verification.

Current next actions are exact-source Construct review/checks, the remaining B55 Host/serialization bridge, bounded workflow-cost diagnosis and real producer/attempt/output binding. Producer revision02 now has a separate source-with-limits approval; its 14 tests and native path remain unrun. None implies full39, StepInv, P1/native issuer, finance, recursion, stagenet or EARS closure. The designated Muse/Opus circuit concepts remain separate creative source; the finished site goes live only after full Sherlock completion.


## October 20 later lessons — comparison domains, bounded resources and real entry paths

Proposed factual lessons from [this exact later batch](../docs/results/sherlock-source-workflow-limits-20261020/SCOPE.md), not a new runner, review policy or acceptance gate. Earlier entries and actual failure records stay unchanged.

1. **Name the resource amendment precisely.** Full Clean09 passed only after two fresh votes and Root adopted one invocation's per-file 16 → 64 MiB change. The existing `resourcesPerLeafNotRaised:true` description was explicitly inaccurate for that limit. Record the actual changed dimension and preserved limits; neither success nor a matching hash converts a one-run amendment into a standing policy or whole native bridge.
2. **Check comparability before claiming equality.** K compiled with classified warnings and later qualified36symbols, yet six independent local-row specimens do not share the retained normal-window event-site contract. Keep NONCOMPARABLE distinct from mismatch, pass and missing-source assertions. Declared source-site support and a valid comparison-domain mapping are separate predicates.
3. **Keep directed coverage and incomplete sampling apart.** Quint06's96 named tests and20 directed witness rows passed while its sampled deadline failed with all witness counts UNKNOWN. Performance07 binds actual installed TypeScript cache/invalidation source and retains10,000 × 60/seed1/30 s / 256 MiB; this is source-supported optimization work, not a measured speedup or permission to weaken workload/invariants. Earlier04zero counts belong only to04.
4. **Exercise the actual entry and cleanup premises.** The two native07 reviewers found an unlaunchable real provisioning/config order and abort ownership problems despite author mock successes and fixes to the three earlier defects. They also identified GNU `time -o` output precreation, outer fingerprint/helper identity, mount-absence and typed-debit seams. Preserve these attributed findings; a return-zero fake operator is not evidence for the actual process, filesystem or receiving contract. Source and eventual real execution still need their distinct evidence.
5. **Record failed delivery as failed delivery.** Muse429 and actual Opus quota failures deliver no new completed source approval. An availability response or dispatched fallback handle is not an implemented correction. Preserve source ownership and actual returned identity; use the standing authorized fallback without adding a human approval flow or relabelling a failed attempt. Marker32source and the separate Core1candidate retain their UNRUN status.

The user beta override remains human-free with the existing objective source/resource/correctness conditions. No SDK/native/proof/target acceptance, full39/frontend U0–U7, finance/refinement or EARS closure follows. Website concepts remain designated Muse/Opus creative source, unchanged and unpublished until full completion. No new visitor copy is authored in this append.


## October 21 UTC — bind the review to the receiving source

These are dated factual lessons from [this batch](../docs/results/sherlock-receiving-boundary-lessons-20261021/SOURCE-PINS.json), not new policy, permissions or acceptance rules. Earlier failures, approvals and resource records remain unchanged.

1. **Mock success and actual receiving compatibility are different predicates.** Root’s selected source08 suite passed170 tests, including41 adapter mocks. Fresh Opus laneC nevertheless returned REVISE: its N1 identifies the actual guard’s flag-prefixed, long process argv versus the decoder’s absolute-first-token/128-element assumption; N2 identifies nonexistent guard constants leaving intended remaining-wall and D/E-freeze branches inactive; N3 identifies an unguarded post-native directory failure that could prevent terminal records. These are attributed reviewer findings, not Sol security rules. A fixture that invents the receiving process shape can agree with an incorrect decoder.
2. **A returned APPROVE is not usable evidence when it describes another API.** Root withheld reviewerB’s source08 opinion after checking absent function names, wrong context paths and the wrong time-binary hash. Preserve that returned opinion and the withholding record together. Re-labeling the verdict or repairing only its prose cannot bind it to the real candidate.
3. **A new namespace does not erase old attempts.** The explicitly new DoctorWatson build namespace has its own initial empty history/reconciliation records. Its provenance retains the actual legacy references, source/resource accounting, unknowns, cleanup duties and stopped C13. Empty local lists are not a global counter reset or authority to replay an old attempt.
4. **Keep an executed public route separate from its next source change.** PR363’s real nominal compiler-to-CompletedRun handoff passed six selected operations and three positive tests. Event/value comparisons were source-author assertions and shared-owner comparisons; complete compared values were not separately serialized. The nine-file source06 simulate change and its revision02 test-only environment-macro correction remain a different, unexecuted cut. Do not turn compatibility with a dep-info gate into a CLI execution result or rewrite the unchanged guard/helper to accommodate a fixture.
5. **Report each formal layer’s exact domain.** K’s15 finite Boolean-law checks passed separately from compilation; Quint’s five tests plus one installation witness and Clean’s seven mathematical theorems plus two examples answer different questions. Compilation products alone are not evaluated laws; synthetic UNADMITTED observations are not genuine Core admission; conditional mathematics is not an SDK/Rust extraction proof. Their exact actual receipts and open premises must accompany any reported pass.
6. **Correct receipt-contract prose without rewriting authored history.** The fresh Opus ValidateEntry opinion and Root’s scoped adoption say `const_found`, `const_count` and `const_row` are all ungated derived receipts. UPDATE026’s exception parenthetical is false; its five creative strings remain immutable, with a separate engineering erratum required. Consumers bind constrained carrier lanes, not candidate receipt values. The opinion approves only the exact bounded emitter source, not a later consumer/test source, native proof, measured degree or performance. Reviewer elapsed wall time does not measure reviewer CPU.

The next work remains the actual circuit/consumer source and eligible native path. Source09 author work is recorded only as in progress at Root’s task cut, not delivered source or a result. Full frontendU0–U7,39selectors, StepInv, finance, formal correspondence, native recursion, stagenet settlement and EARS obligations remain open. The human-free beta override remains unchanged. Website concepts remain designated-author prose; the site stays unpublished until full Sherlock completion.


## 2026-10-21 — DW-FLOW-04: integrate ready source before expanding the queue

**Status: documented workflow revision; execution and acceptance remain separate.**
The [receiving-boundary lessons](../docs/results/sherlock-receiving-boundary-lessons-20261021/SOURCE-PINS.json)
record why source review must use the actual receiver: a returned approval can
describe an absent API, and passing mocks can miss the real producer shape.
More workers help independent missing relations but also add integration load.
Prefer Root integrating ready source, then the next executable consumer check,
before opening more research.

The [current loop](../docs/sherlock/implementation-loop.md#current-operational-queue)
now names one integration owner and a queue toward native build, linked recursive
proofs, independent verification, stagenet effects and Fenrir measurements. Each
task names one product predicate, actual API and immediate next call in one
current continuation. Clean, Quint, K and handwritten implementation stay active.
Root generates exact identity metadata mechanically; semantic reviews quote
changed source and adjacent APIs or give counterexamples. Test-only edits use
focused checks, and two same-class failures require a changed approach.

Next owner: Root integrates the exact reviewed documentation and affected trace
pins, then advances the eligible receiving source/check. Validate improvement
against actual integrated behavior and observed review/check latency; this entry
claims no measured speedup, armed runtime, native success or acceptance. Earlier
failures, dissent, resource/cleanup history and security obligations remain.


## October21 UTC — actual receiving APIs and complete-data checks

Dated implementation results; prior failures remain immutable.

1. **Compile the actual receiver early.** Ordinary compilation found E0609 field errors and E0433 (`FuelPolicy`) missed by static review. Keep the failure receipts and dated normal05/cfg06 qualification. A static review is not a typecheck.
2. **Use one declared-schema mapping across all callers.** Public artifact admission has six fields; complete-snapshot admission has four. Source07 corrected the top-level test, but the next quote run found the same invalid whole-object equality in its shared assertion helper. Source09 must centralize the full-value mapping and inspect all callers before another run. Keep exact field sets and semantic expectations. The original twelve attempted Core tests produced eight passes and four failures; eighteen remained unrun. The later source07 quote failure is separate evidence, not a passing semantic comparison.
3. **Preserve full data within the real export limits.** Fitting the compact trace budget did not ensure its pretty diagnostic fit the public exporter. The designated source08 remedy must preserve every represented field and record under the existing limits. Keep the direct-export assertion and independent71/145 expectations intact; do not substitute a summary or larger cap.
4. **Compose owned functions, then inspect their receiving boundary.** Root combined native03 policy and native04's three owned parser functions, discarding overlapping alternatives. Two remaining seams were identified: author/voter PROMPT visibility inside the worker and honest unknown-returned-identity routing. Source06 addresses those seams. Composition and source delivery alone do not establish native readiness or a successful build.
5. **Report the exact formal result.** K's twelve new return-projection tests passed. Both complete recursive typed return trees and original23-field source identity matched unchanged independent expectations. Full NG states and ordered74/108 events remain retained without an independent whole-state/event comparison. This is no Core-clock, admission, refinement, native proof or EARS closure.
6. **Publish private formal source with its limits.** Clean PR72/73 merged twenty-nine exact source/evidence additions into the private fork. Source15 derives the represented logical observation window from source-pool premises. Native occurrence joining, charged Rust schedule correspondence, full Lake and target proof remain separate obligations. Public references must preserve the private-fork boundary.

Continue the existing DoctorWatson loop with local affected checks and regular reviewed integration. Retain all Sherlock, financial, native, recursive, stagenet and release obligations. UPDATE026's five creative strings remain unchanged; the site remains unpublished until full Sherlock completion.

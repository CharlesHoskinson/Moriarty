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

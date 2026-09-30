# W-D2H bounded design result

**Status: proposed / specified-only, 2026-09-30.** This is an author design
result, not an approval, adoption, implementation or authentication result.
W-D2/W-D3 and B01–B17 remain open.

## Delivered scope

Current files are [SPEC](SPEC.md), [DECISION-MATRIX](DECISION-MATRIX.md) and this
RESULT in `formal/mil4/hash-images/`. Four exact reviewed prior three-file
snapshots are preserved under [history/pre-grok-repair-01](history/pre-grok-repair-01/SPEC.md)
and [history/pre-grok-repair-02](history/pre-grok-repair-02/SPEC.md), plus
[history/pre-grok-repair-03](history/pre-grok-repair-03/SPEC.md), plus
[history/pre-grok-repair-04](history/pre-grok-repair-04/SPEC.md).
Only those fifteen document paths are authored/preserved by this sprint. W-D2F/W-D2G,
source modules, fixtures and protected records are unchanged by this author.

Repository inspection covers current W-D2F FIELD-MAP/PLAN, W-D2G SPEC,
Source/6 frontend/grammar/contract, Core/5 preparer/S0 contract, the Source
wrapper and wire/3 specification. The development skill was loaded and guarded
status inspected. Status still reports unresolved product execution/admission
gaps; no registered execution was dispatched. AGENTS' historical September10
ACTIVE-ROUTING/assignment paths are absent in this worktree; no execution or
model-routing substitution is inferred from that absence.

## Findings and recommendation

Repository observation: Source/6 is a builtin-selection stage proposal, not a
user-defined program body. Its embedded Source/policy claims are opaque and
currently copied unchanged. Whole-document hashing without exclusions would
self-reference `source_hash`; no current producer resolves that issue.

Recommendation: select the canonical instance-scoped Source definition
projection, an exact raw three-module Core implementation package, and an
exact scoped policy that commits computed Source/Core digests. SPEC gives
included/excluded bytes, fixed purposes, primitive widths, package roles,
static literals, action variants and acyclic generation order. A grammar-located
two-token document projection is a defined preimage alternative only; its full
authenticated consumer/policy linkage/schedule is absent and it cannot replace B.

Inference: this split makes Source presentation changes harmless to the
definition while preserving exact implementation identity and explicit
policy terms. It does not authenticate any artifact or prove that committed
source bytes were executed. B06 still needs independently verified loaded
artifact/toolchain/lowering correspondence, and B16 must replace the local
lowerer's incomplete replay projection with an adopted authenticated transport.

SPEC preserves W-D2F's tag order:8/11/12 compare authenticated artifact hashes.
At12 it immediately compares the policy body's nine already-bound identities/
hash links to retained context, then compares the current policyHash. Authentic
inconsistent body identities receive policy.<field> mismatch diagnostics at12.
Retained asset/scale/later policy terms compare at their own later fields.
This avoids using an early candidate hash to move a cap/endpoint mismatch
ahead of its specified anchor. Upgrade consequences include fresh
signature evidence for changed wire claims, conservative Core hash changes
even on package comments, explicit suite/version binding and no implicit
state/replay/registry reset. Wire/3's existing content-digest/signature-message
boundary is unchanged.

The decision matrix defines2 positive construction oracles,1 presentation
invariance oracle and28 hostile/invariance design cases, including self-reference,
selected-action mismatch, exact-code mismatch, policy/context mismatch,
zero-fee recipient, lowerer substitution, incomplete history and cross-purpose
digest collision. These are expected predicates, not frozen digest vectors
or executed outcomes.

## Review feedback repair

The parent supplied prior Grok xhigh packet-review feedback: one high defect
for missing immediate policy-body identity comparisons, plus medium ambiguities
in exclusions, provider precedence, oracles, bytes, correspondence and upgrades.
The reviewed files are preserved byte-for-byte in pre-grok-repair-01; the
repaired current files require fresh independent review. This author does not
claim approval or adjudicate the review as a successful decision vote.

The repair adds exact tag12 comparison/diagnostic order and hostile authentic
but internally inconsistent policy-body controls; tag23 policy/B14 and tag35
policy/B11 precedence; separate H-H14A/B/C old-hash versus corrected-hash
declaration controls; exact scalar UTF-8 plus Core opaque keyRef restrictions;
actual package export/label checks and false B16-handoff evidence controls.
Policy scope now explicitly excludes repayment debtor/creditor and keyScheme
and names their signed/B11/B09 anchors. F-S0-01 applies only to the closed
failure relation, and upgrades distinguish package-byte/image-schema changes
from separately required runtime correspondence changes. No behavior was coded.

A fresh Grok 4.7 xhigh static review of repair-01 was reported by the parent
with no high finding and four medium findings. Its exact three files are
preserved in pre-grok-repair-02. Repair-02 adds the six tag8 Source body checks,
their exact diagnostics and body-before-hash order; replaces ambiguous fallback
wording with authenticated suite selection; defines B14's genuine grant-scope
first failure/paths; and gives invalid policy keyRef an exact image-domain
rejection before tag12's nine body checks. Existing H-H14A/H-H21/H-H23 oracles
are refined without changing the2/1/28 counts. This new candidate remains
specified-only and requires fresh review; prior review does not approve it.

Fresh Grok 4.7 xhigh repair-02 review was reported by the parent to close those
four medium findings, with no high and two new medium first-failure gaps.
Repair-03 preserves its exact three-file input in pre-grok-repair-03, then adds
Source authenticated debtor/creditor cells before genuine B11 comparisons,
distinct claim/fact diagnostics and the three X/Y isolation controls. It also
fixes asset/tag16 and scale/tag17 total order and owner/fact paths across
B10 authentication/scope, B01 membership, B10 current facts, B05 definition
and B07 policy, aligning the proposed integration schedule. H-H14C/H-H26 are
refined in place; the2/1/28 counts remain. The integration proposal was inspected
read-only at formal/mil4/integration/SPEC.md. No implementation or adoption occurs.

Fresh Grok 4.7 xhigh repair-03 review was reported to close the prior two
medium findings, with no high and two new diagnostic contradictions. Repair-04
preserves the exact input in pre-grok-repair-04. It distinguishes the asset/scale
old-hash case from a paired selector/constructor switch (fourth prior-body
comparison) and an unpaired kind (image-domain rejection). H-H14C now labels
direct Source, missing/invalid B10, genuine B10 scope and each later asset/
scale fact separately. A whole-document first-failure wording scan also narrows
H-H14A's early control to actual Source literal differences, removing a blanket
claim of a provider comparison at tag2. Counts remain2/1/28. All revised rules
remain specified-only pending fresh independent review, with no code or tests.

## Evidence limits and next bounded work

Implementation changes: **0**. New encoders/consumers/verifiers: **0**.
Tests added/run: **0**. Computed Source/Core/policy digest vectors: **0**.
Authentication/correspondence proofs/native evidence/ledger submissions: **0**.
Fresh audits of the repaired current candidate/decision votes: **0**. Prior
review feedback is recorded above; it does not approve these new bytes.
No gate is closed.

The author performed a text consistency review of field inclusion, dependency
cycles, tag ordering, selector roles and claim-versus-computation language.
This is not an independent audit or an executed design-oracle result.

Next bounded work: independently review the proposed suite versus the document
alternative and the exact-code-versus-abstract-Core choice. Only after adoption
freeze independent image bytes/digests and hostile expectations, then implement
the smallest image encoder/comparator with its honest content-only scope.
Full authenticated consumption additionally needs actual B01–B17 definitions
and evidence at their existing gates; B17 remains a separate ledger result.

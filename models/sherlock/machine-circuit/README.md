# Sherlock A semantic output map

[A-REFERENCE-OUTPUTS-EVAL.json](A-REFERENCE-OUTPUTS-EVAL.json) is the exact
54,334-byte P1 r3 semantic source map for the 39 current typed Rust step events.
Its SHA-256 is `6823badbbca40cb945881ae689cd812f4ddca3aceee9b7ec8dbfde5cf9e122c2`.
Full Opus and Muse source votes and the Root adoption cover these bytes under
the conditions below. This is semantic source adoption only: full P1 remains
unfinished, and this publication candidate requires its own complete reviews.
No compilation, runtime test, circuit acceptance or native result is claimed.

The JSON's original `UNADOPTED` author label is retained byte-for-byte as
history. The later [Root record](source-review-20261016/ROOT-FULL39-SOURCE-ADOPTION.json)
records the scoped semantic adoption without rewriting the reviewed proposal.
The original twelve row arrays remain unchanged, but the full39 common lines
and all current conditions now govern them; the old scoped12 record alone
is insufficient.

The map has 36 charged events and three free publications. The pinned
[A-RULE-OPERATIONS.json](A-RULE-OPERATIONS.json) and
[Markdown view](A-RULE-OPERATIONS.md) retain the full R5 semantic dependencies:
42 K machine rows, 71 pure K helper rules, 14 inline-operation groups and all
eight Snapshot cells. A074 is constructor startup; A078 negative startup and
A105 invalid literal are outside the typed Rust input domain. They are not
additional Rust step events. Names outside the existing canonical signed-decimal
K common domain remain host String semantics, not an invented K correspondence.

The actual normative sources are
[reference.rs](../../../crates/mori-core/src/sherlock/reference.rs),
[reference_types.rs](../../../crates/mori-core/src/sherlock/reference_types.rs)
and [main.k](../k/main.k). The seven existing L4 files are unchanged and byte-equal
to the reviewed pins:

- [design](../../../openspec/changes/sherlock-machine-circuit-layout/design.md)
- [interfaces](../../../openspec/changes/sherlock-machine-circuit-layout/interfaces.md)
- [crosswalk](../../../openspec/changes/sherlock-machine-circuit-layout/crosswalk.json)
- [extensions and obligations](../../../openspec/changes/sherlock-machine-circuit-layout/extensions-and-obligations.md)
- [proposal](../../../openspec/changes/sherlock-machine-circuit-layout/proposal.md)
- [tasks](../../../openspec/changes/sherlock-machine-circuit-layout/tasks.md)
- [owner repair provenance](../../../openspec/changes/sherlock-machine-circuit-layout/OWNER-REPAIR-PROVENANCE.md)

No OpenSpec relocation or new schema is introduced. Original absolute local
paths in frozen source/review metadata are historical locators; the
[publication binding](source-review-20261016/publication-binding.json) identifies
the actual public paths, source bodies, sizes and hashes. This data is not loaded
by a compiler or evaluator through a new registration. Existing fetch, codec,
observation and literal-fragment sources retain their separate scopes.

## Binding owner conditions, verbatim

The following text is copied exactly from the Root record's
`owner_binding_conditions_verbatim` field. It preserves Opus's judgments;
this README does not select an interpretation beyond that record.

```text
**ADOPT_FULL39_P1_SOURCE_WITH_CONDITIONS**

This is the designated-owner security and scope vote only. Conditions:

1. **Separate votes.** A fresh Muse nonsecurity vote on these exact bytes and a Root adoption record are both required. This vote and any old-12 vote do not substitute for either.
2. **File inventory.** Resolve the 32-versus-33 discrepancy and pin the manifest hash `a742…` in Root's record.
3. **F1.** Record that all five inherited common lines are charged-only. `OBL.P4.fuel` never applies to A080, A082 or A084.
4. **F2.** Record that the old 12 are now governed by this full-39 adoption, including the six appended common lines.
5. **F3.** A080's post-image is not asserted canonical; boundary projection remains F4/P5-open.
6. **F4.** For A185 and A195, MD2 stays UNSELECTED; the explicit CR0′ is a post-relation only.
7. **F5 and F6.** For A177, A181 and A214, copy-versus-extend, the bindings-root reference rule, and append order remain P1/P3 gaps. They are not resolved here.
8. **F7.** The PRESERVATION line neither authorizes nor forbids failure scratch. P3 must specify partial-work behavior for A204 and A153.
9. **No emitter acceptance.** No emitter may treat these rows as acceptance criteria until P2–P6, G4 and F4 are owner-specified and reviewed.
10. **Byte edits.** Any byte change to this proposal, including wording fixes for F1, F6 or F7, voids this vote for the changed bytes.

This adoption grants no runtime, resource, native, circuit, proof, ledger, stagenet or roadmap permission. Native authority remains 0.
```

## Binding normal conditions, verbatim

The following text is copied exactly from the Root record's
`normal_binding_conditions_verbatim` field. The complete original votes,
identities, reading limits and coverage qualifications are available in the
[review support](source-review-20261016/README.md).

```text
1. This is the NORMAL semantic vote only. A designated-owner (Opus) security vote on these exact bytes and a Root adoption record are still required; neither this vote nor any old-12 vote substitutes for them. Crosswalk/script/patch artifact approval is not sought or granted here.
2. Root must pin manifest `a742817c…159b` and proposal `6823b…122c2` and record the 32-listed + manifest = 33-artifact clarification.
3. F1: all five inherited common lines are charged-only (36 events); A080/A082/A084 carry only their row-local obligations; `OBL.P4.fuel` never applies to them.
4. F2: the old 12 are governed by this full-39 adoption including the six appended common lines.
5. F3: A080's post-image is not asserted canonical; boundary projection remains F4/P5-open.
6. F4: A185/A195 CR0′ is a post-relation only; MD2's ControlRow choice stays UNSELECTED.
7. F5/F6: A177/A181 copy-vs-extend and append placement, and A214's bindings-root reference rule and physical append order, remain P1/P3 gaps — unresolved here.
8. F7: PRESERVATION neither authorizes nor forbids failure scratch; A204/A153 partial-work behavior remains for P2/P3.
9. No emitter may treat these rows as acceptance criteria until P2–P6, G4, and F4 are owner-specified and reviewed; `resultRef`/`resultRoot`/`newDone` are symbolic relations, not expected data, free flags, or chosen addresses. No fresh addresses/default limits are chosen; physical gaps are not resolved.
10. Any byte change to the proposal, including wording fixes for F1/F6/F7, voids this vote for the changed bytes.


```

## Remaining scope

The F1–F7 conditions above retain the unresolved P1/P3 matching-root and
argument-slice choices, P2–P6 realization, G4 checks, F4 boundary projection and
MD2 ControlRow choice. Symbols such as `resultRef`, `resultRoot` and `newDone`
remain relations to specify and review, not supplied truth flags or addresses.
A080 may retain a Constant-space alias and is not asserted canonical.
No emitter acceptance follows from these rows.

Root's retained open-obligation list is reproduced verbatim below:

- F1: All five inherited common lines apply only to36 charged events; A080/A082/A084 only carry row-local OBL lists; OBL.P4.fuel never applies to free publications.
- F3: A080 may alias Constant space and is not a canonical boundary image; F4/P5 projection remains open.
- F4: A185/A195 CR0 prime is a post-relation only; MD2 rewrite-versus-retain UNSELECTED.
- F5: A177/A181 copy-versus-extend, placement and quadratic copy bound remain P3 open.
- F6: A214 bindings-root reference rule and physical append order remain P1/P3 open; Nil zero new cells and other successful chains do not select physical references.
- F7: PRESERVATION neither authorizes nor forbids failure scratch. A204 failed bind and A153 equality scratch remain P2/P3 open; A214 miss has no partial-insert source path.
- No emitter acceptance from these rows until P2-P6/G4/F4 owner-specified and reviewed. Symbols resultRef/resultRoot/newDone do not supply witnesses, chosen addresses or flags.
- StepInv, global dispatch, byte bridge, Constant typing, native binding, resources, B/general ADT, financial/proof/ledger/Midnight stagenet and all U0-U7/M1-M17/MC/SP/G exits retained.
- Any proposal byte change requires fresh applicable reviews. No source body changed by this record.
- C4 compile dependency gate unchanged: no dependent circuit changes before admitted A102 compilation; compile failure registration revert rather than unreviewed hotpatch.

The previous author packet's manifest is published only as an inventory receipt.
Its 32 listed local files plus the manifest total 33 artifacts; they are not all
published or approved as bodies. In particular, the four auxiliary bodies
`SEMANTIC-CROSSWALK.json`, `SOURCE.patch`, `author_proposal.py` and
`freeze_source_data.py` were identity-only to Muse and are not included here.
Historical author and scoped review records retain their original status.

Native authority remains zero and executable validation remains UNRUN. Source
publication preserves all U0–U7/M1–M17/MC/SP/G, financial, proof, B/general-ADT,
Midnight target/ledger and stagenet obligations. Maintainer source review controls
this work; it does not introduce public developer compile/prove/deploy permissions.

# Sherlock circuit atlas: website concept log

This folder is a running log of concept notes for a planned Moriarty website that presents the handwritten Sherlock ZKIR circuits as explorable three-dimensional boards. It holds ideas and the source they are grounded in. It contains no site code, assets, packages, renderer output or deployment, and writing it does not start the website. Building the site would be a separate change with its own review.

## Brief

The user asked for a log of concept notes for a beautiful website, kept by Opus 5.5 while the circuits are being built. The site would use Moriarty branding, a high-tech circuit and PCB look in the manner of Tron, and Three.js models of the handwritten circuits. The brainstorm happens asynchronously, once per iteration. This brief was recorded on 2026-10-16 UTC.

The Tron reference is taken as an aesthetic: luminous routes on dark layered boards. It is not a franchise identity, so the notes use no franchise logos, typefaces, characters or recognizable set designs.

## How iterations arrive

Each note is triggered by an existing implementation event:

1. The implementation lane (Astra high) freezes a handwritten source candidate with SHA256 pins.
2. Root forwards those bytes.
3. Opus 5.5 returns one note.
4. Root writes the note verbatim and records provider provenance.

Nothing new is scheduled, polled or orchestrated, and implementation never waits on this log. Security source reviews of the same bytes are separate documents and are not part of any note. This holds even when one session writes both, as in iteration 003.

Each frozen source iteration gets one `ITERATION-NNN-*.md` file. Earlier notes are never rewritten; later notes supersede them by reference. Each note carries its date, author, source pins, observations from the source, alternatives, a recommended concept and open notes.

## Provenance

- [`AUTHORSHIP.json`](AUTHORSHIP.json) records, for each iteration:
  - requested and returned model, and effort;
  - original reply and note hashes;
  - source pins;
  - source base scope.
  These are Root-recorded provider receipts.
- The original public replies are kept for verbatim comparison:
  - [`provenance/iteration-000-opus-public-reply.json.gz`](provenance/iteration-000-opus-public-reply.json.gz)
  - [`provenance/iteration-001-opus-public-reply.json.gz`](provenance/iteration-001-opus-public-reply.json.gz)
  - [`provenance/iteration-002-opus-public-reply.json.gz`](provenance/iteration-002-opus-public-reply.json.gz)
  - [`provenance/iteration-003-opus-public-reply.json.gz`](provenance/iteration-003-opus-public-reply.json.gz). Root recorded the original public reply when materializing iteration 003.
- Source bases are recorded per iteration:

| Iteration | Source base |
|---|---|
| 000 | Not recorded by the original receipt; the exact file digests are authoritative |
| 001 | `1fb794d1598e19fe34b9735f90996fc316b743fa` |
| 002 | `1fb794d1598e19fe34b9735f90996fc316b743fa` |
| 003 | `1fb794d1598e19fe34b9735f90996fc316b743fa`; the G3 candidate files are uncommitted |

- The concept checkout base `f7ff06ed05bcef5fc3540bdde70ddbf6402f5547` is where this log is written. It is not a source base.
- Root's corrections after review changed provenance metadata only. Notes 000 and 001 were not rewritten.
- Publication statements in each note describe its recorded freeze time. The source snapshots and later publication records retain that history; publication never supplies executable validation. Root added this provenance clarification and the iteration 002 reply link during integration; the brainstorm notes remain verbatim.
- For iteration 003, Root appends the exact source-capture metadata to `AUTHORSHIP.json` after the reply. The note's pins are the values from the forwarded author manifest.
- Notes 000 to 002 were published with G2 in [PR #91](https://github.com/CharlesHoskinson/Moriarty/pull/91), merge `5178d576e3f1931560e626738dd8ddfe6de243f7`. At its materialization freeze, iteration 003 was local to the concept working tree. This source publication package carries iteration 003 with G3; publication supplies no executable validation. Root updated this publication provenance after the author reply; the iteration notes remain unchanged.

## Source, idea and status

| Kind | What it states | How notes mark it |
|---|---|---|
| Source | What pinned bytes contain | Path, SHA256, symbol anchor |
| Idea | A proposed visual or interaction | Written as a proposal |
| Status | What ran and what it returned | Copied from the source record; unrun work says UNRUN |

A site built from these notes should keep the same channels visually distinct. Proposed light is decoration or selection. Only a future measured result may be drawn as activity.

## Brand evolution

The current design system (`site/src/styles.css`) is built like a ledger and says `Nothing glows`. Emissive circuitry is a deliberate, user-directed departure from that line, and this log records the departure rather than adopting it silently.

The following carry over unchanged:

- The Moriarty wordmark, in the existing sans at weight 600 with tight tracking. It stays flat and is never extruded or lit.
- Tabular monospace figures and identifiers that never reflow or abbreviate.
- The 4px spine and the measured grid.
- The ink grounds and the copper accent (`#9a3412` light, `#e8724a` dark).

Any new hue or emission is a proposal pending design review and contrast measurement. That includes the cyan proposed in iteration 000 and the copper obligation emission proposed in iteration 001. Iteration 003 adds no new hue. `site/VOICE.md` continues to govern page copy.

## Index

| Iteration | Source slice and snapshot | Note | Source status and review scope |
|---|---|---|---|
| 000 | Scalar literal step (`sherlock_step`, `sherlock-scalar-literal-step/1`) | [Literal step](ITERATION-000-LITERAL-STEP.md) | Candidate frozen, not on `main`; execution UNRUN. A different module from 001/002. Its field, test and instruction counts are author observations that the latest G audit did not review. |
| 001 | Row fetch, r1 snapshot | [Bounded row fetch](ITERATION-001-BOUNDED-ROW-FETCH.md) | Unregistered; four tests and execution UNRUN. Grounding reviewed by Muse. Its unprefixed identifiers belong to r1 only. |
| 002 | Row fetch, r2 snapshot (`%` identifiers, rejection tests, locked wiring) | [Identifiers and wiring](ITERATION-002-IDENTIFIERS-AND-WIRING.md) | Unregistered; six tests SOURCE-ONLY UNRUN; actual output UNRUN. |
| 003 | Separate enabled fetch, G3 snapshot (`emit_enabled_fetch_block`) | [Enabled fetch](ITERATION-003-ENABLED-FETCH.md) | Unregistered; five new and six inherited tests UNRUN; actual output UNRUN, including the disabled 2^128 raw-index case. A separate fresh security source review exists outside this log. |

| 004 | Full-image A102 Literal and 16-byte LE borrow; original three-file manifest `662bc7748f2a98e2186372970951c08ebed0328b04c574f6f7e4f5eab42e7f85` | [Full-image literal and borrow](ITERATION-004-FULL-IMAGE-LITERAL-AND-BORROW.md) | Historical unregistered/uncompiled freeze; five tests UNRUN. Initial Gemini fallback concepts, Opus source-accuracy revision. Later source approvals do not approve this concept note or a website. |

## Latest iteration

Iteration 004 records the full-image A102 Literal fragment and its sixteen-byte fuel borrow. Its corrected note is the exact decoded Opus reply, with initial Gemini concepts credited. The [original source snapshot](provenance/iteration-004-source-snapshot.json.gz) retains the original unregistered three-file freeze. The current registered ten-file source candidate and its separate source/data reviews are documented in [the A102 source package](../../models/sherlock/literal-step/source-review-20261016/README.md); compilation and all sixteen machine tests remain UNRUN. No circuit, site or runtime result is implied.

## Previous iteration 003

Iteration 003 responds to the separate enabled fetch, a sibling of the unchanged G2 constructor. It proposes the following:

- **Enable bus.** The single enable operand is one inner-layer net with vias only at its real instruction terminals. The enable tags on selected outputs are silkscreen flags, because they are host data rather than instructions.
- **Select glyphs.** Every CondSelect is drawn as a trapezoid. The raw index taps only the effective-index true arm, the prefix Assert sits behind a guard select, and the sum terminator is a via on the enable bus.
- **Disabled-zero.** It is shown as construction data: the zero seed and a DOM card quoting the definition. It is never a table row, padding or an animated lane. The board stays identical across fixture cases.
- **Caller obligations.** Dispatch provenance of enable and consumer gating are drawn as dashed, unlit off-board sockets, because no dispatch or consumer source exists.

The 41-instruction and 19-temporary hand counts for the 3×2 test shape size the scene only. Dispatch backplanes and recursion stay conceptual sketches.

The next note is triggered by one of these events:

- Astra high freezes dispatch, consumer or adapter source.
- Root forwards a recorded, admitted run.

A recorded run would be the first input to the measured light channel.

## Boundaries

- Notes use only public, forwarded source. No private Fenrir or Clean source and no secret witness data appear or are requested.
- Notes do not state constraint success, circuit size parameters, fees, speeds, proving benchmarks or recursion completion unless a forwarded result records them.
- No proposed page evaluates circuits in the browser or animates proving.
- Disabled-zero values are construction data, never fetched rows, padding or animated execution.
- Visuals for recursive calls and multi-branch machines stay labelled conceptual until actual source exists.
- No note claims that a source candidate is machine-ready or that Sherlock is complete.

## Iteration 004 captured provenance

Initial concepts came from Gemini 4 Argon high via AGY under the explicit outage fallback. The request used plan mode; the returned permission mode was always-proceed. No plan-mode enforcement or mechanical tool-absence claim follows. Source-accuracy revisions came from Claude Opus 5.5 high after recovery. Iterations 000–003 and their original author entries remain unchanged.

The complete public adapter JSON and exact `RESULT.text` are retained separately:

- [Gemini adapter](provenance/iteration-004-gemini-public-reply.json.gz) and [exact text](provenance/iteration-004-gemini-public-reply.md.gz).
- [Opus adapter](provenance/iteration-004-opus-public-reply.json.gz) and [exact text](provenance/iteration-004-opus-public-reply.md.gz).

The Opus adapter SHA256 is `31aedb8f0b59fae1287d2090d33047375db61e2d5a36c06dd368e966a5ee1dbf`; the original Gemini adapter SHA256 is `975cb29561fc0b0bc94bd673a2ef465bbc726fc3aa2746495569229b9d98e1a2`. The note retains provider placeholders and historical status verbatim. `AUTHORSHIP.json` appends actual captured hashes, relative receipt paths and the returned canonical Opus identity without rewriting those strings. Astra materialized these bytes under Root's delegation; publication remains with Root. No private provider/debug transcript is included.

## Design companion 005

[Design 005: arena slices and byte lanes](DESIGN-005-ARENA-SLICES-AND-BYTE-LANES.md)
is an Opus-authored companion concept note. It does not add a frozen circuit
iteration. The latest circuit iteration remains 004. Read the adjacent
[status and provenance clarification](DESIGN-005-STATUS-AND-PROVENANCE.md) for
the historical candidate labels, later source adoption and exact captures.
Earlier notes remain unchanged. This folder still contains no website code,
assets, rendered scene or deployment.


## Design companion 006

[Design 006: source boards and host replay](DESIGN-006-SOURCE-BOARDS-AND-HOST-REPLAY.md) continues the concept log for the finished Sherlock v1 website. It develops the circuit assembly history: source-pinned additions and changes become cumulative Three.js board layers, with execution inspection and source clickthrough. Its visitor copy assumes the finished release. The [materializer record](DESIGN-006-STATUS-AND-PROVENANCE.md) and [factual source manifest](provenance/design-006-source-manifest.json) preserve actual author identity, source pins and current evidence scope. This companion does not add a circuit iteration; the latest actual circuit entry remains 004. Earlier entries remain unchanged.


## Latest circuit iteration 005

[Iteration 005: Scalar.tag and two-space fetch](ITERATION-005-SCALAR-TAG-AND-TWO-SPACE-FETCH.md) records the new handwritten positive RetRight/Add/First/UInt leaf and the board changes it adds to the assembly history. It stages operand A in sixteen byte lanes, advances the helper phase, preserves the helper complement and relates the work counters byte by byte. Its [materializer record](ITERATION-005-STATUS-AND-PROVENANCE.md) and [source manifest](provenance/iteration-005-source-manifest.json) bind the exact freeze and current audit scope. This is the latest actual circuit source iteration. Earlier latest-iteration labels retain their historical freeze context; Designs 005/006 remain visual companions. The website's visitor experience describes finished Sherlock v1 and launches when the complete goal is accepted.


## Source and evidence update 006

[Update 006: statement boundary and settlement route](UPDATE-006-STATEMENT-BOUNDARY-AND-SETTLEMENT-ROUTE.md) is the preserved Opus-authored source/evidence note that followed circuit iteration 005. It adds no circuit iteration and is distinct from Design 006 above. It proposes complete logical program/pre/post pin banks and an off-board statement, native proof, verification and settlement route for the finished Moriarty Tron/PCB and Three.js experience. Read its [editorial status and provenance](UPDATE-006-STATUS-AND-PROVENANCE.md) alongside it: the historical author freeze is preserved, two wording/binding passages are explicitly qualified, and later public bridge and actual codec observations have their own limited scope. Its [factual manifest](provenance/update-006-source-manifest.json) records actual authorship and exact source pins. The latest actual circuit entry remains 005. The site launches when the complete Sherlock v1 goal is accepted; this append publishes a design-log entry only.

## Editorial update007 — actual host observations and revision03

[Host observation and completion design](UPDATE-007-HOST-OBSERVATION-AND-COMPLETION-DESIGN.md) links the actual stream and composed carrier results to the existing assembly-history concept. It is a Root/Sol factual editorial update, not a new circuit-source iteration or Opus creative note. Latest actual circuit-source entry remains005. The finished website launches after full Sherlock v1 acceptance.


## Editorial update008 — nominal declaration assembly

[Nominal declaration assembly](UPDATE-008-NOMINAL-DECLARATION-ASSEMBLY.md) records the adopted compiler table substrate and its independent complete-table normal observation. Root/Sol factual editorial update; circuit iteration005 remains the latest circuit-source entry.


## Editorial update009 — selected word host lowering

[Selected word host lowering](UPDATE-009-SELECTED-WORD-HOST-LOWERING.md) records Root's whole030 host arithmetic adoption and the exact normal/error observations. Root/Sol factual editorial update only; circuit-source iteration005 remains latest. Host lowering is not circuit/native acceptance, and no circuit layout/cost winner is selected.


## Editorial update010 — actual trace and preparation

[Actual trace and preparation assembly](UPDATE-010-ACTUAL-TRACE-AND-PREPARATION.md) connects the observed typed-source preparation and finite F03 model trace to a future source-clickthrough view. Root/Sol factual concept only; no website code or circuit-source iteration is added. Latest actual circuit entry remains005.


## Editorial update 011 — retained native audit provenance

[Retained native audit provenance](UPDATE-011-NATIVE06-AUDIT-PROVENANCE.md) links the two scoped result acceptances and actual preservation/cleanup. Root/Sol factual editorial entry only; circuit-source iteration 005 remains latest. SDK target execution, proofs and stagenet remain open.


- [Editorial update012 — canonical TypeId carriers](UPDATE-012-CANONICAL-TYPE-CARRIERS.md): three compiles and ten host tests passed; six real fields, nominal storage only, no new circuit iteration.


- [Editorial update013 — financial await-prefix host evidence](UPDATE-013-FINANCIAL-AWAIT-PREFIX.md): consumed returned-prefix publication and two fixed positive consumers; no new circuit source, iteration005 remains latest.

- [Editorial update 014 — complete observations and portable prefixes](UPDATE-014-COMPLETE-OBSERVATIONS-AND-PORTABLE-PREFIXES.md): full-field comparison and distinct portability stages; concept only, circuit iteration 005 remains latest.


## Circuit source iterations006/007 — October18 dated staging

[006: checked Add and staged outcomes](ITERATION-006-CHECKED-ADD-AND-STAGED-OUTCOMES.md) and its [historical provenance](ITERATION-006-STATUS-AND-PROVENANCE.md) retain the exact source/concept freeze. Root subsequently installed the exact three006 Add files at short head334e1d08 under draftPR263; SDK compilation, eight tests and constraint/proof execution remain unrun. [007: staged-result completion](ITERATION-007-STAGED-RESULT-COMPLETION.md) and its [provenance](ITERATION-007-STATUS-AND-PROVENANCE.md) describe returned R-only source, not an installed or executed completion circuit. These factual notes are Root/Sol drafts grounded in actual Gemini4ArgonHIGH source, not Opus creative replies. Earlier005/latest labels preserve their historical cuts. The finished site goes live only when the whole Sherlock goal is finished and accepted; no website is built or deployed here. [Dated source/evidence bindings](../../docs/results/sherlock-four-lane-progress-20261018/addenda/20261018-literal035-clean-circuit/EVIDENCE.json).


## Design companion 007 — subtraction and typed handoff

[Design 007: subtraction and typed handoff](DESIGN-007-SUBTRACTION-AND-TYPED-HANDOFF.md) is the preserved Opus5.5 high creative companion for the cumulative Moriarty Tron/PCB and Three.js assembly history. Its sixteen-byte borrow ladder connects staged outcomes, completion and a typed handoff ribbon. [Status and provenance](DESIGN-007-STATUS-AND-PROVENANCE.md) distinguish the raw note date from the actual October19 return, bind the exact source cut and retain unregistered/SDK-unrun scope. [The source manifest](provenance/design-007-source-manifest.json) records full pins and the raw public reply. Finished-release visitor copy stays confident; the site goes live when the full Sherlock goal is finished. This companion adds no circuit iteration or website code. Earlier Root/Sol factual iterations006/007 and the independent website thread keep their separate authorship and history.


## Design companion 008 — Sub5 composer and typed backplane

[Design 008: Sub5 composer and typed backplane](DESIGN-008-SUB5-COMPOSER-AND-TYPED-BACKPLANE.md) is the preserved **Muse** creative contribution under the observed Opus outage fallback. It extends Design 007 into the source-grounded five-stage Sub composer, typed handoff and independently source-audited parent backplane. [Status and provenance](DESIGN-008-STATUS-AND-PROVENANCE.md) bind the actual contributor identity, exact source cut and draft PR276 publication; all 46 existing SDK tests and the separate three-branch overlay remain UNRUN. The test review does not establish exact guard isolation, and old-W2 P9 compilation covers none of the new thirteen hashes. [The factual manifest](provenance/design-008-source-manifest.json) and [public reply projection](provenance/design-008-muse-public-reply.json.gz) preserve the bindings. Finished-release visitor copy remains future-facing: the site launches only after the whole Sherlock goal is finished and accepted. This companion adds no circuit iteration, website implementation, source/security vote or execution/proof/ledger acceptance. Earlier notes and their authorship remain unchanged.


## Design companion 009 — full product and typed completion

[Design 009: full product and typed completion](DESIGN-009-FULL-PRODUCT-AND-TYPED-COMPLETION.md) preserves the exact Muse creative reply: an 8×8 limb-product field, separate low/high rails and typed completion dock. Its [status and provenance](DESIGN-009-STATUS-AND-PROVENANCE.md) retain the actual configured/contributor identity, historical source gaps and later dated fixture/review facts. The [source manifest](provenance/design-009-source-manifest.json) binds the exact note and public reply. This companion adds no circuit iteration or website implementation. The finished site launches only after the whole Sherlock goal is finished and accepted.

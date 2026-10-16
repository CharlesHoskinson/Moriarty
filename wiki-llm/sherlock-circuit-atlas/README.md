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

## Latest iteration

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

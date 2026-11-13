# Slop log: narration scripts

Scope: the viewer text only, which is the `Narration:` paragraphs, the `Labels:` text and the scene titles. Every edit kept the beat headers, the durations, the labels, the inline code spans and the numbers. A script compared these between `astra-N.orig.md` and `astra-N.md`, and they are identical (the only number diff is the comma that follows "10" and "63"). No rewritten narration is longer than its beat: word count / 2.5, rounded up, is never more than the duration in the header. Fourteen beats are now shorter than their durations; they are listed under each file for the director, and the durations stay as written. The only change outside the narration is in builder text: each em or en dash in a `Numbers used:` line or a director note became a colon or "to". A diff of the non-narration lines shows nothing else. All labels have four words or fewer (an identifier counts as one word), and no beat has more than six labels.

On "prevent": the writers' brief requires each beat to say what a check rules out, so that content stays in every beat. The edit removes the stock closer sentence ("This prevents ...", "These checks prevent ...", "Without this check, a prover can ...") and joins the rule-out to its check. The joined form is mostly ", so ... cannot ...". It still recurs, and its counts per file are given below.

Facts were checked against `BRIEF.md` sections 2 and 3. Where a beat goes past BRIEF, the check used `/home/hoskinson/Projects/Moriarty/research/sherlock-final-design-20261111/ARCHITECTURE.md` (main checkout, read only; the worktree copy of that folder is empty) and `openspec/changes/sherlock-vm-consensus-20261107/{design,requirements}.md`.

## astra-1.md (steps 1 to 3)

Kind: explanation (my-voice section 1, reports/studies/explanations row); labels follow the promo/motion row.

rhythm_check, before:

```
Sentences: 238  mean 12.4  sd 4.1  median 12.0  max 23
Tic patterns: absolutes×5
Prose smells: announced structure×1
Numbers: 32  per paragraph mean 0.64  max 9  paragraphs over two 4 of 50
Flags:
  - Low sentence-length variation: monotone rhythm.
```

rhythm_check, after:

```
Sentences: 217  mean 13.5  sd 5.1  median 13  max 29
Tic patterns: absolutes×7
Prose smells: announced structure×1
Numbers: 32  per paragraph mean 0.64  max 9  paragraphs over two 4 of 50
Flags:
  - Low sentence-length variation: monotone rhythm.
```

Notes on the counts:
- absolutes: every hit is the mechanism verb "proves" or "never" in a rule of the machine. These are correct in a system description, so no change.
- announced structure: "has six parts" is a count from the system (BRIEF step 2). It stays.
- Numbers, max 9 in one paragraph: beats 3.5 (chip ids 1 to 4) and 3.15 (depth 63, 2^66, 3 x 2^64 < 2^66). Each is a true enumeration or one inequality, and labels carry the same values. They stay.
- monotone: the spread rose from 0.33 to 0.38 of the mean. Each paragraph has a fixed word count (timing) and carries one mechanism, which limits how much the sentence length can vary. It remains a soft flag.

Top problems:
1. A repeated closer that names what a check rules out (humanizer §2, Z-37): "prevent" ×26, with 11 standalone closers of the form "This prevents" or "These checks prevent". After: "prevent" ×0 and standalone closers ×0. The rule-out is now joined to its check. The forms that replace the closer: ", so ... cannot/no/never" ×11 (3 before), "stops ... from" ×1. Three of these joins were recast again (2.6, 3.2, 3.12) so that the "so" form does not end every beat.
2. -ing riders (humanizer §15): ", preventing ..." or ", which prevents ..." ×11. After: 0.
3. Nominalizations and light verbs (P-23, P-24): "Authentication connects that row", "preventing substitution of", "gives no authority to certify".
4. Clipped contrast (humanizer §1): "only changes the tape cut and does not produce this outcome". The contrast "not the size of a circuit" stays: BRIEF states it and it corrects a belief the viewer holds.
5. Monotone rhythm (Engaging): short runs merged ("Each slot's final ... The next slot's initial ...").

Before and after:
1. 1.2: "Authentication connects that row to this image, preventing substitution of another operation or constant." → "Each row is authenticated against this image, so nobody can substitute another operation or constant."
2. 2.1: "Execution produces the detailed evidence that the leaf checks. The simulator's record alone gives no authority to certify the result." → "The run produces the detailed evidence that the leaf checks. The simulator's own record certifies nothing."
3. 3.11: "Together these checks prevent skipped addresses, repeated allocation and substituted record contents." → "No address is skipped or allocated twice, and no record's contents are swapped."
4. 3.14: "Matching these paths prevents overwriting an occupied segment or inserting the new records at another address." → "The matching paths stop the flush from overwriting an occupied segment or placing records elsewhere."
5. 2.3: "Each step fires exactly one rule from the inventory of 120 rules. Enabled means that the rule's conditions hold ..." → "Each step fires exactly one rule out of 120. A rule is enabled when its conditions hold ..."

voice-verify (atoms of the viewer text; A = ARCHITECTURE.md line):

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| 1 | `prog_d` Poseidon digest of program image; one key set (leaf, node, decider) | VERIFIED | BRIEF §3.1 |
| 2 | `input_d` under `sherlock-value/1`; refs contribute target digest, sharing does not change it | VERIFIED | BRIEF §3.1; design.md table "a reference ... the vd of the value it refers to", "Shared structure: the digest equals that of the unfolded tree" |
| 3 | Six state parts; 120 rules, exactly one fires; witness frame = rule id, before, after | VERIFIED | BRIEF §3.2 |
| 4 | 64-bit counter, up to 2^64 - 1 steps, 2^64 is range not circuit size | VERIFIED | BRIEF §3.2 |
| 5 | Terminal publication counts as a step | VERIFIED | A:2331 "`n_steps` as the count of enabled owner events, publication included" |
| 6 | Rejected code 10 overflow; Exhausted with kind Fuel | VERIFIED | BRIEF §3.2; design.md:193 lists Fuel as an exhaust kind |
| 7 | Capacity cut produces no Rejected/Exhausted; closing a leaf adds no charge | VERIFIED | A:605 |
| 8 | `S = 16`; one circuit, one key; modules and their roles | VERIFIED | BRIEF §3.3 |
| 9 | Chip ids 1 ALU, 2 MEM, 3 HASH, 4 ROM; two request lanes (script: "channels"); CORE sole requester | VERIFIED | BRIEF §3.3. Term drift: BRIEF says "request lanes" and the script says "request channels" (narration, on-screen text, `Numbers used`). Not changed, because labels and on-screen text are kept as written. |
| 10 | ALU also checks subtraction, multiplication, equality | VERIFIED | A:331 `BinaryOp` Add, Sub, Mul, Eq, Lt |
| 11 | 12-bit limbs; RANGE/DEC table tags separate | VERIFIED | BRIEF §3.3; A:107 |
| 12 | Domain tags separate records, tree levels, stacks, bodies; no chain of short bus hashes | VERIFIED | A:477; A:3036 "No chain of 2-input bus hashes SHALL be used" |
| 13 | At most 3 cells per step; 8-cell tail; one flush per leaf; depth 63; 2^66; 3 x 2^64 < 2^66 | VERIFIED | BRIEF §2 |
| 14 | Cache line segment lies before the imported tail | VERIFIED | A:87 heap lines `addr < 8 q_pre`, window `addr >= 8 q_pre` |
| 15 | TASKS rewrite precedes push as one event | VERIFIED | A:582, A:717 |
| 16 | Stack link hashes the depth | VERIFIED | A:721 |
| 17 | Header fields and body contents | VERIFIED | BRIEF §2 |
| 18 | Equal tokens bind equal states via Poseidon collision resistance | VERIFIED | A:484 (a premise) |
| 19 | Scratch is consumed only within its slot | VERIFIED | A:606-607 |
| 20 | INIT only at `h_pre = 0`, proves `Init(prog_d, input_d)`, empty genesis memory | VERIFIED | BRIEF §2, §3.3 |
| 21 | Seven lanes; partition independence | VERIFIED | BRIEF §3.3 |
| 22 | Flush 8,821 rows, old read 4,761 rows, leaf fits 2^16 | VERIFIED | BRIEF §3.3 |
| 23 | `input_d` is the value digest only | CONTESTED (source disagreement) | BRIEF §3.1 says value digest. The writer's director note says that ARCHITECTURE's INIT kit and design.md make `input_d` an envelope that holds the value digest, the fuel components and a stack limit. The script follows BRIEF. The lead must decide; no change made. |

Humanizer pass: re-read the revised narration against §1, §2, §6, §8 and §19. It changed four lines. In 3.33 a semicolon became a period. In 3.10 the fragment "Chained BUS hashes miss" became a full clause. In 3.7 the clipped tail ", with no silent wrap" became "and the ordinary result passes unwrapped". In 2.6 and 3.2 the "so no ..." closer got a different shape. No dashes and no bold remain.

Beats now shorter than their duration: none.

## astra-2.md (steps 4 and 5)

Kind: explanation; labels follow the promo/motion row.

rhythm_check, before:

```
Sentences: 133  mean 12.5  sd 5.1  median 11  max 24
Tic patterns: absolutes×2
Numbers: 10  per paragraph mean 0.32  max 3  paragraphs over two 1 of 31
No flags.
```

rhythm_check, after:

```
Sentences: 126  mean 12.9  sd 5.2  median 12.0  max 24
Tic patterns: absolutes×3
Numbers: 10  per paragraph mean 0.32  max 3  paragraphs over two 1 of 31
No flags.
```

absolutes: "proves" and "never licenses" are rules of the system, so no change. The paragraph with 3 numbers is 4.10 (65 s, 20 threads, 17 GB), which BRIEF states as one measurement.

Top problems:
1. A repeated closer in the form "Without this check, a prover can ..." ×6 (humanizer §2). After: 0. A first fix put "stops a prover from" in five beats (4.4, 4.5, 4.6, 5.2, 5.5). That is the same closer in new words, so each of the five got a different shape. After: "stops ... from" ×0, ", so ... cannot" ×3.
2. "This prevents", "These checks prevent", "Pinning prevents" ×6 (humanizer §2, §15). After: 0.
3. A contrast split across two sentences (humanizer §1): "The node does not perform that pairing. It folds ...".
4. A false contrast (humanizer §1): "Unlike MERGE, JOIN requires no execution boundary ... JOIN carries no claim counter."
5. A run of one-line fragments (humanizer §2, Z-46) in 4.6: "MERGE also requires ... . The left segment's ending height equals ... . Without height equality ... . The claimed coverage then ...".

Before and after:
1. 4.5: "Without this check, a prover can replace registers, stacks or memory between valid segments and present the disconnected executions as a continuous run." → "Registers, stacks or memory replaced between valid segments fail this check, so disconnected executions cannot pass as one run."
2. 4.8: "The node does not perform that pairing. It folds the new check ..." → "The node defers that pairing and folds the new check ..."
3. 4.4: "Both proofs therefore describe the same program. Without this equality, a prover can splice execution from different programs into a segment that claims a single program identity." → "..., so both proofs describe the same program. Execution spliced from different programs cannot form a segment that claims one program identity."
4. 5.1: "Pinning prevents a prover from replacing the node with a circuit that accepts an invalid run." → "With pinned keys, no prover can swap in a node circuit that accepts an invalid run."
5. 5.13: "Keeping them out of the public statement makes the claim independent of those cuts ..." → "With those fields outside the public statement, the claim is independent of those cuts ..."

voice-verify:

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| 1 | One self-recursive node at `k = 20` | VERIFIED | BRIEF §3.4 |
| 2 | MERGE equalities and output tuple | VERIFIED | BRIEF §3.4 |
| 3 | Heights below 2^64; `steps` positive | VERIFIED | BRIEF §3.4; A:599 `1 <= steps <= S`, A:1776 `steps >= 1` |
| 4 | One accumulator, every deferred pairing folds into it, same size at any height; linear work | VERIFIED | BRIEF §3.4 |
| 5 | About 65 s, 20 threads, 17 GB; 3 provers give 1.73x | VERIFIED | BRIEF §3.4 |
| 6 | MERGE-WORK: prior proofs plus new steps in one node | VERIFIED in ARCHITECTURE, NOT IN BRIEF | A:1753. BRIEF §2 GP-4 lists only MERGE and JOIN, and CREATIVE-BRIEF section 6 says that only BRIEF §2 choices appear. A:1786 says that MERGE-WORK is not measured yet. The lead must decide whether beat 4.12 stays. |
| 7 | CONTINUATION checkpoint and resume (beats 4.15, 4.16) | VERIFIED in ARCHITECTURE, NOT IN BRIEF | A:34, A:77. BRIEF has no checkpoint mechanism. The lead must decide. |
| 8 | JOIN `set_root = H(SHK_SET_JOIN, l, r)`; no claim counter | VERIFIED | BRIEF §2 |
| 9 | `shape_d` binds the proof graph: relation types, enabled slots, key selections, child shapes, work descriptors | VERIFIED | A:2105 `shape_d = H(SHK_SHAPE, T_NODE, mode, slot_bits, key_sel_0, key_sel_1, shape_0, shape_1, work_kind, work_steps, out_a, out_b)` |
| 9b | Key selector picks proof type and key; unused slots and the genesis carry are neutral | VERIFIED | A:1727 `key_sel_j` one-hot over {LEAF, NODE, FOREIGN}; A:1769 neutral accumulator, genesis carry neutral |
| 10 | SEAL: `h_lo == 0`, header opens to five fields, Running refused | VERIFIED | BRIEF §3.5 |
| 11 | Ordinary SEAL requires empty `deps_d` | VERIFIED | A:1754 `deps_d == EMPTY_DEPS` |
| 12 | `outcome_d` hashes outcome domain, version, kind, payload | VERIFIED | requirements.md SRC-022 `H(O, 1, tag, ...)`; BRIEF gives kind and payload only |
| 13 | `claim_d` formula, excludes `n_steps` | VERIFIED | BRIEF §3.5; A:1843 |
| 14 | `n_steps` positive, 8-byte range check, below 2^64 | VERIFIED | BRIEF §3.5; A:1843 `[1, 2^64)` |
| 15 | Exactly four public values; decider fits 2^20 rows | VERIFIED | BRIEF §3.5 |

Humanizer pass: it removed one "would" that the fix had put into 4.6. It also removed filler that had been added only to reach a word count: "Instead," (4.8), "merged output" (4.12), "ever ... that comes" (5.2), "Together the checks keep" (5.11) and "and then ... can result" (5.12).

Beats now shorter than their duration (words / 2.5, rounded up, against the header): 4.4 16 s of 18, 4.5 20 of 21, 4.6 19 of 20, 4.8 23 of 24, 4.12 27 of 28, 5.2 22 of 25, 5.5 19 of 21, 5.11 24 of 26, 5.12 23 of 24. The director decides whether to shorten these durations.

## astra-3.md (steps 6 to 8)

Kind: explanation; labels follow the promo/motion row.

rhythm_check, before:

```
Sentences: 167  mean 12.1  sd 4.5  median 12  max 24
Tic patterns: absolutes×6
Numbers: 4  per paragraph mean 0.11  max 2  paragraphs over two 0 of 35
Flags:
  - Low sentence-length variation: monotone rhythm.
```

rhythm_check, after:

```
Sentences: 163  mean 12.3  sd 4.8  median 12  max 24
Tic patterns: absolutes×8
Numbers: 4  per paragraph mean 0.11  max 2  paragraphs over two 0 of 35
Flags:
  - Low sentence-length variation: monotone rhythm.
```

absolutes: "proves", "never enter", "never acquire" and "never combines" describe rules of the system, so no change. Monotone: the spread rose from 0.37 to 0.39 of the mean, under the same constraint as astra-1.

Top problems:
1. A repeated closer in the form "This prevents", "This rejects", "This lookup", "This equality" or "These checks prevent" ×11, with "prevent" ×9 (humanizer §2, §15). After: standalone closers ×0, "prevent" ×0. The rule-out is now joined to its check: ", so ... cannot" ×4 (2 before), "stops ... from" ×1 (7.3).
2. Not-X-but-Y and "alone" contrasts (humanizer §1, P-42): "rather than executing the witness tape", "without relying on another node's assurance", "Matching a key digest alone is insufficient:", "but another path to it grants no additional settlement". One "alone" contrast stays in 7.17 ("A valid encoding alone cannot establish ...") because it corrects a belief the viewer holds: that a proof which parses is a valid proof.
3. Runs of very short sentences (Z-46, monotone): 6.2 "Verification against the altered transcript fails. The transaction is refused."
4. Negation load (P-42): "grants no permission to accept its claim repeatedly", "The program identity does not select a new contract or verifier key."
5. New before given (P-28): 6.3 began with the negative and ended with the shared contract.

Before and after:
1. 6.2: "A sender replaces `outcome_d` while keeping the proof. Verification against the altered transcript fails. The transaction is refused. ... This binding prevents a valid proof from authorizing a different claim or step count." → "When a sender replaces `outcome_d` and keeps the proof, verification against the altered transcript fails and the transaction is refused. ... A valid proof therefore authorizes only its own claim and step count."
2. 6.3: "The program identity does not select a new contract or verifier key. One contract and one key set serve every program and every input." → "One contract and one key set serve every program and every input, so the program identity selects no contract and no verifier key."
3. 7.13: "Matching a key digest alone is insufficient: substituted arithmetic or commitment rules cannot pass this check." → "Substituted arithmetic or commitment rules fail this check, even when they arrive under a matching key digest."
4. 8.2: "It checks the final decider proof rather than executing the witness tape or checking every leaf separately." → "It checks the final decider proof. It does not execute the witness tape or check each leaf." The negative stays because it corrects a belief the viewer holds: that a node re-runs the program.
5. 7.9: "This equality rejects missing, added, reordered or removed repetitions." → "Missing, added or reordered entries and removed repetitions fail this equality."

voice-verify:

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| 1 | Transaction calls `SherlockRun.acceptRun`; transcript binds four values; DUST; about 6.4 KB | VERIFIED | BRIEF §3.6 |
| 2 | "about 6.4 KB, whatever the length of the run"; node check cost does not depend on run length | VERIFIED (inference) | BRIEF §3.6 size and §3.8 one key; the decider proof is fixed size (BRIEF §3.4, §3.5) |
| 3 | One contract, `prog_d` call data, one key set | VERIFIED | BRIEF §2 |
| 4 | `acceptRun` verifies, recomputes `claim_d`, refuses repeats including another partition, stores `outcome_d` and `n_steps`, no state digest | VERIFIED | BRIEF §3.7 |
| 5 | No claim counter | VERIFIED | BRIEF §2; A:1712 |
| 6 | Consumption at seal via SEAL-WITH-DEPS; `deps_d` ordered with repetition; the discharge chain equals the opened `deps_d` | VERIFIED | BRIEF §2 GP-2; A:1974; A:1984 `DL.chain_d ==` the opened `deps_d` |
| 7 | Input link: the producer's Halted value digest equals the consumer's input value digest; the link needs SEAL-WITH-DEPS even with an empty list | VERIFIED in ARCHITECTURE | A:1996 "the linked input value digest equals the producer's Halted value digest"; A:1755 (the input link exists only in the SEAL-WITH-DEPS row). The link depends on `input_d` being the value digest (see astra-1 #23). |
| 8 | Registry entry binds proof system, curve, commitment scheme, setup params, transcript, schema and count, adapter, format, policy | VERIFIED | A:2049, A:2053 |
| 9 | Foreign lift checks membership of adapter key and foreign key in `reg_root` | VERIFIED | A:2050 |
| 10 | Different setup parameters: whole check in circuit, never folded | VERIFIED | A:2055 |
| 10b | Parser (on-curve, subgroup, canonical scalars, refusal of missing and trailing elements), transcript-derived challenges, verifier arithmetic and opening equation (7.15 to 7.17) | VERIFIED in ARCHITECTURE | A:2150-2215, rows FV-PARSE, FV-TRANSCRIPT, FV-ARITH, FV-FINAL |
| 10c | Each work unit continues the committed `fv_state_d`; no claim before every phase is done; verification work adds no machine steps | VERIFIED (last atom by inference) | `fv_state_d = H(SHK_FV, entry_d, phase, cursor, transcript_d, acc_d, pi_d, consumed_d)` and FV-SCHED "DONE only after every phase"; A:1761 "Inline owner work is legal only in MERGE-WORK" (an FV statement carries no height) |
| 11 | Bounded verification work units (7.18) | VERIFIED in ARCHITECTURE, beyond BRIEF | A:2052 describes the path in bounded work units as needed for the full GP-3 scope. BRIEF §3.7 says only "enters through the registry". Beats 7.12 to 7.20 (nine beats, about 236 s) add GP-3 detail that BRIEF does not give. The lead must decide on the scope. |
| 12 | Stale registry root refused; `reg_root` read from contract state | VERIFIED | A:1860, A:2048 |
| 13 | Ledger 9.1.0.1; same key for every program; collection statement `[reg_root, set_root, shape_d]` | VERIFIED | BRIEF §3.8 |
| 14 | "The final proof size and verification cost stay independent of graph depth and claim count" (8.6) | NOT FOUND (in BRIEF) | BRIEF gives no size for a collection proof. The claim follows from a fixed decider, but no source states it in these words. The writer's note says it gives no byte figure on purpose. The lead must confirm. |

Humanizer pass: it removed one "would" that the fix had put into 7.7. It replaced the semicolons in 8.2 and 8.7 with periods. It removed filler that had been added only to reach a word count: "therefore" in 7.15, 7.16, 7.20 and 8.5, and "when they arrive" in 7.13.

Beats now shorter than their duration: 7.13 25 s of 26, 7.15 25 of 26, 7.16 22 of 23, 7.20 29 of 30, 8.5 25 of 26.

For the director, with no edit made:
- 8.8 is a recap of all eight steps and adds no mechanism (echo ending, Z-44). It is the closing wide shot, so it may be deliberate.
- Power notation differs between files: astra-1 writes `2^64` and astra-2 writes `2<sup>64</sup>`. CREATIVE-BRIEF 3.3 asks for the superscript.

## Grep results

On the viewer text (narration, labels, titles) of all three cleaned files, these all print nothing:
- `grep -nP '[\x{2013}\x{2014}]'`
- the register grep (designed, planned, proposed, estimate, pending, roadmap, landed, will, would, future, should, may, agent, lane, council, audit, experiment, finding)
- the id and path grep

"open" appears 5 times, every time as the mechanism verb: "opens to five header fields", "every opened field", "the opened `deps_d`" (twice) and "opens its `outcome_d`". "Membership opening(s)", "Opening equation" and "Final opening check" are labels, and they also name the mechanism.

On the whole files, the dash grep prints nothing. The register grep and the id and path grep print hits only in builder text:
- astra-2 director notes, line 519, have "estimates".
- The director notes of all three files have source paths and ids such as SRC-007.
- Each `Numbers used:` line cites `BRIEF.md`.

None of these hits is shown to the viewer.

## STORYBOARD.md (115 beats)

Kind: explanation for the narration (my-voice section 1, reports/studies/explanations row). Titles and labels follow the promo/motion row. The original is saved as `STORYBOARD.orig.md`. Only the six title lines listed below changed. No narration and no label changed, so no duration and no row of `SCENE-CONTRACT.md` section 6 changed. All 115 beat durations equal words / 2.5, rounded up (minimum 12 s), and every scene row of `SCENE-CONTRACT.md` section 6 matches the beat headers.

The narration differs from the cleaned scripts in these beats: 1.4, 1.5 (new), 2.1, 2.5, 3.30, 5.3, 6.1 and 7.10 change the `input_d` wording, and 2.7, 3.15 and 3.34 change only to superscripts. In 3.5, "request channels" became "request lanes". Each changed sentence was read against the catalog and humanizer §1, §2, §6, §8 and §19. None needed an edit:
- 1.5 opens on the formula as its own sentence, the same shape as 5.9 and 7.2.
- 1.5 has three numbers (128-bit, two 64-bit halves, 2<sup>64</sup>). They describe one encoding, and the labels carry the identifiers.

rhythm_check on the narration of all 115 beats:

```
Sentences: 509  mean 13.1  sd 5.1  median 13  max 29
Tic patterns: absolutes×18
Prose smells: announced structure×1
Numbers: 43  per paragraph mean 0.37  max 6  paragraphs over two 6 of 115
Flags:
  - Low sentence-length variation: monotone rhythm.
```

Notes on the counts:
- absolutes: the mechanism verb "proves" and "never" in the rules of the system.
- announced structure: "has six parts" is the count of state parts that BRIEF step 2 gives.
- monotone: the same reason as in the three scripts (a fixed word count per beat, one mechanism per paragraph).

Titles (promo/motion row: plain noun phrase, module names as the system spells them, no slogan):
1. 1.2 "Source to rom" → "Source to ROM"
2. 3.20 "Authenticated rom line" → "Authenticated ROM line"
3. 3.2 "Core register chain" → "CORE register chain"
4. 3.5 "Bus tuple" → "BUS tuple"
5. 5.8 "Equal values, equal faces" → "Equal outcome digests". This fixes a parallel slogan; "faces" is the drawing, and the mechanism is equal digests.
6. 8.2 "Fixed verification" → "Fixed verification cost". The fix states the quantity that stays fixed, which beat 8.2 narrates.

Labels: 401 distinct label texts. Each has four words or fewer (an identifier counts as one word), and no beat has more than six. No label holds a status word, a hedge, a dash, a path, "channel", or `^` in place of a superscript.

voice-verify on the new and changed `input_d` sentences:

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| 1 | `input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)` (1.5) | VERIFIED | BRIEF §3.1; A:2228; A:1409 |
| 2 | 128-bit fuel as two 64-bit halves; fuel halves and stack limit range-checked below 2^64 | VERIFIED | BRIEF §3.1; A:2228 "so the u128 fuel has one encoding"; A:1991 |
| 3 | A different fuel budget or stack limit gives a different `input_d` | VERIFIED (follows from the hash; collision resistance is a premise, A:484) | BRIEF §3.1 |
| 4 | `val_d` under `sherlock-value/1` (1.4, 3.30) | VERIFIED | BRIEF §3.1; A:1409 |
| 5 | Simulator receives value and resource policy; fuel starts at the bound budget (2.1, 2.5) | VERIFIED | BRIEF §3.1 |
| 6 | Header `input_d` binds value and resource policy (5.3); the transaction's `input_d` binds value digest, fuel, stack limit (6.1) | VERIFIED | BRIEF §2 header fields; BRIEF §3.1 |
| 7 | Input link compares the producer's Halted value digest with `val_d` bound in the consumer's `input_d` (7.10) | VERIFIED | A:1984 "the consumer's opened `input_d` is re-hashed from a witnessed `(val_d, fuel_lo, fuel_hi, stack_limit ...`"; A:1996 |
| 8 | Collection proof size and cost independent of shape, depth, claim count (8.6) | VERIFIED | BRIEF §8, second ruling |
| 9 | MERGE-WORK, continuation, GP-3 path presented as built | HELD | BRIEF §8, first ruling |

Grep results on the viewer text (titles, labels, narration):
- The dash grep prints nothing.
- The register grep prints nothing, apart from "lanes" in 3.5, which is the system term that BRIEF §8 names.
- The id and path grep prints nothing.
- "open" appears 5 times, every time as the mechanism verb.

Humanizer pass: no changes beyond the six titles.

## STORYBOARD-AMEND-1.md (14 changed beats, new beat 8.9)

Kind: explanation for the narration. The location chip, labels and inventory card follow the promo/motion row. The original is saved as `STORYBOARD-AMEND-1.orig.md`. Two narration sentences changed. Every stated duration still equals words / 2.5, rounded up.

rhythm_check (narration of 15 beats), after:

```
Sentences: 82  mean 13.6  sd 7.3  median 13.0  max 57
Tic patterns: absolutes×5
Numbers: 4  per paragraph mean 0.27  max 2  paragraphs over two 0 of 15
Flags:
  - Paragraphs are nearly uniform in length.
```

Notes on the counts:
- absolutes: "never runs on Midnight", "never go on-chain" and "never runs the Sherlock chip" state the location rule of the amendment. They stay.
- Uniform paragraphs: each beat has a fixed word count per its duration.
- The 57-word sentence in 8.9 is the inventory list and matches the card. It is a true enumeration, so it stays.

Edits:
1. 3.1, a one-line closer (humanizer §2): "...and it lives on the prover's machine. It never runs on Midnight." → "...and it lives on the prover's machine and never runs on Midnight."
2. 6.3, a run-on sentence of 33 words (Z-30): "...serve every program and every input, so the program identity selects no contract and no key, and a new program deploys nothing." → "...serve every program and every input. The program identity selects no contract and no verifier key, and a new program deploys nothing."

Checks on the narration, labels, titles and inventory text:
- The dash grep prints nothing.
- The register grep prints nothing; "open" does not occur.
- Every label has four words or fewer.

voice-verify (A = ARCHITECTURE.md line):

| # | Claim | Verdict | Evidence |
|---|---|---|---|
| 1 | Contract state holds `runs` keyed by `claim_d`, collection roots, the registry root (8.9) | VERIFIED | A:2526 `runs`, `verified_roots`, `reg_root` |
| 2 | One contract and one decider verifier key serve every program; a new program deploys nothing (6.3, 8.9) | VERIFIED | BRIEF §2 "One contract"; A:2772 GP-001 |
| 3 | Proving keys, witness, private input and trace stay off-chain; only the decider proof and the four values cross (5.13, 6.4) | VERIFIED | BRIEF §3.5, §3.6 |
| 4 | "One proof verification, one lookup of `claim_d` and one insert are the whole on-chain work of a call" (7.4) | OVERCLAIM | A:2531 `acceptRun` also recomputes `claim_d` (one Poseidon), and beat 7.2 shows that step. The sentence leaves it out. |
| 5 | "on a proving service the user picks" (1.1, 4.1, 8.9) | NOT FOUND | Not in BRIEF or ARCHITECTURE |
| 6 | "the wallet on the user's machine" builds the transaction (6.1) | NOT FOUND | Not in BRIEF or ARCHITECTURE |
| 7 | Leaves proved in parallel "across its CPU cores and GPU streams" (3.35, label, 8.9 card) | NOT FOUND | BRIEF §3.4 gives only "three provers in parallel give 1.73x" |
| 8 | "Sherlock chip" as the name for the leaf circuit with its proving key (3.1, 6.4, 7.1, 8.2, 8.9) | NOT FOUND (term) | No source uses this term |
| 9 | The run "executes ... as native code" (2.1) | NOT FOUND | BRIEF names `mori simulate` and a CEK-style machine, with no word on native code |
| 10 | A native Compact contract puts one verifier key on-chain for each exported circuit (8.9 narration and comparison row) | NOT FOUND | Not in BRIEF or ARCHITECTURE. It needs a Midnight or Compact source. |
| 11 | JOIN and `shape_d` are computed off-chain; collections settle in the contract (1.1 table) | VERIFIED | A:2105; A:2519 `acceptCollection` |

Humanizer pass: no further changes.

### STORYBOARD-AMEND-1.md: lead rulings applied

All changed beats keep their stated durations (words / 2.5, rounded up): 2.1 is 28 s, 3.35 is 29 s, 7.4 is 29 s and 8.9 is 56 s.

1. 7.4: the overclaim is fixed. The new sentence reads "One proof verification, the computation of `claim_d`, one lookup and one insert are the whole on-chain work of a call." This follows A:2527-2531. To keep the duration, "These are the values that the verified proof binds" became "These are the values the verified proof binds".
2. 2.1: "as native code" became "as native execution".
3. 3.35: "The machine proves leaves in parallel, across its CPU cores and GPU streams." became "Independent leaves let the machine prove them in parallel on its CPU cores." The label became `CPU cores`. Source: BRIEF §3.3 (partition independence) and ARCHITECTURE.md SRC-023 (line 1613). The GPU part waits for the elastic multicore chip design round.
4. 8.9: the card row "CPU cores and GPU" became "CPU cores". The narration changed from "the CPU cores and GPU that do the proving. One transaction crosses." to "the CPU cores that do the proving. One transaction crosses the boundary." The added "the boundary" keeps the duration.
5. The rest are kept as the lead ruled; each is now VERIFIED or HELD:
   - "proving service the user picks": the proof server, `midnight-ledger/proof-server/src/endpoints.rs:245,328` (ledger-9).
   - "the wallet": `midnight-dapp-connector-api/SPECIFICATION.md`.
   - Native Compact keys, one verifier key per entry point: `midnight-ledger/onchain-state/src/state.rs:997,1136-1170`.
   - "Sherlock chip": the owner's term.

# Slop rules for the pipeline script

Use this checklist on every word a viewer can read: narration paragraphs, on-screen labels, scene titles, controls and alt text. Narration is explanation prose: present-tense mechanism, standalone. Labels are frame text: one short phrase, at most four words or one identifier with one value.

## 1. Facts

- Keep every fact, number, identifier, inline code, tuple, formula, beat id and duration exactly as written.
- Never add a fact. If a sentence needs a detail that `BRIEF.md` does not give, write a simpler sentence.
- Never drop a mechanism. Cut staging and repetition only.
- Use only the numbers of `BRIEF.md` sections 2 and 3. A quantity with no number in `BRIEF.md` appears without a number ("a fixed number of cache lines per leaf"), with no label on it.
- Keep the qualifiers that `BRIEF.md` itself gives: "about 65 s", "about 6.4 KB", "1.73x throughput".

## 2. Register

- Use the present tense. The system is built and runs. Describe what it does.
- Never use these status words: designed, planned, proposed, estimate, pending, roadmap, landed, will, would, future, should, may, in principle, roughly speaking.
- Use "open" only as the mechanism verb: open a header, open a stack link, a Merkle opening, a cache line opened against the pre-state root. Never use "open" as a status ("an open question", "still open").
- Never name how the system was made: no agent, lane, council, audit, experiment, finding, reviewer, decision ids (R-4, OD-1), profile names, file paths, crate names, line numbers, commit ids.
- No apparatus: no "this scene shows", "as we saw", "next we look at", no counts of beats or labels.
- No alternative backend, no rejected option, no comparison with one.

## 3. Punctuation

- No em dash (U+2014) and no en dash (U+2013) in narration or labels. Use a period, a comma, a colon or parentheses. Ranges take "to": "stations 1 to 5".
- A hyphen inside an identifier or a term stays: `sherlock-value/1`, `sherlock-k-a/1`, pre-state, self-recursive, 64-bit, 12-bit.
- Check: `grep -nP '[\x{2013}\x{2014}]' FILE` prints nothing.

## 4. Banned patterns, with one fix each

### Not X but Y (humanizer section 1)

Before: "The node does not just check the proofs. It joins them into one."
After: "The node checks both child proofs and joins them into one slab."

Allowed: one contrast that corrects a belief the viewer holds. "The program is data, not circuit" stays, because a viewer expects one circuit per program.

### One-line closers and fragments (section 2)

Before: "The two tokens slide onto one spot. Same digest. Same run. Nothing else matters."
After: "The two `post_d` and `pre_d` tokens slide onto one spot, so the right slice starts where the left slice ends."

### Sayings that sound deep (section 3)

Before: "At its core, the digest is the language of trust between the prover and the chain."
After: "The call transcript binds the four values, so changing any of them invalidates the proof."

### Forced triads (section 6)

Before: "The leaf is compact, uniform and universal."
After: "The leaf is one circuit with one key for every program."

Keep a list of three when the system has three things: the three outcomes, the three keys, the three MERGE equalities.

### Dashes as connectors (section 8)

Before: "A full tail — eight cells — becomes a segment."
After: "A full tail of eight cells becomes a segment."

### Overused AI words (section 12)

Watch for: crucial, key (adjective), pivotal, seamless, robust (figurative), elegant, powerful, simply, quietly, effortlessly, essentially, leverage, underscore, highlight, showcase, landscape, journey, magic.

Before: "A crucial check ensures that the seal is robust."
After: "The decider requires a terminal outcome: a Running state cannot seal."

### Inflated significance (section 13)

Before: "This marks a pivotal moment: the claim becomes a permanent part of the network's history."
After: "The claim is now a fact on chain: any contract or later proof can rely on it."

### Avoiding is, are and has (section 18)

Before: "The genesis leaf serves as the anchor of the run and boasts an INIT module."
After: "The genesis leaf is the first leaf of the run. It runs INIT at height 0."

### Bold as decoration (section 19)

Before: "The **node** checks **three equalities** and emits **one slab**."
After: "The node checks three equalities and emits one slab."

Code formatting stays on identifiers. Bold appears nowhere in narration or labels.

### Writing about the document (section 25)

Before: "In this beat we see how the bus carries a request."
After: "CORE puts a tuple `(chip, key, a, b, c)` on the bus, and the MEM chip answers it."

## 5. Voice-audit checks for this script

The narration is the "explanation" kind (`my-voice` section 1). Run `python3 ~/Projects/workflowskills/skills/voice-audit/scripts/rhythm_check.py FILE --genre essay` and give each flag a finding.

### Number walls

Keep every number: here the numbers are the facts. Put the one number a sentence needs in it, and move the others to labels, not to a run of figures in one sentence.

Before: "A flush is 8,821 rows, an old read is 4,761 rows, the leaf fits 2^16 rows, the node runs at k = 20 and takes about 65 s at 20 threads and 17 GB."
After: "A flush is 8,821 rows. An old-memory read is 4,761 rows." The leaf bound `k <= 16` goes on a label; the node figures go to scene 4 with the node.

### Dilution and stacked qualifiers

State the mechanism flat. No "we think", "in effect", "essentially", "in a sense". One qualifier on a number at most, and only the one `BRIEF.md` gives.

Before: "In effect, the node essentially stays roughly the same size."
After: "A node of any height has the same size."

### Counting and announced structure

Do not count the beat's own points ("three things happen", "first, second, finally") unless the system has that count: the three outcomes, the three equalities, the seven lanes and the four public values are system counts and stay.

Before: "This beat makes two points. First, the leaf has one key. Second, ..."
After: "The leaf has one key. ..."

### Nominalizations and light verbs (P-23, P-24)

Before: "The node performs the verification of both children."
After: "The node verifies both children."

### Negation load (P-42)

One negation per sentence. State what the check does, then what it rules out.

Before: "No prover cannot skip a step without the rule not matching."
After: "Each step must match its rule. A prover that skips a step fails that match."

### Lost referent (P-39)

Repeat the noun where "it" or "this" could mean two digests.

Before: "The node compares it with this."
After: "The node compares `L.post_d` with `R.pre_d`."

### Given before new (P-28)

Start with the object the viewer already sees and end on the new fact.

Before: "A Merkle opening against the pre-state root is how an old record is read."
After: "An old record is read by a Merkle opening against the pre-state root."

### Pre-valued facts (Z-48)

Never "remarkably", "interestingly", "notably", "importantly" before a fact.

### Signposts and maps (P-20)

No "now we turn to", "next", "here we see", "as the camera shows". Each paragraph opens on its subject.

## 6. Shapes to use

- Narration: say what moves, why it moves and what it proves. Short declarative sentences. Subject first: "The decider opens the header of `post_d`."
- Labels: one short phrase per frame, at most four words, or one identifier with its value. "One leaf key", "`S = 16`", "depth 63". No status word, no hedge, no path.
- One term for one thing, spelled as in `BRIEF.md`, from first use to last.
- No rhetorical questions, no second-person pep, no slogans.

## 7. Check before hand-off

Run these checks on the viewer text only: the `Narration:` paragraphs, the `Labels:` lines and the scene titles. `On screen:`, `Numbers used:` and the director notes are working text for builders.

1. `grep -nP '[\x{2013}\x{2014}]' FILE` prints nothing.
2. `grep -niwE 'designed|planned|proposed|estimate[sd]?|pending|roadmap|landed|will|would|future|should|may|agents?|lanes?|council|audits?|experiments?|findings?' FILE` prints nothing in prose. ("lanes" for the seven public lanes and the two request lanes of the bus is a system term and stays. "enabled" for the 120 enabled rules stays.)
3. `grep -nE '\b(R|OD|SRC|GP)-[0-9]+|[A-Za-z_/]+\.(md|rs|js|qnt|lean)\b' FILE`: no decision id or file path. GP-1 to GP-4 stay only where `BRIEF.md` names them.
4. `grep -niw 'open' FILE`: each hit is the mechanism verb.
5. Every number in the prose appears in `BRIEF.md`.

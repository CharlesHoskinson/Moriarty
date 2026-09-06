# Prose pass brief (inkwell edit and revise, then humanizer)

You are giving one finished chapter of the ZKIR K definition documentation a
prose pass. The chapter is technically correct and has been checked against
the sources; your job is the writing, not the facts.

## What must not change

- Any fact, number, name, path, symbol, message string, command or claim.
  Every claim in the chapter survives into the rewrite; none is added. If
  you cannot rewrite a sentence without changing what it asserts, leave it.
- Code blocks, tables (cell text may be tightened only if the meaning is
  identical), headings' meaning, link targets, inline code spans.
- The chapter's section structure and its one `#` title.
- The register: technical reference documentation in plain present tense.
  Neutral is the correct voice here; do not add opinions, first person,
  humour or rhetorical hooks.
- The rules of `briefs/COMMON.md`: no em-dashes or en-dashes, no
  placeholders, no mention of drafting, reviewing, auditing, personas,
  agents, editors or "this document", no status tags, no dates of writing.

## What to do, in order

1. Gottlieb edit (read `${PLUGIN}/agents/gottlieb-editor.md`). Read the
   whole chapter once for impression, then again to diagnose. Name the
   patterns: docent's elbow, coda inflation, voice in the workshop,
   assertion by adverb, the "not X but Y" reflex, superlative fatigue,
   sentence-length uniformity, machinery in the prose. Audit the five tell
   families (content, language, style, communication, filler and hedging).
   Write the findings to `${SURGE}/prose/<NN>-gottlieb.md` in the agent's
   output format, then apply them: cut what should be cut, keep what works.
2. Le Guin revise (read `${PLUGIN}/agents/leguin-reviser.md`). Read the
   applied text aloud in your head. Fix the worst rolling window of monotony
   first: vary sentence length where every sentence has the same shape, join
   or subordinate where a run of short declaratives is merely the default
   register, break an over-long sentence at its natural joint. Do not
   manufacture variation (no fronted participles or inversions for their own
   sake). Measure before and after with
   `python3 ${PLUGIN}/RSI/metrics/compute_kpis.py <file>` and run the
   displacement check
   `python3 ${PLUGIN}/narrative/metrics/displacement.py ${BEFORE} ${AFTER}`
   where `${BEFORE}` and `${AFTER}` are two private directories of your own
   under the scratchpad, each holding one file named `chNN.md` (the script
   only reads files matching `ch*.md`): the original of your chapter in the
   first, your revised text in the second.
   If any habit rose, fix it and re-run.
3. Humanizer (read `~/.claude/skills/humanizer/SKILL.md`, file mode). Run
   the draft, audit, final loop on the result and answer its two questions
   in your report: what still reads as machine-written, and does the
   rewrite state any fact not in the source. Register rule: for reference
   text neutral and plain is the correct human voice.
4. Write the final text over `experiments/zkir-k/docs/<chapter>`. Run
   `python3 ${SURGE}/lint_docs.py` and make sure your chapter has one title
   and zero hits (a `CLM-` hit inside a parenthetical citing the plan is
   allowed in chapters 05, 08 and 15).
5. Preservation check on yourself: `diff` the original (from `${BEFORE}`)
   against your final and confirm, line by line, that every number, path,
   symbol, message string and command is unchanged and no sentence asserts
   something the original did not. Fix anything that slipped.

## Report

Reply with: DONE <chapter>, the word count before and after, the
sentence_length_variance before and after, the displacement result, the
three or four Gottlieb patterns that mattered most in this chapter, the
humanizer's two answers, and any sentence you left alone because rewriting
it would have changed its meaning.

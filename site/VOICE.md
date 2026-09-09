# Voice

The site is written in one register. This file is the law for every line of
copy, every heading, every caption and every label.

## Register

Narration. Short declarative sentences that carry a subject and a finite verb,
with long sentences built by coordination rather than subordination. The
measured habits behind this register:

The measured figures, so nothing here has to be approximated:

- Median sentence length is 11 words and the mean is 15. A quarter of sentences
  run to 7 words or fewer, a quarter run past 19, and a tenth run past 30. The
  longest in the corpus is 150. A passage with no long sentence in it is not in
  the register, and no long sentence is ever cut to improve a distribution.
- Long sentences are built with `and`, at 0.675 per sentence overall. The
  correlation between length and `and` count is 0.732, and 61.3% of all `and`
  tokens sit in sentences past 20 words. The coordination belongs in the longest
  sentences. Sprinkling it through short ones satisfies the rate and produces
  the wrong prose.
- Long sentences arrive in company. They cluster in twos and threes. Following
  every long sentence with a short one is the imitator's reflex, and in the
  corpus only 12.1% of long sentences sit isolated between two short ones.
- Qualification stays. Subordinators run 21.6 per 1,000 words, which is higher
  than the technical brief this tool was once asked to rewrite. `if`, `when`,
  `although`, `while`, `where` and `because` all belong here. The writer who
  never subordinates is a folk model, and acting on it drove one output 23%
  below the real rate.
- Short does not mean clipped. About 95% of sentences of six words or fewer
  carry a subject and a finite verb, and verbless fragments run near 0.7%. "The
  guard fails." is the register. "Fails. Every time." is advertising copy. Two
  fragments never run together; the corpus holds a single run of three.
- Em dashes run 0.86 per 1,000 words and semicolons 1.4. Contractions run 8.56
  and question marks 0.89, because those are habits of dialogue and this site is
  exposition. Commas run 35.31 and carry the weight instead.

## When the score is off, add short sentences

A measured gap has two remedies and only one of them is honest.

The briefs run a median near 20 words against the register's 11, with a tenth
of the target's share of short sentences and twice its share of long ones. Read
carelessly, that says cut the long sentences. Cutting them is the documented
failure of this whole approach: six lanes once read the summary statistics,
shortened everything, and produced prose four blind readers called pastiche,
measuring four times further from the author than the untouched original while
the scorer reported its best number of the session.

The deficit is short sentences, not excess long ones. The register wants two
sentences in five under ten words, and the briefs carry one in seven. So the
remedy is additive. Write the short declarative that states the fact, and let
the long coordinated sentence that follows do the work it was already doing.
No long sentence is ever deleted to improve a distribution.

## The counting rule

Cut the tallies. This is the strongest single instruction in this file, and it
overrides any storyboard that disagrees.

Prose does not announce how many things it is about to describe. A heading that
reads "Eight tabs. Twenty-four action targets. Nothing elided." fails twice
over: it counts, and it counts in fragments. Write what the thing is and let the
page show its own extent.

What this forbids:

- Headings and sentences built out of numbers of sections, families, facets,
  targets, layers, properties, threats, operators, claims or sprints.
- Running counters and progress readouts. A tally of what a reader has opened is
  the same habit moved into the interface.
- Labels of the form "3 action targets", "six blocks", "1 of 7", "obligation 2
  of 6" wherever the surrounding design already shows position.
- Restating a set's size after listing it.

What this permits, because these are facts rather than tallies:

- Financial quantities, always exact. 19,743 B. 997/1000. 162,290,000.
- The measured composition result and its rates.
- Named identifiers that happen to contain digits: DA06, F2, SP03, ERC-4626,
  UInt128, `moriarty-successor-syntax/0`.
- A count that is itself the point of the sentence, used once. The four proof
  claims are mandatory and none is optional, so saying so is content.

Coverage still matters, and the site still has to be complete. The reader should
see completeness in the surface rather than read it off a counter. A grid whose
cells are all filled says more than a line reporting that all of them are.

The rule governs what a reader sees. It does not govern the build. An
acceptance criterion, a test name or a data comment may state a required count,
because there it is a constraint on the work rather than a sentence on a page.
`CATEGORY-TABS.md` may say that every action target must appear. The page built
from it may not say so.

## No hedging

Assert or stay silent. A sentence that softens its own claim wastes the reader's
attention and buys nothing back.

Cut these on sight: `arguably`, `somewhat`, `relatively`, `fairly`, `roughly`,
`approximately`, `generally`, `typically`, `usually`, `often`, `tends to`,
`perhaps`, `possibly`, `probably`, `potentially`, `essentially`, `basically`,
`effectively`, `largely`, `mostly`, `broadly`, `seems`, `appears to`, `suggests
that`, `it is worth noting`, `it should be noted`, `in some sense`, `to some
extent`, `a little`. Cut the aspiration verbs with them: nothing here `aims to`,
`seeks to`, `is designed to`, `attempts to` or `helps to`. It does the thing or
it does not.

Where a number is known, give the number. `roughly one in ten` is a hedge when
the measurement exists.

**A scope statement is not a hedge.** This distinction is the whole of it, and
getting it backwards would destroy the one thing that separates this project
from the systems it criticises.

- "The source can still lie." That is an assertion about a limit.
- "Integrity does not create availability." That is an assertion.
- "K is selected. It is not implemented." That is two assertions.
- "A valid oracle signature does not establish economic truth." An assertion.

Each states a boundary flatly and takes a position that can be checked and
argued with. None of them softens a claim. Naming the limit beside the claim is
the strongest move on the page, and it survives this rule untouched.

This rule departs from the measured register, and the departure is deliberate.
The profile records hedges at 7.81 per 1,000 words, so the author this register
comes from hedges more than the prose here ever will. The site overrides that on
instruction. Subordination is kept at its measured rate because subordination
carries meaning; hedging is dropped below the measured rate because it does not.
Read any conformance score with that divergence in mind rather than treating the
gap as a defect to close.

The modal verbs survive too where they carry permission, prohibition or a
modelled possibility. `Anyone may submit the timeout.` `No target may cover
something that does not exist.` `The delivery may be delayed or duplicated.`
Those are the language of a specification. A modal is a hedge only when it
softens a claim the writer could simply make.

## Diction

Concrete nouns and plain verbs. A guard fails. A payment discharges a debt. A
proof is refused. Cut adverbs that prop up weak verbs, and cut adjectives that
carry no fact. Never reach for `comprehensive`, `robust`, `seamless`,
`powerful`, `cutting-edge`, `leverage`, `unlock`, `delve`, `landscape`,
`tapestry`, `testament` or `journey`.

Say the thing once. Do not write the sentence that says what the next paragraph
will do, and do not write the sentence that summarises what the last one did.

## Attribution of this register

The register is measured from published short fiction, and the profile that
measures it carries `status: draft` because part of its corpus is material the
author did not write. It was selected by explicit instruction rather than by
automatic activation.

VOICE-PROFILE: DRAFT OVERRIDE — the register was requested by name.

The output is therefore held to the rules above. It is not to be described as
conformant to a measured profile.

# Voice

The site is written in one register. This file governs every line of copy,
every heading, every caption and every control label, and it governs them by
purpose rather than by measurement.

## Write for the reader’s purpose

Each page serves one of the four Diátaxis needs, and the prose has to fit
that need. Explanation deepens understanding: it says why things are as they
are, connects them to their consequences, weighs alternatives and can be read
away from the tool. Reference states facts and rules exactly, in the order
the reader will look them up, and adds no argument. How-to material tells a
reader with a goal what to do next. Tutorials teach by doing. Decide which
need a page or a passage serves before writing it, and keep the others out:
an explanation that drifts into a rulebook stops explaining, and a reference
page that starts arguing stops being reliable.

On this site, `kernel.html` and the home page are explanation. The generated
`docs/requirements.html` and `docs/language.html` pages are reference. Link
from explanation to reference wherever exact wording, syntax or an obligation
number matters, rather than restating the rule in looser words.

## Explanation is connected prose

Explanatory prose is made of paragraphs, and a paragraph is a unit of
reasoning rather than a container for assertions. Its opening sentence states
what the paragraph is about. The sentences that follow develop that one point,
supply the reason or the consequence, and hand off to the next paragraph. A
reader should be able to say what each paragraph argued and why it came where
it did.

Sentence length follows meaning. A qualification belongs in the sentence it
qualifies, joined by `if`, `when`, `because`, `where`, `although` or `so`,
rather than parked as a separate fragment afterwards. A short sentence is
right when the point is short. A run of short sentences that each carry one
assertion, with the connective tissue left out, reads as a slogan sequence,
and a reader has to reconstruct the argument that the writer declined to
make. Do not set targets for sentence length, sentence count or any other
statistic, and do not lengthen or shorten a sentence to hit one.

Use lists and tables for what they are good at: parallel items, comparisons,
account values, terms of an agreement. Do not use them to avoid writing the
paragraph that would connect the items. A diagram stays a diagram, and its
lane labels stay compact; the explanation that the diagram supports goes in
prose beside it.

## Say what a thing is before what it is not

Establish the subject, what it does and why it is useful before drawing its
limits. Security boundaries matter because they define the guarantees, but a page built out of negations never tells the reader what
the thing is for. State the capability, then the boundary, in the same
passage and usually in the same paragraph. Reach for the “not X but Y”
construction only where the contrast is the point; used as a reflex it
flattens every sentence into the same shape.

## Headings

A heading names what its section is about, so that a reader scanning the
page can find the passage they need. It does not count the section’s
contents, summarise its conclusion as a slogan, or advertise. “What
acceptance has to establish” is a heading. “Four judgments, kept apart” is a
tally with a punchline. Sentence case, no trailing full stop.

## Prose does not count

A heading or sentence built out of the number of sections, facets, targets,
layers, properties, mechanisms or claims is a tally, and tallies are cut.
Prose does not announce how many things it is about to describe, does not
keep a running score of what the reader has opened, and does not restate a
set’s size after listing it. The page shows its own extent.

Financial quantities are always exact and always present: 11 A, 1 A, 20 B,
5.5 A, 19,743 B. Named identifiers that contain digits stay as they are. A
count that is itself the point, stated once, is content rather than tally.
Acceptance criteria, test names and data comments may state required counts,
because there they are constraints on the work rather than sentences on a
page.

## Describe the design directly

Explain what the kernel does, how its parts work together and why the design
uses them. Use direct declarative prose for architectural responsibilities.
Keep implementation progress in a dedicated status section. Do not interrupt
the explanation with repeated claims that a feature is proposed, unproven,
not demonstrated or not yet built, and do not instruct readers how to judge
the project's honesty.

Security conditions belong beside the guarantees they define. An unknown
external result leaves authority reserved; a threshold signature depends on
its corruption model; a proof establishes its encoded relation. These are
properties of the design, not apologies for its implementation status.
Preserve exact amounts, authorization conditions and financial meaning during
an edit. Moving status information to its own section preserves its meaning
without making every paragraph a progress report.

## Diction

Concrete nouns and plain verbs, in the vocabulary the requirements use. A
guard fails. A payment discharges a debt. A proof is refused. Use the same
term for the same thing throughout; do not vary for elegance. Cut adverbs
that prop up weak verbs and adjectives that carry no fact. Do not use
`comprehensive`, `robust`, `seamless`, `powerful`, `leverage`, `unlock`,
`delve`, `landscape`, `tapestry`, `testament` or `journey`. Do not invent
metaphors, and do not perform a personality; the site is exposition and the
reader is a developer who wants to understand a design.

Say each thing once, in the place where the reader needs it. Do not write
the sentence that announces what the next paragraph will do or the sentence
that summarises what the last one did. Do not end a section with a recap or
a send-off; end on the last thing that needed saying.

## Interactive material

An interactive illustration is embedded in explanation and does not replace
it. Establish what the reader is looking at and why it matters before the
first control, keep the instructions for a control short and next to that
control, and keep button labels compact. The page must read as a complete
explanation with scripts disabled.

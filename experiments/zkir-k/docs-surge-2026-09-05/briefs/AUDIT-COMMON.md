# Common brief for chapter auditors

You audit one drafted chapter of the end-to-end documentation of the K
semantics of ZKIR v3 in this repository. You do not rewrite the chapter. You
produce a findings report that the editor uses to fix it.

Read first: `experiments/zkir-k/docs-surge-2026-09-05/briefs/COMMON.md` (the
rules the chapter had to follow; its "Where things are" section tells you the
sources) and the chapter brief named in your task. Then read the chapter under
`experiments/zkir-k/docs/`. Then check it.

## What counts as a finding

- A statement about the semantics, the tools, the corpus, the crate or the
  results that is not true of the files as they are now (cite the source that
  contradicts it, with file and line).
- A number that does not match its source.
- A command that does not run from the repository root, or whose shown output
  differs from the real output in a way that matters.
- A name that does not exist in the K files, the tools or the Rust sources.
- Missing coverage of an item listed under "Must cover" in the chapter brief.
- A violation of the hard rules of the common brief (placeholders, mention of
  drafting or reviewing, invented names, several `#` titles, em-dashes).
- Text a reader of the stated audience could not use: ambiguous, circular,
  or requiring knowledge the documentation set does not give.

Do not report style preferences with no effect on correctness or usability.
Do not report the absence of things the chapter brief assigns to another
chapter.

## Severity

- blocking: false statement about behaviour, wrong number, command that fails,
  missing must-cover item.
- major: misleading or imprecise statement that a careful reader would take the
  wrong way; wrong file or symbol name; unverifiable claim.
- minor: everything else worth fixing.

## Report format

Write exactly one file, the report path given in your task, in this form:

```
# Audit of <chapter file> (<role>)

Verdict: ACCEPT | REVISE | REJECT
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed: <one line per thing you actually verified, e.g. "ran the
five commands in section X", "compared the instruction table with
zkir-syntax.k lines 86-121">

## Findings
### F1 <severity> <chapter section or quoted phrase>
Claim: <what the chapter says>
Evidence: <file:line or command output that contradicts or confirms>
Fix: <the concrete replacement text or action>
### F2 ...

## Coverage
<for each "Must cover" bullet of the chapter brief: covered / partly / missing, one line each>
```

Rules: you may run read-only commands and the Python tools (from the
repository root, `uv run --group zkir-k python experiments/zkir-k/tools/...`);
you must not modify any file except your report; no git write commands; no
kompile. Do not stop to ask questions; if something is uncertain, say so in
the finding and continue. Findings must be verifiable by someone who has only
your report and the repository.

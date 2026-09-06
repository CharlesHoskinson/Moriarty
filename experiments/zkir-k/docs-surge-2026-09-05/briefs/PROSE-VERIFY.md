# Preservation check after the prose pass

You compare the original of one chapter with its prose-edited version and
report every place where the meaning changed. You do not edit the chapter.

Inputs: the original at `${BEFORE}/<chapter>` and the edited chapter at
`experiments/zkir-k/docs/<chapter>`. Produce a unified diff first
(`diff -u`), then read both versions in full.

Report, in this form, to `${SURGE}/prose/<NN>-verify.md`:

```
# Preservation check of <chapter>

Verdict: PRESERVED | CHANGED

## Meaning changes
### C1 <severity blocking|major|minor> <section>
Before: <quoted>
After: <quoted>
What changed: <the fact, number, name, path, symbol, command, message or
claim that differs, or the claim that was added or dropped>
Fix: <the exact text to restore>

## Dropped material
<claims, sentences or list items present before and absent after, each
with a one-line judgement: harmless cut / must restore>

## Rule violations
<em or en dashes, placeholders, process words (draft, review, audit,
persona, agent, editor, "this document"), several `#` titles, altered code
blocks or tables, altered link targets, altered inline code>

## Residual machine tells
<sentences that still read as machine-written, quoted, with the pattern
named>
```

Severity: blocking when a fact, number, symbol, path, command, message or
claim changed or a claim was added; major when a claim was dropped that the
chapter brief (`briefs/<NN>.md`) lists under "Must cover"; minor otherwise.
A rewording with identical meaning is not a finding. Verdict PRESERVED means
no blocking or major finding.

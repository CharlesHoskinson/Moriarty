# Independent PL review: README specification coverage and usability

Reviewer: separate GPT-6 Astra agent `/root/pl_readme_spec`, not the author.
Tree: `9157a29700233a23d3ecdd22f92807533e983f34` in `/home/charl/Moriarty`.
Scope: read-only comparison of root README, canonical successor grammar, lexical rules, syntax bounds, successor README, SP02 plan, funded profile and the two linked successor examples. No product edits, network activity, financial execution or infrastructure changes. Scratch report only.

Startup: loaded checked-in development skill and AGENTS.md; ran guarded `status --json`. No pending transactions. Status reports operational-history blockage for loan-swap repair; that dependent dispatch was not attempted.

## Verdict

The README faithfully and completely reproduces the current provisional successor grammar file, and its adjacent explanation is adequate for readers to locate lexical rules and simultaneous parser limits. Its presentation is structurally suitable for GitHub Markdown. This establishes documentation coverage of `moriarty-successor-syntax/0`, not completion of the full future Moriarty language or SP02.

No blocking README-specific coverage or structural formatting defect found. Exact ISO/IEC 14977 conformance and parser-language equivalence require the other reviewers' findings; this report does not certify those predicates. Actual GitHub rendering, highlighting and responsive display were not observed here and remain with the parent reviewer.

## Findings and minimal improvements

1. **Low: make the completeness qualifier local to its sentence.** README.md:49 says “complete current canonical grammar.” README.md:41 and the embedded header already identify the provisional profile, and lines 45 and 166 prevent a full-semantic-completion reading. Still, a reader arriving at the section anchor would benefit from “The complete current grammar of the provisional `moriarty-successor-syntax/0` profile follows, including its admission notes.” This is a clarity refinement, not a missing-production finding.
2. **Low: ambiguous prose in precedence note.** grammar.ebnf:101 / README.md:152 says `not a == b is not (a == b)`. The intended reading is correct under the productions, but an English reader can parse “is not” as a denial. Use `not a == b parses as not (a == b)` and preserve byte equality between canonical grammar and README. No operator or parser change is implied.
3. **Optional: show the reserved-word boundary more directly.** README.md:162 explains that keywords cannot be identifiers, while the complete list and contextual `pre`/`post` status reside in the linked lexical specification. This is sufficient for a linked specification; adding a short sentence that `pre` and `post` are ordinary identifiers and linking directly to `lexical.md#identifiers-and-keywords` would reduce confusion for the syntax-only example. Do not imply that every unsupported declaration name is globally reserved: the lexical list, grammar terminals and unsupported-declaration list have distinct roles.

## Coverage observations

- Repository observation: the sole `ebnf` fenced block is byte-for-byte identical to all 108 lines of canonical grammar, including lexical special sequences, EOF and ten admission notes.
- Repository observation: declarations, action statement categories, trailing postconditions, type arguments, expression precedence, optional calls, projections and lexical nonterminal definitions appear in the README, not merely excerpts.
- Repository observation: the EBNF introduction explains each notation used in the displayed grammar (`=`, `;`, `,`, `|`, optionality, repetition, grouping, terminals, special sequences and comments). The separate lexical link provides token priority, reserved words, whitespace/comment details, Unicode and escape policy, source spans, the `>=` type-close split and EOF-inclusive token bound.
- Repository observation: README bounds match syntax-profile.json's published numerical limits. It correctly distinguishes these from financial runtime limits and atomic execution bounds.
- Repository observation: the root README names the atomic and successor profiles separately, tells readers that later examples use atomic syntax, and distinguishes the funded preparation subset from general syntax acceptance. The linked funded example uses the same syntax header deliberately; the funded profile documents API selection of semantics. The syntax-only partial-payment example's state, guards, update and postcondition all fit the displayed grammar, while the README explicitly says it does not implement a funded payment.
- Repository observation: SP02 still requires static semantics, the fuller source contract, profile-explicit checking, broader positive and negative financial examples, and participant-based syntax evaluation. The provisional successor README expressly excludes obligation/request/composition declarations and semantic checking. The full language is not complete, despite the current grammar reproduction being complete.

## Display and link checks

- Heading sequence is coherent: the grammar is an H3 under the H2 language-specification section, followed by the next H2 developer section. No heading-level jumps in the inspected hierarchy.
- The fence uses `ebnf`, starts at column zero, closes correctly, and has surrounding blank lines. Grammar punctuation is protected from Markdown interpretation.
- Longest grammar line is 76 characters. This is a reasonable desktop code-block width; a 108-line block requires vertical scrolling. Narrow-screen horizontal scrolling cannot be excluded without rendering. Paragraph source lines are long but Markdown renders them as wrapping prose.
- All ten local links in the inspected specification section resolve to present files. Remote links were not checked in this local review.
- The existing `node --test experiments/moriarty-language/tests/readme-grammar.test.mjs` passes: 1 test, 0 failures. An independent extraction comparison also confirms byte equality. This check establishes synchronization, not ISO conformance or accepted-language equivalence.
- `git status --short` returned no changes at the time of inspection.

## SHA-256 of inspected candidate files

```text
bfd450586dfd719e1451e0892fbfb91ce70fca3583cd0fbe1c3c3f5078a998cb README.md
8e46569779725f7cc450d4d88375f320050c6b7a8bbe34f53325ce55a184ac54 experiments/moriarty-language/spec/successor/grammar.ebnf
4b1e18117a59e4304955f8227dabe7891e1ace44fe98eb8d111d4428d0d78dc8 experiments/moriarty-language/spec/successor/lexical.md
07d5a4a8a528cfbaaf617d133f3a5021823315d0e42b04acd82237c028f0d988 experiments/moriarty-language/spec/successor/syntax-profile.json
eaeafafc52860ce9d03b052c055e362aa97106395d90b03e3d174f15de3cfc30 experiments/moriarty-language/spec/successor/README.md
8297e2b9ed84e1cc4a6dad00846653ee83a7475965eb7b0eb73a0293077d268c openspec/sprints/sp02-complete-mori-authoring-frontend.md
e6c70a8ddb980fd5d86f161597cfa40e2d9960a52c8891c8b1e44ed973b8cc20 experiments/moriarty-language/spec/successor/funded-source.md
39d0619d60407c3ba41fbc43abf990efaa362e76d1736b59ba6b74d574201446 experiments/moriarty-language/spec/successor/examples/partial-payment.mori
553b9d3d7c02e1c6de2429506d1fb4ca4962061921bc8aad3af89e65d205904e experiments/moriarty-language/spec/successor/examples/funded-partial-payment.mori
```

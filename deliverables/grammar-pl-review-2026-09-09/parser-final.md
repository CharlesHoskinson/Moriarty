# Final independent parser/lexical review

Verdict: APPROVE for the current syntax-profile documentation and notation scope. Earlier P3 Unicode diagnostic mismatch is resolved. No new blocking ambiguity or parser conformance defect found in the three-file candidate.

Checkout: `/home/charl/Moriarty/.worktrees/grammar-pl-review`. Restored repository develop skill and refreshed CLI status; no pending transactions. Operational-history restriction unchanged. No product edits or network/native/K activity.

Reviewed the exact three-file diff. Nonterminal camelCase renaming changes specification notation only; source terminals and production structure are preserved. Comparison and `not` notes now state parser behavior without ambiguous prose. The README's EBNF fence equals the complete canonical file byte-for-byte. The syntax profile remains provisional and distinguished from semantic completeness.

The lexical Unicode rule now matches the implementation's L/N/Pc/Mn/Mc predicate, both initially and following ASCII identifiers. Re-ran standalone U+0301/U+0903 and ASCII-prefixed versions: all produce NON_ASCII_IDENTIFIER. Also checked enclosing mark U+20DD, NBSP and emoji produce UNEXPECTED_CHAR. EOF wording agrees with scanner behavior: trailing ignored material is accepted, trailing non-ignored source rejects, unterminated block comment rejects, and a missing closer at EOF reports a zero-width span at the UTF-8 source byte length. Focused precedence/comparison probes match revised admission notes. Exact focused output: `parser-final-probes.out`.

Prior empirical evidence remains applicable: frontend SHA-256 `910f0012e57b82b677744c60a4f5bd1f30c4e9c63dc7234a634214bb334e99de` and formatter SHA-256 `bc1f7ac114b0c870ccd8569827bd1416a564c9edc9885cb1748220bb4d1968ef` are unchanged from the original review. The earlier 1,196 probe checks and 78 existing-test passes are retained in sibling reports/output; the generated suite was not repeated for documentation-only changes. Root's reported 236-test result was not independently rerun here.

Exact reviewed candidate SHA-256:

```text
edee090bb4eb2492169d3f2dc1cd545cc8b1a6f92bb6b12bd3ba9c19a7a2e1a7  README.md
a3376201d04dc2dd55049798a33af5be3d82701e0079adf8a18d4f6735dff345  experiments/moriarty-language/spec/successor/grammar.ebnf
88fa3353d1b1a889f1d06f8b35868ae187800719c67ab8345ebb09515e8de639  experiments/moriarty-language/spec/successor/lexical.md
```

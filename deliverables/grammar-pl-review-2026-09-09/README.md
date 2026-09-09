# Successor grammar review — September 9, 2026

The EBNF is complete for `moriarty-successor-syntax/0`, together with the linked lexical rules and bounds. It is not the complete future language: successor semantic freeze and full SP02/SP03 acceptance remain open.

Three independent GPT-6 Astra PL-focused agents reviewed formal notation, parser conformance, and README/specification clarity. All three passed the final candidate identified in candidate.json. The correction renames EBNF meta-identifiers to ISO/IEC 14977-compatible camelCase, clarifies existing Unicode diagnostics and EOF behavior, and distinguishes syntactic admission from typing. Runtime acceptance is unchanged. ISO/IEC 14977 clauses 4.14–4.15 are the normative basis for meta-identifiers: https://www.cl.cam.ac.uk/~mgk25/iso-14977.pdf.

Verification: the root GPT-6 agent ran all 236 existing language tests; all passed (language-tests.log). The parser reviewer additionally performed 1,196 independent checks and 18 focused final checks. These results are scoped tests, not an exhaustive ambiguity or semantic correspondence proof. Initial findings, final reviews, probes and outputs are preserved alongside this report.

The README contains the canonical 108-line, 36-production grammar exactly. GitHub's Markdown API rendered the candidate inside the live GitHub page for desktop/mobile inspection. The nine-row notation table renders its literal alternation pipe correctly. Desktop grammar fits; mobile horizontal scrolling stays inside the code block with no page overflow. Screenshots and render-check.json describe a prepublication preview, not evidence that the candidate was already published. Source highlighting and representative visible grammar lines were inspected.

The user subsequently replaced the pending Claude reviewer with Opus; earlier agent identities remain unchanged. The Opus publication audit passed; opus-final.json records the resolved model `claude-opus-5` and its scope. No K, native proof or Midnight transaction was executed for these documentation changes.

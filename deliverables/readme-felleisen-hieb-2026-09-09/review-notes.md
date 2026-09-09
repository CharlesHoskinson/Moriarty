# Final review scope and nonblocking notes

GPT-6 and Claude Opus (`claude-opus-5`, medium effort) both pass the exact README SHA-256 `65ae1b35eff79e0e0c6c57582519a140e3fa0c0907cc6ca21089c556721f62d2`. Original independent reports are retained unchanged, including their findings and stated limits.

The Opus packet supplied README's semantics section, K source, CORPUS, LICENSES, BEST-PRACTICES and two selected source excerpts. It did not include the graph or complete archive/PDF contents. Its “four full files” shorthand is imprecise: the packet has the README section and four complete supporting files. The separate GPT-6 corpus audit covers acquisition records, licenses and graph integrity.

During final acquisition checks the corpus author corrected one packaging statement after the Opus packet was sent: PLFA has an external `standard-library` submodule whose contents are not included in its source archive. CORPUS and CHECKS now explicitly record that limitation; this changes no README rule or semantic recommendation. BEST-PRACTICES retains its original inspected README hash as historical evidence; CHECKS and the independent final README audit bind the applied lessons to the final candidate.

The Redex exception example is an analogy for discarding an evaluation context, not an identity between the two languages. The README leaves codec reconstruction, complete source/K correspondence and ledger acceptance explicit. K's unchecked metadata and replay inputs remain part of the trusted codec boundary. No independent review is presented as a proof of that boundary.

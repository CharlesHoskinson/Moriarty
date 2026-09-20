# Acquisition and review record

Scope: Applications layer of APSS/CAKE for a permissionless Midnight financial language. First pass only: 12 substantive sources, ten independence groups, three discovery rounds and capture batches of five, five and two. Auxiliary requests discovered two paper URLs and pinned two repository files; they are not additional substantive sources. Public primary/official material only; no credentials, private context, paid services or vault writes.

Tools: Scrapling 0.4.15 `Fetcher.get` for every substantive source; `pdftotext -layout` for PDF navigation; installed PixelRAG `pixelshot` at 100 DPI and actual image inspection for five PDFs. Twelve PDF pages visually read; all 157 pages were rendered, not all read. PDF bytes were validated by their `%PDF-` prefix. HTML/Markdown captures were inspected for substantive content, not accepted from HTTP status alone. Two moving GitHub Markdown URLs were resolved to exact file-history commits and matched byte-for-byte.

Starting context: CAKE `ANALYSIS.md`, Moriarty vault index/hot and bounded ledger inspection. Existing material was treated as historical context, not proof of applications or permission to alter the vault. Source/claim draft manifests are separate from canonical vault ledgers.

Reproduction: `capture.py START END` fetches the selected slice with one-second spacing and updates the local manifest. `pixelshot captures/APP02.pdf ... --output pixelrag --dpi 100` renders PDFs. `write_notes.py` produces the 12 source notes, and `finish.py` builds reference/claim records and verifies hashes, visual locators and conceptual exercise arithmetic. The manually written explanation/how-to/tutorial remain distinct. Acquisition helpers overwrite local draft captures on explicit rerun; SHA-bound current artifacts should be preserved before rerunning.

Checks completed: all 12 raw capture hashes match manifest; 12 image locators exist; all five PDF captures have valid headers; CoW pinned bytes match. The tutorial arithmetic was checked independently. No product tests or financial transactions were executed.

Stop: useful support and counterexamples cover language/composition, program specification versus intent, asynchronous DeFi and privacy within the 12-source bound. Further work should inspect ACTUS and additional private application case studies, current prover ergonomics, and actual source-to-ledger behavior; it must not extrapolate complete coverage from this set.

Pattern/cookie review: existing static-document, raw GitHub and paper-PDF patterns suffice. No cookies retained and no shared skill files changed. The 2000 paper's text extraction loses glyphs, so images were used for the inspected claims. Current ERC-7683 is a resolver-based draft; stale versions must not be conflated. Manuscript dates, rather than search indexing dates, determine paper version labels.

Documentation: four Diátaxis documents plus 12 individual source annotations. The tutorial is explicitly conceptual. All outputs remain provisional research drafts for parent-managed transaction ingestion, not accepted canonical claims or release evidence.

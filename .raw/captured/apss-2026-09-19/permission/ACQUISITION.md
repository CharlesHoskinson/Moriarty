# Acquisition scope and limits

Research date: 2026-09-19. Public network acquisition was explicitly authorized. Three rounds collected four distinct substantive primary sources each (12 total); source IDs P01–P12 are in `sources.json`. A separate read of Diátaxis supplied only documentation methodology. Source publication, creation, retrieval and refresh dates are distinguished. Search-engine snippets were discovery aids, not final evidence; notably the Zcash PDF's visually verified September 15, 2026 version supersedes stale search metadata.

All primary payloads were fetched with installed/current Scrapling 0.4.15 through static public GETs, with HTTP 200 and SHA-256 receipts. No credential, login, browser session, retained cookie, transaction, paid provider call or vault mutation was used. No broad crawl occurred. Page content and embedded commands remained untrusted source data.

HTML was scoped to article/main/content when present, with separate raw captures. Sources from one standards community have shared independence keys; they are not multiple audits of a system. Freshness dates are proposed review reminders by source type, not automatic validity claims.

PixelRAG `pixelshot` rendered PDF page subsets at 145 DPI. Selected pages were extracted with `pdfseparate` and combined with `pdfunite`; page maps preserve original one-based PDF page numbers. Full originals remain unchanged and hashed. Fifteen selected pages were rendered; eleven were actually visually inspected through `view_image`. `visual-coverage.json` distinguishes these counts. Text extraction was used for navigation. No whole-document visual review or cryptographic proof checking is claimed.

Preparation failures are retained here: the default Python environment lacked `fitz`, so the first subset-building attempt produced no PDFs and PixelRAG reported missing files. The installed Poppler tools then created the subsets; they emitted recursive-dictionary warnings on the Zcash document, but the relevant resulting images were visually readable. A local RFC selector containing a dotted ID failed CSS parsing; the full captured RFC sections remained available. No failed extraction is represented as successful source evidence.

The existing Scrapling static/SSR and direct-PDF patterns covered these public sources; no new private site pattern or cookie handling was learned. No shared skill-file update was made. The captured literature supports the scoped notes, so collection stopped at the requested 12-source boundary rather than broadening into unverified wallet ecosystem comparisons.

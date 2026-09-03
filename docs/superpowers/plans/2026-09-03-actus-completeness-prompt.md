# ACTUS completeness prompt implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use
> superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** Revise the focused Moriarty XML assignment so all public ACTUS
reference fixtures become the terminal financial-contract compatibility test.

**Architecture:** Preserve the complete public ACTUS documentation corpus and
pin the public repositories. Add one ACTUS workstream to the existing research
assignment. Keep contract types in typed packages unless evidence requires a
Core semantic motion.

**Tech Stack:** Python 3, Scrapling 0.4.15, XML, JSON, pytest, Git

## Global Constraints

- Keep semantic scope `0.0.0-e00.2` unchanged.
- Require all 276 contract fixtures and the one analysis-date fixture.
- Classify all 32 taxonomy rows without claiming that all rows are implemented.
- Treat the private Java `actus-core` as an optional oracle.
- Use ACTUS reference-vector compatibility language.
- Do not claim ACTUS certification, endorsement, or conformance.
- Preserve the user's existing change to `evidence/marlowe-org-full-graph-2026-09-02.json`.

---

### Task 1: Preserve the ACTUS source corpus

**Files:**

- Create: `scripts/acquire_actus_public_sources.py`
- Create: `.gitattributes`
- Create: `raw/sources/actus-public-2026-09-03/**`
- Create: `evidence/actus-public-source-acquisition-2026-09-03.json`

**Interfaces:**

- Consumes: ACTUS website sitemaps and `documentation.actusfrf.org/sitemap.xml`.
- Produces: Raw responses, AI-targeted Markdown, adjacent receipts, and one coverage manifest.

- [x] **Step 1: Implement deterministic sitemap acquisition**

  Use `scrapling.fetchers.Fetcher` for every HTTP request. Save the response
  body before extraction. Use `scrapling.core.shell.Convertor` with
  `main_content_only=True` for HTML pages.

- [x] **Step 2: Preserve the documentation-host correction**

  Keep every original `https://your-docusaurus-site.example.com/...` URL in
  the receipt. Replace only its origin with
  `https://documentation.actusfrf.org` for retrieval.

- [x] **Step 3: Acquire all discovered pages**

  Run:

  ```bash
  python3 scripts/acquire_actus_public_sources.py
  ```

  Expected: the manifest accounts for every main-site and documentation-site
  sitemap entry. Each entry has one successful receipt or one explicit failure.

- [x] **Step 4: Verify the manifest**

  Run:

  ```bash
  jq '.coverage, .limitations' evidence/actus-public-source-acquisition-2026-09-03.json
  ```

  Expected: documentation entry count `220`, placeholder rewrite count `220`,
  and no entry without a disposition.

### Task 2: Add a failing prompt contract test

**Files:**

- Create: `tests/test_semantics_intent_prompt.py`

**Interfaces:**

- Consumes: the focused XML assignment.
- Produces: structural and semantic regression checks for prompt version 1.3.

- [x] **Step 1: Write assertions before editing the XML**

  Assert well-formed XML, version `1.3`, one ACTUS workstream, all 18
  executable contract-type acronyms, the exact fixture split, all 32 taxonomy
  rows, optional Java-core language, compatibility language, and no fixture
  exclusion permission.

- [x] **Step 2: Run the test and confirm failure**

  Run:

  ```bash
  uv run pytest -q tests/test_semantics_intent_prompt.py
  ```

  Expected: failure because prompt version 1.2 has no ACTUS profile.

### Task 3: Revise the XML assignment

**Files:**

- Modify: `deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml`

**Interfaces:**

- Consumes: the approved ACTUS design and preserved source evidence.
- Produces: prompt version 1.3 with a falsifiable ACTUS completion predicate.

- [x] **Step 1: Add ACTUS sources and authority rules**

  Add the public website, documentation, dictionary, technical specification,
  tests, service repositories, and Marlowe implementations. Require exact
  revisions and digests. Require contradictions to remain visible.

- [x] **Step 2: Add the ACTUS workstream and data contracts**

  Require package implementations for PAM, LAM, LAX, NAM, ANN, CLM, UMP, CSH,
  STK, COM, FXOUT, SWPPV, SWAPS, CAPFL, OPTNS, FUTUR, CEG, and CEC. Add source
  lock, fixture, observation, expected-event, comparison, coverage, taxonomy,
  and license records.

- [x] **Step 3: Add experiments and release gates**

  Require exact discovery of all 277 fixtures. Compare every supplied trace
  field without binary32 conversion. Run two independent semantics and the
  generated Compact backend. Add negative and metamorphic mutations.

- [x] **Step 4: Add claim and license restrictions**

  Keep Java-core access optional. Prohibit access-control bypasses, fixture
  exclusions, coverage inflation, and unauthorized conformance claims.

- [x] **Step 5: Run the focused test**

  Run:

  ```bash
  uv run pytest -q tests/test_semantics_intent_prompt.py
  ```

  Expected: pass.

### Task 4: Update durable research records

**Files:**

- Modify: `evidence/source-inventory.csv`
- Create: `evidence/actus-public-code-survey-2026-09-03.json`
- Modify: `wiki/research-program.md`
- Modify: `wiki/research-journal.md`
- Modify: `wiki/contradictions.md`
- Modify: `wiki/index.md`
- Modify: `wiki/log.md`

**Interfaces:**

- Consumes: the acquisition manifest and prompt version 1.3.
- Produces: source IDs, claims, contradictions, index descriptions, and a query log entry.

- [x] **Step 1: Add ACTUS source records**

  Add stable source identifiers for the public ACTUS corpus, code survey, and
  prompt version 1.3. Record exact commits and artifact digests.

- [x] **Step 2: Add prompt iteration P04**

  Record the 32/18/277 scope distinction, Java-core access boundary, Marlowe
  coverage gap, completion predicate, and unchanged semantic scope.

- [x] **Step 3: Record contradictions**

  Record taxonomy versus executable-type scope, the placeholder sitemap host,
  source-version drift, and public versus access-controlled code.

- [x] **Step 4: Update the index and log**

  Describe the prompt as an ACTUS-gated assignment. Add a dated query record.

### Task 5: Verify and commit

**Files:**

- Verify: all files above

**Interfaces:**

- Consumes: the completed source and prompt changes.
- Produces: a tested commit that excludes the user's unrelated graph change.

- [x] **Step 1: Validate XML and content**

  Run:

  ```bash
  python3 -c 'import xml.etree.ElementTree as ET; ET.parse("deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml")'
  uv run pytest -q tests/test_semantics_intent_prompt.py
  ```

  Expected: XML parses and the focused tests pass.

- [x] **Step 2: Run the repository suite**

  Run:

  ```bash
  uv run pytest -q
  ```

  Expected: all tests pass.

- [x] **Step 3: Inspect the staged change**

  Run:

  ```bash
  git diff --check
  git status --short
  ```

  Expected: no whitespace errors. Do not stage
  `evidence/marlowe-org-full-graph-2026-09-02.json`.

- [x] **Step 4: Commit the implementation**

  Run:

  ```bash
  git add .gitattributes scripts/acquire_actus_public_sources.py raw/sources/actus-public-2026-09-03 evidence/actus-public-source-acquisition-2026-09-03.json evidence/actus-public-code-survey-2026-09-03.json evidence/source-inventory.csv tests/test_semantics_intent_prompt.py deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml wiki/research-program.md wiki/research-journal.md wiki/contradictions.md wiki/index.md wiki/log.md docs/superpowers/plans/2026-09-03-actus-completeness-prompt.md docs/superpowers/specs/2026-09-03-moriarty-semantics-intent-compiler-sdk-prompt-design.md
  git commit -m "docs: gate Moriarty with ACTUS vectors"
  ```

  Expected: one commit containing only the ACTUS prompt and evidence change.

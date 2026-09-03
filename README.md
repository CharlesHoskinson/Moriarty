# Moriarty

Moriarty is the proposed bounded financial-agreement language for a Compact/ZKIR
DeFi kernel. This repository is its evidence base, working wiki, experiment
suite, and decision packet. The September 2, 2026 Marlowe V2 modernization study
is the principal design and compatibility input.

The project uses a Karpathy-style LLM wiki: immutable material in `raw/` feeds
maintained, interlinked synthesis in `wiki/`. Evidence inventories, repository
locks, and reproducibility records live in `evidence/`; executable studies live
in `experiments/`; decision-grade outputs live in `deliverables/`.

Start with:

- [`deliverables/moriarty-decision-study-2026-09-02.md`](deliverables/moriarty-decision-study-2026-09-02.md)
  for the full decision study;
- [`deliverables/moriarty-stakeholder-preread.md`](deliverables/moriarty-stakeholder-preread.md)
  for the ten-page-max workshop packet;
- [`WIKI_SCHEMA.md`](WIKI_SCHEMA.md) for evidence and maintenance rules;
- [`wiki/index.md`](wiki/index.md) for accumulated knowledge;
- [`graphs/marlowe-org-full/graphify-out/graph.html`](graphs/marlowe-org-full/graphify-out/graph.html)
  for the aggregated interactive Marlowe graph;
- [`deliverables/moriarty-defiformal-category-strawmen.md`](deliverables/moriarty-defiformal-category-strawmen.md)
  for seven primary family demonstrations and 13 migration patterns covering
  the 12 legacy DeFiFormal areas;
- [`deliverables/moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml`](deliverables/moriarty-defi-kernel-deep-research-prompt-2026-09-03.xml)
  for the executable deep-research loop and decision gates;
- [`deliverables/moriarty-agentic-council-record-2026-09-03.md`](deliverables/moriarty-agentic-council-record-2026-09-03.md)
  for the Grok, Fable 5.1, and GPT-5.6 Sol advisory synthesis and dissent;
- [`raw/assignments/modernizing-marlowe-assignment-2026-09-02.xml`](raw/assignments/modernizing-marlowe-assignment-2026-09-02.xml)
  for the controlling assignment;
- [`evidence/source-inventory.csv`](evidence/source-inventory.csv) for source
  acquisition status.

The Python environment is managed with `uv`. Scrapling and its HTTP, browser,
stealth, and AI-targeted extraction dependencies are locked in `uv.lock`.

```bash
uv sync
uv run python -c "import scrapling; print(scrapling.__version__)"
```

No report claim is authoritative merely because it appears in the wiki. Every
material claim must retain its source, scope, status, reproduction state, and
confidence.

# Instructions for researchers 4–10

The user explicitly requested ten agents to deeply search the public web with Scrapling, download UTXO/eUTXO papers, repos and standards, and assess Moriarty on a UTXO ledger. You are one of those agents. Do not spawn additional agents. Complete a focused 8–10 minute research pass and return.

Read RESEARCH-CONTRACT.md, MORIARTY-CONTEXT.md (including its critical September11 amendment) and LOCAL-ARCHITECTURE.md in this directory. Mandatory Moriarty startup: read `/home/charl/Moriarty/AGENTS.md` and `/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.0+codex.20260912173932/skills/develop/SKILL.md`, then run `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` from `/home/charl/Moriarty`. Read `wiki/index.md` and only relevant current code/notes. No Moriarty source/wiki/roadmap edits.

Read `/home/charl/.agents/skills/scrapling/SKILL.md`, `references/security.md` and relevant `references/site-patterns.md`. Parent has verified installed Scrapling0.4.15 against current PyPI and completed a successful public-page smoke. No installation needed. Use actual `web__run` for discovery: at most3 rounds, up to4 queries per round. Download up to15 relevant public sources and up to3 public repo snapshots in your assigned directory. Read essential downloaded evidence, not only snippets. Original papers, maintained author repositories and normative docs take precedence over tutorials or marketing.

Helper:

```
python3 /home/charl/moriarty-utxo-research-2026-10-12/acquire.py --dir YOUR_DIR fetch HTTPS_URL UNIQUE_SLUG
python3 /home/charl/moriarty-utxo-research-2026-10-12/acquire.py --dir YOUR_DIR repo HTTPS_GIT_URL UNIQUE_SLUG
```

It stores exact source bytes, hashes, text extracts, robots observations and immutable receipts. PDFs are checked by magic bytes and extracted using pdftotext. Git snapshots record remote/branch/commit and are shallow without submodules. Never execute downloaded source code or install its dependencies. Existing receipts cannot be overwritten. Failures/disallowed paths remain evidence; use public alternate author sources without bypassing access controls. Check final command outputs, do not claim a download from an unfinished session. Avoid serial 45sec retries for a dead domain.

Write REPORT.md and sources.json in your directory. Report primary findings with URLs AND exact artifact/path/section locators, standards activation status, source/implementation distinction, Moriarty compatibility inference, concrete barriers/counterevidence, and specified-only experiment recommendations. Cite minimal quotations; preserve uncertainty. Record publication dates only when verified, retrieval dates, title/authors/publisher, authority, independence_key, receipt path and access failures. Repositories cloned are not codebases fully reviewed; say what files were inspected. Never conflate ordinary Plutus validation with zero-knowledge proofs or preserved privacy. Cardano history consumption does not automatically discharge liability correctness, oracle truth, liveness or private handoff.

Final message to parent: main findings, count of papers/docs/repos, report path, gaps and concrete next experiment. Source facts, repository observations and inference stay distinct. No implementation or ledger submissions are requested.

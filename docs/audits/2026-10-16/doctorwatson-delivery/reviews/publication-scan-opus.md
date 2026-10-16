# DoctorWatson delivery: pre-push credential scan

**Verdict: ALLOW_PUBLICATION.** None of the 96 non-gzip files contain credentials, private keys, tokens, API keys, wallet keys or seed phrases, or dumped environment variables.

## Scope
- **Directory:** `/home/charl/Moriarty/.worktrees/sherlock-runtime-observations-20261016/docs/audits/2026-10-16/doctorwatson-delivery/` only. Nothing else was read.
- **Read-only:** no file edits, git mutations, network, installs, or SDK/native/K/Quint/Lean/Clean actions. Git was used only for `ls-files`, `status`, `diff --quiet`, `ls-tree` and `cat-file -t`.
- **Files:** 170 on disk. I scanned all 96 non-gzip files: 34 json, 14 stderr, 10 stdout, 7 log, 8 md and 5 `.d`, including 14 empty files. I skipped the 74 `.gz` files as you allowed.
- **Named captures:** design-01/02, ordinary-01/02, entrypoint-tests, entrypoint-host-tests, stop-counters, performance-02, host-GREEN and host-GREEN-outer were all covered, and I read their content directly as well as pattern-matching it.

## Files changed during the scan
- **New files:** 6 files appeared at 09:31:05, after my first listing: `REVIEW-RESULTS.md`, `reviews/FINAL-CANDIDATE.json`, `reviews/final-{muse,opus}.md` and `reviews/final-{muse,opus}-identity.json`. I re-ran every pattern over all 96 files, so they are covered.
- **Index vs candidate:** between two of my git checks, another process staged those 6 files. The index now has 165 entries: the 159 entries of candidate tree `f91045548545…` exactly, plus those 6. The working tree matches the index, so the bytes I scanned are what would be pushed.
- **Who integrates them:** I have not judged whether the 6 new files belong in the publication. That's the integrator's call, but they are clean.
- **Not pushed:** the three `host-*/build/` directories (5 `.d` files) are gitignored. I scanned them anyway and they are clean.

## Commands run
1. `ls`/`find`/`file`: listed files, counted by extension and folder, confirmed every non-gzip file is text.
2. `rg -c` for high-confidence patterns (no matches, both runs):
   - PEM private keys and certificates
   - AWS `AKIA`/`ASIA` keys
   - GitHub, OpenAI and Anthropic `sk-`, Slack, Google, GitLab and npm tokens
   - Bearer tokens and JWTs, and URLs with `user:pass@`
   - `xprv`/`tprv` extended keys and Cardano `*_sk`/`*_xsk` keys
3. `rg -P` keyword scan: password, secret, api key, private key, mnemonic, seed phrase, credential, authorization, cookie, session id, provider environment variable names, wallet, `.ssh`, `.aws`, `.netrc`, `.env`, `.skey`. I then looked at every hit.
4. `python3 -I` entropy scans for long base64-like or mixed-case strings. I printed only 4–6 character prefixes, never full values.
5. `rg` for environment dumps (`HOME=`, `USER=`, `SSH_*`, `declare -x`), email addresses, IPv4 addresses, `set -x` variable assignments, and `export`/`env`/`curl`/`ssh`/`gh`/`git push` lines.
6. `python3 -I`: printed every environment block in the receipts and result files, and the key structure of the review identity and usage JSON files.
7. Read-only git checks: index vs candidate tree, and working tree vs index.

## What the matches turned out to be
- **"token":** 24 are model usage counters (`inputTokens`, `thinkingTokens`, etc.). The rest are prose about Quint `jobToken`/token equality.
- **"private-key":** one sentence in `reviews/preflight-muse.md:14` saying no private-key action is approved.
- **"secret":** prose in `reviews/final-opus.md`, including the line that asked for this scan.
- **"seed":** all are Quint RNG seeds (`--seed=1`, `--seed=0x251664`), not wallet seeds.
- **Environment blocks:** only an allowlist. Child processes get `LANG`, `PATH=/usr/bin:/bin` and `TMPDIR`; the outer runs get `DOCTORWATSON_OUTPUT_DIR` and `DOCTORWATSON_RUSTC`, both local paths. The `set -x` traces only set `design`, `host`, `repo_root`, `node_*`, `quint_*` and `PYTHONDONTWRITEBYTECODE`, and the tools run under `env -i`, which starts them with an empty environment.
- **High-entropy strings:** the only ones that aren't hex hashes are the filename `03-stop-counters-GREEN` (5 hits). Everything else is sha256/sha1 digests or file paths.
- **Emails, IPs, `.ssh`/`.aws`/`.netrc`, environment dumps:** none.
- **For your information only (not a blocker):** the muse identity files contain random run/command IDs in UUID format and labels for the model, profile and provider. They don't grant access to anything. They are tool-run metadata, of the same kind as the usage files that were already in the commit.

## Result
**ALLOW_PUBLICATION.** This scan doesn't change or overrule the scope of the earlier audit.

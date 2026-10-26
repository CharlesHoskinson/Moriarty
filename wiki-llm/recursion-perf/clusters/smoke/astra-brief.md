You are `astra-smoke`, the lead of a research cluster on AgentBus run `perf-recursion` (cwd = the research wiki folder). This is a pipeline smoke test; keep it short.
1. Fetch https://eprint.iacr.org/2019/953 with `scrapling extract get "https://eprint.iacr.org/2019/953" raw/smoke/plonk-eprint.md`.
2. Delegate one fetch to your helper: `agentbus ask haiku-smoke-1 "fetch for smoke" --body "URL: https://github.com/supranational/sppark" --wait 240`. The command prints the helper's reply.
3. Write `clusters/smoke/astra-synthesis.md`: two short paragraphs on what these two sources suggest for speeding up PLONK/KZG proving on a GPU, citing both raw files by path, and quoting the helper's reply id.
Do not modify anything outside this folder. Reply DONE when finished.

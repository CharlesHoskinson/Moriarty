# Acquisition notes and limitations
> Source: conversation://astra-s6-opening/wave-1/tool-log
> Collected: 2026-10-26
> Published: 2026-10-26

SOURCE — The supplied ../../wiki-llm/loki/10x/loki-04-halo2-gpu-precedents.md path did not exist. The existing prior study was read at ../loki/nodes/loki-04-halo2-gpu-precedents.md and archived in prior-loki04.md.

SOURCE — The initially delegated privacy-scaling-explorations Halo2 main SHPLONK URL redirected to privacy-ethereum and returned a file-not-found page. It is retained at shplonk-prover.md. Algorithm analysis instead uses the pinned Axiom GPU source at b2d4bcfd4261cc506b5412fb04fe631f028849d7.

SOURCE — A supplementary urllib request for https://eprint.iacr.org/2021/1167.pdf returned HTTP 403. The compound command stopped before fetching the FK20 PDF; no full-paper PDF evidence was claimed. Landing pages for 2021/1167, 2020/081, 2023/033 and 2024/1279 were successfully fetched with scrapling. Hypothesis: the PDF endpoint treats this client differently from scrapling's landing-page client. No installation or repeated bypass attempts were made.

SOURCE — Scrapling's Markdown extraction of raw source text stripped some generic type syntax. For the source modules used in the analysis, exact public raw HTTP responses or in-memory published crate archive members replaced the damaged body, retaining the required provenance header. For the PR565 files API the final body is faithful JSON, including escaped patch newlines. No crate archive was unpacked to the workspace or installed.

SOURCE — The owner supplied the target hardware. No new hardware or proving performance measurement was made. Only the deterministic Python field-algebra experiment was executed. AgentBus helpers supplied fetches and summaries; summaries were checked against source text before use.

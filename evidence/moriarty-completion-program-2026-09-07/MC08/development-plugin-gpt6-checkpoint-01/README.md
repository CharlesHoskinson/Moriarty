# GPT-6 reservation checkpoint

GPT-6 implemented the reservation correction. A separate GPT-6 reviewer passed this narrow checkpoint. The source snapshot remains an unfinished plugin candidate.

A primary reservation now covers all linked Git worktrees. A dead parent does not release it: its child or resource charge may survive. Recorded completion still permits the next launch.

Three regression cases failed before the correction and passed afterward. Root ran the full 127-test suite successfully. Independent review ran four focused regressions and a twelve-process race: exactly one process acquired the primary slot. Rejected retries and history reads preserved stored state.

See `review.json` for the precise scope and limits. Runner authentication, durable charging, authenticated transaction delivery, installation, host interception and the product repair pilot remain open. The current CLI still denies execution. This checkpoint does not accept the whole plugin or any language/network milestone. No blockchain transaction was submitted.

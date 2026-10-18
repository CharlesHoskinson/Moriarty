---
id: source.zk-language-corpus
type: source
title: ZK language corpus
status: active
updated_at: 2026-09-13T06:00:00Z
sources:
  - SRC-0114
  - SRC-0115
  - SRC-0116
  - SRC-0117
  - SRC-0118
  - SRC-0119
created: 2026-09-13
updated: 2026-09-13
tags:
  - moriarty
  - research
  - language
---

# ZK language corpus

Seven repositories cloned to `~/zk-langs` on 2026-09-13 as primary sources for
surface-language design. All findings in [[wiki/language/zk-language-survey|the survey]]
and [[wiki/language/jet-discipline|the jet analysis]] cite these by identifier.

| Id | Repository | Role in the survey |
| --- | --- | --- |
| SRC-0114 | `BlockstreamResearch/simplicity` | Nine-combinator verified core, Coq formalisation, the jet mechanism |
| SRC-0115 | `BlockstreamResearch/SimplicityHL` | The high-level sugar layer over SRC-0114 |
| SRC-0116 | `o1-labs/o1js` | Mina's decorator-based TypeScript DSL |
| SRC-0117 | `starkware-libs/cairo` | Starknet's attribute-driven Rust-like language |
| SRC-0118 | `noir-lang/noir` | Aztec's privacy-annotated circuit language |
| SRC-0119 | `matter-labs/zksync-era` | The reuse-Solidity counterexample |

**CLM-0961 — Method.** Every claim in the dependent notes was taken from
compiler or formalisation source in these repositories, not from published
documentation. Where a repository does not settle a question the dependent note
records that rather than substituting general knowledge (CLM-0961; this note;
method; reproduced; high; S4).

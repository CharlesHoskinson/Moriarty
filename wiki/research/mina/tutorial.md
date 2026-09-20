---
title: "Paper tutorial — a valid proof under the wrong key"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: tutorial
tags: [moriarty, consolidation, recursion]
---

# Paper tutorial — a valid proof under the wrong key

This is a paper exercise, not an executed exploit.

1. An owner signs a policy requiring a financial relation R.
2. A solver supplies a valid proof for trivial relation T and a key whose data matches its hash.
3. Key integrity passes, but the program must reject T because the signed policy authorizes R.
4. Replace T with R, then alter an auxiliary recipient returned next to the proof. The consumer must use the bound statement/commitment, not unproved adjacent data.
5. Restore the recipient, then reuse a valid old-state proof after another transition has consumed that state. Current-state acceptance must reject the replay.
6. Finally verify the correct current successor with all deferred checks and real proof mode. Only this supported path can support the claimed effect.

[Explanation](explanation.md) · [Index](index.md).


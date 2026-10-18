# Sherlock Top-of-Page Editorial & Information Architecture Audit (`top_prose_03_editorial_header.md`)

## 1. Why the Old Top Subtitle Fails
In `/home/charles/sherlock-bench/index.html`, the page opened with:
> `Choose a program. Follow its calls, inspect the saved state, and see how the result is computed.`

It failed because:
1. It describes generic UI widget mechanics rather than the machine on screen.
2. It omits the core problem Sherlock exists to solve (verifying recursive programs without unrolling custom circuits or trusting a hidden hardware call stack).
3. It leaves the 3D hollow die, the 6 split-arc rings (`R1..R6`), the 10-sector central control dial, and the fuel rim unexplained above the fold.

## 2. Recommended Top-of-Page Copy: Masthead + 3-Card Plain-English Primer Strip

### Masthead Copy
- **Brandmark / Kicker (`.brandmark`):** `SHERLOCK · MORIARTY EXECUTION ENGINE`
- **Headline (`<h1 class="subtitle">`):** `A glass-box machine that runs recursive programs one verifiable step at a time.`
- **Lead Paragraph (`<p class="deck">`):** `Most computers run a recursive function by hiding unfinished work on an internal call stack until the final answer pops out. Sherlock puts every step on the table: it runs any program in its language through one fixed set of simple rules, writing every paused calculation, variable list, and fuel count into a flat snapshot that can be checked one move at a time.`

### 3-Column Top Primer Strip
1. **`01 · No hidden scratchpad` — `Every paused calculation sits out in the open.`**
   When `fact(4)` needs `fact(3)` before it can multiply by `4`, a normal processor hides that unfinished work inside hardware stack memory. Sherlock has no hidden stack and no changeable memory cells. Instead, it writes a visible reminder card—`4 × [?]`—alongside a snapshot of the caller’s variables, and stacks that frame in plain sight while it works on `fact(3)`.
2. **`02 · One rule at a time` — `Big recursive runs become small, checkable steps.`**
   Proving that a recursive program ran honestly is hard if you have to swallow the whole run at once. Sherlock breaks execution into single-rule ticks: each step reads the current snapshot, applies one numbered rule, produces the next snapshot, and burns `1` unit of fuel. You can check any single step on its own, and runaway programs stop cleanly when fuel or frame space runs out.
3. **`03 · Reading the rings` — `Watch calls stack inward and answers unwind outward.`**
   The six concentric rings on the chip (`R1` on the outside to `R6` on the inside) hold up to 12 saved frames—two per ring (`Slot A` on the left, `Slot B` on the right). As a program calls itself deeper, frames fill the rings from the outside in. Once it hits a base case, answers travel back out, solving `1 × [?]`, `2 × [?]`, `3 × [?]`, and `4 × [?]` until the rings clear.

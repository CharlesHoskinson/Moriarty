# Missing recovery language transition: design candidate

Read [DESIGN.md](DESIGN.md), [worked examples](EXAMPLES.md), [proposed EARS](PROPOSED-EARS.md) and [independent custody/review gates](CUSTODY-AND-REVIEW.md).

The recommendation is a reusable source-declared finite recovery arm attached to an Await, with a typed operation on its actual pending frame. A separate proposed `moriarty/3` / `moriarty-core/3` profile makes changed lifecycle, accounting and output semantics explicit while preserving existing `/1` and `/2` contracts. Successful recovery closes only the returned local normal path, with complete residual duties and cumulative effects/fees/work retained. Controlled return and new forward compensation have separate source/report modes and explicit financial bodies. Timeout alone can leave the guard false and the duty pending.

This is a proposal against exact merged main `7396d7d316ab72957bca9709483f4638daa0d5cb`, not implementation. The complete public source inputs and retained cancellation gap are pinned in [SOURCE-PINS.json](SOURCE-PINS.json). Existing normal multi-condition tests supply no cancellation result.

Two substantive agreeing non-author design votes and designated security-owner review are required before implementation. Neither is present. Authentic control/authority/outcome evidence, replay/consumption/races, history/cap/identity binding and native/ledger/proof decisions remain explicitly owner-unassessed. Local output remains authentication:none and consumption:untracked. No provider or runtime/model/SDK/native operation was dispatched; no working checkout or Git state was mutated. No requirement or product gate is closed.

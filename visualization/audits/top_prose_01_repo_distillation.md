# Sherlock Purpose & Plain-English Distillation (Agent 1 of 3)

**Sources Reviewed:**
- `/home/charles/repos/Moriarty/openspec/changes/sherlock-universal-interpreter-design/design.md`
- `/home/charles/repos/Moriarty/openspec/changes/sherlock-universal-interpreter-design/proposal.md`
- `/home/charles/repos/Moriarty/docs/SHERLOCK-REFERENCE-STATUS.md`
- `/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/reference_types.rs`
- `/home/charles/repos/Moriarty/crates/mori-core/src/sherlock/reference.rs`

---

## 1. What Sherlock Actually Is and Why Moriarty Built It

On a normal computer, when a function calls itself (like calculating `factorial(4)` by asking for `factorial(3)`), the processor hides its place on an internal hardware call stack and jumps back when the inner call finishes. In zero-knowledge proofs and verifiable smart contracts—the world Moriarty targets—that approach breaks down for two reasons:
1. **Circuits cannot loop indefinitely or rely on hidden hardware state.** A cryptographic circuit is a fixed mathematical grid. If a compiler tries to turn a recursive function directly into a custom circuit by unrolling every call ahead of time, it either has to guess the maximum recursion depth at compile time or build a bespoke, sprawling circuit for every new program.
2. **Every step of a verifiable computation must be countable, bounded, and auditable.** When a contract runs on a ledger, nothing can be hidden inside a host CPU's call stack or native memory heap. Every pending calculation, variable binding, and remaining budget must sit out in the open as plain data so that one step can be checked against the next.

Moriarty built **Sherlock** to solve this with **one small, fixed-rule interpreter** (`sherlock-k-a/1` in `reference.rs`). Instead of baking each recursive program into its own custom circuit or trusting a hidden hardware stack, Sherlock treats the program itself as an immutable lookup table and runs it one tiny, deterministic step at a time.

## 2. Three Non-Technical Top-of-Page Prose Drafts

### Draft A — The "Waiting Notes on an Open Table" Explanation (Concrete & Everyday)
- **Subtitle**: How Moriarty runs recursive programs without a hidden call stack
- **Deck**: Normal computers hide unfinished function calls inside hardware memory, which makes recursive programs hard to verify in a cryptographic proof. Sherlock is a tiny, step-by-step calculator that puts every piece of waiting work out in the open: when it needs to solve `4 × factorial(3)`, it sets a `4 × [?]` note on a visible tray, works out `factorial(3)`, and plugs the answer back into the blank—paying one unit of fuel per step under a strict limit on how high those waiting notes can stack.

### Draft B — Why Moriarty Built It (The "One Small Machine" Angle)
- **Subtitle**: Moriarty’s step-by-step engine for verifiable recursion
- **Deck**: Instead of building a separate custom circuit for every program that calls itself, Moriarty uses one small machine with fixed rules. Sherlock breaks any recursive calculation into single steps that each cost 1 unit of fuel. Whenever a step has to wait on another call to finish, Sherlock saves a note with the unfinished math and current variables on a bounded tray—so every step, saved variable, and remaining budget can be checked in plain sight.

### Draft C — Ultra-Compact Masthead (Short & Punchy)
- **Subtitle**: Moriarty’s open-state calculator for recursive programs
- **Deck**: To make recursive code provable step by step, Sherlock replaces the hidden hardware call stack with an open tray of waiting notes (`4 × [?]`). Every step burns 1 unit of fuel, every waiting call takes 1 slot on the tray, and when the inner answer comes back, Sherlock plugs it into the blank until the final result is reached.

# Sherlock Plain-Language Visual Metaphor & Chip Guide (Agent 2 of 3)

## 1. How to Explain What Sherlock Does in 60 Seconds

- **How ordinary computers handle recursion:** When a program calls itself—like computing `factorial(4)` by pausing `4 × …` to go compute `factorial(3)`—a conventional CPU hides that unfinished work inside a private call stack of memory return addresses and mutable scratch space.
- **How Sherlock handles recursion:** Sherlock has **no hidden call stack, no memory pointers, and no mutable scratch cells**. Every time a calculation pauses to wait for a sub-calculation, Sherlock writes down the unfinished job as a self-contained waiting note on an open shelf—for example: *"Hold the caller's variables"* (`Return`) and *"When an answer comes back, multiply 4 by it: `4 × [?]`"* (`Right`).
- **Why that matters:** Because the entire state of a running program—what it is doing right now, its current variables, its shelf of waiting work, and its remaining fuel—is laid out in the open on every step, a cryptographic proof can verify any recursive program **one small, explicit step at a time**: *"Did Step 14 legally turn into Step 15?"*

## 2. Proposed Top-of-Page Copy & 3-Column Introductory Strip

### Top Header (`.masthead`)
- **Headline (`.subtitle`):** `See how a computer solves recursion when nothing is allowed to hide in memory.`
- **Deck (`.deck`):** `Sherlock is Moriarty’s universal recursion interpreter. Instead of burying unfinished calls in a hidden hardware stack, it keeps every paused calculation out in the open—so a cryptographic proof can verify execution one step at a time.`

### Three-Column Introductory Strip (Above `.workbench-grid`)
1. **`01 · What Sherlock Does` — `Recursion without hidden memory`**  
   When a normal computer runs a recursive call like `fact(4)`, it hides unfinished work in a private stack of memory addresses. Sherlock keeps everything in the open: every paused calculation—such as *“multiply 4 by whatever answer comes back”*—is placed on the shelf as an explicit piece of waiting work, ready to be checked one step at a time.
2. **`02 · How to Read the Chip` — `Center dial & six split rings`**  
   The **center dial** lights the exact action taken on this step (reading code, shelving work, calling a function, or returning an answer). Around it, the **six split rings** hold waiting work from the outside in: **West** saves the caller’s variables (Teal), and **East** holds paused math like `4 × [?]` (Gold). When the deepest call finishes, the rings clear from the inside out.
3. **`03 · Built-In Safety Bounds` — `The fuel rim & shelf limit`**  
   Every step burns one unit of fuel from the **outer rim**, and the shelf only holds a fixed number of waiting cards. Try lowering the **Initial fuel** or **Frame limit** sliders on the left: a program can never loop forever or overflow memory silently—it always halts with a verifiable `Done`, `Exhausted`, or `Rejected` outcome.

# Sherlock top-of-page copy

Paste-ready text for the masthead and the three cards above the workbench. The audit under it is not page copy.

Brandmark stays `Sherlock`. No second kicker.

## Masthead headline

Recursive programs, with the unfinished work saved in the machine.

## Lead paragraph

Sherlock is Moriarty’s interpreter for programs whose functions call themselves or each other. A normal run leaves unfinished calls on the computer’s call stack. Sherlock stores that waiting work in its own state. When `fact(4)` has to wait for `fact(3)`, the caller’s variables and the unfinished `4 × [?]` are separate frames. Every program the language accepts uses these same rules, so a proof checks this interpreter instead of a custom encoding of each source program. This page runs the reference interpreter, profile `sherlock-k-a/1`. Pick a program and step through it.

## 3-card primer strip

### 01 · What is saved

**The caller’s variables and the pending multiplication are different frames.**

`fact(4)` cannot multiply by 4 until `fact(3)` returns. One frame holds the caller’s variables. Another holds `4 × [?]`. That 4 stays put until the inner result arrives. An `if` test, a `match`, a `let`, or a list of arguments still being evaluated is a frame too. The number of saved frames is the depth.

### 02 · The six rings

**The outside ring is the oldest work. Each ring holds two frames.**

The rings run from `R1` on the outside to `R6` on the inside. On each ring the left slot is the older frame and the right slot is the newer one. Added frames appear toward the center. When a frame is removed, the newest slot clears. The rings draw twelve frames at most. A `+N` mark counts frames past that window. Colour is the type of frame. Teal is a saved caller. Cyan is a `let` scope waiting to be restored. Gold is a stored operand, such as `4 × [?]`. Amber is a left operand, a unary operator, or a call’s arguments still in progress. Violet is an open `if`, `match`, or `let`.

### 03 · Dial, rim, and limits

**The lit sector is this step. The outer rim is fuel still left.**

The dial has ten sectors, the controls of `sherlock-k-a/1`: Eval, Push, Return, Invoke, Enter, Bound, Cases, Match, Prim, and Error. The lit sector is the kind of work at the step on screen. Return on the dial means a value is coming back into the computation. It is not the teal frame that saved the caller. The needle points left or right, to the side of the newest frame still drawn. It does not select a ring. The segmented rim is remaining fuel, not another ring of frames. A charged rule spends one unit. Publishing `Done`, `Exhausted`, or `Rejected` spends none. Lookup and matching inside a rule are not charged separately. `Exhausted` means the fuel ran out. `Rejected` means the step was refused, which includes a push past the frame-limit slider. `Done` means the run produced a value.

---

## Understandability

A first-time reader has three questions: what Sherlock is, why Moriarty uses it, and how to read the chip. None of the three drafts answers all three without teaching something false.

| Question | Draft 1 | Draft 2 | Draft 3 |
|---|---|---|---|
| What is it? | A “calculator” or a “small machine.” The shared-rules idea is the right one, then buried under trays and notes. | Names the interpreter, then spends the definition on what memory is forbidden to do. | Opens on a metaphor, “glass-box machine,” so the noun arrives after the slogan. |
| Why Moriarty uses it? | The custom-circuit problem is real, and then overstated into “that approach breaks down” and “any” program. | “A cryptographic proof can verify any recursive program” treats a design aim as a finished verifier, and “out in the open” sounds like the values are published. | “Checkable steps” is the right aim. “Every step burns 1 fuel” and “you can check any single step on its own” are both too clean to be true. |
| How do I read the chip? | The factorial note is concrete. The rings, the dial, the fuel rim, and the `+N` window are missing. | Best attempt at the picture, and the picture is wrong: west is not “the caller,” east is not “the math,” and twelve slots are not the frame limit. | Describes six rings and two slots, then turns one factorial moment into the law of the machine. |

After the copy above, a new reader should be able to say: Sherlock is one interpreter for the recursive programs its language accepts; unfinished work is saved as frames so a proof can check these steps; `R1` is oldest; left is older than right on that ring; colour is frame type; the dial is the current control; the outer rim is fuel; the slider, not the twelfth slot, is the frame cap.

What this top copy deliberately does not teach, because the lower cards already do and the masthead cannot survive the extra clauses: in-place rewrites (`Left` becomes `Right` at the same depth), the eleven-frame peak of `fact(4)` versus the eight-frame pause before `fact(0)`, and Candidate B’s bounded image. Those facts are true. They are not the first paragraph.

## Audit

The shared failure is a template. Each draft finds the host call stack, declares it hidden, and offers a household object in its place: tray, shelf, table, reminder card, scratchpad. The template is repeated until it sounds like an explanation. The stack is not a secret compartment in the CPU. It is the ordinary place a running program keeps unfinished calls. Sherlock’s difference is that those calls are fields in its defined state, under one rule set, profile `sherlock-k-a/1`. Say that. The furniture can go.

The same template smuggles in three technical errors every time it appears.

- Explicit state is not public disclosure. This page shows values because it is an inspector. A proof of a step does not, by that fact, print them.
- Fuel is not one unit per anything that happens. On this profile a charged machine rule costs 1. Publishing `Done`, `Rejected`, or `Exhausted` costs 0. Lookup, binding, matching, equality, and copying inside the step are not charged again.
- Left and right are ages, not jobs. Slot A, on the left, is the older frame of the pair. Slot B, on the right, is the newer. Either side can hold any frame type. `4 × [?]` is a gold `Right` frame during factorial, after the left operand has been stored. It is not the meaning of the east arc.

### Draft 1 — repository distillation

The body in section 1 is closer to the design than the other two drafts, and the page copy throws that advantage away.

Keep the instinct of Draft B: one interpreter, the same rules, no fresh circuit per program. Drop “small,” “tiny,” and “step-by-step engine.” Nothing in the design gives a size, and “engine” is a label pasted on after the noun “interpreter” was already available.

“The world Moriarty targets” is filler. “Countable, bounded, and auditable” is three adjectives doing the work of a sentence that was never written. “Baking,” “sprawling,” and “bespoke” editorialize a compiler strategy that is one option, not the only alternative. “Guess the maximum recursion depth” makes unrolling sound like a mistake someone got caught making.

“Open-state calculator” in Draft C invents a term and picks the wrong object. Sherlock runs a language: calls, `let`, conditionals, match, constructors. A calculator does arithmetic. “Waiting notes on an open table” and “one slot on the tray” then teach the wrong data. One call can occupy several frames. A `Right` frame does not hold the caller’s variables. Some frames are operands, tests, or scopes, not suspended calls. The run is not promised to reach a final answer. It can end `Exhausted` or `Rejected`.

`4 × [?]` is the one image worth saving. It is a stored left operand and a hole, drawn on a gold arc. It is not a note on a tray.

### Draft 2 — plain-language guide

The sixty-second section is a blog shape: how ordinary computers do it, how Sherlock does it, why that matters. The middle beat is a triad of absences (“no hidden call stack, no memory pointers, and no mutable scratch cells”). That cadence announces importance and hides the qualifier that matters. Those absences belong to the source semantics. The host program that runs this page is allowed ordinary references and mutable structures. The reader is looking at a rendered chip, not at a proof that the machine has no memory.

“See how a computer solves recursion when nothing is allowed to hide in memory” is a headline that needs translating before it says anything. Recursion is not a problem a computer solves. “Nothing is allowed to hide” is house style for a rule the draft never states.

“Did Step 14 legally turn into Step 15?” is understandable, and “legally” is the wrong word. The rule either applies or it does not.

Column 02 is the draft that tries hardest to orient a newcomer, and it is the one that will stick as a false legend. “West saves the caller’s variables (Teal), and East holds paused math like `4 × [?]` (Gold)” is one settled moment of `fact`, mistaken for the geometry. `sum_list` puts a scope on the right of ring 1. During the factorial test, the second slot is an `If`, not a multiplication. Tail calls leave a return frame and no gold partner. “When the deepest call finishes, the rings clear from the inside out” is the successful factorial unwind told as if programs cannot return and then call again.

Column 03 invites the reader to drag the sliders, which is the right gesture, and then claims the machine “can never loop forever or overflow memory silently.” Fuel and the frame limit end the interpreted run with `Exhausted` or `Rejected`. They are not a promise about the host process. “Built-In Safety Bounds” is a product heading for two counters.

“Universal recursion interpreter” is the current page title and it should not survive. In the design, a universal realization is one option among others, and no winner is selected. The language is a strict first-order fragment, not every recursive program a visitor can imagine.

### Draft 3 — editorial header

The diagnosis of the old deck is half right. “Choose a program. Follow its calls, inspect the saved state, and see how the result is computed.” tells you the gesture and not what you are looking at. It is still the best sentence in the three files for tone. It does not strut. The new copy should keep that plainness and add the missing subject. It should not sneer at the old line as “generic UI widget mechanics.”

“A glass-box machine that runs recursive programs one verifiable step at a time” asks the reader to unpack a metaphor and a verification claim before learning the product. Glass-box is “not a black box,” which is a slogan aimed at a slogan. “Verifiable” reads as a delivered proof. This page shows reference steps. Candidate B is specified, not shipped, and the design does not establish a circuit here.

“Puts every step on the table” and “a visible reminder card” are the tray metaphor again, plus a false merger: the card is asked to hold both `4 × [?]` and the caller’s variables. “Flat snapshot” reaches for Candidate B’s bounded image. The page shows the reference snapshot.

“No hidden scratchpad,” “single-rule ticks,” and “swallow the whole run” are writer’s furniture. “Ticks” is not what the inspector calls a step. A local step check needs the program and the state it started from. It does not, by itself, certify the whole run. “Watch calls stack inward and answers unwind outward” is memorable and too narrow. Frames are not only calls. Expression evaluation and argument processing push frames too. The base-case story is factorial, and the sentence forgets to say so.

The ring sentence is the most usable mechanical description in the drafts, once “hold up to 12 saved frames” becomes “draw up to 12,” and once the `+N` mark is mentioned. “Slot A on the left, Slot B on the right” matches the chip. Keep the positions. Delete the moral.

“SHERLOCK · MORIARTY EXECUTION ENGINE” adds a register the page does not need. The brandmark is already the name.

## Where the other review holds up

`prose_review_gpt_sol.md` is right about the facts the drafts get wrong: the host stack is not secret hardware, explicit is not the same as public, fuel is not uniform, left and right are chronological, twelve is a display window, and this page is the reference run rather than a delivered verifier. Its recommended cards for the frame window, the `+N` mark, and the two outcomes are the first limits copy that would not mislead.

Its own page text is still not what should ship.

The headline “How Sherlock runs recursive programs” is a section title. Under a brandmark that already says Sherlock, it tells you the genre of the paragraph and not the thing.
The lead never shows `4 × [?]`, which is the only image in the drafts a newcomer can see on the chip a second later. It also says “a verifier can check” in the present tense, and it never says that this screen is that reference run.
“Colours identify frame types” is true and useless while the arcs are painted teal, cyan, gold, amber, and violet.
The dial card folds ten controls into “evaluating, saving unfinished work, calling, or returning.” Those four phrases are not the labels on the metal, and “returning” collides with the teal caller frame. The needle, which does point left or right, is absent.
The three cards are entirely a legend. After them, the reader still has not been told, in the strip itself, why `fact(4)` saved anything.

The copy at the top of this file keeps that review’s corrections and puts back the factorial frame, the colour legend, the dial’s real names, and the needle.

Draft 1B is the strongest starting point: it explains that Sherlock uses the same execution rules across supported programs. None of the drafts is ready unchanged. The recurring “hidden hardware versus visible waiting notes” contrast introduces both artificial rhythm and technical errors.

**Recommended page copy**

**Masthead headline**

How Sherlock runs recursive programs

**Lead paragraph**

Sherlock is Moriarty’s interpreter for recursive programs, where functions call themselves or one another. It uses the same execution rules for every supported program. To support cryptographic proofs of execution, Sherlock records unfinished work and variable values explicitly, so a verifier can check each step without relying on the computer’s call stack.

**Three-column orientation strip**

| 01 · Center dial | 02 · Six rings | 03 · Execution limits |
|---|---|---|
| The highlighted sector shows the kind of work at the selected step: evaluating an expression, saving unfinished work, calling a function, or returning a value. | The rings show saved pieces of unfinished work, called **frames**. Each ring has two slots, filled from the outside inward, with the older frame on the left. The display shows up to 12 frames; a **+N** badge counts additional frames. Colours identify frame types. | The outer rim shows remaining **fuel**, Sherlock’s execution budget. The **Frame limit** slider sets how many saved frames a run may use. Running out of fuel produces **Exhausted**; trying to exceed the frame limit produces **Rejected**. |

This explains the interpreter, its reason for recording state explicitly, and the visual display without introducing a second vocabulary of trays, shelves, cards, and scratchpads.

**Technical corrections that apply across the drafts**

These are substantive, not merely stylistic:

- **An ordinary call stack is not inherently secret or internal hardware.** It is usually a software-managed memory structure. Sherlock makes pending computation part of its defined machine state rather than depending on the host’s execution context.
- **Explicit does not mean public.** “Everything out in the open” risks suggesting that a zero-knowledge proof reveals variable values.
- **Not every transition costs one fuel unit.** The displayed `sherlock-k-a/1` profile charges machine rules, while terminal publication costs zero and helpers remain unmetered. Fuel is not a measurement of all host work. See [the design’s accounting section](/home/charles/repos/Moriarty/openspec/changes/sherlock-universal-interpreter-design/design.md:25).
- **Left and right are chronological slots, not fixed semantic roles.** Either side can hold different frame types. The renderer fills slots oldest first. See [the ring mapping](/home/charles/sherlock-bench/src/designers/designer2_well.js:2178).
- **Twelve is the display window, not the interpreter’s frame limit.** The slider allows more than twelve frames.
- **The page illustrates reference execution.** Candidate B remains specified-only; neither the design nor this page establishes a delivered cryptographic verifier. See [the design’s delivery qualifications](/home/charles/repos/Moriarty/openspec/changes/sherlock-universal-interpreter-design/design.md:81).

The sentence checks below follow each file’s order. Quoted openings identify the sentences without reproducing the drafts in full.

**Draft 1 — [Repository distillation](/home/charles/sherlock-bench/audits/top_prose_01_repo_distillation.md:12)**

| Sentence or copy element | Assessment |
|---|---|
| “On a normal computer…” | The recursive example helps. “The processor hides its place” anthropomorphizes the CPU and misdescribes the stack as internal hardware. |
| “In zero-knowledge proofs…that approach breaks down…” | Overstates the problem. Ordinary recursive execution needs a suitable proof representation; it does not simply “break down.” “The world Moriarty targets” is disposable filler. |
| “Circuits cannot loop indefinitely or rely on hidden hardware state.” | The finite-circuit point is useful, but the second half needs explanation: the proof must constrain the relevant state changes. |
| “A cryptographic circuit is a fixed mathematical grid.” | “Grid” supplies a picture without explaining the thing. “A fixed set of mathematical constraints” is more accurate. Neither belongs in the masthead. |
| “If a compiler tries…” | Presents a false either/or. “Guess,” “bespoke,” and “sprawling” editorialize. Unrolling is one approach, not the only alternative to Sherlock. |
| “Every step…must be countable, bounded, and auditable.” | A generic three-adjective proclamation. It also obscures the distinction between charged machine steps and unmetered helper work. |
| “When a contract runs on a ledger…” | Too broad. Host stacks and heaps can implement execution; the issue is whether the proof accounts for the relevant behavior. |
| “Every pending calculation…must sit out in the open…” | Retain the explicit-state principle. Remove the visibility metaphor and distinguish proof state from publicly disclosed data. |
| “Moriarty built Sherlock…one small, fixed-rule interpreter…” | “Fixed-rule interpreter” is useful. “Small” has no stated measure. Keep the profile identifier below the introductory copy. |
| “Instead of baking each recursive program…” | The shared-rules explanation is strong. “Baking,” “trusting,” and “tiny” add rhetoric. The program includes more than a lookup table; named functions resolve through one. |
| A subtitle: “How Moriarty runs recursive programs…” | Clear and relevant. “Without a hidden call stack” needs the host-stack distinction. |
| A: “Normal computers hide unfinished function calls…” | Repeats the inaccurate premise. Memory itself is not what makes recursion difficult to verify. |
| A: “Sherlock is a tiny, step-by-step calculator…” | Severely overloaded. It combines definition, example, representation, evaluation, fuel, and capacity. “Calculator” undersells the language; tray/notes imagery adds vocabulary; fuel claim is false. |
| B subtitle: “Moriarty’s step-by-step engine for verifiable recursion” | Reasonably concise, but “verifiable recursion” remains specialist language. |
| B: “Instead of building a separate custom circuit…” | Best conceptual starting point, but makes an implementation choice sound settled. State the supported-program/shared-rules fact directly. |
| B: “Sherlock breaks any recursive calculation…” | “Any” exceeds the supported language, and “each cost 1” is incorrect. |
| B: “Whenever a step has to wait…” | Understandable intention, muddled representation. Different frame types save different information; not every frame combines unfinished arithmetic and current variables. Cut “bounded tray” and “plain sight.” |
| C subtitle: “Moriarty’s open-state calculator…” | “Open-state” is invented explanatory jargon. “Interpreter” is the accurate noun. |
| C: “To make recursive code provable…” | States the purpose, but the tray metaphor conceals the actual mechanism: explicit frames. |
| C: “Every step burns…every waiting call takes 1 slot…” | Both accounting claims are wrong. One call can involve several frames, and frames also represent operands, branches, and scopes. It also assumes successful completion. |

**Draft 2 — [Plain-language guide](/home/charles/sherlock-bench/audits/top_prose_02_plain_language_guide.md:3)**

| Sentence or copy element | Assessment |
|---|---|
| Background: “When a program calls itself…” | The example works. “Private call stack,” “memory return addresses,” and “mutable scratch space” make a simple idea harder while repeating the hardware misconception. |
| “Sherlock has no hidden call stack, no memory pointers…” | The emphatic three-part denial sounds generated. Qualify this as the source semantics; implementation storage can use references and mutable structures. |
| “Every time a calculation pauses…” | The examples distinguish `Return` and `Right` usefully, but “self-contained waiting note on an open shelf” is unnecessary and misleading: a `Right` frame does not contain the caller’s environment. |
| “Because the entire state…a cryptographic proof can verify any recursive program…” | Too long and too absolute. Restrict the language scope and describe verification as the design purpose. “Did Step 14 legally turn into Step 15?” is understandable; “follow the rules” is plainer than “legally.” |
| Headline: “See how a computer solves recursion…” | Recursion is a programming technique, not a problem a computer “solves.” “Nothing is allowed to hide in memory” is theatrical and inaccurate. |
| Lead: “Sherlock is Moriarty’s universal recursion interpreter.” | Accurate project identification, but requires an immediate explanation of recursion and the supported-language meaning of “universal.” |
| Lead: “Instead of burying unfinished calls…” | Repeats the same contrast without adding information. “Burying” supplies drama; “out in the open” confuses explicit state with disclosure. |
| Column 1 title: “Recursion without hidden memory” | Overstates the claim. “Why Sherlock records unfinished work” would be more precise. |
| Column 1: “When a normal computer runs…” | Repeats the lead’s flawed premise. Remove. |
| Column 1: “Sherlock keeps everything in the open…” | “Multiply 4 by whatever answer comes back” is immediately understandable. Keep that explanation if an example is needed; remove the surrounding shelf language and universal visibility claim. |
| Column 2 title: “Center dial & six split rings” | Concrete and useful. |
| Column 2: “The center dial lights the exact action taken…” | Almost useful, but “exact action taken” confuses control state with the transition just executed. Say it shows the kind of work at the selected step. |
| Column 2: “Around it…the six split rings…” | Technically wrong. West does not always save variables, and East does not always hold arithmetic. Those are patterns in particular factorial states. |
| Column 2: “When the deepest call finishes…” | Describes the successful factorial unwind, not general execution. Other programs can return and then make further calls. |
| Column 3 title: “Built-In Safety Bounds” | Broader than the evidence. “Execution limits” names what is actually shown. |
| Column 3: “Every step burns one unit…” | Incorrect fuel claim. Also confuses the configurable frame capacity with a fixed shelf. |
| Column 3: “Try lowering…the sliders…” | The interaction suggestion is useful. The remainder overclaims host-memory protection and delivered verification. Explain `Exhausted` and frame-limit rejection directly. |

**Draft 3 — [Editorial header](/home/charles/sherlock-bench/audits/top_prose_03_editorial_header.md:3)**

| Sentence or copy element | Assessment |
|---|---|
| Existing copy: “Choose a program.” | Clear instruction. It simply needs explanatory context before it. |
| Existing copy: “Follow its calls…” | Understandable, useful interaction guidance. The audit calls this the old subtitle, but it is the current **lead paragraph**; the existing headline already identifies Sherlock. |
| Audit: “It describes generic UI widget mechanics…” | Fair diagnosis of insufficient orientation. “Generic UI widget mechanics” overstates how unhelpful the instruction is. |
| Audit: “It omits the core problem…” | Fair, but retain the explicit-state rationale rather than repeating the custom-circuit/hidden-hardware dichotomy. |
| Audit: “It leaves the 3D hollow die…” | Correct that the display needs explanation. Listing its geometry and sector counts is not itself an explanation. |
| Kicker: “SHERLOCK · MORIARTY EXECUTION ENGINE” | Unnecessary expansion of the brandmark. “Sherlock” is sufficient. |
| Headline: “A glass-box machine…” | Reject. “Glass-box” asks the reader to decode a metaphor before learning what the software does. “One verifiable step at a time” sounds polished but leaves verification status unclear. |
| Lead: “Most computers run…until the final answer pops out.” | “Hiding” misdescribes execution; “pops out” trivializes it. The sentence explains neither recursive calls nor stack restoration clearly. |
| Lead: “Sherlock puts every step on the table…” | The shared rules and supported-language qualification are good. The sentence is overloaded, and “flat snapshot” conflates reference state with the proposed bounded representation. |
| Column 1 title: “No hidden scratchpad” | Adds another metaphor with no explanatory benefit. |
| Column 1: “When fact(4) needs fact(3)…” | Good dependency example. Replace the second half with the ordinary fact that the caller must retain enough information to resume. |
| Column 1: “Sherlock has no hidden stack…” | Too broad without the semantic qualification. “Changeable memory cells” is awkward and distracts from the state-recording explanation. |
| Column 1: “Instead, it writes a visible reminder card…” | Cut “reminder card” and “plain sight.” It also risks implying one combined frame contains both `4 × [?]` and the caller’s variables. |
| Column 2 title: “Big recursive runs become small, checkable steps.” | The idea is sound, but the balanced big/small phrasing is generic. “How steps are checked” is more direct. |
| Column 2: “Proving…swallow the whole run…” | Delete. It explains no mechanism and creates an unsupported premise about proof methods. |
| Column 2: “Sherlock breaks execution into single-rule ticks…” | Keep the before-state/rule/after-state explanation. Drop “ticks,” and correct the claim that every transition burns fuel. |
| Column 2: “You can check any single step on its own…” | A local transition check needs its program and state context; it does not alone establish the full run. Resource-limit behavior is a separate point. |
| Column 3 title: “Watch calls stack inward…” | Memorable but imprecise: frames represent more than calls. |
| Column 3: “The six concentric rings…hold up to 12 saved frames…” | Mostly useful. Say **display** up to twelve, and explain the additional-frame badge. |
| Column 3: “As a program calls itself deeper…” | Oversimplifies frame growth. Expression evaluation and argument processing also add frames. |
| Column 3: “Once it hits a base case…” | Correct for the successful factorial example, but presented as general behavior. Label it as an example or remove it. |

The strongest material across all three is the shared execution rules, explicit saved state, and the concrete meaning of `4 × [?]`. Those facts carry the explanation without the repeated visibility slogans.
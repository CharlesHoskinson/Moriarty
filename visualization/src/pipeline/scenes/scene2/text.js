// Beat ids, titles, durations and narration of scene 2, copied from the storyboard.
export const BEATS = [
  {
    "id": "machine-arrival",
    "title": "Machine arrival",
    "duration": 23,
    "narration": "<code>mori simulate</code> receives the ROM program, the input value and the resource policy that <code>input_d</code> binds. It runs a CEK-style machine, which keeps the current computation, the variable bindings and the saved continuations explicit. The reference machine <code>sherlock-k-a/1</code> defines every transition. The run produces the detailed evidence that the leaf checks. The simulator's own record certifies nothing."
  },
  {
    "id": "machine-state",
    "title": "Machine state",
    "duration": 24,
    "narration": "The machine state has six parts. Control names the expression, return or helper action in progress. The environment maps variable names to values. The frame stack holds suspended computation and return context, and the task stack holds unfinished helper work. Memory stores immutable records. Fuel counters track available and consumed work, beside the logical resource counters that execution updates."
  },
  {
    "id": "enabled-rule",
    "title": "Enabled rule",
    "duration": 24,
    "narration": "Each step fires exactly one rule out of 120. A rule is enabled when its conditions hold for the current state and authenticated instruction, under the reference priority. Exactly one means that the selected rule fixes every permitted update and other state stays unchanged. A claimed transition cannot mix writes from two rules or pick a convenient branch."
  },
  {
    "id": "named-call-context",
    "title": "Named call context",
    "duration": 24,
    "narration": "The call to lift saves the caller's continuation in a frame, so returning restores the environment and next expression. The addition inside lift produces the raised value. The comparison produces the Boolean that selects the conditional branch, and named calls continue from that choice. The saved input stays reachable through its environment reference until the final addition reads it."
  },
  {
    "id": "resource-accounting",
    "title": "Resource accounting",
    "duration": 24,
    "narration": "Fuel starts at the budget that <code>input_d</code> binds. Each rule applies its specified charge to remaining fuel and to consumed work. Helper counters track traversal and construction, and resource limits bound stack growth and allocation. Each check compares the requested work with the remaining allowance, so no free computation, hidden allocation or counter wrap can extend the authorized run."
  },
  {
    "id": "witness-frame",
    "title": "Witness frame",
    "duration": 23,
    "narration": "Each fired rule records one frame on the witness stream, the private execution tape. A frame holds the rule id, the state before and the state after. Each frame begins in the state where the previous frame ended. An unexplained change between recorded steps breaks that equality. The leaf checks this continuity and the rule's actual transition."
  },
  {
    "id": "step-counter",
    "title": "Step counter",
    "duration": 23,
    "narration": "A 64-bit counter counts the fired steps, terminal publication included. A run has up to 2<sup>64</sup> - 1 steps, and its total is <code>n_steps</code>. The value 2<sup>64</sup> is the counter's range, not the size of a circuit. Counter checks stop a long execution from wrapping into a smaller claimed count. Fuel charges and step counts stay separate."
  },
  {
    "id": "arithmetic-rejection",
    "title": "Arithmetic rejection",
    "duration": 21,
    "narration": "An addition that overflows ends the run as Rejected with code 10, the payload for arithmetic overflow. The proof checks that the arithmetic really overflows, and the wrapped result never becomes a successful return value. This ending uses the same rule system and the same proof key as the successful one."
  },
  {
    "id": "fuel-exhaustion",
    "title": "Fuel exhaustion",
    "duration": 23,
    "narration": "When the next required work charge exceeds the remaining fuel, the run ends as Exhausted with the kind Fuel. The payload names the resource. The check compares the required charge with the remaining allowance, so no prover can fake exhaustion. A leaf that fills its proving capacity only moves the tape cut and never produces this outcome."
  },
  {
    "id": "halted-tape-handoff",
    "title": "Halted tape handoff",
    "duration": 24,
    "narration": "The main run reads the saved input, completes its final addition and returns the selected value. Each return through the frame stack restores a suspended context. At the outer return, publication records Halted with the result value as its payload. The witness tape ends at that outcome frame and carries <code>n_steps</code>, the run's step count, into leaf proving."
  }
];

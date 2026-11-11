// Beat ids, titles, durations and narration of scene 1, copied from the storyboard text.
export const TEXT = [
  {
    "id": "source-and-input",
    "title": "Source and input",
    "duration": 22,
    "narration": "Moriarty source enters with an input value named input. The program saves that value, calls lift to add increment and compares the result with ceiling. A conditional selects the next value. Further named calls create intermediate values, and the selected branch then adds the saved input. The main run passes every arithmetic and resource check."
  },
  {
    "id": "source-to-rom",
    "title": "Source to ROM",
    "duration": 22,
    "narration": "The compiler turns the source into a program image: the machine language, stored in ROM (read-only memory). Its tables hold expressions, function entries, argument references, types, constructors and constants. A rule reads the table and row that its instruction names. Each row is authenticated against this image, so nobody can substitute another operation or constant."
  },
  {
    "id": "program-digest",
    "title": "Program digest",
    "duration": 22,
    "narration": "Poseidon, the hash function of Sherlock, digests the program image into <code>prog_d</code>. The image holds the code tables and their entry information in a fixed encoding. Changing any encoded operation, constant, function body or entry changes what the digest commits to. Later program reads prove membership under this commitment, so substituted code cannot run."
  },
  {
    "id": "canonical-input",
    "title": "Canonical input",
    "duration": 22,
    "narration": "The input value becomes <code>val_d</code> under <code>sherlock-value/1</code>, the canonical value digest. Canonical means that equal values hash from one encoding, fixed by type, value form and ordered contents. A reference contributes the digest of its target, so moving records or sharing equal substructures keeps the digest. A different value gives a different <code>val_d</code>."
  },
  {
    "id": "input-commitment",
    "title": "Input commitment",
    "duration": 29,
    "narration": "<code>input_d = Poseidon(SHK_IN, val_d, fuel_lo, fuel_hi, stack_limit)</code>. The digest binds the input value to the resource policy of the run: the 128-bit fuel, written as two 64-bit halves, and the stack limit. The two halves and the stack limit are each range-checked below 2<sup>64</sup>. The same value under a different fuel budget or stack limit gives a different <code>input_d</code>, so a claim names its resource policy together with its input value."
  },
  {
    "id": "fixed-key-handoff",
    "title": "Fixed key handoff",
    "duration": 25,
    "narration": "The ROM rack and the tokens <code>prog_d</code> and <code>input_d</code> identify the run. Three key cards serve it: leaf, node and decider. A key fixes a proof circuit, the constraints that check its witness. Every program and input uses this key set, because the circuit reads authenticated program data from ROM. A second program changes the program token and leaves the keys unchanged."
  }
];

// Beat text of scene 3, copied from STORYBOARD.md scene 3. Label positions are in labelpos.js.
export const BEATS = [
 {
  "id": "tape-to-slices",
  "title": "Tape to slices",
  "duration": 23,
  "narration": "The witness tape arrives with its Halted outcome frame attached. The leaf cuts the tape into slices of <code>S = 16</code> step slots, each entering the same circuit under one leaf key. A full slice uses every slot. A shorter slice keeps its unused suffix inactive, so state holds still and the proof covers the recorded steps.",
  "labels": [
   {
    "text": "Witness stream"
   },
   {
    "text": "<code>S = 16</code>"
   },
   {
    "text": "Leaf key"
   },
   {
    "text": "Active slice"
   },
   {
    "text": "Inactive suffix"
   }
  ]
 },
 {
  "id": "core-register-chain",
  "title": "CORE register chain",
  "duration": 23,
  "narration": "CORE, the module that checks machine transitions, holds the persistent registers and the rule slots. Its registers encode control, environment reference, cached stack tops, helper work, result, outcome and resource counters. Each slot's final register cells equal the next slot's initial cells. Registers outside the selected rule's write set keep their values and hide no change.",
  "labels": [
   {
    "text": "CORE"
   },
   {
    "text": "Persistent registers"
   },
   {
    "text": "Rule slot"
   },
   {
    "text": "Equal state"
   },
   {
    "text": "Unchanged registers"
   }
  ]
 },
 {
  "id": "decoder-control-word",
  "title": "Decoder control word",
  "duration": 25,
  "narration": "DEC, the decoder, derives a rule control word from a fixed table. It checks the current control and outcome, the authenticated instruction, stack tags and comparison results against each rule's conditions, and reference priority selects exactly one matching rule. The returned word fixes the rule id, operation order and write set. Bounded fields stop any other word passing for that entry.",
  "labels": [
   {
    "text": "DEC"
   },
   {
    "text": "Dispatch facts"
   },
   {
    "text": "Fixed table"
   },
   {
    "text": "Control word"
   },
   {
    "text": "Write set"
   }
  ]
 },
 {
  "id": "active-prefix",
  "title": "Active prefix",
  "duration": 22,
  "narration": "Active slots form one continuous prefix. Each active slot counts as one step, and its internal operations add no steps. An inactive slot keeps every persistent register and makes no request. Terminal publication ends the active prefix. A prover cannot skip a transition, pad the step count or keep executing after Halted, Rejected or Exhausted.",
  "labels": [
   {
    "text": "Active prefix"
   },
   {
    "text": "Terminal publication"
   },
   {
    "text": "Inactive suffix"
   },
   {
    "text": "State preserved"
   }
  ]
 },
 {
  "id": "bus-tuple",
  "title": "BUS tuple",
  "duration": 24,
  "narration": "BUS connects CORE requests to checked provider tuples <code>(chip, key, a, b, c)</code>. The chip names the operation family, the key selects the operation or record, and the other cells carry the data. Chip ids are 1 for ALU, 2 for MEM, 3 for HASH and 4 for ROM. CORE alone issues requests, through the two request lanes.",
  "labels": [
   {
    "text": "BUS"
   },
   {
    "text": "<code>(chip, key, a, b, c)</code>"
   },
   {
    "text": "ALU 1"
   },
   {
    "text": "MEM 2"
   },
   {
    "text": "HASH 3"
   },
   {
    "text": "ROM 4"
   }
  ]
 },
 {
  "id": "request-meets-provider",
  "title": "Request meets provider",
  "duration": 24,
  "narration": "CORE puts the addition's operation, operands and claimed result into a BUS request. The whole tuple must equal a checked ALU provider tuple. Every provider satisfies its operation constraints, even when no request uses it. Several requests can match the same provider, but requests cannot authenticate each other. An invented answer therefore has no valid computation to match.",
  "labels": [
   {
    "text": "CORE request"
   },
   {
    "text": "ADD"
   },
   {
    "text": "Checked provider"
   },
   {
    "text": "Equal tuple"
   }
  ]
 },
 {
  "id": "checked-addition",
  "title": "Checked addition",
  "duration": 29,
  "narration": "ALU, the arithmetic module, proves the exact unsigned addition of the value and increment. It constrains both operands, the result and the overflow flag, and a carry past the permitted integer width sets the flag. CORE connects a set flag to Rejected with code 10. In the main run the flag is clear, and the ordinary result passes unwrapped. The same ALU checks subtraction, multiplication and equality under their machine rules.",
  "labels": [
   {
    "text": "ALU"
   },
   {
    "text": "ADD"
   },
   {
    "text": "Operands"
   },
   {
    "text": "Result"
   },
   {
    "text": "Overflow flag"
   },
   {
    "text": "Code 10"
   }
  ]
 },
 {
  "id": "checked-comparison",
  "title": "Checked comparison",
  "duration": 24,
  "narration": "The LT request asks ALU whether the raised value is less than ceiling. ALU constrains its Boolean result to that comparison of the authenticated operands, and CORE uses the same result cell to choose the conditional branch. A Boolean changed without a change to the operands fails the comparison check, so the prover cannot pick the branch it prefers.",
  "labels": [
   {
    "text": "LT"
   },
   {
    "text": "Raised value"
   },
   {
    "text": "Ceiling"
   },
   {
    "text": "Boolean result"
   },
   {
    "text": "Branch condition"
   }
  ]
 },
 {
  "id": "integer-range-checks",
  "title": "Integer range checks",
  "duration": 31,
  "narration": "RANGE proves that integer cells fit their required widths. It reconstructs each checked cell from 12-bit limbs, small integer pieces checked against a fixed table, and the highest piece also obeys the exact width of the original integer. The reconstruction equals the caller's cell. Oversized operands and modular arithmetic tricks that imitate valid integer equations fail here. Fixed table tags keep RANGE queries apart from DEC queries, so neither check can borrow the other's table entries.",
  "labels": [
   {
    "text": "RANGE",
    "kind": "neutral"
   },
   {
    "text": "12-bit limbs",
    "kind": "neutral"
   },
   {
    "text": "Exact top width",
    "kind": "neutral"
   },
   {
    "text": "Equal reconstruction",
    "kind": "state"
   },
   {
    "text": "Fixed table tags",
    "kind": "neutral"
   }
  ]
 },
 {
  "id": "poseidon-relations",
  "title": "Poseidon relations",
  "duration": 34,
  "narration": "HASH computes Poseidon over the caller's ordered input cells. It constrains the initial state, absorption, permutation rounds and final output. Fixed domain tags tell memory records, tree levels, stacks and state bodies apart, so a stack link cannot pass for a memory node. Public digest formats keep their specified encoding. Each input copies the caller's cell, so substituted data cannot be hashed. Longer hashes use direct ports with copied input cells in a fixed order. Chained short BUS hashes differ from the whole-message hash.",
  "labels": [
   {
    "text": "HASH"
   },
   {
    "text": "Ordered inputs"
   },
   {
    "text": "Domain tag"
   },
   {
    "text": "Poseidon"
   },
   {
    "text": "Digest"
   },
   {
    "text": "Direct port"
   }
  ]
 },
 {
  "id": "append-address-chain",
  "title": "Append address chain",
  "duration": 25,
  "narration": "CORE owns the append chain, the ordered sequence of new memory addresses. The first append starts at the incoming length, each active append advances the length to the next address, and the final length equals the outgoing memory length. Every appended record also matches a MEM provider through BUS. No address is skipped or allocated twice, and no record's contents are swapped.",
  "labels": [
   {
    "text": "Append chain"
   },
   {
    "text": "Incoming length"
   },
   {
    "text": "Append address"
   },
   {
    "text": "MEM request"
   },
   {
    "text": "Outgoing length"
   }
  ]
 },
 {
  "id": "memory-window",
  "title": "Memory window",
  "duration": 24,
  "narration": "The MEM window holds the imported partial tail and this leaf's new records, in a single arena. Each logical record occupies one cell. Active records form a continuous prefix, and the imported records end exactly at the incoming length. Rules append at most 3 cells per step. Padding serves no read. Unused capacity therefore holds no invented program data.",
  "labels": [
   {
    "text": "MEM window"
   },
   {
    "text": "Imported tail"
   },
   {
    "text": "New records"
   },
   {
    "text": "Padding"
   },
   {
    "text": "At most 3 cells"
   }
  ]
 },
 {
  "id": "authenticated-tail",
  "title": "Authenticated tail",
  "duration": 24,
  "narration": "The named calls and intermediate bindings append records in execution order until the 8-cell tail is full. The dense MEMORY BACKEND authenticates the imported tail: it recomputes the tail's digest and matches the incoming tail token. Unused tail positions hold fixed padding. At a leaf boundary, inherited records cannot be replaced and a partial tail cannot take a second encoding.",
  "labels": [
   {
    "text": "MEMORY BACKEND"
   },
   {
    "text": "8-cell tail"
   },
   {
    "text": "Imported records"
   },
   {
    "text": "Fixed padding"
   },
   {
    "text": "Equal tail digest"
   }
  ]
 },
 {
  "id": "segment-flush",
  "title": "Segment flush",
  "duration": 24,
  "narration": "The dense MEMORY BACKEND turns the full 8-cell tail into a segment. Its flush proves that the next segment position is empty under the current tree root. The new segment follows the same address path and sibling hashes to compute the new root. The matching paths stop the flush from overwriting an occupied segment or placing records elsewhere.",
  "labels": [
   {
    "text": "Segment flush"
   },
   {
    "text": "Empty position"
   },
   {
    "text": "Same address path"
   },
   {
    "text": "Shared siblings"
   },
   {
    "text": "New root"
   }
  ]
 },
 {
  "id": "memory-outlasts-counter",
  "title": "Memory outlasts counter",
  "duration": 22,
  "narration": "The memory tree has depth 63 over segments, a capacity of 2<sup>66</sup> cells. Rules append at most 3 cells per step, and 3 x 2<sup>64</sup> &lt; 2<sup>66</sup>, so memory outlasts the step counter even at the maximum append rate. Address checks keep each record inside that space, so field arithmetic cannot disguise an outside address.",
  "labels": [
   {
    "text": "Depth 63"
   },
   {
    "text": "2<sup>66</sup> cells"
   },
   {
    "text": "64-bit counter"
   },
   {
    "text": "Capacity inequality"
   },
   {
    "text": "Address bound"
   }
  ]
 },
 {
  "id": "capacity-closes-slice",
  "title": "Capacity closes slice",
  "duration": 22,
  "narration": "A leaf has one flush unit. Its remaining capacity depends on the imported tail and the work already in the slice. When the next step exceeds that capacity, the leaf ends before it, and execution resumes in the next leaf with no extra charge. The unused suffix stays inactive and preserves the machine state.",
  "labels": [
   {
    "text": "Flush unit"
   },
   {
    "text": "Remaining capacity"
   },
   {
    "text": "Slice boundary"
   },
   {
    "text": "Next step"
   },
   {
    "text": "State preserved"
   }
  ]
 },
 {
  "id": "read-address-and-time",
  "title": "Read address and time",
  "duration": 24,
  "narration": "Every read goes through CORE's authenticated read port. The requested address equals the address in the requesting register or preceding operation, and the record kind equals the kind the rule requires. The address must lie below the running memory length at that exact read point. A read cannot reach a later append, take another reference or confuse record kinds.",
  "labels": [
   {
    "text": "Read port"
   },
   {
    "text": "Requested address"
   },
   {
    "text": "Expected kind"
   },
   {
    "text": "Running length"
   },
   {
    "text": "Later append"
   }
  ]
 },
 {
  "id": "scalar-old-read",
  "title": "Scalar old read",
  "duration": 24,
  "narration": "The last use of the saved input requests an old record from a completed segment. A scalar cold read, one isolated authenticated memory read, hashes the record and follows its Merkle opening: the sibling hashes along the address path. The computed root must equal the leaf's pre-state root, which rules out a forged payload, wrong kind or substituted address.",
  "labels": [
   {
    "text": "Saved input"
   },
   {
    "text": "Scalar cold read"
   },
   {
    "text": "Merkle opening"
   },
   {
    "text": "Computed root"
   },
   {
    "text": "Pre-state root"
   }
  ]
 },
 {
  "id": "authenticated-cache-line",
  "title": "Authenticated cache line",
  "duration": 24,
  "narration": "PAGE/ROM also serves old memory as an 8-record cache line. The dense MEMORY BACKEND hashes the line and checks its path against the same pre-state root. The line's segment must lie before the imported tail. Its checked record cells become MEM providers on BUS. Repeated reads are authenticated, and the cache can neither invent contents nor overlap the window.",
  "labels": [
   {
    "text": "PAGE/ROM"
   },
   {
    "text": "8-record cache line"
   },
   {
    "text": "Pre-state root"
   },
   {
    "text": "Old segment"
   },
   {
    "text": "MEM providers"
   }
  ]
 },
 {
  "id": "authenticated-rom-line",
  "title": "Authenticated ROM line",
  "duration": 24,
  "narration": "ROM lines authenticate program table rows against the program root that <code>prog_d</code> binds. CORE fixes the requested table and range-checks the row index. PAGE/ROM derives each provider key from the authenticated path and row position, and the served cells equal the hashed cells. A row from another table or another program cannot supply an instruction, type or constant.",
  "labels": [
   {
    "text": "ROM line"
   },
   {
    "text": "Table and row"
   },
   {
    "text": "Authenticated path"
   },
   {
    "text": "Program root"
   },
   {
    "text": "<code>prog_d</code>"
   }
  ]
 },
 {
  "id": "frame-chain-push",
  "title": "Frame chain push",
  "duration": 24,
  "narration": "STACKS keeps FRAMES and TASKS as Poseidon hash chains with cached top records. FRAMES stores continuations and TASKS stores helper work. Pushing during the named call hashes the previous top, its depth and the preceding chain into a new link, and the new frame becomes the cached top. Separate hash domains stop a task record posing as a frame.",
  "labels": [
   {
    "text": "STACKS"
   },
   {
    "text": "FRAMES"
   },
   {
    "text": "TASKS"
   },
   {
    "text": "Previous top"
   },
   {
    "text": "Stack depth"
   },
   {
    "text": "Cached top"
   }
  ]
 },
 {
  "id": "frame-chain-pop",
  "title": "Frame chain pop",
  "duration": 24,
  "narration": "Returning from lift pops FRAMES. STACKS recomputes the current chain link from the restored frame supplied, its depth and its predecessor digest. That hash must equal the stored link before the frame becomes the cached top. Counts and empty-stack constants fix the endpoints. A forged return context, wrong predecessor or frame restored at another depth fails this equality.",
  "labels": [
   {
    "text": "Frame pop"
   },
   {
    "text": "Restored frame"
   },
   {
    "text": "Predecessor"
   },
   {
    "text": "Stored link"
   },
   {
    "text": "Equal link"
   }
  ]
 },
 {
  "id": "task-rewrite-before-push",
  "title": "Task rewrite before push",
  "duration": 24,
  "narration": "Helper work uses the same checked chain protocol on TASKS. When a helper advances a parent cursor and pushes a child, the top rewrite precedes the push, so the chain commits to the updated parent. Popping the child restores that updated record. CORE checks this order as one machine event, so no work repeats and no cursor update is lost.",
  "labels": [
   {
    "text": "TASKS"
   },
   {
    "text": "Updated parent"
   },
   {
    "text": "Child task"
   },
   {
    "text": "Cursor"
   },
   {
    "text": "Same machine step"
   }
  ]
 },
 {
  "id": "value-digest-at-creation",
  "title": "Value digest at creation",
  "duration": 23,
  "narration": "VALUE DIGEST computes <code>sherlock-value/1</code> whenever a rule creates a value. The value form, scalar contents and authenticated child digests determine the hash. CORE supplies the child records and ROM data through its checked read ports. The resulting digest equals the digest stored with the new record, so a correct-looking record cannot carry an invented semantic value.",
  "labels": [
   {
    "text": "VALUE DIGEST"
   },
   {
    "text": "<code>sherlock-value/1</code>"
   },
   {
    "text": "Authenticated contents"
   },
   {
    "text": "Value digest"
   },
   {
    "text": "New record"
   }
  ]
 },
 {
  "id": "structured-value-checks",
  "title": "Structured value checks",
  "duration": 24,
  "narration": "VALUE DIGEST checks scalar bounds, Boolean values, constructor identity, field order and field count. A structured value combines its children's digests, and constructor processing carries checked fold state across steps. Completion confirms that every field contributes before the value is used. A reference keeps its target's digest. Sharing and relocation keep value identity, and a missing or substituted field fails.",
  "labels": [
   {
    "text": "Constructor identity"
   },
   {
    "text": "Ordered fields"
   },
   {
    "text": "Fold state"
   },
   {
    "text": "Completion check"
   },
   {
    "text": "Equal value digest"
   }
  ]
 },
 {
  "id": "boundary-body",
  "title": "Boundary body",
  "duration": 24,
  "narration": "BOUNDARY COMMITMENT binds the state that execution needs to continue. Its body hashes the persistent registers, constructor progress, stack hashes and counts, and memory <code>(len, root, tail_d)</code>. Here len is the allocated length, root commits completed segments and tail_d commits the partial tail. Each input equals its owning module's checked cell, so the commitment cannot describe another state.",
  "labels": [
   {
    "text": "BOUNDARY COMMITMENT"
   },
   {
    "text": "Persistent registers"
   },
   {
    "text": "Stack state"
   },
   {
    "text": "<code>(len, root, tail_d)</code>"
   },
   {
    "text": "<code>body_d</code>"
   }
  ]
 },
 {
  "id": "boundary-header",
  "title": "Boundary header",
  "duration": 24,
  "narration": "The boundary header holds <code>outcome_tag</code>, <code>outcome_payload</code>, <code>input_d</code>, <code>deps_d</code> and <code>body_d</code>. The outcome fields carry the execution status and its value digest, rejection code or exhaustion kind. The dependency digest binds consumed claims. Poseidon hashes the header in this fixed order. Its digest binds the readable outcome fields to the complete body, and checking the header leaves the registers hidden.",
  "labels": [
   {
    "text": "<code>outcome_tag</code>"
   },
   {
    "text": "<code>outcome_payload</code>"
   },
   {
    "text": "<code>input_d</code>"
   },
   {
    "text": "<code>deps_d</code>"
   },
   {
    "text": "<code>body_d</code>"
   },
   {
    "text": "State digest"
   }
  ]
 },
 {
  "id": "endpoint-digests",
  "title": "Endpoint digests",
  "duration": 24,
  "narration": "The entry header digest becomes <code>pre_d</code> and the exit header digest becomes <code>post_d</code>. Each public token equals its computed hash output. By Poseidon's collision resistance, equal boundary tokens bind equal committed states. Temporary scratch stays outside the boundary because each use depends on a value produced earlier in the same slot. Incoming scratch is unconstrained and cannot change persistent execution.",
  "labels": [
   {
    "text": "<code>pre_d</code>"
   },
   {
    "text": "<code>post_d</code>"
   },
   {
    "text": "Computed digest"
   },
   {
    "text": "Within-slot scratch"
   },
   {
    "text": "Producer connection"
   }
  ]
 },
 {
  "id": "genesis-initialization",
  "title": "Genesis initialization",
  "duration": 25,
  "narration": "INIT checks the initial state only when <code>h_pre = 0</code>. It proves <code>Init(prog_d, input_d)</code>: the entry control, Running outcome, resource counters, empty stacks and empty genesis memory match their prescribed initial values. It also authenticates the program and input commitments. The height check drives the INIT selector, so a prover cannot switch initialization off and start from a convenient intermediate state.",
  "labels": [
   {
    "text": "INIT"
   },
   {
    "text": "<code>h_pre = 0</code>"
   },
   {
    "text": "Running"
   },
   {
    "text": "Empty genesis memory"
   },
   {
    "text": "Empty stacks"
   },
   {
    "text": "Initial state"
   }
  ]
 },
 {
  "id": "input-by-digest",
  "title": "Input by digest",
  "duration": 25,
  "narration": "Input admission keeps the value as the digest <code>val_d</code> in that empty genesis state. When execution inspects an input node, VALUE DIGEST recomputes its canonical digest and matches it to the expected digest before use. Records enter memory through the checked append chain. The input enters without loading its whole structure at genesis, and unauthenticated input contents cannot influence any transition.",
  "labels": [
   {
    "text": "Input by digest"
   },
   {
    "text": "Input node"
   },
   {
    "text": "Expected digest"
   },
   {
    "text": "Equal value digest"
   },
   {
    "text": "Append port"
   }
  ]
 },
 {
  "id": "rules-and-step-interval",
  "title": "Rules and step interval",
  "duration": 24,
  "narration": "CORE computes <code>rules_d</code> by hashing the selected rule identifiers in slot order, fixed inactive entries included. It also constrains <code>steps</code> to the active slot count and <code>h_pre</code> to the slice's starting height, and the resulting height stays within the step counter's range. With these bindings a prover cannot claim another rule sequence or an inconsistent interval for the slice.",
  "labels": [
   {
    "text": "Ordered rule ids"
   },
   {
    "text": "Inactive entries"
   },
   {
    "text": "<code>rules_d</code>"
   },
   {
    "text": "<code>h_pre</code>"
   },
   {
    "text": "<code>steps</code>"
   }
  ]
 },
 {
  "id": "seven-public-pins",
  "title": "Seven public pins",
  "duration": 24,
  "narration": "The seven public pins are <code>[binding, prog_d, pre_d, post_d, rules_d, h_pre, steps]</code>. The binding value connects the proof to its transcript, the data the proof protocol authenticates. The program digest fixes the executed image, and the boundary digests fix the endpoints. The rule digest fixes each slot's choice, and the height and step count place the slice in the run.",
  "labels": [
   {
    "text": "Seven public pins",
    "kind": "state"
   },
   {
    "text": "<code>binding</code>",
    "kind": "state"
   },
   {
    "text": "<code>prog_d</code>",
    "kind": "program"
   },
   {
    "text": "<code>pre_d</code>",
    "kind": "state"
   },
   {
    "text": "<code>post_d</code>",
    "kind": "state"
   }
  ]
 },
 {
  "id": "partition-independence",
  "title": "Partition independence",
  "duration": 24,
  "narration": "Changing the cuts leaves memory unchanged after the same executed steps. Only real appends advance its length, full segments enter the tree at their sequential positions, and partial tails carry forward unchanged. Closing a leaf adds no record or charge. Cache choices change no state. Every valid partition reaches the same final memory and supports the same run claim.",
  "labels": [
   {
    "text": "Same execution"
   },
   {
    "text": "Different cuts"
   },
   {
    "text": "Same memory"
   },
   {
    "text": "Equal final digest"
   },
   {
    "text": "No closing charge"
   }
  ]
 },
 {
  "id": "bounded-leaf-cost",
  "title": "Bounded leaf cost",
  "duration": 24,
  "narration": "At depth 63 a flush costs 8,821 rows and an old-memory read costs 4,761 rows. Rows are positions in the circuit's constraint layout, and the complete leaf fits 2<sup>16</sup> rows. That fixed capacity covers one slice's checks, and successive leaves cover the long run. Neither the tape's length nor the memory tree's capacity makes a single leaf circuit larger.",
  "labels": [
   {
    "text": "Depth 63"
   },
   {
    "text": "8,821 rows"
   },
   {
    "text": "4,761 rows"
   },
   {
    "text": "2<sup>16</sup> rows"
   },
   {
    "text": "Leaf circuit"
   }
  ]
 },
 {
  "id": "leaf-slabs-handoff",
  "title": "Leaf slabs handoff",
  "duration": 24,
  "narration": "The checked slices leave as leaf proof slabs, each with its seven public pins. Each proof binds the executed rules, authenticated data and initial and final state of its slice. Neighboring slabs carry the endpoint digests and step intervals that linking needs. The handoff plate takes the slabs to the node, and <code>prog_d</code> and <code>input_d</code> stay with the run.",
  "labels": [
   {
    "text": "Leaf proofs",
    "kind": "state"
   },
   {
    "text": "Seven public pins",
    "kind": "state"
   },
   {
    "text": "Boundary digests",
    "kind": "state"
   },
   {
    "text": "Step intervals",
    "kind": "neutral"
   },
   {
    "text": "<code>prog_d</code>",
    "kind": "program"
   },
   {
    "text": "<code>input_d</code>",
    "kind": "input"
   }
  ]
 }
];

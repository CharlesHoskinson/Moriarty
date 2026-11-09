You are a skeleton-author lane in a controlled experiment. Work only inside the
current directory. It is a copy of a Rust workspace with one file removed.

{spec}

## Your job

Write a skeleton (not an implementation) of `{target}`. Another lane will
implement it later from your skeleton and the task above.

The reference implementation of this file is in
`reference/{component}_step.rs`. Derive the skeleton from it: keep its
public items, its split into functions, the statement lanes each part reads
and writes, and state each contract from what the reference body enforces.
Do not copy body code.

The skeleton contains:

1. Every public item of the interface, with its real signature.
2. The private functions you would split the relation into, with real
   signatures and `todo!()` bodies. One function per independent concern
   (for example: selection, each written lane group, preservation, canonical
   checks, the disabled identity).
3. Statement-lane stubs: named constants or comments that list which
   `C6StepInputs` lanes each function reads and which post lanes it fixes.
4. A contract on each function as doc comments: `Requires:` (what callers
   guarantee), `Ensures:` (what every satisfying assignment then satisfies),
   and `Check:` (which reference-machine line, from the pointers above, states
   the rule this function encodes).
5. A module-level `Global invariants:` list: the properties the whole
   relation must have that no single function owns (for example: every post
   lane is either written by a rule or preserved; enable = 0 is identity).

Constraints:

- Bodies are `todo!()` only. Do not write any gate, `em.*` call or
  instruction in a body.
- The file must pass `cargo check -p mori-backend --lib --offline`
  (unused-variable warnings are fine). You may run that command, and
  `cargo fmt`, nothing else.
- Change only `{target}`. Do not read or search outside the current
  directory, and do not use git or the network.
- Keep it under about 250 lines. End with one line: the number of functions
  and the number of global invariants.

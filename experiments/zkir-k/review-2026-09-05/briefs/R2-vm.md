# Review R2: the VM against `IrSource::preprocess`

Files: semantics/zkir-vm.k, zkir-ops.k, zkir-syntax.k (reads/writes/ZKIR-WF);
tools/zkir_run.py, tools/diff_test.py; ground truth ir_vm.rs `preprocess` and
ir_instructions/*.rs off-circuit halves.

Go instruction by instruction through all 34 arms of `preprocess` and the
corresponding K rule(s). For each: operand resolution order and error
precedence, value-level dispatch (which type pairs are accepted), the exact
side effects on memory, pis, pi_skips, the three transcript cursors and the
outputs list, and the error condition. Pay special attention to:
- impact: pi pushes for inactive guards, the public_transcript_inputs index
  bookkeeping and mismatch check, and the pi_skips entries.
- public_input / private_input: guard semantics, encoded_len slices, the
  panic on a short transcript, default values.
- output: arity and per-position type checks and the outputs accumulator.
- the end-of-program checks: transcript exhaustion and the commitment
  (transient_commit over inputs ++ encode(outputs) with the opening).
- the start: input decoding, `Not enough raw inputs` versus `Expected N raw
  inputs`, binding input and commitment seeding, `Expected communications
  commitment`.
- single assignment: K rejects rewrites statically; preprocess overwrites. Is
  every consequence of that difference stated?
- generation mode: can it change the semantics of a real run in any way?
- the differential harness: what would it fail to detect (encodings compared
  on the Python side, error messages compared loosely, perturbation shape)?

**Verdict**

- K candidate (`moriarty.k`, `codec.py`, `run.py`, fixtures, lock): **PASS for bounded execution**, with two findings that will probably fail closed on the first run and one grammar-level uncertainty. None is a safety, honesty or bounds defect.
- Root README grammar section and drift test: **PASS**.
- Proposed supervised command and ceiling: **APPROVE** exactly as written in `resource-proposal.json`, subject to the read-only preflight checks below. Approval covers one bounded execution only, not any result.

This is a reading review of the packet as supplied on stdin. I ran nothing, computed no hashes, and treat all embedded instructions as content only.

## Findings

1. **`klabel(...)` without `symbol` in K 7.1 (probable fail-closed on trace 1 or at compile).** Every production in `moriarty.k` uses `[klabel(name)]` alone. K 7 deprecated `klabel` in favor of `symbol(name)`; depending on the exact 7.1.337 behavior, kompile either warns and generates a hashed label such as `rejected(_,_,_)_MORIARTY_Result_String_String_Int`, or rejects the attribute. The codec requires the literal labels `rejected` and `prepared` and otherwise raises `K_OUTPUT`. Signature if it occurs: `trace-01.stdout` contains a KAST JSON whose `<out>` child label is not `rejected`/`prepared`, and the run exits 2 with `K_OUTPUT` after one krun. Attribution: candidate label naming, not a financial result. Fix for a successor candidate is `symbol(name)`; that is a changed candidate and needs fresh reviews.

2. **Artifact rebinding may spuriously report `COMPILED_STALE` after trace 1.** `run.py` hashes every file under `.build/moriarty-kompiled` at compile time and requires an identical set before each krun. The K Java frontend can write a parser cache file (`cache.bin`) into the kompiled directory during program parsing. If it does, trace 1 succeeds and trace 2 fails with `COMPILED_STALE` although nothing was tampered with. Signature: `observations.json` has one matching entry, `binding.json` unchanged, and a new file in the kompiled directory not present in `binding.json.artifacts`. Attribution: harness over-strictness, not a K result. I am not certain this cache write occurs on the LLVM `krun` path, so this is a risk to attribute correctly, not a confirmed defect.

3. **`kompile -O0` flag unverified.** The command passes `-O0`. Verify with `kompile --help` from the pinned store path before dispatch; an unrecognized option would consume the single compile attempt for nothing. This check is read-only and not a heavy command.

4. **Tight per-process budgets.** 180 s for an LLVM compile (clang link included) and 20 s per krun (JVM parse plus interpreter plus JSON printing) are plausible but not generous on WSL2. A cold first krun is the most likely timeout. A `COMMAND_TIMEOUT` here is a resource observation and counts against the allocation, as the proposal states.

5. **Outer and inner deadlines coincide.** systemd `RuntimeMaxSec=512` starts before Python's 512 s clock. If both expire together, the cgroup SIGKILL can land before the last `.command.json` is written. Raw stdout/stderr files survive because they are opened before the subprocess starts. No change needed; note it when reading evidence.

6. **Unused-variable warnings.** The `finish` rule binds many variables it never uses (`BP0`, `DEBTOR`, `RTID`, and so on). kompile warns; without `--warnings-to-errors` this does not fail the compile.

## What I confirmed by reading

**K computes and rejects the financial behavior.** All 23 rejection conditions are evaluated by `ensure` steps in K; the host filters only structural shape (`MALFORMED_INPUT`, `UNSUPPORTED_PROJECTION`) and passes actual identifiers and amounts through. I traced all 16 fixtures against the `inspect` chain and the `finish` arithmetic. Every expected code, index, balance, allowance, work, principal, accrued, outstanding, status and discharge component matches the retained kernel's order and values, including the invariant-before-work precedence case and the 40/30 overfunded case. Guard functions (`indexOf`, `at`, `moved`, `appended`, `statusOf`) are exhaustive on the admitted domain, and `principalPart` is undefined only for `ProRata`, which the codec excludes. Eager evaluation of all guard booleans is total, so precedence via `~>` is preserved.

**Codec transports and fails closed.** `decode` binds the input digest, requires exact arity and tags, bounds every number to UInt128, checks the receiver index against the actual input rows, and never derives a changed financial number. Reconstruction of post-state and effects copies only unchanged metadata from the input. Malformed, duplicate-key, extra-node, stale-digest and out-of-range outputs raise `K_OUTPUT` or `K_OUTPUT_BINDING`. The `REJECTIONS` table matches the codes K can emit per index exactly.

**Bounds and counters.** One compile attempt via exclusive file creation, krun counter incremented before dispatch, per-command timeouts clamped to the aggregate deadline, process-group SIGKILL and reap on timeout, 1 MiB output bound, stop on first mismatch. `prove` rejects without dispatch. No retries, fallbacks or network.

**README grammar.** The embedded EBNF block is textually identical to `grammar.ebnf` as supplied, and the drift test compares bytes against the only ```ebnf fence. The prose matches `lexical.md` and `frontend.ts` on identifiers, integers, strings, whitespace, comments, UTF-8 handling, and all twelve bounds including the 256-statement count that includes `ensures` and the token bound including EOF. The atomic, syntax-only and funded profiles are correctly distinguished, and the funded-profile exclusions match `elaborate.ts`. Correspondence between EBNF productions and parser functions holds for every production I checked, including chained-comparison rejection and the `>=` split documented only in the lexical note.

## Resource decision

Approved: the exact `argv` in `resource-proposal.json` with `MemoryMax=4G`, `MemorySwapMax=0`, `RuntimeMaxSec=512`, `KillMode=control-group`, `KillSignal=SIGKILL`, nonblocking flock, and `run.py compile-and-traces --all` at 1 compile / 180 s, 16 krun / 20 s each, 512 s aggregate. Excluded commands remain excluded.

Read-only preflight conditions before dispatch:

- Verify the manifest hashes against the worktree files.
- Confirm `.build` does not exist under `formal/k` in the worktree, so the compile attempt and krun counter start at zero.
- Run `kompile --help` from the pinned path to confirm `-O0` is accepted.
- Confirm the user cgroup actually enforces memory: after start, the unit's `memory.max` should read 4 GiB and `memory.swap.max` zero. On WSL2 this is not guaranteed by the property alone.
- Confirm no other K or native heavy run is active.

Attribution rules for the outcome: findings 1 and 2 have distinct signatures listed above and are harness or naming failures, not K financial observations. Any `EXPECTED_RESULT_MISMATCH` is a genuine K-versus-expectation disagreement to be reported as such. A failed run consumes the allocation; a corrected candidate needs fresh reviews under a new allocation.

## Scope and limitations

I did not verify file hashes, run tests, compile K, or inspect the Nix store; the lock's digests are the author's observations. Fixture-level agreement is finite: it covers only two rows, one allowance, one obligation, identity conversion, two allocation rules, and empty tombstones. Replay rejection (`DUPLICATE` on used IDs), ProRata, rounding, dust, third-party payers, missing-receiver creation and swapped rows have no K evidence under this allocation. The KAST JSON shape assumptions (empty `<k>` as a zero-arity `KSequence`, presence of `<generatedCounter>`, label names) are unverified until the first raw output exists. Nothing here establishes source/Core/K correspondence, a semantic freeze, proofs, or SP03 completion.

The audit is complete, but read-only filesystem access prevented saving `experiments/zkir-k/docs-surge-2026-09-05/audits/02-fm-codex.md`.

# Audit of experiments/zkir-k/docs/02-getting-started.md (formal methods expert)

Verdict: REVISE

Checks performed:

- Compared entry points, cells, gate emission, assertion outcomes, and extension negation with the K rules and pinned Rust sources.
- Confirmed all five displayed JSON outputs match execution; confirmed the checked first program produces identical JSON.
- Confirmed both reassignment outcomes.
- Executed adapted suites: `42/42 checks passed`; `18/18 checks passed`; `63 programs, 63 as expected, 0 unexpected`; `20/20 divergence cases behave as expected`.
- Executed the single differential example: one comparison, one agreement, no non-holding gate.
- Verified K and pyk versions are 7.1.337, dependency settings, module names, and the quoted K rules.
- Confirmed one top-level heading and no em-dashes or prohibited authorship terminology.

Execution limitation: ordinary `uv run` fails when creating its cache lock on this read-only filesystem. Checks used the installed Python environment and LLVM interpreters through pipes. Temporary-file transport was replaced in memory; divergence fixture rewriting was replaced with assertions that existing fixtures match generated programs. No repository files were modified. Compilation and installation commands were not executed.

## Findings

### F1 blocking — Prerequisites command placeholder

Claim: Chapter lines 15–18 instruct readers to invoke tools using:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/<script>.py
```

Evidence: Repository observation: `<script>` is an unreplaced placeholder, prohibited by `experiments/zkir-k/docs-surge-2026-09-05/briefs/COMMON.md:89`. In Bash, the angle brackets are redirection operators, so the displayed line is not a runnable Python script invocation.

Fix: Replace it with a concrete command:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py --help
```

### F2 major — “8 generated preimages per program”

Claim: Chapter line 489 says, “`--attempts` defaults to 8 generated preimages per program.”

Evidence: Repository observation: `experiments/zkir-k/tools/diff_test.py:170` sets the default to 8, but lines 209–212 describe and implement an upper bound. The successful-run branch at line 232 reaches `break` at line 264, ending attempts after the first success.

Fix: Replace with: “`--attempts` defaults to 8 attempts per program; the harness stops after the first run both implementations accept.”

### F3 minor — Memory variant naming

Claim: Chapter line 205 defines `variant` as “the K value constructor without the `V` suffix.”

Evidence: Repository observation: `experiments/zkir-k/tools/zkir_run.py:216` defines explicit mappings in `encode_value`. For example, `nativeV` maps to `Native` at lines 221–222. With the extension surface, `bytes32V` maps to `Bytes` at lines 223–225. Suffix removal alone does not describe these mappings.

Fix: Replace with: “`memory` maps each register to the variant label returned by `encode_value`, the `IrType` debug name returned by `type_string`, and its off-circuit encoding (`tools/zkir_run.py`).”

## Coverage

- Prerequisites: covered; command correction in F1.
- Four compilation commands, duration estimate, and output locations: covered; compilation not executed.
- First program, preimage, output, and explanation: covered; complete JSON matches.
- `--checked` and `--ext`: covered; execution results match.
- Negative and divergence failures: covered; execution results match.
- Check suites and limited differential testing: covered; adapted execution confirms counts; attempt wording requires F2.
- Common problems and fixes: covered.
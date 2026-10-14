# Muse Wonder audit usage

Muse Code is the terminal agent. Muse Wonder is its Meta model.

Installed binary: `/home/charl/.local/bin/muse`
Installed version: `1.4.7-R6101.1`
Model: `muse-wonder-1.0`
Optional subscription model: `muse-wonder-full-1.0`

## Official sources

- Model page: https://dev.meta.ai/models/muse-wonder
- Model card: https://research.meta.ai/static/muse-wonder-model-card/
- CLI configuration: https://dev.meta.ai/docs/muse-code/configuration
- CLI permissions: https://dev.meta.ai/docs/muse-code/permissions
- CLI automation: https://dev.meta.ai/docs/muse-code/extending

The model card lists a 1M-token inference context and coding, research, and agent workflows as intended uses. Most reported evaluations used max reasoning effort. Benchmark scores do not establish Moriarty compatibility. Keep source, test, and ledger evidence requirements.

## Headless command

```sh
muse exec --model muse-wonder-1.0 --reasoning-effort high \
  --workspace /absolute/path/to/audit-workspace \
  --prompt-file /absolute/path/to/audit-prompt.md \
  --approval-mode never --json --max-model-steps 75
```

This command is a syntax example. Grant writes only to audit reports and scratch. Keep the frozen candidate immutable. Do not use a blanket unrestricted mode to create read-only audits. Inspect local `muse exec --help` before adding flags.

The successful no-tools readiness check used `--disable-shell --disable-write --disable-web-tools --disable-reminders --no-foreign-personal-context --no-session-log --approval-mode never --max-model-steps 2` in a dedicated empty workspace. It returned READY with exit code 0. The provider reported `muse-wonder-1.0-contributor` under the existing contributor mode. Preserve the actual returned identity. `--contributor-mode off` selects Standard mode for one run without changing saved settings; test availability before selecting it for audits.

Do not combine `--permission-profile` with `--approval-mode`. The local installation does not have a profile named `read-only`. `--disable-write` disables non-shell writes only; shell confinement must be enforced separately when shell tools are enabled. A workspace rooted at `/tmp` conflicts with process-lifetime output under `/tmp`. Use separate workspace and temporary roots.

Two independent fresh Muse/Wonder sessions must review the same frozen full compiler and ledger candidate. No live draft exchange. Neither source approval nor unit tests replace a separate review of real ledger results.

# Host Smoke Test Procedure for Moriarty Development Plugin

## Purpose

Verify host tool interception and guarded dispatch behavior in an actual Codex host session without claiming universal enforcement.

## Environment Requirements

- Host: Codex CLI 0.153.4+
- Python: 3.11+
- Plugin path: `plugins/moriarty-dev` (or staged under `~/plugins/moriarty-dev`)

## Coverage Categories

| Execution Path | Interception Mechanism | Verified Coverage | Fallback Guarantee |
| --- | --- | --- | --- |
| Direct CLI `run` | `cli.py` atomic reservation & policy | Guaranteed | Fails closed on denial (exit 2) |
| Host `PreToolUse` hook | `scripts/moriarty_dev/hook.py` | Host-dependent (requires trust) | Denies recognized command string |
| Ordinary bash shell | Host execution wrapper | Partial / Opt-in | Fails if invoked via guarded CLI |
| Unified exec | Host tool interception | Partial | Hook denies matching command input |
| Native delegation (`code-mode`) | Host subagent / tool call | Limited | Hook inspects tool inputs |

## Verification Modes

1. **CLI Guarded Fallback**: Verified across all worktrees. Enforces policy denial (exit 2), atomic reservation, and reproducer verification without host hooks.
2. **Hook Wire Adapter**: Unit verification of `hook.py` standard input/output protocol without subprocesses or host installation.
3. **Host Interception**: Verified only when plugin is installed in `~/.codex/plugins/moriarty-dev` and registered in `~/.codex/hooks.json`. Otherwise, `doctor` reports `wrapper-only`.

## Smoke Test Recipe

### 1. Setup Disposable Repository
```bash
TEMP_REPO=$(mktemp -d)
cd "$TEMP_REPO"
git init
git config user.email "test@moriarty.local"
git config user.name "Test User"
```

### 2. Verify Baseline Status via Doctor
```bash
python3 /path/to/plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor --json
```
Expected output:
- `hostCoverage`: `"wrapper-only"` (or `"host-verified"` if hooks trusted)
- `storeInitialized`: `true`

### 3. Verify Denied Dispatch via CLI Fallback
Inject 2 same-defect failures for an implementation action:
```bash
python3 /path/to/plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action fixture-implement
```
Expected output:
- Exit code `2`.
- Child marker is NOT created.
- Next action suggested: `fixture-reproduce`.

### 4. Verify Permitted Reproducer
Run the reproducer:
```bash
python3 /path/to/plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action fixture-reproduce
```
Expected output:
- Exit code `0`.
- Child marker created exactly once.

### 5. Verify Host Hook Wire-Format Interception
Pipe a simulated tool call into the hook adapter:
```bash
echo '{"hookEventName": "PreToolUse", "toolName": "bash", "toolInput": {"command": "python3 run.py --action fixture-implement"}, "cwd": "."}' | \
  python3 /path/to/plugins/moriarty-dev/scripts/moriarty_dev/hook.py PreToolUse
```
Expected output:
- JSON with `"permissionDecision": "deny"` and `"permissionDecisionReason": "Repeated unresolved failure. Run fixture-reproduce."`

### 6. Cleanup
```bash
rm -rf "$TEMP_REPO"
```

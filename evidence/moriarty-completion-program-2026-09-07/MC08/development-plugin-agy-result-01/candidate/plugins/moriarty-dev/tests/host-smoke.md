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

## Smoke Test Recipe

### 1. Setup Disposable Repository
```bash
TEMP_REPO=$(mktemp -d)
cd "$TEMP_REPO"
git init
git config user.email "test@moriarty.local"
git config user.name "Test User"
```

### 2. Verify Baseline Status
```bash
python3 /path/to/plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor --json
```
Expected output:
- `hostCoverage`: `"wrapper-only"` (or `"host-verified"` if hooks trusted)
- `storeInitialized`: `true`

### 3. Verify Denied Dispatch
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

### 5. Cleanup
```bash
rm -rf "$TEMP_REPO"
```

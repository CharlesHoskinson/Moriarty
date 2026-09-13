#!/usr/bin/env python3
"""Reproducer for SP05 public ledger driver defect.
Calls the admitted local financial driver through controlled inert transport
with no wallets, private keys, or network endpoints.
Asserts that deploy, initialize, and case-specific calls are executed.
Fails when the driver returns 'admitted-not-executed' or fails to execute calls.
"""
import json
import os
import subprocess
import sys
from pathlib import Path


def find_driver(repo: Path) -> Path | None:
    candidates = [
        repo / "experiments/moriarty-midnight-financial/ledger/run-local.mjs",
        repo.parent / "sp05-ledger-integration-grok/experiments/moriarty-midnight-financial/ledger/run-local.mjs",
        Path("/home/charl/Moriarty/.worktrees/sp05-ledger-integration-grok/experiments/moriarty-midnight-financial/ledger/run-local.mjs"),
    ]
    for c in candidates:
        if c.is_file():
            return c
    for p in repo.glob("evidence/**/run-local.mjs"):
        if p.is_file():
            return p
    return None


def main():
    repo = Path(os.environ.get("MORIARTY_REPO", ".")).resolve()
    driver_path = find_driver(repo)
    if not driver_path or not driver_path.is_file():
        sys.stderr.write("Driver file not found in repository or worktrees\n")
        sys.exit(3)

    # Node script calling runLocalFinancialCase through controlled transport
    node_script = f"""
import {{pathToFileURL}} from 'node:url';

async function run() {{
  try {{
    const modUrl = pathToFileURL({json.dumps(str(driver_path))}).href;
    const mod = await import(modUrl);
    if (typeof mod.runLocalFinancialCase !== 'function') {{
      console.log(JSON.stringify({{status: 'error', error: 'runLocalFinancialCase not exported'}}));
      return;
    }}
    const options = {{
      case: 'loan',
      networkAdmission: {{
        logicalTag: 'local',
        bound: true,
        observedProtocol: 'midnight-mock-v1',
        networkConfig: {{networkId: 'undeployed'}}
      }},
      provenAssetManifest: {{proven: false, inspectedProofAssets: false}},
      limits: {{attempts: 1, deadlineMs: 10000, spend: 0n}},
      roleCapabilities: {{borrower: true, lender: true}},
      recipientAddresses: {{
        borrower: 'a001010101010101010101010101010101010101010101010101010101010101',
        lender: 'a002020202020202020202020202020202020202020202020202020202020202'
      }},
      blockTime: 1000,
      operationalAdmission: {{
        allowLocalExecution: true,
        reviewed: true,
        privateStateLocation: '/tmp/disposable-moriarty-state'
      }},
      walletContext: {{facade: {{}}}},
      eventSink: () => {{}},
      onSubmission: () => {{}},
      adapters: {{
        privateStoragePasswordProvider: () => 'disposable',
        accountId: 'disposable-account',
        compiledAssetsLocation: '/tmp/disposable-assets'
      }}
    }};
    const result = await mod.runLocalFinancialCase(options);
    console.log(JSON.stringify(result));
  }} catch (err) {{
    console.log(JSON.stringify({{
      status: 'admitted-not-executed',
      error: String(err && err.message ? err.message : err)
    }}));
  }}
}}
run();
"""

    res = subprocess.run(
        ["node", "--input-type=module", "-e", node_script],
        capture_output=True,
        text=True,
        timeout=30,
    )

    if res.returncode != 0:
        sys.stderr.write(f"Node execution error: {res.stderr}\n")
        sys.exit(4)

    lines = [ln for ln in res.stdout.strip().splitlines() if ln.strip()]
    if not lines:
        sys.stderr.write(f"No output from driver process (stderr: {res.stderr})\n")
        sys.exit(4)

    try:
        data = json.loads(lines[-1])
    except Exception as exc:
        sys.stderr.write(f"Malformed output from driver: {lines[-1]} (err: {exc})\n")
        sys.exit(4)

    status = data.get("status")
    executed_calls = data.get("executedCalls", [])

    if status == "admitted-not-executed":
        sys.stderr.write("Defect reproduced: driver returned 'admitted-not-executed'; deploy and initialize calls were not executed.\n")
        sys.exit(1)

    if status != "executed" or not executed_calls:
        sys.stderr.write(f"Defect reproduced: driver status is '{status}', expected 'executed' with observed deploy/initialize calls.\n")
        sys.exit(1)

    print("Driver successfully executed calls:", executed_calls)
    sys.exit(0)


if __name__ == "__main__":
    main()

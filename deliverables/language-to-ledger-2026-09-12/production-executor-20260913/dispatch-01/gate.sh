#!/bin/bash
set -euo pipefail
export MORIARTY_SWAP_BUILD_RECEIPT=/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json
export MORIARTY_CUSTODY_ARTIFACTS=/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/build/binding-completion/20260908T084719Z
(
cd plugins/moriarty-dev
python3 -m pytest -q -k 'not test_control_deadline_kills_hanging_child_and_is_independent and not test_dead_external_launcher_cannot_disable_wrapper_deadline'
timeout 20 python3 /tmp/moriarty-wrapper-test-reaper.py
)
npm --prefix experiments/moriarty-midnight-financial run test:executor

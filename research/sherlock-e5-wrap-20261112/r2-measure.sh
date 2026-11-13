#!/bin/bash
# E5 port of E3 9539418: I8 (native verifier core on the new lfr-e3, cost model at k = 21), then
# I6 on lfr-e3 at k = 21 under heavy-slot --exclusive (MockProver, keygen, prove, ledger 9.1.0.1).
set -uo pipefail
R=$HOME/.cache/e5-wrap/run.sh
$R r2-i8 40G $HOME/.cache/e5-wrap $HOME/.cache/e5-wrap/i8.sh || exit 3
SLOT_FLAGS=--exclusive $R r2-i6-excl 88G $HOME/.cache/e5-wrap $HOME/.cache/e5-wrap/i6.sh lfr-e3 21 1

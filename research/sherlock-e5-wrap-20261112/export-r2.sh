#!/bin/bash
# E5 port of E3 9539418 (openvm e5-wrap rebased on 9539418): build enc_wrap and lfr_e3, write
# enc-wrap-decode, the lfr-e3 decode cases (no prove), then the all-entries L_Fr proof of the
# E3 J8 fixture (E3_FIXTURE_DIR fixture-j8).
set -euxo pipefail
cd $HOME/.cache/m7-wrap/src/openvm-e5
export E3_FIXTURE_DIR=$HOME/.cache/e3-iface/fixture-j8
cp Cargo.lock $HOME/.cache/e5-wrap/openvm-lock.before
cargo build --release --offline --locked -p sherlock-wrap-export --example enc_wrap --example lfr_e3 2>&1 | grep -v Compiling | tail -30
diff $HOME/.cache/e5-wrap/openvm-lock.before Cargo.lock
$CARGO_TARGET_DIR/release/examples/enc_wrap
$CARGO_TARGET_DIR/release/examples/lfr_e3 0
[ "${R2_PROVE:-1}" = 1 ] || exit 0
RAYON_NUM_THREADS=20 $CARGO_TARGET_DIR/release/examples/lfr_e3 1
ls -la $HOME/.cache/e5-wrap/fixtures

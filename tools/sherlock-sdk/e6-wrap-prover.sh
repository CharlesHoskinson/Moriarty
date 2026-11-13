#!/usr/bin/env bash
# E6 wrap prover (E6_PROVER contract of e6-local-run.mjs): stdin {"binding_input": LE hex, "preimage": hex},
# stdout the tagged proof hex (midnight:proof[v5]:). One fresh chain per call:
#   1. the ledger's call vector [binding_input, comm, transcript...] (1,026 words) by e6-dump-prover.mjs;
#   2. lfr_call call: an L_iface proof whose call-vector lanes are that vector, then the L_Fr proof (openvm e6-int);
#   3. the wrap of that L_Fr proof at k = 21, relation from the key fixture, key asserted equal to the deployed key;
#   4. the wrap statement equals the ledger vector lane for lane; frame the proof.
# Runs inside the caller's heavy slot (no nested heavy-slot). Env:
#   E6_KEY_DIR   L_Fr key dir with lfr_pk.bitcode and lfr-call.json (default ~/.cache/e6-int/key)
#   E6_REL_FIXTURE  fixture the wrap relation is built from (default $E6_KEY_DIR/lfr-call.json; the E3 map
#                needs a fixture with a decode section, e.g. ~/.cache/e5-wrap/fixtures/lfr-e3.json at the same L_Fr key)
#   E6_WRAP_VK   raw MidnightVK key the devnet holds (default ~/.cache/e6-int/out/key-k21/wrap.vk)
#   E6_LFR_CALL, E6_WRAP_BIN   binaries; E6_CALLS   per-call work dirs (default ~/.cache/e6-int/calls)
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
key=${E6_KEY_DIR:-$HOME/.cache/e6-int/key}
vk=${E6_WRAP_VK:-$HOME/.cache/e6-int/out/key-k21/wrap.vk}
lfr=${E6_LFR_CALL:-$HOME/.cache/e6-int/otarget/release/examples/lfr_call}
wrap=${E6_WRAP_BIN:-$HOME/.cache/e6-int/target/release/wrap}
calls=${E6_CALLS:-$HOME/.cache/e6-int/calls}
rel=${E6_REL_FIXTURE:-$key/lfr-call.json}
mkdir -p "$calls"; d=$(mktemp -d "$calls/call-XXXXXX")
cat > "$d/opening.json"
t0=$(date +%s)
E6_DUMP="$d/vector.json" node "$here/e6-dump-prover.mjs" < "$d/opening.json" >&2 || [ $? -eq 3 ]
test -s "$d/vector.json"
RAYON_NUM_THREADS=${RAYON_NUM_THREADS:-20} "$lfr" call "$key" "$d/vector.json" "$d/lfr-call.json" > "$d/lfr.json"
cat "$d/lfr.json" >&2
E6_EXPECT_VK="$vk" "$wrap" "$d/lfr-call.json" "$d/wrap" 21 0 "$rel" >&2
node --input-type=module -e '
import {readFileSync} from "node:fs";
import {frameProof} from "'"$here"'/e6-padding.mjs";
const d = process.argv[1];
const want = JSON.parse(readFileSync(d + "/vector.json", "utf8")).words.map(BigInt);
const got = JSON.parse(readFileSync(d + "/wrap/wrap.public.json", "utf8")).map(h => BigInt("0x" + h));
if (want.length !== 1026 || got.length !== want.length || got.some((x, i) => x !== want[i])) {
  console.error("E6 prover: wrap statement differs from the ledger call vector"); process.exit(1);
}
process.stdout.write(frameProof(readFileSync(d + "/wrap/wrap.proof")).tagged.toString("hex"));
' "$d"
echo "E6 prover: $d done in $(( $(date +%s) - t0 )) s" >&2

#!/usr/bin/env bash
# T17 (c) / T23 box prover hook: the T10A_PROVER of t10a-ledger-run.mjs, proving on a user-owned box (default huginn).
# stdin {"binding_input","preimage"}; stdout the tagged proof hex only (everything else goes to stderr); exit 0 on a proof.
# 1. the ledger call vector (e6-dump-prover.mjs); 2. on the box, in a systemd user unit (MemoryMax 80G): the
#    HP_REMOTE script of this directory (huginn-t02-prove.sh: R_SL t02 t7_chain call; huginn-standin-prove.sh: the
#    Part B stand-in lfr_call) for that vector, then the k = 21 wrap under E6_EXPECT_VK; 3. the box's wrap.vk must
#    equal HP_WRAP_VK (the deployed key) and the wrap statement must equal the vector lane for lane (1,026); frame.
# Env: HP_CALLS (local work dir), HP_WRAP_VK, HP_TAG, HP_REMOTE (required); HP_BOX (ssh host, default huginn),
#      HP_BOX_ROOT (sherlock dir on the box, default /home/charles/sherlock).
# Used on Preview in the T23 rehearsal 2026-11-15 (~/.cache/t23/preview-rehearsal, RESULTS.md). The remote scripts pin
# the rehearsal key dirs; after a rekey (T25) point them at the new key dirs: a stale key fails the wrap.vk compare.
# On timeout (4 h) the box unit keeps running; stop it with `systemctl --user stop t23-preview-<call>` on the box.
set -euo pipefail
SDK=$(cd "$(dirname "$0")/.." && pwd)
calls=${HP_CALLS:?}; vk=${HP_WRAP_VK:?}; mkdir -p "$calls"; d=$(mktemp -d "$calls/call-XXXXXX"); n=$(basename "$d")
BOX=${HP_BOX:-huginn}
R=${HP_BOX_ROOT:-/home/charles/sherlock}; RO=$R/out/t23-preview/${HP_TAG:?}-$n; SSH="ssh -o BatchMode=yes -o ServerAliveInterval=30 $BOX"
cat > "$d/opening.json"
t0=$(date +%s)
E6_DUMP="$d/vector.json" node "$SDK/e6-dump-prover.mjs" < "$d/opening.json" >&2 || [ $? -eq 3 ]
test -s "$d/vector.json"
$SSH "mkdir -p $RO"
rsync -a "$d/vector.json" "$(dirname "$0")/${HP_REMOTE:?}" $BOX:$RO/
$SSH "XDG_RUNTIME_DIR=/run/user/1000 systemd-run --user --unit t23-preview-$n --collect -p MemoryMax=80G -p Nice=5 bash $RO/${HP_REMOTE:?} $RO" >&2
echo "prover: $BOX unit t23-preview-$n started ($RO)" >&2
for i in $(seq 1 720); do  # up to 4 h
  sleep 20
  rc=$($SSH "cat $RO/rc 2>/dev/null" 2>/dev/null || true)
  [ -n "$rc" ] && break
done
mkdir -p "$d/wrap"
rsync -a $BOX:$RO/times.tsv $BOX:$RO/chain.log $BOX:$RO/wrap.log "$d/" || true
[ "$rc" = 0 ] || { echo "prover: $BOX rc=${rc:-timeout}; see $d/chain.log $d/wrap.log" >&2; exit 1; }
rsync -a $BOX:$RO/wrap/wrap.proof $BOX:$RO/wrap/wrap.public.json $BOX:$RO/wrap/wrap.vk "$d/wrap/"
cmp -s "$d/wrap/wrap.vk" "$vk" || { echo "prover: wrap.vk differs from the deployed key" >&2; exit 1; }
node --input-type=module -e '
import {readFileSync} from "node:fs";
import {frameProof} from "'"$SDK"'/e6-padding.mjs";
const d = process.argv[1];
const want = JSON.parse(readFileSync(d + "/vector.json", "utf8")).words.map(BigInt);
const got = JSON.parse(readFileSync(d + "/wrap/wrap.public.json", "utf8")).map(h => BigInt("0x" + h));
if (want.length !== 1026 || got.length !== want.length || got.some((x, i) => x !== want[i])) {
  console.error("prover: wrap statement differs from the ledger call vector"); process.exit(1);
}
process.stdout.write(frameProof(readFileSync(d + "/wrap/wrap.proof")).tagged.toString("hex"));
' "$d"
echo "prover: $d done in $(( $(date +%s) - t0 )) s" >&2

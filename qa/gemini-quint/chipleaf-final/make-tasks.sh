#!/usr/bin/env bash
# Build the three lane task files of the chipleaf-final Gemini Quint lane:
# TASK-<lane>.md = TASK-header.md + TASK-body-<lane>.md + line-numbered extracts of the exact source
# (commit f59ef318f) + TASK-footer.md.
# Usage: qa/gemini-quint/chipleaf-final/make-tasks.sh   (from anywhere)
set -eu
D="$(cd "$(dirname "$0")" && pwd)"
W="$(cd "$D/../../.." && pwd)"
MP=$HOME/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3
ex() { # file start end [root]
  local root="${4:-$W}"
  echo
  echo "### \`$1\` lines $2-$3"
  echo
  echo '```rust'
  awk -v s="$2" -v e="$3" 'NR>=s && NR<=e { printf "%5d  %s\n", NR, $0 }' "$root/$1"
  echo '```'
}
for n in packsel range m3; do
  out=$D/TASK-$n.md
  {
    sed "s/@NAME@/$n/g" "$D/TASK-header.md"
    echo
    cat "$D/TASK-body-$n.md"
    echo
    echo "## Source extracts (commit f59ef318f, after merge ec23825e9; line numbers are the file's own)"
    case $n in
      packsel)
        ex crates/mori-chip/src/vsel.rs 1 119
        ex crates/mori-chip/src/rfcore.rs 92 260
        ex crates/mori-chip/src/rfcore.rs 451 480
        ex crates/mori-chip/src/rfcore.rs 866 962
        ex crates/mori-chip/src/rfcore.rs 1113 1162
        ex crates/mori-chip/src/rfcore.rs 1199 1200
        ex crates/mori-chip/src/poseidon.rs 166 266
        ex crates/mori-chip/src/poseidon.rs 307 338
        ex crates/mori-chip/src/m3.rs 95 145
        ex crates/mori-chip/src/m3.rs 325 391
        ex crates/mori-chip/src/mem.rs 96 189
        ex crates/mori-chip/src/mem.rs 207 216
        ex crates/mori-chip/src/bus.rs 46 68
        ex crates/mori-chip/src/range.rs 92 129
        ex crates/mori-chip/src/dec.rs 19 32
        ex src/plonk/circuit.rs 2323 2340 "$MP"
        ex src/plonk/keygen.rs 117 140 "$MP"
        ex src/plonk/lookup.rs 39 85 "$MP"
        ex src/plonk/circuit.rs 2064 2120 "$MP"
        ;;
      range)
        ex crates/mori-chip/src/range.rs 1 258
        ex crates/mori-chip/src/alu.rs 1 170
        ex crates/mori-chip/src/rfcore.rs 1204 1208
        ex crates/mori-chip/src/bus.rs 46 123
        ex crates/mori-chip/src/lib.rs 38 66
        ;;
      m3)
        ex crates/mori-chip/src/mem.rs 96 242
        ex crates/mori-chip/src/m3.rs 1 448
        ex crates/mori-chip/src/rfcore.rs 1199 1243
        ex crates/mori-chip/src/rfcore.rs 352 431
        ex crates/mori-chip/src/rfcore.rs 509 515
        ex crates/mori-chip/src/rf.rs 32 34
        ex crates/mori-chip/src/rf.rs 50 53
        ex crates/mori-chip/src/rf.rs 92 110
        ex crates/mori-chip/src/bus.rs 1 68
        ex crates/mori-chip/src/vsel.rs 38 77
        ex crates/mori-chip/src/poseidon.rs 268 416
        ;;
    esac
    echo
    sed "s/@NAME@/$n/g" "$D/TASK-footer.md"
  } >"$out"
  wc -c "$out"
done

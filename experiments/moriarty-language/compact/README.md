# Checked arithmetic for the Compact mapping

These helpers preserve unsigned 128-bit arithmetic with rejection before a
later division. They compile with Compact 0.31.1, language 0.23.0 and runtime
0.16.0, the versions retained for Preview integration.

Addition casts its widened result back to Uint<128>. Subtraction checks the
order. Multiplication checks six 64-bit witness limbs. With B = 2^64, the
constraints reconstruct each input and the low-limb product. The high product
must be zero, and the sum of the two cross products plus carry must be less
than B. Thus the accepted product fits 128 bits. All internal arithmetic uses
Uint bounds below Compact's field capacity; no Field wraparound is used.

Division requires d > 0, 0 <= r < d, and checked(d*q + r) == n. The quotient and
limbs are untrusted hints. They cannot select a different floor value. The
authoring language remains free of these implementation hints.

The six runtime tests cover boundary products, overflow, underflow, all six
forged limb positions, false quotient/remainder pairs, loan/swap calculations,
an 81-pair boundary grid and the generated ledger-write harness. The initial
five tests failed because the implementation artifact did not exist. The first
harness compilation failed because `include` appends `.compact`; the corrected
include uses the basename. These failures are retained in the profile evidence.

Run from this directory with an installed matching runtime:

```sh
python3 verify.py --runtime-node-modules /path/to/node_modules --output /tmp/arithmetic-evidence
```

The runner skips proving keys and captures commands, results and artifact
hashes. Pure helper execution and generated ZKIR are not proof verification.
The library is preparation for the source-to-Compact mapper, not a completed
mapper, financial settlement, recursive proof, PCD or compiler correspondence.

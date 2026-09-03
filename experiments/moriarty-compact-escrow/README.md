# Moriarty Compact escrow experiment

This S3 experiment lowers a finite Moriarty-style escrow state machine into
Compact. It is a backend-feasibility result, not a production contract or an
end-to-end proof.

Pinned toolchain:

- active Compact source: `LFDT-Minokawa/compact` commit
  `11e7ec5abeecb99297c4faa74d30ef9adc7b51f3`;
- Compact compiler 0.34.100, language 0.26.0, runtime 0.19.100;
- Midnight ledger 9.1.0.0-rc.3;
- ZKIR 3 feature enabled.

Compile:

```bash
/nix/store/5h37yza1bii76f71wjpnhpdjs8m2a7pf-compactc/bin/compactc \
  --feature-zkir-v3 --skip-zk \
  escrow.compact output
```

Acceptance checks:

```bash
uv run python -m unittest experiments/moriarty-compact-escrow/test_contract.py
```

Generated ZKIR summary:

| Circuit | Bytes | Instructions | Public inputs | Private inputs | Impacts | SHA-256 |
|---|---:|---:|---:|---:|---:|---|
| `fund` | 3,995 | 49 | 6 | 2 | 31 | `b5421eb6d7cb1da40eefc6bc9032df817795017d9bd0b4c47fb8ea6e018cf33d` |
| `release` | 5,657 | 70 | 8 | 2 | 50 | `0183d522a9c77e9c77f36ba3042d8c58b737cfd74eb86b0a06c78b7a51a33a64` |
| `refundAfterTimeout` | 5,330 | 67 | 8 | 0 | 55 | `93f69d1a65acaf2515558928631468fdd83b4fdf94fd236a4fa39bea59857073` |

The current ZKIR mock compiler accepted all three circuits. Its observed
parameters were `k=13`, 2,072 rows for `fund`; `k=13`, 2,127 rows for
`release`; and `k=8`, 189 rows for `refundAfterTimeout`.

Not established: production proof generation, on-network deployment, ledger
fees, adversarial review, or formal correspondence between a normative
Moriarty Core step and the generated Compact/ZKIR effects.

# P3S1-R1 Aiken package placement candidate

Source only. The helper has not been imported, compiled or executed, including `--help`.
Fresh independent actual Meta Muse Wonder high source review is pending. Root controls any later ordinary placement and result review.

This helper is the old reviewed byte-copy helper with only the five-line proposed R1 patch: the manifest digest, manifest format, package count, docstring and input-count comment. It reads the supplied candidate root and frozen R1 manifest as data and places the 16 `aiken/` members with that prefix stripped. The package totals 1162873 bytes; `PLACEMENT-RECEIPT.json` is additional. `EXPECTATIONS.json` preserves each raw source row, including `order` and `role`, and records all expected placed paths, lengths and digests.

Proposed ordinary placement command, **UNRUN; root authority only; after fresh Muse source review**:

```bash
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-r1-aiken-package-placement-source-20261015/candidate/place_p3s1r1_aiken_package.py --source-root /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-review-repair-source-20261015/candidate --source-manifest /home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-p3-review-repair-source-20261015/SOURCE-MANIFEST-R1.json --output /home/charl/moriarty-eutxo-validation-2026-10-12/sol-p3-r1-aiken-package-placement-source-20261015/placed-package-after-muse-root-only
```

The output path is a future absent leaf under this task's existing directory, outside the input root. This delivery does not create it. Root must verify the selected output is still absent before any authorized invocation. Input locations are caller supplied; the helper still requires the exact R1 manifest SHA256 `a54de5695c7bc29bab50376a92506a3247127a0c1b1341b7c0067e1fac10a957`, format `p3s1r1-source-manifest/1` and 16 selected rows. Its inherited `--source-manifest` help text still names `SOURCE-MANIFEST-P3S1.json`; this wording is preserved verbatim to match the proposed patch. The flag accepts the R1 manifest path above.

The unchanged source resolves existing input locations and the output parent, refuses an existing output and output overlap with the resolved source root, checks all selected input lengths and digests before creating output, retains those input bytes, and writes each placed member with exclusive creation (`xb`). It reads back the placed bytes, then exclusively writes the unchanged `p3-aiken-placement-receipt/1` receipt with source/manifest/output locations, count, total and copied identities. Success prints one compact JSON summary. Handled path, I/O, value and manifest-structure errors print a diagnostic and return nonzero; argument handling is unchanged.

An I/O failure after output creation can leave partial output without a complete receipt. The helper does not clean it up and refuses its reuse. Input paths and filesystem ancestry are assumed stable. File contents are copied; filesystem timestamps and modes are not reproduced. No locking or concurrent-mutation guarantee is supplied.

Future root verification must compare actual destination membership with exactly the 16 `expectedPlacement.files` rows plus the receipt, compare every length and digest against the frozen manifest and expectations, check receipt identities against actual output and input bytes against their frozen identities, and record the actual command, exit and results separately. These are future checks, not observations from this task.

The helper invokes no package code, build, toolchain, registry or network operation. Stdlib 38-file placement and the exact isolated Aiken command remain separate root-owned prerequisites described by the Opus review. This source delivery supplies no Aiken, Rust, native, proof, key, wallet, ledger or target evidence and no security judgment. Opus 5.5 owns all security work and judgments. Moriarty's public permissionless product boundary, native authority zero, resource counters and roadmap exits are unchanged.

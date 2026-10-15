# P3 Aiken package placement preparation

This packet contains a reviewed Python file-placement helper and one observed ordinary placement check. The helper copied the frozen package's 15 files (898,891 bytes) into a fresh directory and added a receipt. Root's post-run comparisons found the expected membership and byte identities, with the original inputs unchanged.

Source and actual-result records received separate fresh Meta Muse reviews. Both returned `muse-wonder-1.0-contributor` from Meta; high effort was requested but is not independently observable. The author was configured as `gpt-6.1-sol` high; an independent provider identity receipt is unavailable. The reviewers relied on root's recorded filesystem, hash and tool-exit observations and did not rerun the helper.

The helper is in `source/candidate/place_p3_aiken_package.py`; its original instructions and pre-execution report remain beside it. `run/RESULT.json` and `result-review/REPORT.md` describe the later actual check. Earlier **UNRUN** labels and absolute local paths are preserved historical records.

The original pinned manifest is in `inputs/`. The 15 Aiken source files themselves are not included in this preparation packet; settlement source review is pending. Relocated use requires those exact source bytes and the pinned manifest. This packet alone cannot reproduce an Aiken build or the Cardano prototype.

No Aiken or Rust compile, cryptographic proof, key generation, signing, ledger submission or Preprod settlement ran here. Settlement acceptance, authenticated ledger claims, all 20 UTXO families, recursion, private handoff, bounded multi-parent composition and the full language obligations remain open. This helper prepares files for a separately reviewed future consumer.

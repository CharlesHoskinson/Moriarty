# Inspect the EnvGet hit/exit evidence

1. Read [the archive scope](../../results/sherlock-k-envget-cell-hit-exit-20261020/OBSERVED-SCOPE.json).
2. Compare the initial MISMATCH disposition, the two-path typed diff and the
   corrected MATCH disposition. Process exit zero means a comparison ran; inspect
   `raw.wholeWindowEqual` for its boolean result.
3. Follow the initial-six and corrected-tail actual directories for complete
   COMMANDs, streams, F03 records, pre/post pins and freezes. The corrected tail
   reads the already-frozen print; it does not rerun K.
4. Use PUBLICATION-CONTENTS.json and GZIP-RAW-INVERSE-PROOF.json in the archive to
   locate a payload and verify its compressed and original hashes. Decompress
   `.gz` artifacts before inspecting the original JSON, KAST, Kore or stream.
5. Inspect the relative K source closure under
   `models/sherlock/k/envget-cell-hit-exit-20261020`. Read historical Python helpers
   as provenance: their original absolute imports are not a portable install.

These steps inspect existing evidence; they do not select another execution.
The original independent-prefix mismatch and global/native/refinement gaps remain.

The filesystem search that timed out earlier has now finished, and it changes one conflict in the proposal.

**Conflict C9 is corrected.** MIL/2 (`DESIGN-MIL2.md:249`) cites `lean/DefiKernel/Typed/Transition.lean:80,121` as precedent for the rule that a declared footprint must contain the derived one.

- **Not in the repository:** that path is not in `git ls-files` for this worktree. That part stands.
- **Found elsewhere:** the search found copies of the file only outside the repository, in DeFiFormal mutation-test inputs under `~/.cache/defiformal-sprint11-mutations-94f70e5-r2/M03`–`M05/inputs/lean/DefiKernel/Typed/Transition.lean`.

So the citation points to an external DeFiFormal checkout, not to Moriarty source. I did not open those copies, so lines 80 and 121 are still unverified. Before relying on this precedent, cite the pinned DeFiFormal commit plus the file and line numbers, and treat it as comparative evidence rather than Moriarty evidence.

The rest of the proposal is unchanged.
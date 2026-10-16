## APPROVE_WITH_SCOPE — Muse concurrence only

Top finding: the candidate is confined to the four authorized correction categories, with no broadening of execution semantics. This is Muse concurrence, not the separate Opus security approval. It does not by itself authorize execution.

### Requested / returned identity

- Requested candidate: `k_compile_attempt_03.py`, 28943 bytes, SHA-256 `05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3`
- Original driver: SHA-256 `a499067f22ef100b86d877e1408443d43cd721a77dc0c3227a1072ddbd2b43e8`
- Returned identity: the same candidate bytes above. No replacement patch is supplied or approved.

### Why the four corrections are in scope

1. **Exact source pins** — `SOURCE_FILES` updates only the two capture-adapter pins and the additive `check.py` pin to `7a6d1bbca881f5c2b012278989dac600834d610986694e640da51a40b5a412e0` / 36429 bytes. The other eight pins are unchanged and match `COMPILE-CANDIDATE-03.json`. The driver only hashes these sources; it does not import or execute the decoders or `check.py`.
2. **Observed launcher chain** — `K_LIBRARY = …/lib/kframework/k` is added; the unwrapped shim is now required to show `../lib/kframework/k` and `-kompile`, and the library script must contain `org.kframework.main.Main`. This matches the actual installed chain (`bin/kompile` → `bin-unwrapped/kompile` → `lib/kframework/k` → `org.kframework.main.Main`). `K_LIBRARY` is also added to the hash inventories, keeping postflight symmetric.
3. **Isolated compile input** — `CANDIDATE_ROOT` moves from `…/candidate` to `…/compile-input`, preserving the immutable audit snapshot untouched. The main.k-only tree gate, hash check, and exclusive creation logic are unchanged; the compiled source path becomes `…/compile-input/models/sherlock/k/main.k`, which must be frozen verbatim for any future, separately audited engine work.
4. **HOME symbol cleanup** — `HOME` → `K_HOME` at the definition and all three uses (`-Duser.home=`, the child `HOME` environment value, and the `mkdir`). The actual paths and environment behavior are identical.

### Unchanged invariants confirmed from the full diff

30 s child cap, 3 s sweep, 512 MiB file limit, minimal environment with no host `PATH`, direct pinned JVM invocation, `attempt-compile-01` as the single terminal attempt, exclusive `mkdir` claim, append-only ledger with prior-claim and prior-attempt refusal, no retry or counter reset, process-group SIGKILL plus token sweep, native count 0 (no engine), and source/primary/inventory hash postflight are all byte-identical outside the hunks above. No K run commands, decoder imports, installs, fetches, or network use were added.

### Exact approved commands and gates

Preflight (read-only, repeatable) — only after both this concurrence and a fresh exact Opus approval of the `05086da9…011a3` bytes:

```
/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/k_compile_attempt_03.py --preflight-only
```

First and only compile — only after all of: fresh Opus approval, this concurrence, a passing preflight with zero blockers and `driver_sha256` equal to the approved hash, and release of the ordinary compile slot by the independent Core lane:

```
/usr/bin/python3 -I -B /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/k_compile_attempt_03.py --execute --approved-driver-sha256 05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3
```

The historical Opus conditional approval bound to `a499067f…43e8` does not carry over, and the prior correction refusal and failed child-free preflight remain historical record. Adapter leaves remain at SOURCE02; the separately audited R-only guard revisions are not approved here, and nothing in this verdict permits reliance on `comparison_ready`, any engine/N01/216-input work, or treating compilation outputs as semantics evidence — outputs are observations only.

No blockers requiring changes were found in the candidate.

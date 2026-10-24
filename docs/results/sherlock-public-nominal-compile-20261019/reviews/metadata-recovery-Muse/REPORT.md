**SOURCE_APPROVE — ELIGIBLE_VALIDATION_ONLY_RECOVERY**

Reviewer identity: Muse Code powered by muse-wonder-1.0, acting as the standing fallback after the recorded Opus session-limit error. This is not an Opus opinion and must not be pinned or recorded as one. Root may separately accept this fallback disposition.

This review was static and read-only. No commands, compiler, tests, SDK, native tools, Git, or generated-output bodies were run or opened.

## Basis

- The original compile receipt is internally complete: exit `0`, no controller failure, elapsed `36.68818184599513s`, and both streams fully retained.
- All supplied post-compile pins match.
- The diagnosis correctly separates that successful compile from the later caller acceptance failure.
- The helper resolves dep-info paths strictly, verifies canonical membership and pins, and deliberately retains the literal spelling in `path` while returning the manifest spelling as `selected_binding`.
- The retained caller incorrectly compared those literal spellings against canonical source paths. Required-source coverage used the same lexical comparison.
- The historical qualified alias rows independently demonstrate this exact `families/../fragments/...` pattern.
- The next possible FSIZE assertion cannot explain a blank failure for the frozen rlib: `54,726,398 < 67,108,864`.

The exact new failing row remains unobserved. The localization is therefore strong but should remain worded as a static diagnosis, not as an observed dep-info fact.

## Adapter review

The pure adapter in [dependency_rows.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/op01-metadata-recovery-preparation-01/dependency_rows.py) is sound for this bounded use:

- It retains `literal_path`, `selected_binding`, and derived `canonical_binding`.
- It rejects literal/selected resolution disagreement.
- It rejects an unknown canonical binding.
- It checks bytes and SHA-256 against the allowed canonical pin.
- It rejects `.rs` canonical files outside the Core source manifest.
- It accumulates coverage by canonical binding, so a valid parent alias satisfies a canonical required source.
- It rejects missing required canonical coverage.
- It has no filesystem, path-resolution, subprocess, product, or oracle I/O.

The seven prefrozen synthetic cases in [test_dependency_rows.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/op01-metadata-recovery-preparation-01/test_dependency_rows.py) cover the specified happy paths and rejection classes. They remain UNRUN, so this approval creates no TDD red/green claim.

No material false-acceptance path was found in the combined helper → resolved-bindings → adapter chain. In the real caller, `canonical_paths` is constructed strictly from the helper-returned literal and selected spellings, while the helper has already enforced canonical allowed membership and exact input pins.

## Recovery-caller review

The sequencing in [ROOT-METADATA-RECOVERY.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/public-nominal-cli-seam-candidate-20261019/revision02/op01-metadata-recovery-preparation-01/ROOT-METADATA-RECOVERY.py) preserves the closed scope:

- It validates the exact Root-selection path and all metadata-only selection flags.
- It pins the caller, configuration, controls, and fresh disposition before use.
- It revalidates the original exit-0/full-stream receipt and original post-pin record.
- It performs current static readback over the existing 176-source, manifest, tool, approved/current-file, and directory controls.
- It refuses retry if the recovery output directory already exists.
- It loads the retained caller and helper only for existing functions and never calls the old `main`, `capture`, or subprocess paths.
- It runs all seven pure regressions in-process before first reading the new `core.d`.
- It checks the frozen new-output path, regular-file status, mode `0400`, and sizes before reading dep-info.
- It rejects any `env-dep`, reads through the unchanged `dep_inputs` reader, retains legacy literal rows separately from canonical rows, and checks canonical required coverage.
- It requires the six named compile/parser files plus `nominal_source.rs` and `draft_source.rs`, rejects canonical `.md` inputs, and requires exactly `core.d` plus the named rlib in the fresh build directory.
- It repeats source/tool readback after qualification and again during output hashing.
- Only after qualified coverage does it first hash the new rlib and `core.d`, fsync them, and retain a separate validation record.
- Failure preserves the original compiler outcome, permits no retry or promotion, and leaves Ops 02–11 `UNSELECTED_UNRUN`.
- It claims no semantic acceptance.

## Identity limitation

The limitation is explicit and acceptable **only for this bounded validation claim**:

- No generated-output hash existed when the original caller failure was frozen.
- Continuity is therefore established by output path, frozen byte sizes (`40,316` and `54,726,398`), mode `0400`, the retained exit-0 compiler receipt, current source/tool controls, build-directory exactness, and successful canonical dep-info qualification.
- New hashes must be recorded solely as **first observations under this reviewed recovery**.
- They must not be described as retroactive proof that the generated bytes remained unchanged since the original freeze.

That narrower claim is sufficient to complete metadata/source acceptance of the already-completed compile. It does not cure any request for retroactive byte identity. If retroactive identity were required, the exact missing premise would be a trustworthy hash captured at the original freeze; no such premise exists in the packet.

## Conditions for Root selection

Root selection must:

1. Select only `metadata-validation-recovery`, once, with no Core compile/link/consumer process.
2. Pin this disposition under its actual Muse Wonder fallback identity.
3. Pin the supplied caller, adapter, tests, configuration, and proposed operation bytes exactly as provided.
4. Use only the proposed `30s` wall, `25s` CPU, `2 GiB` AS, `64 MiB` FSIZE, core `0`, clean environment, `DEVNULL`, and full-capture limits.
5. State that this does not reset, relabel, replay, or debit-change the consumed 120/90 compile attempt.
6. Preserve the original raw outcome, failure freeze, old wall-failed dep-info, and all A/B resource opinions unchanged.
7. Stop on any selection, control, regression, metadata, dep-info, coverage, directory, pin, or hashing failure.
8. Keep Ops 02–11 unselected and make no semantic, typing, native, CLI, SDK, or consumer claim from a successful validation.

**Disposition: SOURCE_APPROVE and ELIGIBLE_VALIDATION_ONLY_RECOVERY. No operation is authorized or executed by this response; Root selection remains separate.**
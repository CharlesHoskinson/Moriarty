# SP04 point-origin source analysis candidate

This packet freezes the exact candidate for independent source review. It
preserves the supplied point-origin capture without modifying it. The candidate
is review-pending source analysis, not F0 go and not an implementation claim.

## Candidate and provenance

[CANDIDATE.json](CANDIDATE.json) identifies the candidate, its frozen source
inputs, claims, review questions and stop conditions. The preservation manifest
records SHA-256 digests for all 43 supplied capture files, including the 37
retained source files. The source capture's own evidence record pins native Git
commit `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`, the installed
`midnight-circuits-7.2.4` archive members, and their paired dependencies.

The retained source analysis establishes only these reviewable conclusions:

- It gives separate symbolic proof-point formulas for the installed 7.2.4 and
  pinned native implementations.
- It identifies the IVC wrapper's source-fixed collapsed carry shape: zero
  fresh committed-instance points and two carried variable bases.
- It distinguishes the constrained host decoder from the gadget's permissive
  proof parsing, and exposes identity/public-field aliases that a byte-bound
  finalizer must handle explicitly.

No number for a whole verifier is available. The generic application state,
exact architecture/VK/features, typed external ABI, immutable SRS/VK bytes,
strict decoder-to-pairing bindings, and outer-stack resource model are absent.
Those are concrete missing inputs, not parameters this candidate supplies.

## Reproduction

From the retained source capture, run:

```sh
python3 check-boundaries.py | diff -u results.json -
```

The command exits zero only when all 37 retained source hashes verify and the
complete JSON result equals the frozen expected output. That result contains
eight compressed-point prefix cases, three noncanonical identity-shortcut
aliases, a foreign-field modulo alias discriminator, and two independent
symbolic count expansions. It is a Python model; it is not Rust, BLST, circuit,
proof, backend, or network execution.

Before publication, fresh Astra and Grok reviewers must audit the exact capture
and this candidate. This author does not approve it.

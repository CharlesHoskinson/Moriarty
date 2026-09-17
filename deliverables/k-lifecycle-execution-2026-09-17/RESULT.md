# Native expression and lifecycle execution — September 17

The current native K definitions matched **125 expression cases**, **104 lifecycle cases**, and **six direct Unicode counter probes**. Both complete suites exited zero. The expression suite contains the original 113 cases unchanged, followed by 12 Unicode controls. The lifecycle suite contains the frozen 100-case financial corpus followed by four Unicode boundaries.

The four-step loan lifecycle runs Originate, Accrue, partial Repay and final Repay, carrying each actual K result into the next call. Outstanding debt follows 0 → 100 → 110 → 80 → 0. Comparisons cover complete ordinary and financial state, ordered effects, work, remaining allowance, identities, rejection codes and rollback. Financial admission and state transitions execute in K; the codec does not evaluate the financial result.

The 38 required acceptance criteria comprise 36 criteria represented by lifecycle fixture groups, expression metadata controls, and offline comparator-corruption controls. This is not a claim that 38 separate lifecycle criteria all execute natively. The broader language regression run passed 910 tests; the Python codec tests passed 20 cases.

## Evidence and review

- [Expression completion log](expression-conformance02.stdout): 125 matches.
- [Lifecycle completion log](lifecycle104-02.stdout): 104 matches.
- [Direct Unicode counter results](unicode-counter-result04.json): six matches, exit zero, including accented text, three-byte characters, astral scalars, escapes and the maximum scalar.
- [Independent Astra audit](astra-final-audit01.json): all 229 raw case outputs independently decoded, inputs reconstructed with actual predecessor results, 436 installed toolchain file hashes and 1,142 compiled artifact hashes checked. No blocking findings within native conformance scope.
- [Independent Grok audit](grok-final-review02/review.json): approved the same native conformance scope with no findings. The [terminal receipt](grok-final-review02/receipt.json) records `end_turn` and the returned usage identity `grok-4.6-build`; the reviewer described itself as `grok-4.6`.
- `native-captures/` retains compressed original commands, inputs, outputs, results and bindings, with an original-path digest manifest. Compiled binaries remain local; their recorded hashes do not imply portability to another host.

The first independent Grok final audit reached its 1,200-second caller limit without a final verdict; its [attempt record](grok-final-attempt01.json) preserves that failure and unknown usage. Continuing that review session with streaming observation produced the retained approval without replaying native cases. Both independent audits now approve the finite native conformance results. Published review material contains only public text and whitelisted terminal metadata.

Grok did not rerun the reported 910 language tests or find their TAP capture in this result directory. It also found no service-properties capture for the final 104-case lifecycle run: that run's zero exits and outputs are retained, while the 64 MiB stack setting is recorded in the allocation and separately captured for expression execution and diagnosis. These limits remain distinct from the verified native results.

## Preserved failures and repairs

The original machine and its Nix-built artifact were unavailable. Sources and the exact historical trace 106 input were recovered, then rebuilt with the pinned WSL K 7.1.337 installation. This is a new artifact with explicit provenance, not a reconstruction of the old binary.

The deep metadata input reproduced a native KORE parser stack crash at an 8 MiB stack. With the same input and rebuilt artifact, a 64 MiB stack completed successfully. The full 65,536-byte bound remains intact; metadata at depth 5,947 is not truncated to avoid the failure.

The first full expression run then exposed Unicode handling errors. The codec now decodes K byte escapes as strict UTF-8. K byte counts use the installed byte-backed string operations, while UTF-16 counters count continuation bytes and astral scalars correctly. Both complete suites were rerun after these source repairs.

Three direct-counter attempts failed at input/output representation boundaries. Their commands and outputs remain preserved. A complete generated configuration, checked offline through a KAST/KORE roundtrip, resolved that representation issue; all six final native counter probes passed. No semantic K source or compiled artifact changed between the complete suites and those successful probes.

## Scope

These are finite native conformance results. They do not discharge determinism, progress, preservation, termination or general correspondence theorems; do not establish mandatory proof-carrying transactions; and do not connect the newer lifecycle to Midnight. The separate September 17 Preview delivery targets the existing fixed LAM test-asset loan.

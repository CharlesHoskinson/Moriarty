# Independent GPT-6 source review

APPROVED for the exact launch-source-02 source candidate only.

Candidate: `candidate.json`, SHA-256 `d27fc39cc7d61945e13c55546106bf02dab185073a40de0dc1ad1e84cd2e711d`.
Reviewer: fresh independent GPT-6 Astra agent `/root/sp05_launch_source_review`.
Author: `/root/sp05_final_integration_audit`. No other review was consulted.

All four candidate file hashes match. The selected installed runtime bytes match the manifest. Source inspection followed the CLI through plan validation, private input handling, SDK initialization, restored unshielded identity validation, synchronization, provider reservation, public payload retention, integration and cleanup.

The SDK accepts `WalletFacade.init` child factory callbacks and each child exposes strict serialized-state `restore`. The launcher supplies all three existing snapshots and has no generation, registration, fresh-state or restore fallback. Account-0/index-0 role derivation, keystore address methods, the unshielded state identity fields, facade start/sync/stop methods, and native Bip340 signing conversion agree with the installed implementation. `getSecretKey()` returns a buffer copy of the derived Uint8Array, so clearing that temporary buffer does not clear the live keystore.

The closed plan confines configured services to literal loopback HTTP/WS addresses and fixes the submission count at four with finite deadline and unsigned 128-bit decimal ceilings. Private readers reject foreign ownership, group/other permissions, nonregular files and symlink ancestry. Public plan evidence contains paths and public identities, not secret contents.

Provider reservations and issued-byte checks precede the wrapped submit call. The wrapper requires the actual native transaction class and canonical finalized serialization. It exclusively writes and fsyncs the public binary and metadata before invoking submission. An inert submit failure retains both; a metadata-storage failure prevents submission while preserving the binary. Duplicate capture prevents repeat submission. Tests deserialize only the historical public transaction; they do not prove or create one.

The launcher retains the real integration producer's complete public result on success and exception, including driver receipts, reservations, cleanup, all comparisons and the completed financial comparison when available. The driver and comparison producer project public fields and decimal amounts; private SDK transaction recipes, decoded private objects and exception text are not copied. The result writer is intentionally scoped to that producer, not a general-purpose sanitizer. The CLI emits fixed failure diagnostics.

Deadline races, child/facade stop attempts and the CLI terminal timer exist. Integration owns cleanup once handed the wallet. These mechanisms do not certify cancellation or complete resource containment. In particular, a facade initialization failure can occur after SDK service allocation and before a facade handle is returned. The documented mandatory outer process containment remains necessary; incomplete containment continues to prevent PASS/acceptance claims. This is a preserved execution limitation, not an approval of containment.

Validation: 30/30 launch and integration source tests passed; five independent adversarial groups passed (integer bounds, endpoint spoofing/caller mutation, ancestor symlinks, private recipe rejection and stopped-event error projection). Evidence is in `gpt6-source-tests.txt`, `gpt6-adversarial-checks.mjs`, and `gpt6-adversarial-checks.txt`. Hash details and inspected supporting source digests are in `gpt6-review.json`.

No blocking source finding was identified. No wallet was constructed/restored/started; no existing private wallet artifacts were read; no key, proof or contract compilation was generated; no network call or Opus invocation occurred. Synthetic private-file canaries were created only in temporary test directories and removed.

This review does not authorize execution, approve a build result, establish local service readiness, confirm snapshot compatibility or DUST availability, or establish financial/network/proof acceptance. Runtime pins cover selected entries, not every transitive import. Both required independent reviews and all execution/resource/acceptance gates remain separate; an unavailable Opus reviewer has not approved this candidate.

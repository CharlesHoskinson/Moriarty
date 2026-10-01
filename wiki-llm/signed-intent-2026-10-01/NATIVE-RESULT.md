# Numerical transfer proof result

Both fresh independent source/resource reviews approved one bounded experiment
on the six hashed runnable inputs. See [Astra](evidence/native-astra-source.md)
and [Grok](evidence/native-grok-source.md). The actual provider receipt identifies
Grok `grok-4.6-build`; the requested model was `grok-4.6`, high effort.

The one allowed attempt exited0: a 10,924-byte real KZG proof at k10 verified
through native prepare, strict transcript EOF and final pairing. All28 public
input changes, first-byte corruption, half truncation and an appended byte were
refused. [Raw log](evidence/native-run.log) and
[resource receipt](evidence/native-run-receipt.json) retain the observation.

The process group completed in0.90seconds with sampled peak RSS106,070,016bytes.
Sampling is at0.1seconds; it cannot report an exact continuous peak. Per-process
address space was also capped at8GiB, group memory/CPU and disk monitored, Cargo
and Rayon capped at2workers, wall limit600seconds. Target apparent size remained
2,323,673,542bytes; final free space26,980,806,656bytes. No retry or k increase.
The ephemeral unsafe local SRS was generated inside the test and not published.

This proves the scoped numerical relation only. Identities, source hash, signature,
key authority, authentic state, nonce/head and compiler correspondence are outside
this circuit. Native zero-fee and invalid-witness emitted proofs were not executed;
their constraint diagnostics are separate. Proofs0.8/curves0.3 are not established
as compatible with the ledger proof0.7.3/curves0.2.1 verifier. No Preview transaction
or wallet action occurred. Whole-candidate actual-result audits remain owed.

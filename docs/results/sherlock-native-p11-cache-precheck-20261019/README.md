# Native P11 cache precheck result

The single reviewed native attempt stopped before Cargo because the historical A0 source digest for the Camino registry index differed from the mutable host cache. The guard refused after 13.154616678 seconds. Partial source-home staging occurred before the failing row; no completed reconstitution, Cargo build, new test executable or SDK check resulted.

The read-only diagnostic compared all 2,243 historical A0 inputs and found exactly one mismatch. The host Camino index was 20,024 bytes; the pinned source was 18,266 bytes. The already preserved recovery file matches the expected SHA-256. The previous successful build's cleanup had restored the host version. Current stat equality therefore did not establish historical content equality.

Raw current E/S/P usage was 12,906,928 / 9,308,089 / 3,598,838 microseconds. Added to retained history, cumulative usage is 1,092,254,321 / 252,227,979 / 840,026,337 microseconds, with residual 5 microseconds. No usage was refunded. The failed attempt remains consumed.

The five evidence files were collected, and teardown removed all three quota mounts and all execution, supervisor and phase cgroups. Cleanup is complete. This attempt did not modify the host cache, so it created no cache restoration obligation.

The next correction uses the existing reversible cache-recovery procedure and retains this failed attempt's usage. This archive establishes the refusal and cleanup only; it closes no native, proof, stagenet or EARS requirement. Raw command/result/stream bytes and failure markers remain unchanged, with provenance in [MANIFEST.json](MANIFEST.json).

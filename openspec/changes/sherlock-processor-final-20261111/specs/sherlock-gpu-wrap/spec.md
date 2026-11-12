## ADDED Requirements

### Requirement: GPW-001 The wrap runs on the user's GPU with a CPU fallback
The wrap prover SHALL run on the user's GPU. The stock CPU path SHALL stay as the fallback. Under the test seed it SHALL give the same proof bytes as the GPU path.

#### Scenario: No GPU
- **WHEN** the user's machine has no usable GPU
- **THEN** the CPU path proves the wrap, and the proof is accepted by the ledger 9 verifier

### Requirement: GPW-002 Kernels in build order
The GPU wrap kernels SHALL be built in order: K0 the Fr field header shared with the Fr Poseidon kernel of `L`, K1 NTT over Fr, K2 MSM on G1, K3 the coset-tiled quotient numerator, K4 grand products, K5 evaluations and multiopen, K6 the lookup sort.

#### Scenario: Kernel out of order
- **WHEN** a kernel is integrated
- **THEN** every kernel before it in the order passes its byte-identity test first (GPW-003)

### Requirement: GPW-003 GPU and CPU proofs are byte-identical under the test seed
Under `GW_DET=1`, every CPU and GPU configuration SHALL give one proof SHA-256. Each proof SHALL pass the library verifier and ledger 9 `VerifierKey::verify`. The GPU quotient numerator SHALL equal the CPU values on every point of the extended domain.

#### Scenario: Measured on the k = 20 decider
- **WHEN** seven CPU and GPU configurations prove the k = 20 decider under the test seed
- **THEN** all seven give one SHA-256 and the GPU numerator matches the CPU in all 4,194,304 extended-domain values (MEASURED, `PR` section 8), and the G4 wrap repeats the test after K1 to K5

#### Scenario: Different bytes
- **WHEN** one configuration gives other proof bytes under `GW_DET=1`
- **THEN** that configuration is not released

### Requirement: GPW-004 The MSM skew mask
The wrap prover SHALL compute `MSM(s)` as `MSM(s + r) - MSM(r)` with a fixed vector `r` and `MSM(r)` cached per base table and length. The mask SHALL be witness-independent. It SHALL run on the user's GPU. It SHALL leave the proof bytes unchanged.

#### Scenario: Skewed witness columns
- **WHEN** witness columns repeat small range-checked values
- **THEN** the masked MSM gives the same proof bytes as the unmasked MSM, and on the k = 20 decider the 45 calls take 23.2 s against 622 to 647 s (MEASURED once under load)

### Requirement: GPW-005 MSM split between CPU and GPU
The MSM SHALL stay split between CPU and GPU while the GPU MSM takes more than 0.2 s per `2^20` points.

#### Scenario: Slow GPU MSM
- **WHEN** D2 measures the idle GPU MSM above 0.2 s per `2^20`
- **THEN** the split stays

### Requirement: GPW-006 Fallbacks of the GPU path
If the Fr Poseidon GPU kernel of `L` slips, the build SHALL wait for the kernel (OD-10). No second Fr proving engine SHALL be built (OD-11). A measured GPU wrap above 8 s at the chosen k after K1 to K5 SHALL be a fallback trigger of LW-014.

#### Scenario: Wrap above 8 s
- **WHEN** the GPU wrap takes more than 8 s at the chosen k after K1 to K5
- **THEN** the trigger of LW-014 is recorded and goes to the owner

### Requirement: GPW-007 Time figures need a run record
A wrap time figure SHALL be reported MEASURED only with its run record and host load. Heavy jobs SHALL run one at a time under `heavy-run`. A quiet-host repeat SHALL stay PENDING until measured. The wrap times of call-time paths 2 and 3 SHALL be reported as ESTIMATE conditional on the fused quotient kernel and the MSM rewrite, with the table of `PR` section 8 (k = 19 proxy, 20 and 21, MEASURED under load) as the baseline.

#### Scenario: Figure from a loaded host
- **WHEN** a time is measured at load average 16 to 24
- **THEN** the record names the load, and a ratio from that run holds for that run only

### Requirement: GPW-008 Lean obligations of the GPU wrap prover
The design lists no Lean obligation for the GPU wrap prover. Its correctness SHALL rest on GPW-003 and on the wrap circuit of kit L AND WRAP, whose obligations are LW-015.

#### Scenario: Obligation status
- **WHEN** the status is reported
- **THEN** it reads "no Lean obligation in the design", and the byte-identity tests are the evidence

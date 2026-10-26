# Resource-bound arithmetic (ESTIMATE, not new measurements)

SOURCE inputs: raw/s9-orchestration/prior-astra3.md. All phase shares, worker efficiency, finality and windows are assumptions.

ESTIMATE: T=1617.292017 s; C=808.646008 core-s; G=121.296901 s; Q=16.172920 s; best-path L=503.515354 s. 16 productive CPU cores, four reserved; one serialized GPU server.

| Legal batch window W | Finality F (s) | Interval lower bound (s) | Speedup upper bound vs historical T | Upper bound vs CPU baseline T/2 |
|---:|---:|---:|---:|---:|
| 1 | 0 | 503.515 | 3.212x | 1.606x |
| 2 | 0 | 251.758 | 6.424x | 3.212x |
| 3 | 0 | 167.838 | 9.636x | 4.818x |
| 4 | 0 | 125.879 | 12.848x | 6.424x |
| 5 | 0 | 121.297 | 13.333x | 6.667x |
| 8 | 0 | 121.297 | 13.333x | 6.667x |
| 1 | 300 | 803.515 | 2.013x | 1.006x |
| 2 | 300 | 401.758 | 4.026x | 2.013x |
| 3 | 300 | 267.838 | 6.038x | 3.019x |
| 4 | 300 | 200.879 | 8.051x | 4.026x |
| 5 | 300 | 160.703 | 10.064x | 5.032x |
| 8 | 300 | 121.297 | 13.333x | 6.667x |

ESTIMATE: historical 10x target interval=161.729202 s; CPU budget at 16 cores=2587.667 core-s/batch; window minimum at F=0: ceil(L/target)=4; at F=300: 5. These are necessary, not sufficient.

ESTIMATE: four infinitely provisioned leaf workers with unchanged proofs: 1617.292017/(49.369468+867.279750+586.381690)=1.076020x latency ceiling. Zero-time leaves ceiling=1.112564x.

ESTIMATE: B3 recorded target endpoints: 92.0389--128.7534 s; B5 suite/12=463.239833 s per batch instance; B5 10x suite target=555.8878 s. B5 division is suite amortization, never steady throughput or latency.

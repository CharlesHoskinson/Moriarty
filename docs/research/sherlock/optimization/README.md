# Sherlock optimization wave

Ten source studies assess the frozen Candidate A design. Their hashes are in
[manifest.json](manifest.json); the [synthesis](10/proposal.md) records ordering,
comparison predicates and remaining decisions. They are research proposals,
not independent product audits or measured improvements.

| Study | Topic |
|---|---|
| [01](01/proposal.md) | Metered helpers and implementation caching |
| [02](02/proposal.md) | AST, predecoded tables and typed bytecode |
| [03](03/proposal.md) | Recursive data, arenas and structural sharing |
| [04](04/proposal.md) | Frames, environments and tail-call representation |
| [05](05/proposal.md) | Specialized, universal and fixed-family circuit realizations |
| [06](06/proposal.md) | Batching, segmentation and stopping boundaries |
| [07](07/proposal.md) | Witness generation, traces and host storage |
| [08](08/proposal.md) | Exact UInt128 representation and arithmetic |
| [09](09/proposal.md) | Rule mappings and correspondence engineering |
| [10](10/proposal.md) | Implementation sequence and comparison matrix |

The source snapshot remains frozen. Later parser repairs to the Quint source
are recorded separately; none retroactively changes that snapshot. Logical
work, circuit size, proving time, proof size, DUST and financial fees are
separate quantities. No study executed native or model benchmarks. Required
Clean implementation stays in the existing Fenrir fork.

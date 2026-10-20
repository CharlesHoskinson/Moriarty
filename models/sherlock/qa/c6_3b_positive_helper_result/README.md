# Public Chunk3B positive helper/result consumer

These two Rust files are byte-exact source for the reviewed and executed standalone positive codec consumer. They call the public `mori_core::sherlock::helper_machine::c6_phys_helper` record and section codecs. This directory is opt-in QA source; it is not a Cargo automatic test target and changes no Cargo manifest, module registration or production codec.

The supplied cohort contains an absent helper plus the ten non-Scalar helper variants, and an absent result plus all nineteen result variants. Inputs are synthetic owned carriers under explicit 4096 caller caps. They are not admitted or asserted reachable machine states. Wide `u128` values are serialized as exact decimal JSON numbers. Absent vector variant labels and absent outer decoded carriers are JSON null; present labels are exact strings.

Read the [actual result, source cut and limits](../../../../docs/results/sherlock-c6-3b-positive-helper-result-20261020/README.md) and [SAFE inputs](../../../../docs/results/sherlock-c6-3b-positive-helper-result-20261020/inputs/INPUTS.json). The recorded external `rustc` command and qualified non-LTO Core artifact are retained as evidence; their local cache paths are historical, not a portable installed toolchain or an automatic test command. The source's fixed provenance strings describe that recorded cut and require external artifact-pin checks; they do not authenticate a future supplied library.

The consumer contains no private expected vector and does not derive one from the encoder. Root froze the complete actual capture before the separately selected whole-vector comparison child read the original private expected document. Public evidence intentionally withholds both populated expected vectors and semantic capture output, so replaying that independent comparison requires the separately retained inputs. No such private input is a public developer compile permission.

The Scalar helper/independent ScalarStaged numeric-tag mapping, task codec QA, full section/full ABI coverage, owner semantics, admission, security, native/proof/target and EARS acceptance remain open.

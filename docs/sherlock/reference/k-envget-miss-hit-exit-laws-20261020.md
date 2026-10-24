# K EnvGet miss/hit/Exit law reference

The entry `egThreeEventTraversal(EGSnapshot)` computes a four-snapshot, three-event K term. The six Bool laws check whole-carrier miss preservation, a previous-None miss staying Pending, nonzero cursor to Some(0), complete miss-event accounting, linked miss/hit/Exit composition, and an off-chain previous-pointer discriminator. The source assertions and every input field are retained in [the dated evidence](../../results/sherlock-k-envget-miss-hit-exit-laws-20261020/README.md).

The diagnostic API has `render OUT_DIR MANIFEST` and `decode LAW PRINTED REPORT`. It accepts only the six declared Bool roots and the single nested traversal position. It creates source inputs and queries, not afterstates or answers. `true` is `LAW_PASS`; `false` is `LAW_FAIL`; residual or malformed output is a distinct parse failure. Current genuine runtime `BEFORE_PIN` remains unbound in this diagnostic scope.

The observed procedure used the existing JSONv4 frontend, depth-1 kore-exec, kprint and the reviewed Bool decoder under unchanged per-child bounds. It produced six true reports in 25 clean children. This is finite synthetic representation/relation QA only, not genuine/native/admission/refinement/EARS closure.

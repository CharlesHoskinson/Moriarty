# Native key generation: independent next-phase expectations

2026-10-01. Source facts from pinned standalone ZKIR e82d81d25aabcc5f5092e2bd559083487f577914, not execution/resource approval. Current v4 compile/handoff/prepare candidate remains frozen. No keygen source is implemented here and no SRS body has been acquired. Existing full nativeledger acceptance contract continues to apply.

Pinned IrSource implements public Zkir::keygen(&ParamsProverProvider), computes self.k(), gets the matching actual prover parameters, invokes setup_vk and setup_pk, and returns a tagged-serializable ProverKey plus native VerifierKey. The official CLI compile arm uses this exact API, but creates/truncates output files before keygen and its OnDemand provider may fetch. Inference/recommendation: a next bounded native-caller keygen mode can reuse the existing fixed offline provider and actual API, avoid a separate CLI dependency graph, and exclusively publish new outputs after actual success. That recommendation needs current source and resource review before execution. It does not mutate the current reviewed bytes or consume a fifth Compact compilation.

Independent implementation requirements: closed input config contains immutable IR path/SHA and exact official k17 SRS path/SHA only; check config/IR/SRS identities, literal25,166,212byte SRS size, k17 and full parameter decode EOF. Digest identity is not an independent ceremony audit. New exclusive output directory, no overwrites of original IR or earlier failed keys. Run actual IrSource.keygen with offline provider requiring k17; retain exact tagged PK/VK and serialized IR with SHA/size receipt, no wallet/private keys, no nativeTransaction.prove/WF/apply. If setup fails or resource stops, retain failure and consumed reservation; never label partial keys usable. Verify tagged re-decode EOF/identity and resolver registration/key agreement under the future actual finalized proof. Native keygen is not financial proof or ledger acceptance.

Execution prerequisites: actual complete current v4 preparation success and both fresh current source/result audits, separately reviewed SRS body acquisition and exact bounded keygen/proof resource authority. Do not reset oldv3attempt or exhausted Compact/R3k17 debt. A future resource proposal must state actual current compiler/preflight observations and explicit caps before dispatch. Frozen-key identity and identical IR must persist through registered operation, provider resolver, final proof selfverification and independent ledger verification. The supplied transaction binding must come from real ledger finalization and differ from zero. Whole financial state/effects/fees, signatures, strict wellformed/application, replay/faults and trustedgenesis/authenticstate limitations remain mandatory; keys alone satisfy none of them.

Source inspection SHA256:

{
  "/home/charl/.cargo/git/checkouts/midnight-zkir-d3b0dbbf065d6ece/e82d81d/zkir/src/ir.rs": "61118e731ae492f61bbcae73633c7ba4ab627cbe7cd36b5ac455fc721028052c",
  "/home/charl/.cargo/git/checkouts/midnight-zkir-d3b0dbbf065d6ece/e82d81d/zkir/src/main.rs": "a6e8f3fb434efffce84ba492cb2664fff8c73e790aa22129fe213d185491b1c9"
}

# A102 proof and independent verification consumer — source

Adds one explicitly ignored, exact-selectable native test. It constructs the existing typed Literal transition through the public producer, emits its circuit, and calls the existing proof generator once and independent reconstruction/verification once. The complete proof bundle is persisted before verification; program bytes, emitted ZKIR and the exposed verification report are retained with create-new writes and file/directory synchronization.

Fresh Opus5.5HIGH authored and independently reviewed this source. Both security and normal/API source opinions approve it with no mandatory fixes. All 20,522 original test-module bytes remain an exact prefix; the append is 11,539 bytes. All nine relation sources are unchanged, so the same fixture context is preserved. The existing SDK-check selectors are not called or replayed by the new test.

This draft is uncompiled and unrun. It requires explicit offline SRS, degree cap and output-directory inputs; it supplies no native resource eligibility. The Task4 hook is a no-op in the fresh, unarmed exact-selector process, as confirmed from its full source. The raw VK and binding scalar are not exported; the persisted 2,077 statement lanes are not the full native input vector including the binding. Verification independently reconstructs the circuit/VK using the same in-process request and SRS.

The historical module header is deliberately preserved although it describes the old nonproving prefix. Source approval is not proof, soundness, recursion, ledger or stagenet evidence. All existing open obligations remain. Native build and proof selection are separate next steps.

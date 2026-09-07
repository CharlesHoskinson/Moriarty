# A5 kernel Task1 local admission

Root admits the joint evaluator kernel and immutable recorder infrastructure
under adopted plan900bb20. The full38-file original snapshots match the recorded
pins. Recursive RED typecheck passed; the supplied-result rejection control
then failed with QNT508. The sole correction restores comparison of the supplied
and actual transaction result. Recursive GREEN typecheck and all six kernel tests
passed with stable source/tool/runtime pins. The root audit independently checked
all original archive members and command streams. Its first wrong-stream lookup
is disclosed in audit-diagnostic.md.

Root's earlier branch review compared raw computation, malformed-before-mismatch
precedence, timeout rollback, cancellation identity and pre/deposit/post effect
ordering. No frozen Core, common authority or original Candidate A file changed.
Derived26-module corpus equivalence and the bounded funding pilot remain pending.
This is not full A5 acceptance or model-checking success.

Original receipts and author report are in original-receipts.tar.gz. The one
shared315704660-byte runtime archive is retained without loss in seven ordered
50MiB-or-smaller parts. audit-parts.py verifies their concatenation against the
original archive SHA256. Reconstruct by binary concatenation in numeric order
into a fresh destination before replay. This partition changes no archive bytes
and avoids a single Git blob above100MiB. Runtime manifest/version streams and
bootstrap receipt are inside original-receipts.tar.gz. The reused3329-file Python
archive remains in the A4 checkerTask1 evidence; it is not duplicated here.

Task2 may be dispatched after this commit. Runtime helper bytes remain frozen
because the concurrent producer references them. No Task3 pilot is authorized
until the full Task2 evidence is independently admitted.

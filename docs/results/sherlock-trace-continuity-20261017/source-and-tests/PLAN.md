# R37 fixed implementation plan

Authorization: delegated ordinary A-profile continuity API under already-adopted SCON-007/R02. Root is sole integrator. Source edits remain outside W. Existing pairwise equality, B/shared/guards and native authority remain unchanged.

Predicate: first_continuity_difference returns None exactly when every complete boundary agrees and the checked total record charge equals checked endpoint remaining-fuel subtraction; this does not establish transition validity or authenticity.

1. Freeze exact compare/reference-types preimages and independent N01 expected input.
2. Handwrite fixed N01 snapshots/tests, add typed API stub, capture isolated rustc test failure.
3. Implement state-edge scan with existing snapshot_difference, then checked endpoint/charge arithmetic; capture same isolated tests passing.
4. Verify identical pairwise first_difference and exact source closure; provide afterimages, minimal patch, hashes and Diataxis reference proposal for independent review/integration.

Boundary order: initial/first, inter-record edges in index order, last/returned; empty uses initial/returned. Within an edge, existing SnapshotField order applies. State mismatch precedes accounting. Undefined endpoint delta precedes charge overflow and sum mismatch. Overflow is represented/checked but cannot be reached by an addressable u8-charge slice on this64-bit target.

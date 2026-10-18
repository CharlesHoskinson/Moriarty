Independent Astra medium reviewed the existing paths and approved the bounded adapter design after correcting reserve accounting.
The kernel closureReserve is separate from remaining; subtracting it again would double reserve work.
Expected work: remainingAfter=remainingBefore-E-N; spentAfter=spentBefore+E+N; reserve unchanged.
Quantity nominal values are restricted to nonnegative signed-128 range. Preserve all funding/identity/rollback controls.
This review specifies expected behavior; final implementation audit must use a fresh agent.

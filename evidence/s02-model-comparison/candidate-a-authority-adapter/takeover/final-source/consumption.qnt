module consumption {
  import effects.* from "./effects"

  type Domain = SwapDomain | InstallmentDomain
  type AuthorityKey = {domain: Domain, principal: Principal, nonce: int}
  type Parent[p] = {
    key: AuthorityKey, policy: p, source: Location, recipient: Principal,
    asset: Asset, budget: int, slots: int -> int,
  }
  type Claim[p] = Unclaimed | Claimed(Parent[p])
  type Entry[p] = {
    claim: Claim[p], usedSlots: Set[int], paid: int,
    remainingAllowance: int, cancelled: bool, revision: int,
  }

  pure def initialEntry(parent: Parent[p]): Entry[p] = {
    claim: Unclaimed, usedSlots: Set(), paid: 0,
    remainingAllowance: parent.budget, cancelled: false, revision: 0,
  }
  pure def validParent(parent: Parent[p]): bool =
    parent.key == {domain: InstallmentDomain, principal: Alice, nonce: 0} and
    parent.source == Escrow({owner: Alice, asset: TokenA}) and
    parent.recipient == Bob and parent.asset == TokenA and parent.budget == 10 and
    parent.slots == Map(1 -> 5, 2 -> 5)
  pure def slotQuantity(parent: Parent[p], slot: int): int =
    if (parent.slots.keys().contains(slot)) parent.slots.get(slot) else 0
  pure def validEntry(parent: Parent[p], entry: Entry[p]): bool = {
    val parentMatches = match entry.claim {
      | Unclaimed => entry == initialEntry(parent)
      | Claimed(actual) => actual == parent and (entry.usedSlots.size() > 0 or entry.cancelled)
    }
    validParent(parent) and parentMatches and
    entry.usedSlots.subseteq(Set(1, 2)) and
    (not(entry.usedSlots.contains(2)) or entry.usedSlots.contains(1)) and
    entry.paid == entry.usedSlots.fold(0, (total, slot) => total + slotQuantity(parent, slot)) and
    entry.remainingAllowance == parent.budget - entry.paid and
    entry.remainingAllowance >= 0 and
    entry.revision == entry.usedSlots.size() + (if (entry.cancelled) 1 else 0) and
    (not(entry.cancelled) or entry.remainingAllowance > 0)
  }
  pure def canConsumeSlot(parent: Parent[p], entry: Entry[p], slot: int, prepared: Entry[p]): bool =
    validEntry(parent, entry) and prepared == entry and not(entry.cancelled) and
    parent.slots.keys().contains(slot) and not(entry.usedSlots.contains(slot)) and
    (slot == 1 or entry.usedSlots.contains(1)) and
    slotQuantity(parent, slot) <= entry.remainingAllowance
  pure def applyConsumeSlot(parent: Parent[p], entry: Entry[p], slot: int): Entry[p] = {
    ...entry, claim: Claimed(parent), usedSlots: entry.usedSlots.union(Set(slot)),
    paid: entry.paid + slotQuantity(parent, slot),
    remainingAllowance: entry.remainingAllowance - slotQuantity(parent, slot),
    revision: entry.revision + 1,
  }
  pure def canCancelParent(parent: Parent[p], entry: Entry[p], prepared: Entry[p]): bool =
    validEntry(parent, entry) and prepared == entry and not(entry.cancelled) and entry.remainingAllowance > 0
  pure def applyCancelParent(parent: Parent[p], entry: Entry[p]): Entry[p] = {
    ...entry, claim: Claimed(parent), cancelled: true, revision: entry.revision + 1,
  }
  pure def validResidual(parent: Parent[p], before: Entry[p], slot: int, proposed: Entry[p]): bool =
    canConsumeSlot(parent, before, slot, before) and proposed == applyConsumeSlot(parent, before, slot)
}

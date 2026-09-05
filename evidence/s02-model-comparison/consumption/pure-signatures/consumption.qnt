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
  pure def validParent(parent: Parent[p]): bool = false
  pure def slotQuantity(parent: Parent[p], slot: int): int = 0
  pure def validEntry(parent: Parent[p], entry: Entry[p]): bool = false
  pure def canConsumeSlot(parent: Parent[p], entry: Entry[p], slot: int, prepared: Entry[p]): bool = false
  pure def applyConsumeSlot(parent: Parent[p], entry: Entry[p], slot: int): Entry[p] = entry
  pure def canCancelParent(parent: Parent[p], entry: Entry[p], prepared: Entry[p]): bool = false
  pure def applyCancelParent(parent: Parent[p], entry: Entry[p]): Entry[p] = entry
  pure def validResidual(parent: Parent[p], before: Entry[p], slot: int, proposed: Entry[p]): bool = false
}

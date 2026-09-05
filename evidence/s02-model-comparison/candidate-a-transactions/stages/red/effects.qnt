module effects {
  type Principal = Alice | Bob | Mallory
  type Asset = TokenA | TokenB
  type Location = Wallet(Principal) | Escrow({owner: Principal, asset: Asset})
  type Transfer = {source: Location, destination: Location, asset: Asset, quantity: int}
  type Ledger = (Location, Asset) -> int

  pure val PRINCIPALS: Set[Principal] = Set(Alice, Bob, Mallory)
  pure val ASSETS: Set[Asset] = Set(TokenA, TokenB)
  pure val LOCATIONS: Set[Location] = PRINCIPALS.map(p => Wallet(p)).union(
    PRINCIPALS.map(p => ASSETS.map(a => Escrow({owner: p, asset: a}))).flatten())
  pure val LEDGER_KEYS: Set[(Location, Asset)] = LOCATIONS.map(l => ASSETS.map(a => (l, a))).flatten()

  pure def locationAccepts(location: Location, asset: Asset): bool = match location {
    | Wallet(_) => true
    | Escrow(account) => account.asset == asset
  }
  pure def balance(ledger: Ledger, location: Location, asset: Asset): int =
    if (ledger.keys().contains((location, asset))) ledger.get((location, asset)) else 0
  pure def initialLedger(aliceA: int, bobB: int): Ledger =
    LEDGER_KEYS.mapBy(_ => 0).put((Wallet(Alice), TokenA), aliceA).put((Wallet(Bob), TokenB), bobB)
  pure def validLedger(ledger: Ledger): bool =
    ledger.keys() == LEDGER_KEYS and
    LOCATIONS.forall(l => ASSETS.forall(a =>
      balance(ledger, l, a) >= 0 and (locationAccepts(l, a) or balance(ledger, l, a) == 0)))
  pure def validTransfer(effect: Transfer): bool =
    effect.quantity > 0 and effect.source != effect.destination and
    locationAccepts(effect.source, effect.asset) and locationAccepts(effect.destination, effect.asset)
  pure def applyOne(ledger: Ledger, effect: Transfer): Ledger = {
    val debited = ledger.put((effect.source, effect.asset), balance(ledger, effect.source, effect.asset) - effect.quantity)
    debited.put((effect.destination, effect.asset), balance(debited, effect.destination, effect.asset) + effect.quantity)
  }
  pure def applyTransfers(ledger: Ledger, effects: List[Transfer]): Ledger =
    effects.foldl(ledger, (current, effect) => applyOne(current, effect))
  pure def canApply(ledger: Ledger, effects: List[Transfer]): bool = {
    val checked = effects.foldl({ledger: ledger, ok: validLedger(ledger)}, (acc, effect) => {
      ledger: applyOne(acc.ledger, effect),
      ok: acc.ok and validTransfer(effect) and balance(acc.ledger, effect.source, effect.asset) >= effect.quantity,
    })
    checked.ok
  }
  pure def occurrences(effects: List[Transfer], effect: Transfer): int =
    effects.select(candidate => candidate == effect).length()
  pure def submultiset(left: List[Transfer], right: List[Transfer]): bool =
    left.foldl(true, (ok, effect) => ok and occurrences(left, effect) <= occurrences(right, effect))
  pure def validEffects(effects: List[Transfer]): bool =
    effects.foldl(true, (ok, effect) => ok and validTransfer(effect))
  pure def policyAllows(required: List[Transfer], allowed: List[Transfer], actual: List[Transfer]): bool =
    validEffects(required) and validEffects(allowed) and validEffects(actual) and
    submultiset(required, allowed) and submultiset(required, actual) and submultiset(actual, allowed)
  pure def totalAsset(ledger: Ledger, asset: Asset): int =
    LOCATIONS.fold(0, (total, location) => total + balance(ledger, location, asset))
}

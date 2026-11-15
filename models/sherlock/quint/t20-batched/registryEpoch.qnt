// T20 fix 6: one reg_root update per registry epoch, never one write per key (Midnight write rule W-4).
// The registry entry list stays off chain; a foreign proof's key is admitted by a membership path against
// reg_root, proved in circuit (GP-3 hook); acceptCollection checks reg_pub against the current root.
// The history and revocation rule of N-8 stays open; this model takes the strict reading (current root only),
// which is what the contract does today (sealed reg_root, one equality check).
//
// MUT:
//   "none"                 the design
//   "per_key_write"        each registered or revoked key writes reg_root (one write per key)
//   "no_reg_check"         acceptCollection does not compare reg_pub with the current root
//   "unbound_membership"   the collection may use a key that is not in the root it names
module registryEpoch {
  const MUT: str

  pure val KEYS = Set(1, 2, 3)
  pure val MAX_REG_EPOCH = 3

  var regEpoch: int                      // the registry period (time), closed by closeRegEpoch
  var roots: int -> Set[int]             // registry root id -> its entry list (off chain)
  var current: int                       // id of the root on chain
  var nextRoot: int
  var pendingAdd: Set[int]
  var pendingRevoke: Set[int]
  var regWrites: int -> int              // reg_root writes per registry epoch
  var accepted: List[(int, int, Set[int])]   // ghost: (root named, root current at acceptance, keys used)

  action init = all {
    regEpoch' = 0, roots' = Map(0 -> Set(1)), current' = 0, nextRoot' = 1, pendingAdd' = Set(), pendingRevoke' = Set(),
    regWrites' = Map(), accepted' = List(),
  }

  pure def inc(m: int -> int, k: int): int -> int = m.put(k, (if (m.keys().contains(k)) m.get(k) else 0) + 1)

  // a key is registered or revoked: off chain until the registry epoch closes
  action changeKey(k: int, add: bool): bool = all {
    regEpoch < MAX_REG_EPOCH,
    if (MUT == "per_key_write") all {
      val r = if (add) roots.get(current).union(Set(k)) else roots.get(current).exclude(Set(k))
      all {
        roots' = roots.put(nextRoot, r), current' = nextRoot, nextRoot' = nextRoot + 1, regEpoch' = regEpoch,
        regWrites' = inc(regWrites, regEpoch),
      },
      pendingAdd' = pendingAdd, pendingRevoke' = pendingRevoke,
    } else all {
      pendingAdd' = if (add) pendingAdd.union(Set(k)) else pendingAdd,
      pendingRevoke' = if (add) pendingRevoke else pendingRevoke.union(Set(k)),
      roots' = roots, current' = current, nextRoot' = nextRoot, regEpoch' = regEpoch, regWrites' = regWrites,
    },
    accepted' = accepted,
  }

  // registry period close: one new root over the whole entry list, one ledger write; a period with no
  // change writes nothing
  action closeRegEpoch: bool = all {
    regEpoch < MAX_REG_EPOCH,
    val changed = pendingAdd.size() + pendingRevoke.size() > 0
    val r = roots.get(current).union(pendingAdd).exclude(pendingRevoke)
    all {
      roots' = if (changed) roots.put(nextRoot, r) else roots,
      current' = if (changed) nextRoot else current,
      nextRoot' = if (changed) nextRoot + 1 else nextRoot,
      regWrites' = if (changed) inc(regWrites, regEpoch) else regWrites,
    },
    regEpoch' = regEpoch + 1, pendingAdd' = Set(), pendingRevoke' = Set(), accepted' = accepted,
  }

  // acceptCollection(reg_pub, claims_root) of a collection whose foreign keys are `used`, proved under root r
  action acceptCollection(r: int, used: Set[int]): bool = all {
    roots.keys().contains(r),
    MUT == "no_reg_check" or r == current,
    MUT == "unbound_membership" or used.subseteq(roots.get(r)),
    accepted' = accepted.append((r, current, used)),
    regEpoch' = regEpoch, roots' = roots, current' = current, nextRoot' = nextRoot, pendingAdd' = pendingAdd, pendingRevoke' = pendingRevoke,
    regWrites' = regWrites,
  }

  action step = any {
    nondet k = KEYS.oneOf()
    nondet a = Set(true, false).oneOf()
    changeKey(k, a),
    closeRegEpoch,
    nondet r = 0.to(6).oneOf()
    nondet u = KEYS.powerset().oneOf()
    acceptCollection(r, u),
  }

  // ---- invariants
  val RegWritesPerEpochAtMostOne = regWrites.keys().forall(e => regWrites.get(e) <= 1)
  // StaleOrForeignRegRootRefused: every accepted collection names the root current at its acceptance
  val StaleOrForeignRegRootRefused = accepted.indices().forall(i => accepted[i]._1 == accepted[i]._2)
  // every key a collection used is in the root it named (membership in circuit, GP-3)
  val KeysUsedAreRegistered = accepted.indices().forall(i => accepted[i]._3.subseteq(roots.get(accepted[i]._1)))

  val Inv = RegWritesPerEpochAtMostOne and StaleOrForeignRegRootRefused and KeysUsedAreRegistered

  // ---- non-vacuity: one registry epoch carried two key changes in one write, and a collection used a key
  // added in a later epoch
  val WitBatchedUpdate = roots.keys().exists(e => e > 0 and roots.keys().contains(e - 1)
    and (roots.get(e).exclude(roots.get(e - 1)).size() + roots.get(e - 1).exclude(roots.get(e)).size()) >= 2)
    and RegWritesPerEpochAtMostOne
  val WitNewKeyUsed = accepted.indices().exists(i => accepted[i]._1 > 0 and accepted[i]._3.exclude(Set(1)).size() > 0)
  val NotWitBatchedUpdate = not(WitBatchedUpdate)
  val NotWitNewKeyUsed = not(WitNewKeyUsed)
}

module authorization {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"

  // Finite symbolic signatures model exact content, not cryptographic security.
  type SignedPolicy[s,c,a,p] = {policy: Policy[s,c,a,p], signer: Principal, token: int}
  type AuthorityCell[s,c,a,p] = AuthorityUnused | AuthorityRegistered(SignedPolicy[s,c,a,p])
    | AuthorityConsumed({signed: SignedPolicy[s,c,a,p], revision: int})
  type ParentCell[s,c,a,p] = ParentVacant
    | ParentLive({parent: Parent[SignedPolicy[s,c,a,p]], entry: Entry[SignedPolicy[s,c,a,p]]})
  type AuthorityContext[s,c,a,p] = {candidate: s, ledger: Ledger, environment: Environment,
    registry: AuthorityKey -> AuthorityCell[s,c,a,p], parents: AuthorityKey -> ParentCell[s,c,a,p]}
  type SigningSnapshot[s,c,a,p] = {candidate: s, ledger: Ledger, environment: Environment,
    keyCell: AuthorityCell[s,c,a,p], parentCells: AuthorityKey -> ParentCell[s,c,a,p],
    parentAuthorities: AuthorityKey -> AuthorityCell[s,c,a,p]}
  type SigningCheck[s,c,a,p] = {policy: Policy[s,c,a,p], signer: Principal, snapshot: SigningSnapshot[s,c,a,p]}
  type SigningRecord[s,c,a,p] = NoSigningCheck | PreparedSigning(SigningCheck[s,c,a,p])
    | CompletedSigning(SignedPolicy[s,c,a,p])
  type SigningState[s,c,a,p] = {context: AuthorityContext[s,c,a,p],
    signing: AuthorityKey -> SigningRecord[s,c,a,p]}

  pure def isParentKey(key: AuthorityKey): bool = key.domain == InstallmentDomain and key.nonce == 0
  pure def validSignedPolicy(signed: SignedPolicy[s,c,a,p]): bool = validPolicy(signed.policy)
    and signed.signer == signed.policy.body.key.principal and Set(0, 1).contains(signed.token)
  pure def validAuthorityCell(key: AuthorityKey, cell: AuthorityCell[s,c,a,p]): bool = match cell {
    | AuthorityUnused => true
    | AuthorityRegistered(signed) => validSignedPolicy(signed) and signed.policy.body.key == key
    | AuthorityConsumed(consumed) => validSignedPolicy(consumed.signed)
        and consumed.signed.policy.body.key == key and consumed.revision >= 1
  }
  pure def validParentCell(key: AuthorityKey, cell: ParentCell[s,c,a,p], authority: AuthorityCell[s,c,a,p]): bool = match cell {
    | ParentVacant => not(isParentKey(key)) or authority == AuthorityUnused
    | ParentLive(live) => live.parent.key == key and validParent(live.parent)
        and validEntry(live.parent, live.entry) and validSignedPolicy(live.parent.policy)
        and live.parent.policy.policy.body.key == key and match authority {
          | AuthorityRegistered(signed) => signed == live.parent.policy and live.entry == initialEntry(live.parent)
          | AuthorityConsumed(consumed) => consumed.signed == live.parent.policy
              and consumed.revision == live.entry.revision and live.entry.revision > 0
          | AuthorityUnused => false
        }
  }
  pure def validAuthorityContext(context: AuthorityContext[s,c,a,p]): bool =
    if (context.registry.keys() == AUTHORITY_KEYS and context.parents.keys() == AUTHORITY_KEYS)
      validLedger(context.ledger) and validEnvironment(context.environment)
        and AUTHORITY_KEYS.forall(key => validAuthorityCell(key, context.registry.get(key))
          and validParentCell(key, context.parents.get(key), context.registry.get(key)))
    else false
  pure def parentFacts(cell: ParentCell[s,c,a,p]): ParentFacts = match cell {
    | ParentVacant => ParentAbsent
    | ParentLive(live) => ParentPresent({cancelled: live.entry.cancelled, usedSlots: live.entry.usedSlots,
        paid: live.entry.paid, remainingAllowance: live.entry.remainingAllowance, revision: live.entry.revision})
  }
  // Callers must validate the context before treating this projection as authentic.
  pure def contextPolicyFacts(context: AuthorityContext[s,c,a,p]): PolicyFacts = {
    ledger: context.ledger, environment: context.environment,
    parents: context.parents.keys().mapBy(key => parentFacts(context.parents.get(key))),
  }
  pure def operationParents(op: Operation): Set[AuthorityKey] = match op {
    | OpFillSlot(fill) => Set(fill.parent)
    | OpCancelParent(parent) => Set(parent)
    | OpRecover(recovery) => Set(recovery.parent)
    | _ => Set()
  }
  pure def policyParentDependencies(body: PolicyBody): Set[AuthorityKey] = body.clauses.foldl(Set(), (deps, clause) =>
    clause.conditions.foldl(deps.union(operationParents(clause.operation)), (acc, condition) => match condition {
      | ParentMatches(parent) => acc.union(Set(parent.key))
      | _ => acc
    }))
  // Preconditions: complete maps and finite dependency keys. Never bind unrelated
  // registry cells: one signer's registration must not invalidate the other.
  pure def signingSnapshot(context: AuthorityContext[s,c,a,p], body: PolicyBody): SigningSnapshot[s,c,a,p] = {
    val deps = policyParentDependencies(body)
    {candidate: context.candidate, ledger: context.ledger, environment: context.environment,
      keyCell: context.registry.get(body.key), parentCells: deps.mapBy(key => context.parents.get(key)),
      parentAuthorities: deps.mapBy(key => context.registry.get(key))}
  }
  pure def validSigningState(state: SigningState[s,c,a,p]): bool = validAuthorityContext(state.context)
    and state.signing.keys() == AUTHORITY_KEYS
  pure def canPrepareSigning(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal): bool =
    if (validSigningState(state) and validPolicy(policy) and policyParentDependencies(policy.body).subseteq(AUTHORITY_KEYS)) {
      val expected = {policy: policy, signer: signer, snapshot: signingSnapshot(state.context, policy.body)}
      signer == policy.body.key.principal and state.context.registry.get(policy.body.key) == AuthorityUnused
        and policy.body.implementationVersion == state.context.environment.implementationVersion
        and policy.body.enforcementMechanism == state.context.environment.enforcementMechanism
        and state.signing.get(policy.body.key) != PreparedSigning(expected)
    } else false
  pure def applyPrepareSigning(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal): SigningState[s,c,a,p] = {
    ...state, signing: state.signing.put(policy.body.key,
      PreparedSigning({policy: policy, signer: signer, snapshot: signingSnapshot(state.context, policy.body)})),
  }
  pure def fixtureParent(signed: SignedPolicy[s,c,a,p]): Parent[SignedPolicy[s,c,a,p]] = {
    key: signed.policy.body.key, policy: signed, source: Escrow({owner: Alice, asset: TokenA}),
    recipient: Bob, asset: TokenA, budget: 10, slots: Map(1 -> 5, 2 -> 5),
  }
  // Registering the parent materializes its unclaimed accounting cell; signing
  // never transfers money, consumes a nonce, or asserts a recovery succeeded.
  pure def applySign(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal, token: int): SigningState[s,c,a,p] = {
    val signed = {policy: policy, signer: signer, token: token}
    val parent = fixtureParent(signed)
    val nextParents = if (isParentKey(policy.body.key))
      state.context.parents.put(policy.body.key, ParentLive({parent: parent, entry: initialEntry(parent)}))
      else state.context.parents
    {...state, context: {...state.context, registry: state.context.registry.put(policy.body.key, AuthorityRegistered(signed)),
        parents: nextParents}, signing: state.signing.put(policy.body.key, CompletedSigning(signed))}
  }
  pure def canSign(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal, token: int): bool =
    if (validSigningState(state) and validPolicy(policy) and policyParentDependencies(policy.body).subseteq(AUTHORITY_KEYS))
      match state.signing.get(policy.body.key) {
        | PreparedSigning(checked) => checked.policy == policy and checked.signer == signer
            and signer == policy.body.key.principal and Set(0, 1).contains(token)
            and state.context.registry.get(policy.body.key) == AuthorityUnused
            and checked.snapshot == signingSnapshot(state.context, policy.body)
            and validAuthorityContext(applySign(state, policy, signer, token).context)
        | _ => false
      }
    else false
}

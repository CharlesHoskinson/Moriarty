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

  pure def canPrepareSigning(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal): bool = false
  pure def applyPrepareSigning(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal): SigningState[s,c,a,p] = state
  pure def canSign(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal, token: int): bool = false
  pure def applySign(state: SigningState[s,c,a,p], policy: Policy[s,c,a,p], signer: Principal, token: int): SigningState[s,c,a,p] = state
}

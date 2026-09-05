module candidate_a_authority_boundary_harness {
  import effects.* from "./effects"
  import consumption.* from "./consumption"
  import observations.* from "./observations"
  import policies.* from "./policies"
  import authorization.* from "./authorization"
  import execution.* from "./execution"
  import candidate_a_types.* from "./candidate_a_types"
  import candidate_a_programs.* from "./candidate_a_programs"
  import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
  import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
  import candidate_a_authority_boundary_fixtures.* from "./candidate_a_authority_boundary_fixtures"

  // One actual funding transaction, not either complete agreement lifecycle.
  var boundaryExecutionA: AAuthorityExecution
  var boundaryProfileA: SigningProfile

  action initProfileA(profile: SigningProfile): bool = all {
    fundingFixtureValidA,
    boundaryExecutionA' = fundingUnsignedA,
    boundaryProfileA' = profile,
  }
  action init: bool = {
    nondet profile = Set(SignAfterResolve, SignBeforeResolve).oneOf()
    initProfileA(profile)
  }
  action prepareFundingA: bool = all {
    canPrepareSigningAuthorityA(boundaryExecutionA.authority, fundingPolicyA(boundaryProfileA), Alice),
    boundaryExecutionA' = {...boundaryExecutionA,
      authority: applyPrepareSigning(boundaryExecutionA.authority, fundingPolicyA(boundaryProfileA), Alice)},
    boundaryProfileA' = boundaryProfileA,
  }
  action signFundingA: bool = all {
    canSignAuthorityA(boundaryExecutionA.authority, fundingPolicyA(boundaryProfileA), Alice, 0),
    boundaryExecutionA' = {...boundaryExecutionA,
      authority: applySign(boundaryExecutionA.authority, fundingPolicyA(boundaryProfileA), Alice, 0)},
    boundaryProfileA' = boundaryProfileA,
  }
  action proposeFundingA: bool = match fundingAdaptationA {
    | AuthorityAdaptedA(obs) => all {
        validAuthorityExecutionA(boundaryExecutionA),
        boundaryExecutionA.authority.signing.get(fundingKeyA) ==
          CompletedSigning({policy: fundingPolicyA(boundaryProfileA), signer: Alice, token: 0}),
        canPropose(boundaryExecutionA, FundingOneAttempt, OpFund, obs, Alice),
        boundaryExecutionA' = applyPropose(boundaryExecutionA, FundingOneAttempt, OpFund, obs, Alice),
        boundaryProfileA' = boundaryProfileA,
      }
    | _ => all { false, boundaryExecutionA' = boundaryExecutionA, boundaryProfileA' = boundaryProfileA }
  }
  action verifyFundingA: bool = match boundaryExecutionA.attempts.get(FundingOneAttempt) {
    | ProposedAttempt(attempt) => {
        val evidence = fundingEvidenceForA(attempt, boundaryProfileA)
        all {
          canVerifyAuthorityA(boundaryExecutionA, FundingOneAttempt, evidence),
          boundaryExecutionA' = applyVerify(boundaryExecutionA, FundingOneAttempt, evidence),
          boundaryProfileA' = boundaryProfileA,
        }
      }
    | _ => all { false, boundaryExecutionA' = boundaryExecutionA, boundaryProfileA' = boundaryProfileA }
  }
  action commitFundingA: bool = all {
    canCommitAuthorityA(boundaryExecutionA, FundingOneAttempt),
    boundaryExecutionA' = applyCommit(boundaryExecutionA, FundingOneAttempt),
    boundaryProfileA' = boundaryProfileA,
  }
  action step: bool = any { prepareFundingA, signFundingA, proposeFundingA, verifyFundingA, commitFundingA }

  val fundingPreparedWitnessA = boundaryExecutionA == fundingPreparedA(boundaryProfileA)
  val fundingSignedWitnessA = boundaryExecutionA == fundingSignedA(boundaryProfileA)
  val fundingProposedWitnessA = boundaryExecutionA == fundingProposedA(boundaryProfileA)
  val fundingVerifiedWitnessA = boundaryExecutionA == fundingVerifiedA(boundaryProfileA)
  val fundingCommittedWitnessA = boundaryExecutionA == expectedFundingCommitA(boundaryProfileA)
  val fundingCommittedAfterWitnessA = boundaryProfileA == SignAfterResolve and fundingCommittedWitnessA
  val fundingCommittedBeforeWitnessA = boundaryProfileA == SignBeforeResolve and fundingCommittedWitnessA
  val boundarySafetyA = if (validAuthorityExecutionA(boundaryExecutionA))
    totalAsset(boundaryExecutionA.authority.context.ledger, TokenA) == 10
    and totalAsset(boundaryExecutionA.authority.context.ledger, TokenB) == 0
    and match boundaryExecutionA.attempts.get(FundingOneAttempt) {
      | ExecutedOperation(_) => fundingCommittedWitnessA
      | _ => boundaryExecutionA.authority.context.candidate == fundingBeforeA
          and boundaryExecutionA.authority.context.ledger == fundingLedgerA
    } else false
}

module candidate_a_programs {
  import effects.* from "./effects"
  import observations.* from "./observations"
  import candidate_a_types.* from "./candidate_a_types"

  pure val aliceA: CoreAccount = {owner: Alice, asset: TokenA}
  pure val bobB: CoreAccount = {owner: Bob, asset: TokenB}
  pure val closeProgram: AProgram = {root: N0, nodes: A_NODE_IDS.mapBy(_ => CloseA)}
  pure def emptyAState(continuation: NodeId): AState = {continuation: continuation,
    accounts: CORE_ACCOUNTS.mapBy(_ => 0), choices: A_CHOICE_IDS.mapBy(_ => NoInt), minimumTime: Time0}
  // Literal-table scaffolding; the Task 1 test must reject this empty graph.
  pure val canonicalSwap: AProgram = {...closeProgram, root: N6}
}

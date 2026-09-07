module candidate_a_funding_pilot_test {
  import policies.* from "../policies"
  import candidate_a_funding_pilot as O from "./candidate_a_funding_pilot"
  import candidate_a_funding_pilot_f as F from "./candidate_a_funding_pilot_f"
  pure def prefixOriginal(profile: SigningProfile, bad: bool, n: int): O::PilotState =
    0.to(4).fold(O::initial(profile,bad), (p, i) => if (i < n) O::advance(p) else p)
  pure def prefixFactored(profile: SigningProfile, bad: bool, n: int): F::PilotState =
    0.to(4).fold(F::initial(profile,bad), (p, i) => if (i < n) F::advance(p) else p)
  run allPilotPrefixesTest = assert(Set(SignAfterResolve,SignBeforeResolve).forall(profile =>
    Set(false,true).forall(bad => 0.to(if (bad) 4 else 5).forall(n => {
      val original = prefixOriginal(profile,bad,n)
      val factored = prefixFactored(profile,bad,n)
      original == factored and O::safety(original) and F::safety(factored)
        and O::ready(original) == F::ready(factored)
        and (if (O::ready(original)) O::advance(original) == F::advance(factored) else true)
    }))))
}

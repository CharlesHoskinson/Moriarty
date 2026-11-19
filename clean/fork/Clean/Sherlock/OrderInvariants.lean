import Init

/-!
Accounting projection of ConstantProductEpoch, bounded-atomic/1, transition0
and transition1. The program, kernel and ledger assumptions are identified in
deliverables/competitor-report/II.6/obligations.md. Successful projected steps
omit pricing, lifetime and upper-bound guards, so the relation overapproximates
the kernel. Values are natural numbers in a single asset's quantum.
-/
namespace Clean.Sherlock.OrderInvariants

structure Balances where
  reserveA : Nat
  reserveB : Nat
  traderA : Nat
  traderB : Nat
  providerA : Nat
  providerB : Nat

inductive Party where
  | trader | pool | provider
  deriving DecidableEq

inductive Asset where
  | a | b
  deriving DecidableEq

structure Transfer where
  asset : Asset
  payer : Party
  recipient : Party
  amount : Nat
  deriving DecidableEq

inductive Action where
  | swap (actor recipient : Party) (input output : Nat)
  | close (actor : Party)

def Action.actor : Action → Party
  | .swap actor _ _ _ | .close actor => actor

def Action.designatedOwner : Action → Party
  | .swap .. => .trader
  | .close .. => .provider

def Action.transfers (before : Balances) : Action → List Transfer
  | .swap _ recipient input output =>
    [⟨.a, .trader, .pool, input⟩, ⟨.b, .pool, recipient, output⟩]
  | .close _ =>
    [⟨.a, .pool, .provider, before.reserveA⟩, ⟨.b, .pool, .provider, before.reserveB⟩]

/-- The encoded guard requires trader/trader for swap and provider for close.
Only successful checked subtractions enter the projection. -/
inductive Step : Balances → Action → Balances → Prop where
  | swap (s : Balances) (input output : Nat)
      (fundedA : input ≤ s.traderA) (fundedB : output ≤ s.reserveB) :
      Step s (.swap .trader .trader input output)
        {s with reserveA := s.reserveA + input, traderA := s.traderA - input,
                reserveB := s.reserveB - output, traderB := s.traderB + output}
  | close (s : Balances) : Step s (.close .provider)
      {s with reserveA := 0, reserveB := 0, providerA := s.providerA + s.reserveA,
              providerB := s.providerB + s.reserveB}

/-- Accumulate the gross A paid by the trader from decoded outgoing effects. -/
def grossTraderA (effects : List Transfer) : Nat :=
  (effects.filter (fun e => decide (e.asset = .a ∧ e.payer = .trader))).map Transfer.amount |>.sum

/-- A receipt contains each step's exact effects in execution order. -/
inductive Trace : Balances → List Transfer → Balances → Prop where
  | nil (s : Balances) : Trace s [] s
  | cons (step : Step x action y) (rest : Trace y later z) :
      Trace x (action.transfers x ++ later) z

def Balances.totalA (s : Balances) := s.reserveA + s.traderA + s.providerA
def Balances.totalB (s : Balances) := s.reserveB + s.traderB + s.providerB

theorem conservation (trace : Trace x effects z) :
    x.totalA = z.totalA ∧ x.totalB = z.totalB := by
  induction trace with
  | nil => exact ⟨rfl, rfl⟩
  | cons step _ ih =>
    obtain ⟨ha, hb⟩ := ih
    cases step <;> simp [Balances.totalA, Balances.totalB] at * <;> omega

theorem gross_debit_identity (trace : Trace x effects z) :
    grossTraderA effects + z.traderA = x.traderA := by
  induction trace with
  | nil => simp [grossTraderA]
  | cons step _ ih =>
    cases step <;> simp [Action.transfers, grossTraderA, List.filter] at * <;> omega

theorem cumulative_budget (trace : Trace x effects z) (initialFunding : x.traderA ≤ budget) :
    grossTraderA effects ≤ budget := by
  have accounted := gross_debit_identity trace
  omega

structure Intent where
  principal : Party
  permittedRecipients : List Party

/-- Outcome-intent checks projected after external authentication of the signer,
genesis principal mapping, program/domain, nonce and predecessor. -/
def intentAdmits (signer : Party) (intent : Intent) (s : Balances) (a : Action) : Bool :=
  decide (signer = intent.principal ∧ intent.principal = a.actor) &&
    (a.transfers s).all (fun e => decide (e.recipient ∈ intent.permittedRecipients))

theorem owner_authority (step : Step s a s') (admitted : intentAdmits signer intent s a = true) :
    signer = a.designatedOwner ∧
      ∀ e ∈ a.transfers s, e.recipient ∈ intent.permittedRecipients := by
  simp [intentAdmits] at admitted
  obtain ⟨⟨hs, hp⟩, allowed⟩ := admitted
  refine ⟨?_, allowed⟩
  rw [hs, hp]
  cases step <;> rfl

#print axioms conservation
#print axioms gross_debit_identity
#print axioms cumulative_budget
#print axioms owner_authority

end Clean.Sherlock.OrderInvariants

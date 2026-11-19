import Init

/-!
Accounting projection of ConstantProductEpoch, bounded-atomic/1, transition0
and transition1. The program, kernel and ledger assumptions are identified in
deliverables/competitor-report/II.6/obligations.md. Successful projected steps
omit pricing, lifetime and upper-bound guards, so the relation overapproximates
the kernel. Values are natural numbers in a single asset's quantum.

Transfers are the model's own `Action.transfers`, not effects decoded from the
generated kernel. That link is an open correspondence obligation. Recipient
preservation in `owner_authority` follows from the actor and recipient guards
of `Step`; the outcome-intent premise only maps the signer to the actor.
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

/-- Accumulate the gross A paid by the trader over model transfers. -/
def grossTraderA (effects : List Transfer) : Nat :=
  (effects.filter (fun e => decide (e.asset = .a ∧ e.payer = .trader))).map Transfer.amount |>.sum

/-- A trace lists each successful step's model transfers in execution order. -/
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

/-- The guards of a successful step fix the actor to the designated owner. -/
theorem step_actor (step : Step s a s') : a.actor = a.designatedOwner := by
  cases step <;> rfl

/-- A successful step pays only the pool or the action's designated owner.
`Action.transfers` alone does not: an unguarded swap may name any recipient. -/
theorem step_recipients (step : Step s a s') :
    ∀ e ∈ a.transfers s, e.recipient = .pool ∨ e.recipient = a.designatedOwner := by
  cases step <;> simp [Action.transfers, Action.designatedOwner]

/-- Without the `Step` guards the recipient conclusion fails. -/
example : ¬ ∀ s, ∀ e ∈ (Action.swap .trader .provider 1 1).transfers s,
    e.recipient = .pool ∨ e.recipient = (Action.swap .trader .provider 1 1).designatedOwner := by
  intro h
  have := h ⟨0, 0, 0, 0, 0, 0⟩ ⟨.b, .pool, .provider, 1⟩ (by simp [Action.transfers])
  simp [Action.designatedOwner] at this

structure Intent where
  principal : Party
  permittedRecipients : List Party

/-- Outcome-intent principal check projected after external authentication of
the signer, genesis principal mapping, program/domain, nonce and predecessor.
It does not inspect transfer recipients. -/
def intentAdmits (signer : Party) (intent : Intent) (a : Action) : Bool :=
  decide (signer = intent.principal ∧ intent.principal = a.actor)

theorem owner_authority (step : Step s a s') (admitted : intentAdmits signer intent a = true)
    (poolPermitted : Party.pool ∈ intent.permittedRecipients)
    (principalPermitted : intent.principal ∈ intent.permittedRecipients) :
    signer = a.designatedOwner ∧
      ∀ e ∈ a.transfers s, e.recipient ∈ intent.permittedRecipients := by
  simp only [intentAdmits, decide_eq_true_eq] at admitted
  obtain ⟨hs, hp⟩ := admitted
  have owner : intent.principal = a.designatedOwner := hp.trans (step_actor step)
  refine ⟨hs.trans owner, fun e member => ?_⟩
  rcases step_recipients step e member with toPool | toOwner
  · rw [toPool]
    exact poolPermitted
  · rw [toOwner, ← owner]
    exact principalPermitted

#print axioms conservation
#print axioms gross_debit_identity
#print axioms cumulative_budget
#print axioms owner_authority

end Clean.Sherlock.OrderInvariants

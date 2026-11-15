import SherlockCodec.Wire
import Governance.Spec

/-! Oracle executable of the Governance context: codec only (see `SherlockCodec.Wire`).
Era slot `eK` is flattened to the cells `eKSt`, `eKStart`, `eKFin`, `eKKeyset`, `eKLedger`,
`eKAnnAt`. -/

open SherlockCodec Governance

def fields : List (String × Kind) :=
  [("now", .int),
   ("tl", .int),
   ("rtl", .int),
   ("q", .int),
   ("n", .int),
   ("grid", .int),
   ("current", .int),
   ("pending", .int),
   ("nextId", .int),
   ("e1St", .int),
   ("e1Start", .int),
   ("e1Fin", .int),
   ("e1Keyset", .int),
   ("e1Ledger", .int),
   ("e1AnnAt", .int),
   ("e2St", .int),
   ("e2Start", .int),
   ("e2Fin", .int),
   ("e2Keyset", .int),
   ("e2Ledger", .int),
   ("e2AnnAt", .int),
   ("e3St", .int),
   ("e3Start", .int),
   ("e3Fin", .int),
   ("e3Keyset", .int),
   ("e3Ledger", .int),
   ("e3AnnAt", .int),
   ("roster", .int),
   ("rosterPending", .int),
   ("rosterAt", .int),
   ("rosterGen", .int),
   ("nonce", .int),
   ("attestRoot", .int),
   ("attestAt", .int),
   ("attestUses", .int),
   ("tableDeployed", .bool),
   ("tableSealed", .bool),
   ("l1", .int),
   ("l2", .int),
   ("l3", .int),
   ("sherlockUsable", .int),
   ("unusable", .int),
   ("pathEvidence", .int),
   ("sealedReports", .int),
   ("eraRow", .bool),
   ("prem1", .int),
   ("prem2", .int),
   ("prem3", .int),
   ("rekeyStarted", .bool),
   ("c1", .int),
   ("c2", .int),
   ("c3", .int),
   ("uses", .int),
   ("lastRefusal", .int)]

def enc (s : St) : List V :=
  [.i s.now,
   .i s.tl,
   .i s.rtl,
   .i s.q,
   .i s.n,
   .i s.grid,
   .i s.current,
   .i s.pending,
   .i s.nextId,
   .i s.e1.st,
   .i s.e1.start,
   .i s.e1.fin,
   .i s.e1.keyset,
   .i s.e1.ledger,
   .i s.e1.annAt,
   .i s.e2.st,
   .i s.e2.start,
   .i s.e2.fin,
   .i s.e2.keyset,
   .i s.e2.ledger,
   .i s.e2.annAt,
   .i s.e3.st,
   .i s.e3.start,
   .i s.e3.fin,
   .i s.e3.keyset,
   .i s.e3.ledger,
   .i s.e3.annAt,
   .i s.roster,
   .i s.rosterPending,
   .i s.rosterAt,
   .i s.rosterGen,
   .i s.nonce,
   .i s.attestRoot,
   .i s.attestAt,
   .i s.attestUses,
   .b s.tableDeployed,
   .b s.tableSealed,
   .i s.l1,
   .i s.l2,
   .i s.l3,
   .i s.sherlockUsable,
   .i s.unusable,
   .i s.pathEvidence,
   .i s.sealedReports,
   .b s.eraRow,
   .i s.prem1,
   .i s.prem2,
   .i s.prem3,
   .b s.rekeyStarted,
   .i s.c1,
   .i s.c2,
   .i s.c3,
   .i s.uses,
   .i s.lastRefusal]

def dec (l : List V) : Option St := do
  let a := l.toArray
  guard (a.size = 54)
  let gi (k : Nat) : Option Int := match a[k]? with
    | some (.i x) => some x
    | _ => none
  let gb (k : Nat) : Option Bool := match a[k]? with
    | some (.b x) => some x
    | _ => none
  pure <|
    { now := (← gi 0)
      tl := (← gi 1)
      rtl := (← gi 2)
      q := (← gi 3)
      n := (← gi 4)
      grid := (← gi 5)
      current := (← gi 6)
      pending := (← gi 7)
      nextId := (← gi 8)
      e1 := { st := (← gi 9), start := (← gi 10), fin := (← gi 11), keyset := (← gi 12), ledger := (← gi 13), annAt := (← gi 14) }
      e2 := { st := (← gi 15), start := (← gi 16), fin := (← gi 17), keyset := (← gi 18), ledger := (← gi 19), annAt := (← gi 20) }
      e3 := { st := (← gi 21), start := (← gi 22), fin := (← gi 23), keyset := (← gi 24), ledger := (← gi 25), annAt := (← gi 26) }
      roster := (← gi 27)
      rosterPending := (← gi 28)
      rosterAt := (← gi 29)
      rosterGen := (← gi 30)
      nonce := (← gi 31)
      attestRoot := (← gi 32)
      attestAt := (← gi 33)
      attestUses := (← gi 34)
      tableDeployed := (← gb 35)
      tableSealed := (← gb 36)
      l1 := (← gi 37)
      l2 := (← gi 38)
      l3 := (← gi 39)
      sherlockUsable := (← gi 40)
      unusable := (← gi 41)
      pathEvidence := (← gi 42)
      sealedReports := (← gi 43)
      eraRow := (← gb 44)
      prem1 := (← gi 45)
      prem2 := (← gi 46)
      prem3 := (← gi 47)
      rekeyStarted := (← gb 48)
      c1 := (← gi 49)
      c2 := (← gi 50)
      c3 := (← gi 51)
      uses := (← gi 52)
      lastRefusal := (← gi 53) }

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["tl", "rtl", "q", "n", "grid"],
    refusals := fun ps ops => match ps with
      | [a, b, c, d, e] => (refusals a b c d e ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [a, b, c, d, e] => run a b c d e ops
      | _ => init 1 0 1 1 1 }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args

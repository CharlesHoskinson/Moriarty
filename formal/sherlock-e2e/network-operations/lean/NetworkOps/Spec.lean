/-!
# Network Operations oracle (bounded context Network Operations, slice C)

Executable reference semantics of the Preview run of one ledger call, from the
spec text of `sherlock-e2e-preview-deployment` (PVW), `-wallet` (WAL), `-sdk-cli`
(SCL), `-observability` (OBS) and `-recovery` (FRC), with LPC-009 (validium). No
code was read. The requirements with no state machine (loop process, tool
surfaces, file layouts) are checked for consistency in `Consistency.lean`.

The wallet is a guarded resource modelled by flags only: the phrase is read
(`walletRead`), the wallet started (`walletStarted`), the lock held (`lockHeld`),
redactions counted (`redactions`). No wallet content is modelled.

Phases of the runner:

| phase | meaning |
|---|---|
| 0 | not started |
| 1 | started: `T10A_TTL_S` in range (SCL-007) |
| 2 | chain checked: devnet or the pinned Preview genesis (PVW-001, PVW-002) |
| 3 | state folder checked: its deployed key equals `--vk` (SCL-004) |
| 4 | wallet modes checked, phrase read in-process (WAL-010, WAL-001) |
| 5 | wallet address equals the pin, wallet started (WAL-003) |
| 6 | contract deployed and confirmed (or read back, FRC-002) |
| 9 | stopped (`stop` names the guard) |

The call cells (`saved`, `submitted`, `unknown`, `included`, `readback`,
controls) live inside phase 6. The validium cells (`vRoot`, `vBatches`,
`vRefused`) model the validium contract of PVW-011 and LPC-009 and take
operations in every phase.
-/

namespace NetworkOps

/-! ## Constants of the spec -/

/-- SCL-007: `T10A_TTL_S` must be above 0 and below 1,209,600 (14 days). -/
def ttlMax : Int := 1209600
/-- FRC-003: a saved call is reused only with more than 5 minutes of TTL left. -/
def reuseTtl : Int := 300
/-- FRC-004: at most 4 balancing attempts in total. -/
def maxAttempts : Int := 4
/-- PVW-006: the 4 controls R1, R2, R4, R3. -/
def nControls : Int := 4
/-- PVW-013: one bit for each of RKY-003, RKY-004, RKY-005, the T12 gate, ZK-012..017. -/
def gateAll : Int := 31
def opBase : Int := 1000000000

/-! ## Codes recorded in the state -/

-- `stop` (terminal, phase 9)
def stopChain : Int := 1      -- PVW-001, PVW-002 "the RPC genesis is not the pinned Preview genesis"
def stopAddr : Int := 2       -- WAL-003 "not the OP-PV-WALLET wallet"
def stopMode : Int := 3       -- WAL-010 (directory wider than 0700 or file wider than 0600)
def stopTtl : Int := 4        -- SCL-007 "T10A_TTL_S out of range"
def stopStateVk : Int := 5    -- SCL-004 (state folder of another key)
def stopBalance : Int := 7    -- FRC-004 (4 attempts failed)
-- `lastRefusal` (non-terminal)
def refGate : Int := 6        -- PVW-013 (gate record incomplete)
def refOffHost : Int := 11    -- WAL-007 (submit from a machine without the wallet)
def refUnknown : Int := 12    -- FRC-013 (inclusion state unknown)
def refNoReadback : Int := 13 -- FRC-006 (no readback of the honest call)
def refNotOwned : Int := 14   -- PVW-010, case A (a box the user does not own)
def refFinalNotGpu : Int := 15 -- PVW-010 (final call not proved on an owned GPU box)
-- `lastVerdict`
def vRefusedByNode : Int := 1   -- the node refused (JSON-RPC error)
def vIncluded : Int := 2        -- PVW-012 `included`
def vTimeout : Int := 3         -- PVW-008 `not-included-timeout`
def vNoNodeVerdict : Int := 4   -- PVW-007 `no-node-verdict`

structure St where
  phase : Int
  dryRun : Bool
  ttlCfg : Int
  chain : Int
  stop : Int
  lastRefusal : Int
  walletRead : Bool
  walletStarted : Bool
  lockHeld : Bool
  deploys : Int
  deployPending : Bool
  gate : Int
  final : Bool
  proofs : Int
  proofHost : Int
  saved : Bool
  savedRefused : Bool
  balanced : Bool
  balanceAttempts : Int
  needResync : Bool
  submitted : Bool
  honestSubmits : Int
  unknown : Bool
  included : Bool
  receipts : Int
  errRecords : Int
  readback : Bool
  refusalsStarted : Bool
  controls : Int
  controlsRefused : Int
  controlFails : Int
  pendingControl : Bool
  lastVerdict : Int
  netConns : Int
  logLines : Int
  redactions : Int
  finished : Bool
  exitCode : Int
  vRoot : Int
  vBatches : Int
  vRefused : Int
  deriving Repr, DecidableEq

/-- The start state. `dryRun` selects the `dry-run` phase (SCL-003). -/
def init (dryRun : Bool) : St :=
  { phase := 0, dryRun, ttlCfg := 0, chain := 0, stop := 0, lastRefusal := 0,
    walletRead := false, walletStarted := false, lockHeld := false, deploys := 0,
    deployPending := false, gate := 0, final := false, proofs := 0, proofHost := 0,
    saved := false, savedRefused := false, balanced := false, balanceAttempts := 0,
    needResync := false, submitted := false, honestSubmits := 0, unknown := false,
    included := false, receipts := 0, errRecords := 0, readback := false,
    refusalsStarted := false, controls := 0, controlsRefused := 0, controlFails := 0,
    pendingControl := false, lastVerdict := 0, netConns := 0, logLines := 0, redactions := 0,
    finished := false, exitCode := 0, vRoot := 0, vBatches := 0, vRefused := 0 }

/-! ## Operations -/

inductive Op where
  | start (ttl : Int)                 -- 1
  | chainCheck (kind : Int)           -- 2: 1 devnet, 2 pinned Preview, other = another chain
  | stateCheck (sameVk : Bool)        -- 3
  | walletModes (ok : Bool)           -- 4
  | walletAddr (same : Bool)          -- 5
  | lock (acquire : Bool)             -- 6
  | deploy (gate : Int) (final : Bool) -- 7: arg = gate bits + 32 * final
  | deployConfirm                     -- 8
  | prove (host : Int)                -- 9: 0 wallet machine, 1 owned GPU box, 2 a box not owned
  | rerun (ttlLeft : Int) (hashOk : Bool) -- 10: arg = 2 * ttlLeft + hashOk
  | balance (kind : Int)              -- 11: 1 ok, 0 transport error, 2 other wallet error
  | submit (who host : Int)           -- 12: arg = who + 10 * host; who 0 honest, 1..4 control
  | outcome (kind : Int)              -- 13: 0 refused, 1 included, 2 600 s timeout, 3 no verdict
  | resolve (wasIncluded : Bool)      -- 14
  | readbackOp (present : Bool)       -- 15
  | startRefusals                     -- 16
  | log (secret : Bool)               -- 17
  | finish                            -- 18
  | resync                            -- 19
  | batch (progOk : Bool) (inRoot newRoot : Int) -- 20: arg = progOk + 2*inRoot + 2000*newRoot
  deriving Repr, DecidableEq

def bit (a : Int) (t f : Op) : Option Op := if a = 0 then some f else if a = 1 then some t else none

def Op.ofInt (n : Int) : Option Op :=
  let c := n / opBase
  let a := n % opBase
  if n < 0 then none
  else if c = 1 then some (.start a)
  else if c = 2 then some (.chainCheck a)
  else if c = 3 then bit a (.stateCheck true) (.stateCheck false)
  else if c = 4 then bit a (.walletModes true) (.walletModes false)
  else if c = 5 then bit a (.walletAddr true) (.walletAddr false)
  else if c = 6 then bit a (.lock true) (.lock false)
  else if c = 7 then (if a < 64 then some (.deploy (a % 32) (a / 32 = 1)) else none)
  else if c = 8 then (if a = 0 then some .deployConfirm else none)
  else if c = 9 then (if a ≤ 2 then some (.prove a) else none)
  else if c = 10 then some (.rerun (a / 2) (a % 2 = 1))
  else if c = 11 then (if a ≤ 2 then some (.balance a) else none)
  else if c = 12 then (if a % 10 ≤ 4 ∧ a / 10 ≤ 1 then some (.submit (a % 10) (a / 10)) else none)
  else if c = 13 then (if a ≤ 3 then some (.outcome a) else none)
  else if c = 14 then bit a (.resolve true) (.resolve false)
  else if c = 15 then bit a (.readbackOp true) (.readbackOp false)
  else if c = 16 then (if a = 0 then some .startRefusals else none)
  else if c = 17 then bit a (.log true) (.log false)
  else if c = 18 then (if a = 0 then some .finish else none)
  else if c = 19 then (if a = 0 then some .resync else none)
  else if c = 20 then some (.batch (a % 2 = 1) ((a / 2) % 1000) (a / 2000))
  else none

/-- A call is in flight: an honest submission or a control awaits its outcome. -/
def inFlight (s : St) : Bool := s.submitted || s.pendingControl

/-- The phase (and call state) in which each operation is enabled. -/
def enabled (s : St) : Op → Bool
  | .start _ => s.phase = 0 && !s.finished
  | .chainCheck _ => s.phase = 1 && !s.dryRun && !s.finished
  | .stateCheck _ => s.phase = 2 && !s.finished
  | .walletModes _ => s.phase = 3 && !s.finished
  | .walletAddr _ => s.phase = 4 && !s.finished
  | .lock acq => (s.phase = 5 || s.phase = 6) && !s.finished &&
      (acq || (!inFlight s && !s.balanced))
  | .deploy g _ => s.phase = 5 && s.lockHeld && 0 ≤ g && g ≤ gateAll && !s.finished
  | .deployConfirm => s.phase = 5 && s.deployPending && !s.finished
  | .prove x => s.phase = 6 && 0 ≤ x && x ≤ 2 && !s.saved && !s.included && !s.unknown && !s.finished
  | .rerun _ _ => s.phase = 6 && !s.submitted && !s.unknown && !s.included && !s.finished
  | .balance _ => s.phase = 6 && s.lockHeld && !inFlight s && s.saved && !s.savedRefused && !s.balanced &&
      !s.submitted && !s.unknown && !s.included && !s.needResync && !s.finished
  | .submit who _ => s.phase = 6 && s.lockHeld && !inFlight s && !s.finished &&
      (if who = 0 then s.balanced && !s.included
       else s.refusalsStarted && s.controls < nControls && who = s.controls + 1)
  | .outcome k => inFlight s && 0 ≤ k && k ≤ 3 && !s.finished
  | .resolve _ => s.unknown && !s.finished
  | .readbackOp _ => s.included && !s.readback && !s.finished
  | .startRefusals => s.phase = 6 && !s.refusalsStarted && !s.finished
  | .log _ => !s.finished
  | .finish => !s.finished && !inFlight s
  | .resync => s.needResync && !s.finished
  | .batch _ i r => 0 ≤ i && 0 ≤ r && !s.finished

def stopped (s : St) (code : Int) : St := { s with phase := 9, stop := code }

def doStart (s : St) (t : Int) : St :=
  -- SCL-007: stop before any transaction is built
  if 0 < t ∧ t < ttlMax then { s with phase := 1, ttlCfg := t } else stopped s stopTtl

def doChain (s : St) (k : Int) : St :=
  -- PVW-001, PVW-002: only devnet `undeployed1` (1) or the pinned Preview genesis (2)
  if k = 1 ∨ k = 2 then { s with phase := 2, chain := k, netConns := s.netConns + 1 }
  else stopped { s with netConns := s.netConns + 1 } stopChain

def doState (s : St) (b : Bool) : St :=
  -- SCL-004: a state folder of another key is refused
  if b then { s with phase := 3 } else stopped s stopStateVk

def doModes (s : St) (b : Bool) : St :=
  -- WAL-010: stop before the phrase is read; WAL-001: read in-process
  if b then { s with phase := 4, walletRead := true } else stopped s stopMode

def doAddr (s : St) (b : Bool) : St :=
  -- WAL-003: stop before the wallet starts
  if b then { s with phase := 5, walletStarted := true } else stopped s stopAddr

def doDeploy (s : St) (g : Int) (f : Bool) : St :=
  -- FRC-002: a sent, unconfirmed deploy is read back from the indexer; no second deploy
  if s.deployPending then { s with phase := 6, deployPending := false, netConns := s.netConns + 1 }
  -- PVW-013: the final deploy needs every gate pass
  else if f = true ∧ g ≠ gateAll then { s with lastRefusal := refGate }
  -- PVW-009: deploy on Preview with the existing wallet
  else { s with deploys := s.deploys + 1, deployPending := true, gate := g, final := f, netConns := s.netConns + 1, lastRefusal := 0 }

def doProve (s : St) (h : Int) : St :=
  -- PVW-010, case A: never on a box the user does not own
  if h = 2 then { s with lastRefusal := refNotOwned }
  -- PVW-010: the final call is proved on an owned GPU box
  else if s.final = true ∧ h ≠ 1 then { s with lastRefusal := refFinalNotGpu }
  else { s with proofs := s.proofs + 1, proofHost := h, saved := true, savedRefused := false, lastRefusal := 0 }

def doRerun (s : St) (ttlLeft : Int) (hashOk : Bool) : St :=
  -- FRC-003, FRC-014: reuse a proven, unsubmitted, unrefused call with > 5 min TTL and a
  -- matching hash; FRC-005: a refused call is never reused
  if s.saved = true ∧ s.savedRefused = false ∧ reuseTtl < ttlLeft ∧ hashOk = true then s
  else { s with saved := false, savedRefused := false, balanced := false }

def doBalance (s : St) (kind : Int) : St :=
  if kind = 1 then { s with balanced := true, netConns := s.netConns + 1 }
  -- FRC-004: a transport error is retried, at most 4 attempts, a resync before each new
  -- attempt; another wallet error stops at once (LEAD PICK, FAL-NETOPS-3); OBS-008: each recorded
  else if kind = 0 ∧ s.balanceAttempts + 1 < maxAttempts then
    { s with balanceAttempts := s.balanceAttempts + 1, needResync := true, errRecords := s.errRecords + 1, netConns := s.netConns + 1 }
  else
    { s with balanceAttempts := s.balanceAttempts + 1, errRecords := s.errRecords + 1, netConns := s.netConns + 1, lockHeld := false, phase := 9, stop := stopBalance }

def doSubmit (s : St) (who host : Int) : St :=
  -- WAL-007: only the machine that holds the wallet submits
  if host ≠ 0 then { s with lastRefusal := refOffHost }
  else if who = 0 then
    { s with submitted := true, honestSubmits := s.honestSubmits + 1, balanced := false, netConns := s.netConns + 1, lastRefusal := 0 }
  -- PVW-006: the controls, in order, after the honest call is included and read back
  else { s with pendingControl := true, controls := s.controls + 1, netConns := s.netConns + 1, lastRefusal := 0 }

def doOutcome (s : St) (k : Int) : St :=
  if s.submitted then
    -- FRC-005, OBS-002: refused honest call is marked refused, error recorded
    if k = 0 then { s with submitted := false, savedRefused := true, errRecords := s.errRecords + 1, lastVerdict := vRefusedByNode }
    -- OBS-001: an included call gets a receipt
    else if k = 1 then { s with submitted := false, included := true, receipts := s.receipts + 1, lastVerdict := vIncluded }
    -- PVW-008: `not-included-timeout`; FRC-013: the inclusion state is unknown
    else if k = 2 then { s with submitted := false, unknown := true, lastVerdict := vTimeout }
    else { s with submitted := false, unknown := true, lastVerdict := vNoNodeVerdict }
  else
    -- PVW-006: a control refused by the node; OBS-002: error recorded
    if k = 0 then { s with pendingControl := false, controlsRefused := s.controlsRefused + 1, errRecords := s.errRecords + 1, lastVerdict := vRefusedByNode }
    -- PVW-012: an included control fails the verdict
    else if k = 1 then { s with pendingControl := false, controlFails := s.controlFails + 1, receipts := s.receipts + 1, lastVerdict := vIncluded }
    -- PVW-008: timeout fails the verdict (no refusal check accepts it)
    else if k = 2 then { s with pendingControl := false, controlFails := s.controlFails + 1, lastVerdict := vTimeout }
    -- PVW-007: no inclusion, no timeout, no node error
    else { s with pendingControl := false, controlFails := s.controlFails + 1, lastVerdict := vNoNodeVerdict }

def doResolve (s : St) (inc : Bool) : St :=
  -- FRC-013: resolve from the node and the indexer; an unindexed call is not marked refused
  if inc then { s with unknown := false, netConns := s.netConns + 1, included := true, receipts := s.receipts + 1 }
  else { s with unknown := false, netConns := s.netConns + 1 }

def doReadback (s : St) (p : Bool) : St :=
  { s with readback := p, netConns := s.netConns + 1 }

def doRefusals (s : St) : St :=
  -- FRC-006: the refusals step needs the readback of the honest call
  if s.readback then { s with refusalsStarted := true, lastRefusal := 0 } else { s with lastRefusal := refNoReadback }

def doLog (s : St) (secret : Bool) : St :=
  -- WAL-002: each secret is replaced with `<seed>`; OBS-005: one line per event
  { s with logLines := s.logLines + 1, redactions := if secret then s.redactions + 1 else s.redactions }

def passed (s : St) : Bool :=
  s.dryRun || (s.included && s.controlsRefused = nControls && s.controlFails = 0)

def doFinish (s : St) : St :=
  -- SCL-002: 0 passed, 4 failed verdict, 1 error
  { s with finished := true, exitCode := if s.stop ≠ 0 then 1 else if passed s then 0 else 4 }

def doBatch (s : St) (progOk : Bool) (inRoot newRoot : Int) : St :=
  -- PVW-011, LPC-009: another program commitment or a stale input root is refused
  if progOk = true ∧ inRoot = s.vRoot then { s with vRoot := newRoot, vBatches := s.vBatches + 1 }
  else { s with vRefused := s.vRefused + 1 }

/-- One operation. A disabled operation leaves the state unchanged (`--step` refuses it). -/
def apply (s : St) (op : Op) : St :=
  if !enabled s op then s else
  match op with
  | .start t => doStart s t
  | .chainCheck k => doChain s k
  | .stateCheck b => doState s b
  | .walletModes b => doModes s b
  | .walletAddr b => doAddr s b
  | .lock acq => { s with lockHeld := acq }
  | .deploy g f => doDeploy s g f
  | .deployConfirm => { s with phase := 6, deployPending := false }
  | .prove h => doProve s h
  | .rerun t h => doRerun s t h
  | .balance ok => doBalance s ok
  | .submit w h => doSubmit s w h
  | .outcome k => doOutcome s k
  | .resolve i => doResolve s i
  | .readbackOp p => doReadback s p
  | .startRefusals => doRefusals s
  | .log b => doLog s b
  | .finish => doFinish s
  | .resync => { s with needResync := false, netConns := s.netConns + 1 }
  | .batch p i r => doBatch s p i r

/-! ## Step domain -/

inductive Refusal where
  | notAnOperation
  | opNotEnabled
  | outsideStateDomain
  deriving Repr, DecidableEq

def Refusal.code : Refusal → String
  | .notAnOperation => "not-an-operation"
  | .opNotEnabled => "op-not-enabled"
  | .outsideStateDomain => "outside-state-domain"

def validState (s : St) : Bool :=
  ((0 ≤ s.phase && s.phase ≤ 6) || s.phase = 9) && 0 ≤ s.stop && s.stop ≤ 7
    && 0 ≤ s.deploys && 0 ≤ s.gate && s.gate ≤ gateAll && 0 ≤ s.proofs
    && 0 ≤ s.balanceAttempts && s.balanceAttempts ≤ maxAttempts && 0 ≤ s.honestSubmits
    && 0 ≤ s.receipts && 0 ≤ s.errRecords && 0 ≤ s.controls && s.controls ≤ nControls
    && 0 ≤ s.controlsRefused && 0 ≤ s.controlFails && 0 ≤ s.netConns && 0 ≤ s.logLines
    && 0 ≤ s.redactions && 0 ≤ s.vBatches && 0 ≤ s.vRefused

def stepRefusals (s : St) (n : Int) : List Refusal :=
  match Op.ofInt n with
  | none => [.notAnOperation] ++ (if validState s then [] else [.outsideStateDomain])
  | some op => (if enabled s op then [] else [.opNotEnabled])
      ++ (if validState s then [] else [.outsideStateDomain])

def stepOp (s : St) (n : Int) : St :=
  match Op.ofInt n with
  | none => s
  | some op => apply s op

def runOps (s : St) : List Int → St
  | [] => s
  | n :: ns => runOps (stepOp s n) ns

def refusals (dryRun : Int) (ops : List Int) : List Refusal :=
  (if dryRun = 0 ∨ dryRun = 1 then [] else [.outsideStateDomain])
    ++ (if ops.all (fun n => (Op.ofInt n).isSome) then [] else [.notAnOperation])

def run (dryRun : Int) (ops : List Int) : St := runOps (init (dryRun = 1)) ops

/-! ## Invariants -/

structure Inv (s : St) : Prop where
  phaseRange : (0 ≤ s.phase ∧ s.phase ≤ 6) ∨ s.phase = 9
  /-- a stop is terminal and named -/
  stopIff : s.phase = 9 ↔ s.stop ≠ 0
  stopRange : 0 ≤ s.stop ∧ s.stop ≤ 7
  /-- SCL-007: once started, the TTL is in range; nothing reaches the network before. -/
  ttlOk : (s.phase ≠ 0 ∧ s.stop ≠ stopTtl) → 0 < s.ttlCfg ∧ s.ttlCfg < ttlMax
  /-- SCL-007: before the start, or after a TTL stop, nothing has happened. -/
  early : (s.phase = 0 ∨ s.stop = stopTtl) → s.netConns = 0 ∧ s.unknown = false ∧
    s.included = false ∧ s.needResync = false ∧ s.submitted = false ∧ s.pendingControl = false ∧
    s.walletRead = false ∧ s.deploys = 0 ∧ s.saved = false
  /-- FRC-002: the deploy count by phase -/
  deployPhase : (s.phase ≤ 4 → s.deploys = 0) ∧ (s.phase = 5 → (s.deploys = 1 ↔ s.deployPending = true)) ∧
    (s.phase = 6 → s.deploys = 1 ∧ s.deployPending = false)
  /-- call flags arise only in phase 6 (or a later stop) -/
  flagsPhase : (s.unknown = true ∨ s.included = true ∨ s.needResync = true ∨ s.submitted = true ∨
    s.pendingControl = true ∨ s.balanced = true ∨ s.saved = true) → s.phase = 6 ∨ (s.phase = 9 ∧ s.stop = stopBalance)
  inFlightNotBalanced : (s.submitted = true ∨ s.pendingControl = true) → s.balanced = false
  ttlNet : 0 < s.netConns → 0 < s.ttlCfg ∧ s.ttlCfg < ttlMax
  /-- PVW-001, PVW-002: after the chain check the chain is devnet or the pinned Preview. -/
  chainOk : 2 ≤ s.phase ∧ s.phase ≤ 6 → s.chain = 1 ∨ s.chain = 2
  /-- PVW-001: no wallet step after a failed chain check; WAL-010: no phrase read after a
  failed mode check. -/
  walletAfterChecks : s.walletRead = true → (s.chain = 1 ∨ s.chain = 2) ∧ s.stop ≠ stopChain ∧
    s.stop ≠ stopStateVk ∧ s.stop ≠ stopMode ∧ s.stop ≠ stopTtl
  /-- WAL-001, WAL-010: the phrase is read exactly from phase 4 on (or stopped later). -/
  readPhase : s.walletRead = true ↔ ((4 ≤ s.phase ∧ s.phase ≤ 6) ∨
    (s.phase = 9 ∧ (s.stop = stopAddr ∨ s.stop = stopBalance)))
  /-- WAL-003: the wallet starts only with the pinned address. -/
  startedPhase : s.walletStarted = true ↔ ((5 ≤ s.phase ∧ s.phase ≤ 6) ∨
    (s.phase = 9 ∧ s.stop = stopBalance))
  /-- SCL-004, WAL-003: no deploy and no proof before the state and address checks. -/
  workAfterChecks : (0 < s.deploys ∨ 0 < s.proofs) → s.walletStarted = true
  /-- SCL-003: the dry-run phase opens 0 network connections. -/
  dryNoNet : s.dryRun = true → s.netConns = 0
  /-- WAL-004: the lock is held while a transaction is balanced, submitted or waited for. -/
  lockOk : (s.balanced = true ∨ s.submitted = true ∨ s.pendingControl = true) → s.lockHeld = true
  /-- FRC-002: at most one deploy. -/
  oneDeploy : 0 ≤ s.deploys ∧ s.deploys ≤ 1 ∧ (s.deployPending = true → s.deploys = 1)
  /-- PVW-009, PVW-013: the final deploy carries every gate pass. -/
  finalGate : s.final = true → s.deploys = 1 → s.gate = gateAll
  finalDeployed : s.final = true → s.deploys = 1
  /-- PVW-010, case A: a saved proof was made on an owned box, the final one on a GPU box. -/
  proofHostOk : s.saved = true → (s.proofHost = 0 ∨ s.proofHost = 1) ∧ (s.final = true → s.proofHost = 1)
  /-- FRC-005: a refused saved call is never balanced or submitted again. -/
  refusedNotReused : (s.balanced = true ∨ s.submitted = true) → s.saved = true ∧ s.savedRefused = false
  /-- FRC-004: at most 4 balancing attempts. -/
  attempts : 0 ≤ s.balanceAttempts ∧ s.balanceAttempts ≤ maxAttempts ∧
    (s.phase ≠ 9 → s.balanceAttempts < maxAttempts)
  /-- FRC-013: no new submission while the inclusion state is unknown; nothing after inclusion. -/
  flight : (s.submitted = true → s.unknown = false ∧ s.included = false) ∧
    (s.unknown = true → s.submitted = false ∧ s.included = false)
  /-- FRC-006, PVW-006: controls only after the readback; at most 4. -/
  readbackIncluded : s.readback = true → s.included = true
  refusalsAfterReadback : s.refusalsStarted = true → s.readback = true
  controlsOk : 0 ≤ s.controls ∧ s.controls ≤ nControls ∧ (0 < s.controls → s.refusalsStarted = true)
  /-- PVW-007, PVW-008, PVW-012: every control ends refused or failed, the open one aside. -/
  controlCount : 0 ≤ s.controlsRefused ∧ 0 ≤ s.controlFails ∧
    s.controlsRefused + s.controlFails + (if s.pendingControl then 1 else 0) = s.controls
  oneInFlight : ¬(s.submitted = true ∧ s.pendingControl = true)
  /-- SCL-002: exit codes 0 passed, 4 failed verdict, 1 error. -/
  exitOk : s.finished = true →
    ((s.exitCode = 1 ↔ s.stop ≠ 0) ∧ (s.exitCode = 0 ↔ (s.stop = 0 ∧ passed s = true)) ∧
     (s.exitCode = 0 ∨ s.exitCode = 1 ∨ s.exitCode = 4))
  /-- WAL-002: redactions never exceed log lines. -/
  redact : 0 ≤ s.redactions ∧ s.redactions ≤ s.logLines
  /-- LPC-009, PVW-011: counters of the validium contract. -/
  validium : 0 ≤ s.vBatches ∧ 0 ≤ s.vRefused
  counts : 0 ≤ s.netConns ∧ 0 ≤ s.proofs ∧ 0 ≤ s.honestSubmits ∧ 0 ≤ s.receipts ∧ 0 ≤ s.errRecords
    ∧ 0 ≤ s.logLines
  gateRange : 0 ≤ s.gate ∧ s.gate ≤ gateAll
  /-- a saved call exists only after the deploy (phase 6, or a later stop) -/
  savedPhase : s.saved = true → s.phase = 6 ∨ s.phase = 9
  /-- a balanced call is neither included nor of unknown state -/
  balancedClean : s.balanced = true → s.included = false ∧ s.unknown = false
  /-- SCL-003: a dry run never leaves phase 1 and raises no call flag -/
  dryFlags : s.dryRun = true → (s.phase ≤ 1 ∨ (s.phase = 9 ∧ s.stop = stopTtl)) ∧
    s.unknown = false ∧ s.included = false ∧ s.needResync = false ∧ s.submitted = false ∧
    s.pendingControl = false ∧ s.balanced = false ∧ s.saved = false

theorem inv_init (b : Bool) : Inv (init b) := by
  constructor <;> simp [init, passed, ttlMax, gateAll, maxAttempts, nControls, stopChain, stopAddr,
    stopMode, stopTtl, stopStateVk, stopBalance]

section
variable (s : St) (h : Inv s)
include h

macro "inv_tac" : tactic => `(tactic| (
  cases h
  constructor <;> simp_all [stopChain, stopAddr, stopMode, stopTtl, stopStateVk, stopBalance,
      refGate, refOffHost, refUnknown, refNoReadback, refNotOwned, refFinalNotGpu, ttlMax, reuseTtl,
      maxAttempts, nControls, gateAll, inFlight, passed] <;> omega))

set_option maxHeartbeats 4000000

theorem inv_start (t : Int) (he : enabled s (.start t) = true) : Inv (doStart s t) := by
  simp [enabled] at he; unfold doStart stopped; split <;> inv_tac
theorem inv_chain (k : Int) (he : enabled s (.chainCheck k) = true) : Inv (doChain s k) := by
  simp [enabled] at he; unfold doChain stopped; split <;> inv_tac
theorem inv_state (b : Bool) (he : enabled s (.stateCheck b) = true) : Inv (doState s b) := by
  simp [enabled] at he; unfold doState stopped; split <;> inv_tac
theorem inv_modes (b : Bool) (he : enabled s (.walletModes b) = true) : Inv (doModes s b) := by
  simp [enabled] at he; unfold doModes stopped; split <;> inv_tac
theorem inv_addr (b : Bool) (he : enabled s (.walletAddr b) = true) : Inv (doAddr s b) := by
  simp [enabled] at he; unfold doAddr stopped; split <;> inv_tac
theorem inv_lock (b : Bool) (he : enabled s (.lock b) = true) : Inv { s with lockHeld := b } := by
  simp [enabled, inFlight] at he; cases b <;> inv_tac
theorem inv_deploy (g : Int) (f : Bool) (he : enabled s (.deploy g f) = true) : Inv (doDeploy s g f) := by
  simp [enabled] at he; unfold doDeploy; (repeat' split) <;> inv_tac
theorem inv_confirm (he : enabled s .deployConfirm = true) :
    Inv { s with phase := 6, deployPending := false } := by
  simp [enabled] at he; inv_tac
theorem inv_prove (x : Int) (he : enabled s (.prove x) = true) : Inv (doProve s x) := by
  simp [enabled] at he; unfold doProve; (repeat' split) <;> inv_tac
theorem inv_rerun (t : Int) (b : Bool) (he : enabled s (.rerun t b) = true) : Inv (doRerun s t b) := by
  simp [enabled] at he; unfold doRerun; split
  · exact h
  · inv_tac
theorem inv_balance (b : Int) (he : enabled s (.balance b) = true) : Inv (doBalance s b) := by
  simp [enabled, inFlight] at he; unfold doBalance; (repeat' split) <;> inv_tac
theorem inv_submit (w x : Int) (he : enabled s (.submit w x) = true) : Inv (doSubmit s w x) := by
  simp [enabled, inFlight] at he; unfold doSubmit; (repeat' split) <;> inv_tac
theorem inv_outcome (k : Int) (he : enabled s (.outcome k) = true) : Inv (doOutcome s k) := by
  simp [enabled, inFlight] at he; unfold doOutcome; (repeat' split) <;> inv_tac
theorem inv_resolve (b : Bool) (he : enabled s (.resolve b) = true) : Inv (doResolve s b) := by
  simp [enabled] at he; unfold doResolve; split <;> inv_tac
theorem inv_readback (b : Bool) (he : enabled s (.readbackOp b) = true) : Inv (doReadback s b) := by
  simp [enabled] at he; unfold doReadback; inv_tac
theorem inv_refusals (he : enabled s .startRefusals = true) : Inv (doRefusals s) := by
  simp [enabled] at he; unfold doRefusals; split <;> inv_tac
theorem inv_log (b : Bool) (he : enabled s (.log b) = true) : Inv (doLog s b) := by
  simp [enabled] at he; unfold doLog; inv_tac
theorem inv_finish (he : enabled s .finish = true) : Inv (doFinish s) := by
  simp [enabled, inFlight] at he
  by_cases h1 : s.stop = 0 <;> by_cases h2 : passed s = true <;>
    simp only [doFinish, h1, h2, ne_eq, not_true_eq_false, not_false_eq_true, ite_true, ite_false] <;>
    (try simp only [Bool.not_eq_true] at h2) <;> inv_tac
theorem inv_resync (he : enabled s .resync = true) :
    Inv { s with needResync := false, netConns := s.netConns + 1 } := by
  simp [enabled] at he; inv_tac
theorem inv_batch (p : Bool) (i r : Int) (he : enabled s (.batch p i r) = true) : Inv (doBatch s p i r) := by
  simp [enabled] at he; unfold doBatch; split <;> inv_tac
end

theorem apply_disabled (s : St) (op : Op) (he : enabled s op = false) : apply s op = s := by
  simp [apply, he]

/-- Every invariant holds after every decoded operation. -/
theorem inv_apply (s : St) (op : Op) (h : Inv s) : Inv (apply s op) := by
  cases hen : enabled s op
  · rw [apply_disabled s op hen]; exact h
  have hn : (!enabled s op) = false := by simp [hen]
  unfold apply; rw [hn]
  rw [ite_eq_right (by decide : ¬ (false = true))]
  cases op with
  | start t => exact inv_start s h t hen
  | chainCheck k => exact inv_chain s h k hen
  | stateCheck b => exact inv_state s h b hen
  | walletModes b => exact inv_modes s h b hen
  | walletAddr b => exact inv_addr s h b hen
  | lock b => exact inv_lock s h b hen
  | deploy g f => exact inv_deploy s h g f hen
  | deployConfirm => exact inv_confirm s h hen
  | prove x => exact inv_prove s h x hen
  | rerun t b => exact inv_rerun s h t b hen
  | balance b => exact inv_balance s h b hen
  | submit w x => exact inv_submit s h w x hen
  | outcome k => exact inv_outcome s h k hen
  | resolve b => exact inv_resolve s h b hen
  | readbackOp b => exact inv_readback s h b hen
  | startRefusals => exact inv_refusals s h hen
  | log b => exact inv_log s h b hen
  | finish => exact inv_finish s h hen
  | resync => exact inv_resync s h hen
  | batch p i r => exact inv_batch s h p i r hen

theorem inv_stepOp (s : St) (n : Int) (h : Inv s) : Inv (stepOp s n) := by
  unfold stepOp
  split
  · exact h
  · rename_i op _
    exact inv_apply s op h

theorem inv_runOps (s : St) (ns : List Int) (h : Inv s) : Inv (runOps s ns) := by
  induction ns generalizing s with
  | nil => exact h
  | cons n ns ih => exact ih _ (inv_stepOp s n h)

theorem inv_run (d : Int) (ns : List Int) : Inv (run d ns) := inv_runOps _ _ (inv_init _)

/-! ## Requirement-level theorems -/

theorem PVW_001_no_wallet_on_other_chain (d : Int) (ns : List Int) :
    (run d ns).walletRead = true → (run d ns).chain = 1 ∨ (run d ns).chain = 2 :=
  fun h => ((inv_run d ns).walletAfterChecks h).1

theorem PVW_002_two_chains (d : Int) (ns : List Int) :
    2 ≤ (run d ns).phase ∧ (run d ns).phase ≤ 6 → (run d ns).chain = 1 ∨ (run d ns).chain = 2 :=
  (inv_run d ns).chainOk

theorem SCL_007_ttl (d : Int) (ns : List Int) :
    0 < (run d ns).netConns → 0 < (run d ns).ttlCfg ∧ (run d ns).ttlCfg < 1209600 :=
  (inv_run d ns).ttlNet

theorem SCL_003_dry_run (ns : List Int) : (run 1 ns).netConns = 0 := by
  have h := (inv_run 1 ns).dryNoNet
  apply h
  unfold run
  suffices ∀ s ns, (runOps s ns).dryRun = s.dryRun by rw [this]; rfl
  intro s ns
  induction ns generalizing s with
  | nil => rfl
  | cons n ns ih =>
    simp only [runOps]; rw [ih]
    unfold stepOp; split
    · rfl
    · unfold apply; split
      · rfl
      · rename_i op _ _
        cases op <;> simp only [doStart, doChain, doState, doModes, doAddr, doDeploy, doProve,
          doRerun, doBalance, doSubmit, doOutcome, doResolve, doReadback, doRefusals, doLog,
          doFinish, doBatch, stopped] <;> (repeat' split) <;> rfl

theorem WAL_004_lock (d : Int) (ns : List Int) :
    ((run d ns).submitted = true ∨ (run d ns).pendingControl = true) → (run d ns).lockHeld = true :=
  fun h => (inv_run d ns).lockOk (by rcases h with h | h <;> simp [h])

theorem FRC_002_one_deploy (d : Int) (ns : List Int) : (run d ns).deploys ≤ 1 :=
  (inv_run d ns).oneDeploy.2.1

theorem PVW_013_final_gate (d : Int) (ns : List Int) :
    (run d ns).final = true → (run d ns).gate = 31 :=
  fun h => (inv_run d ns).finalGate h ((inv_run d ns).finalDeployed h)

theorem PVW_010_final_on_gpu (d : Int) (ns : List Int) :
    (run d ns).final = true → (run d ns).saved = true → (run d ns).proofHost = 1 :=
  fun hf hs => ((inv_run d ns).proofHostOk hs).2 hf

theorem FRC_004_attempts (d : Int) (ns : List Int) : (run d ns).balanceAttempts ≤ 4 :=
  (inv_run d ns).attempts.2.1

theorem FRC_006_refusals_need_readback (d : Int) (ns : List Int) :
    0 < (run d ns).controls → (run d ns).readback = true :=
  fun h => (inv_run d ns).refusalsAfterReadback ((inv_run d ns).controlsOk.2.2 h)

theorem PVW_006_four_controls (d : Int) (ns : List Int) : (run d ns).controls ≤ 4 :=
  (inv_run d ns).controlsOk.2.1

theorem SCL_002_exit_codes (d : Int) (ns : List Int) : (run d ns).finished = true →
    (run d ns).exitCode = 0 ∨ (run d ns).exitCode = 1 ∨ (run d ns).exitCode = 4 :=
  fun h => ((inv_run d ns).exitOk h).2.2

/-- PVW-007, PVW-008, PVW-012: a failed control never gives exit status 0. -/
theorem PVW_012_failed_control_fails (d : Int) (ns : List Int) :
    (run d ns).finished = true → (run d ns).dryRun = false → 0 < (run d ns).controlFails →
    (run d ns).exitCode ≠ 0 := by
  intro hf hd hc h0
  have := (((inv_run d ns).exitOk hf).2.1).mp h0
  simp [passed, hd] at this; omega

/-- FRC-005: after a node refusal, a rerun drops the saved call (a new proof follows). -/
theorem FRC_005_no_reuse_of_refused (s : St) (t : Int) (b : Bool) (hr : s.savedRefused = true) :
    (doRerun s t b).saved = false := by
  unfold doRerun; split
  · rename_i hc; simp_all
  · rfl

/-- FRC-003: a fresh, unrefused saved call with > 5 min TTL and a good hash is reused. -/
theorem FRC_003_reuse (s : St) (t : Int) (hs : s.saved = true) (hr : s.savedRefused = false)
    (ht : 300 < t) : doRerun s t true = s := by
  unfold doRerun; simp [hs, hr, reuseTtl, ht]

/-- FRC-014: a failed hash check never reuses. -/
theorem FRC_014_hash_checked (s : St) (t : Int) : (doRerun s t false).saved = false := by
  unfold doRerun; simp

/-- FRC-013: a submit while the inclusion state is unknown is not enabled. -/
theorem FRC_013_no_submit_while_unknown (s : St) (w x : Int) (hu : s.unknown = true)
    (hi : Inv s) : enabled s (.submit w x) = false := by
  have h1 := hi.flight.2 hu
  have hb : s.balanced = false := by
    cases hb : s.balanced
    · rfl
    · have := hi.balancedClean hb; simp_all
  have hr : s.refusalsStarted = false := by
    cases hr : s.refusalsStarted
    · rfl
    · have := hi.readbackIncluded (hi.refusalsAfterReadback hr); simp_all
  simp [enabled, hb, hr, h1.2]

theorem WAL_002_redactions (d : Int) (ns : List Int) : (run d ns).redactions ≤ (run d ns).logLines :=
  (inv_run d ns).redact.2

/-- PVW-011: another program commitment or a stale root leaves the stored root. -/
theorem PVW_011_validium_refuses (s : St) (p : Bool) (i r : Int)
    (h : p = false ∨ i ≠ s.vRoot) : (doBatch s p i r).vRoot = s.vRoot ∧
      (doBatch s p i r).vRefused = s.vRefused + 1 := by
  unfold doBatch; split
  · rename_i hc; rcases h with h | h <;> simp_all
  · simp

/-! ## Constants -/

/-- SCL-007: 1,209,600 s is 14 days; the spec's scenario value 2,000,000 is refused. -/
theorem ttlMax_derived : ttlMax = 14 * 24 * 3600 ∧ ¬((2000000 : Int) < ttlMax) := by decide
/-- FRC-003: 5 minutes; FRC-004: 4 attempts 30 s apart span 90 s of waits. -/
theorem reuse_and_attempts : reuseTtl = 5 * 60 ∧ maxAttempts = 4 ∧ (maxAttempts - 1) * 30 = 90 := by decide

end NetworkOps

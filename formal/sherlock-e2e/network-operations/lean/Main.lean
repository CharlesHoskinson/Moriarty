import SherlockCodec.Wire
import NetworkOps.Spec

/-! Oracle executable of the Network Operations context: codec only (see `SherlockCodec.Wire`). -/

open SherlockCodec NetworkOps

def fields : List (String × Kind) :=
  [("phase", .int), ("dryRun", .bool), ("ttlCfg", .int), ("chain", .int), ("stop", .int), ("lastRefusal", .int), ("walletRead", .bool), ("walletStarted", .bool), ("lockHeld", .bool), ("deploys", .int), ("deployPending", .bool), ("gate", .int), ("final", .bool), ("proofs", .int), ("proofHost", .int), ("saved", .bool), ("savedRefused", .bool), ("balanced", .bool), ("balanceAttempts", .int), ("needResync", .bool), ("submitted", .bool), ("honestSubmits", .int), ("unknown", .bool), ("included", .bool), ("receipts", .int), ("errRecords", .int), ("readback", .bool), ("refusalsStarted", .bool), ("controls", .int), ("controlsRefused", .int), ("controlFails", .int), ("pendingControl", .bool), ("lastVerdict", .int), ("netConns", .int), ("logLines", .int), ("redactions", .int), ("finished", .bool), ("exitCode", .int), ("vRoot", .int), ("vBatches", .int), ("vRefused", .int)]

def enc (s : St) : List V :=
  [.i s.phase, .b s.dryRun, .i s.ttlCfg, .i s.chain, .i s.stop, .i s.lastRefusal, .b s.walletRead, .b s.walletStarted, .b s.lockHeld, .i s.deploys, .b s.deployPending, .i s.gate, .b s.final, .i s.proofs, .i s.proofHost, .b s.saved, .b s.savedRefused, .b s.balanced, .i s.balanceAttempts, .b s.needResync, .b s.submitted, .i s.honestSubmits, .b s.unknown, .b s.included, .i s.receipts, .i s.errRecords, .b s.readback, .b s.refusalsStarted, .i s.controls, .i s.controlsRefused, .i s.controlFails, .b s.pendingControl, .i s.lastVerdict, .i s.netConns, .i s.logLines, .i s.redactions, .b s.finished, .i s.exitCode, .i s.vRoot, .i s.vBatches, .i s.vRefused]

def dec : List V → Option St
  | (.i x_phase) :: (.b x_dryRun) :: (.i x_ttlCfg) :: (.i x_chain) :: (.i x_stop) :: (.i x_lastRefusal) :: (.b x_walletRead) :: (.b x_walletStarted) :: (.b x_lockHeld) :: (.i x_deploys) :: (.b x_deployPending) :: (.i x_gate) :: (.b x_final) :: (.i x_proofs) :: (.i x_proofHost) :: (.b x_saved) :: (.b x_savedRefused) :: (.b x_balanced) :: (.i x_balanceAttempts) :: (.b x_needResync) :: (.b x_submitted) :: (.i x_honestSubmits) :: (.b x_unknown) :: (.b x_included) :: (.i x_receipts) :: (.i x_errRecords) :: (.b x_readback) :: (.b x_refusalsStarted) :: (.i x_controls) :: (.i x_controlsRefused) :: (.i x_controlFails) :: (.b x_pendingControl) :: (.i x_lastVerdict) :: (.i x_netConns) :: (.i x_logLines) :: (.i x_redactions) :: (.b x_finished) :: (.i x_exitCode) :: (.i x_vRoot) :: (.i x_vBatches) :: (.i x_vRefused) :: [] =>
    some { phase := x_phase, dryRun := x_dryRun, ttlCfg := x_ttlCfg, chain := x_chain, stop := x_stop, lastRefusal := x_lastRefusal, walletRead := x_walletRead, walletStarted := x_walletStarted, lockHeld := x_lockHeld, deploys := x_deploys, deployPending := x_deployPending, gate := x_gate, final := x_final, proofs := x_proofs, proofHost := x_proofHost, saved := x_saved, savedRefused := x_savedRefused, balanced := x_balanced, balanceAttempts := x_balanceAttempts, needResync := x_needResync, submitted := x_submitted, honestSubmits := x_honestSubmits, unknown := x_unknown, included := x_included, receipts := x_receipts, errRecords := x_errRecords, readback := x_readback, refusalsStarted := x_refusalsStarted, controls := x_controls, controlsRefused := x_controlsRefused, controlFails := x_controlFails, pendingControl := x_pendingControl, lastVerdict := x_lastVerdict, netConns := x_netConns, logLines := x_logLines, redactions := x_redactions, finished := x_finished, exitCode := x_exitCode, vRoot := x_vRoot, vBatches := x_vBatches, vRefused := x_vRefused }
  | _ => none

def oracle : Oracle St :=
  { fields, enc, dec,
    stepRefusals := fun s n => (stepRefusals s n).map Refusal.code,
    stepOp,
    params := ["dryRun"],
    refusals := fun ps ops => match ps with
      | [d] => (refusals d ops).map Refusal.code
      | _ => ["outside-capability"],
    run := fun ps ops => match ps with
      | [d] => run d ops
      | _ => init false }

def main (args : List String) : IO UInt32 := SherlockCodec.main oracle args

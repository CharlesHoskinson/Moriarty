import SherlockCodecB
import BoxFleet.Spec

/-! Oracle executable of Box Fleet: codec only (see `SherlockCodecB`). -/

open Lean SherlockCodecB BoxFleet

def fieldNames : List String :=
  ["roster", "slotsUsed", "exclRunning", "remoteJobs", "gpuHolder", "idleMin", "termSent", "termAgeS", "busLog", "shaOk", "transfers", "ownerAway", "q1", "q2", "daemonUp", "socketMode", "keysLoaded", "daemonManifest", "reqManifest", "reqPeerOk", "reqLeaves", "served", "errors", "statsReplies", "statsWitness", "poolFree", "slotsMade", "slotsSpent", "lastPath", "cuda", "nodeEnv", "lEnv", "shardsProved", "runStopped", "stopShard", "stopDevice", "lastAppDev", "lastNodeDev", "lastLDev", "runsStarted", "unitRuns", "claims", "gpuQueries"]

def enc (s : St) : Json := Json.mkObj [
  ("roster", natJ s.roster),
  ("slotsUsed", natJ s.slotsUsed),
  ("exclRunning", toJson s.exclRunning),
  ("remoteJobs", natJ s.remoteJobs),
  ("gpuHolder", natJ s.gpuHolder),
  ("idleMin", natJ s.idleMin),
  ("termSent", toJson s.termSent),
  ("termAgeS", natJ s.termAgeS),
  ("busLog", natJ s.busLog),
  ("shaOk", toJson s.shaOk),
  ("transfers", natJ s.transfers),
  ("ownerAway", toJson s.ownerAway),
  ("q1", natJ s.q1),
  ("q2", natJ s.q2),
  ("daemonUp", toJson s.daemonUp),
  ("socketMode", natJ s.socketMode),
  ("keysLoaded", toJson s.keysLoaded),
  ("daemonManifest", natJ s.daemonManifest),
  ("reqManifest", natJ s.reqManifest),
  ("reqPeerOk", toJson s.reqPeerOk),
  ("reqLeaves", natJ s.reqLeaves),
  ("served", natJ s.served),
  ("errors", natJ s.errors),
  ("statsReplies", natJ s.statsReplies),
  ("statsWitness", natJ s.statsWitness),
  ("poolFree", natJ s.poolFree),
  ("slotsMade", natJ s.slotsMade),
  ("slotsSpent", natJ s.slotsSpent),
  ("lastPath", natJ s.lastPath),
  ("cuda", toJson s.cuda),
  ("nodeEnv", natJ s.nodeEnv),
  ("lEnv", natJ s.lEnv),
  ("shardsProved", natJ s.shardsProved),
  ("runStopped", toJson s.runStopped),
  ("stopShard", natJ s.stopShard),
  ("stopDevice", natJ s.stopDevice),
  ("lastAppDev", natJ s.lastAppDev),
  ("lastNodeDev", natJ s.lastNodeDev),
  ("lastLDev", natJ s.lastLDev),
  ("runsStarted", natJ s.runsStarted),
  ("unitRuns", natJ s.unitRuns),
  ("claims", natJ s.claims),
  ("gpuQueries", natJ s.gpuQueries)]

def dec (j : Json) : Except String St := do
  exactKeys j "state" fieldNames
  pure { roster := ← getNat j "roster",
         slotsUsed := ← getNat j "slotsUsed",
         exclRunning := ← getBool j "exclRunning",
         remoteJobs := ← getNat j "remoteJobs",
         gpuHolder := ← getNat j "gpuHolder",
         idleMin := ← getNat j "idleMin",
         termSent := ← getBool j "termSent",
         termAgeS := ← getNat j "termAgeS",
         busLog := ← getNat j "busLog",
         shaOk := ← getBool j "shaOk",
         transfers := ← getNat j "transfers",
         ownerAway := ← getBool j "ownerAway",
         q1 := ← getNat j "q1",
         q2 := ← getNat j "q2",
         daemonUp := ← getBool j "daemonUp",
         socketMode := ← getNat j "socketMode",
         keysLoaded := ← getBool j "keysLoaded",
         daemonManifest := ← getNat j "daemonManifest",
         reqManifest := ← getNat j "reqManifest",
         reqPeerOk := ← getBool j "reqPeerOk",
         reqLeaves := ← getNat j "reqLeaves",
         served := ← getNat j "served",
         errors := ← getNat j "errors",
         statsReplies := ← getNat j "statsReplies",
         statsWitness := ← getNat j "statsWitness",
         poolFree := ← getNat j "poolFree",
         slotsMade := ← getNat j "slotsMade",
         slotsSpent := ← getNat j "slotsSpent",
         lastPath := ← getNat j "lastPath",
         cuda := ← getBool j "cuda",
         nodeEnv := ← getNat j "nodeEnv",
         lEnv := ← getNat j "lEnv",
         shardsProved := ← getNat j "shardsProved",
         runStopped := ← getBool j "runStopped",
         stopShard := ← getNat j "stopShard",
         stopDevice := ← getNat j "stopDevice",
         lastAppDev := ← getNat j "lastAppDev",
         lastNodeDev := ← getNat j "lastNodeDev",
         lastLDev := ← getNat j "lastLDev",
         runsStarted := ← getNat j "runsStarted",
         unitRuns := ← getNat j "unitRuns",
         claims := ← getNat j "claims",
         gpuQueries := ← getNat j "gpuQueries" }

def ctx : Ctx St where
  fields := fieldNames
  decode := dec
  encode := enc
  stepRefusals s op := (stepRefusals s op).map Refusal.code
  step := step
  runRefusals s ops := (runRefusals s ops).map Refusal.code
  run := run

def main (args : List String) : IO UInt32 := mainWith ctx args

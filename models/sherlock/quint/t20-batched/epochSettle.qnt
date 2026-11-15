// T20 fix 1: operator-settled jobs settle by one acceptCollection per epoch; acceptRun stays for a
// run its own user submits and pays for (owner rule 2026-11-14: no per-event Midnight write).
//
// Ghost counters make the owner rule itself an invariant: ledger writes <= closed epochs that
// carried jobs + user-paid runs, whatever the job count.
//
// MUT:
//   "none"                 the design
//   "per_job_accept_run"   the audit row 2 form: the operator settles each job by its own acceptRun
//   "collection_drops_job" the epoch collection leaves out one pending job
//   "split_epoch"          the operator writes one collection per job at epoch close
module epochSettle {
  const MUT: str

  pure val JOBS = Set(1, 2, 3, 4)        // operator jobs (each one proved run)
  pure val USERS = Set(11, 12)           // runs that their own users submit and pay for
  pure val MAX_EPOCH = 4

  var epoch: int
  var pending: Set[int]                  // jobs proved in the open epoch, held off chain
  var submittedAt: int -> int            // job -> epoch it was proved in
  var settledAt: int -> int              // job -> epoch whose collection holds it (ghost)
  var runs: Set[int]                     // acceptRun claims on chain
  var roots: List[Set[int]]              // accepted collections (members held off chain)
  var ledgerWrites: int                  // every SherlockLedger write transaction
  var opWrites: int -> int               // operator writes per epoch
  var userRuns: int                      // user-paid acceptRun calls
  var busyEpochs: int                    // closed epochs that carried at least one job

  action init = all {
    epoch' = 0, pending' = Set(), submittedAt' = Map(), settledAt' = Map(), runs' = Set(), roots' = List(),
    ledgerWrites' = 0, opWrites' = Map(), userRuns' = 0, busyEpochs' = 0,
  }

  pure def inc(m: int -> int, k: int): int -> int = m.put(k, (if (m.keys().contains(k)) m.get(k) else 0) + 1)

  // the operator proves a job: in the design it waits for the epoch collection
  action proveJob(j: int): bool = all {
    epoch < MAX_EPOCH, not(submittedAt.keys().contains(j)),
    submittedAt' = submittedAt.put(j, epoch),
    if (MUT == "per_job_accept_run") all {
      pending' = pending, runs' = runs.union(Set(j)), settledAt' = settledAt.put(j, epoch),
      ledgerWrites' = ledgerWrites + 1, opWrites' = inc(opWrites, epoch),
    } else all {
      pending' = pending.union(Set(j)), runs' = runs, settledAt' = settledAt,
      ledgerWrites' = ledgerWrites, opWrites' = opWrites,
    },
    epoch' = epoch, roots' = roots, userRuns' = userRuns, busyEpochs' = busyEpochs,
  }

  // a user submits its own run and pays: one acceptRun (OK under the owner rule)
  action userAcceptRun(u: int): bool = all {
    not(runs.contains(u)),
    runs' = runs.union(Set(u)), ledgerWrites' = ledgerWrites + 1, userRuns' = userRuns + 1,
    epoch' = epoch, pending' = pending, submittedAt' = submittedAt, settledAt' = settledAt, roots' = roots,
    opWrites' = opWrites, busyEpochs' = busyEpochs,
  }

  // epoch close: one acceptCollection over every pending job (one root, members off chain)
  action closeEpoch: bool = all {
    epoch < MAX_EPOCH,
    val dropped = if (MUT == "collection_drops_job" and pending.size() > 1) pending.exclude(Set(pending.fold(0, (a, b) => if (b > a) b else a))) else pending
    val nWrites = if (pending.size() == 0) 0 else if (MUT == "split_epoch") pending.size() else 1
    all {
      roots' = if (pending.size() == 0) roots else roots.append(dropped),
      settledAt' = dropped.fold(settledAt, (m, j) => m.put(j, epoch)),
      ledgerWrites' = ledgerWrites + nWrites,
      opWrites' = if (nWrites == 0) opWrites else opWrites.put(epoch, nWrites),
      busyEpochs' = busyEpochs + (if (pending.size() == 0) 0 else 1),
    },
    pending' = Set(), epoch' = epoch + 1,
    submittedAt' = submittedAt, runs' = runs, userRuns' = userRuns,
  }

  action step = any {
    nondet j = JOBS.oneOf()
    proveJob(j),
    nondet u = USERS.oneOf()
    userAcceptRun(u),
    closeEpoch,
  }

  // ---- invariants
  // the owner rule: at most one operator write per epoch
  val OperatorWritesPerEpochAtMostOne = opWrites.keys().forall(e => opWrites.get(e) <= 1)
  // ledger writes do not grow with jobs: closed busy epochs plus user-paid runs
  val LedgerWritesBounded = ledgerWrites <= busyEpochs + userRuns
  // every job proved in a closed epoch is a member of that epoch's collection
  val JobsSettledAtClose = submittedAt.keys().forall(j => submittedAt.get(j) < epoch implies
    (settledAt.keys().contains(j) and settledAt.get(j) == submittedAt.get(j)))
  // operator jobs never enter the runs map (no permanent per-job entry)
  val NoJobInRuns = JOBS.forall(j => not(runs.contains(j)))

  val Inv = OperatorWritesPerEpochAtMostOne and LedgerWritesBounded and JobsSettledAtClose and NoJobInRuns

  // ---- non-vacuity: a collection that batches at least three jobs in one write, and a user run beside it
  val WitBatch = roots.indices().exists(i => roots[i].size() >= 3) and userRuns >= 1
  val NotWitBatch = not(WitBatch)
}

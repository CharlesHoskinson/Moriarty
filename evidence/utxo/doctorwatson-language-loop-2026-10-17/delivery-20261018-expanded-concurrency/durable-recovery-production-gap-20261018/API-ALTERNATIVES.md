# Exact API alternatives for the missing caller

All new names below are **unadopted signature proposals only**. Types that do not exist in K are deliberately not implemented or given fake constructors. Error types denote owner-supplied exact policies/DTOs; no error variant, order, hash recipe, hostile rule, work charge or publication algorithm is authored here. This file is not a compilable patch.

## Existing signatures that the real caller can call today

```rust
pub fn load(src: &str) -> Result<StagedProgram, Error>;
pub fn continuation(text: &str) -> Result<Continuation, Error>;
pub fn resume(
    sp: &StagedProgram,
    c: Continuation,
    e: StageEvent,
    cli_action: Option<&str>,
    cli_cap: Option<UInt>,
) -> Result<StageOutcome, StageRejection>;
```

`commands::simulate::run_resume` calls these in order after file reads. There is no signature that accepts only a supported durable checkpoint plus the retained source pair and returns its reopened state. `resume` cannot be called without an event, and supplying a fabricated false event would run work and make a source policy choice. `decode::continuation` cannot return the absent lifetime/closed fields. `StageOutcome`/`StageSegment` are output-only; they are not a durable history importer.

## Alternative A — explicit versioned durable record boundary

```rust
// Each of these new types requires the still-unadopted recovery DTO contract.
pub fn load_pair(
    base_source: &str,
    recovery_source: &str,
) -> Result<RecoveryProgram, RecoveryLoadError>;

// Receives actual file bytes. Validation and exact refusal DTO are owner-owned.
pub fn reopen_checkpoint(
    program: &RecoveryProgram,
    checkpoint_bytes: &[u8],
) -> Result<ReopenedRecovery, RecoveryReopenError>;

impl ReopenedRecovery {
    pub fn checkpoint(&self) -> &RecoveryCheckpoint;
    pub fn report(&self) -> &RecoveryReopenReport;
}

// Serialization of the exact selected complete carrier; publication is separate.
pub fn checkpoint_bytes(
    checkpoint: &RecoveryCheckpoint,
) -> Result<Vec<u8>, RecoveryExportError>;
```

Missing existing caller: add a real paired branch in `commands::simulate::run` before legacy `Args::legacy()`, selected by adopted `--recovery-module` and `--reopen` argument contract; read retained sources and actual bounded file bytes; call `load_pair`/`reopen_checkpoint`; print the exact adopted report. The existing Core2 route stays unchanged without companion selection. The function must return both pending and closed supported state under the selected new DTO. No false guard event, evaluation or scenario reconstruction can stand in for reopen.

Normal publication then needs a separately adopted CLI-owned boundary with the actual complete carrier from start/resume/recover. Its eventual shape is constrained by D2; it cannot yet be written as `Result<(), io::Error>` while silently discarding known transition/checkpoint disposition after a print/write failure. No signature here pretends D2 is fixed.

## Alternative B — owned process checkpoint boundary

```rust
pub fn load_pair(
    base_source: &str,
    recovery_source: &str,
) -> Result<RecoveryProgram, RecoveryLoadError>;

impl RecoveryProcess {
    pub fn checkpoint(&self) -> Result<RecoveryCheckpoint, RecoveryExportError>;
    pub fn reopen(
        program: RecoveryProgram,
        checkpoint_bytes: &[u8],
    ) -> Result<Self, RecoveryReopenError>;
    pub fn reopen_report(&self) -> Result<RecoveryReopenReport, RecoveryExportError>;
}
```

Same missing CLI branch, but reopening reconstructs the complete owned process instead of exposing an explicit reopened record wrapper. A real durable wire version and owner-defined import/history/accounting rules are still necessary; a private in-memory clone is insufficient. This interface can preserve runtime ownership internally, but its constructor must be implemented from actual bytes and source, not a trusted fixture or caller-supplied validator. It is not adopted merely because private Sherlock and disjoint objects use owned state. D2 still controls publication.

## Narrow legacy alternative — cannot satisfy full recovery

```rust
// Illustrative only: this event-free admission API does not currently exist.
pub fn reopen_continuation(
    sp: &StagedProgram,
    continuation: Continuation,
) -> Result<ReopenedContinuation, ContinuationReopenError>;
```

This could expose only the unchanged old pending object, preserving its currently supported fields. It cannot preserve fields the old DTO never held or represent a terminal recovery checkpoint. Even here the owner must select which event-free checks apply and the exact report/error/work interpretation: existing R5 is event-dependent and R6 is resume-specific. Factoring those checks out without a decision would create a new contract. This alternative is listed to make the boundary concrete, **not recommended as a scope waiver or replacement delivery**.

## Required decision receipt for the next bounded source task

The smallest useful decision identifies A or B (or exact revised signatures), the versioned complete pending/closed carrier, and owner-authored source-bound event-free reopen admission/report/refusal contract. It names what fields are preserved and what reopen does, without introducing maintainer/reviewer identities as public prerequisites. The next source task can then implement a real caller and exact decoder/export portions under that contract; it must not expand into a new framework. Actual writer work waits for the separately explicit D2 publication/failure contract. These decisions and independent freezes remain owed; no vote, owner review, implementation or run is inferred here.

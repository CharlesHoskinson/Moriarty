// Fin-route R0/R1 import and run_segment entry, scoped to the pending segment
// index (FinImage2.segment vs FinOutcome2::Waiting{site}).
// All program/plan/policy/site/capture/snapshot/counter checks are fixed
// successful premises for the two-segment F01 action (await at site 1, ordinal 3).
// Runtime host confirmation: probe_financial_import.py.
module financialSegmentImport {
  pure val SITE = 1
  pure val SEGMENT_COUNT = 2
  type Phase =
    | Supplied
    | Accepted
    | Refused
    | NotReady
    | SegmentRan
    | IndexOutOfBounds
    | InvertedSliceRange
    | PlanCostMismatch
  type State = {
    site: int,
    segment: int,
    canonical: bool,
    retainPolicy: bool,
    guardReady: bool,
    phase: Phase,
  }
  var checkpoint: State
  pure def segmentValid(s: State): bool = s.segment == s.site
  // decode_body deserializes segment: usize; R1/R2/R6/R7 validate Waiting{site},
  // program/plan/policy, committed_logical at site, and committed snapshot,
  // but never compare image.segment with usize::from(site).
  pure def decoderAccepts(s: State): bool =
    s.canonical and s.site == SITE and s.segment >= 0
  action initCase(
    segment: int, canonical: bool, retainPolicy: bool, guardReady: bool
  ): bool =
    checkpoint' = {
      site: SITE,
      segment: segment,
      canonical: canonical,
      retainPolicy: retainPolicy,
      guardReady: guardReady,
      phase: Supplied,
    }
  action init: bool = {
    nondet segment = Set(0, 1, 2, 3).oneOf()
    nondet canonical = Set(false, true).oneOf()
    nondet retainPolicy = Set(false, true).oneOf()
    nondet guardReady = Set(false, true).oneOf()
    initCase(segment, canonical, retainPolicy, guardReady)
  }
  action importStep: bool = all {
    checkpoint.phase == Supplied,
    checkpoint' = {
      ...checkpoint,
      phase: if (decoderAccepts(checkpoint)) Accepted else Refused,
    },
  }
  action resumeStep: bool = all {
    checkpoint.phase == Accepted,
    checkpoint' = if (not(checkpoint.guardReady))
      {...checkpoint, phase: NotReady}
    else if (checkpoint.segment >= SEGMENT_COUNT)
      // sherlock2.rs:260 segments(sp, action)[segment]
      {...checkpoint, phase: IndexOutOfBounds}
    else if (checkpoint.segment < checkpoint.site)
      // begin = ordinal + 1 = 4, end = segments[0].1 = 3 (begin > end)
      if (checkpoint.retainPolicy)
        // sherlock2.rs:137 statements[start..end] in fee_end
        {...checkpoint, phase: InvertedSliceRange}
      else
        {...checkpoint, phase: PlanCostMismatch}
    else
      {...checkpoint, phase: SegmentRan},
  }
  action observeStep: bool = all {
    checkpoint.phase != Supplied,
    checkpoint' = checkpoint,
  }
  action step: bool = any {importStep, resumeStep, observeStep}
  val imported = Set(
    Accepted, NotReady, SegmentRan, IndexOutOfBounds, InvertedSliceRange, PlanCostMismatch
  ).contains(checkpoint.phase)
  val boundsPanicReached = Set(IndexOutOfBounds, InvertedSliceRange).contains(checkpoint.phase)
  val malformedSegmentAccepted = imported and not(segmentValid(checkpoint))
  val notReadyWithMalformedSegment =
    checkpoint.phase == NotReady and not(segmentValid(checkpoint))
  val currentCodeInv = and {
    checkpoint.site == SITE,
    checkpoint.segment >= 0,
    imported implies checkpoint.canonical,
    (checkpoint.phase == Refused) implies not(checkpoint.canonical),
    (checkpoint.phase == SegmentRan) implies segmentValid(checkpoint),
    boundsPanicReached implies not(segmentValid(checkpoint)),
  }
  // Design R0/R1 requirement: a Waiting{site} checkpoint carries segment == site.
  val acceptedSegmentInv = imported implies segmentValid(checkpoint)
}

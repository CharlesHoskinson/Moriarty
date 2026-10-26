// Fin-route R0/R1/R4/R7 import and run_segment entry, scoped to the pending
// segment index (FinImage2.segment vs FinOutcome2::Waiting{site}), site capture
// layout (FinImage2.captures vs plan.sites[site-1].captures), and SI-26
// terminal-only diagnostics (FinImage2.discarded_effects / retained_by_policy).
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
    capturesValid: bool,
    diagnosticsEmpty: bool,
    retainPolicy: bool,
    guardReady: bool,
    phase: Phase,
  }
  var checkpoint: State
  pure def segmentValid(s: State): bool = s.segment == s.site
  pure def metadataValid(s: State): bool = and {
    segmentValid(s),
    s.capturesValid,
    s.diagnosticsEmpty,
  }
  // decode_body deserializes segment, captures, discarded_effects and
  // retained_by_policy; R1/R2/R6/R7 validate Waiting{site}, program/plan/policy,
  // committed_logical at site, and committed snapshot, but:
  // 1. never compare image.segment with usize::from(site),
  // 2. decode::canonical_boundary ignores captures (`let _ = captures;` at
  //    decode.rs:109) and sherlock2::resume omits Core/2's layout_matches check
  //    (admit.rs:345),
  // 3. never require discarded_effects.is_empty() && retained_by_policy.is_empty()
  //    (SI-26) on a Waiting checkpoint, and run_segment never clears them on
  //    Waiting/Halted completion (sherlock2.rs:371-400).
  pure def decoderAccepts(s: State): bool =
    s.canonical and s.site == SITE and s.segment >= 0
  action initCase(
    segment: int, canonical: bool, retainPolicy: bool, guardReady: bool
  ): bool =
    initFullCase(segment, canonical, true, true, retainPolicy, guardReady)
  action initFullCase(
    segment: int,
    canonical: bool,
    capturesValid: bool,
    diagnosticsEmpty: bool,
    retainPolicy: bool,
    guardReady: bool
  ): bool =
    checkpoint' = {
      site: SITE,
      segment: segment,
      canonical: canonical,
      capturesValid: capturesValid,
      diagnosticsEmpty: diagnosticsEmpty,
      retainPolicy: retainPolicy,
      guardReady: guardReady,
      phase: Supplied,
    }
  action init: bool = {
    nondet segment = Set(0, 1, 2, 3).oneOf()
    nondet canonical = Set(false, true).oneOf()
    nondet capturesValid = Set(false, true).oneOf()
    nondet diagnosticsEmpty = Set(false, true).oneOf()
    nondet retainPolicy = Set(false, true).oneOf()
    nondet guardReady = Set(false, true).oneOf()
    initFullCase(segment, canonical, capturesValid, diagnosticsEmpty, retainPolicy, guardReady)
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
  val malformedCapturesAccepted = imported and not(checkpoint.capturesValid)
  val si26DiagnosticsLeakedToHalted =
    checkpoint.phase == SegmentRan and not(checkpoint.diagnosticsEmpty)
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
  // Full R0/R1/R4/SI-26 requirement on FinImage2 structural fields at Waiting{site}.
  val acceptedMetadataInv = imported implies metadataValid(checkpoint)
}

# Sherlock production owner regression tests

Author: fresh Muse Wonder 1.0 HIGH, designated SECURITY owner under the explicit user-authorized Opus-outage fallback. The canonical Opus author call ended on a session limit with no code. No canonical Opus authorship or approval is claimed.

## Attachment and scope

This directory records one source-only cfg(test) regression module appended after the unchanged production body of the [actual production owner runtime](../../../../crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs). The production body remains the exact byte prefix. Only namespace paths inside the appended test module were adapted from the former standalone module graph to the actual crate::sherlock::helper_machine namespace. No production semantics, test assertions, helper logic, framework, counter, limit or admission rule was changed.

Direct-validation cells call the real owner_validate_plan with test-constructed Selected values. Those cells are direct-validation evidence only and are not selector or end-to-end coverage. Stepped cells use CheckedOwnerEngine::new_reviewed and the real owner_step path with the real HostFamilyPlans. Hostile stepped cells delegate to the real family plans and then mutate one written plan field.

## Coverage stated narrowly

The appended source covers the SC05 active and inactive Match name columns, including the nameless case, and the mapped H01 ExprId, H02 FunctionRowId, H03 NameId and H04 Match column and arm checks. H05 is partial: it pins the control Return holder, the Validate helper root, a pushed ValidateTask, alloc_value_slot, the alloc_env value and a pushed RightF frame, and does not claim the remaining holder classes. H06 is partial: it pins SI-08 parent-resume precedence, Disabled and Extension precedence, and the stepped PS-before-P3 helper-work precedence, and does not claim dedicated stepped P6-cap or physical-overflow precedence cells. The mapped N02 through N07 cells and the R6 copied RefC and EnvId provenance cells retain their source-stated limits.

## Open gaps and status

N01, the mandatory full 96-Enabled positive differential, remains UNIMPLEMENTED as an ignored unimplemented cell and remains UNRUN. It must never be reported as passing. Full96, Stage2, native authority and execution eligibility remain open. Native authority is 0, and C4, counters and limits are unchanged.

All tests in this attachment are UNCOMPILED and UNEXECUTED. No compiler, typecheck, test, model, SDK, native or subject execution was performed for this namespace integration. Source05 v2 incident 60460 is nonadmitted and carries no accepted runtime fact or authority inheritance.

The earlier SEC and NONSEC source-only votes concerned the former standalone source bytes. They do not transfer to these changed namespace bytes. Fresh separate SECURITY and NONSECURITY reviews of the exact changed production source and its full module context are required before Root integration. Source approval, if later given, is not execution eligibility.

# R218 real public draft compiler source contract

Status: SOURCE_ONLY / UNRUN. Root authorized ordinary composition against the
actual returned R208 API; fresh R219/R220 designated security reviews remain
separate and no execution is performed here. R207/R208/W/Git are unchanged.

Public positive entry:
`pub fn compile_draft_host(source: &str, config_source: &str, target_source: &str) -> Result<draft_report::DraftCompileArtifact, CompileFailure>`.
It decodes the explicit DraftExecutionConfig and target JSON, derives exact caller
parse/type/copy and owner bounds/layout/run inputs, calls R208
prepare_checked_draft_bounded then admit_checked_draft_bounded, and immediately
projects that same newly admitted/unstepped pair through R112. It never runs B
steps, substitutes an evaluated literal, falls back to unbounded copying or Core,
or manufactures empty/refusal-only artifacts. Source/config/target identities are
hashes of the exact input bytes. Existing Core compile_host is preserved.

DraftPreprocessingExtents has the original10 fields plus the exact6 R208-required
UsizeDecimal fields: max_copy_expr_nodes, max_copy_match_arms, max_copy_name_bytes,
max_copy_list_elements, max_copy_tree_nodes, max_copy_tree_depth. All are required;
no numerical defaults or new policy guards. parser_budget() is unchanged;
copy_budget() is the owner's exact ten-field projection. max_type_depth.0 is
forwarded unchanged. Bounds::owner, Layout::owner and RunInput::owner are the
actual R120 conversions. No ConsumerConfig/action-bound alias is fabricated.

Failure stages preserve full typed payloads: Source->parse via source diagnostic,
Types->typecheck via type diagnostic, Copy->copy via exact SourceCopyError fields,
Admit->admit and AdmitRun->admit-run via existing OwnerWire. Failure while projecting
the unstepped pair is observation with full HostViolation OwnerWire. Every draft
failure has evaluation_performed=false, events_forwarded="0", completed=false.
Copy Budget resource variants retain PascalCase names; limit is a decimal string;
observed/function_index/child_index are decimal strings or null. No new error code.

The existing R120 CLI calls the now-real public body only under the explicit
Sherlock execution profile and draft source profile/closed --entry. Omitted draft
profile keeps Core action behavior; draft --observe remains unsupported. Core/2,
Core and publication/test tails remain unchanged. Actual current lib.rs keeps
financial and typed-word public exports and removes only the two test cfgs on
draft parser/checker registrations. No draft image evaluator is exposed.

Two source-first positive tests in the external `tests/sherlock_draft_compile.rs`
consumer call the public function on exact R115 fixtures
and compare all four R112 selected fields (source_digest, full source, full
preparation and every ProgramView field) to the independently sealed EXPECTED.
Additional metadata checks preserve actual input identities, compile-only flags,
profiles and unimplemented plans. This is not a full genesis/effects oracle.
No implementation output supplies expected cells. Local fixture extents are test
inputs, not defaults/admission policy. Tests remain UNRUN.

All5 owner gaps remain: B Drop transient allocations; A recursive Drop; owner
prepare/admit/admit_run allocations; report/provenance/diagnostic/serialization
allocations; public route/H1 process-wide heap/stack/time guarantees. This source
supplies the public body portion without closing those gaps, untrusted-input
capability, UIREQ022/SCON or native/target/full-language acceptance.

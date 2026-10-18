# Declaration-only publication contract

Source: financial_await_projection_impl direct declaration message; no implementation or output consulted.

Waiting keys: status, advanced, segment, continuation, attempted, committed, residual, financial_observation, await_prefix, qualification, authentication, consumption. Complete omits advanced/continuation; all other listed keys remain. status is waiting or complete.

financial_observation fields: program, input, result, outcome, accounting, tentative_publication. program is {program, plan, program_hash, plan_hash}. input is externally tagged Start{action,scenario,cap} or Resume{continuation,event,cli_action,cli_cap}. Raw Result, FinancialObservation, AccountingAvailability and TentativePublication retain the existing017 external-tag serializers. Stage labels remain existing local-reference-stage / none / untracked. New await_prefix external tag and complete SegmentObservation membership are frozen in basis/NEW-DECLARATION-ONLY-PUBLIC-API.md.

This declares shapes only, not correctness or acceptance. Complete publication literals are independently assembled from input/domain, retained R175 independent components and new018 prefix expectations.

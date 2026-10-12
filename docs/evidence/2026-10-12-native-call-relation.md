# Native call relation ordinary checks

Repository observation: `mori compile-call` now compiles current typed Core
statements to source/static-binding ZKIRv3. `prepare-call --relation` is the real
consumer: it checks exact artifact identity and SDK constraints, executes the
Noop-processed native transcript and checks full state/effects/events/gas/public
inputs. The [author result and public captures](../../deliverables/native-call-relation-2026-10-12/RESULT.md)
record32 ledger tests,6 CLI tests,2 publication controls,15 Quint tests and actual
payment/collection commands. Fresh full source/result audits remain required.

This is ordinary checking of unproven local calls. Production provider/keys,
authenticated deployed context, global funding/custody/balance prefixes,
economic intention authority, history/PCD and settlement remain open. The
processed call is inspection-only; use raw calls for any separately authorized
SDK proving integration. This result consumes no new native proof authority.

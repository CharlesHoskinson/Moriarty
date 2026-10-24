#!/usr/bin/env bash
# F10: bus events validate: schema, event_id determinism, 1 KiB cap, stop-rule dedupe, inbound is data.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/models/doctorwatson"
exec python3 -m unittest \
  factory.tests.test_bus.Events \
  factory.tests.test_bus.StopDedupe \
  factory.tests.test_bus.Hooks \
  factory.tests.test_bus.BusCommands.test_inbox_prints_typed_fields_only \
  factory.tests.test_bus.InProcess.test_oversize_body_is_trimmed_or_refused_never_sent_large \
  factory.tests.test_bus.InProcess.test_rate_caps

# Verifier timeout repair

Experiment observation: a configured verifier can spawn a descendant that keeps
stdout/stderr pipes open. Killing the direct process at the deadline did not emit
Node's `close` event; the API therefore waited beyond its advertised time bound.
The retained reproduction remained pending after1500ms with a250ms timeout.

Root repaired the actual subprocess consumer to settle the error immediately,
clear its timer, kill the direct process and destroy its pipe handles. The same
reproduction now returned BETA_CRYPTO_TIMEOUT after266ms. A checked-in regression
retains the descendant case and cleans up its test process. This bounds the API
and its owned pipe handles; it does not claim control over arbitrary descendant
processes of a caller-selected executable. The installed verifier path is trusted
configuration, never supplied by the source or scenario.

Red/green receipts: external task directory timeout-red.json, timeout-green.json
and timeout-regression-green.txt; these will be retained in the reviewed dossier.

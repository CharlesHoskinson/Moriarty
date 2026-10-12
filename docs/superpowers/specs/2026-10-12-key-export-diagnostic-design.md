# Key export failure diagnostics

This ordinary repair follows the independently reviewed expectations in
`key-export-diagnostic-independent-expectations-01.md` (SHA256
`57f466a7109397576836c924f87dfe39ff532d640d6d5cf89b1ecc023c72cc7e`).
The root adopted those expectations before implementation. Routine repair and
execution decisions are delegated; no additional user confirmation is required.

Attach typed key export metadata to the existing ledger error without changing
its layer, code or legacy Display. Record the first bounded-writer refusal and
distinguish byte cap refusal, byte count overflow and serializer failure. Carry
the actual action, Prover/Verifier kind, previously discovered degree, effective
cap and accepted count. A refused chunk has its length and a checked attempted
minimum; overflow and serializer failure have no numeric attempted minimum.
These counts do not describe the total encoded key size.

Extend the existing one-pass tagged encoding adapter. Count headers as well as
payload. Refuse before append and reject an encoding even when a serializer
ignores or wraps the writer refusal. Never retain raw serializer error text or
emit accepted bytes. Forward existing context through public VK, selected PK
and selected full VK calls, without another degree discovery or size pass.
The first refusal is terminal for further writer appends, preserving its exact
accepted prefix and first-failure metadata.

The CLI keeps its current error category and renders the exact action once as
an ASCII-safe JSON string. Quiet/ColorNever shell reporting still emits errors
on stderr. Existing degree diagnostics retain their current rendering.

Bounds, resource phase counts, financial/compiler semantics, native owners,
parameter admission, decoding, all-action derivation and publication stay under
their existing contracts. This source change alters the source closure; old
exports remain tied to their original frozen producer. No native work is
authorized by this repair.

Use ordinary writer/Serializable fixtures to check every independent control:
boundaries, rejected chunks, empty writes, arithmetic overflow, serializer and
misleading errors, ignored/wrapped refusal, one-pass success, metadata fidelity,
three consumer calls, hostile action escaping and actual quiet shell stderr.

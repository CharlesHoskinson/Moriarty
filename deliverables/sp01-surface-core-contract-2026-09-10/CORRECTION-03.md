# Representable rejection-span correction

Fresh GPT-6 review finding A-1 rejects candidate02 because Rejected.span must be a valid P. Grok51 passed the same candidate under E’s raw offending-span wording. Both reviews remain intact: the conflict is resolved by requiring the diagnostic provenance itself to satisfy the output representation. This is a repair of the existing valid-output obligation, not a change to Boolean selection or admission.

Candidate03 adds an explicit synthetic [0,0) fallback when the offending input span is absent, malformed or out of range. It retains nodePath, error code and workUsed. A valid original source span still propagates exactly. The malformed input [10,15) for a14-byte source remains in BC1-dead-invalid-source-span; only its impossible output span changes.

Three controls cover accepted fallback, rejected raw invalid output, and rejection of a fallback that would erase valid source provenance. The first two failed against candidate02 and all23 pass after repair. No truth values, error codes, work counts or paths changed in the48 cases.

The old124 pins resolve through preservation-map-03.json; five modified files were copied byte-for-byte under candidate-02-original. The original source-candidate-02.json, source patch, both reviewer receipts and strict /0 archive remain unchanged. No source approval is inferred from either prior review for these new bytes. Fresh independent review remains required.

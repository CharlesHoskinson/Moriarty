## APPROVE_WITH_SCOPE

The supplied candidate confirms the required nonsecurity delta: the new manifest differs from the prior manifest only in `run_streaming.py`, now listed as SHA-256 `7b4a615b62604b57208ec023a3412ed2b5afad15b292bd326f31f5cd48df410c` and 4310 bytes, and its supplied text matches the Opus replacement verbatim. All other manifest entries, including `CONFIG.json`, the guard, the Rust sources, and the harness, are unchanged. The new manifest digest supplied for pinning is `73fb5864bbb05e7e0d8d1949fb7bc7250c3e1c02d7bb7b3abffbe118cb94a4b1`.

Accordingly, the prior nonsecurity findings remain applicable to this exact candidate: the same standalone compile job, the same eight fixed `reference_stream_tests`, the same fixed N01, N02, and N05 inputs and bounds, and the same unwired production status and native authority of zero. No execution has been performed or approved as evidence by this review.

This is a nonsecurity confirmation only. It supplies no substitute security judgment on B1 through B4, result attribution, or execution safety; those remain for the designated security reviewer’s fresh confirmation against the new manifest. It also approves no actual results, future production wiring, product acceptance, or any K, Quint, Lean, Clean, circuit, native, U, M, MC, SP, or G exit.

Experimental eleven-API Cardano host helper baseline.

The exact reviewed `host_codec.py` is 23,252 bytes. It retains the original 16,134-byte encoder/request prefix and the 19,096-byte statement prefix unchanged. Its eleven public callables are:

- `encode_data`
- `build_prove_request`
- `validate_prove_request`
- `validate_context_input`
- `parse_statement_header`
- `statement_digest_hex`
- `statement_public_inputs`
- `build_value_data`
- `build_output_reference`
- `build_env_data`
- `build_c2_mint_data`

These APIs build structural Data, validate request shapes, and inspect or hash ABI v2 statement bytes. Request profiles remain Rust `moriarty/1` and `moriarty-core/1`. Core and Rust own financial semantics. Caller context remains `CALLER_SUPPLIED_UNAUTHENTICATED`. No actual external consumer or runner wiring is established.

[Root source adoption](provenance/eleven-api/SOURCE-ADOPTION.json) narrowly adopts these exact source bytes and the captured ordinary results. The [Opus qualification](reviews/eleven-api/opus/ROOT-QUALIFICATION.json) records all 37 SH/DF clauses matched and AGREE, with qualifications. Opus performed the full combined cryptographic/security/source/result review. The [Muse qualification](reviews/eleven-api/muse/ROOT-QUALIFICATION.json) records independent nonsecurity implementation/result readiness. Both audits had 55 pinned inputs. Actual metadata identifies `claude-opus-5-5` and `meta` / `muse-wonder-1.0`. HIGH effort was requested. Actual effort was not independently observed. Opus author/reviewer independence is context-only within the same provider. Neither reviewer substitutes for the other.

[Original captured results and logs](results/eleven-api/original/RESULT.json) record **82 passing methods: 13 Data + 19 statement + 31 request + 19 encoder**. The encoder selection excludes only the historical phase1 API-absence method. The encoder loader adjustment and all new source/tests are exact reviewed bytes. Six request-suite checks compare gold fixtures, with 69 slot lookups and six deferred compiler accounting rows. They are not compiler coverage. Root will perform separate packaged-layout checks. This packet records no such rerun.

`build_value_data(None, tokens)` omits ada and represents canonical no-ada. `build_value_data(0, tokens)` retains a zero entry by DESIGN and is **not a canonical Aiken stdlib Value**. A future consumer must choose explicitly. This note establishes no compiled or ledger correspondence.

Issue payload admission is structural and opaque. Actual verifier sizes remain **488-byte VK and 192-byte proof**. Other admitted sizes serve negative fixtures only. Factory admission establishes no cryptographic encoding, digest correspondence, or proof validity.

Use stable single-threaded inputs. Host validity does not imply Rust byte-parser admission. The reviewed ordinary envelope uses inspected sources, dedicated read-only test directories, no pycache, minimal secret-free environment, 10-second wall/CPU limits and 512 MiB address space. Environmental failures are unclaimed. The statement importlib loader requires this envelope.

The [eleven-API addendum](ELEVEN-API-ADDENDUM.md) explains evidence history and the selected suites. The original [four-API manifest](PUBLICATION-MANIFEST.json), [Opus review](reviews/opus/REPORT.md), [Muse audit](reviews/muse/REPORT.md), [results](results/original/RESULT.json) and inherited contracts remain historical and unchanged. Exact copies of the three replaced files are under `history/four-api/`, including the [four-API README snapshot](history/four-api/README.md). Its original relative links describe the component-root layout at that time. Author-time unrun, pending-review, seven-API and encoder-only docstring fields remain unchanged in frozen records. Use the later qualifications and adoption for current narrow status.

Consumer wiring, host byte emission, Rust parse/from_records/compiler acceptance, compiled Aiken/VM roundtrip, source-to-VK/matrix correspondence, authenticated ledger context and real T0-T3 remain open. Preview-first work and later Preprod acceptance remain open. So do all 20 UTxO families, ACTUS277/18/32, DS01-07, DeFi72, DA01-24, held-outs, intents, regressions, K/Quint, recursion, private handoff, multi-parent, universal and PCD coverage. This is no operator/native, deployment, generic compatibility, ledger or proof certification grant.

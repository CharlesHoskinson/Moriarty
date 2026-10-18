Experimental Cardano host helper baseline, prepared for publication at `experiments/cardano-preprod-host/`.

The module exposes exactly four public APIs: `encode_data`, `build_prove_request`, `validate_prove_request`, and `validate_context_input`. These are pure structural data helpers. The request format references Rust `moriarty/1` source and `moriarty-core/1` semantics. Core and Rust own program semantics. No supplied external caller or runner wiring has been established. The public APIs add no maintainer admission policy.

The exact 16,134-byte module has two separate reviews. [Opus security review](reviews/opus/REPORT.md) records nine clause agreements and no blocking defect. Process metadata records actual model `claude-opus-5-5`. [Muse independent nonsecurity implementation audit](reviews/muse/REPORT.md) records no blocking implementation defect. Its metadata records provider `meta` and model `muse-wonder-1.0`. Both requested HIGH effort. Actual effort was not independently observed. The Opus contract authors and reviewer share the Anthropic provider, with fresh non-author review contexts. Each sourceReady declaration applies only to its recorded narrow scope.

[Captured ordinary results](results/original/RESULT.json) record 31 DTO methods and 19 encoder behavior methods passing on these exact source and test bytes. Six DTO-suite checks compare gold fixtures. They are not compiler coverage. This packet contains no new packaging-verification result. Root will inspect and revalidate the packaged layout separately.

From this component folder, the recorded suite selections are runnable as:

```sh
python3 -I -B test_core_request_wire.py
python3 -I -B test_plutus_data_serialisation.py TestIntegers TestBytes TestContainers TestConstructors TestRejection TestBounds TestInterfaceAndScope.test_exact_signature_and_return_type TestInterfaceAndScope.test_deterministic_and_input_not_mutated
```

The second command selects 19 behavior methods. It excludes only the historical `test_phase1_scope_gate_only_encode_data_public` method under the reviewed I1 scope. The frozen test bytes remain unchanged. The reviewed manual combined-scope inspection covers exactly four public functions and no imports. The default full 20-method encoder command is not claimed to pass.

Calls assume no concurrent input mutation. Host-valid data can still fail Rust byte-parser admission. Context remains `CALLER_SUPPLIED_UNAUTHENTICATED` and does not authenticate ledger facts.

The encoder prefix, including its original docstring, remains byte-identical. Frozen contracts retain historical wording, including the older 18 GREEN statement. Original receipts and earlier reports also retain their author-time pending-review wording. The latest captured 31/19 results are in `results/original/RESULT.json`. The later exact-candidate reviews are in `reviews/opus/` and `reviews/muse/`. The Muse audit completed at 2026-10-16 19:37:26 UTC for this same 16,134-byte baseline. The newer [root Muse qualification](results/original/ROOT-MUSE-QUALIFICATION.json) is current for that audit. Earlier Muse-unavailable wording remains historical.

Host request byte emission, Rust byte-parser and from_records acceptance, native VM correspondence, actual runner wiring, source-to-VK matrix correspondence, and actual ledger context remain open. Real T0/T1/T2/T3 and Preprod transactions remain open. Full 20-family UTxO, financial, ACTUS 277/18/32, DS01-07, DeFi72, DA01-24, held-out, intent, regression, recursion, private-handoff, multi-parent, universal, and PCD coverage remain unestablished. This is an experimental helper baseline. It establishes no deployment, generic compatibility, or proof certification.

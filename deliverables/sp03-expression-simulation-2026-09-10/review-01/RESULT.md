# Reviewed local expression simulation CLI

The CLI now evaluates real `.mori` source against explicit local schema and snapshot files for both accepted expression source profiles. Success displays the initial state and work budget with the exact evaluator result; rejection preserves the API diagnostic and exposes no tentative state or descriptors. The README contains the runnable command.

Terra implemented candidate `d4ee817f6f69a9c2a3a50ab7f77531edd248d964`. Fresh Astra medium and Grok 4.6 high returned **PASS_SCOPED** on the pinned candidate. See [acceptance](accepted-result.json), [Astra review](astra-review.json), and [Grok review](grok-review.json). Grok performed static review of the supplied source and parent execution evidence; Astra additionally ran ten independent CLI probes. The parent verified all 30 manifest file hashes and both review identities before acceptance.

The parent build passed, all 682 language tests passed, and eight additional production CLI probes passed. These cover exact financial output, rejection rollback, work exhaustion, canonical admission priority, and a valid snapshot larger than 65536 bytes. The unchanged base rejected the new command with CLI_USAGE; see [baseline control](baseline-cli-control.json). Final integration checks are recorded separately in `../integration-01/`.

The Moriarty development plugin was loaded and its guarded status/next inspected; see [plugin evidence](../PLUGIN.md). The unrelated legacy SP01 history stop remains intact. This acceptance covers the local CLI adapter only. Full SP03, expression K execution/correspondence, metatheorems, financial transition acceptance and ledger acceptance remain open.

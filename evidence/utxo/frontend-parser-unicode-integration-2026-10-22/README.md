# Parser recovery and Unicode diagnostics

Fifteen reviewed lexer, parser and diagnostics files are integrated into the workstream checkout. The parser reports the next declaration as a missing-value boundary, preserves keyword calls and bare keyword references, and reports a missing comma before a record field instead of treating the field name as an asset. Human diagnostic source attachments can report Unicode-scalar columns while retaining UTF-8 byte spans. Existing callers must attach NamedUtf8Source to obtain that behavior.

The retained frozen candidate runs passed all42 lexer,48 parser and18 diagnostics tests. Their source files match the integrated files. These are108 private-source observations, not a fresh Cargo build or test run in the public checkout. Original fixtures and prior lexer/parser/checker snapshots remain byte-identical. Whole independent opinions, root dispositions and raw test streams are included.

The checker test-loader edit is excluded pending its fresh review. Public CLI/protocol integration, the non-test default/fancy feature matrix, parser traversal/resource safety, complete financial semantics and proof/ledger/target acceptance remain open. The existing suggestion and multi-label/rendering documentation/test gaps are retained in the diagnostics opinion and disposition. No full roadmap or EARS exit is closed by this delivery.

See PUBLICATION.json for exact artifacts, source integration checks and qualifications.

Raw stdout retains its original terminal blank lines. The local .gitattributes rule exempts only those three immutable captures from whitespace checks; source files retain normal checks.

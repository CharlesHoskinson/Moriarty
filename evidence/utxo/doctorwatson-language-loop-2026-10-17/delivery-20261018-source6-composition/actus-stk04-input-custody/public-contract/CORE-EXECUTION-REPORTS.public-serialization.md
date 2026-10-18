## Public serialization and existing compatibility

NR-06 requires a successfully printed normal accepted result, ordinary or metered, to return exit0 and print one complete JSON report followed by one final newline, with no ordinary status text on stderr. The existing bounded pretty public exporter is retained. Complete semantic JSON equality and independently frozen public-byte compatibility SHALL be checked separately. Object member order is not execution semantics; statement, effect and snapshot-row arrays retain their prescribed ordering. Exact bytes must not be inferred from library field declaration order.


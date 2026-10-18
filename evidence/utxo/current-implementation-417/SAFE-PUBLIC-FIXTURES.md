# Public deterministic UNIT fixture classification

The signing-key-shaped literals in native-financial-caller-188.test.mjs and native-financial-consumer-129.test.mjs are deterministic public test inputs. Caller188 line358 uses a repeating single-byte pattern; line635 and consumer129 line319 use the repeated ascending/descending hexadecimal textbook pattern. They are created inside UNIT test directories and consumed by the stub transport or the unsigned/witness-negative test paths. They were not read from real wallet identities or historical native credentials. Their labels and caller unitSpawn/transport path supply this provenance; no genuine capture receipt or native admission is issued by these test fixtures.

This classification applies only to these known literal test inputs. No external wallet files, private keys, raw model logs or runtime key files are copied into the implementation packet. Publication must continue excluding real credentials and key-shaped runtime files.

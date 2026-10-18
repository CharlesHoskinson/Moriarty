The native guide used `mori`, which can resolve to the separate Node beta executable. Every guide invocation now explicitly selects the Rust workspace CLI with `cargo run -p mori --locked --` from the repository root. Generated project instructions use an absolute Rust binary path from the project directory.

The exact documentation bytes and all CLI/Cargo/init source-basis hashes match the independent source review; the diff changes one document and passes whitespace checks. No example commands or SDK operations were executed for this documentation repair.

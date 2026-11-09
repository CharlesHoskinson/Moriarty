You are a builder lane in a controlled experiment. Work only inside the current
directory. It is a copy of a Rust workspace with one file removed.

{spec}
{skeleton_section}
## Rules of this lane

- Change only `{target}`. Changes to any other file are discarded.
- You may read any file in the current directory.
- You may run `cargo check -p mori-backend --lib --offline`,
  `cargo clippy -p mori-backend --no-deps --lib --offline` and `cargo fmt`.
  Do not run `cargo test` (the circuit tests need about 11 GB per thread).
- Do not read or search outside the current directory. Do not use git, the
  network or any other copy of this repository. Do not look for an existing
  implementation of this file; none is available to you.
- Finish when `cargo check -p mori-backend --lib --offline` passes and you
  believe the relation is complete and sound. End with a short summary: what
  the relation constrains, and anything you are unsure of.

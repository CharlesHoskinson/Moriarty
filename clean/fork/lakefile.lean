import Lake

open Lake DSL

package sherlockSpec where
  buildDir := "../../target/sherlock-lean"

@[default_target]
lean_lib SherlockSpec where
  globs := #[.submodules `Clean.Sherlock]

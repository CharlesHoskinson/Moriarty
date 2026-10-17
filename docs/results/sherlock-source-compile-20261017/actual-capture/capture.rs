use std::io::Write;
use mori_core::sherlock_compile_artifact::compile_host;
fn main() {
    let name=std::env::args().nth(1).expect("fixed case required");
    let (source,action)=match name.as_str() {
        "quote" => (include_str!("/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/tests/sherlock_compile_artifact_positive/fixed-quote.mori"),"quote"),
        "eligibility" => (include_str!("/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/tests/sherlock_compile_artifact_positive/fixed-eligibility.mori"),"inspect"),
        _ => panic!("unsupported fixed case"),
    };
    let artifact=compile_host(source,action,include_str!("/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/tests/sherlock_compile_artifact_positive/config.json"),include_str!("/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/28-compiler-wire-binding/target.json"),false)
        .unwrap_or_else(|failure|panic!("positive preparation failed: {}",mori_core::json::export(&failure).unwrap()));
    let raw=mori_core::json::export(&artifact).expect("actual artifact export");
    std::io::stdout().write_all(raw.as_bytes()).unwrap();
    std::io::stdout().write_all(b"\n").unwrap();
}

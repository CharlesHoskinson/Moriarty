//! Sherlock E5: the one BLS12-381 wrap, a `ZkStdLib`-architecture circuit that verifies an
//! `L_Fr` proof natively in Fr. Plan: `research/sherlock-e5-wrap-20261112/PLAN.md`.

pub mod alg;
pub mod native;
pub mod smoke;
pub mod util;
pub mod replay;
pub mod wrapin;
pub mod merkle;
pub mod transcript;
pub mod merkle_test;
pub mod poly;
pub mod verifier;
pub mod verifier_test;

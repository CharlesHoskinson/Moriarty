fn main() { let cases: &[(&str, &[&str], &str)] = &[("case", &[], "needle")]; for (_, _, expected_msg) in cases { assert!("needle".contains(expected_msg)); } }

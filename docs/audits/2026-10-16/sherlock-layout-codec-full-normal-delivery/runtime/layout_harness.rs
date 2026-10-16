//! Fixed N01–N08 standalone codec observation producer, compiled with --cfg test.
#[path = "crates/mori-core/src/sherlock/layout_harness_shim.rs"]
mod sherlock;
fn main() {
    if let Err(error) = sherlock::capture() {
        eprintln!("layout observation capture refused: {error:?}");
        std::process::exit(1);
    }
}

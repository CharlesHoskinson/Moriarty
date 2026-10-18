# Literal blst prebuilt check

Full pinned original: build.rs; SHA256 3b0a6eea2bf5c4d0c327fbe48f81b900e6896bdf92d4cfafcc1010d999b0fea7. Lines61-75 of 247:

```rust
61:      * for trouble-shooting purposes. Idea is that libblst.a can be
62:      * compiled with flags independent from cargo defaults, e.g.
63:      * '../../build.sh -O1 ...'.
64:      */
65:     if Path::new("libblst.a").exists() {
66:         println!("cargo:rustc-link-search=.");
67:         println!("cargo:rustc-link-lib=blst");
68:         println!("cargo:rerun-if-changed=libblst.a");
69:         return;
70:     }
71: 
72:     let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
73: 
74:     let mut blst_base_dir = manifest_dir.join("blst");
75:     if !blst_base_dir.exists() {
```

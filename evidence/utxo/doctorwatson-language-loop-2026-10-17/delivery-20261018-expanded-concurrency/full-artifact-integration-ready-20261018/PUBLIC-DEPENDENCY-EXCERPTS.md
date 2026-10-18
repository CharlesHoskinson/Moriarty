# Public source excerpts; no execution/effect judgment

## compiler-sdk-source-inspection/public-code/zkir-v3/src/ir.rs
SHA256 b0dd55776696f829a9b228ef7558b2b0cc4c4e1ebf4b2e4b23533eb428c01ec8

```text
26: use transient_crypto::curve::Fr;
27: use transient_crypto::proofs::{
28:     ParamsProverProvider, Proof, ProofPreimage, ProverKey, ProvingError, TranscriptHash, Zkir,
29: };
30: 
31: use crate::ir_types::IrType;
32: 
33: /// A low-level IR allowing the prover to populate circuit witnesses.
34: #[cfg_attr(feature = "proptest", derive(Arbitrary))]
35: #[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, Serializable)]
36: #[tag = "ir-source[v3-generic]"]
37: pub struct IrSource {
38:     /// The minor version of this IR.
39:     pub version: IrMinorVersion,
40:     /// The list of input identifiers for this circuit
41:     pub inputs: Vec<TypedIdentifier>,
42:     /// The output types this circuit returns, positionally. The actual
43:     /// values are produced by an `Instruction::Output` terminator whose
44:     /// operand list is type-checked against this signature.
45:     pub outputs: Vec<IrType>,
46:     /// Whether this IR should compile a communications commitment
47:     pub do_communications_commitment: bool,
48:     /// The sequence of instructions to run in-circuit
49:     pub instructions: Arc<Vec<Instruction>>,
50: }
51: tag_enforcement_test!(IrSource);
52: tag_enforcement_test!(ProverKey<IrSource>);
53: 
54: #[cfg_attr(feature = "proptest", derive(Arbitrary))]
55: #[derive(
56:     Clone,
57:     Debug,
58:     Default,
59:     PartialEq,
60:     serde_repr::Serialize_repr,
61:     serde_repr::Deserialize_repr,
62:     Serializable,
63: )]
64: #[tag = "ir-minor-version[v3]"]
65: #[repr(u8)]
```

```text
945: 
946: impl IrSource {
947:     /// Retrieves a model representation of this circuit.
948:     pub fn model(&self) -> Model {
949:         Model {
950:             model: midnight_zk_stdlib::cost_model(self, None),
951:         }
952:     }
953: 
954:     /// Attempts to parse an arbitrary input as IR.
955:     pub fn load<R: Read>(reader: R) -> io::Result<Self> {
956:         let value: serde_json::Value = serde_json::from_reader(reader)?;
957:         match value {
958:             serde_json::Value::Object(mut obj) => {
959:                 let ver = serde_json::from_value(
960:                     obj.get("version")
961:                         .ok_or(io::Error::new(
962:                             io::ErrorKind::InvalidData,
963:                             "Expected a version entry",
964:                         ))?
965:                         .clone(),
966:                 )?;
967:                 match ver {
968:                     SerdeVersion {
969:                         major: 3,
970:                         minor: 0..=0,
971:                     } => {
972:                         obj.insert(
973:                             "version".into(),
974:                             serde_json::Value::Number(ver.minor.into()),
975:                         );
976:                         Ok(serde_json::from_value(serde_json::Value::Object(obj))?)
977:                     }
978:                     SerdeVersion { major, minor } => Err(io::Error::new(
979:                         io::ErrorKind::InvalidData,
980:                         format!("Unhandled version: {major}.{minor}"),
981:                     )),
982:                 }
983:             }
984:             _ => Err(io::Error::new(
985:                 io::ErrorKind::InvalidData,
986:                 "Expected a JSON object",
987:             )),
988:         }
989:     }
990: 
991:     /// Intended for testing only. This method enables fully controlling the inputs passed to
992:     /// proving, to test malicious prover behavior.
```

## compiler-registry-ffi-source-preparation/public-code/blst-0.3.17/build.rs
SHA256 3b0a6eea2bf5c4d0c327fbe48f81b900e6896bdf92d4cfafcc1010d999b0fea7

```text
1: #![allow(unused_imports)]
2: 
3: extern crate cc;
4: 
5: use std::env;
6: use std::path::{Path, PathBuf};
7: 
8: fn assembly(
9:     file_vec: &mut Vec<PathBuf>,
10:     base_dir: &Path,
11:     _arch: &str,
12:     _is_msvc: bool,
13: ) {
14:     #[cfg(target_env = "msvc")]
15:     if _is_msvc {
16:         let sfx = match _arch {
17:             "x86_64" => "x86_64",
18:             "aarch64" => "armv8",
19:             _ => "unknown",
20:         };
21:         let files =
22:             glob::glob(&format!("{}/win64/*-{}.asm", base_dir.display(), sfx))
23:                 .expect("unable to collect assembly files");
24:         for file in files {
25:             file_vec.push(file.unwrap());
26:         }
27:         return;
28:     }
29: 
30:     file_vec.push(base_dir.join("assembly.S"));
31: }
32: 
33: fn main() {
34:     if env::var("CARGO_FEATURE_SERDE_SECRET").is_ok() {
35:         println!(
36:             "cargo:warning=blst: non-production feature serde-secret enabled"
37:         );
38:     }
39: 
40:     // account for cross-compilation [by examining environment variables]
41:     let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
42:     let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
43:     let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
44:     let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
45: 
46:     let target_no_std = target_os.eq("none")
47:         || (target_os.eq("unknown") && target_arch.eq("wasm32"))
48:         || target_os.eq("uefi")
49:         || env::var("BLST_TEST_NO_STD").is_ok();
50: 
51:     if !target_no_std {
52:         println!("cargo:rustc-cfg=feature=\"std\"");
53:         if target_arch.eq("wasm32") || target_os.eq("unknown") {
54:             println!("cargo:rustc-cfg=feature=\"no-threads\"");
55:         }
56:     }
57:     println!("cargo:rerun-if-env-changed=BLST_TEST_NO_STD");
58: 
59:     /*
60:      * Use pre-built libblst.a if there is one. This is primarily
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
76:         // Reach out to ../.., which is the root of the blst repo.
77:         // Use an absolute path to avoid issues with relative paths
78:         // being treated as strings by `cc` and getting concatenated
79:         // in ways that reach out of the OUT_DIR.
80:         blst_base_dir = manifest_dir
81:             .parent()
82:             .and_then(|dir| dir.parent())
83:             .expect("can't access parent of parent of current directory")
84:             .into();
85:     }
86:     println!("Using blst source directory {}", blst_base_dir.display());
87: 
88:     // Set CC environment variable to choose alternative C compiler.
89:     // Optimization level depends on whether or not --release is passed
90:     // or implied.
91: 
92:     if target_os.eq("uefi") && env::var("CC").is_err() {
93:         match std::process::Command::new("clang")
94:             .arg("--version")
95:             .output()
96:         {
97:             Ok(_) => env::set_var("CC", "clang"),
98:             Err(_) => { /* no clang in sight, just ignore the error */ }
99:         }
100:     }
101: 
102:     if target_env.eq("sgx") && env::var("CC").is_err() {
103:         match std::process::Command::new("clang")
104:             .arg("--version")
105:             .output()
106:         {
107:             Ok(out) => {
108:                 let version = String::from_utf8(out.stdout)
109:                     .unwrap_or("unintelligible".to_string());
110:                 if let Some(x) = version.find("clang version ") {
111:                     let x = x + 14;
112:                     let y = version[x..].find('.').unwrap_or(0);
113:                     if version[x..x + y].parse::<i32>().unwrap_or(0) >= 11 {
114:                         env::set_var("CC", "clang");
115:                     }
116:                 }
117:             }
118:             Err(_) => { /* no clang in sight, just ignore the error */ }
119:         }
120:     }
121: 
122:     if target_env.eq("msvc")
123:         && env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().eq("32")
124:         && env::var("CC").is_err()
125:     {
126:         match std::process::Command::new("clang-cl")
127:             .args(["-m32", "--version"])
128:             .output()
129:         {
130:             Ok(out) => {
131:                 if String::from_utf8(out.stdout)
132:                     .unwrap_or("unintelligible".to_string())
133:                     .contains("Target: i386-pc-windows-msvc")
134:                 {
135:                     env::set_var("CC", "clang-cl");
136:                 }
137:             }
138:             Err(_) => { /* no clang-cl in sight, just ignore the error */ }
139:         }
140:     }
141: 
142:     let mut cc = cc::Build::new();
143: 
144:     let c_src_dir = blst_base_dir.join("src");
145:     println!("cargo:rerun-if-changed={}", c_src_dir.display());
146:     let mut file_vec = vec![c_src_dir.join("server.c")];
147: 
148:     if target_arch.eq("x86_64") || target_arch.eq("aarch64") {
149:         let asm_dir = blst_base_dir.join("build");
150:         println!("cargo:rerun-if-changed={}", asm_dir.display());
151:         assembly(
152:             &mut file_vec,
153:             &asm_dir,
154:             &target_arch,
155:             cc.get_compiler().is_like_msvc(),
156:         );
157:     } else {
158:         cc.define("__BLST_NO_ASM__", None);
159:     }
160:     match (cfg!(feature = "portable"), cfg!(feature = "force-adx")) {
161:         (true, false) => {
162:             if target_arch.eq("x86_64") && target_env.eq("sgx") {
163:                 panic!("'portable' is not supported on SGX target");
164:             }
165:             println!("Compiling in portable mode without ISA extensions");
166:             cc.define("__BLST_PORTABLE__", None);
167:         }
168:         (false, true) => {
169:             if target_arch.eq("x86_64") {
170:                 println!("Enabling ADX support via `force-adx` feature");
171:                 cc.define("__ADX__", None);
172:             } else {
173:                 println!("`force-adx` is ignored for non-x86_64 targets");
174:             }
175:         }
176:         (false, false) => {
177:             if target_arch.eq("x86_64") {
178:                 if target_env.eq("sgx") {
179:                     println!("Enabling ADX for Intel SGX target");
180:                     cc.define("__ADX__", None);
181:                 } else if env::var("CARGO_ENCODED_RUSTFLAGS")
182:                     .unwrap_or_default()
183:                     .contains("target-cpu=")
184:                 {
185:                     // If target-cpu is specified on the rustc command line,
186:                     // then obey the resulting target-features.
187:                     let feat_list = env::var("CARGO_CFG_TARGET_FEATURE")
188:                         .unwrap_or_default();
189:                     let features: Vec<_> = feat_list.split(',').collect();
190:                     if !features.contains(&"ssse3") {
191:                         println!(
192:                             "Compiling in portable mode without ISA extensions"
193:                         );
194:                         cc.define("__BLST_PORTABLE__", None);
195:                     } else if features.contains(&"adx") {
196:                         println!(
197:                             "Enabling ADX because it was set as target-feature"
198:                         );
199:                         cc.define("__ADX__", None);
200:                     }
201:                 } else {
202:                     #[cfg(target_arch = "x86_64")]
203:                     if std::is_x86_feature_detected!("adx") {
204:                         println!(
205:                             "Enabling ADX because it was detected on the host"
206:                         );
207:                         cc.define("__ADX__", None);
208:                     }
209:                 }
210:             }
211:         }
212:         (true, true) => panic!(
213:             "Cannot compile with both `portable` and `force-adx` features"
214:         ),
215:     }
216:     if target_env.eq("msvc") && cc.get_compiler().is_like_msvc() {
217:         cc.flag("-Zl");
218:     }
219:     cc.flag_if_supported("-mno-avx") // avoid costly transitions
220:         .flag_if_supported("-fno-builtin")
221:         .flag_if_supported("-Wno-unused-function")
222:         .flag_if_supported("-Wno-unused-command-line-argument");
223:     if target_arch.eq("wasm32") || target_family.is_empty() {
224:         cc.flag("-ffreestanding");
225:     }
226:     if target_arch.eq("wasm32") || target_no_std {
227:         cc.define("SCRATCH_LIMIT", "(45 * 1024)");
228:     }
229:     if target_env.eq("sgx") {
230:         cc.flag_if_supported("-mlvi-hardening");
231:         cc.define("__SGX_LVI_HARDENING__", None);
232:         cc.define("__BLST_NO_CPUID__", None);
233:         cc.define("__ELF__", None);
234:         cc.define("SCRATCH_LIMIT", "(45 * 1024)");
235:     }
236:     if !cfg!(debug_assertions) {
237:         cc.opt_level(2);
238:     }
239:     cc.files(&file_vec).compile("blst");
240: 
241:     // pass some DEP_BLST_* variables to dependents
242:     println!(
243:         "cargo:BINDINGS={}",
244:         blst_base_dir.join("bindings").to_string_lossy()
245:     );
246:     println!("cargo:C_SRC={}", c_src_dir.to_string_lossy());
247: }
```

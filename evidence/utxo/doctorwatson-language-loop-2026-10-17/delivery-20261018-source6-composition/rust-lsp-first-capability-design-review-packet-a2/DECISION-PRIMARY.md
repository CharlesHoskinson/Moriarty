# Slim ordinary design decision: three finite choices

NOT EXECUTED. NO Cargo resolution. These are currently Rust todo! stubs. No production/test patch or lock generation is authorized by this review. Both prior a1 contexts were rejected by root for material invented source facts; all complete objects and comparison tables remain physical and must not become votes. This changed approach presents literal current facts and only the three decisions. Do not reproduce old narratives or repeat full hashes/versions in your reasoning; use the bound table.

D1 recommended: lsp-server0.10.0 public Message::read/Message::write on locked stdin/stdout, synchronous loop. Alternative: full Connection::stdio with its actual three transport threads, or new manual framing over current serde_json. Decide ordinary protocol/library and implementation-fit only.

D2 recommended: alias gen-lsp-types0.11.0 as lsp-types, default-features=false, no features. Generated from3.18, but emit the specified3.17 subset and String diagnostic message only. Without url/fluent-uri Uri is an opaque String wrapper. Alternative original lsp-types0.97.0 has different manifest declarations; full actual manifests remain physical. Root workspace policy declares versions once and child crate uses workspace=true.

D3 recommended: same exact profile routing as existing Rust check, covering beta/1 via parser+checker, Core1 via parse/legacy validate, Core2 via stage::load parse/admit/plan/cost/digest. Alternative beta-only must explicitly report Core unsupported; it cannot pretend Core source is an invalid beta file. Diagnostics remain local-check-only, never compilation or proof/ledger qualification.

Normal design: Full push synchronization with UTF16 positions, explicit local-check-only/profiles metadata, no unrelated editor/MCP/workspace/auth feature. Use actual current diagnostic code/message; append existing help. Primary label (else first) supplies range; existing Core optional offsets produce zero-width ranges, absent offset source-start; no invented full Core spans. Other located labels can be related information subject to client capability. Text is supplied by client; no URI-to-file reads proposed. Normal initialize/initialized, open/full-change/close, shutdown/exit are specified. Exact normal transcript/expectations remain in PROPOSED-INPUT-SESSION.json and PROPOSED-EXPECTED-SESSION.json; they are unrun hand-derived normal predictions, not an observed implementation. The actual-binary regression is required after complete source/lock and root inspection. Neither source approval nor full release/U milestone follows here.

Do not supply security/resource/hostile judgments or recommendations in this ordinary review. Those are separate designated-owner tasks. Source APIs and dependency selection can be reviewed without asserting their adequacy or granting execution. Exact new selected transitive identities remain UNKNOWN; do not replace range minima with inferred lock pins. Current lock is immutable during this task. All original U0-U7/M/MC/SP/G exits and old caps/failures remain unchanged.

===== ACTUAL-SOURCE-FACTS.json COMPLETE =====
{
  "status": "NOT EXECUTED; NO Cargo resolution; Rust LSP/MCP not implemented in observed input",
  "repoCommit": "588d811dbbd9e88d22d3ebe90b0544672870caa0",
  "originalInputManifestSha256": "a337f8a473c885c188957d1199fbe1a3a8ca669818e43251afa8383700e10028",
  "ordinaryEars": {
    "path": "EARS-PROPOSAL.md",
    "ids": [
      "LSP-N01",
      "LSP-N02",
      "LSP-N03",
      "LSP-N04",
      "LSP-N05"
    ],
    "status": "Proposed child clauses, unadopted"
  },
  "normativeEarsPaths": [
    "source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
    "source/openspec/changes/consolidated-language-kernel/traceability.md"
  ],
  "currentCargoLockVersions": {
    "serde": [
      "1.0.229"
    ],
    "serde_derive": [
      "1.0.229"
    ],
    "serde_json": [
      "1.0.151"
    ],
    "log": [
      "0.4.34"
    ],
    "crossbeam-utils": [
      "0.8.23"
    ],
    "crossbeam-channel": [],
    "lsp-server": [],
    "gen-lsp-types": [],
    "lsp-types": []
  },
  "literalTokenDeclaration": {
    "file": "source/crates/mori-lexer/src/lib.rs",
    "name": "MAX_TOKENS",
    "value": 8192,
    "coreCaller": "source/crates/mori-core/src/parser.rs -> mori_lexer::lex"
  },
  "existingConsumers": {
    "rustLsp": "source/crates/mori/src/commands/lsp.rs run -> todo!",
    "rustMcp": "source/crates/mori/src/commands/mcp.rs run -> todo!",
    "rustCheck": "source/crates/mori/src/commands/check.rs",
    "routes": [
      "language first token + moriarty/2 -> mori_core::stage::load",
      "other language first token -> mori_core::parse (includes legacy validate)",
      "otherwise -> mori_parser::parse then mori_checker::check"
    ]
  },
  "proposedDirectVersionsNotCurrentLock": {
    "lsp-server": "=0.10.0",
    "lsp-types alias package gen-lsp-types": "=0.11.0, default-features=false, no features"
  },
  "publishedManifestRangesNotResolvedVersions": {
    "lsp-server0.10.0": {
      "crossbeam-channel": "0.5.15",
      "log": "0.4.27",
      "serde": "1.0.219",
      "serde_derive": "1.0.219",
      "serde_json": "1.0.140"
    },
    "gen-lsp-types0.11.0": {
      "serde": "1.0.228 with derive",
      "serde_json": "1.0.150",
      "optionalFeaturesNotProposed": [
        "url",
        "fluent-uri"
      ]
    }
  },
  "typeProtocolQualification": "gen-lsp-types0.11.0 generated from3.18; proposed serialization emits only specified3.17 subset, Diagnostic.message plain String. Uri is String wrapper with no optional URI feature.",
  "resolvedNewGraph": "UNKNOWN. Manifest ranges are not exact new selected identities. No claim that current log/serde/json pins must be downgraded or replaced.",
  "normalPredicate": "Actual mori lsp binary,11 input messages ->9 ordered complete semantic JSON messages, beta unknown-name UTF16 line2 chars25..33, full edit/close clear, ordinary Core1/Core2 open/close, shutdown/exit0 and empty stderr. Proposed hand-derived UNRUN expectation; not independent finance or execution evidence."
}

===== source/crates/mori/src/commands/lsp.rs lines1..10 of10 =====
1: use std::process::ExitCode;
2: 
3: use crate::shell::Shell;
4: 
5: #[derive(Debug, clap::Args)]
6: pub struct Args {}
7: 
8: pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
9:     todo!("mori lsp")
10: }

===== source/crates/mori/src/commands/mcp.rs lines1..10 of10 =====
1: use std::process::ExitCode;
2: 
3: use crate::shell::Shell;
4: 
5: #[derive(Debug, clap::Args)]
6: pub struct Args {}
7: 
8: pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
9:     todo!("mori mcp")
10: }

===== source/crates/mori/src/commands/check.rs lines1..136 of136 =====
1: use std::path::PathBuf;
2: use std::process::ExitCode;
3: use std::time::Instant;
4: 
5: use mori_diagnostics::{MoriDiagnostic, NamedSource};
6: 
7: use crate::json::CheckReport;
8: use crate::shell::Shell;
9: 
10: #[derive(Debug, clap::Args)]
11: pub struct Args {
12:     /// Source file
13:     pub file: PathBuf,
14: 
15:     /// Print the result as JSON
16:     #[arg(long)]
17:     pub json: bool,
18: }
19: 
20: /// Checks syntax, names, values, types, declarations, calls and intents.
21: pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
22:     let name = args.file.display().to_string();
23:     let source = super::core_io::read_source(&args.file)?;
24: 
25:     if !args.json {
26:         shell.status("Checking", &name);
27:     }
28:     let started = Instant::now();
29:     if mori_lexer::first_token_text(&source).is_ok_and(|token| token == "language") {
30:         let staged = mori_core::language(&source).is_ok_and(|language| language == "moriarty/2");
31:         let result = if staged {
32:             mori_core::stage::load(&source).map(|program| program.program().clone())
33:         } else {
34:             mori_core::parse(&source)
35:         };
36:         if args.json {
37:             let report = match &result {
38:                 Ok(program) => {
39:                     serde_json::json!({"status":"accepted", "file":name,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
40:                 }
41:                 Err(error) => {
42:                     serde_json::json!({"status":"rejected","file":name,"diagnostics":[error]})
43:                 }
44:             };
45:             if staged {
46:                 super::simulate::print_stage_report(report)?;
47:             } else {
48:                 super::core_io::print(&report)?;
49:             }
50:         } else {
51:             match &result {
52:                 Ok(_) => shell.status("Finished", "bounded core check"),
53:                 Err(error) => shell.error(error),
54:             }
55:         }
56:         return Ok(if result.is_ok() {
57:             ExitCode::SUCCESS
58:         } else {
59:             ExitCode::FAILURE
60:         });
61:     }
62:     let (accepted, report) = match mori_parser::parse(&source) {
63:         Err(error) => {
64:             let diagnostics = vec![error];
65:             let report = args
66:                 .json
67:                 .then(|| CheckReport::new(name.clone(), &source, None, &diagnostics));
68:             (
69:                 finish(shell, &name, &source, diagnostics, args.json),
70:                 report,
71:             )
72:         }
73:         Ok(ast) => {
74:             if !args.json {
75:                 shell.verbose_status(
76:                     "Parsed",
77:                     format!("{} tokens, {} nodes", ast.tokens.len(), ast.nodes.len()),
78:                 );
79:             }
80:             let result = mori_checker::check(&ast);
81:             let report = args.json.then(|| {
82:                 CheckReport::new(
83:                     name.clone(),
84:                     &source,
85:                     Some(&result.checked),
86:                     &result.diagnostics,
87:                 )
88:             });
89:             (
90:                 finish(shell, &name, &source, result.diagnostics, args.json),
91:                 report,
92:             )
93:         }
94:     };
95: 
96:     if let Some(report) = report {
97:         super::core_io::print(&report)?;
98:     } else if accepted {
99:         shell.status(
100:             "Finished",
101:             format!("check in {:.2}s", started.elapsed().as_secs_f64()),
102:         );
103:     }
104:     Ok(if accepted {
105:         ExitCode::SUCCESS
106:     } else {
107:         ExitCode::FAILURE
108:     })
109: }
110: 
111: /// Renders the diagnostics, unless the report is JSON. True when there are none.
112: fn finish(
113:     shell: &mut Shell,
114:     name: &str,
115:     source: &str,
116:     diagnostics: Vec<MoriDiagnostic>,
117:     json: bool,
118: ) -> bool {
119:     if diagnostics.is_empty() {
120:         return true;
121:     }
122:     if !json {
123:         let count = diagnostics.len();
124:         for diagnostic in diagnostics {
125:             shell.report(&diagnostic.with_source_code(NamedSource::new(
126:                 crate::display::text(name),
127:                 source.to_owned(),
128:             )));
129:         }
130:         let plural = if count == 1 { "" } else { "s" };
131:         shell.error(format!(
132:             "could not check `{name}` due to {count} previous error{plural}"
133:         ));
134:     }
135:     false
136: }

===== source/crates/mori-parser/src/lib.rs lines1..37 of37 =====
1: //! Parser for `moriarty-beta/1` source.
2: //!
3: //! [`parse`] lexes and parses a file into an [`Ast`], stopping at the first
4: //! error. It checks syntax and syntactic bounds only; names, types and values
5: //! are checked by later phases.
6: 
7: mod diagnostics;
8: mod parser;
9: 
10: #[cfg(test)]
11: mod proptests;
12: #[cfg(test)]
13: mod tests;
14: 
15: use mori_ast::Ast;
16: use mori_diagnostics::MoriDiagnostic;
17: 
18: /// The only profile this parser accepts.
19: pub const PROFILE: &str = "moriarty-beta/1";
20: /// Deepest nesting of expressions and types.
21: pub const MAX_DEPTH: usize = 64;
22: /// Most fields in one record or call, and most type arguments.
23: pub const MAX_FIELDS: usize = 64;
24: /// Most declarations and actions in one agreement.
25: pub const MAX_ITEMS: usize = 256;
26: 
27: /// Lexes and parses `source`, or reports the first error.
28: pub fn parse(source: &str) -> Result<Ast<'_>, MoriDiagnostic> {
29:     let tokens = mori_lexer::lex(source)?;
30:     let (nodes, extra) = parser::Parser::new(source, &tokens).run()?;
31:     Ok(Ast {
32:         source,
33:         tokens,
34:         nodes,
35:         extra,
36:     })
37: }

===== source/crates/mori-checker/src/lib.rs lines34..53 of120 =====
34: /// Everything [`check`] could check, and every error it found. The program
35: /// is accepted only when there are no diagnostics.
36: pub struct CheckResult<'a> {
37:     pub checked: Checked<'a>,
38:     pub diagnostics: Vec<MoriDiagnostic>,
39: }
40: 
41: impl CheckResult<'_> {
42:     pub fn is_accepted(&self) -> bool {
43:         self.diagnostics.is_empty()
44:     }
45: }
46: 
47: /// Index of a declaration in [`Checked`]. An agreement has at most 256 items.
48: #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
49: pub struct DeclIdx(u16);
50: 
51: impl DeclIdx {
52:     pub fn new(index: usize) -> Self {
53:         Self(u16::try_from(index).expect("declaration index exceeds u16"))
Other lines are omitted inline; full file is physically pinned.

===== source/crates/mori-core/src/parser.rs lines1..175 of407 =====
1: use crate::*;
2: use mori_lexer::{TokenIdx, TokenKind};
3: 
4: /// Parses the explicitly versioned core profile; beta grammar is untouched.
5: pub fn parse(source: &str) -> Result<Program, Error> {
6:     parse_profile(source, false)
7: }
8: 
9: /// Syntax-only parser; only stage::load admits the resulting source.
10: pub(crate) fn parse_v2(source: &str) -> Result<Program, Error> {
11:     parse_profile(source, true)
12: }
13: 
14: fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
15:     let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
16:     let tokens = (0..lexed.len())
17:         .map(|i| {
18:             let idx = TokenIdx::new(i);
19:             let span = lexed.span(idx, source);
20:             let raw = &source[span.start as usize..span.end as usize];
21:             let text = if lexed.kind(idx) == TokenKind::String {
22:                 mori_lexer::decode_string(raw)
23:             } else {
24:                 raw.to_owned()
25:             };
26:             Tok {
27:                 text,
28:                 kind: lexed.kind(idx),
29:                 start: span.start,
30:             }
31:         })
32:         .collect();
33:     let mut parser = Parser {
34:         tokens,
35:         pos: 0,
36:         program: Program {
37:             language: if staged {
38:                 crate::stage::LANGUAGE_2
39:             } else {
40:                 LANGUAGE
41:             }
42:             .into(),
43:             semantics: if staged {
44:                 crate::stage::SEMANTICS_2
45:             } else {
46:                 SEMANTICS
47:             }
48:             .into(),
49:             name: String::new(),
50:             states: vec![],
51:             assets: vec![],
52:             expressions: vec![],
53:             actions: vec![],
54:         },
55:     };
56:     parser.expect("language")?;
57:     let profile = parser.string()?;
58:     let expected_language = if staged {
59:         crate::stage::LANGUAGE_2
60:     } else {
61:         LANGUAGE
62:     };
63:     if profile != expected_language {
64:         return Err(parser.error(
65:             "profile",
66:             format!("Expected language \"{expected_language}\"; found {profile:?}."),
67:         ));
68:     }
69:     parser.expect(";")?;
70:     parser.expect("agreement")?;
71:     parser.program.name = parser.word()?;
72:     parser.expect("{")?;
73:     while !parser.eat("}") {
74:         if parser.eat("state") {
75:             let name = parser.word()?;
76:             parser.expect(":")?;
77:             let ty = parser.ty()?;
78:             parser.expect("=")?;
79:             let initial = parser.literal()?;
80:             parser.expect(";")?;
81:             parser.program.states.push(StateDecl { name, ty, initial });
82:         } else if parser.eat("asset") {
83:             let name = parser.word()?;
84:             parser.expect("=")?;
85:             parser.expect("{")?;
86:             parser.expect("network")?;
87:             parser.expect(":")?;
88:             let network = parser.string()?;
89:             parser.expect(",")?;
90:             parser.expect("issuer")?;
91:             parser.expect(":")?;
92:             let issuer = parser.string()?;
93:             parser.expect(",")?;
94:             parser.expect("reference")?;
95:             parser.expect(":")?;
96:             let reference = parser.string()?;
97:             parser.expect(",")?;
98:             parser.expect("scale")?;
99:             parser.expect(":")?;
100:             let Value::UInt(UInt(scale)) = parser.literal()? else {
101:                 return Err(parser.error("asset", "Asset scale must be an unsigned integer."));
102:             };
103:             let scale = u8::try_from(scale)
104:                 .map_err(|_| parser.error("asset", "Asset scale must fit u8."))?;
105:             parser.eat(",");
106:             parser.expect("}")?;
107:             parser.expect(";")?;
108:             parser.program.assets.push(AssetDecl {
109:                 name,
110:                 identity: AssetId {
111:                     network,
112:                     issuer,
113:                     reference,
114:                     scale,
115:                 },
116:             });
117:         } else if parser.eat("action") {
118:             let name = parser.word()?;
119:             parser.expect("(")?;
120:             let mut inputs = vec![];
121:             if !parser.eat(")") {
122:                 loop {
123:                     let name = parser.word()?;
124:                     parser.expect(":")?;
125:                     let ty = parser.ty()?;
126:                     inputs.push(InputDecl { name, ty });
127:                     if parser.eat(")") {
128:                         break;
129:                     }
130:                     parser.expect(",")?;
131:                 }
132:             }
133:             parser.expect("{")?;
134:             let mut statements = vec![];
135:             while !parser.eat("}") {
136:                 if parser.eat("input") {
137:                     if !statements.is_empty() {
138:                         return Err(
139:                             parser.error("input", "Declare action inputs before statements.")
140:                         );
141:                     }
142:                     let name = parser.word()?;
143:                     parser.expect(":")?;
144:                     let ty = parser.ty()?;
145:                     parser.expect(";")?;
146:                     inputs.push(InputDecl { name, ty });
147:                     continue;
148:                 }
149:                 statements.push(parser.statement()?);
150:                 if statements.len() > MAX_STATEMENTS {
151:                     return Err(parser.error("bound", "Action exceeds 1,024 statements."));
152:                 }
153:             }
154:             parser.program.actions.push(Action {
155:                 name,
156:                 inputs,
157:                 statements,
158:             });
159:         } else {
160:             return Err(parser.error("unsupported", "Expected state, asset or action declaration; loops and native calls are unsupported."));
161:         }
162:         if parser.program.states.len() + parser.program.assets.len() + parser.program.actions.len()
163:             > MAX_DECLARATIONS
164:         {
165:             return Err(parser.error("bound", "Program exceeds 256 declarations."));
166:         }
167:     }
168:     if parser.peek().kind != TokenKind::Eof {
169:         return Err(parser.error("syntax", "Unexpected content after agreement."));
170:     }
171:     if !staged {
172:         validate(&parser.program)?;
173:     }
174:     Ok(parser.program)
175: }
Other lines are omitted inline; full file is physically pinned.

===== source/crates/mori-core/src/parser.rs lines366..407 of407 =====
366: pub fn language(source: &str) -> Result<&'static str, Error> {
367:     let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
368:     let mut tokens = Vec::new();
369:     for i in 0..lexed.len() {
370:         let idx = TokenIdx::new(i);
371:         let span = lexed.span(idx, source);
372:         let raw = &source[span.start as usize..span.end as usize];
373:         let kind = lexed.kind(idx);
374:         tokens.push(Tok {
375:             text: if kind == TokenKind::String {
376:                 mori_lexer::decode_string(raw)
377:             } else {
378:                 raw.to_owned()
379:             },
380:             kind,
381:             start: span.start,
382:         });
383:     }
384:     let mut parser = Parser {
385:         tokens,
386:         pos: 0,
387:         program: Program {
388:             language: LANGUAGE.into(),
389:             semantics: SEMANTICS.into(),
390:             name: String::new(),
391:             states: vec![],
392:             assets: vec![],
393:             expressions: vec![],
394:             actions: vec![],
395:         },
396:     };
397:     parser.expect("language")?;
398:     let profile = parser.string()?;
399:     match profile.as_str() {
400:         LANGUAGE => Ok(LANGUAGE),
401:         crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
402:         _ => Err(parser.error(
403:             "profile",
404:             format!("Expected language \"{LANGUAGE}\"; found {profile:?}."),
405:         )),
406:     }
407: }
Other lines are omitted inline; full file is physically pinned.

===== source/crates/mori-core/src/stage/mod.rs lines72..92 of191 =====
72: /// remain `/1`-only, so a `/2` source fails every legacy consumer.
73: pub fn load(src: &str) -> Result<StagedProgram, Error> {
74:     let program = crate::parser::parse_v2(src)?;
75:     profile::require_core2(&program)?;
76:     validate::validate_v2(&program)?;
77:     let mut plan = plan::build(&program)?;
78:     validate::plan_invariants(&program, &plan)?;
79:     cost::fill(&program, &mut plan)?;
80:     validate::cost_invariants(&program, &plan)?;
81:     let program_hash = digest::program_hash(&program)?;
82:     plan.program_hash = digest::hex(&program_hash);
83:     let plan_hash = digest::plan_hash(&plan)?;
84:     Ok(StagedProgram {
85:         program,
86:         plan,
87:         program_hash,
88:         plan_hash,
89:     })
90: }
91: 
92: #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
Other lines are omitted inline; full file is physically pinned.

===== source/crates/mori-core/src/ir.rs lines464..483 of483 =====
464: }
465: decode_via_v1!(Statement, Action, Program, Step, Execution);
466: #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
467: #[error("{code}: {message}")]
468: pub struct Error {
469:     pub code: String,
470:     pub message: String,
471:     pub offset: Option<u32>,
472:     pub step: Option<u16>,
473: }
474: impl Error {
475:     pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
476:         Self {
477:             code: format!("mori::core::{code}"),
478:             message: message.into(),
479:             offset: None,
480:             step: None,
481:         }
482:     }
483: }
Other lines are omitted inline; full file is physically pinned.

===== source/crates/mori-lexer/src/lib.rs lines25..35 of244 =====
25: 
26: pub use crate::token::{Comment, Token, TokenIdx, TokenKind, Tokens};
27: 
28: /// Largest accepted source file, in bytes.
29: pub const MAX_SOURCE_BYTES: usize = 65_536;
30: /// Most tokens in one file, including the end-of-file token.
31: pub const MAX_TOKENS: usize = 8_192;
32: /// Longest identifier or keyword, in bytes.
33: pub const MAX_IDENT_BYTES: usize = 64;
34: /// Longest decoded string, in UTF-8 bytes.
35: pub const MAX_STRING_BYTES: usize = 1_024;
Other lines are omitted inline; full file is physically pinned.

===== public-dependencies/source/lsp-server-0.10.0/src/msg.rs lines165..204 of390 =====
165:     io::Error::new(io::ErrorKind::InvalidData, error)
166: }
167: 
168: macro_rules! invalid_data {
169:     ($($tt:tt)*) => (invalid_data(format!($($tt)*)))
170: }
171: 
172: impl Message {
173:     pub fn read(r: &mut impl BufRead) -> io::Result<Option<Message>> {
174:         Message::_read(r)
175:     }
176:     fn _read(r: &mut dyn BufRead) -> io::Result<Option<Message>> {
177:         let text = match read_msg_text(r)? {
178:             None => return Ok(None),
179:             Some(text) => text,
180:         };
181: 
182:         let msg = match serde_json::from_str(&text) {
183:             Ok(msg) => msg,
184:             Err(e) => {
185:                 return Err(invalid_data!("malformed LSP payload `{e:?}`: {text:?}"));
186:             }
187:         };
188: 
189:         Ok(Some(msg))
190:     }
191:     pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
192:         self._write(w)
193:     }
194:     fn _write(&self, w: &mut dyn Write) -> io::Result<()> {
195:         #[derive(Serialize)]
196:         struct JsonRpc<'a> {
197:             jsonrpc: &'static str,
198:             #[serde(flatten)]
199:             msg: &'a Message,
200:         }
201:         let text = serde_json::to_string(&JsonRpc { jsonrpc: "2.0", msg: self })?;
202:         write_msg_text(w, &text)
203:     }
204: }
Other lines are omitted inline; full file is physically pinned.

===== public-dependencies/source/lsp-server-0.10.0/src/msg.rs lines265..303 of390 =====
265: fn read_msg_text(inp: &mut dyn BufRead) -> io::Result<Option<String>> {
266:     let mut size = None;
267:     let mut buf = String::new();
268:     loop {
269:         buf.clear();
270:         if inp.read_line(&mut buf)? == 0 {
271:             return Ok(None);
272:         }
273:         if !buf.ends_with("\r\n") {
274:             return Err(invalid_data!("malformed header: {:?}", buf));
275:         }
276:         let buf = &buf[..buf.len() - 2];
277:         if buf.is_empty() {
278:             break;
279:         }
280:         let mut parts = buf.splitn(2, ": ");
281:         let header_name = parts.next().unwrap();
282:         let header_value =
283:             parts.next().ok_or_else(|| invalid_data!("malformed header: {:?}", buf))?;
284:         if header_name.eq_ignore_ascii_case("Content-Length") {
285:             size = Some(header_value.parse::<usize>().map_err(invalid_data)?);
286:         }
287:     }
288:     let size: usize = size.ok_or_else(|| invalid_data!("no Content-Length"))?;
289:     let mut buf = buf.into_bytes();
290:     buf.resize(size, 0);
291:     inp.read_exact(&mut buf)?;
292:     let buf = String::from_utf8(buf).map_err(invalid_data)?;
293:     log::debug!("< {buf}");
294:     Ok(Some(buf))
295: }
296: 
297: fn write_msg_text(out: &mut dyn Write, msg: &str) -> io::Result<()> {
298:     log::debug!("> {msg}");
299:     write!(out, "Content-Length: {}\r\n\r\n", msg.len())?;
300:     out.write_all(msg.as_bytes())?;
301:     out.flush()?;
302:     Ok(())
303: }
Other lines are omitted inline; full file is physically pinned.

===== public-dependencies/source/lsp-server-0.10.0/src/stdio.rs lines13..54 of91 =====
13: pub(crate) fn stdio_transport() -> (Sender<Message>, Receiver<Message>, IoThreads) {
14:     let (drop_sender, drop_receiver) = bounded::<Message>(0);
15:     let (writer_sender, writer_receiver) = bounded::<Message>(0);
16:     let writer = thread::Builder::new()
17:         .name("LspServerWriter".to_owned())
18:         .spawn(move || {
19:             let stdout = stdout();
20:             let mut stdout = stdout.lock();
21:             writer_receiver.into_iter().try_for_each(|it| {
22:                 let result = it.write(&mut stdout);
23:                 let _ = drop_sender.send(it);
24:                 result
25:             })
26:         })
27:         .unwrap();
28:     let dropper = thread::Builder::new()
29:         .name("LspMessageDropper".to_owned())
30:         .spawn(move || drop_receiver.into_iter().for_each(drop))
31:         .unwrap();
32:     let (reader_sender, reader_receiver) = bounded::<Message>(0);
33:     let reader = thread::Builder::new()
34:         .name("LspServerReader".to_owned())
35:         .spawn(move || {
36:             let stdin = stdin();
37:             let mut stdin = stdin.lock();
38:             while let Some(msg) = Message::read(&mut stdin)? {
39:                 let is_exit = matches!(&msg, Message::Notification(n) if n.is_exit());
40: 
41:                 debug!("sending message {msg:#?}");
42:                 if let Err(e) = reader_sender.send(msg) {
43:                     return Err(io::Error::other(e));
44:                 }
45: 
46:                 if is_exit {
47:                     break;
48:                 }
49:             }
50:             Ok(())
51:         })
52:         .unwrap();
53:     let threads = IoThreads { reader, writer, dropper };
54:     (writer_sender, reader_receiver, threads)
Other lines are omitted inline; full file is physically pinned.

===== public-dependencies/source/gen-lsp-types-0.11.0/src/generated/common.rs lines26..69 of757 =====
26: /// URIs are transferred as strings. The URI's format is defined in https://tools.ietf.org/html/rfc3986.
27: #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
28: pub struct Uri(pub String);
29: #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
30: impl From<String> for Uri {
31:     fn from(s: String) -> Self {
32:         Self(s)
33:     }
34: }
35: #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
36: impl From<&str> for Uri {
37:     fn from(s: &str) -> Self {
38:         Self(s.into())
39:     }
40: }
41: #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
42: impl From<Box<str>> for Uri {
43:     fn from(s: Box<str>) -> Self {
44:         Self(s.into())
45:     }
46: }
47: #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
48: impl From<Cow<'_, str>> for Uri {
49:     fn from(s: Cow<'_, str>) -> Self {
50:         Self(s.into())
51:     }
52: }
53: #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
54: impl AsRef<str> for Uri {
55:     fn as_ref(&self) -> &str {
56:         &self.0
57:     }
58: }
59: #[cfg(all(not(feature = "url"), not(feature = "fluent-uri")))]
60: impl fmt::Display for Uri {
61:     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
62:         write!(f, "{}", self.0)
63:     }
64: }
65: #[cfg(feature = "url")]
66: pub type Uri = url::Url;
67: #[cfg(all(feature = "fluent-uri", not(feature = "url")))]
68: pub type Uri = fluent_uri::Uri<String>;
69: #[cfg(all(feature = "url", feature = "fluent-uri"))]
Other lines are omitted inline; full file is physically pinned.

===== public-dependencies/source/gen-lsp-types-0.11.0/src/generated/enum_ors.rs lines1182..1192 of1986 =====
1182: #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash)]
1183: #[serde(untagged)]
1184: pub enum Message {
1185:     String(String),
1186:     MarkupContent(MarkupContent),
1187: }
1188: impl From<String> for Message {
1189:     fn from(v: String) -> Self {
1190:         Self::String(v)
1191:     }
1192: }
Other lines are omitted inline; full file is physically pinned.

===== public-dependencies/source/gen-lsp-types-0.11.0/src/generated/structures.rs lines6269..6321 of11109 =====
6269: pub struct Diagnostic {
6270:     /// The range at which the message applies
6271:     pub range: Range,
6272:     /// The diagnostic's severity. To avoid interpretation mismatches when a
6273:     /// server is used with different clients it is highly recommended that servers
6274:     /// always provide a severity value.
6275:     #[serde(skip_serializing_if = "Option::is_none")]
6276:     pub severity: Option<DiagnosticSeverity>,
6277:     /// The diagnostic's code, which usually appear in the user interface.
6278:     #[serde(skip_serializing_if = "Option::is_none")]
6279:     pub code: Option<Code>,
6280:     /// An optional property to describe the error code.
6281:     /// Requires the code field (above) to be present/not null.
6282:     ///
6283:     /// @since 3.16.0
6284:     #[serde(skip_serializing_if = "Option::is_none")]
6285:     pub code_description: Option<CodeDescription>,
6286:     /// A human-readable string describing the source of this
6287:     /// diagnostic, e.g. 'typescript' or 'super lint'. It usually
6288:     /// appears in the user interface.
6289:     #[serde(skip_serializing_if = "Option::is_none")]
6290:     pub source: Option<String>,
6291:     /// The diagnostic's message. It usually appears in the user interface.
6292:     ///
6293:     /// @since 3.18.0 - support for MarkupContent. This is guarded by the client
6294:     /// capability `textDocument.diagnostic.markupMessageSupport`.
6295:     pub message: Message,
6296:     /// Additional metadata about the diagnostic.
6297:     ///
6298:     /// @since 3.15.0
6299:     #[serde(skip_serializing_if = "Option::is_none")]
6300:     pub tags: Option<Vec<DiagnosticTag>>,
6301:     /// An array of related diagnostic information, e.g. when symbol-names within
6302:     /// a scope collide all definitions can be marked via this property.
6303:     #[serde(skip_serializing_if = "Option::is_none")]
6304:     pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
6305:     /// A data entry field that is preserved between a `textDocument/publishDiagnostics`
6306:     /// notification and `textDocument/codeAction` request.
6307:     ///
6308:     /// @since 3.16.0
6309:     #[serde(skip_serializing_if = "Option::is_none")]
6310:     pub data: Option<LspAny>,
6311: }
6312: impl Diagnostic {
6313:     #[must_use]
6314:     pub const fn new(
6315:         range: Range,
6316:         severity: Option<DiagnosticSeverity>,
6317:         code: Option<Code>,
6318:         code_description: Option<CodeDescription>,
6319:         source: Option<String>,
6320:         message: Message,
6321:         tags: Option<Vec<DiagnosticTag>>,
Other lines are omitted inline; full file is physically pinned.

===== EARS-PROPOSAL.md lines1..15 of15 =====
1: # Proposed normal Rust LSP child clauses — unadopted
2: 
3: Existing UNI-018/024, SP02/SP12 and all U/M/MC/SP/G exits stay authoritative. These proposed child clauses do not modify the original clauses or close them.
4: 
5: LSP-N01 — WHEN a public Rust mori lsp client initializes a stdio session, THEN the server SHALL return the literal supported Full-sync UTF16 diagnostic capabilities and local-check-only source-profile metadata; it SHALL advertise no unimplemented editor feature.
6: 
7: LSP-N02 — WHEN the client opens or supplies a newer full version of a Moriarty document, THEN the server SHALL analyze the supplied text through the same profile-selected public parse/check/load APIs as mori check and publish complete diagnostics with the matching URI and version. Profile and Rust-consumer qualification SHALL remain explicit; no compilation/target/evaluation result is inferred.
8: 
9: LSP-N03 — WHEN a normal source error has a primary/fallback source label, THEN the published diagnostic SHALL retain its code/message/help, severity and UTF16 range; other located labels SHALL remain related information. For existing Core error values without full labels it SHALL preserve the existing optional offset and use the proposed explicit zero-width fallback rather than inventing spans.
10: 
11: LSP-N04 — WHEN a normal full edit removes the source mistake, THEN the matching newer version SHALL publish diagnostics []; WHEN the document closes, THEN its retained text is dropped and diagnostics [] is published without a version.
12: 
13: LSP-N05 — WHEN the normal proposed initialization/edit/close/shutdown/exit transcript completes, THEN the actual Rust CLI SHALL return exit0, write exactly the9 specified ordered framed semantic JSON objects on stdout, and leave stderr empty. This finite positive protocol predicate establishes no hostile-input, resource, editor-platform, compiler/SDK/proof or ledger qualification.
14: 
15: Owner pending: input/resource/hostile/security clauses and tests must be separately authored/reviewed by the designated owner. This ordinary proposal does not assert them.

===== source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md lines190..201 of301 =====
190: ### Requirement: UNI-018 Profile and consumer admission
191: When a tool reports checking, compilation or simulation, it SHALL identify the source/Core profile, resolved executable, consumer and admitted subset, and SHALL report unsupported consumer constructs explicitly.
192: 
193: #### Scenario: Supported Rust Core program
194: - **WHEN** A `moriarty/1` program is checked and compiled by its supported Rust consumers.
195: - **THEN** The record names `moriarty-core/1`, both consumers and the construct coverage actually exercised.
196: 
197: #### Scenario: Different authoring profile
198: - **WHEN** A beta or Source/6 program passes its local checker but is outside a Rust compiler's admitted input.
199: - **THEN** The record retains the local checking result and reports the compiler boundary as unsupported; compilation receives no inferred pass.
200: 
201: **Normal accepted report clauses NR-01–03.** These adopted design requirements refine profile/consumer admission through the [normal Core/1 report contract](../../../../../docs/CORE-EXECUTION-REPORTS.md). They cover accepted Let, Requires, Next, Ensures and Emit actions, including the otherwise valid empty action; other statements retain their existing contracts.
Other lines are omitted inline; full file is physically pinned.

===== source/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md lines286..295 of301 =====
286: ### Requirement: UNI-024 Developer journey and current maintainer routing
287: When a developer journey or maintainer task is documented, the documentation SHALL identify implemented local commands, stubs and missing stages, and SHALL apply the current scoped routing without rewriting historical receipts or terminal task contracts.
288: 
289: #### Scenario: Current nonsecurity task
290: - **WHEN** New nonsecurity language work is implemented under the October 14 assignment.
291: - **THEN** The record names GPT-6.1 Sol high authorship and fresh non-author Muse Wonder 1.0 high review of the exact candidate and applicable results, with requested and returned identities retained.
292: 
293: #### Scenario: Incomplete public journey
294: - **WHEN** Local Rust commands and a separate Node beta package exist while Rust LSP/MCP are stubs or a required compile/target stage is unavailable.
295: - **THEN** The journey reports those exact boundaries, leaves U7/M10 open and does not label all CLI commands as stubs or claim an independently qualified release.
Other lines are omitted inline; full file is physically pinned.

## COMPLETE FILE: REVIEW-SCOPE.md
```text
# Ordinary exact-source and actual-result review

Review ONLY the public Core language-header diagnostic increment. No code authoring, runtime commands, builds, network/provider calls, delegation, private-file reads or security judgments. Read the complete inline candidate and actual receipts; relevant whole public source files are physically supplied. Exact range headings are excerpts, not whole-file inspection claims.

Exact immutable base 8269b5b7ceeec5169c706aa31ea33f390d49c969; candidate commit 3bb93ec676340ebca4b772c9ff9277289328c4e0; whole tree 6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4. Parents are 1cb98865f37863152cdf51a38773ca4548adea12 and 8269b5b7ceeec5169c706aa31ea33f390d49c969. Diff versus this base contains exactly parser.rs and tests/language.rs. Both changed base/candidate source bodies and the whole two-file patch are byte-identical to the older 79de/1cb diagnostic increment. This is a NEW composition and NEW actual result, with current public caller context. No old approval/result transfers. Root integration metadata is supplied; source body and receipt hashes independently bind this packet. No approval applies to a hypothetical future base/head/tree or changed consumed input.

SOURCE AXIS: exported mori_core::language() reads a Core header and returns moriarty/1 or moriarty/2. Its fallback previously named only moriarty/1. The candidate names both profiles on unsupported headers and adds both-profile context to missing keyword or quoted-profile errors. Verify actual Error code/offset preservation, unchanged lexer path, success matches, parse dispatch and accepted grammar. Only this function's diagnostic construction changes. Profile-specific parse_profile() still admits one declared profile at a time, with its unchanged corresponding message.

CURRENT PUBLIC CONSUMER LIMIT: check.rs now delegates to check_source::dispatch, whose language(source).is_ok_and(Core2) chooses staged loading or Core1 parsing. The complete CURRENT check.rs and check_source.rs are inline, and current commands/mod.rs is physically supplied. compile/inspect also use is_ok_and and simulate compares Ok(Core2). These routes discard language() errors. This test calls the public library API directly; it does not establish changed CLI error output. Unsupported CLI profiles still pass to the separate parse_profile() diagnostic. Do not imply this increment modifies shared-checker, MCP/LSP, help, compiler/target or feature semantics, or that unsupported source reaches Core2.

RESULT AXIS: root's new exact-source driver ran ONE selected public-library integration test at 3bb93ec676340ebca4b772c9ff9277289328c4e0/6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4, with native features disabled, 90-second timeout and 5-second kill grace, frozen offline dependencies, and a fresh owned temporary root. Actual terminal exit 0; 29.058231172 seconds; 1 passed and 18 filtered. Two supported and four negative case iterations occur inside that one test, not six separately run tests. Assertions compare literal profile/message/code/offset values. Complete new stdout/stderr/start/child/result, plan, source pins and driver are supplied. The driver checks clean source and pins before and after; FINAL-RESULT records owned root removed and session empty. Root explicitly confirmed terminal collection before this packet intake. This is captured evidence, not a rerun by the packet author. Do not invent argv, temporary roots, assertions, report fields or broader checks.

FORMATTING HISTORY: historical-formatting/FORMATTING.json is the preserved ROOT-WRITTEN record from the original integration, not raw formatter stdout/stderr or a new formatting run. It records first author-candidate rustfmt --check exit 1 (root tool 20305a, two tuple layouts only), then formatted check exit 0 (root tool a839d3). No separate raw formatting logs exist. ROOT-FORMATTING.diff shows the exact byte difference, which remains identical for this merged candidate. Preserve this initial failure context. Original author UNRUN/static metadata is historical source preparation. Historical 1cb formatting and the NEW 3bb selected-test receipts are separate evidence families; the older selected-test logs are not offered as current evidence.

BOUNDARIES: no native, SDK relation, circuit, proof/key/wallet/ledger/target or financial execution is established. Ordinary dependency compilation is not cryptographic correctness testing. No full grammar/parser/CLI/property suite or the 18 filtered tests are established. All original U/M/MC/SP/G, finance/composition and target acceptance obligations remain. Do not supply security judgment, resource waiver, provider identity attestation or hypothetical future-base approval. This packet grants no approval.

Deliver exactly one whole structured review with separate sourceFindings and actualResultFindings, accurate filesRead/range coverage, literal evidence, preserved limitations/dissent, and the exact base/head/tree guard. State missing source or contradiction instead of guessing. Scope any approval to this ordinary increment and the new captured result only.

```

## COMPLETE FILE: CANDIDATE.patch
```text
diff --git a/crates/mori-core/src/parser.rs b/crates/mori-core/src/parser.rs
index 34262724..8ecb3c18 100644
--- a/crates/mori-core/src/parser.rs
+++ b/crates/mori-core/src/parser.rs
@@ -394,14 +394,24 @@ pub fn language(source: &str) -> Result<&'static str, Error> {
             actions: vec![],
         },
     };
-    parser.expect("language")?;
-    let profile = parser.string()?;
+    let expected = format!(
+        "language \"{LANGUAGE}\" or \"{}\"",
+        crate::stage::LANGUAGE_2
+    );
+    parser.expect("language").map_err(|mut error| {
+        error.message = format!("Expected {expected}. {}", error.message);
+        error
+    })?;
+    let profile = parser.string().map_err(|mut error| {
+        error.message = format!("Expected {expected}. {}", error.message);
+        error
+    })?;
     match profile.as_str() {
         LANGUAGE => Ok(LANGUAGE),
         crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
         _ => Err(parser.error(
             "profile",
-            format!("Expected language \"{LANGUAGE}\"; found {profile:?}."),
+            format!("Expected {expected}; found {profile:?}."),
         )),
     }
 }
diff --git a/crates/mori-core/tests/language.rs b/crates/mori-core/tests/language.rs
index 4c43ee59..95a7226a 100644
--- a/crates/mori-core/tests/language.rs
+++ b/crates/mori-core/tests/language.rs
@@ -424,3 +424,52 @@ fn quoted_operators_cannot_become_syntax() {
         r#"language "moriarty/1"; agreement Quoted { action run() { requires 1 = "=" 1; } }"#;
     assert!(parse(source).is_err(), "quoted tail cannot form equality");
 }
+
+#[test]
+fn language_header_diagnostics_name_both_supported_core_profiles() {
+    for (source, expected) in [
+        (
+            r#"language "moriarty/1"; agreement Example {}"#,
+            "moriarty/1",
+        ),
+        (
+            r#"language "moriarty/2"; agreement Example {}"#,
+            "moriarty/2",
+        ),
+    ] {
+        assert_eq!(language(source).unwrap(), expected);
+    }
+
+    let expected = r#"Expected language "moriarty/1" or "moriarty/2""#;
+    for (source, code, offset, suffix) in [
+        (
+            r#"language "other";"#,
+            "mori::core::profile",
+            16,
+            r#"; found "other"."#,
+        ),
+        (
+            "",
+            "mori::core::syntax",
+            0,
+            r#". Expected "language"; found ""."#,
+        ),
+        (
+            "agreement Empty {}",
+            "mori::core::syntax",
+            0,
+            r#". Expected "language"; found "agreement"."#,
+        ),
+        (
+            "language ;",
+            "mori::core::syntax",
+            9,
+            ". Expected a quoted string.",
+        ),
+    ] {
+        let error = language(source).unwrap_err();
+        assert_eq!(error.code, code, "{source:?}");
+        assert_eq!(error.offset, Some(offset), "{source:?}");
+        assert_eq!(error.message, format!("{expected}{suffix}"), "{source:?}");
+    }
+}

```

## COMPLETE FILE: ROOT-FORMATTING.diff
```text
--- original-author/crates/mori-core/tests/language.rs
+++ root-formatted/crates/mori-core/tests/language.rs
@@ -428,8 +428,14 @@
 #[test]
 fn language_header_diagnostics_name_both_supported_core_profiles() {
     for (source, expected) in [
-        (r#"language "moriarty/1"; agreement Example {}"#, "moriarty/1"),
-        (r#"language "moriarty/2"; agreement Example {}"#, "moriarty/2"),
+        (
+            r#"language "moriarty/1"; agreement Example {}"#,
+            "moriarty/1",
+        ),
+        (
+            r#"language "moriarty/2"; agreement Example {}"#,
+            "moriarty/2",
+        ),
     ] {
         assert_eq!(language(source).unwrap(), expected);
     }

```

## COMPLETE FILE: candidate/crates/mori-core/src/parser.rs
```text
use crate::*;
use mori_lexer::{TokenIdx, TokenKind};

/// Parses the explicitly versioned core profile; beta grammar is untouched.
pub fn parse(source: &str) -> Result<Program, Error> {
    parse_profile(source, false)
}

/// Syntax-only parser; only stage::load admits the resulting source.
pub(crate) fn parse_v2(source: &str) -> Result<Program, Error> {
    parse_profile(source, true)
}

fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let tokens = (0..lexed.len())
        .map(|i| {
            let idx = TokenIdx::new(i);
            let span = lexed.span(idx, source);
            let raw = &source[span.start as usize..span.end as usize];
            let text = if lexed.kind(idx) == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            };
            Tok {
                text,
                kind: lexed.kind(idx),
                start: span.start,
            }
        })
        .collect();
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: if staged {
                crate::stage::LANGUAGE_2
            } else {
                LANGUAGE
            }
            .into(),
            semantics: if staged {
                crate::stage::SEMANTICS_2
            } else {
                SEMANTICS
            }
            .into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    parser.expect("language")?;
    let profile = parser.string()?;
    let expected_language = if staged {
        crate::stage::LANGUAGE_2
    } else {
        LANGUAGE
    };
    if profile != expected_language {
        return Err(parser.error(
            "profile",
            format!("Expected language \"{expected_language}\"; found {profile:?}."),
        ));
    }
    parser.expect(";")?;
    parser.expect("agreement")?;
    parser.program.name = parser.word()?;
    parser.expect("{")?;
    while !parser.eat("}") {
        if parser.eat("state") {
            let name = parser.word()?;
            parser.expect(":")?;
            let ty = parser.ty()?;
            parser.expect("=")?;
            let initial = parser.literal()?;
            parser.expect(";")?;
            parser.program.states.push(StateDecl { name, ty, initial });
        } else if parser.eat("asset") {
            let name = parser.word()?;
            parser.expect("=")?;
            parser.expect("{")?;
            parser.expect("network")?;
            parser.expect(":")?;
            let network = parser.string()?;
            parser.expect(",")?;
            parser.expect("issuer")?;
            parser.expect(":")?;
            let issuer = parser.string()?;
            parser.expect(",")?;
            parser.expect("reference")?;
            parser.expect(":")?;
            let reference = parser.string()?;
            parser.expect(",")?;
            parser.expect("scale")?;
            parser.expect(":")?;
            let Value::UInt(UInt(scale)) = parser.literal()? else {
                return Err(parser.error("asset", "Asset scale must be an unsigned integer."));
            };
            let scale = u8::try_from(scale)
                .map_err(|_| parser.error("asset", "Asset scale must fit u8."))?;
            parser.eat(",");
            parser.expect("}")?;
            parser.expect(";")?;
            parser.program.assets.push(AssetDecl {
                name,
                identity: AssetId {
                    network,
                    issuer,
                    reference,
                    scale,
                },
            });
        } else if parser.eat("action") {
            let name = parser.word()?;
            parser.expect("(")?;
            let mut inputs = vec![];
            if !parser.eat(")") {
                loop {
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    inputs.push(InputDecl { name, ty });
                    if parser.eat(")") {
                        break;
                    }
                    parser.expect(",")?;
                }
            }
            parser.expect("{")?;
            let mut statements = vec![];
            while !parser.eat("}") {
                if parser.eat("input") {
                    if !statements.is_empty() {
                        return Err(
                            parser.error("input", "Declare action inputs before statements.")
                        );
                    }
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    parser.expect(";")?;
                    inputs.push(InputDecl { name, ty });
                    continue;
                }
                statements.push(parser.statement()?);
                if statements.len() > MAX_STATEMENTS {
                    return Err(parser.error("bound", "Action exceeds 1,024 statements."));
                }
            }
            parser.program.actions.push(Action {
                name,
                inputs,
                statements,
            });
        } else {
            return Err(parser.error("unsupported", "Expected state, asset or action declaration; loops and native calls are unsupported."));
        }
        if parser.program.states.len() + parser.program.assets.len() + parser.program.actions.len()
            > MAX_DECLARATIONS
        {
            return Err(parser.error("bound", "Program exceeds 256 declarations."));
        }
    }
    if parser.peek().kind != TokenKind::Eof {
        return Err(parser.error("syntax", "Unexpected content after agreement."));
    }
    if !staged {
        validate(&parser.program)?;
    }
    Ok(parser.program)
}
struct Tok {
    text: String,
    kind: TokenKind,
    start: u32,
}
struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    program: Program,
}
impl Parser {
    fn peek(&self) -> &Tok {
        &self.tokens[self.pos]
    }
    fn error(&self, code: &str, message: impl Into<String>) -> Error {
        let mut e = Error::new(code, message);
        e.offset = Some(self.peek().start);
        e
    }
    fn eat(&mut self, text: &str) -> bool {
        if self.peek().text == text && self.peek().kind != TokenKind::String {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, text: &str) -> Result<(), Error> {
        if self.eat(text) {
            Ok(())
        } else {
            Err(self.error(
                "syntax",
                format!("Expected {text:?}; found {:?}.", self.peek().text),
            ))
        }
    }
    fn word(&mut self) -> Result<String, Error> {
        if self.peek().kind.is_word() {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a name."))
        }
    }
    fn string(&mut self) -> Result<String, Error> {
        if self.peek().kind == TokenKind::String {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a quoted string."))
        }
    }
    fn ty(&mut self) -> Result<Type, Error> {
        match self.word()?.as_str() {
            "UInt" => Ok(Type::UInt),
            "Bool" => Ok(Type::Bool),
            _ => Err(self.error("type", "Supported types are UInt and Bool.")),
        }
    }
    fn literal(&mut self) -> Result<Value, Error> {
        if self.eat("true") {
            return Ok(Value::Bool(true));
        }
        if self.eat("false") {
            return Ok(Value::Bool(false));
        }
        if self.peek().kind != TokenKind::Number {
            return Err(self.error("syntax", "Expected an unsigned integer or boolean literal."));
        }
        let raw = self.peek().text.replace('_', "");
        let n = raw.parse::<u128>().map_err(|_| {
            self.error(
                "number",
                "UInt literal must fit u128; amounts use integral atomic units.",
            )
        })?;
        self.pos += 1;
        Ok(Value::UInt(UInt(n)))
    }
    fn push(&mut self, expr: Expr) -> Result<ExprId, Error> {
        if self.program.expressions.len() >= MAX_EXPRESSIONS {
            return Err(self.error("bound", "Program exceeds 8,192 expression nodes."));
        }
        let id = self.program.expressions.len() as u16;
        self.program.expressions.push(expr);
        Ok(id)
    }
    fn expr(&mut self, minimum: u8, depth: usize) -> Result<ExprId, Error> {
        if depth > 64 {
            return Err(self.error("bound", "Expression nesting exceeds 64."));
        }
        let mut lhs = if self.eat("(") {
            let e = self.expr(0, depth + 1)?;
            self.expect(")")?;
            e
        } else if self.eat("not") {
            let value = self.expr(6, depth + 1)?;
            self.push(Expr::Not { value })?
        } else if self.peek().kind == TokenKind::Number
            || self.peek().text == "true"
            || self.peek().text == "false"
        {
            let value = self.literal()?;
            self.push(Expr::Literal { value })?
        } else {
            let name = self.word()?;
            if matches!(name.as_str(), "liability_present" | "liability_amount") && self.eat("(") {
                let duty = self.string()?;
                self.expect(")")?;
                self.push(if name == "liability_present" {
                    Expr::LiabilityPresent { name: duty }
                } else {
                    Expr::LiabilityAmount { name: duty }
                })?
            } else {
                self.push(Expr::Name { name })?
            }
        };
        loop {
            // A decoded string spelling an operator is still a string token.
            if self.peek().kind == TokenKind::String {
                break;
            }
            let next = self
                .tokens
                .get(self.pos + 1)
                .filter(|t| t.kind != TokenKind::String)
                .map(|t| t.text.as_str());
            let (op, precedence, length) = match (self.peek().text.as_str(), next) {
                ("or", _) => (BinaryOp::Or, 1, 1),
                ("and", _) => (BinaryOp::And, 2, 1),
                ("=", Some("=")) => (BinaryOp::Eq, 3, 2),
                ("not", Some("=")) => (BinaryOp::Ne, 3, 2),
                ("<", Some("=")) => (BinaryOp::Le, 3, 2),
                (">", Some("=")) => (BinaryOp::Ge, 3, 2),
                ("<", _) => (BinaryOp::Lt, 3, 1),
                (">", _) => (BinaryOp::Gt, 3, 1),
                ("+", _) => (BinaryOp::Add, 4, 1),
                ("-", _) => (BinaryOp::Sub, 4, 1),
                ("*", _) => (BinaryOp::Mul, 5, 1),
                _ => break,
            };
            if precedence < minimum {
                break;
            }
            self.pos += length;
            let rhs = self.expr(precedence + 1, depth + 1)?;
            lhs = self.push(Expr::Binary { op, lhs, rhs })?;
        }
        Ok(lhs)
    }
    fn statement(&mut self) -> Result<Statement, Error> {
        let keyword = self.word()?;
        let s=match keyword.as_str() {
            "await" if self.program.language == crate::stage::LANGUAGE_2 => {
                let label = self.word()?;
                self.expect("(")?;
                let mut binders = Vec::new();
                if !self.eat(")") {
                    loop {
                        let name = self.word()?;
                        self.expect(":")?;
                        let ty = self.ty()?;
                        binders.push(InputDecl { name, ty });
                        if self.eat(")") { break; }
                        self.expect(",")?;
                    }
                }
                self.expect("when")?;
                Statement::Await { label, binders, guard: self.expr(0, 0)? }
            },
            "let"=>{let name=self.word()?;self.expect("=")?;Statement::Let{name,value:self.expr(0,0)?}},
            "next"=>{let state=self.word()?;self.expect("=")?;Statement::Next{state,value:self.expr(0,0)?}},
            "requires"=>Statement::Requires{predicate:self.expr(0,0)?},
            "ensures"=>Statement::Ensures{predicate:self.expr(0,0)?},
            "emit"=>{let name=self.string()?;self.expect("(")?;let value=self.expr(0,0)?;self.expect(")")?;Statement::Emit{name,value}},
            "transfer"|"fee"=>{self.expect("(")?;let asset=self.word()?;self.expect(",")?;let from=self.string()?;self.expect(",")?;let to=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;if keyword=="fee"{Statement::Fee{asset,from,to,amount}}else{Statement::Transfer{asset,from,to,amount}}},
            "liability"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let asset=self.word()?;self.expect(",")?;let debtor=self.string()?;self.expect(",")?;let creditor=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Liability{name,asset,debtor,creditor,amount}},
            "discharge"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Discharge{name,amount}},
            _=>return Err(self.error("unsupported",format!("Unsupported statement {keyword:?}; use let, requires, ensures, next, emit, transfer, fee, liability or discharge."))),
        };
        self.expect(";")?;
        Ok(s)
    }
}

/// Read the explicit source profile for CLI routing; this does not admit a program.
pub fn language(source: &str) -> Result<&'static str, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let mut tokens = Vec::new();
    for i in 0..lexed.len() {
        let idx = TokenIdx::new(i);
        let span = lexed.span(idx, source);
        let raw = &source[span.start as usize..span.end as usize];
        let kind = lexed.kind(idx);
        tokens.push(Tok {
            text: if kind == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            },
            kind,
            start: span.start,
        });
    }
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: LANGUAGE.into(),
            semantics: SEMANTICS.into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    let expected = format!(
        "language \"{LANGUAGE}\" or \"{}\"",
        crate::stage::LANGUAGE_2
    );
    parser.expect("language").map_err(|mut error| {
        error.message = format!("Expected {expected}. {}", error.message);
        error
    })?;
    let profile = parser.string().map_err(|mut error| {
        error.message = format!("Expected {expected}. {}", error.message);
        error
    })?;
    match profile.as_str() {
        LANGUAGE => Ok(LANGUAGE),
        crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
        _ => Err(parser.error(
            "profile",
            format!("Expected {expected}; found {profile:?}."),
        )),
    }
}

```

## COMPLETE FILE: candidate/crates/mori-core/tests/language.rs
```text
use mori_core::*;

const COUNTER: &str = r#"language "moriarty/1";
agreement Counter {
 state count: UInt = 0;
 action increment(amount: UInt) {
   let total = count + amount;
   requires total <= 100;
   next count = total;
   ensures count == total;
   emit "count"(count);
 }
 action double() { next count = count * 2; }
}"#;
fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
#[test]
fn authored_counter_selects_actions_and_preserves_dependencies() {
    let p = parse(COUNTER).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(7));
    let run = simulate(&p, "increment", &s).unwrap();
    assert_eq!(run.post.state["count"], uint(7));
    assert_eq!(run.steps.len(), 5);
    replay(&p, "increment", &s, &run).unwrap();
    s.state = run.post.state;
    s.inputs.clear();
    assert_eq!(
        simulate(&p, "double", &s).unwrap().post.state["count"],
        uint(14)
    );
}
#[test]
fn failed_predicate_leaves_original_scenario_unchanged() {
    let p = parse(COUNTER).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(101));
    let original = s.clone();
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::requires"
    );
    assert_eq!(s, original);
}

const PAYMENT: &str = r#"language "moriarty/1";
agreement ConditionalPayment {
 asset USD = { network: "preview", issuer: "issuer", reference: "usd", scale: 2 };
 state paid: UInt = 0;
 action settle(amount: UInt, approved: Bool) {
   requires approved and amount > 0;
   let charge = amount + 2;
   transfer(USD, "alice", "bob", amount);
   fee(USD, "alice", "treasury", 2);
   liability("delivery", USD, "bob", "alice", amount);
   next paid = paid + charge;
   ensures paid >= charge;
   emit "settled"(paid);
 }
 action finish(amount: UInt) { discharge("delivery", amount); }
}"#;
fn payment() -> (Program, Scenario) {
    let p = parse(PAYMENT).unwrap();
    let asset = p.assets[0].identity.clone();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(20));
    s.inputs.insert("approved".into(), Value::Bool(true));
    s.balances = vec![
        Balance {
            account: "alice".into(),
            asset: asset.clone(),
            amount: UInt(100),
        },
        Balance {
            account: "bob".into(),
            asset: asset.clone(),
            amount: UInt(0),
        },
        Balance {
            account: "treasury".into(),
            asset,
            amount: UInt(0),
        },
    ];
    (p, s)
}
#[test]
fn authored_payment_has_complete_ordered_effects_and_persistent_liability() {
    let (p, s) = payment();
    let run = simulate(&p, "settle", &s).unwrap();
    assert_eq!(run.post.state["paid"], uint(22));
    assert_eq!(run.effects.len(), 5);
    assert_eq!(
        run.post
            .balances
            .iter()
            .find(|b| b.account == "alice")
            .unwrap()
            .amount,
        UInt(78)
    );
    assert_eq!(
        run.post.balances.iter().map(|b| b.amount.0).sum::<u128>(),
        100
    );
    assert_eq!(run.post.liabilities[0].amount, UInt(20));
    let mut next = Scenario {
        state: run.post.state,
        balances: run.post.balances,
        liabilities: run.post.liabilities,
        ..Scenario::default()
    };
    next.inputs.insert("amount".into(), uint(7));
    assert_eq!(
        simulate(&p, "finish", &next).unwrap().post.liabilities[0].amount,
        UInt(13)
    );
}
#[test]
fn late_fee_failure_rolls_back_every_effect() {
    let (p, mut s) = payment();
    s.balances[0].amount = UInt(21);
    let before = s.clone();
    let e = simulate(&p, "settle", &s).unwrap_err();
    assert_eq!(e.code, "mori::core::balance");
    assert_eq!(e.step, Some(3));
    assert_eq!(s, before);
}
#[test]
fn replay_identifies_first_step_divergence_and_missing_effects() {
    let (p, s) = payment();
    let run = simulate(&p, "settle", &s).unwrap();
    let mut wrong = run.clone();
    wrong.steps[2].effects.clear();
    assert_eq!(replay(&p, "settle", &s, &wrong).unwrap_err().step, Some(2));
    let mut wrong = run;
    wrong.effects.pop();
    assert_eq!(
        replay(&p, "settle", &s, &wrong).unwrap_err().code,
        "mori::core::replay"
    );
}
#[test]
fn exact_input_and_balance_schemas_are_required() {
    let (p, mut s) = payment();
    s.inputs.remove("amount");
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::input"
    );
    let (_, mut s) = payment();
    s.inputs.insert("unexpected".into(), uint(1));
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::input"
    );
    let (_, mut s) = payment();
    s.inputs.insert("amount".into(), Value::Bool(true));
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::input"
    );
    let (_, mut s) = payment();
    s.balances.pop();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::balance"
    );
    let (_, mut s) = payment();
    s.balances[1].asset.issuer = "other".into();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::asset"
    );
}
#[test]
fn checked_arithmetic_and_invalid_ir_refuse() {
    let p = parse(COUNTER).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(u128::MAX));
    s.state.insert("count".into(), uint(1));
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::overflow"
    );
    let mut p = p;
    p.expressions[0] = Expr::Not { value: 0 };
    assert_eq!(validate(&p).unwrap_err().code, "mori::core::expression");
    let bad = r#"language "moriarty/1"; agreement Bad { state x: UInt = 0; action run(){ next x = x - 1; } }"#;
    assert_eq!(
        simulate(&parse(bad).unwrap(), "run", &Scenario::default())
            .unwrap_err()
            .code,
        "mori::core::underflow"
    );
}
#[test]
fn unsupported_and_ill_typed_programs_refuse() {
    assert_eq!(
        parse(&COUNTER.replace(
            "let total = count + amount;",
            "let total = missing + amount;"
        ))
        .unwrap_err()
        .code,
        "mori::core::name"
    );
    assert_eq!(
        parse(&COUNTER.replace("requires total <= 100;", "requires total;"))
            .unwrap_err()
            .code,
        "mori::core::type"
    );
    assert_eq!(
        parse(&COUNTER.replace("next count = total;", "while true { }"))
            .unwrap_err()
            .code,
        "mori::core::unsupported"
    );
    assert_eq!(
        parse(&PAYMENT.replace("scale: 2", "scale: 39"))
            .unwrap_err()
            .code,
        "mori::core::asset"
    );
}
#[test]
fn quantities_are_canonical_decimal_strings_even_above_javascript_range() {
    let v = uint(u128::MAX);
    let encoded = serde_json::to_string(&v).unwrap();
    assert!(encoded.contains(&format!("\"{}\"", u128::MAX)));
    assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(), v);
    for text in ["1", "\"01\"", "\"-1\"", "\"1.0\"", "\"+1\""] {
        assert!(serde_json::from_str::<UInt>(text).is_err());
    }
    assert!(
        serde_json::from_str::<Scenario>(
            r#"{"state":{},"inputs":{},"balances":[],"liabilities":[],"ignored":true}"#
        )
        .is_err()
    );
}

#[test]
fn every_asset_has_independent_conservation_and_recipient_overflow_refuses() {
    let (p, mut s) = payment();
    s.balances[1].amount = UInt(u128::MAX);
    let original = s.clone();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::overflow"
    );
    assert_eq!(s, original);
    let extended=PAYMENT.replace("state paid:","asset EUR = { network: \"preview\", issuer: \"issuer\", reference: \"eur\", scale: 2 }; state paid:");
    let p = parse(&extended).unwrap();
    let eur = p.assets[1].identity.clone();
    let (_, mut s) = payment();
    s.balances.push(Balance {
        account: "alice".into(),
        asset: eur.clone(),
        amount: UInt(50),
    });
    s.balances.push(Balance {
        account: "bob".into(),
        asset: eur,
        amount: UInt(9),
    });
    let run = simulate(&p, "settle", &s).unwrap();
    assert_eq!(
        run.post
            .balances
            .iter()
            .filter(|b| b.asset.reference == "eur")
            .map(|b| b.amount.0)
            .sum::<u128>(),
        59
    );
    let extended = extended.replace(
        "reference: \"eur\", scale: 2",
        "reference: \"usd\", scale: 3",
    );
    assert_eq!(parse(&extended).unwrap_err().code, "mori::core::asset");
}
#[test]
fn postcondition_failure_is_atomic_and_initial_state_is_exact() {
    let p = parse(&COUNTER.replace("ensures count == total;", "ensures count > total;")).unwrap();
    let mut s = Scenario::default();
    s.inputs.insert("amount".into(), uint(7));
    let original = s.clone();
    let e = simulate(&p, "increment", &s).unwrap_err();
    assert_eq!(e.code, "mori::core::ensures");
    assert_eq!(e.step, Some(3));
    assert_eq!(s, original);
    s.state.insert("surplus".into(), uint(1));
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::state"
    );
    s.state.clear();
    s.state.insert("count".into(), Value::Bool(false));
    assert_eq!(
        simulate(&p, "increment", &s).unwrap_err().code,
        "mori::core::state"
    );
    assert_eq!(
        simulate(&p, "missing", &s).unwrap_err().code,
        "mori::core::action"
    );
}
#[test]
fn liability_binding_and_partial_discharge_cannot_be_rewritten() {
    let (p, mut s) = payment();
    let asset = p.assets[0].identity.clone();
    s.liabilities.push(Liability {
        name: "delivery".into(),
        asset,
        debtor: "bob".into(),
        creditor: "mallory".into(),
        amount: UInt(5),
    });
    let original = s.clone();
    assert_eq!(
        simulate(&p, "settle", &s).unwrap_err().code,
        "mori::core::liability"
    );
    assert_eq!(s, original);
    s.inputs.remove("approved");
    s.inputs.insert("amount".into(), uint(6));
    assert_eq!(
        simulate(&p, "finish", &s).unwrap_err().code,
        "mori::core::liability"
    );
    s.inputs.insert("amount".into(), uint(5));
    s.liabilities[0].creditor = "alice".into();
    let run = simulate(&p, "finish", &s).unwrap();
    assert_eq!(run.post.liabilities[0].amount, UInt(0));
}
#[test]
fn deterministic_order_is_independent_of_balance_serialization() {
    let (p, s) = payment();
    let one = simulate(&p, "settle", &s).unwrap();
    let mut reversed = s;
    reversed.balances.reverse();
    assert_eq!(simulate(&p, "settle", &reversed).unwrap(), one);
    reversed.balances.push(reversed.balances[0].clone());
    assert_eq!(
        simulate(&p, "settle", &reversed).unwrap_err().code,
        "mori::core::balance"
    );
}
#[test]
fn shared_expression_dag_is_linear_and_bounded() {
    let mut p=parse(r#"language "moriarty/1"; agreement Shared { state value: UInt = 0; action run(){ next value = 0; } }"#).unwrap();
    p.expressions = vec![Expr::Literal { value: uint(0) }];
    for i in 1..64 {
        p.expressions.push(Expr::Binary {
            op: BinaryOp::Add,
            lhs: i - 1,
            rhs: i - 1,
        });
    }
    p.actions[0].statements = vec![Statement::Next {
        state: "value".into(),
        value: 63,
    }];
    assert_eq!(
        simulate(&p, "run", &Scenario::default())
            .unwrap()
            .post
            .state["value"],
        uint(0)
    );
    p.expressions.push(Expr::Binary {
        op: BinaryOp::Add,
        lhs: 63,
        rhs: 63,
    });
    p.expressions.push(Expr::Not { value: 64 });
    assert_eq!(validate(&p).unwrap_err().code, "mori::core::bound");
}
#[test]
fn malformed_identifiers_refuse() {
    let mut p = parse(COUNTER).unwrap();
    p.name = "path/../name".into();
    assert_eq!(validate(&p).unwrap_err().code, "mori::core::name");
}
#[test]
fn excessive_trace_refuses() {
    let (p, mut s) = payment();
    let asset = p.assets[0].identity.clone();
    for i in 0..4090 {
        s.balances.push(Balance {
            account: format!("account{i}"),
            asset: asset.clone(),
            amount: UInt(0),
        });
    }
    let mut p = p;
    let stmt = p.actions[0].statements.last().unwrap().clone();
    p.actions[0]
        .statements
        .extend(std::iter::repeat_n(stmt, 20));
    assert_eq!(
        simulate(&p, "settle", &s)
            .expect_err("trace budget must refuse")
            .code,
        "mori::core::bound"
    );
}
proptest::proptest! {
 #[test]
 fn arbitrary_source_never_panics(source in ".{0,2048}") { let _=parse(&source); }
}

#[test]
fn quoted_operators_cannot_become_syntax() {
    let source = r#"language "moriarty/1"; agreement Quoted { state x: UInt = 0; action run() { next x = 1 "+" 2; } }"#;
    assert!(
        parse(source).is_err(),
        "quoted plus must not become an operator"
    );
    let source =
        r#"language "moriarty/1"; agreement Quoted { action run() { requires 1 = "=" 1; } }"#;
    assert!(parse(source).is_err(), "quoted tail cannot form equality");
}

#[test]
fn language_header_diagnostics_name_both_supported_core_profiles() {
    for (source, expected) in [
        (
            r#"language "moriarty/1"; agreement Example {}"#,
            "moriarty/1",
        ),
        (
            r#"language "moriarty/2"; agreement Example {}"#,
            "moriarty/2",
        ),
    ] {
        assert_eq!(language(source).unwrap(), expected);
    }

    let expected = r#"Expected language "moriarty/1" or "moriarty/2""#;
    for (source, code, offset, suffix) in [
        (
            r#"language "other";"#,
            "mori::core::profile",
            16,
            r#"; found "other"."#,
        ),
        (
            "",
            "mori::core::syntax",
            0,
            r#". Expected "language"; found ""."#,
        ),
        (
            "agreement Empty {}",
            "mori::core::syntax",
            0,
            r#". Expected "language"; found "agreement"."#,
        ),
        (
            "language ;",
            "mori::core::syntax",
            9,
            ". Expected a quoted string.",
        ),
    ] {
        let error = language(source).unwrap_err();
        assert_eq!(error.code, code, "{source:?}");
        assert_eq!(error.offset, Some(offset), "{source:?}");
        assert_eq!(error.message, format!("{expected}{suffix}"), "{source:?}");
    }
}

```

## COMPLETE FILE: original-author/EXPECTED-DIAGNOSTICS.json
```text
{
  "schema": "language-header-diagnostic-expectations/1",
  "base": "79de664a3d16b5e0456043a530c8f33d723561a2",
  "predicate": "Unsupported or missing Core language header diagnostics name both supported Core profiles; accepted dispatch and original codes/offsets remain.",
  "sourceExposure": "Author inspected actual language(), Parser::expect/string/error and ordinary negative tests. These are independently literal diagnostic expectations, not financial/security oracles.",
  "accepted": [
    {
      "source": "language \"moriarty/1\"; agreement Example {}",
      "profile": "moriarty/1"
    },
    {
      "source": "language \"moriarty/2\"; agreement Example {}",
      "profile": "moriarty/2"
    }
  ],
  "rejected": [
    {
      "source": "language \"other\";",
      "code": "mori::core::profile",
      "offset": 16,
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\"; found \"other\"."
    },
    {
      "source": "",
      "code": "mori::core::syntax",
      "offset": 0,
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected \"language\"; found \"\"."
    },
    {
      "source": "agreement Empty {}",
      "code": "mori::core::syntax",
      "offset": 0,
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected \"language\"; found \"agreement\"."
    },
    {
      "source": "language ;",
      "code": "mori::core::syntax",
      "offset": 9,
      "message": "Expected language \"moriarty/1\" or \"moriarty/2\". Expected a quoted string."
    }
  ],
  "lexErrors": "Unchanged lexer error path; no header-context wrapping added before lex succeeds."
}

```

## COMPLETE FILE: original-author/CANDIDATE.patch
```text
--- a/crates/mori-core/src/parser.rs
+++ b/crates/mori-core/src/parser.rs
@@ -394,14 +394,24 @@
             actions: vec![],
         },
     };
-    parser.expect("language")?;
-    let profile = parser.string()?;
+    let expected = format!(
+        "language \"{LANGUAGE}\" or \"{}\"",
+        crate::stage::LANGUAGE_2
+    );
+    parser.expect("language").map_err(|mut error| {
+        error.message = format!("Expected {expected}. {}", error.message);
+        error
+    })?;
+    let profile = parser.string().map_err(|mut error| {
+        error.message = format!("Expected {expected}. {}", error.message);
+        error
+    })?;
     match profile.as_str() {
         LANGUAGE => Ok(LANGUAGE),
         crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
         _ => Err(parser.error(
             "profile",
-            format!("Expected language \"{LANGUAGE}\"; found {profile:?}."),
+            format!("Expected {expected}; found {profile:?}."),
         )),
     }
 }
--- a/crates/mori-core/tests/language.rs
+++ b/crates/mori-core/tests/language.rs
@@ -424,3 +424,46 @@
         r#"language "moriarty/1"; agreement Quoted { action run() { requires 1 = "=" 1; } }"#;
     assert!(parse(source).is_err(), "quoted tail cannot form equality");
 }
+
+#[test]
+fn language_header_diagnostics_name_both_supported_core_profiles() {
+    for (source, expected) in [
+        (r#"language "moriarty/1"; agreement Example {}"#, "moriarty/1"),
+        (r#"language "moriarty/2"; agreement Example {}"#, "moriarty/2"),
+    ] {
+        assert_eq!(language(source).unwrap(), expected);
+    }
+
+    let expected = r#"Expected language "moriarty/1" or "moriarty/2""#;
+    for (source, code, offset, suffix) in [
+        (
+            r#"language "other";"#,
+            "mori::core::profile",
+            16,
+            r#"; found "other"."#,
+        ),
+        (
+            "",
+            "mori::core::syntax",
+            0,
+            r#". Expected "language"; found ""."#,
+        ),
+        (
+            "agreement Empty {}",
+            "mori::core::syntax",
+            0,
+            r#". Expected "language"; found "agreement"."#,
+        ),
+        (
+            "language ;",
+            "mori::core::syntax",
+            9,
+            ". Expected a quoted string.",
+        ),
+    ] {
+        let error = language(source).unwrap_err();
+        assert_eq!(error.code, code, "{source:?}");
+        assert_eq!(error.offset, Some(offset), "{source:?}");
+        assert_eq!(error.message, format!("{expected}{suffix}"), "{source:?}");
+    }
+}

```

## COMPLETE FILE: source-integration/SOURCE-INTEGRATION.json
```text
{
  "head": "3bb93ec676340ebca4b772c9ff9277289328c4e0",
  "tree": "6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4",
  "main": "8269b5b7ceeec5169c706aa31ea33f390d49c969",
  "oldHead": "1cb98865f37863152cdf51a38773ca4548adea12",
  "oldBase": "79de664a3d16b5e0456043a530c8f33d723561a2",
  "wholeCandidateDiffByteIdentical": true,
  "candidateDiffSha256": "8931f6b177d0b1515b9e587f2bd45e483636689294dad0b47647fafc438089e3",
  "driver": "/home/charl/moriarty-c2-opus55-restored-20261017/run_language_header_diagnostic_check_20261018_main8269.py",
  "driverSha256": "0ff6d7d3cd5d4ce525ba5543e17090676863a5f4760562a425ea19db437b27d0"
}

```

## COMPLETE FILE: actual-check/CHECK-PLAN.json
```text
{
  "recordedAt": "2026-10-18T14:59:35.392468+00:00",
  "head": "3bb93ec676340ebca4b772c9ff9277289328c4e0",
  "tree": "6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4",
  "base": "8269b5b7ceeec5169c706aa31ea33f390d49c969",
  "sourceManifestSha256": "ff832d223b1cf170d4c047b251239cfd3b43c4fe75275bfce48799e90bd229da",
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-core-header-e3z3attd/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-core-header-e3z3attd/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-core-header-e3z3attd/tmp"
  },
  "ownedRoot": "/tmp/doctorwatson-core-header-e3z3attd",
  "absentAncestorCargoConfigs": [
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/.cargo/config",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/.cargo/config.toml",
    "/home/charl/Moriarty/.worktrees/.cargo/config",
    "/home/charl/Moriarty/.worktrees/.cargo/config.toml",
    "/home/charl/Moriarty/.cargo/config",
    "/home/charl/Moriarty/.cargo/config.toml",
    "/home/charl/.cargo/config",
    "/home/charl/.cargo/config.toml",
    "/home/.cargo/config",
    "/home/.cargo/config.toml",
    "/.cargo/config",
    "/.cargo/config.toml",
    "/tmp/doctorwatson-core-header-e3z3attd/cwd/.cargo/config",
    "/tmp/doctorwatson-core-header-e3z3attd/cwd/.cargo/config.toml",
    "/tmp/doctorwatson-core-header-e3z3attd/.cargo/config",
    "/tmp/doctorwatson-core-header-e3z3attd/.cargo/config.toml",
    "/tmp/.cargo/config",
    "/tmp/.cargo/config.toml"
  ],
  "toolPins": [
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
      "sha256": "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
      "sha256": "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
      "sha256": "a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"
    },
    {
      "path": "/usr/bin/timeout",
      "sha256": "f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"
    },
    {
      "path": "/usr/bin/bash",
      "sha256": "3efccc187bafa75ff1e37d246270ab3e7aa559f242c7a52bf3ec2a1b5450bdbd"
    }
  ],
  "scope": "One ordinary public Core language-header regression only; no SDK/native/proof/key/wallet/ledger action."
}

```

## COMPLETE FILE: actual-check/FINAL-RESULT.json
```text
{
  "finishedAt": "2026-10-18T15:00:04.599577+00:00",
  "head": "3bb93ec676340ebca4b772c9ff9277289328c4e0",
  "tree": "6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4",
  "results": [
    {
      "name": "language-header-test",
      "finishedAt": "2026-10-18T15:00:04.501212+00:00",
      "exitCode": 0,
      "wallSeconds": 29.05823117199907,
      "stdoutSha256": "1301c42af24d2ab43bfb0ed4a9ec976e27a5ff466da8a5fd47ea164a3ccd1b28",
      "stderrSha256": "58ecee045e84bab2d3fbd947ca149fc5abf06dce9d053b0235d9aae27c49d5e1",
      "ownedSessionRemaining": []
    }
  ],
  "selectedCheckCompleted": true,
  "ownedRoot": "/tmp/doctorwatson-core-header-e3z3attd",
  "existsAfter": false,
  "scope": "Public Core header diagnostics only; compiler/proofs/native/ledger and parent EARS not established. Originalauthorpatch retained; root rustfmt expanded two test tuples only."
}

```

## COMPLETE FILE: historical-formatting/FORMATTING.json
```text
{
  "command": [
    "rustfmt",
    "--edition",
    "2024",
    "--check",
    "crates/mori-core/src/parser.rs",
    "crates/mori-core/tests/language.rs"
  ],
  "originalAuthorPatchSha256": "defc665a910f972a7fcdeb7d8958807f57cc4653b9fdd905183cad99aac92919",
  "firstAuthorCandidateCheckExit": 1,
  "firstDifference": "Only two accepted-input test tuple layouts expanded by rustfmt; original author patch remains immutable.",
  "finalFormattedHead": "1cb98865f37863152cdf51a38773ca4548adea12",
  "finalCheckExit": 0
}

```

## COMPLETE FILE: actual-check/language-header-test/START.json
```text
{
  "startedAt": "2026-10-18T14:59:35.442921+00:00",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori-core",
    "--no-default-features",
    "--test",
    "language",
    "language_header_diagnostics_name_both_supported_core_profiles",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-core-header-e3z3attd/cwd",
  "head": "3bb93ec676340ebca4b772c9ff9277289328c4e0",
  "tree": "6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4"
}

```

## COMPLETE FILE: actual-check/language-header-test/CHILD.json
```text
{
  "pid": 853769,
  "session": 853769
}

```

## COMPLETE FILE: actual-check/language-header-test/RESULT.json
```text
{
  "finishedAt": "2026-10-18T15:00:04.501212+00:00",
  "exitCode": 0,
  "wallSeconds": 29.05823117199907,
  "stdoutSha256": "1301c42af24d2ab43bfb0ed4a9ec976e27a5ff466da8a5fd47ea164a3ccd1b28",
  "stderrSha256": "58ecee045e84bab2d3fbd947ca149fc5abf06dce9d053b0235d9aae27c49d5e1",
  "ownedSessionRemaining": []
}

```

## COMPLETE FILE: actual-check/language-header-test/STDOUT.log
```text

running 1 test
test language_header_diagnostics_name_both_supported_core_profiles ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s


```

## COMPLETE FILE: actual-check/language-header-test/STDERR.log
```text
   Compiling cfg-if v1.0.5
   Compiling libc v0.2.190
   Compiling typenum v1.20.1
   Compiling zeroize v1.9.1
   Compiling version_check v0.9.5
   Compiling generic-array v0.14.9
   Compiling subtle v2.6.1
   Compiling const-oid v0.9.6
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling getrandom v0.2.17
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling der v0.7.10
   Compiling getrandom v0.3.4
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling syn v3.0.6
   Compiling hybrid-array v0.4.15
   Compiling once_cell v1.21.4
   Compiling serde_core v1.0.229
   Compiling rustix v1.1.5
   Compiling zerocopy v0.8.61
   Compiling base16ct v0.2.0
   Compiling getrandom v0.4.3
   Compiling unicode-width v0.1.14
   Compiling miette v7.6.0
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling rand_core v0.9.5
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling syn v2.0.119
   Compiling zmij v1.0.23
   Compiling linux-raw-sys v0.12.1
   Compiling bitflags v2.13.2
   Compiling autocfg v1.5.1
   Compiling num-traits v0.2.19
   Compiling soa-rs-derive v1.0.1
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-span)
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling signature v2.2.0
   Compiling serde v1.0.229
   Compiling const-oid v0.10.2
   Compiling serde_json v1.0.151
   Compiling thiserror v2.0.21
   Compiling cpufeatures v0.2.17
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-diagnostics)
   Compiling ppv-lite86 v0.2.21
   Compiling soa-rs v1.0.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling wait-timeout v0.2.1
   Compiling memchr v2.8.3
   Compiling bit-vec v0.8.0
   Compiling quick-error v1.2.3
   Compiling fnv v1.0.7
   Compiling cpufeatures v0.3.1
   Compiling itoa v1.0.18
   Compiling sha2 v0.11.0
   Compiling rusty-fork v0.3.1
   Compiling bit-set v0.8.0
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-lexer)
   Compiling rand_chacha v0.9.0
   Compiling k256 v0.13.4
   Compiling rand_xorshift v0.4.0
   Compiling rand v0.9.5
   Compiling regex-syntax v0.8.11
   Compiling unarray v0.1.4
   Compiling proptest v1.11.0
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018/crates/mori-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 29.03s
     Running tests/language.rs (/tmp/doctorwatson-core-header-e3z3attd/target/debug/deps/language-257a0549876f96e6)

```

## COMPLETE FILE: run_actual.py
```text
from pathlib import Path
import datetime,hashlib,json,os,shutil,subprocess,tempfile,time
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
K=Path('/home/charl/Moriarty/.worktrees/utxo-positive-wave32-20261018')
P=R/'language-header-diagnostic-main79-20261018'
C=R/'language-header-diagnostic-main8269-check-cycle-20261018';C.mkdir()
HEAD='3bb93ec676340ebca4b772c9ff9277289328c4e0'
TC=Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def write(name,x):
 p=C/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(x,indent=2)+'\n')
def git(*args):return subprocess.check_output(['git',*args],cwd=K).decode().strip()
assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')
tree=git('rev-parse','HEAD^{tree}')
assert tree=='6b6204185c316ceaf7ecd19cc6fb8164ad1d79b4'
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',HEAD,'crates','Cargo.toml','Cargo.lock','rust-toolchain.toml','docs/NATIVE-CORE.md'],cwd=K).decode().splitlines()
pins=[]
for path in paths:
 raw=subprocess.check_output(['git','show',HEAD+':'+path],cwd=K);assert(K/path).read_bytes()==raw,path
 pins.append({'path':path,'sha256':hashlib.sha256(raw).hexdigest()})
write('SOURCE-PINS.json',{'head':HEAD,'tree':tree,'files':pins})
manifest=json.loads((P/'MANIFEST.json').read_text())
root=Path(tempfile.mkdtemp(prefix='doctorwatson-core-header-'))
for n in ['cwd','tmp','cargo-home']:(root/n).mkdir()
for n in ['registry','git']:(root/'cargo-home'/n).symlink_to(Path('/home/charl/.cargo')/n,target_is_directory=True)
env={'PATH':str(TC)+':/usr/bin:/bin','RUSTC':str(TC/'rustc'),'RUSTDOC':str(TC/'rustdoc'),'CARGO_HOME':str(root/'cargo-home'),'CARGO_TARGET_DIR':str(root/'target'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','CARGO_CACHE_AUTO_CLEAN_FREQUENCY':'never','GIT_CONFIG_NOSYSTEM':'1','GIT_CONFIG_GLOBAL':'/dev/null','GIT_CONFIG_SYSTEM':'/dev/null','LANG':'C.UTF-8','TMPDIR':str(root/'tmp')}
configs=[]
for directory in [K,*K.parents,root/'cwd',*(root/'cwd').parents]:
 for n in ['config','config.toml']:
  p=directory/'.cargo'/n
  if str(p) not in configs:assert not p.exists(),str(p);configs.append(str(p))
write('CHECK-PLAN.json',{'recordedAt':now(),'head':HEAD,'tree':tree,'base':'8269b5b7ceeec5169c706aa31ea33f390d49c969','sourceManifestSha256':sha(P/'MANIFEST.json'),'environment':env,'ownedRoot':str(root),'absentAncestorCargoConfigs':configs,'toolPins':[{'path':str(p),'sha256':sha(p)} for p in [TC/'cargo',TC/'rustc',TC/'rustdoc',Path('/usr/bin/timeout'),Path('/usr/bin/bash')]],'scope':'One ordinary public Core language-header regression only; no SDK/native/proof/key/wallet/ledger action.'})
def run(name,argv,bound):
 d=C/name;d.mkdir();argv=['/usr/bin/timeout','--signal=TERM','--kill-after=5s',str(bound)+'s',*argv]
 write(name+'/START.json',{'startedAt':now(),'argv':argv,'cwd':str(root/'cwd'),'head':HEAD,'tree':tree})
 start=time.monotonic()
 with(d/'STDOUT.log').open('wb')as out,(d/'STDERR.log').open('wb')as err:
  child=subprocess.Popen(argv,cwd=root/'cwd',env=env,stdin=subprocess.DEVNULL,stdout=out,stderr=err,start_new_session=True);write(name+'/CHILD.json',{'pid':child.pid,'session':child.pid});code=child.wait()
 remaining=[int(p)for p,s in(line.split()for line in subprocess.check_output(['ps','-eo','pid=,sid=']).decode().splitlines())if int(s)==child.pid]
 result={'finishedAt':now(),'exitCode':code,'wallSeconds':time.monotonic()-start,'stdoutSha256':sha(d/'STDOUT.log'),'stderrSha256':sha(d/'STDERR.log'),'ownedSessionRemaining':remaining};write(name+'/RESULT.json',result)
 assert not remaining;return result,d
results=[]
try:
 result,d=run('language-header-test',[str(TC/'cargo'),'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori-core','--no-default-features','--test','language','language_header_diagnostics_name_both_supported_core_profiles','--','--exact','--test-threads=1'],90)
 results.append({'name':'language-header-test',**result});print(json.dumps(results[-1]),flush=True)
 assert result['exitCode']==0
 text=(d/'STDOUT.log').read_text();assert '1 passed; 0 failed' in text and 'language_header_diagnostics_name_both_supported_core_profiles ... ok' in text
 assert git('rev-parse','HEAD')==HEAD and not git('status','--porcelain=v1')and all(sha(K/p['path'])==p['sha256']for p in pins)
finally:
 shutil.rmtree(root)
 write('FINAL-RESULT.json',{'finishedAt':now(),'head':HEAD,'tree':tree,'results':results,'selectedCheckCompleted':len(results)==1,'ownedRoot':str(root),'existsAfter':root.exists(),'scope':'Public Core header diagnostics only; compiler/proofs/native/ledger and parent EARS not established. Originalauthorpatch retained; root rustfmt expanded two test tuples only.'})
 print(json.dumps({'selectedCheckCompleted':len(results)==1,'ownedRootRemoved':not root.exists()}),flush=True)

```

## COMPLETE FILE: source-context/crates/mori-core/src/lib.rs
```text
//! Bounded composable Moriarty language and deterministic local semantics.
pub mod intent;
mod ir;
pub mod json;
mod logical_cost;
mod parser;
pub mod policy;
pub mod sherlock;

// Unregistered draft source capability: Core-only ordinary host test proposal.
// These lexer-dependent modules stay outside Sherlock's standalone Std graph.
mod replay;
mod runtime;
#[cfg(test)]
#[path = "sherlock/draft_source.rs"]
mod sherlock_draft;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge.rs"]
mod sherlock_draft_image;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge_tests.rs"]
mod sherlock_draft_image_tests;
#[cfg(test)]
#[path = "sherlock/draft_source_tests.rs"]
mod sherlock_draft_tests;
#[cfg(test)]
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
mod source_sponsor;
pub mod stage;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;

```

## COMPLETE FILE: source-context/crates/mori-lexer/src/lib.rs
```text
//! Lexer for `moriarty-beta/1` source.
//!
//! Turns source text into [`Tokens`] in a single pass.
//!
//! Lexing stops at the first error. Whitespace is skipped; comments are kept
//! separately for the formatter and do not count toward [`MAX_TOKENS`].

mod diagnostics;
mod number;
mod scan;
mod string;
mod token;

#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;
use soa_rs::Soa;

use crate::number::NumberError;
use crate::string::StringErrorKind;

pub use crate::token::{Comment, Token, TokenIdx, TokenKind, Tokens};

/// Largest accepted source file, in bytes.
pub const MAX_SOURCE_BYTES: usize = 65_536;
/// Most tokens in one file, including the end-of-file token.
pub const MAX_TOKENS: usize = 8_192;
/// Longest identifier or keyword, in bytes.
pub const MAX_IDENT_BYTES: usize = 64;
/// Longest decoded string, in UTF-8 bytes.
pub const MAX_STRING_BYTES: usize = 1_024;

/// The text of a string token, with escapes decoded.
///
/// `raw` must be the full text of a [`TokenKind::String`] token, quotes
/// included, as produced by [`lex`].
pub fn decode_string(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    string::decode_with(raw, |c| text.push(c)).expect("lexed strings are valid");
    text
}

/// Splits `source` into tokens and comments, or reports the first error.
pub fn lex(source: &str) -> Result<Tokens, MoriDiagnostic> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::source_too_large(source.len()));
    }
    Lexer::new(source).run(false)
}

/// Identify a leading profile token without requiring the remaining source to
/// lex successfully. Inspect only a bounded prefix; full parsing retains all
/// source and token limits.
pub fn first_token_text(source: &str) -> Result<&str, MoriDiagnostic> {
    let end = source.floor_char_boundary(source.len().min(MAX_SOURCE_BYTES));
    let prefix = &source[..end];
    let tokens = Lexer::new(prefix).run(true)?;
    Ok(tokens.span(TokenIdx::new(0), prefix).source_text(prefix))
}

struct Lexer<'src> {
    source: &'src str,
    bytes: &'src [u8],
    pos: usize,
    tokens: Soa<Token>,
    comments: Soa<Comment>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
            // Roughly one token per eight bytes, as in Zig.
            tokens: Soa::with_capacity(source.len() / 8 + 1),
            comments: Soa::new(),
        }
    }

    fn run(mut self, first_only: bool) -> Result<Tokens, MoriDiagnostic> {
        while let Some(&b) = self.bytes.get(self.pos) {
            let start = self.pos;
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => self.pos += 1,
                b'/' if self.peek(1) == Some(b'/') => self.line_comment(start),
                b'/' if self.peek(1) == Some(b'*') => self.block_comment(start)?,
                b'"' => self.string(start)?,
                b'0'..=b'9' => self.number(start)?,
                b'A'..=b'Z' | b'a'..=b'z' => self.word(start)?,
                _ => match TokenKind::punctuation(b) {
                    Some(kind) => self.push(kind, start, start + 1)?,
                    None => return Err(self.unexpected_character(start)),
                },
            }
            if first_only && !self.tokens.is_empty() {
                break;
            }
        }
        // The end-of-file token is always pushed: the bound reserves room for it.
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            start: offset(self.pos),
        });
        Ok(Tokens {
            tokens: self.tokens,
            comments: self.comments,
        })
    }

    fn peek(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos + ahead).copied()
    }

    /// Records a token ending at `end` and moves past it.
    fn push(&mut self, kind: TokenKind, start: usize, end: usize) -> Result<(), MoriDiagnostic> {
        if self.tokens.len() >= MAX_TOKENS - 1 {
            return Err(diagnostics::too_many_tokens(span(start, end)));
        }
        self.tokens.push(Token {
            kind,
            start: offset(start),
        });
        self.pos = end;
        Ok(())
    }

    fn push_comment(&mut self, start: usize, end: usize) {
        self.comments.push(Comment {
            start: offset(start),
            end: offset(end),
        });
        self.pos = end;
    }

    /// `//` up to, but not including, the next line break.
    fn line_comment(&mut self, start: usize) {
        let end = self.bytes[start..]
            .iter()
            .position(|&b| b == b'\r' || b == b'\n')
            .map_or(self.bytes.len(), |len| start + len);
        self.push_comment(start, end);
    }

    /// `/*` through the first `*/`. Block comments do not nest.
    fn block_comment(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let body = start + 2;
        let Some(len) = self.source[body..].find("*/") else {
            return Err(diagnostics::unterminated_block_comment(span(start, body)));
        };
        self.push_comment(start, body + len + 2);
        Ok(())
    }

    fn string(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let Some(end) = scan::string_end(self.bytes, start) else {
            return Err(diagnostics::unterminated_string(span(start, start + 1)));
        };
        let raw = &self.source[start..end];
        let mut decoded_len = 0;
        if let Err(error) = string::decode_with(raw, |c| decoded_len += c.len_utf8()) {
            let at = span(start + error.start, start + error.end);
            return Err(match error.kind {
                StringErrorKind::ControlCharacter('\r' | '\n') => {
                    diagnostics::string_line_break(at)
                }
                StringErrorKind::ControlCharacter(c) => {
                    diagnostics::string_control_character(at, c)
                }
                StringErrorKind::InvalidEscape => {
                    diagnostics::invalid_escape(at, at.source_text(self.source))
                }
                StringErrorKind::InvalidUnicodeEscape => diagnostics::invalid_unicode_escape(at),
                StringErrorKind::LoneSurrogate => diagnostics::lone_surrogate(at),
            });
        }
        if decoded_len > MAX_STRING_BYTES {
            return Err(diagnostics::string_too_long(span(start, end), decoded_len));
        }
        self.push(TokenKind::String, start, end)
    }

    fn number(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::number_end(self.bytes, start);
        let raw = &self.source[start..end];
        if let Err(error) = number::validate(raw) {
            return Err(match error {
                NumberError::LeadingZero => diagnostics::number_leading_zero(
                    span(start, end),
                    &number::without_leading_zeros(raw),
                ),
                NumberError::MisplacedUnderscore { at } => {
                    diagnostics::number_misplaced_underscore(span(start + at, start + at + 1))
                }
                NumberError::MissingFraction => {
                    diagnostics::number_missing_fraction(span(end - 1, end))
                }
                NumberError::ExtraDot { at } => {
                    diagnostics::number_extra_dot(span(start + at, start + at + 1))
                }
            });
        }
        self.push(TokenKind::Number, start, end)
    }

    /// An identifier, or a keyword when the text matches one.
    fn word(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::word_end(self.bytes, start);
        if end - start > MAX_IDENT_BYTES {
            return Err(diagnostics::identifier_too_long(span(start, end)));
        }
        let kind = TokenKind::keyword(&self.source[start..end]).unwrap_or(TokenKind::Ident);
        self.push(kind, start, end)
    }

    fn unexpected_character(&self, start: usize) -> MoriDiagnostic {
        let c = self.source[start..]
            .chars()
            .next()
            .expect("a character remains at the current position");
        let at = span(start, start + c.len_utf8());
        match c {
            '/' => diagnostics::division(at),
            '“' | '”' | '‘' | '’' => diagnostics::curly_quote(at),
            '\'' => diagnostics::single_quote(at),
            '_' => diagnostics::leading_underscore(at),
            c if !c.is_ascii() => diagnostics::non_ascii(at, c),
            c => diagnostics::unexpected_character(at, c),
        }
    }
}

/// Converts a byte index to an offset. Sources fit in `u32` after the bound check.
fn offset(index: usize) -> u32 {
    u32::try_from(index).expect("source offsets fit in u32")
}

fn span(start: usize, end: usize) -> Span {
    Span::new(offset(start), offset(end))
}

```

## COMPLETE FILE: source-context/crates/mori-lexer/src/token.rs
```text
use mori_span::Span;
use soa_rs::{Soa, Soars};

use crate::scan;

/// The kind of a token. Keywords get their own kinds so the parser compares
/// tags instead of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    /// A name that is not a keyword: `[A-Za-z][A-Za-z0-9_]*`, at most 64 bytes.
    Ident,
    /// A canonical decimal number, such as `1_000` or `10.50`.
    Number,
    /// A JSON string literal, such as `"Midnight"`.
    String,

    // Keywords stay contiguous from `KwProfile` to `KwShareClass`; see `is_keyword`.
    KwProfile,
    KwAgreement,
    KwAction,
    KwUses,
    KwTrue,
    KwFalse,
    KwNone,
    KwSuccessOnly,

    // Declaration kinds stay contiguous from `KwDomain` to `KwShareClass`; see
    // `is_declaration_kind`.
    KwDomain,
    KwAccount,
    KwAsset,
    KwConst,
    KwIntent,
    KwObligation,
    KwPool,
    KwInstrument,
    KwObservation,
    KwPolicy,
    KwGrant,
    KwStage,
    KwEpisode,
    KwParty,
    KwShareClass,

    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    Semicolon,
    Comma,
    Eq,
    Lt,
    Gt,
    Dot,
    Plus,
    Minus,
    Star,

    Eof,
}

impl TokenKind {
    /// The keyword spelled `text`, if any.
    pub fn keyword(text: &str) -> Option<Self> {
        let kind = match text {
            "profile" => Self::KwProfile,
            "agreement" => Self::KwAgreement,
            "action" => Self::KwAction,
            "uses" => Self::KwUses,
            "true" => Self::KwTrue,
            "false" => Self::KwFalse,
            "None" => Self::KwNone,
            "SuccessOnly" => Self::KwSuccessOnly,
            "domain" => Self::KwDomain,
            "account" => Self::KwAccount,
            "asset" => Self::KwAsset,
            "const" => Self::KwConst,
            "intent" => Self::KwIntent,
            "obligation" => Self::KwObligation,
            "pool" => Self::KwPool,
            "instrument" => Self::KwInstrument,
            "observation" => Self::KwObservation,
            "policy" => Self::KwPolicy,
            "grant" => Self::KwGrant,
            "stage" => Self::KwStage,
            "episode" => Self::KwEpisode,
            "party" => Self::KwParty,
            "share_class" => Self::KwShareClass,
            _ => return None,
        };
        Some(kind)
    }

    /// The punctuation token for byte `b`, if any.
    pub fn punctuation(b: u8) -> Option<Self> {
        let kind = match b {
            b'{' => Self::LBrace,
            b'}' => Self::RBrace,
            b'[' => Self::LBracket,
            b']' => Self::RBracket,
            b'(' => Self::LParen,
            b')' => Self::RParen,
            b':' => Self::Colon,
            b';' => Self::Semicolon,
            b',' => Self::Comma,
            b'=' => Self::Eq,
            b'<' => Self::Lt,
            b'>' => Self::Gt,
            b'.' => Self::Dot,
            b'+' => Self::Plus,
            b'-' => Self::Minus,
            b'*' => Self::Star,
            _ => return None,
        };
        Some(kind)
    }

    pub fn is_keyword(self) -> bool {
        (Self::KwProfile as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Keywords that start a declaration, such as `const` or `asset`.
    pub fn is_declaration_kind(self) -> bool {
        (Self::KwDomain as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Identifiers and keywords. Record fields and call names accept either.
    pub fn is_word(self) -> bool {
        self == Self::Ident || self.is_keyword()
    }
}

/// A token's kind and start offset. Its end is recovered by re-scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: u32,
}

/// A `//` or `/* */` comment, kept for the formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Comment {
    pub start: u32,
    pub end: u32,
}

/// Index of a token in [`Tokens`]. Sources hold at most 8,192 tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenIdx(u16);

impl TokenIdx {
    pub fn new(index: usize) -> Self {
        Self(u16::try_from(index).expect("token index exceeds u16"))
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

/// The output of [`lex`](crate::lex): every token, ending with
/// [`TokenKind::Eof`], and every comment, both in source order.
#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub tokens: Soa<Token>,
    pub comments: Soa<Comment>,
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Always false: a token list ends with [`TokenKind::Eof`].
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn kind(&self, idx: TokenIdx) -> TokenKind {
        self.tokens.kind()[idx.index()]
    }

    pub fn start(&self, idx: TokenIdx) -> u32 {
        self.tokens.start()[idx.index()]
    }

    /// The token's span, recovered by re-scanning `source` from its start.
    pub fn span(&self, idx: TokenIdx, source: &str) -> Span {
        let start = self.start(idx);
        Span::new(
            start,
            scan::token_end(self.kind(idx), source.as_bytes(), start),
        )
    }

    pub fn comment_span(&self, index: usize) -> Span {
        Span::new(self.comments.start()[index], self.comments.end()[index])
    }
}

```

## COMPLETE FILE: source-context/crates/mori-lexer/src/scan.rs
```text
//! Token extent rules, shared by the lexer and by token-end recovery.

use crate::TokenKind;

pub fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// End of the identifier or keyword starting at `start`.
pub fn word_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| is_word_byte(b))
            .count()
}

/// End of the number starting at `start`. Numbers extend over digits, `_`
/// and `.`; spelling is validated separately.
pub fn number_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| b.is_ascii_digit() || b == b'_' || b == b'.')
            .count()
}

/// End of the string starting with `"` at `start`, just past the closing
/// quote, or `None` if it is unterminated. A backslash escapes the next byte.
pub fn string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut escaped = false;
    for (offset, &b) in bytes[start + 1..].iter().enumerate() {
        if b == b'"' && !escaped {
            return Some(start + 1 + offset + 1);
        }
        escaped = b == b'\\' && !escaped;
    }
    None
}

/// Recovers the end offset of a token from its kind and start.
pub fn token_end(kind: TokenKind, bytes: &[u8], start: u32) -> u32 {
    let start_index = start as usize;
    let end = match kind {
        TokenKind::Ident => word_end(bytes, start_index),
        _ if kind.is_keyword() => word_end(bytes, start_index),
        TokenKind::Number => number_end(bytes, start_index),
        TokenKind::String => string_end(bytes, start_index).expect("lexed strings are terminated"),
        TokenKind::Eof => start_index,
        _ => start_index + 1,
    };
    end as u32
}

```

## COMPLETE FILE: source-context/crates/mori-lexer/src/string.rs
```text
//! JSON string literals.
//!
//! Strings use JSON syntax: no raw control characters, and only the escapes
//! `\" \\ \/ \b \f \n \r \t \uXXXX`. A `\u` escape that encodes half of a
//! surrogate pair without its other half is rejected, but only after the
//! whole literal is valid JSON, matching the reference implementation.

use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringErrorKind {
    /// A raw character below U+0020, such as a line break.
    ControlCharacter(char),
    /// A backslash followed by a character that is not a JSON escape.
    InvalidEscape,
    /// `\u` not followed by four hexadecimal digits.
    InvalidUnicodeEscape,
    /// A `\u` escape for half of a surrogate pair, without its other half.
    LoneSurrogate,
}

/// An error at byte range `start..end` of the literal, quotes included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringError {
    pub kind: StringErrorKind,
    pub start: usize,
    pub end: usize,
}

/// Decodes the literal `raw`, quotes included, passing each character to
/// `emit`. `raw` must start and end with an unescaped `"`.
pub fn decode_with(raw: &str, mut emit: impl FnMut(char)) -> Result<(), StringError> {
    let body = &raw[1..raw.len() - 1];
    let mut chars = body.char_indices().peekable();
    let mut pending_high: Option<(u32, usize)> = None;
    let mut first_lone: Option<(usize, usize)> = None;

    let error = |kind, start: usize, end: usize| StringError {
        kind,
        start: start + 1,
        end: end + 1,
    };

    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            let Some((_, escape)) = chars.next() else {
                return Err(error(StringErrorKind::InvalidEscape, i, i + 1));
            };
            if escape == 'u' {
                let unit = hex4(&mut chars).ok_or_else(|| {
                    let end = chars.peek().map_or(body.len(), |&(j, _)| j);
                    error(StringErrorKind::InvalidUnicodeEscape, i, end)
                })?;
                let end = i + 6;
                match (pending_high.take(), unit) {
                    (Some((high, _)), 0xDC00..=0xDFFF) => {
                        let scalar = 0x10000 + ((high - 0xD800) << 10) + (unit - 0xDC00);
                        emit(char::from_u32(scalar).expect("surrogate pair is a scalar"));
                    }
                    (pending, _) => {
                        if let Some((_, start)) = pending {
                            first_lone.get_or_insert((start, start + 6));
                            emit(char::REPLACEMENT_CHARACTER);
                        }
                        match unit {
                            0xD800..=0xDBFF => pending_high = Some((unit, i)),
                            0xDC00..=0xDFFF => {
                                first_lone.get_or_insert((i, end));
                                emit(char::REPLACEMENT_CHARACTER);
                            }
                            _ => emit(char::from_u32(unit).expect("non-surrogate is a scalar")),
                        }
                    }
                }
                continue;
            }

            let decoded = match escape {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{8}',
                'f' => '\u{c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => {
                    let end = i + 1 + escape.len_utf8();
                    return Err(error(StringErrorKind::InvalidEscape, i, end));
                }
            };
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(decoded);
        } else if c < ' ' {
            return Err(error(StringErrorKind::ControlCharacter(c), i, i + 1));
        } else {
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(c);
        }
    }
    flush_high(&mut pending_high, &mut first_lone, &mut emit);

    match first_lone {
        Some((start, end)) => Err(error(StringErrorKind::LoneSurrogate, start, end)),
        None => Ok(()),
    }
}

/// A high surrogate not followed by a low surrogate escape is lone.
fn flush_high(
    pending_high: &mut Option<(u32, usize)>,
    first_lone: &mut Option<(usize, usize)>,
    emit: &mut impl FnMut(char),
) {
    if let Some((_, start)) = pending_high.take() {
        first_lone.get_or_insert((start, start + 6));
        emit(char::REPLACEMENT_CHARACTER);
    }
}

/// Reads exactly four hexadecimal digits.
fn hex4(chars: &mut Peekable<CharIndices<'_>>) -> Option<u32> {
    let mut value = 0;
    for _ in 0..4 {
        let digit = chars.peek()?.1.to_digit(16)?;
        chars.next();
        value = value * 16 + digit;
    }
    Some(value)
}

```

## COMPLETE FILE: source-context/crates/mori-lexer/src/number.rs
```text
//! Canonical number spelling.
//!
//! A number is an integer part, optionally followed by `.` and a fraction
//! part. The integer part is `0` or starts with `1`–`9`. In both parts `_` may
//! only separate two digits. This matches the reference spelling
//! `(0|[1-9][0-9]*(_[0-9]+)*)(\.[0-9]+(_[0-9]+)*)?`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberError {
    /// The integer part has more than one digit and starts with `0`.
    LeadingZero,
    /// A `_` at this byte offset is not between two digits.
    MisplacedUnderscore { at: usize },
    /// The `.` at the end has no digits after it.
    MissingFraction,
    /// A second `.` at this byte offset.
    ExtraDot { at: usize },
}

/// Checks the spelling of a number scanned by `scan::number_end`.
pub fn validate(raw: &str) -> Result<(), NumberError> {
    let mut dots = raw.match_indices('.').map(|(at, _)| at);
    let dot = dots.next();
    if let Some(at) = dots.next() {
        return Err(NumberError::ExtraDot { at });
    }

    let integer = &raw[..dot.unwrap_or(raw.len())];
    if integer.len() > 1 && integer.starts_with('0') {
        return Err(NumberError::LeadingZero);
    }
    check_underscores(integer, 0)?;

    if let Some(dot) = dot {
        let fraction = &raw[dot + 1..];
        if fraction.is_empty() {
            return Err(NumberError::MissingFraction);
        }
        check_underscores(fraction, dot + 1)?;
    }
    Ok(())
}

/// `part` contains only digits and `_` and starts at `offset` in the number.
fn check_underscores(part: &str, offset: usize) -> Result<(), NumberError> {
    let bytes = part.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let between_digits = i > 0
            && bytes[i - 1].is_ascii_digit()
            && bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        if b == b'_' && !between_digits {
            return Err(NumberError::MisplacedUnderscore { at: offset + i });
        }
    }
    Ok(())
}

/// The number with redundant leading zeros removed, for suggestions.
pub fn without_leading_zeros(raw: &str) -> String {
    let (integer, rest) = raw.split_at(raw.find('.').unwrap_or(raw.len()));
    let trimmed = integer.trim_start_matches(['0', '_']);
    let integer = if trimmed.is_empty() { "0" } else { trimmed };
    format!("{integer}{rest}")
}

```

## COMPLETE FILE: source-context/crates/mori/src/commands/check.rs
```text
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

use super::check_source::{self, SourceCheckConsumer};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    let outcome = check_source::dispatch(
        &source,
        CliConsumer {
            name: &name,
            source: &source,
            json: args.json,
            shell,
        },
    )?;
    if outcome.beta {
        if let Some(report) = outcome.report {
            super::core_io::print(&report)?;
        } else if outcome.accepted {
            shell.status(
                "Finished",
                format!("check in {:.2}s", started.elapsed().as_secs_f64()),
            );
        }
    }
    Ok(if outcome.accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

struct CliOutcome {
    accepted: bool,
    beta: bool,
    report: Option<CheckReport>,
}

struct CliConsumer<'a> {
    name: &'a str,
    source: &'a str,
    json: bool,
    shell: &'a mut Shell,
}

impl SourceCheckConsumer for CliConsumer<'_> {
    type Output = super::Result<CliOutcome>;

    fn parsed(&mut self, tokens: usize, nodes: usize) {
        if !self.json {
            self.shell
                .verbose_status("Parsed", format!("{tokens} tokens, {nodes} nodes"));
        }
    }

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        let report = self.json.then(|| {
            check_source::beta_report(self.name.to_owned(), self.source, checked, &diagnostics)
        });
        let accepted = finish(self.shell, self.name, self.source, diagnostics, self.json);
        Ok(CliOutcome {
            accepted,
            beta: true,
            report,
        })
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        if self.json {
            let report = check_source::core_report(self.name, self.source, &result, staged);
            super::core_io::print(&report)?;
        } else {
            match &result {
                Ok(_) => self.shell.status("Finished", "bounded core check"),
                Err(error) => self.shell.error(error),
            }
        }
        Ok(CliOutcome {
            accepted: result.is_ok(),
            beta: false,
            report: None,
        })
    }
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}

```

## COMPLETE FILE: source-context/crates/mori/src/commands/check_source.rs
```text
//! Shared source-profile dispatch and the existing check report formats.

use mori_diagnostics::MoriDiagnostic;

use crate::json::CheckReport;

/// Consume checked values while the Beta AST they borrow is still alive.
/// Reports remain optional: a human consumer can render diagnostics directly.
pub(super) trait SourceCheckConsumer {
    type Output;

    /// Called after Beta parsing, before its checker starts.
    fn parsed(&mut self, _tokens: usize, _nodes: usize) {}

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output;

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output;
}

/// Select exactly the same Beta, Core1 or staged Core2 path as `mori check`.
pub(super) fn dispatch<C: SourceCheckConsumer>(source: &str, mut consumer: C) -> C::Output {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(source).is_ok_and(|language| language == "moriarty/2");
        let result = if staged {
            mori_core::stage::load(source).map(|program| program.program().clone())
        } else {
            mori_core::parse(source)
        };
        return consumer.core(result, staged);
    }
    match mori_parser::parse(source) {
        Err(error) => consumer.beta(None, vec![error]),
        Ok(ast) => {
            consumer.parsed(ast.tokens.len(), ast.nodes.len());
            let mori_checker::CheckResult {
                checked,
                diagnostics,
            } = mori_checker::check(&ast);
            consumer.beta(Some(&checked), diagnostics)
        }
    }
}

/// Preserve the Beta struct's serialization order and diagnostic/span mapping.
pub(super) fn beta_report(
    file: String,
    source: &str,
    checked: Option<&mori_checker::Checked<'_>>,
    diagnostics: &[MoriDiagnostic],
) -> CheckReport {
    CheckReport::new(file, source, checked, diagnostics)
}

/// Construct the existing Core JSON object only when a consumer requests it.
pub(super) fn core_report(
    file: &str,
    source: &str,
    result: &Result<mori_core::Program, mori_core::Error>,
    staged: bool,
) -> serde_json::Value {
    let mut report = match result {
        Ok(program) => {
            serde_json::json!({"status":"accepted", "file":file,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
        }
        Err(error) => {
            serde_json::json!({"status":"rejected","file":file,"diagnostics":[error]})
        }
    };
    if staged {
        let object = report
            .as_object_mut()
            .expect("stage reports are JSON objects");
        for (name, value) in mori_core::stage::STAGE_LABELS {
            object.insert(name.into(), serde_json::Value::String(value.into()));
        }
    }
    report
}

```

## EXACT RANGE backed by source-context/crates/mori-core/src/ir.rs lines 466-483: context-ranges/01.txt
```text
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub offset: Option<u32>,
    pub step: Option<u16>,
}
impl Error {
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: format!("mori::core::{code}"),
            message: message.into(),
            offset: None,
            step: None,
        }
    }
}

```

## EXACT RANGE backed by source-context/crates/mori/src/commands/simulate.rs lines 61-156: context-ranges/02.txt
```text
    // malformed invocation reads no file.
    if let Some(resume) = &args.resume {
        return run_resume(&args, resume);
    }
    let (action, scenario_path) = args.legacy()?;
    if args.trace {
        let path = args
            .target
            .as_ref()
            .ok_or_else(|| super::core_io::operation("--trace requires --target"))?;
        #[cfg(feature = "native")]
        let target = super::core_io::read_json(path)?;
        #[cfg(feature = "native")]
        let authorization = args.authorization.load(&target)?;
        #[cfg(feature = "native")]
        let source = super::core_io::read_source(&args.file)?;
        #[cfg(feature = "native")]
        let scenario = super::core_io::read_json(scenario_path)?;
        #[cfg(feature = "native")]
        return match mori_backend::replay::generate(
            &source,
            action,
            &scenario,
            &target,
            authorization.as_input(),
        ) {
            Ok(record) => {
                super::core_io::print(&record)?;
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                super::core_io::print(
                    &serde_json::json!({"status":"rejected","qualification":"local-reference-replay","error":error}),
                )?;
                Ok(ExitCode::FAILURE)
            }
        };
        #[cfg(not(feature = "native"))]
        {
            let _ = path;
            return super::native_unavailable();
        }
    }
    if args.target.is_some() || args.authorization.present() {
        return Err(super::core_io::operation(
            "--target and signed replay arguments require --trace",
        ));
    }
    let source = super::core_io::read_source(&args.file)?;
    // Hook: /2 routing. A /2 start keeps every core failure labelled.
    if mori_core::language(&source) == Ok(mori_core::stage::LANGUAGE_2) {
        return run_stage_start(&args, &source, action, scenario_path);
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let scenario = super::core_io::read_json(scenario_path)?;
    if let Some(cap) = args.logical_cap {
        return match mori_core::simulate_metered(&program, action, &scenario, cap) {
            Ok(metered) => {
                super::core_io::print(&serde_json::json!({
                    "status": "accepted", "qualification": "local-reference-execution",
                    "execution": metered.execution, "profile": metered.cost.profile,
                    "logical": {"cap": metered.cost.cap, "used": metered.cost.used}
                }))?;
                Ok(ExitCode::SUCCESS)
            }
            Err(refused) => {
                super::core_io::print(&serde_json::json!({
                    "status": "rejected", "error": refused.error,
                    "profile": refused.cost.profile,
                    "logical": {"cap": refused.cost.cap, "used": refused.cost.used}
                }))?;
                Ok(ExitCode::FAILURE)
            }
        };
    }
    match mori_core::simulate(&program, action, &scenario) {
        Ok(execution) => {
            super::core_io::print(
                &serde_json::json!({"status":"accepted", "qualification":"local-reference-execution", "execution":execution}),
            )?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            super::core_io::print(&serde_json::json!({"status":"rejected", "error":error}))?;
            Ok(ExitCode::FAILURE)
        }
    }
}

/// Render the same explicit qualification on every staged public object.
pub(super) fn print_stage_report(mut report: serde_json::Value) -> super::Result<()> {
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in mori_core::stage::STAGE_LABELS {
        object.insert(name.into(), serde_json::Value::String(value.into()));

```

## EXACT RANGE backed by source-context/crates/mori/src/commands/compile.rs lines 1-73: context-ranges/03.txt
```text
#[cfg(feature = "native")]
use super::core_io;
#[cfg(feature = "native")]
use crate::error::CliError;
use crate::shell::Shell;
#[cfg(feature = "native")]
use std::path::Path;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    /// Exact target profile JSON with network, anchor and compiler limits
    #[arg(long)]
    pub target: PathBuf,
    /// New output directory; refuses to overwrite existing artifacts
    #[arg(long)]
    pub out: PathBuf,
}
#[cfg(feature = "native")]
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let source = core_io::read_source(&args.file)?;
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        let target: mori_backend::Target = core_io::read_json(&args.target)?;
        let artifact = mori_backend::stage::compile_staged(&source, &target, &args.out)
            .map_err(core_io::operation)?;
        let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
        std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
            path: args.out.clone(),
            source,
        })?;
        publish_staged_artifact(&artifact, &manifest, &args.out)?;
        shell.status(
            "Compiled",
            format!(
                "{} circuits into {}",
                artifact.circuits.len(),
                args.out.display()
            ),
        );
        return Ok(ExitCode::SUCCESS);
    }
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    let target: mori_backend::Target = core_io::read_json(&args.target)?;
    // The trusted Target selects the mode explicitly; a signed Target never
    // reaches the unsigned compiler.
    let reported = mori_backend::source_projection::compile_reported(&program, &target)
        .map_err(core_io::operation)?;
    let artifact = reported.artifact;
    let projection = serde_json::to_string_pretty(&serde_json::json!({
        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
        "compilation": reported.source_projection,
    }))
    .map_err(core_io::operation)?;
    // Compile and serialize fully before touching the destination.
    let manifest = serde_json::to_string_pretty(&artifact).map_err(core_io::operation)?;
    std::fs::create_dir(&args.out).map_err(|source| CliError::Write {
        path: args.out.clone(),
        source,
    })?;
    publish_artifact(&artifact, &manifest, Some(&projection), &args.out)?;
    shell.status(
        "Compiled",
        format!(
            "{} circuits into {}",
            artifact.circuits.len(),
            args.out.display()
        ),
    );
    Ok(ExitCode::SUCCESS)
}

// The caller has reserved a new directory after all input/compiler preflight.

```

## EXACT RANGE backed by source-context/crates/mori/src/commands/inspect.rs lines 1-110: context-ranges/04.txt
```text
#[cfg(feature = "native")]
use std::collections::BTreeSet;
use std::{path::PathBuf, process::ExitCode};

#[cfg(feature = "native")]
use mori_backend::{Artifact, CircuitArtifact, PublicBinding};
#[cfg(feature = "native")]
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type};
use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
    /// Compile a public enforcement map against this exact target descriptor
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,
}

pub fn run(args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    #[cfg(not(feature = "native"))]
    if args.target.is_some() {
        return super::native_unavailable();
    }
    let source = super::core_io::read_source(&args.file)?;
    if args.target.is_none()
        && mori_core::language(&source).is_ok_and(|language| language == "moriarty/2")
    {
        let staged = match mori_core::stage::load(&source) {
            Ok(staged) => staged,
            Err(error) => {
                super::simulate::print_stage_report(json!({"status":"rejected", "error":error}))?;
                return Ok(ExitCode::FAILURE);
            }
        };
        let program = staged.program();
        super::simulate::print_stage_report(json!({
            "status": "accepted", "language": program.language, "semantics": program.semantics,
            "agreement": program.name, "states": program.states, "assets": program.assets,
            "actions": program.actions, "expressions": program.expressions,
            "stage_plan": staged.plan(), "enforcement_report": null
        }))?;
        return Ok(ExitCode::SUCCESS);
    }
    #[cfg(feature = "native")]
    if mori_core::language(&source).is_ok_and(|language| language == "moriarty/2") {
        // The enforcement map reports the /3 relation. A Core2 --target refuses
        // with Core's typed profile error before reading the target, backend
        // layout or any output; source-only Core2 inspect is handled above.
        let staged = mori_core::stage::load(&source).map_err(super::core_io::operation)?;
        let refusal = match mori_core::stage::require_core1(staged.program()) {
            Err(error) => error,
            Ok(()) => core1_refusal(),
        };
        return Err(super::core_io::operation(refusal));
    }
    let program = mori_core::parse(&source).map_err(super::core_io::operation)?;
    let enforcement = match args.target {
        #[cfg(feature = "native")]
        Some(path) => {
            let target: mori_backend::Target = super::core_io::read_json(&path)?;
            // Use the same whole-program admission and explicit mode as compile.
            // No scenario, key, signature, proof or native execution is consumed.
            let artifact = if target.signed_policy.is_some() {
                mori_backend::compile_signed(&program, &target)
            } else {
                mori_backend::compile(&program, &target)
            }
            .map_err(super::core_io::operation)?;
            enforcement_report(&source, &program, &artifact)?
        }
        #[cfg(not(feature = "native"))]
        Some(_) => return super::native_unavailable(),
        None => Value::Null,
    };
    let qualification = if enforcement.is_null() {
        "source-only-unqualified"
    } else {
        "local-compiled-unqualified"
    };
    // Export validates depth/work/byte bounds and escapes public labels before
    // printing anything. A compiler or report-size refusal leaves stdout empty.
    super::core_io::print(&json!({
        "format": "moriarty-enforcement-report/1",
        "qualification": qualification,
        "language": program.language, "semantics": program.semantics,
        "agreement": program.name, "states": program.states,
        "assets": program.assets, "actions": program.actions,
        "expressions": program.expressions,
        "enforcement_report": enforcement,
        "open_premises": [
            "a supplied exact target is required for the compiled enforcement map",
            "ledger authentication and acceptance",
            "native proof soundness and compiler correspondence",
            "account and sponsor authority"
        ]
    }))?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "native")]
fn encoded(predicates: &[&str]) -> Value {
    json!({"status": "encoded-not-verified", "predicates": predicates})
}
#[cfg(feature = "native")]
fn reference(predicates: &[&str]) -> Value {
    json!({"status": "available-not-executed", "predicates": predicates})
}

```

## EXACT RANGE backed by source-context/crates/mori/src/commands/core_io.rs lines 1-23: context-ranges/05.txt
```text
use crate::error::CliError;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub fn read_source(path: &Path) -> super::Result<String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut source = String::new();
    file.take(1_048_577)
        .read_to_string(&mut source)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if source.len() > 1_048_576 {
        return Err(operation("input file exceeds 1 MiB"));
    }
    Ok(source)
}


```


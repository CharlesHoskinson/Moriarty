Read-only independent Grok 4.6 high-effort full-candidate audit. Review the exact file bytes embedded below. Do not use tools or edit files. Identify concrete high/medium correctness defects in Source/6 formation, lowering, judgment precedence, financial effects, typed external premises, and K/Quint correspondence. Cite file:line and give a verdict: adequate provisional prototype or reject, with scope. Do not claim tests, execution, verification, or native/ledger acceptance. Preserve dissent. The repository guarded SP01.6 delivery is blocked; this packet is an isolated research implementation.

FILE experiments/moriarty-language/spec/successor/financial-agreement-source-v6-grammar.ebnf
SHA256 5fb03d51d863e5c959e1cb80f67da9d261c668200837bfab4d748cda0403f72e
```
(* ISO 14977 EBNF; provisional Source/6 S0 presentation syntax only.
   This is not the /3 signed-intent byte codec. The exact profile string is
   moriarty-financial-agreement-source/6. Whitespace, comments, identifiers,
   integers and strings use lexical.md. New words and the .. interval token
   below are profile-local and do not extend Source/5.
   All productions are closed; unknown fields and duplicate fields reject.
   The companion contract defines nominal typing, caps and Core/5 mapping. *)

source = header, agreement, ? end of file ? ;
header = "profile", '"moriarty-financial-agreement-source/6"', ";" ;
agreement = "agreement", identifier, "{", domain, settlement,
            selected, intent, authenticated, submitted, "}" ;
domain = "domain", identifier, ";" ;
settlement = "settlement", identifier, "scale", integerToken, ";" ;
selected = "selected", identifier, "source_hash", stringToken,
           "digest", stringToken, ";" ;

intent = "intent", "{", "signer", identifier, "key", stringToken, ";",
         "nonce", stringToken, ";", "pre_head", stringToken, ";",
         "valid", integerToken, "..", integerToken, ";",
         "gross_cap", integerToken, ";", "fee_cap", integerToken, ";",
         "net_floor", integerToken, ";", "failure", "success_only", ";",
         "signed_action", ( transfer | repay ),
         "observations", "empty", ";", "disclosures", "empty", ";",
         "retained_effects", "empty", ";", "retained_duties", "empty", ";",
         "delegation", "none", ";", "recovery", "none", ";", "}" ;

authenticated = "authenticated", "{", "head", stringToken, ";",
                "predecessor", stringToken, ";", "round", integerToken, ";",
                "balance", identifier, integerToken, ";",
                "balance", identifier, integerToken, ";",
                [ "balance", identifier, integerToken, ";" ],
                "allowance", identifier, "remaining", integerToken,
                "spent", integerToken, ";",
                [ obligation ], "replay", ( "unused" | "consumed" ), ";",
                "work_remaining", integerToken, ";",
                "work_spent", integerToken, ";", "}" ;
obligation = "obligation", identifier, "{", "debtor", identifier, ";",
             "creditor", identifier, ";", "asset", identifier, ";",
             "principal", integerToken, ";", "accrued", integerToken, ";",
             "outstanding", integerToken, ";", "status", "outstanding", ";",
             "}" ;

submitted = "submit", ( transfer | repay ), "effects", "{", effectVector, "}",
            "post_head", stringToken, ";" ;
transfer = "transfer", "from", identifier, "to", identifier,
           "fee_to", identifier, "value", integerToken,
           "fee", integerToken, ";" ;
repay = "repay", "obligation", identifier, "payer", identifier,
        "amount", integerToken, "conversion", "identity", ";" ;
effectVector = transferEffects | repayEffects ;
transferEffects = debit, credit, [ credit ], useAllowance, useReplay, advanceHead ;
repayEffects = debit, credit, setObligation, useAllowance, useReplay, advanceHead ;
debit = "debit", identifier, integerToken, ";" ;
credit = "credit", identifier, integerToken, ";" ;
setObligation = "set_obligation", identifier, "principal", integerToken,
                "accrued", integerToken, "outstanding", integerToken,
                "status", ( "outstanding" | "settled" ), ";" ;
useAllowance = "use_allowance", identifier, integerToken, ";" ;
useReplay = "use_replay", stringToken, ";" ;
advanceHead = "advance_head", stringToken, stringToken, ";" ;

identifier = ? ASCII identifier token in lexical.md ? ;
integerToken = ? canonical unsigned decimal token in lexical.md ? ;
stringToken = ? JSON string token in lexical.md ? ;

```

FILE experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md
SHA256 39b0e8c9d3c5301f3575f6c3aea47fe2c3b2e1ab5b0c70dca00f4e82a645c511
```
# Financial agreement Source/6 and Core/5 S0 contract

**Status:** provisional Sprint 0 specification, 2026-09-29. The grammar is a closed presentation syntax for one S0 stage proposal. It is not an implemented parser, a signed `/3` encoding, or an adopted MIL/4 profile. W-D0–W-D4 and independent votes remain open. See the [MIL/4 semantic contract](../../formal/mil4/semantics-contract.md) and [S0 implementation contract](../../formal/mil4/s0-implementation-contract.md).

## Formation and version gate

The [EBNF](financial-agreement-source-v6-grammar.ebnf) parses exactly one `profile`, one `agreement`, and one stage proposal. Its profile string must equal `moriarty-financial-agreement-source/6` before parsing the agreement body. The fixed declaration and field order is presentation syntax, not a claim about signed bytes. Unknown, repeated, omitted or out-of-order fields reject at Source/6 formation with `SOURCE6_SHAPE`; an unknown action or effect tag rejects with `SOURCE6_UNKNOWN_TAG`. A different header rejects with `SOURCE6_VERSION`. The closed source grammar also rejects a nonempty failure, observation, disclosure, retained-effect or duty form at formation with `SOURCE6_SHAPE`; `S0_FAILURE_UNSUPPORTED` applies only when a typed Core/5 stage reaches the failure judgment. These source rejections have no Core/5 term or K transition. Source/5 sees a `/6` header as a version rejection; Core/4 sees a Core/5 tag as a version rejection. Neither compatibility behavior has been implemented or verified here.

The grammar borrows only the token definitions from `lexical.md`; the words added here are local to Source/6. Limits are simultaneous: source UTF-8 bytes ≤65536, tokens ≤8192, AST nodes ≤8192, nesting depth ≤64, identifiers ≤64 ASCII characters, decoded strings ≤1024 UTF-8 bytes, and one stage/effect vector per document. `scale` is 0..18; each nominal amount (`value`, `fee`, `amount`, caps and floor) is 0..`2^127−1`; each balance, allowance counter and work count is 0..`2^128−1`. S0 uses checked UInt128 intermediates, including `value+fee`, `principal+accrued`, credits and spent counters. The current source/kernel nominal bound does not by itself impose the same bound on every lifecycle state field. This S0 proposal additionally caps principal, accrued and outstanding at `2^127−1` and requires `outstanding=principal+accrued`. W-D4 must review that additional narrowing. Invalid bounds reject before K admission with `SOURCE6_RANGE`.

The grammar's balance count is action dependent. Transfer requires three rows in owner, recipient, fee-recipient order. Repay requires two rows in payer, bound-creditor order and exactly one obligation. All endpoint identifiers are pairwise distinct for transfer; payer and creditor differ for repay. Duplicate or missing authenticated cells reject `SOURCE6_CELL_SHAPE`; an absent receiver balance is not silently initialized. The allowance owner equals signer. The replay row states whether the selected signed key is unused or consumed; a consumed row rejects at history. The authenticated block is a *claim* until a snapshot-to-head premise establishes each cell and the current head. The strings used for key, nonce, digest and heads are opaque typed identifiers in this syntax, not hash byte definitions.

## Source fields to Core/5

One elaboration produces `Core5Stage(pre, action, signedScope, submittedEffects, premises)`. The Core/5 tags below are proposed typed constructors. No caller Boolean can establish authentication or signature validity.

| Source/6 field or form | Core/5 field or constructor | Rule |
| --- | --- | --- |
| `profile`, `agreement`, `domain`, `settlement` | `Version(Source6,Core5)`, `ProgramId`, `DomainId`, `AssetId(scale)` | Version exact; one domain and asset; asset identity is nominal. |
| `selected … source_hash … digest` | `SelectedProgram(actionId,sourceHash,policyDigest)` | Bind all three to the signed statement; no implicit action selection. S0 uses exact action IDs `TransferLiteralFee` and `RepayAccrualFirst`, matching the signed and submitted constructor. Any other action ID rejects `SOURCE6_PROFILE_UNSUPPORTED`. |
| `signer … key`, `nonce`, `pre_head`, `valid` | `SignedScope(signer,keyRef,replayKey,preHead,roundLo,roundHi)` | `replayKey=(domain,signer,nonce)`; the exact digest and verifier are external typed premises. |
| `signed_action` | `SignedAction(TransferLiteralFee | RepayAccrualFirst)` | Fix owner/payer, recipients, fee recipient, obligation ID and quantities under the signature. The submitted action must match this signed action exactly. |
| `gross_cap`, `fee_cap`, `net_floor` | `Bounds(grossCap,feeCap,netFloor)` | Bind all three to the signed scope; authority uses gross debit. |
| `failure success_only` and six explicit empty/none fields | `FailurePolicy(SuccessOnly)`, empty observation, disclosure, retained effect and duty, no delegation or recovery | Any nonempty variant has named S0 rejection `S0_FAILURE_UNSUPPORTED`; no accepted fee-bearing failure. |
| `authenticated head`, `predecessor`, `round` | `PreHead`, `Predecessor`, `CurrentRound` | Must be authenticated against the same snapshot; current head comparison is atomic with replay consumption. |
| `balance`, `allowance`, `obligation`, `replay`, `work_remaining`, `work_spent` | `BalanceCell`, `AllowanceCell`, `ObligationCell`, `ReplayCell`, `WorkCell` | These are read cells. Obligation binds debtor, creditor, asset, principal, accrued, outstanding and status. Both work counters come from the authenticated snapshot; lowering never resets spent work. |
| `transfer` | `TransferLiteralFee(owner,recipient,feeRecipient,value,fee)` | Owner=signer. Prepare debit gross, recipient credit value, and fee credit only when fee>0. |
| `repay` | `RepayAccrualFirst(obligationId,payer,amount,IdentityConversion)` | Payer=debtor=signer. Read bound creditor and asset from obligation; do not accept caller supplied substitutes. |
| `effects` | `PreparedEffects` comparison candidate | Order and values must equal the internally derived vector below. The supplied vector does not define the effect. In this provisional text profile, `use_replay` supplies the signed nonce; Core derives the domain/signer/nonce replay key. |
| `post_head` | `PostHead` candidate | Must be a valid authenticated successor under the external head-extension premise. |

For transfer, prepare `Debit(owner,v+f)`, `Credit(recipient,v)`, optional `Credit(feeRecipient,f)` when `f>0`, `UseAllowance(owner,v+f)`, `UseReplay(key)`, `AdvanceHead(pre,post)` in exactly that order. The optional line is present iff `f>0`; a zero-valued fee line rejects. Require `v>0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, sufficient owner balance and remaining allowance, and no overflow in any receiver or spent counter. Debit and credits conserve the same nominal asset. Preserve gross effects even though balance changes could be netted. The transfer in `submit` must equal the `signed_action` transfer before effects are prepared.

For repay, require one Outstanding obligation, `0<n≤outstanding`, matching asset, identity conversion `(mantissa=1,scale=0,rounding=none)`, `feeCap=0`, `netFloor=0`, sufficient payer balance and allowance, and no overflow in the creditor or spent counter. Let `da=min(n,accrued)` and `dp=n−da`. Prepare `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(p−dp,a−da,p+a−n,status')`, `UseAllowance(payer,n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. `status'` is Settled iff outstanding becomes zero. Every unlisted authenticated cell is unchanged. A debt reduction without the creditor credit rejects. The repayment in `submit` must equal the `signed_action` repayment.

## Admission and observation

Core/5 applies `stage → intent → effect → authority → history → failure`. `stage` checks version, selected program, typed cells and authentication premises. `intent` checks exact signed scope, validity, endpoints, alias policy and all signed nominal caps and floors. `effect` computes and compares the complete ordered vector and post cells. `authority` checks signer, allowance and work budget. `history` compares the current head and unused replay key and verifies successor binding. `failure` accepts only terminal success with empty retained effects and duties. The first failing judgment returns `(judgment,code,diagnosticWork)` without a published post-state or effects. Within-judgment code spelling and precedence remain W-D3 choices; provisional codes are in `s0-implementation-contract.md`.

The result shape is `Core5Observation(pre,action,signedScope,preparedEffects,post,remainingDuty,remainingWork,replay,preHead,postHead,phase,judgment,code)`. Accepted S0 success contains complete effects, consumption, writes, one post-head and empty duty. Atomic rejection contains the first judgment/code and diagnostic work, with `post`, published effects and post-head absent. Snapshot authentication, exact signature verification, head extension and ledger compare-and-consume are typed external premises; failure or absence rejects. Source/6 grammar acceptance alone does not imply admission.

## Boundary with earlier and later forms

Source/5 `profile`, declarations, `action`, expression and `emit` forms have no automatic injection into this S0 stage. A future migration must map every legacy field, selected Core/4 program, authenticated state, signed digest and complete effect obligation, then prove the old and new observations equivalent on the stated domain. `Repay` in Core/4 consumes a separate Transfer; Core/5's `RepayAccrualFirst` is one funded stage. All old Source/5 and Core/4 behavior remains historical until such a mapping is demonstrated.

The eight MIL/4 first families and all later profiles reject from this S0 grammar with `SOURCE6_PROFILE_UNSUPPORTED` or `SOURCE6_UNKNOWN_TAG` at formation. A later full Source/6 grammar must add typed productions and Core/5 constructors per family. General Φ₁, uncertified Ω, division, rounding, mint, reserve, foreign evidence, accepted failures and recovery are outside S0. This document does not turn those forms into generic records or strings.

```

FILE experiments/moriarty-language/spec/successor/lexical.md
SHA256 88fa3353d1b1a889f1d06f8b35868ae187800719c67ab8345ebb09515e8de639
```
# Lexical rules for `moriarty-successor-syntax/0`

These rules are the lexical layer for [grammar.ebnf](grammar.ebnf). They are not a second grammar dialect. Regular expressions below are JavaScript Unicode regular expressions and apply only to this note. The ISO EBNF file refers to the tokens named here through special sequences.

## Encoding and spans

Input is a sequence of Unicode scalar values encoded as UTF-8. Lone UTF-16 surrogates in a JavaScript string, malformed UTF-8 in CLI file bytes, and escaped lone surrogates in string literals all fail. The CLI decoder uses a fatal UTF-8 decode. It does not replace invalid bytes.

Source locations are 0-based UTF-8 byte offsets on the half-open interval `[start, end)`. Comments and whitespace occupy bytes and count toward the 65536-byte source bound. They are not tokens and they are not AST nodes.

The parser does not normalize Unicode, strip a BOM, or rewrite the caller string.

## Whitespace

ASCII space, tab, CR, and LF may separate tokens. No other code point is whitespace.

## Comments

Line comments start with `//` and run through the last character before a U+000A line feed, or through end of file if no line feed remains. A line comment at end of file is terminated.

Block comments start with `/*` and end at the first `*/`. They do not nest. `/* /* */` ends at the first closer. An unclosed `/*` fails with `UNTERMINATED_COMMENT`.

A `/` that does not start `//` or `/*` is not a token. There is no division operator.

## Identifiers and keywords

An identifier is `[A-Za-z][A-Za-z0-9_]*` and is at most 64 ASCII characters. Identifiers are case-sensitive. A non-ASCII letter, number, connector punctuation, nonspacing mark, or spacing combining mark (Unicode categories `L`, `N`, `Pc`, `Mn`, or `Mc`) at the start of a token or immediately after an ASCII identifier prefix fails with `NON_ASCII_IDENTIFIER`. For example, standalone U+0301 and `a` followed by U+0301 both fail with that code. Other non-ASCII code points outside strings and comments fail with `UNEXPECTED_CHAR`.

A word that equals a keyword is a keyword token. Keywords cannot be identifiers.

Keywords:

```
profile agreement unit party asset const state action
requires let next emit ensures
true false not and or
```

`pre` and `post` are identifiers. `next` is a keyword and begins a state update. `floor_div` and `ceil_div` are identifiers and parse as calls when followed by an argument list.

## Integer tokens

An integer token is `0` or `[1-9][0-9]*`. The lexer consumes `[0-9]+` and then checks that form. Leading zeros, signs, decimal points, exponent suffixes, and digit separators fail. The token is kept as decimal text. It is not converted to a IEEE number. More than 78 digits fails with `INTEGER_BOUND`.

## String tokens

A string token is a JSON string in RFC 8259 form, including the surrounding quotes. Allowed escapes are `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, and `\uXXXX` with four hexadecimal digits. Unescaped U+0000 through U+001F, unknown escapes, and an unclosed quote fail.

After decoding, the value must be Unicode scalar values. A decoded lone surrogate, including one produced by `\uD800` without a trailing low surrogate, fails with `INVALID_SURROGATE`. A valid surrogate pair in escapes is one supplementary scalar. The decoded UTF-8 length must be at most 1024 bytes.

The formatter emits the original string token text, so `\u0041` and `A` remain distinct literals.

Unicode scalar values may appear in string and comment contents. Their byte spans follow UTF-8.

## Punctuation and token priority

The lexer uses longest match in this order:

1. Whitespace (discarded)
2. Line comment or block comment (discarded)
3. Two-character operators `==`, `!=`, `<=`, `>=`
4. One-character punctuation `{ } ( ) < > , : ; . = + - *`
5. String
6. Integer
7. Keyword or identifier
8. Failure

There is no `>>` token. Nested generics `Map<Debt<USD>>` are two `>` tokens.

`>=` is one token. A type that is immediately followed by `=` without whitespace therefore lexes as `>=`. The type parser splits a `>=` token into `>` and `=` when it is closing a type argument list, so `Debt<USD>= debt(1, USD)` is accepted. The split rewrites that token in place and does not emit a new lexer token. The extra `=` counts toward the 8192 token bound including the end-of-file token. The formatter always writes a space before `=`.

`<` and `>` are type-argument delimiters only in type position. In expressions they are comparisons. This profile has no call-site type arguments, so `foo<bar>` in an expression is `foo < bar` plus a leftover `>`.

## End of file

The grammar's `? end of file ?` special sequence matches the lexer’s single EOF sentinel after all source characters, including trailing whitespace or a terminated comment, have been consumed. EOF has zero width: its start and end spans both equal the source UTF-8 byte length. It consumes no source character and cannot match before remaining non-ignored text. The sentinel counts as one token toward the token bound.

## Token bound

Whitespace and comments are not counted. The token bound is 8192 including the end-of-file token. The lexer reserves one slot for end of file, so 8191 non-EOF tokens plus EOF are accepted and a 8192nd non-EOF token is rejected. The type parser may rewrite a `>=` token in place into `>` and `=` when it closes a type argument list. That rewrite does not emit a lexer token. The extra `=` counts toward the 8192 bound including the end-of-file token. If the bound would be exceeded, the parser fails with `TOKEN_BOUND`.

```

FILE experiments/moriarty-language/formal/mil4/s0-implementation-contract.md
SHA256 04dea37f8ba667ccd36ca9188bef973484e03d575bda3a9689d1ac1d7e8151e5
```
# S0 implementation contract, provisional

**Status:** executable prototype contract. It fixes choices for isolated K and Quint work, but does not adopt MIL/4, close W-D0–W-D4, or qualify a native/ledger path. The [semantic contract](semantics-contract.md), [projection](projection.md) and [independent cases](s0-discriminators.md) control the intended behavior.

## Version and input

Use a new `Source/6` and `Core/5` identity. S0's signed intent envelope remains a separate proposed `/3` encoding. A stage carries one selected program, domain, signer, settlement asset, signed nonce, signed pre-head, validity round interval, fixed recipient and fee recipient, gross and fee caps, net floor, and a complete submitted effect vector. Repayment also carries an authenticated obligation with debtor, creditor, settlement asset, principal, accrued, outstanding and status. A K input may use symbolic terms rather than JSON, but it must retain these fields as typed values or explicit premises.

The signer is the debited owner/payer in S0. Delegation is absent. The replay key is `(domain, signer, signed nonce)`. A second intent may repay the same obligation with a distinct nonce and current head. Snapshot-to-head authentication, signature verification of the exact intent digest, and atomic ledger compare-and-consume are named premises. An unavailable premise rejects. A caller Boolean does not establish a premise.

## Provisional Source/6 lowering

The Source/6 parser preserves the agreement ID, selected action ID, settlement scale, authenticated predecessor, signed action, submitted action, and explicit empty failure/observation/duty fields in a typed stage wrapper. The selected `digest` is the policy digest; the `/3` signed-intent digest is not defined by Source/6 and must not be fabricated during lowering. `signed_action` and `submit` must have the same constructor and every field must match before the Core/5 preparer runs. `work_remaining` and `work_spent` are both authenticated cells; neither counter may be synthesized as zero. The Source/6 `use_replay` string names the signed nonce in this provisional presentation. Core/5 derives the composite replay key from domain, signer and nonce and compares that complete line. This textual convention remains a W-D3 candidate, not final signed bytes.

## Canonical effects and arithmetic

Transfer requires `v>0`, `f≥0`, `f≤feeCap`, `v+f≤grossCap`, `v≥netFloor`, and balances/allowance sufficient for gross `v+f`. The candidate first profile requires owner, recipient and fee recipient to be pairwise distinct. This explicit narrow-profile rejection avoids ambiguous alias writes until a later alias policy is selected. Ordered effects are `Debit(owner,v+f)`, `Credit(recipient,v)`, then `Credit(feeRecipient,f)` if `f>0`; omit the zero-fee line. Preserve gross debit before deriving net cell deltas. Consume allowance remaining and increase spent by `v+f`.

Repayment requires `0<n≤outstanding`, `outstanding=principal+accrued`, status Outstanding, payer=debtor=signer, creditor from the authenticated obligation, matching settlement asset, and identity conversion. Set `da=min(n,accrued)`, `dp=n−da`, `accrued'=accrued−da`, `principal'=principal−dp`, `outstanding'=principal'+accrued'`. Ordered effects are `Debit(payer,n)`, `Credit(boundCreditor,n)`, `SetObligation(...)`, `UseAllowance(n)`, `UseReplay(key)`, `AdvanceHead(pre,post)`. Transfer has the corresponding allowance, replay and head lines. No impairment substitutes for payment.

Use checked UInt128 for balances, allowances and each intermediate sum. Apply the current source/kernel `2^127−1` bound to S0 nominal amounts and liability caps; do not mislabel every lifecycle state field as signed-width. Subtraction underflow and credit overflow reject. S0 has no division, rounding, reserve, mint or accepted retained-effect failure.

## Judgment order and result

Apply `stage → intent → effect → authority → history → failure` in that order. A candidate error uses `(firstJudgment, stableCode, diagnosticWork)` and publishes no post-state or effects. Within each judgment, check fields in the order given here; exact wire code spelling is provisional and belongs to W-D3. `intent` checks the fixed action and nominal signed caps/floor. The `effect` judgment compares complete ordered effects with the internally prepared vector and checks post-state conservation. `authority` checks signer, allowance and work budget. `history` checks pre-head and replay key. `failure` accepts only terminal success with empty retained effects and duties.

| Condition | First judgment | Provisional code |
| --- | --- | --- |
| Unsupported source/Core/profile or malformed typed state | stage | `S0_STAGE_UNSUPPORTED` |
| Changed signed endpoint, nonce, bounds or invalid validity round | intent | `S0_INTENT_SCOPE` |
| Endpoint alias in this first profile | intent | `S0_INTENT_ALIAS` |
| Submitted vector or derived post-state differs from preparation | effect | `S0_EFFECT_MISMATCH` |
| Insufficient balance, numeric overflow or invalid obligation arithmetic | effect | `S0_EFFECT_RANGE` |
| Signer/payer mismatch, allowance or work budget exceeded | authority | `S0_AUTH_SCOPE` |
| Stale pre-head or consumed replay key | history | `S0_HISTORY_STALE` or `S0_HISTORY_REPLAY` |
| Nonempty retained effect/duty or unselected failure branch | failure | `S0_FAILURE_UNSUPPORTED` |

This table is an implementation discriminator, not an accepted canonical diagnostic schedule. Compile and typecheck results may establish syntax only. Semantic traces, proofs, native qualification and ledger readback remain separate evidence.

```

FILE experiments/moriarty-language/src/successor/financial-agreement-source-v6-frontend.ts
SHA256 13c1431e7e525d08dc739d57e57af178da80c704077cecf342bf3e8039fccfe2
```
/** Provisional, closed Source/6 S0 presentation parser. No authentication occurs here. */
import {
  MIL4_S0_CORE, MIL4_S0_INTENT, MIL4_S0_SOURCE,
  type S0Effect, type S0Intent, type S0State,
} from './mil4-s0-core-v5.js';

const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const encoder = new TextEncoder();
const RESERVED = new Set((
  'profile agreement unit party asset const state action requires let next emit ensures true false not and or domain settlement scale selected source_hash digest intent signer key nonce pre_head valid gross_cap fee_cap net_floor failure success_only signed_action observations empty disclosures retained_effects retained_duties delegation none recovery authenticated head predecessor round balance allowance remaining spent obligation debtor creditor principal accrued outstanding settled status replay unused consumed work_remaining work_spent submit transfer from to fee_to value fee repay payer amount conversion identity effects debit credit set_obligation use_allowance use_replay advance_head'
).split(' '));

type TokenKind = 'word' | 'integer' | 'string' | 'punctuation' | 'eof';
interface Token { kind: TokenKind; text: string; value: string; start: number; end: number }
export class Source6Error extends Error {
  constructor(public readonly code: string, public readonly offset: number, message: string) {
    super(message);
    this.name = 'Source6Error';
  }
}
function fail(code: string, offset: number, message: string): never {
  throw new Source6Error(code, offset, message);
}
function isSurrogate(value: number): boolean { return value >= 0xd800 && value <= 0xdfff; }
function scalarString(value: string, offset: number): void {
  for (let i = 0; i < value.length; i++) {
    const c = value.charCodeAt(i);
    if (c >= 0xd800 && c <= 0xdbff && i + 1 < value.length) {
      const low = value.charCodeAt(i + 1);
      if (low >= 0xdc00 && low <= 0xdfff) { i++; continue; }
    }
    if (isSurrogate(c)) fail('INVALID_SURROGATE', offset + encoder.encode(value.slice(0, i)).length, 'Lone UTF-16 surrogate');
  }
}
function lexical(source: string): Token[] {
  scalarString(source, 0);
  if (encoder.encode(source).length > 65536) fail('SOURCE_BOUND', 0, 'Source exceeds 65536 UTF-8 bytes');
  const tokens: Token[] = [];
  let i = 0;
  let byte = 0;
  const advance = (end: number): void => { byte += encoder.encode(source.slice(i, end)).length; i = end; };
  const emit = (kind: TokenKind, end: number, value = source.slice(i, end)): void => {
    if (tokens.length >= 8191) fail('TOKEN_BOUND', byte, 'Too many tokens');
    const start = byte;
    const raw = source.slice(i, end);
    advance(end);
    tokens.push({ kind, text: raw, value, start, end: byte });
  };
  while (i < source.length) {
    const c = source[i];
    if (/[ \t\r\n]/.test(c)) { advance(i + 1); continue; }
    if (source.startsWith('//', i)) {
      const next = source.indexOf('\n', i + 2);
      advance(next < 0 ? source.length : next); continue;
    }
    if (source.startsWith('/*', i)) {
      const next = source.indexOf('*/', i + 2);
      if (next < 0) fail('UNTERMINATED_COMMENT', byte, 'Unclosed block comment');
      advance(next + 2); continue;
    }
    if (c === '"') {
      let end = i + 1;
      let escaped = false;
      for (; end < source.length; end++) {
        const x = source[end];
        if (x === '"' && !escaped) { end++; break; }
        if (x === '\\' && !escaped) escaped = true;
        else escaped = false;
      }
      const raw = source.slice(i, end);
      if (!raw.endsWith('"') || raw.length < 2) fail('INVALID_STRING', byte, 'Unclosed string');
      let value: string;
      try { value = JSON.parse(raw) as string; }
      catch { fail('INVALID_STRING', byte, 'Invalid JSON string'); }
      scalarString(value, byte);
      if (encoder.encode(value).length > 1024) fail('STRING_BOUND', byte, 'Decoded string exceeds 1024 UTF-8 bytes');
      emit('string', end, value); continue;
    }
    if (/[0-9]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[0-9]/.test(source[end])) end++;
      const raw = source.slice(i, end);
      if (raw.length > 78) fail('INTEGER_BOUND', byte, 'Integer exceeds 78 digits');
      if (!/^(0|[1-9][0-9]*)$/.test(raw)) fail('INVALID_INTEGER', byte, 'Noncanonical integer');
      emit('integer', end); continue;
    }
    if (/[A-Za-z]/.test(c)) {
      let end = i + 1;
      while (end < source.length && /[A-Za-z0-9_]/.test(source[end])) end++;
      const next = source.codePointAt(end);
      if (next !== undefined && /[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(next)))
        fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
      if (end - i > 64) fail('IDENTIFIER_BOUND', byte, 'Identifier exceeds 64 characters');
      emit('word', end); continue;
    }
    const point = source.codePointAt(i)!;
    if (/[\p{L}\p{N}\p{Pc}\p{Mn}\p{Mc}]/u.test(String.fromCodePoint(point)))
      fail('NON_ASCII_IDENTIFIER', byte, 'Non-ASCII identifier');
    if (source.startsWith('..', i)) { emit('punctuation', i + 2); continue; }
    if ('{};'.includes(c)) { emit('punctuation', i + 1); continue; }
    fail('UNEXPECTED_CHAR', byte, 'Unexpected source character');
  }
  tokens.push({ kind: 'eof', text: '', value: '', start: byte, end: byte });
  return tokens;
}

export type Source6Action =
  | { kind: 'Transfer'; from: string; to: string; feeTo: string; value: string; fee: string }
  | { kind: 'Repay'; obligation: string; payer: string; amount: string; conversion: 'identity' };
export interface Source6Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string; status: 'Outstanding';
}
export interface Source6Ast {
  profile: typeof MIL4_S0_SOURCE; programId: string; domain: string;
  settlement: { asset: string; scale: string };
  selected: { actionId: string; sourceHash: string; policyDigest: string };
  intent: {
    signer: string; keyRef: string; nonce: string; preHead: string;
    notBefore: string; notAfter: string; grossCap: string; feeCap: string; netFloor: string;
    signedAction: Source6Action; failure: 'success_only';
    observations: 'empty'; disclosures: 'empty'; retainedEffects: 'empty'; retainedDuties: 'empty';
    delegation: 'none'; recovery: 'none';
  };
  authenticated: {
    head: string; predecessor: string; round: string;
    balances: { account: string; amount: string }[];
    allowance: { owner: string; remaining: string; spent: string };
    obligation?: Source6Obligation; replay: 'unused' | 'consumed';
    workRemaining: string; workSpent: string;
  };
  submitted: { action: Source6Action; effects: S0Effect[]; postHead: string };
}

class Parser {
  private index = 0;
  private nodes = 0;
  private depth = 0;
  constructor(private readonly tokens: Token[]) {}
  private get here(): Token { return this.tokens[this.index]; }
  private node(): void {
    if (++this.nodes > 8192) fail('AST_BOUND', this.here.start, 'Too many AST nodes');
  }
  private enter(): void { if (++this.depth > 64) fail('DEPTH_BOUND', this.here.start, 'Nesting exceeds 64'); }
  private leave(): void { this.depth--; }
  private take(word: string, code = 'SOURCE6_SHAPE'): void {
    if (this.here.text !== word) fail(code, this.here.start, `Expected ${word}`);
    this.index++;
  }
  private effectTag(word: string): void {
    if (this.here.text !== word && this.here.kind === 'word'
        && !new Set(['debit', 'credit', 'set_obligation', 'use_allowance', 'use_replay', 'advance_head']).has(this.here.text))
      fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown effect tag');
    this.take(word);
  }
  private id(): string {
    const token = this.here;
    if (token.kind !== 'word' || RESERVED.has(token.text)) fail('SOURCE6_SHAPE', token.start, 'Expected identifier');
    this.index++; return token.value;
  }
  private string(): string {
    const token = this.here;
    if (token.kind !== 'string') fail('SOURCE6_SHAPE', token.start, 'Expected string');
    this.index++; return token.value;
  }
  private uint(max: bigint = U128): string {
    const token = this.here;
    if (token.kind !== 'integer') fail('SOURCE6_SHAPE', token.start, 'Expected integer');
    this.index++;
    if (BigInt(token.value) > max) fail('SOURCE6_RANGE', token.start, 'Integer exceeds nominal bound');
    return token.value;
  }
  private action(): Source6Action {
    this.node();
    if (this.here.text === 'transfer') {
      this.take('transfer'); this.take('from'); const from = this.id();
      this.take('to'); const to = this.id(); this.take('fee_to'); const feeTo = this.id();
      this.take('value'); const value = this.uint(S128); this.take('fee'); const fee = this.uint(S128);
      this.take(';'); return { kind: 'Transfer', from, to, feeTo, value, fee };
    }
    if (this.here.text === 'repay') {
      this.take('repay'); this.take('obligation'); const obligation = this.id();
      this.take('payer'); const payer = this.id(); this.take('amount'); const amount = this.uint(S128);
      this.take('conversion'); this.take('identity'); this.take(';');
      return { kind: 'Repay', obligation, payer, amount, conversion: 'identity' };
    }
    fail('SOURCE6_UNKNOWN_TAG', this.here.start, 'Unknown action');
  }
  private effects(action: Source6Action, asset: string): S0Effect[] {
    this.node(); this.enter(); this.take('effects'); this.take('{');
    const debit = (): S0Effect => { this.effectTag('debit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Debit', account, asset, amount }; };
    const credit = (): S0Effect => { this.effectTag('credit'); const account = this.id(); const amount = this.uint(); this.take(';'); return { kind: 'Credit', account, asset, amount }; };
    const result: S0Effect[] = [debit(), credit()];
    if (action.kind === 'Transfer' && this.here.text === 'credit') result.push(credit());
    if (action.kind === 'Repay') {
      this.effectTag('set_obligation'); const id = this.id(); this.take('principal'); const principal = this.uint(S128);
      this.take('accrued'); const accrued = this.uint(S128); this.take('outstanding'); const outstanding = this.uint(S128);
      this.take('status');
      if (this.here.text !== 'outstanding' && this.here.text !== 'settled')
        fail('SOURCE6_SHAPE', this.here.start, 'Expected obligation status');
      const status = this.here.text === 'settled' ? 'Settled' : 'Outstanding'; this.index++; this.take(';');
      result.push({ kind: 'SetObligation', id, principal, accrued, outstanding, status });
    }
    this.effectTag('use_allowance'); const owner = this.id(); const amount = this.uint(); this.take(';');
    result.push({ kind: 'UseAllowance', owner, amount });
    this.effectTag('use_replay'); const key = this.string(); this.take(';'); result.push({ kind: 'UseReplay', key });
    this.effectTag('advance_head'); const predecessor = this.string(); const successor = this.string(); this.take(';');
    result.push({ kind: 'AdvanceHead', predecessor, successor }); this.take('}'); this.leave();
    return result;
  }
  parse(): Source6Ast {
    this.node(); this.take('profile', 'SOURCE6_VERSION');
    if (this.here.kind !== 'string') fail('SOURCE6_VERSION', this.here.start, 'Expected Source/6 profile string');
    const profile = this.string();
    if (profile !== MIL4_S0_SOURCE || this.tokens[this.index - 1].text !== '"moriarty-financial-agreement-source/6"')
      fail('SOURCE6_VERSION', this.tokens[this.index - 1].start, 'Unsupported source profile');
    this.take(';'); this.take('agreement'); const programId = this.id(); this.take('{'); this.enter();
    this.take('domain'); const domain = this.id(); this.take(';');
    this.take('settlement'); const asset = this.id(); this.take('scale'); const scale = this.uint(18n); this.take(';');
    this.take('selected'); const actionId = this.id(); this.take('source_hash'); const sourceHash = this.string();
    this.take('digest'); const policyDigest = this.string(); this.take(';');
    this.take('intent'); this.take('{'); this.enter(); this.node();
    this.take('signer'); const signer = this.id(); this.take('key'); const keyRef = this.string(); this.take(';');
    this.take('nonce'); const nonce = this.string(); this.take(';');
    this.take('pre_head'); const preHead = this.string(); this.take(';');
    this.take('valid'); const notBefore = this.uint(); this.take('..'); const notAfter = this.uint(); this.take(';');
    this.take('gross_cap'); const grossCap = this.uint(S128); this.take(';');
    this.take('fee_cap'); const feeCap = this.uint(S128); this.take(';');
    this.take('net_floor'); const netFloor = this.uint(S128); this.take(';');
    this.take('failure'); this.take('success_only'); this.take(';'); this.take('signed_action');
    const signedAction = this.action();
    for (const field of ['observations', 'disclosures', 'retained_effects', 'retained_duties']) {
      this.take(field); this.take('empty'); this.take(';');
    }
    this.take('delegation'); this.take('none'); this.take(';');
    this.take('recovery'); this.take('none'); this.take(';'); this.take('}'); this.leave();
    this.take('authenticated'); this.take('{'); this.enter(); this.node();
    this.take('head'); const head = this.string(); this.take(';');
    this.take('predecessor'); const predecessor = this.string(); this.take(';');
    this.take('round'); const round = this.uint(); this.take(';');
    const balances: { account: string; amount: string }[] = [];
    const balanceCount = signedAction.kind === 'Transfer' ? 3 : 2;
    for (let i = 0; i < balanceCount; i++) {
      this.take('balance'); balances.push({ account: this.id(), amount: this.uint() }); this.take(';');
    }
    this.take('allowance'); const owner = this.id(); this.take('remaining'); const remaining = this.uint();
    this.take('spent'); const spent = this.uint(); this.take(';');
    let obligation: Source6Obligation | undefined;
    if (signedAction.kind === 'Repay') {
      this.take('obligation'); const id = this.id(); this.take('{'); this.enter(); this.node();
      this.take('debtor'); const debtor = this.id(); this.take(';');
      this.take('creditor'); const creditor = this.id(); this.take(';');
      this.take('asset'); const obligationAsset = this.id(); this.take(';');
      this.take('principal'); const principal = this.uint(S128); this.take(';');
      this.take('accrued'); const accrued = this.uint(S128); this.take(';');
      this.take('outstanding'); const outstanding = this.uint(S128); this.take(';');
      this.take('status'); this.take('outstanding'); this.take(';'); this.take('}'); this.leave();
      obligation = { id, debtor, creditor, asset: obligationAsset, principal, accrued, outstanding, status: 'Outstanding' };
    }
    this.take('replay');
    if (this.here.text !== 'unused' && this.here.text !== 'consumed')
      fail('SOURCE6_SHAPE', this.here.start, 'Expected replay status');
    const replay = this.here.text as 'unused' | 'consumed'; this.index++; this.take(';');
    this.take('work_remaining'); const workRemaining = this.uint(); this.take(';');
    this.take('work_spent'); const workSpent = this.uint(); this.take(';'); this.take('}'); this.leave();
    this.take('submit'); const action = this.action(); const effects = this.effects(action, asset);
    this.take('post_head'); const postHead = this.string(); this.take(';'); this.take('}'); this.leave();
    if (this.tokens[this.index].kind !== 'eof') fail('SOURCE6_SHAPE', this.here.start, 'Trailing source');
    const expectedActionId = signedAction.kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst';
    if (actionId !== expectedActionId)
      fail('SOURCE6_PROFILE_UNSUPPORTED', this.here.start, 'Selected action is outside the S0 profile');
    const accounts = balances.map((row) => row.account);
    const expectedAccounts = signedAction.kind === 'Transfer'
      ? [signedAction.from, signedAction.to, signedAction.feeTo]
      : [signedAction.payer, obligation?.creditor];
    if (accounts.some((id, i) => id !== expectedAccounts[i]) || new Set(accounts).size !== accounts.length
        || owner !== signer || (signedAction.kind === 'Transfer' &&
          (signedAction.from !== signer || new Set([signedAction.from, signedAction.to, signedAction.feeTo]).size !== 3))
        || (signedAction.kind === 'Repay' && (!obligation || obligation.id !== signedAction.obligation
          || signedAction.payer !== signer || obligation.debtor !== signer || obligation.asset !== asset
          || obligation.creditor === signer)))
      fail('SOURCE6_CELL_SHAPE', this.here.start, 'Authenticated cells do not match action');
    if (BigInt(workRemaining) + BigInt(workSpent) > U128
        || BigInt(remaining) + BigInt(spent) > U128)
      fail('SOURCE6_RANGE', this.here.start, 'Counter total exceeds UInt128');
    if (obligation && BigInt(obligation.principal) + BigInt(obligation.accrued) !== BigInt(obligation.outstanding))
      fail('SOURCE6_RANGE', this.here.start, 'Obligation outstanding must equal principal plus accrued');
    return {
      profile: MIL4_S0_SOURCE, programId, domain, settlement: { asset, scale },
      selected: { actionId, sourceHash, policyDigest },
      intent: { signer, keyRef, nonce, preHead, notBefore, notAfter, grossCap, feeCap, netFloor,
        signedAction, failure: 'success_only', observations: 'empty', disclosures: 'empty',
        retainedEffects: 'empty', retainedDuties: 'empty', delegation: 'none', recovery: 'none' },
      authenticated: { head, predecessor, round, balances, allowance: { owner, remaining, spent },
        obligation, replay, workRemaining, workSpent },
      submitted: { action, effects, postHead },
    };
  }
}

/** Parse one exact Source/6 S0 document. Throws Source6Error on formation failure. */
export function parseSource6(source: string): Source6Ast { return new Parser(lexical(source)).parse(); }

export interface Source6Lowered {
  ast: Source6Ast; state: S0State; intent: S0Intent;
  submittedEffects: S0Effect[]; proposedPostHead: string;
}

/** Lower claims for local Core/5 preparation; no signed digest or external premise is manufactured. */
export function lowerSource6(ast: Source6Ast): Source6Lowered {
  const { authenticated: auth, intent: signed, submitted, settlement, selected } = ast;
  const replayKey = JSON.stringify([ast.domain, signed.signer, signed.nonce]);
  const state: S0State = {
    core: MIL4_S0_CORE, domain: ast.domain, asset: settlement.asset, head: auth.head,
    round: auth.round, workRemaining: auth.workRemaining, workSpent: auth.workSpent,
    balances: auth.balances.map((row) => ({ ...row })), allowances: [{ ...auth.allowance }],
    obligations: auth.obligation ? [{ ...auth.obligation }] : [],
    consumedReplay: auth.replay === 'consumed' ? [replayKey] : [],
  };
  const base = {
    version: MIL4_S0_INTENT, core: MIL4_S0_CORE, sourceProfile: MIL4_S0_SOURCE,
    programId: selected.actionId, sourceHash: selected.sourceHash, policyDigest: selected.policyDigest,
    keyRef: signed.keyRef, domain: ast.domain, asset: settlement.asset,
    signer: signed.signer, nonce: signed.nonce, preHead: signed.preHead,
    notBefore: signed.notBefore, notAfter: signed.notAfter,
    grossCap: signed.grossCap, feeCap: signed.feeCap, netFloor: signed.netFloor,
  };
  const action = signed.signedAction;
  const intent: S0Intent = action.kind === 'Transfer'
    ? { ...base, kind: 'Transfer', recipient: action.to, feeRecipient: action.feeTo,
        amount: action.value, fee: action.fee }
    : { ...base, kind: 'Repay', obligationId: action.obligation, amount: action.amount };
  // Source `use_replay` names a nonce. Core/5 compares its domain/signer/nonce tuple.
  const submittedEffects = submitted.effects.map((effect): S0Effect =>
    effect.kind === 'UseReplay'
      ? { kind: 'UseReplay', key: JSON.stringify([ast.domain, signed.signer, effect.key]) }
      : { ...effect });
  return { ast, state, intent, submittedEffects, proposedPostHead: submitted.postHead };
}

export function parseAndLowerSource6(source: string): Source6Lowered {
  return lowerSource6(parseSource6(source));
}

```

FILE experiments/moriarty-language/src/successor/mil4-s0-source-v6.ts
SHA256 e92decf9d3ef5c2ea5ca5b100aa99c8d52ceaedda7b81ccc04f06517ba0aae30
```
/** Provisional Source/6 to Core/5 local preparation. This module cannot admit a ledger stage. */
import { prepareMil4S0, type S0PreparedUnqualified, type S0Rejected } from './mil4-s0-core-v5.js';
import {
  parseAndLowerSource6, Source6Error,
  type Source6Ast, type Source6Lowered,
} from './financial-agreement-source-v6-frontend.js';

export type Source6S0Outcome =
  | { status: 'SourceRejected'; code: string; offset: number; publishedPost: null; publishedEffects: null }
  | { status: 'CoreRejected'; ast: Source6Ast; rejection: S0Rejected }
  | {
      status: 'PreparedUnqualified'; ast: Source6Ast; candidate: S0PreparedUnqualified;
      unverifiedBindings: readonly ['selected-program', 'asset-scale', 'authenticated-predecessor'];
    };

function sameAction(a: Source6Ast['intent']['signedAction'], b: Source6Ast['submitted']['action']): boolean {
  if (a.kind !== b.kind) return false;
  if (a.kind === 'Transfer' && b.kind === 'Transfer') {
    return a.from === b.from && a.to === b.to && a.feeTo === b.feeTo
      && a.value === b.value && a.fee === b.fee;
  }
  return a.kind === 'Repay' && b.kind === 'Repay'
    && a.obligation === b.obligation && a.payer === b.payer && a.amount === b.amount
    && a.conversion === b.conversion;
}

/** Parse and prepare one S0 stage without asserting source authentication or ledger acceptance. */
export function prepareSource6S0Unqualified(source: string): Source6S0Outcome {
  let lowered: Source6Lowered;
  try {
    lowered = parseAndLowerSource6(source);
  } catch (error) {
    if (!(error instanceof Source6Error)) throw error;
    return {
      status: 'SourceRejected', code: error.code, offset: error.offset,
      publishedPost: null, publishedEffects: null,
    };
  }
  const { ast, state, intent, submittedEffects, proposedPostHead } = lowered;
  if (!sameAction(ast.intent.signedAction, ast.submitted.action)) {
    return {
      status: 'CoreRejected', ast,
      rejection: {
        status: 'Rejected', judgment: 'intent', code: 'S0_INTENT_SCOPE',
        diagnosticWork: null, publishedPost: null, publishedEffects: null,
      },
    };
  }
  const result = prepareMil4S0(state, intent, submittedEffects, proposedPostHead);
  return result.status === 'Rejected'
    ? { status: 'CoreRejected', ast, rejection: result }
    : {
        status: 'PreparedUnqualified', ast, candidate: result,
        unverifiedBindings: ['selected-program', 'asset-scale', 'authenticated-predecessor'],
      };
}

```

FILE experiments/moriarty-language/src/successor/mil4-s0-core-v5.ts
SHA256 1a1e1e039d1b186850b8700235b7fe18eac6f5096562cd21db364eeb6e83e3f0
```
/** Provisional MIL/4 S0 preparation. This module never returns ledger admission. */

export const MIL4_S0_CORE = 'moriarty-core/5' as const;
export const MIL4_S0_INTENT = 'moriarty-intent/3' as const;
export const MIL4_S0_SOURCE = 'moriarty-financial-agreement-source/6' as const;
const U128 = (1n << 128n) - 1n;
const S128 = (1n << 127n) - 1n;
const IDENTIFIER = /^[A-Za-z][A-Za-z0-9._-]{0,63}$/;

export interface S0Balance { account: string; amount: string }
export interface S0Allowance { owner: string; remaining: string; spent: string }
export interface S0Obligation {
  id: string; debtor: string; creditor: string; asset: string;
  principal: string; accrued: string; outstanding: string;
  status: 'Outstanding' | 'Settled';
}
export interface S0State {
  core: typeof MIL4_S0_CORE; domain: string; asset: string;
  head: string; round: string; workRemaining: string; workSpent: string;
  balances: S0Balance[]; allowances: S0Allowance[];
  obligations: S0Obligation[]; consumedReplay: string[];
}
interface S0IntentBase {
  version: typeof MIL4_S0_INTENT; core: typeof MIL4_S0_CORE;
  sourceProfile: typeof MIL4_S0_SOURCE; programId: string;
  sourceHash: string; policyDigest: string; signedDigest?: string; keyRef: string;
  domain: string; asset: string; signer: string; nonce: string;
  preHead: string; notBefore: string; notAfter: string;
  grossCap: string; feeCap: string; netFloor: string;
}
export interface S0TransferIntent extends S0IntentBase {
  kind: 'Transfer'; recipient: string; feeRecipient: string;
  amount: string; fee: string;
}
export interface S0RepayIntent extends S0IntentBase {
  kind: 'Repay'; obligationId: string; amount: string;
}
export type S0Intent = S0TransferIntent | S0RepayIntent;

export type S0Effect =
  | { kind: 'Debit'; account: string; asset: string; amount: string }
  | { kind: 'Credit'; account: string; asset: string; amount: string }
  | { kind: 'SetObligation'; id: string; principal: string; accrued: string; outstanding: string; status: 'Outstanding' | 'Settled' }
  | { kind: 'UseAllowance'; owner: string; amount: string }
  | { kind: 'UseReplay'; key: string }
  | { kind: 'AdvanceHead'; predecessor: string; successor: string };

export type S0Judgment = 'stage' | 'intent' | 'effect' | 'authority' | 'history' | 'failure';
export interface S0Rejected {
  status: 'Rejected'; judgment: S0Judgment; code: string;
  diagnosticWork: null; publishedPost: null; publishedEffects: null;
}
export interface S0PreparedUnqualified {
  status: 'PreparedUnqualified'; core: typeof MIL4_S0_CORE;
  preHead: string; effects: S0Effect[]; candidatePost: S0State;
  requiredPremises: readonly ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'];
}
export type S0Result = S0Rejected | S0PreparedUnqualified;

function reject(judgment: S0Judgment, code: string): S0Rejected {
  return { status: 'Rejected', judgment, code, diagnosticWork: null, publishedPost: null, publishedEffects: null };
}
function id(value: unknown): value is string {
  return typeof value === 'string' && IDENTIFIER.test(value);
}
function opaque(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0 && value.length <= 1024
    && !/[\u0000-\u001f\u007f]/.test(value);
}
function uint(value: unknown, max: bigint = U128): bigint | null {
  if (typeof value !== 'string' || !/^(0|[1-9][0-9]*)$/.test(value)) return null;
  const parsed = BigInt(value);
  return parsed <= max ? parsed : null;
}
function distinct<T>(values: T[]): boolean { return new Set(values).size === values.length; }
function replayId(value: unknown): value is string {
  if (typeof value !== 'string') return false;
  try {
    const parts: unknown = JSON.parse(value);
    return Array.isArray(parts) && parts.length === 3
      && id(parts[0]) && id(parts[1]) && opaque(parts[2])
      && JSON.stringify(parts) === value;
  } catch { return false; }
}
function boundedAdd(a: bigint, b: bigint): bigint | null {
  const sum = a + b;
  return sum <= U128 ? sum : null;
}
function sameEffects(expected: S0Effect[], supplied: unknown): boolean {
  if (!Array.isArray(supplied) || supplied.length !== expected.length) return false;
  return expected.every((line, index) => {
    const got = supplied[index];
    if (got === null || typeof got !== 'object' || Array.isArray(got)) return false;
    const a = line as unknown as Record<string, unknown>;
    const b = got as Record<string, unknown>;
    return Object.keys(a).length === Object.keys(b).length
      && Object.keys(a).every((key) => JSON.stringify(a[key]) === JSON.stringify(b[key]));
  });
}

/**
 * Prepare a complete local candidate against supplied state.
 * The state, signature and ledger head are not authenticated by this function.
 */
export function prepareMil4S0(
  state: S0State,
  intent: S0Intent,
  submittedEffects: unknown,
  proposedPostHead: string,
): S0Result {
  if (state?.core !== MIL4_S0_CORE || intent?.core !== MIL4_S0_CORE
      || intent?.version !== MIL4_S0_INTENT || intent?.sourceProfile !== MIL4_S0_SOURCE
      || (intent?.kind === 'Transfer' && intent?.programId !== 'TransferLiteralFee')
      || (intent?.kind === 'Repay' && intent?.programId !== 'RepayAccrualFirst')
      || !id(state.domain) || !id(state.asset)
      || !opaque(state.head) || !id(intent.domain) || !id(intent.asset)
      || !Array.isArray(state.balances) || !Array.isArray(state.allowances)
      || !Array.isArray(state.obligations) || !Array.isArray(state.consumedReplay)
      || uint(state.round) === null || uint(state.workRemaining) === null
      || uint(state.workSpent) === null
      || boundedAdd(BigInt(state.workRemaining), BigInt(state.workSpent)) === null) {
    return reject('stage', 'S0_STAGE_UNSUPPORTED');
  }
  if (state.balances.some((v) => !v || typeof v !== 'object')
      || state.allowances.some((v) => !v || typeof v !== 'object')
      || state.obligations.some((v) => !v || typeof v !== 'object')
      || !distinct(state.balances.map((v) => v.account))
      || !distinct(state.allowances.map((v) => v.owner))
      || !distinct(state.obligations.map((v) => v.id))
      || !distinct(state.consumedReplay)
      || state.balances.some((v) => !id(v.account) || uint(v.amount) === null)
      || state.allowances.some((v) => !id(v.owner) || uint(v.remaining) === null || uint(v.spent) === null
        || boundedAdd(BigInt(v.remaining), BigInt(v.spent)) === null)
      || state.obligations.some((v) => !id(v.id) || !id(v.debtor) || !id(v.creditor) || !id(v.asset)
        || uint(v.principal, S128) === null || uint(v.accrued, S128) === null
        || uint(v.outstanding, S128) === null
        || !['Outstanding', 'Settled'].includes(v.status)
        || BigInt(v.principal) + BigInt(v.accrued) !== BigInt(v.outstanding)
        || (v.status === 'Settled') !== (v.outstanding === '0'))
      || state.consumedReplay.some((v) => !replayId(v))) return reject('stage', 'S0_STAGE_UNSUPPORTED');

  const notBefore = uint(intent.notBefore);
  const notAfter = uint(intent.notAfter);
  const grossCap = uint(intent.grossCap, S128);
  const feeCap = uint(intent.feeCap, S128);
  const netFloor = uint(intent.netFloor, S128);
  if (!id(intent.programId) || !opaque(intent.sourceHash) || !opaque(intent.policyDigest)
      || (intent.signedDigest !== undefined && !opaque(intent.signedDigest)) || !opaque(intent.keyRef)
      || !id(intent.signer) || !opaque(intent.nonce) || !opaque(intent.preHead)
      || intent.domain !== state.domain || intent.asset !== state.asset
      || notBefore === null || notAfter === null || notBefore > notAfter
      || grossCap === null || feeCap === null || netFloor === null
      || BigInt(state.round) < notBefore || BigInt(state.round) > notAfter) {
    return reject('intent', 'S0_INTENT_SCOPE');
  }

  const replayKey = JSON.stringify([state.domain, intent.signer, intent.nonce]);
  const balances = state.balances.map((v) => ({ ...v }));
  const allowances = state.allowances.map((v) => ({ ...v }));
  const obligations = state.obligations.map((v) => ({ ...v }));
  const ownerBalance = balances.find((v) => v.account === intent.signer);
  const ownerAllowance = allowances.find((v) => v.owner === intent.signer);
  let gross: bigint;
  let effects: S0Effect[];

  if (intent.kind === 'Transfer') {
    const v = uint(intent.amount, S128);
    const fee = uint(intent.fee, S128);
    if (!id(intent.recipient) || !id(intent.feeRecipient) || v === null || fee === null || v === 0n) {
      return reject('intent', 'S0_INTENT_SCOPE');
    }
    if (!distinct([intent.signer, intent.recipient, intent.feeRecipient])) {
      return reject('intent', 'S0_INTENT_ALIAS');
    }
    const sum = boundedAdd(v, fee);
    if (sum === null || sum > S128) return reject('effect', 'S0_EFFECT_RANGE');
    gross = sum;
    if (fee > feeCap || gross > grossCap || v < netFloor) return reject('intent', 'S0_INTENT_SCOPE');
    const recipient = balances.find((row) => row.account === intent.recipient);
    const feeRecipient = balances.find((row) => row.account === intent.feeRecipient);
    if (!ownerBalance || !recipient || !feeRecipient) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (BigInt(ownerBalance.amount) < gross
        || boundedAdd(BigInt(recipient.amount), v) === null
        || (fee > 0n && boundedAdd(BigInt(feeRecipient!.amount), fee) === null)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    ownerBalance.amount = (BigInt(ownerBalance.amount) - gross).toString();
    recipient.amount = (BigInt(recipient.amount) + v).toString();
    if (fee > 0n) feeRecipient!.amount = (BigInt(feeRecipient!.amount) + fee).toString();
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: gross.toString() },
      { kind: 'Credit', account: intent.recipient, asset: state.asset, amount: v.toString() },
      ...(fee > 0n ? [{ kind: 'Credit' as const, account: intent.feeRecipient, asset: state.asset, amount: fee.toString() }] : []),
    ];
  } else if (intent.kind === 'Repay') {
    const n = uint(intent.amount, S128);
    if (!id(intent.obligationId) || n === null || n === 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const obligation = obligations.find((row) => row.id === intent.obligationId);
    if (!obligation || obligation.asset !== state.asset || obligation.debtor !== intent.signer
        || obligation.status !== 'Outstanding') return reject('stage', 'S0_STAGE_UNSUPPORTED');
    const p = BigInt(obligation.principal);
    const a = BigInt(obligation.accrued);
    if (n > BigInt(obligation.outstanding)) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    gross = n;
    if (gross > grossCap || feeCap !== 0n || netFloor !== 0n) return reject('intent', 'S0_INTENT_SCOPE');
    const creditor = balances.find((row) => row.account === obligation.creditor);
    if (!ownerBalance || !creditor) return reject('stage', 'S0_STAGE_UNSUPPORTED');
    if (obligation.creditor === intent.signer
        || BigInt(ownerBalance.amount) < n || boundedAdd(BigInt(creditor.amount), n) === null) {
      return reject('effect', 'S0_EFFECT_RANGE');
    }
    const da = n < a ? n : a;
    const dp = n - da;
    const afterP = p - dp;
    const afterA = a - da;
    const afterOutstanding = afterP + afterA;
    ownerBalance.amount = (BigInt(ownerBalance.amount) - n).toString();
    creditor.amount = (BigInt(creditor.amount) + n).toString();
    obligation.principal = afterP.toString();
    obligation.accrued = afterA.toString();
    obligation.outstanding = afterOutstanding.toString();
    obligation.status = afterOutstanding === 0n ? 'Settled' : 'Outstanding';
    effects = [
      { kind: 'Debit', account: intent.signer, asset: state.asset, amount: n.toString() },
      { kind: 'Credit', account: obligation.creditor, asset: state.asset, amount: n.toString() },
      { kind: 'SetObligation', id: obligation.id, principal: obligation.principal,
        accrued: obligation.accrued, outstanding: obligation.outstanding, status: obligation.status },
    ];
  } else return reject('stage', 'S0_STAGE_UNSUPPORTED');

  effects.push(
    { kind: 'UseAllowance', owner: intent.signer, amount: gross.toString() },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: state.head, successor: proposedPostHead },
  );
  if (!sameEffects(effects, submittedEffects)) {
    return reject('effect', 'S0_EFFECT_MISMATCH');
  }
  if (!ownerAllowance || BigInt(ownerAllowance.remaining) < gross
      || boundedAdd(BigInt(ownerAllowance.spent), gross) === null
      || BigInt(state.workRemaining) < 1n || boundedAdd(BigInt(state.workSpent), 1n) === null) {
    return reject('authority', 'S0_AUTH_SCOPE');
  }
  ownerAllowance.remaining = (BigInt(ownerAllowance.remaining) - gross).toString();
  ownerAllowance.spent = (BigInt(ownerAllowance.spent) + gross).toString();
  if (intent.preHead !== state.head) return reject('history', 'S0_HISTORY_STALE');
  if (state.consumedReplay.includes(replayKey)) return reject('history', 'S0_HISTORY_REPLAY');
  if (!opaque(proposedPostHead) || proposedPostHead === state.head) {
    return reject('history', 'S0_HISTORY_STALE');
  }

  return {
    status: 'PreparedUnqualified', core: MIL4_S0_CORE, preHead: state.head, effects,
    candidatePost: {
      ...state, balances, allowances, obligations,
      consumedReplay: [...state.consumedReplay, replayKey],
      head: proposedPostHead,
      workRemaining: (BigInt(state.workRemaining) - 1n).toString(),
      workSpent: (BigInt(state.workSpent) + 1n).toString(),
    },
    requiredPremises: ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume'],
  };
}

```

FILE experiments/moriarty-language/formal/k/mil4/s0.k
SHA256 4811c982d71b81f7579329322424b8519c16a742f6f41464352eba5d37a5df11
```
// Provisional Source/6 -> Core/5 S0 stage. External verification is abstract.
module MIL4-S0-SYNTAX
  imports INT-SYNTAX
  imports STRING-SYNTAX
  // Opaque head identity. A trusted premise must authenticate the submitted successor.
  syntax Head ::= head(String) [symbol(head)]
  syntax ReplayKey ::= replayKey(String, String, String) [symbol(replayKey)]
  syntax ReplaySet ::= noReplays() [symbol(noReplays)] | used(ReplayKey, ReplaySet) [symbol(used)]
  syntax Balance ::= noBalance() [symbol(noBalance)] | balance(String, String, Int) [symbol(balance)]
  syntax Allowance ::= allowance(String, String, Int, Int) [symbol(allowance)]
  syntax Obligation ::= noObligation() [symbol(noObligation)]
                      | obligation(String, String, String, String, Int, Int, Int, String) [symbol(obligation)]
  syntax State ::= state(Balance, Balance, Balance, Allowance, Obligation, Head, ReplaySet, Int, Int) [symbol(state)]
  syntax Action ::= transfer(Int, Int) [symbol(transfer)] | repay(String, String, Int, Int, Int, String) [symbol(repay)]
  // The intent digest is an opaque identifier; its exact signed bytes remain W-D1/W-D2.
  syntax Intent ::= intent(String, String, String, String, String, String, Head, Int, Int,
                           String, String, Int, Int, Int, Action, String) [symbol(intent)]
  syntax Fill ::= fill(String, String, String, String, Head, String, String,
                       Int, Int, Int, Action, Head) [symbol(fill)]
  syntax Effect ::= debit(String, String, Int) [symbol(debit)]
                  | credit(String, String, Int) [symbol(credit)]
                  | setObligation(String, Int, Int, Int, String) [symbol(setObligation)]
                  | useAllowance(String, String, Int) [symbol(useAllowance)]
                  | useReplay(ReplayKey) [symbol(useReplay)]
                  | advanceHead(Head, Head) [symbol(advanceHead)]
  syntax Effects ::= noEffects() [symbol(noEffects)] | effect(Effect, Effects) [symbol(effect)]
  syntax Phase ::= terminalSuccess() [symbol(terminalSuccess)] | requestedFailure(String) [symbol(requestedFailure)]
  syntax Duty ::= noDuty() [symbol(noDuty)] | retainedDuty(String) [symbol(retainedDuty)]
  syntax Premise ::= unavailable() [symbol(unavailable)]
                   | authenticated(String, String, String, Head, State, Int, Head) [symbol(authenticated)]
  syntax Family ::= ammCP1() [symbol(ammCP1)] | loanFixed1() [symbol(loanFixed1)]
                  | cdp1() [symbol(cdp1)] | optCapped1() [symbol(optCapped1)]
                  | obs1() [symbol(obs1)] | gov1() [symbol(gov1)]
                  | bridgePair1() [symbol(bridgePair1)] | vault1() [symbol(vault1)]
  syntax Request ::= submit(Intent, Fill, State, Effects, Phase, Duty, Int) [symbol(submit)]
                   | submitFamily(Family) [symbol(submitFamily)]
                   | unsupported(String) [symbol(unsupported)]
  syntax Result ::= pending() [symbol(pending)]
                  | rejected(String, String, Int) [symbol(rejected)]
                  | accepted(State, Effects, State, Phase, Duty, Int) [symbol(accepted)]
endmodule

module MIL4-S0
  imports MIL4-S0-SYNTAX
  imports INT
  imports BOOL
  imports STRING
  imports K-EQUAL
  configuration <s0> <k> $PGM:Request </k> <out> pending() </out>
                     <external> unavailable() </external> </s0>

  syntax Int ::= maxU() [function] | maxNominal() [function]
  rule maxU() => 340282366920938463463374607431768211455
  rule maxNominal() => 170141183460469231731687303715884105727
  syntax Bool ::= uint(Int) [function] | nominal(Int) [function]
  rule uint(N) => N >=Int 0 andBool N <=Int maxU()
  rule nominal(N) => N >=Int 0 andBool N <=Int maxNominal()
  syntax Bool ::= hasReplay(ReplayKey, ReplaySet) [function, total]
  rule hasReplay(_, noReplays()) => false
  rule hasReplay(K, used(K, _)) => true
  rule hasReplay(K, used(J, R)) => hasReplay(K, R) requires notBool K ==K J
  syntax String ::= debtStatus(Int) [function]
  rule debtStatus(0) => "Settled"
  rule debtStatus(N) => "Outstanding" requires N >Int 0
  syntax Int ::= accruedPaid(Int, Int) [function]
  rule accruedPaid(N, A) => minInt(N, A)

  syntax Bool ::= goodState(State) [function, total]
  rule goodState(state(balance(O,A,OB),balance(R,A,RB),balance(F,A,FB),
                       allowance(O,A,AR,AS),_,_,_,WR,WS))
    => O =/=String R andBool O =/=String F andBool R =/=String F
       andBool uint(OB) andBool uint(RB) andBool uint(FB)
       andBool uint(AR) andBool uint(AS) andBool uint(WR) andBool uint(WS)
       andBool AR +Int AS <=Int maxU() andBool WR +Int WS <=Int maxU()
  rule goodState(state(balance(O,A,OB),balance(R,A,RB),noBalance(),
                       allowance(_,A,AR,AS),_,_,_,WR,WS))
    => O =/=String R andBool uint(OB) andBool uint(RB)
       andBool uint(AR) andBool uint(AS) andBool uint(WR) andBool uint(WS)
       andBool AR +Int AS <=Int maxU() andBool WR +Int WS <=Int maxU()
  rule goodState(_) => false [owise]
  syntax Bool ::= goodDebt(Obligation) [function, total]
  rule goodDebt(noObligation()) => true
  rule goodDebt(obligation(_,D,C,_,P,A,O,S))
    => D =/=String C andBool nominal(P) andBool nominal(A) andBool nominal(O)
       andBool P +Int A ==Int O andBool P +Int A <=Int maxU()
       andBool ((O >Int 0 andBool S ==String "Outstanding")
                orBool (O ==Int 0 andBool S ==String "Settled"))
  rule goodDebt(_) => false [owise]
  syntax Bool ::= matchingScope(Intent, Fill) [function, total]
  rule matchingScope(intent(_,_,D,S,A,N,H,_,_,R,F,G,FC,NF,transfer(V,Fee),_),
                     fill(D,S,A,N,H,R,F,G,FC,NF,transfer(V,Fee),_)) => true
  // Repay has no recipient or fee endpoint in Source/6; empty strings are absent fields.
  rule matchingScope(intent(_,_,D,S,A,N,H,_,_,"","",G,FC,NF,repay(ID,Payer,V,M,Scale,Rnd),_),
                     fill(D,S,A,N,H,"","",G,FC,NF,repay(ID,Payer,V,M,Scale,Rnd),_)) => true
  rule matchingScope(_,_) => false [owise]
  syntax Bool ::= actionIntentOK(Action, Int, Int, Int) [function, total]
  rule actionIntentOK(transfer(V,F),G,FC,NF)
    => V >Int 0 andBool nominal(V) andBool nominal(F) andBool nominal(G)
       andBool nominal(FC) andBool nominal(NF) andBool F <=Int FC
       andBool V +Int F <=Int G andBool V >=Int NF
  rule actionIntentOK(repay(_,_,N,1,0,"none"),G,FC,NF)
    => N >Int 0 andBool nominal(N) andBool nominal(G) andBool FC ==Int 0
       andBool NF ==Int 0 andBool N <=Int G
  rule actionIntentOK(_,_,_,_) => false [owise]

  syntax Effects ::= transferEffects(Intent, Int, Int, Head) [function]
                   | repayEffects(Intent, Obligation, Int, Head) [function]
  rule transferEffects(intent(_,_,D,S,A,N,H,_,_,R,_,_,_,_,_,_),V,0,Post)
    => effect(debit(S,A,V),effect(credit(R,A,V),
       effect(useAllowance(S,A,V),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects())))))
  rule transferEffects(intent(_,_,D,S,A,N,H,_,_,R,F,_,_,_,_,_),V,FEE,Post)
    => effect(debit(S,A,V +Int FEE),effect(credit(R,A,V),effect(credit(F,A,FEE),
       effect(useAllowance(S,A,V +Int FEE),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects()))))))
    requires FEE >Int 0
  rule repayEffects(intent(_,_,D,S,A,N,H,_,_,_,_,_,_,_,repay(_,Payer,_,_,_,_),_),
                    obligation(ID,_,C,_,P,AC,_,_),PAY,Post)
    => effect(debit(Payer,A,PAY),effect(credit(C,A,PAY),
       effect(setObligation(ID,P -Int (PAY -Int accruedPaid(PAY,AC)),
                            AC -Int accruedPaid(PAY,AC),P +Int AC -Int PAY,
                            debtStatus(P +Int AC -Int PAY)),
       effect(useAllowance(S,A,PAY),effect(useReplay(replayKey(D,S,N)),
       effect(advanceHead(H,Post),noEffects()))))))

  syntax KItem ::= s0Stage(Request) | s0Intent(Request) | s0Effect(Request)
                 | s0Authority(Request) | s0History(Request) | s0Failure(Request)
                 | s0Commit(Request) | s0Stop(String, String, Int)
  rule <k> unsupported(_) => s0Stop("stage","S0_STAGE_UNSUPPORTED",1) </k>
  rule <k> submitFamily(_) => s0Stop("stage","FAMILY_STAGE_ADAPTER_ABSENT",1) </k>
  rule <k> submit(I,F,S,E,P,D,Round) => s0Stage(submit(I,F,S,E,P,D,Round)) </k>
  rule <k> s0Stop(J,C,W) => .K </k> <out> pending() => rejected(J,C,W) </out>

  rule <k> s0Stage(submit(I,_,S,_,_,_,_) #as R) => s0Intent(R) </k>
    requires goodState(S) andBool goodDebt(debtOf(S)) andBool selected(I)
             andBool stageShape(I,S)
  rule <k> s0Stage(_) => s0Stop("stage","S0_STAGE_UNSUPPORTED",1) </k> [owise]
  syntax Obligation ::= debtOf(State) [function]
  rule debtOf(state(_,_,_,_,O,_,_,_,_)) => O
  syntax Bool ::= selected(Intent) [function, total]
  rule selected(intent("Source/6","Core/5",_,_,_,_,_,_,_,_,_,_,_,_,transfer(_,_),_)) => true
  rule selected(intent("Source/6","Core/5",_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,_,_,_,_),_)) => true
  rule selected(_) => false [owise]
  syntax Bool ::= stageShape(Intent,State) [function, total]
  rule stageShape(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,transfer(_,_),_),
                  state(_,_,balance(_,_,_),_,noObligation(),_,_,_,_)) => true
  rule stageShape(intent(_,_,_,_,A,_,_,_,_,_,_,_,_,_,repay(ID,Payer,_,_,_,_),_),
                  state(balance(Payer,A,_),balance(C,A,_),noBalance(),_,
                        obligation(ID,_,C,A,_,_,_,_),_,_,_,_))
    => Payer =/=String C
  rule stageShape(_,_) => false [owise]

  rule <k> s0Intent(submit(I,F,_,_,_,_,Round) #as R) => s0Effect(R) </k>
    requires matchingScope(I,F) andBool inRound(I,Round) andBool distinct(I)
             andBool intentAmountOK(I)
  rule <k> s0Intent(submit(I,F,_,_,_,_,_)) => s0Stop("intent","S0_INTENT_SCOPE",1) </k>
    requires notBool matchingScope(I,F)
  rule <k> s0Intent(submit(I,F,_,_,_,_,_)) => s0Stop("intent","S0_INTENT_ALIAS",1) </k>
    requires matchingScope(I,F) andBool notBool distinct(I)
  rule <k> s0Intent(_) => s0Stop("intent","S0_INTENT_SCOPE",1) </k> [owise]
  syntax Bool ::= inRound(Intent,Int) [function, total] | distinct(Intent) [function, total]
                | intentAmountOK(Intent) [function, total]
  rule inRound(intent(_,_,_,_,_,_,_,Lo,Hi,_,_,_,_,_,_,_),R)
    => uint(Lo) andBool uint(Hi) andBool Lo <=Int R andBool R <=Int Hi
  rule distinct(intent(_,_,_,S,_,_,_,_,_,R,F,_,_,_,transfer(_,_),_))
    => S =/=String R andBool S =/=String F andBool R =/=String F
  rule distinct(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,_,_,_,_),_)) => true
  rule intentAmountOK(intent(_,_,_,_,_,_,_,_,_,_,_,G,FC,NF,A,_))
    => actionIntentOK(A,G,FC,NF)

  rule <k> s0Effect(submit(I,F,S,E,_,_,_) #as R) => s0Authority(R) </k>
    requires effectRange(I,S) andBool E ==K expected(I,F,S)
  rule <k> s0Effect(submit(I,_,S,_,_,_,_)) => s0Stop("effect","S0_EFFECT_RANGE",1) </k>
    requires notBool effectRange(I,S)
  rule <k> s0Effect(_) => s0Stop("effect","S0_EFFECT_MISMATCH",1) </k> [owise]
  syntax Effects ::= expected(Intent,Fill,State) [function]
  rule expected(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,transfer(V,F),_) #as I,Fill,_)
    => transferEffects(I,V,F,postOf(Fill))
  rule expected(intent(_,_,_,_,_,_,_,_,_,_,_,_,_,_,repay(_,_,N,_,_,_),_) #as I,Fill,S)
    => repayEffects(I,debtOf(S),N,postOf(Fill))
  syntax Head ::= postOf(Fill) [function]
  rule postOf(fill(_,_,_,_,_,_,_,_,_,_,_,Post)) => Post
  syntax Bool ::= effectRange(Intent,State) [function, total]
  rule effectRange(intent(_,_,_,S,A,_,_,_,_,R,F,_,_,_,transfer(V,FEE),_),
                   state(balance(S,A,OB),balance(R,A,RB),balance(F,A,FB),_,_,_,_,_,_))
    => V +Int FEE <=Int maxU() andBool OB >=Int V +Int FEE
       andBool RB +Int V <=Int maxU() andBool FB +Int FEE <=Int maxU()
  rule effectRange(intent(_,_,_,_,A,_,_,_,_,_,_,_,_,_,repay(ID,Payer,N,1,0,"none"),_),
                   state(balance(Payer,A,OB),balance(C,A,CB),noBalance(),_,
                         obligation(ID,_,C,A,P,AC,O,"Outstanding"),_,_,_,_))
    => N <=Int O andBool OB >=Int N andBool CB +Int N <=Int maxU()
       andBool P +Int AC ==Int O
  rule effectRange(_,_) => false [owise]

  rule <k> s0Authority(submit(I,F,S,_,_,_,Round) #as R) => s0History(R) </k>
    <external> X </external>
    requires authorityOK(I,S) andBool premiseOK(X,I,S,F,Round)
  rule <k> s0Authority(_) => s0Stop("authority","S0_AUTH_SCOPE",1) </k> [owise]
  syntax Bool ::= authorityOK(Intent,State) [function, total] | premiseOK(Premise,Intent,State,Fill,Int) [function, total]
  rule authorityOK(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,transfer(V,F),_),
                   state(balance(S,A,_),_,_,allowance(S,A,AR,AS),_,_,_,WR,WS))
    => AR >=Int V +Int F andBool AS +Int V +Int F <=Int maxU()
       andBool WR >=Int 1 andBool WS +Int 1 <=Int maxU()
  rule authorityOK(intent(_,_,_,S,A,_,_,_,_,_,_,_,_,_,repay(_,S,N,_,_,_),_),
                   state(balance(S,A,_),_,_,allowance(S,A,AR,AS),
                         obligation(_,S,_,A,_,_,_,_),_,_,WR,WS))
    => AR >=Int N andBool AS +Int N <=Int maxU()
       andBool WR >=Int 1 andBool WS +Int 1 <=Int maxU()
  rule authorityOK(_,_) => false [owise]
  rule premiseOK(authenticated(DG,D,S,H,Pre,Round,_),
                 intent(_,_,D,S,_,_,H,_,_,_,_,_,_,_,_,DG),Pre,_,Round)
    => true
  rule premiseOK(_,_,_,_,_) => false [owise]

  rule <k> s0History(submit(I,F,S,_,_,_,_) #as R) => s0Failure(R) </k>
    <external> X </external>
    requires historyOK(I,S) andBool successorOK(X,I,F)
  rule <k> s0History(submit(I,_,S,_,_,_,_)) => s0Stop("history","S0_HISTORY_STALE",1) </k>
    requires notBool headOK(I,S)
  rule <k> s0History(submit(I,_,S,_,_,_,_)) => s0Stop("history","S0_HISTORY_REPLAY",1) </k>
    requires headOK(I,S) andBool notBool historyOK(I,S)
  rule <k> s0History(_) => s0Stop("history","S0_HISTORY_SUCCESSOR",1) </k> [owise]
  syntax Bool ::= headOK(Intent,State) [function, total] | historyOK(Intent,State) [function, total]
                 | successorOK(Premise,Intent,Fill) [function, total]
  rule headOK(intent(_,_,_,_,_,_,H,_,_,_,_,_,_,_,_,_),state(_,_,_,_,_,H,_,_,_)) => true
  rule headOK(_,_) => false [owise]
  rule historyOK(intent(_,_,D,S,_,N,_,_,_,_,_,_,_,_,_,_) #as I,
                 state(_,_,_,_,_,_,Rs,_,_) #as St)
    => headOK(I,St) andBool notBool hasReplay(replayKey(D,S,N),Rs)
  rule successorOK(authenticated(_,_,_,_,_,_,Post),I,Fill)
    => Post ==K postOf(Fill) andBool notBool Post ==K signedHead(I)
  rule successorOK(_,_,_) => false [owise]
  syntax Head ::= signedHead(Intent) [function]
  rule signedHead(intent(_,_,_,_,_,_,H,_,_,_,_,_,_,_,_,_)) => H

  rule <k> s0Failure(submit(_,_,_,_,terminalSuccess(),noDuty(),_) #as R) => s0Commit(R) </k>
  rule <k> s0Failure(_) => s0Stop("failure","S0_FAILURE_UNSUPPORTED",1) </k> [owise]

  rule <k> s0Commit(submit(intent(_,_,D,S,A,N,H,_,_,R,F,_,_,_,transfer(V,FEE),_),Fill,
                           state(balance(S,A,OB),balance(R,A,RB),balance(F,A,FB),
                                     allowance(S,A,AR,AS),O,H,Rs,WR,WS) #as Pre,E,terminalSuccess(),noDuty(),_))
       => .K </k>
       <out> pending() => accepted(Pre,E,
         state(balance(S,A,OB -Int V -Int FEE),balance(R,A,RB +Int V),
               balance(F,A,FB +Int FEE),allowance(S,A,AR -Int V -Int FEE,AS +Int V +Int FEE),
               O,postOf(Fill),used(replayKey(D,S,N),Rs),WR -Int 1,WS +Int 1),
         terminalSuccess(),noDuty(),WR -Int 1) </out>
  rule <k> s0Commit(submit(intent(_,_,D,S,A,N,H,_,_,_,_,_,_,_,repay(ID,S,PAY,1,0,"none"),_),Fill,
                           state(balance(S,A,OB),balance(C,A,CB),noBalance(),
                                     allowance(S,A,AR,AS),obligation(ID,S,C,A,P,AC,O,"Outstanding"),
                                     H,Rs,WR,WS) #as Pre,E,terminalSuccess(),noDuty(),_))
       => .K </k>
       <out> pending() => accepted(Pre,E,
         state(balance(S,A,OB -Int PAY),balance(C,A,CB +Int PAY),noBalance(),
               allowance(S,A,AR -Int PAY,AS +Int PAY),
               obligation(ID,S,C,A,P -Int (PAY -Int accruedPaid(PAY,AC)),
                          AC -Int accruedPaid(PAY,AC),O -Int PAY,debtStatus(O -Int PAY)),
               postOf(Fill),used(replayKey(D,S,N),Rs),WR -Int 1,WS +Int 1),
         terminalSuccess(),noDuty(),WR -Int 1) </out>
endmodule

```

FILE experiments/moriarty-language/formal/quint/mil4/s0.qnt
SHA256 237ade9027ce5b7861b5e1998ffcd0020a7a4b7d524d010376a8d17ea554fa4f
```
// Provisional MIL/4 S0 transition model. See README.md for its evidence limits.
module s0 {
  type ReplayKey = (str, str, str)
  type Program = TransferProgram | RepayProgram
  type DebtStatus = Outstanding | Settled
  type Judgment = Accepted | Stage | Intent | Effect | Authority | History | Failure
  type Decision = { accepted: bool, judgment: Judgment, code: str,
                    diagnosticWork: int }

  type SignedIntent = {
    sourceVersion: str,
    coreVersion: str,
    profile: str,
    program: Program,
    domain: str,
    asset: str,
    digest: str,
    policyDigest: str,
    evidenceChoice: str,
    signer: str,
    payer: str,
    recipient: str,
    feeRecipient: str,
    obligationId: str,
    debtor: str,
    creditor: str,
    nonce: str,
    preHead: int,
    validFrom: int,
    validThrough: int,
    grossCap: int,
    feeCap: int,
    netFloor: int,
    actionAmount: int,
    actionFee: int,
    conversionMantissa: int,
    conversionScale: int,
    roundingNone: bool,
    terminalOnly: bool,
  }

  type Obligation = {
    debtor: str,
    creditor: str,
    asset: str,
    principal: int,
    accrued: int,
    outstanding: int,
    status: DebtStatus,
  }

  type EffectLine =
    | Debit({ account: str, amount: int })
    | Credit({ account: str, amount: int })
    | SetObligation({ id: str, value: Obligation })
    | UseAllowance({ owner: str, amount: int })
    | UseReplay(ReplayKey)
    | AdvanceHead({ before: int, after: int })

  // These are environmental premises, not checks implemented by Quint. The
  // selected external interfaces must establish their truth before admission.
  const signatureVerified: Set[str]
  const authenticatedSnapshots: Set[int]
  const nativeQualified: Set[str]
  const ledgerAtomicReady: bool
  const executingDomain: str
  const settlementAsset: str
  const initialWorkBudget: int

  pure val UINT128_MAX = 340282366920938463463374607431768211455
  pure val NOMINAL_MAX = 170141183460469231731687303715884105727

  var signed: str -> SignedIntent
  var balances: str -> int
  var allowanceRemaining: str -> int
  var allowanceSpent: str -> int
  var workRemaining: int
  var workSpent: int
  var obligations: str -> Obligation
  var currentHead: int
  var round: int
  var consumed: Set[ReplayKey]
  var lastEffects: List[EffectLine]
  var committedCount: int

  pure def replayKey(i: SignedIntent): ReplayKey = (i.domain, i.signer, i.nonce)
  pure def ok: Decision = { accepted: true, judgment: Accepted, code: "",
                            diagnosticWork: 0 }
  pure def reject(j: Judgment, c: str): Decision =
    { accepted: false, judgment: j, code: c, diagnosticWork: 0 }
  pure def withinUInt(n: int): bool = n >= 0 and n <= UINT128_MAX
  pure def withinNominal(n: int): bool = n >= 0 and n <= NOMINAL_MAX
  pure def minInt(a: int, b: int): int = if (a < b) a else b

  pure def transferLines(i: SignedIntent, v: int, f: int): List[EffectLine] = {
    val gross = v + f
    val money = if (f == 0)
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }))
    else
      List(Debit({ account: i.payer, amount: gross }),
           Credit({ account: i.recipient, amount: v }),
           Credit({ account: i.feeRecipient, amount: f }))
    money.concat(List(
      UseAllowance({ owner: i.payer, amount: gross }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: i.preHead + 1 })
    ))
  }

  pure def repaid(o: Obligation, n: int): Obligation = {
    val da = minInt(n, o.accrued)
    val dp = n - da
    val p = o.principal - dp
    val a = o.accrued - da
    { ...o, principal: p, accrued: a, outstanding: p + a,
      status: if (p + a == 0) Settled else Outstanding }
  }

  pure def repayLines(i: SignedIntent, o: Obligation, n: int): List[EffectLine] =
    List(
      Debit({ account: i.payer, amount: n }),
      Credit({ account: o.creditor, amount: n }),
      SetObligation({ id: i.obligationId, value: repaid(o, n) }),
      UseAllowance({ owner: i.payer, amount: n }),
      UseReplay(replayKey(i)),
      AdvanceHead({ before: i.preHead, after: i.preHead + 1 })
    )

  // A rejected proposal is inspected through this value; it is never a stage.
  // Code spelling and within-judgment precedence are provisional W-D3 leaves.
  def beforeEffect(id: str, supplied: SignedIntent,
                     claimedHead: int, lines: List[EffectLine],
                     expected: List[EffectLine], amount: int,
                     fee: int, recipient: str, isRepay: bool): Decision = {
    if (not(signed.keys().contains(id)) or
        supplied.sourceVersion != "Source/6" or
        supplied.coreVersion != "Core/5" or
        supplied.profile != "S0" or
        supplied.domain != executingDomain or
        supplied.asset != settlementAsset)
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else if (not(signatureVerified.contains(signed.get(id).digest)) or
             not(nativeQualified.contains(signed.get(id).digest)) or
             not(ledgerAtomicReady))
      reject(Stage, "S0_EXTERNAL_PREMISE_UNAVAILABLE")
    else if (signed.get(id) != supplied or
             amount != supplied.actionAmount or
             fee != supplied.actionFee or
             not(withinNominal(supplied.actionAmount)) or
             not(withinNominal(supplied.actionFee)) or
             not(withinNominal(supplied.grossCap)) or
             not(withinNominal(supplied.feeCap)) or
             not(withinNominal(supplied.netFloor)) or
             not(withinUInt(supplied.validFrom)) or
             not(withinUInt(supplied.validThrough)) or
             round < supplied.validFrom or round > supplied.validThrough or
             supplied.validFrom > supplied.validThrough or
             claimedHead != supplied.preHead or
             (not(isRepay) and
              (fee > supplied.feeCap or
               amount + fee > supplied.grossCap or
               amount < supplied.netFloor)) or
             (isRepay and (amount > supplied.grossCap or
                           supplied.feeCap != 0 or
                           supplied.netFloor != 0)))
      reject(Intent, "S0_INTENT_SCOPE")
    else if (supplied.payer == recipient or
             (not(isRepay) and
              (supplied.payer == supplied.feeRecipient or
               supplied.recipient == supplied.feeRecipient)))
      reject(Intent, "S0_INTENT_ALIAS")
    else if (lines != expected)
      reject(Effect, "S0_EFFECT_MISMATCH")
    else if (amount <= 0 or not(withinNominal(amount)) or
             not(withinNominal(amount + fee)) or fee < 0)
      reject(Effect, "S0_EFFECT_RANGE")
    else ok
  }

  def afterEffect(supplied: SignedIntent, claimedHead: int,
                  amount: int, fee: int): Decision = {
    if (supplied.signer != supplied.payer or
             allowanceRemaining.get(supplied.payer) < amount + fee or
             allowanceSpent.get(supplied.payer) + amount + fee > UINT128_MAX or
             workRemaining < 1 or workSpent + 1 > UINT128_MAX)
      reject(Authority, "S0_AUTH_SCOPE")
    else if (claimedHead != currentHead or not(authenticatedSnapshots.contains(currentHead)))
      reject(History, "S0_HISTORY_STALE")
    else if (consumed.contains(replayKey(supplied)))
      reject(History, "S0_HISTORY_REPLAY")
    else if (not(supplied.terminalOnly))
      reject(Failure, "S0_FAILURE_UNSUPPORTED")
    else ok
  }

  def transferObservation(id: str, supplied: SignedIntent,
                          claimedHead: int, v: int, f: int,
                          lines: List[EffectLine]): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != TransferProgram or
          not(balances.keys().contains(bound.payer)) or
          not(balances.keys().contains(bound.recipient)) or
          not(balances.keys().contains(bound.feeRecipient)) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)))
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val base = beforeEffect(id, supplied, claimedHead, lines,
                                transferLines(supplied, v, f), v, f,
                                supplied.recipient, false)
      if (not(base.accepted)) base
      else if (balances.get(supplied.payer) < v + f or
               balances.get(supplied.recipient) + v > UINT128_MAX or
               balances.get(supplied.feeRecipient) + f > UINT128_MAX or
               not(withinUInt(balances.get(supplied.payer))) or
               not(withinUInt(balances.get(supplied.recipient))) or
               not(withinUInt(balances.get(supplied.feeRecipient))))
        reject(Effect, "S0_EFFECT_RANGE")
      else afterEffect(supplied, claimedHead, v, f)
      }
    }
  }

  def repayObservation(id: str, supplied: SignedIntent,
                       claimedHead: int, n: int,
                       lines: List[EffectLine]): Decision = {
    if (not(signed.keys().contains(id)))
      reject(Stage, "S0_STAGE_UNSUPPORTED")
    else {
      val bound = signed.get(id)
      if (bound.program != RepayProgram or
          not(obligations.keys().contains(bound.obligationId)) or
          not(balances.keys().contains(bound.payer)) or
          not(allowanceRemaining.keys().contains(bound.payer)) or
          not(allowanceSpent.keys().contains(bound.payer)))
        reject(Stage, "S0_STAGE_UNSUPPORTED")
      else {
        val o = obligations.get(bound.obligationId)
        if (not(balances.keys().contains(o.creditor)) or
            not(withinNominal(o.principal)) or
            not(withinNominal(o.accrued)) or
            not(withinNominal(o.outstanding)) or
            o.outstanding != o.principal + o.accrued or
            o.status != Outstanding or o.asset != settlementAsset)
          reject(Stage, "S0_STAGE_UNSUPPORTED")
        else {
        val base = beforeEffect(id, supplied, claimedHead, lines,
                                  repayLines(supplied, o, n), n, 0,
                                  o.creditor, true)
        if (not(base.accepted)) base
        else if (n > o.outstanding or
                 balances.get(supplied.payer) < n or
                 balances.get(o.creditor) + n > UINT128_MAX or
                 not(withinUInt(balances.get(supplied.payer))) or
                 not(withinUInt(balances.get(o.creditor))) or
                 not(withinUInt(repaid(o, n).outstanding)))
          reject(Effect, "S0_EFFECT_RANGE")
        else if (supplied.payer != o.debtor or
                 supplied.debtor != o.debtor or
                 supplied.creditor != o.creditor or
                 supplied.conversionMantissa != 1 or
                 supplied.conversionScale != 0 or
                 not(supplied.roundingNone))
          reject(Authority, "S0_AUTH_SCOPE")
        else afterEffect(supplied, claimedHead, n, 0)
        }
      }
    }
  }

  action init: bool = all {
    withinUInt(initialWorkBudget),
    signed' = Map(),
    balances' = Map(),
    allowanceRemaining' = Map(),
    allowanceSpent' = Map(),
    workRemaining' = initialWorkBudget,
    workSpent' = 0,
    obligations' = Map(),
    currentHead' = 0,
    round' = 0,
    consumed' = Set(),
    lastEffects' = List(),
    committedCount' = 0,
  }

  // Environment setup represents authenticated cells; it is not a ledger proof.
  action seedAccount(account: str, balance: int,
                     remaining: int, spent: int): bool = all {
    committedCount == 0,
    not(balances.keys().contains(account)),
    withinUInt(balance) and withinUInt(remaining) and withinUInt(spent),
    remaining + spent <= UINT128_MAX,
    balances' = balances.put(account, balance),
    allowanceRemaining' = allowanceRemaining.put(account, remaining),
    allowanceSpent' = allowanceSpent.put(account, spent),
    workRemaining' = workRemaining, workSpent' = workSpent,
    signed' = signed, obligations' = obligations,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action seedObligation(id: str, o: Obligation): bool = all {
    committedCount == 0,
    not(obligations.keys().contains(id)),
    o.asset == settlementAsset,
    withinUInt(o.principal) and withinUInt(o.accrued) and
      withinUInt(o.outstanding),
    o.outstanding == o.principal + o.accrued,
    obligations' = obligations.put(id, o),
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action advanceRound: bool = all {
    round' = round + 1,
    signed' = signed, balances' = balances,
    allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent,
    workRemaining' = workRemaining, workSpent' = workSpent,
    obligations' = obligations, currentHead' = currentHead,
    consumed' = consumed, lastEffects' = lastEffects,
    committedCount' = committedCount,
  }

  action sign(i: SignedIntent): bool = all {
    not(signed.keys().contains(i.digest)),
    i.domain == executingDomain and i.asset == settlementAsset,
    i.sourceVersion == "Source/6" and i.coreVersion == "Core/5",
    i.profile == "S0",
    signed' = signed.put(i.digest, i),
    balances' = balances, allowanceRemaining' = allowanceRemaining,
    allowanceSpent' = allowanceSpent, obligations' = obligations,
    workRemaining' = workRemaining, workSpent' = workSpent,
    currentHead' = currentHead, round' = round, consumed' = consumed,
    lastEffects' = lastEffects, committedCount' = committedCount,
  }

  action submitTransfer(id: str, supplied: SignedIntent, claimedHead: int,
                        v: int, f: int, lines: List[EffectLine]): bool = all {
    transferObservation(id, supplied, claimedHead, v, f, lines).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - v - f)
      .put(supplied.recipient, balances.get(supplied.recipient) + v)
      .put(supplied.feeRecipient, balances.get(supplied.feeRecipient) + f),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - v - f),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + v + f),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = currentHead + 1,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, obligations' = obligations, round' = round,
  }

  action submitRepay(id: str, supplied: SignedIntent, claimedHead: int,
                     n: int, lines: List[EffectLine]): bool = all {
    repayObservation(id, supplied, claimedHead, n, lines).accepted,
    balances' = balances.put(supplied.payer,
                             balances.get(supplied.payer) - n)
      .put(obligations.get(supplied.obligationId).creditor,
           balances.get(obligations.get(supplied.obligationId).creditor) + n),
    obligations' = obligations.put(supplied.obligationId,
      repaid(obligations.get(supplied.obligationId), n)),
    allowanceRemaining' = allowanceRemaining.put(supplied.payer,
      allowanceRemaining.get(supplied.payer) - n),
    allowanceSpent' = allowanceSpent.put(supplied.payer,
      allowanceSpent.get(supplied.payer) + n),
    workRemaining' = workRemaining - 1,
    workSpent' = workSpent + 1,
    consumed' = consumed.union(Set(replayKey(supplied))),
    currentHead' = currentHead + 1,
    lastEffects' = lines,
    committedCount' = committedCount + 1,
    signed' = signed, round' = round,
  }
}

```

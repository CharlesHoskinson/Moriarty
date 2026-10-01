import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash,generateKeyPairSync,sign,ECDH} from 'node:crypto';
import {existsSync,mkdtempSync,writeFileSync,chmodSync,rmSync,readFileSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join,delimiter} from 'node:path';
import {spawnSync} from 'node:child_process';
import {escapeText,escapeAscii,asciiJson,formatAtoms,signedLeaves,STATEMENT_LABELS,renderOwnerIntentReview,renderVerificationReview,renderErrorReview} from '../src/intent-display.ts';
import {starterSource,starterScenario} from '../src/starter.ts';

const sha=(b)=>createHash('sha256').update(b).digest('hex');
const printable=/^[\x20-\x7e\n]*$/;
const cli=new URL('../src/cli.ts',import.meta.url).pathname;
const repoRoot=new URL('../../../',import.meta.url).pathname;

// Real native verifier: MORIARTY_CRYPTO_BINARY, else a local cargo target directory. Absent means explicit skip.
function nativeBinary(){
 const names=['moriarty-midnight-crypto'],dirs=[];
 if(process.env.MORIARTY_CRYPTO_BINARY)return existsSync(process.env.MORIARTY_CRYPTO_BINARY)?process.env.MORIARTY_CRYPTO_BINARY:null;
 for(const target of [process.env.CARGO_TARGET_DIR,join(repoRoot,'experiments/midnight-crypto/target')])if(target)for(const profile of ['release','debug'])dirs.push(join(target,profile));
 for(const dir of dirs)for(const name of names)if(existsSync(join(dir,name)))return join(dir,name);
 return null;
}
const binary=nativeBinary();
const needBinary={skip:binary?false:'native verifier absent: set MORIARTY_CRYPTO_BINARY to an absolute path of a locally built moriarty-midnight-crypto'};

const baseStatement=()=>({
 profile:'moriarty-signed-intent/1',core:'moriarty-core/5',sourceProfile:'moriarty-financial-agreement-source/6',authoringProfile:'moriarty-beta/1',
 agreementId:'Invoice',actionName:'pay',selectedActionId:'TransferLiteralFee',sourceSha256:'a'.repeat(64),ownerProgramSha256:'b'.repeat(64),
 domain:{id:'Midnight',chain:'midnight',network:'preview'},asset:{id:'A',representation:'canonical',scale:'2',symbol:'USD'},
 signature:{scheme:'ecdsa_secp256k1_sha256',publicKeyHex:'02'+'c'.repeat(64),keyRef:'key1',framing:'raw'},
 intent:{version:'moriarty-intent/3',advertisedSourceHash:'src1',advertisedPolicyDigest:'policy1',signer:'Owner',nonce:'n1',preHead:'h0',
  notBefore:'0',notAfter:'10',grossCap:'1010',feeCap:'10',netFloor:'1000',failure:'success_only',observations:[],disclosures:[],retainedEffects:[],retainedDuties:[],delegation:'none',recovery:'none',
  operation:{kind:'Transfer',from:'Owner',recipient:'Recipient',feeRecipient:'Fee',amount:'1000',fee:'10'}}});
const repayStatement=()=>{const s=baseStatement();s.selectedActionId='RepayAccrualFirst';s.agreementId='LoanAgreement';s.intent.signer='Payer';s.intent.grossCap='3000';s.intent.feeCap='0';s.intent.netFloor='0';s.asset.symbol=null;s.intent.operation={kind:'Repay',payer:'Payer',obligationId:'Loan',amount:'3000',conversion:'identity'};return s;};
function prepared(statement=baseStatement(),framing='raw'){
 const frame=Buffer.from('moriarty-signed-intent/1\0'+'frame body for '+statement.agreementId);
 const message=framing==='raw'?frame:Buffer.concat([Buffer.from(`midnight_signed_message:${frame.length}:`),frame]);
 statement.signature.framing=framing;
 return {status:'OwnerIntentPrepared',statement,frame_hex:frame.toString('hex'),frame_sha256:sha(frame),signing_message_hex:message.toString('hex'),scheme:statement.signature.scheme,framing,authority_valid:null,snapshot_membership_valid:null,transition_valid:null,ledger_accepted:false,qualification:'signature-protocol-only',signaturePossession:'NotChecked',keyAuthority:'Unverified'};
}
const context={sourceSha256:'a'.repeat(64),scenarioSha256:'d'.repeat(64),scenario:starterScenario};
const effects=[{kind:'Debit',account:'Owner',asset:'A',amount:'1010'},{kind:'Credit',account:'Recipient',asset:'A',amount:'1000'},{kind:'Credit',account:'Fee',asset:'A',amount:'10'},{kind:'UseAllowance',owner:'Owner',amount:'1010'},{kind:'UseReplay',key:'["Midnight","Owner","n1"]'},{kind:'AdvanceHead',predecessor:'h0',successor:'h1'}];
const post={core:'moriarty-core/5',domain:'Midnight',asset:'A',head:'h1',round:'1',workRemaining:'9',workSpent:'1',balances:[{account:'Owner',amount:'8990'},{account:'Recipient',amount:'1000'},{account:'Fee',amount:'10'}],allowances:[{owner:'Owner',remaining:'8990',spent:'1010'}],obligations:[],consumedReplay:['["Midnight","Owner","n1"]']};
function verified(local='prepared',valid=true,statement=baseStatement()){
 const receipt={...prepared(statement),status:'IntentSignatureChecked',signature_valid:valid};delete receipt.signaturePossession;delete receipt.keyAuthority;
 const localResult=!valid?null:local==='prepared'?{status:'PreparedUnqualified',sourceHash:'a'.repeat(64),scenarioHash:'d'.repeat(64),qualification:'local-stipulation-only',result:{status:'PreparedUnqualified',candidate:{status:'PreparedUnqualified',core:'moriarty-core/5',preHead:'h0',effects,candidatePost:post,requiredPremises:[]}}}
  :{status:'CoreRejected',sourceHash:'a'.repeat(64),scenarioHash:'d'.repeat(64),qualification:'local-stipulation-only',result:{status:'CoreRejected',rejection:{status:'Rejected',judgment:'intent',code:'S0_INTENT_SCOPE',diagnosticWork:1,publishedPost:null,publishedEffects:null}}};
 return {status:!valid?'SignatureRejected':local==='prepared'?'SignedPreparedUnqualified':'SignedCoreRejected',sourceMatched:true,signature:receipt,keyAuthority:'Unverified',domainMapping:'SourceClaimsBound',expiry:valid?(local==='prepared'?'LocalRoundWithinWindow':'LocalRoundOutsideWindow'):'NotChecked',replay:valid?'LocallyUnused':'NotChecked',financial:!valid?'NotChecked':local==='prepared'?'LocallyPrepared':'CoreRejected',state:'LocalStipulationOnly',nativeProof:'NotChecked',ledger:'NotSubmitted',ledger_accepted:false,local:localResult,requiredPremises:['canonical-intent-signature','snapshot-to-head','head-extension','atomic-ledger-compare-and-consume'],unverifiedBindings:['agreement-id','selected-program','asset-scale','authenticated-predecessor']};
}
const lines=(text)=>text.split('\n');

test('escapeText is escape-by-default: only printable ASCII passes, every other scalar is visible',()=>{
 assert.equal(escapeText('plain Text_09'),'"plain Text_09"');
 assert.equal(escapeText('a"b\\c'),'"a\\"b\\\\c"');
 for(const [input,expected] of [
  ['\n','"\\u{A}"'],['\r','"\\u{D}"'],['\t','"\\u{9}"'],['\0','"\\u{0}"'],['\x1b[2J','"\\u{1B}[2J"'],['\x7f','"\\u{7F}"'],['\x85','"\\u{85}"'],
  ['\u202e','"\\u{202E}"'],['\u2066','"\\u{2066}"'],['\u200b','"\\u{200B}"'],['\ufeff','"\\u{FEFF}"'],['\u2028','"\\u{2028}"'],['\u2029','"\\u{2029}"'],['\ufe0f','"\\u{FE0F}"'],
  ['\u{1F600}','"\\u{1F600}"'],['\ud800','"\\u{D800}"'],['e\u0301','"e\\u{301}"'],['\u00e9','"\\u{E9}"'],
 ])assert.equal(escapeText(input),expected,JSON.stringify(input));
 assert.notEqual(escapeText('\u00e9'),escapeText('e\u0301'),'NFC and NFD forms must look different');
 assert.match(escapeText('pre\u202eview\u200b'),printable);
 assert.equal(escapeAscii('mori: bad \u202e option\n'),'mori: bad \\u{202E} option\\u{A}');
 assert.equal(escapeAscii('a\\u{41}'),'a\\\\u{41}','a literal backslash is escaped so output cannot imitate an escape');
});

test('asciiJson keeps the parsed value while emitting only printable ASCII and newlines',()=>{
 const nasty={network:'pre\u202eview\u200b',line:'a\u2028b\u2029c\nd',del:'x\x7fy',emoji:'\u{1F600}',lone:'\ud800',nfd:'e\u0301','k\u202e':1,list:['\u0085',null,true,0],nested:{a:{b:'\u00e9'}}};
 for(const space of [0,2]){const text=asciiJson(nasty,space);assert.match(text,printable,'ASCII only');assert.deepEqual(JSON.parse(text),nasty);}
 assert.equal(asciiJson({a:1}),'{\n  "a": 1\n}');
 assert.equal(asciiJson('\u202e'),'"\\u202e"');
});

test('formatAtoms derives exact scaled decimals with BigInt only',()=>{
 for(const [atoms,scale,expected] of [
  ['0',0,'0'],['7',0,'7'],['5',1,'0.5'],['50',1,'5.0'],['1000',2,'10.00'],['5',2,'0.05'],['0',2,'0.00'],['99',2,'0.99'],['100',2,'1.00'],['1010',2,'10.10'],
  ['0',18,'0.000000000000000000'],['1',18,'0.000000000000000001'],
  ['170141183460469231731687303715884105727',0,'170141183460469231731687303715884105727'],
  ['170141183460469231731687303715884105727',2,'1701411834604692317316873037158841057.27'],
  ['170141183460469231731687303715884105727',18,'170141183460469231731.687303715884105727'],
 ])assert.equal(formatAtoms(atoms,scale),expected,`${atoms}@${scale}`);
 for(const bad of ['','01','+1','-1','1e3','1.0',' 1','1 ','1,000','0x10','٣'])assert.throws(()=>formatAtoms(bad,2),{code:'BETA_INTENT_AMOUNT_FORMAT'},bad);
 for(const bad of [-1,19,1.5,'x','01'])assert.throws(()=>formatAtoms('1',bad),{code:'BETA_INTENT_AMOUNT_FORMAT'},String(bad));
 assert.equal(formatAtoms('1000','2'),'10.00','scale arrives as canonical decimal text in statements');
});

test('every signed statement leaf has a label and every leaf value appears in the review',()=>{
 for(const [name,statement] of [['transfer',baseStatement()],['repay',repayStatement()]]){
  const leaves=signedLeaves(statement);
  assert.ok(leaves.length>=30,`${name} leaf count ${leaves.length}`);
  for(const leaf of leaves)assert.ok(Object.hasOwn(STATEMENT_LABELS,leaf.path),`unlabeled ${leaf.path}`);
  for(const framing of ['raw','midnight-sign-data']){
   const result=prepared(structuredClone(statement),framing),text=renderOwnerIntentReview(result,context);
   for(const leaf of signedLeaves(result.statement)){
    const shown=leaf.value===null?'none':Array.isArray(leaf.value)?'none':String(leaf.value);
    assert.ok(text.includes(shown)||text.includes(escapeText(shown)),`${name}/${framing} leaf ${leaf.path}=${shown} missing`);
    const row=lines(text).filter(l=>l.includes(`[${leaf.path}]`));assert.equal(row.length,1,`${leaf.path} must appear on exactly one labeled row`);
   }
  }
 }
});

test('unlabeled, missing and extra statement leaves fail closed',()=>{
 const extra=prepared();extra.statement.intent.surprise='x';
 assert.throws(()=>renderOwnerIntentReview(extra,context),{code:'BETA_DISPLAY_SHAPE'});
 const missing=prepared();delete missing.statement.signature.keyRef;
 assert.throws(()=>renderOwnerIntentReview(missing,context),{code:'BETA_DISPLAY_SHAPE'});
 const listed=prepared();listed.statement.intent.observations=['x'];
 assert.throws(()=>renderOwnerIntentReview(listed,context),{code:'BETA_DISPLAY_SHAPE'});
 const unknownKind=prepared();unknownKind.statement.intent.operation={kind:'Swap',amount:'1'};
 assert.throws(()=>renderOwnerIntentReview(unknownKind,context),{code:'BETA_DISPLAY_SHAPE'});
 assert.throws(()=>renderOwnerIntentReview({status:'OwnerIntentPrepared'},context),{code:'BETA_DISPLAY_SHAPE'});
});

test('owner intent review separates signed terms from claims, computed hashes and unsigned context',()=>{
 const text=renderOwnerIntentReview(prepared(),context);
 assert.match(text,printable);
 assert.match(lines(text)[0],/^OwnerIntentPrepared \(UNSIGNED\)/);
 // atoms, asset id and scale always travel together; the decimal is derived and labeled so
 assert.match(text,/\[intent\/operation\/amount\][^\n]*1000 atoms[^\n]*asset "A"[^\n]*scale 2[^\n]*10\.00 \(display only\)/);
 assert.match(text,/\[intent\/operation\/fee\][^\n]*10 atoms[^\n]*asset "A"[^\n]*scale 2[^\n]*0\.10 \(display only\)/);
 assert.match(text,/\[intent\/grossCap\][^\n]*1010 atoms[^\n]*10\.10 \(display only\)/);
 assert.match(text,/\[intent\/notAfter\][^\n]*10[^\n]*round/i);
 assert.ok(!/\[intent\/notAfter\][^\n]*atoms/.test(text),'rounds are not money');
 assert.match(text,/\[asset\/symbol\][^\n]*"USD"[^\n]*label only/);
 assert.match(text,/\[signature\/keyRef\][^\n]*"key1"[^\n]*symbolic label[^\n]*not a (public )?key/i);
 assert.match(text,/\[signature\/publicKeyHex\][^\n]*02c{64}/);
 assert.match(text,/\[signature\/framing\][^\n]*raw/);
 assert.match(text,/\[signature\/scheme\][^\n]*ecdsa_secp256k1_sha256/);
 assert.match(text,/\[sourceSha256\][^\n]*computed[^\n]*a{64}/i);
 assert.match(text,/\[intent\/advertisedSourceHash\][^\n]*"src1"[^\n]*claim/i);
 assert.match(text,/\[intent\/advertisedPolicyDigest\][^\n]*"policy1"[^\n]*claim/i);
 assert.match(text,/\[ownerProgramSha256\][^\n]*b{64}[^\n]*not a compiled/i);
 assert.match(text,/frame sha256[^\n]*claimed[^\n]*[0-9a-f]{64}/i);
 assert.match(text,/frame sha256[^\n]*computed here[^\n]*[0-9a-f]{64}[^\n]*match/i);
 assert.match(text,/decoded bytes/i);assert.match(text,/not the hex text/i);assert.match(text,/not the sha256/i);
 assert.match(text,/Nothing was signed/);
 assert.match(text,/Unsigned scenario context/);assert.match(text,/scenario sha256[^\n]*d{64}/);
 assert.match(text,/not part of the signed statement/i);
 assert.match(text,/Not established/);
 for(const word of ['owner authority','key-to-account binding','current state','replay freshness','proof','ledger acceptance','settlement'])assert.match(text,new RegExp(word),word);
 assert.match(text,/ledger_accepted false/);
 assert.ok(!/\bAuthorized\b|\bApproved\b|Settled\b|\bVerified\b/.test(text),'no success vocabulary');
 assert.ok(!/is a public key|keyRef is the key/i.test(text));
});

test('midnight-sign-data review names the exact prefix and wallet mode has no fallback',()=>{
 const p=prepared(baseStatement(),'midnight-sign-data'),text=renderOwnerIntentReview(p,context),frameLength=p.frame_hex.length/2;
 assert.match(text,new RegExp(`midnight_signed_message:${frameLength}:`));
 assert.match(text,/framing is signed metadata/i);assert.match(text,/no other framing is tried/i);
 assert.match(text,/live wallet compatibility .*not (verified|performed)/i);
 const raw=renderOwnerIntentReview(prepared(),context);
 assert.match(raw,/signing message is the frame itself/i);
});

test('hostile text is escaped everywhere in readable review, with a warning',()=>{
 const p=prepared();p.statement.domain.network='pre\u202eview\u200b';p.statement.signature.keyRef='k\ney\x1b[31m';p.statement.intent.nonce='n\u2028x';p.statement.asset.symbol='\u{1F600}';
 const text=renderOwnerIntentReview(p,context);
 assert.match(text,printable);
 assert.ok(text.includes('"pre\\u{202E}view\\u{200B}"'));assert.ok(text.includes('"k\\u{A}ey\\u{1B}[31m"'));assert.ok(text.includes('\\u{2028}'));
 assert.match(text,/non-ASCII or control text[^\n]*compare the escaped form/i);
 const clean=renderOwnerIntentReview(prepared(),context);assert.ok(!/non-ASCII or control text/.test(clean));
 const hostileContext={...context,scenario:{...starterScenario,head:'h\u202e0',round:'1\n2'}};
 assert.match(renderOwnerIntentReview(prepared(),hostileContext),printable);
});

test('a returned frame hash that differs from the recomputed hash is displayed as a mismatch, never hidden',()=>{
 const p=prepared();p.frame_sha256='0'.repeat(64);
 const text=renderOwnerIntentReview(p,context);assert.match(text,/MISMATCH/);
 const wrongSource=renderOwnerIntentReview(prepared(),{...context,sourceSha256:'e'.repeat(64)});assert.match(wrongSource,/\[sourceSha256\][^\n]*MISMATCH/);
});

test('verification review reports signature, authority gap and local result separately',()=>{
 const text=renderVerificationReview(verified(),context);
 assert.match(text,printable);assert.match(lines(text)[0],/^SignedPreparedUnqualified/);
 assert.match(text,/Signature check[^\n]*\n[^\n]*signature_valid true/i);
 assert.match(text,/the key used signed these exact frame bytes/i);
 assert.match(text,/does not show that the key controls account "Owner"/);
 assert.match(text,/source matched[^\n]*yes/i);
 assert.match(text,/claimed[^\n]*a{64}[^\n]*computed[^\n]*a{64}/is);
 assert.match(text,/keyAuthority Unverified/);
 assert.match(text,/Local result[^\n]*PreparedUnqualified/);
 assert.match(text,/Debit[^\n]*"Owner"[^\n]*1010 atoms[^\n]*scale 2[^\n]*10\.10 \(display only\)/);
 assert.match(text,/Credit[^\n]*"Recipient"[^\n]*1000 atoms/);
 assert.match(text,/AdvanceHead[^\n]*"h0"[^\n]*"h1"/);
 assert.match(text,/balance "Owner"[^\n]*8990 atoms[^\n]*89\.90 \(display only\)/);
 assert.match(text,/requiredPremises[^\n]*canonical-intent-signature[^\n]*atomic-ledger-compare-and-consume/);
 assert.match(text,/unverifiedBindings[^\n]*agreement-id/);
 assert.match(text,/scenario sha256[^\n]*d{64}[^\n]*unsigned/i);
 assert.match(text,/Not established/);assert.match(text,/ledger_accepted false/);
 assert.ok(!/\bAuthorized\b|\bApproved\b|Settled\b/.test(text));
 // every signed leaf of the matched statement is on the screen as well
 for(const leaf of signedLeaves(baseStatement()))assert.ok(text.includes(`[${leaf.path}]`),leaf.path);
});

test('verification review covers core rejection, invalid signature and repay',()=>{
 const rejected=renderVerificationReview(verified('rejected'),context);
 assert.match(lines(rejected)[0],/^SignedCoreRejected/);assert.match(rejected,/signature_valid true/);assert.match(rejected,/S0_INTENT_SCOPE/);assert.match(rejected,/No effects were published/i);
 assert.match(rejected,/does not make a candidate acceptable/i);
 const invalid=renderVerificationReview(verified('prepared',false),context);
 assert.match(lines(invalid)[0],/^SignatureRejected/);assert.match(invalid,/signature_valid false/);assert.match(invalid,/Local preparation did not run/i);assert.ok(!/Debit/.test(invalid));
 assert.match(invalid,/other framing was not tried/i);
 const repay=renderVerificationReview(verified('prepared',true,repayStatement()),context);
 assert.match(repay,/\[intent\/operation\/obligationId\][^\n]*"Loan"/);assert.match(repay,/\[intent\/operation\/amount\][^\n]*3000 atoms[^\n]*30\.00 \(display only\)/);assert.match(repay,/\[asset\/symbol\][^\n]*none/);
});

test('error review keeps actual status and code, escapes hostile messages and separates judgment from inability to judge',()=>{
 const judged=renderErrorReview({code:'BETA_SIGNATURE_SOURCE_MISMATCH',message:'Signed statement differs from current source'},'judgment');
 assert.match(lines(judged)[0],/^FormationRejected\s+BETA_SIGNATURE_SOURCE_MISMATCH/);assert.match(judged,/says nothing yet about the signature/i);
 const unable=renderErrorReview({code:'BETA_CRYPTO_BINARY_UNAVAILABLE',message:'Native verifier unavailable; configure an installed binary'},'unable');
 assert.match(unable,/no judgment was made/i);assert.match(unable,/signature validity is unknown, not false/i);assert.match(unable,/cargo build --release --locked/);assert.match(unable,/--crypto-binary/);
 assert.ok(!/signature_valid (true|false)/.test(unable));assert.ok(!/\binvalid\b|\bvalid\b/i.test(unable),'no validity word on an exit 2 screen');
 const hostile=renderErrorReview({code:'BETA_AUTH_SOURCE',message:'bad \u202e text\nFAKE LINE'},'judgment');
 assert.match(hostile,printable);assert.ok(!/^FAKE LINE/m.test(hostile));
 assert.match(renderErrorReview({code:'BETA_UNKNOWN_FUTURE',message:'x'},'judgment'),/BETA_UNKNOWN_FUTURE/);
});

// ---- actual CLI behavior ----
function runCli(args,{env={},input}={}){return spawnSync(process.execPath,[cli,...args],{encoding:'utf8',env:{PATH:'',...env},input});}
function project(dir,template){const r=runCli(['init',dir,...(template?['--template',template]:[])]);assert.equal(r.status,0,r.stderr);}
const ecdsa=(()=>{
 const {privateKey,publicKey}=generateKeyPairSync('ec',{namedCurve:'secp256k1'}),jwk=publicKey.export({format:'jwk'});
 const keyHex=ECDH.convertKey(Buffer.concat([Buffer.from([4]),Buffer.from(jwk.x,'base64url'),Buffer.from(jwk.y,'base64url')]),'secp256k1',undefined,'hex','compressed');
 const n=BigInt('0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141');
 const signHex=(message)=>{const bytes=sign('sha256',Buffer.from(message,'hex'),{key:privateKey,dsaEncoding:'ieee-p1363'}),s=BigInt('0x'+bytes.subarray(32).toString('hex'));if(s>n/2n)Buffer.from((n-s).toString(16).padStart(64,'0'),'hex').copy(bytes,32);return bytes.toString('hex');};
 return {keyHex,signHex};
})();
const intentArgs=(dir,file,action,framing='raw')=>['intent',join(dir,file),'--action',action,'--scenario',join(dir,'scenario.json'),'--scheme','ecdsa_secp256k1_sha256','--public-key',ecdsa.keyHex,'--framing',framing,'--crypto-binary',binary];
const verifyArgs=(dir,file,action,artifact)=>['verify-intent',join(dir,file),'--action',action,'--scenario',join(dir,'scenario.json'),'--signature',artifact,'--crypto-binary',binary];

test('new commands: --review and --json are mutually exclusive and usage errors stay ASCII',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-usage-'));
 try{
  for(const command of ['intent','verify-intent']){
   const r=runCli([command,join(dir,'x.mori'),'--review','--json']);
   assert.equal(r.status,1);assert.equal(r.stdout,'');assert.match(r.stderr,/--review and --json cannot be combined/);
  }
  const r=runCli(['intent',join(dir,'x.mori'),'--bogus\u202e']);
  assert.equal(r.status,1);assert.match(r.stderr,printable);assert.match(r.stderr,/\\u\{202E\}/);
  const missing=runCli(['intent',join(dir,'caf\u00e9.mori'),'--review']);assert.equal(missing.status,1);assert.match(missing.stderr+missing.stdout,printable);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('existing commands keep their exact output behavior',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-compat-'));
 try{
  project(join(dir,'p'));const file=join(dir,'p/invoice.mori');
  let r=runCli(['check',file,'--review']);assert.equal(r.status,1);assert.match(r.stderr,/Unknown or duplicate option --review/);
  r=runCli(['simulate',file,'--action','pay','--scenario',join(dir,'p/scenario.json')]);assert.equal(r.status,0);assert.equal(r.stdout,JSON.stringify(JSON.parse(r.stdout),null,2)+'\n');
  r=runCli(['check',file,'--bogus\u00e9']);assert.equal(r.status,1);assert.ok(r.stderr.includes('--bogus\u00e9'),'existing commands are not re-escaped');
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('missing, relative or fake native verifier gives exit 2 and never a validity claim',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-nobin-'));
 try{
  project(join(dir,'p'));
  const fake=join(dir,'fake'),artifact=join(dir,'sig.json');
  writeFileSync(fake,`#!${process.execPath}\nprocess.stdin.resume();process.stdin.on('end',()=>console.log('{"status":"IntentSignatureChecked","signature_valid":true}'));\n`);chmodSync(fake,0o755);
  const sigText=JSON.stringify({statement:{signature:{scheme:'ecdsa_secp256k1_sha256',publicKeyHex:'02'+'1'.repeat(64),keyRef:'key1',framing:'raw'}},signatureHex:'0'.repeat(128)});writeFileSync(artifact,sigText);
  for(const crypto of [join(dir,'missing'),'relative/verifier',fake,dir])for(const flag of ['--review','--json']){
   const args=['verify-intent',join(dir,'p/invoice.mori'),'--action','pay','--scenario',join(dir,'p/scenario.json'),'--signature',artifact,'--crypto-binary',crypto,flag];
   const r=runCli(args);assert.equal(r.status,2,`${crypto} ${flag}: ${r.stdout}${r.stderr}`);assert.match(r.stdout,printable);
   assert.ok(!/signature_valid\W+(true|false)/.test(r.stdout),'transport failure never reports validity');
   if(flag==='--json')assert.equal(JSON.parse(r.stdout).status,'FormationRejected');else assert.match(r.stdout,/no judgment was made/i);
  }
  for(const flag of ['--review','--json']){
   const r=runCli(['intent',join(dir,'p/invoice.mori'),'--action','pay','--scenario',join(dir,'p/scenario.json'),'--scheme','ecdsa_secp256k1_sha256','--public-key',ecdsa.keyHex,'--framing','raw','--crypto-binary',fake,flag]);
   assert.equal(r.status,2,r.stdout+r.stderr);assert.match(r.stdout,/BETA_CRYPTO_RESPONSE/);
  }
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('source and scenario judgments stay exit 1 while keeping JSON compatibility',()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-judge-'));
 try{
  project(join(dir,'p'));
  const bad=join(dir,'bad.mori');writeFileSync(bad,'profile "moriarty-beta/1";\nagreement X {}');
  const common=['--action','pay','--scenario',join(dir,'p/scenario.json'),'--scheme','ecdsa_secp256k1_sha256','--public-key',ecdsa.keyHex,'--framing','raw','--crypto-binary','/nonexistent/native'];
  let r=runCli(['intent',bad,...common]);assert.equal(r.status,1);let json=JSON.parse(r.stdout);assert.equal(json.status,'FormationRejected');assert.equal(json.diagnostics[0].code,'BETA_AUTH_SOURCE');assert.deepEqual(Object.keys(json),['status','diagnostics','publishedEffects','publishedPost']);assert.match(r.stdout,printable);
  r=runCli(['intent',bad,...common,'--review']);assert.equal(r.status,1);assert.match(r.stdout,/^FormationRejected\s+BETA_AUTH_SOURCE/);assert.match(r.stdout,printable);
  const artifact=join(dir,'sig.json');writeFileSync(artifact,'{"statement":1,"statement":2}');
  r=runCli(['verify-intent',join(dir,'p/invoice.mori'),'--action','pay','--scenario',join(dir,'p/scenario.json'),'--signature',artifact,'--crypto-binary','/nonexistent/native']);
  assert.equal(r.status,1,'a malformed artifact is a judgment, not a transport failure');assert.equal(JSON.parse(r.stdout).diagnostics[0].code,'BETA_JSON_DUPLICATE');
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('real native: intent review and JSON agree and match source-derived terms',needBinary,()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-real-'));
 try{
  project(join(dir,'p'));
  const json=runCli(intentArgs(join(dir,'p'),'invoice.mori','pay'));assert.equal(json.status,0,json.stderr+json.stdout);assert.match(json.stdout,printable);
  const value=JSON.parse(json.stdout),explicit=runCli([...intentArgs(join(dir,'p'),'invoice.mori','pay'),'--json']);assert.equal(explicit.stdout,json.stdout,'--json equals the default');
  const review=runCli([...intentArgs(join(dir,'p'),'invoice.mori','pay'),'--review']);assert.equal(review.status,0,review.stderr);assert.match(review.stdout,printable);assert.throws(()=>JSON.parse(review.stdout));
  assert.match(lines(review.stdout)[0],/^OwnerIntentPrepared \(UNSIGNED\)/);
  for(const leaf of signedLeaves(value.statement))assert.ok(review.stdout.includes(`[${leaf.path}]`),leaf.path);
  assert.ok(review.stdout.includes(value.statement.sourceSha256));assert.ok(review.stdout.includes(sha(starterSource)),'source hash is computed from the file bytes');
  assert.ok(review.stdout.includes(value.frame_sha256));assert.ok(review.stdout.includes(value.signing_message_hex));
  assert.match(review.stdout,/scenario sha256[^\n]*/);assert.ok(review.stdout.includes(sha(readFileSync(join(dir,'p/scenario.json')))));
  assert.equal(value.ledger_accepted,false);assert.equal(value.authority_valid,null);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('real native: verify review for transfer and repay, expired scenario, bad signature and edited source',needBinary,()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-real-verify-'));
 try{
  for(const [template,file,action,framing] of [[undefined,'invoice.mori','pay','raw'],['repay','repayment.mori','repay_loan','midnight-sign-data']]){
   const p=join(dir,template??'transfer');project(p,template);
   const prepared=JSON.parse(runCli(intentArgs(p,file,action,framing)).stdout),artifact=join(p,'sig.json');
   writeFileSync(artifact,JSON.stringify({statement:prepared.statement,signatureHex:ecdsa.signHex(prepared.signing_message_hex)}));
   let r=runCli([...verifyArgs(p,file,action,artifact),'--review']);assert.equal(r.status,0,r.stdout+r.stderr);assert.match(r.stdout,printable);
   assert.match(lines(r.stdout)[0],/^SignedPreparedUnqualified/);assert.match(r.stdout,/signature_valid true/);assert.match(r.stdout,/keyAuthority Unverified/);assert.match(r.stdout,/Not established/);
   assert.ok(r.stdout.includes(sha(readFileSync(join(p,file)))));
   const json=runCli(verifyArgs(p,file,action,artifact));assert.equal(json.status,0);const value=JSON.parse(json.stdout);assert.equal(value.status,'SignedPreparedUnqualified');assert.equal(value.ledger_accepted,false);
   // expired round: signature stays valid, financial candidate is rejected
   const scenarioPath=join(p,'scenario.json'),scenario=JSON.parse(readFileSync(scenarioPath,'utf8'));writeFileSync(scenarioPath,JSON.stringify({...scenario,round:'11'}));
   r=runCli([...verifyArgs(p,file,action,artifact),'--review']);assert.equal(r.status,1);assert.match(lines(r.stdout)[0],/^SignedCoreRejected/);assert.match(r.stdout,/signature_valid true/);assert.match(r.stdout,/S0_INTENT_SCOPE/);
   writeFileSync(scenarioPath,JSON.stringify(scenario));
   // flipped signature nibble: the verifier judged it false
   const flipped=join(p,'flipped.json'),art=JSON.parse(readFileSync(artifact,'utf8'));art.signatureHex=(art.signatureHex[0]==='0'?'1':'0')+art.signatureHex.slice(1);writeFileSync(flipped,JSON.stringify(art));
   r=runCli([...verifyArgs(p,file,action,flipped),'--review']);assert.equal(r.status,1);assert.match(lines(r.stdout)[0],/^SignatureRejected/);assert.match(r.stdout,/signature_valid false/);assert.match(r.stdout,/Local preparation did not run/i);
   // source edited after signing: exit 1, first difference points at sourceSha256 with claimed vs computed hashes
   const edited=join(p,'edited.mori');writeFileSync(edited,readFileSync(join(p,file),'utf8')+'\n');
   r=runCli(['verify-intent',edited,'--action',action,'--scenario',scenarioPath,'--signature',artifact,'--crypto-binary',binary,'--review']);
   assert.equal(r.status,1);assert.match(lines(r.stdout)[0],/^FormationRejected\s+BETA_SIGNATURE_SOURCE_MISMATCH/);assert.match(r.stdout,printable);
   assert.match(r.stdout,/first difference[^\n]*\/statement\/sourceSha256/i);assert.ok(r.stdout.includes(prepared.statement.sourceSha256));assert.ok(r.stdout.includes(sha(readFileSync(edited))));
   assert.match(r.stdout,/claimed by artifact[^\n]*computed from this source/i);
   const editedJson=runCli(['verify-intent',edited,'--action',action,'--scenario',scenarioPath,'--signature',artifact,'--crypto-binary',binary]);
   assert.equal(editedJson.status,1);assert.equal(JSON.parse(editedJson.stdout).diagnostics[0].code,'BETA_SIGNATURE_SOURCE_MISMATCH');
   // amount changed inside the artifact: first difference names the amount leaf
   const tampered=join(p,'tampered.json'),t=JSON.parse(readFileSync(artifact,'utf8'));t.statement.intent.operation.amount='1';writeFileSync(tampered,JSON.stringify(t));
   r=runCli([...verifyArgs(p,file,action,tampered),'--review']);assert.equal(r.status,1);assert.match(r.stdout,/first difference[^\n]*\/statement\/intent\/operation\/amount/i);
  }
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test('real native: hostile source text is escaped in readable and JSON output',needBinary,()=>{
 const dir=mkdtempSync(join(tmpdir(),'mori-display-hostile-'));
 try{
  project(join(dir,'p'));
  const hostile=starterSource.replace('network: "preview"','network: "pre\u202eview\u200b"').replace('key: "key1"','key: "k\u00e9y\u2028"');
  writeFileSync(join(dir,'p/invoice.mori'),hostile);
  const review=runCli([...intentArgs(join(dir,'p'),'invoice.mori','pay'),'--review']),json=runCli(intentArgs(join(dir,'p'),'invoice.mori','pay'));
  for(const r of [review,json]){assert.match(r.stdout,printable);assert.match(r.stderr,printable);}
  if(review.status===0){
   assert.ok(review.stdout.includes('\\u{202E}'));assert.ok(review.stdout.includes('\\u{2028}'));assert.match(review.stdout,/non-ASCII or control text/);
   assert.ok(json.stdout.includes('\\u202e'));assert.equal(JSON.parse(json.stdout).statement.domain.network,'pre\u202eview\u200b');
  }else assert.equal(review.status,1,'schema rejection is a judgment, never exit 2');
 }finally{rmSync(dir,{recursive:true,force:true});}
});

"""Mechanical citations for a manually reviewed, source-only API comparison."""
from pathlib import Path
import hashlib, json, re

R = Path(__file__).parent
LS = 'candidate/crates/mori/src/commands/lsp.rs'
DG = 'candidate/crates/mori/src/commands/lsp/diagnostics.rs'
S = 'dependency/source/lsp-server-0.10.0/src/'
G = 'dependency/source/gen-lsp-types-0.11.0/src/'
ST = G + 'generated/structures.rs'
EO = G + 'generated/enum_ors.rs'
EN = G + 'generated/enumerations.rs'

def cite(path, start, end):
    lines = (R / path).read_text().splitlines()
    assert 1 <= start <= end <= len(lines)
    return {'path':path,'startLine':start,'endLine':end,'excerpt':'\n'.join(lines[start-1:end])}
def use(path, anchor):
    matches = [i for i,line in enumerate((R/path).read_text().splitlines(),1) if anchor in line]
    assert matches, (path,anchor)
    return {'path':path,'anchor':anchor,'lines':matches}

rows = []
def add(api, calls, definitions, conclusion, condition=None):
    row = {'id':f'API-{len(rows)+1:02}','api':api,'consumerReferences':[use(*c) for c in calls],
           'publicDefinitions':[cite(*d) for d in definitions],
           'sourceComparison':'matching definition/signature found','reason':conclusion}
    if condition: row['conditionOrLimitation'] = condition
    rows.append(row)

add('lsp_server root imports',[(LS,'use lsp_server::')],[(S+'lib.rs',19,28)],
    'ErrorCode, Message, Notification, Request and Response are public crate-root re-exports of msg types.')
add('Message variants',[(LS,'Message::Request('),(LS,'Message::Notification('),(LS,'Message::Response(')],[(S+'msg.rs',11,17)],
    'The public enum has precisely the Request(Request), Response(Response) and Notification(Notification) variants used by dispatch.')
add('Message::read',[(LS,'Message::read(')],[(S+'msg.rs',172,175)],
    'Signature is read(&mut impl BufRead) -> io::Result<Option<Message>>. The candidate passes &mut StdinLock and handles Result then Option.')
add('Message::write',[(LS,'.write(output)')],[(S+'msg.rs',191,203)],
    'Signature is write(&self, &mut impl Write) -> io::Result<()>, matching each candidate output helper and error mapping.')
add('Request public fields',[(LS,'request.method'),(LS,'request.params'),(LS,'request.id')],[(S+'msg.rs',72,80)],
    'method is String; params is serde_json::Value; id is RequestId. Response constructors accept that same RequestId.')
add('Response::new_ok',[(LS,'Response::new_ok(')],[(S+'msg.rs',206,209)],
    'new_ok<R: serde::Serialize>(RequestId, R) -> Response accepts InitializeResult (derived Serialize) and unit ().')
add('Response::new_err',[(LS,'Response::new_err(')],[(S+'msg.rs',210,214)],
    'new_err(RequestId, i32, String) -> Response matches the moved id, cast enum discriminant and inferred String conversion.')
add('ErrorCode variants/casts',[(LS,'ErrorCode::InvalidRequest'),(LS,'ErrorCode::InvalidParams'),(LS,'ErrorCode::ServerNotInitialized'),(LS,'ErrorCode::MethodNotFound')],[(S+'msg.rs',108,126)],
    'All four names are real unit enum variants; integer casts target the required i32 code parameter. The non_exhaustive marker does not prevent naming these variants.')
add('Notification public fields and constructor',[(LS,'notification.method'),(LS,'notification.params'),(LS,'Notification::new(')],[(S+'msg.rs',156,163),(S+'msg.rs',241,244)],
    'method is String, params serde_json::Value, and new(String, impl serde::Serialize) accepts the typed publish parameters.')
add('gen-lsp-types crate-root exports',[(LS,'use lsp_types::'),(DG,'use lsp_types::')],[(G+'lib.rs',1,4),(G+'generated.rs',19,33)],
    'All generated structures/enums/common types are public root re-exports. Workspace alias lsp-types maps to package gen-lsp-types, whose explicit lib target is gen_lsp_types; Cargo dependency renaming supplies the lsp_types external name.',
    'Cargo alias selection has not been executed; manifests are preserved exactly.')
add('Uri and HashMap trait bounds',[(LS,'HashMap<Uri, Document>'),(LS,'uri.clone()'),(DG,'uri.clone()')],[(G+'generated/common.rs',25,40)],
    'Under the adopted no-URI-feature configuration Uri(pub String) derives Clone, Eq and Hash, satisfying clone and HashMap key use; it also derives Serialize/Deserialize.',
    'The exact source cfg is not(url) and not(fluent-uri). No Cargo feature-unification result is asserted.')
add('InitializeParams + capabilities',[(LS,'from_value::<InitializeParams>'),(LS,'.capabilities')],[(ST,2093,2095),(ST,2131,2144)],
    'InitializeParams derives Deserialize and exposes public capabilities: ClientCapabilities. Candidate does not construct deprecated initialize fields.')
add('Client capability traversal',[(LS,'.text_document'),(LS,'document.publish_diagnostics')],[(ST,7820,7829),(ST,8341,8347),(ST,8424,8428)],
    'ClientCapabilities.text_document is Option<TextDocumentClientCapabilities>; that type has public publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>. The as_ref/and_then chain matches these types.')
add('Publish diagnostics client flags',[(LS,'capabilities.version_support'),(LS,'capabilities.diagnostics_capabilities.related_information')],[(ST,9949,9961),(ST,10812,10818)],
    'version_support is Option<bool>; diagnostics_capabilities is a flattened DiagnosticsCapabilities containing related_information: Option<bool>. Candidate uses the real nested Rust field rather than assuming serde flatten creates a direct Rust field.')
add('DidOpenTextDocumentParams',[(LS,'from_value::<DidOpenTextDocumentParams>'),(LS,'let document = params.text_document;')],[(ST,2329,2335)],
    'The type derives Deserialize and has text_document: TextDocumentItem.')
add('TextDocumentItem fields',[(LS,'document.uri'),(LS,'document.text'),(LS,'document.version')],[(ST,5648,5662)],
    'uri is Uri, text String, version i32; these match diagnostics borrow, owned HashMap key/document fields and Option<i32> diagnostic versions. language_id exists but is not read by this candidate.')
add('DidChangeTextDocumentParams',[(LS,'from_value::<DidChangeTextDocumentParams>'),(LS,'params.content_changes')],[(ST,2358,2377)],
    'The type derives Deserialize; content_changes is Vec<TextDocumentContentChangeEvent>, and text_document is VersionedTextDocumentIdentifier.')
add('VersionedTextDocumentIdentifier nesting',[(LS,'params.text_document.text_document_identifier.uri'),(LS,'params.text_document.version')],[(ST,6191,6199),(ST,4644,4650)],
    'version is i32 and the flattened Rust member is text_document_identifier: TextDocumentIdentifier, whose public uri: Uri matches candidate access.')
add('TextDocumentContentChangeEvent full variant',[(LS,'TextDocumentContentChangeEvent::TextDocumentContentChangeWholeDocument(')],[(EO,1688,1705)],
    'The exact named tuple variant exists alongside a partial variant. The let-else pattern binds its whole-document payload.')
add('Whole-document change payload',[(LS,'text = Some(change.text);')],[(ST,7970,7982)],
    'TextDocumentContentChangeWholeDocument.text is String, matching replacement document text.')
add('DidCloseTextDocumentParams',[(LS,'from_value::<DidCloseTextDocumentParams>'),(LS,'let uri = params.text_document.uri;')],[(ST,2414,2425),(ST,4644,4650)],
    'The type derives Deserialize and has text_document: TextDocumentIdentifier with uri: Uri.')
add('InitializeResult literal',[(LS,'fn initialize_result() -> InitializeResult'),(LS,'InitializeResult {')],[(ST,2174,2194)],
    'InitializeResult derives Serialize and its two public fields are capabilities: ServerCapabilities and server_info: Option<ServerInfo>; the candidate initializes both.')
add('ServerCapabilities fields/Default',[(LS,'capabilities: ServerCapabilities {'),(LS,'position_encoding:'),(LS,'text_document_sync:'),(LS,'experimental:')],[(ST,5942,5962),(ST,6083,6089),(G+'generated/type_aliases.rs',15,21)],
    'ServerCapabilities derives Default; the three explicitly set fields have the expected Option types. LspAny aliases serde_json::Value, matching json!(...). Remaining fields come from Default.')
add('PositionEncodingKind::UTF16',[(LS,'PositionEncodingKind::UTF16')],[(EN,1931,1943),(EN,1954,1961)],
    'UTF16 is a real enum variant; its Serialize conversion maps to the string utf-16. No dependency constant named UTF_16 is assumed.')
add('TextDocumentSyncOptions fields/Default',[(LS,'TextDocumentSyncOptions {'),(LS,'open_close:'),(LS,'change: Some(')],[(ST,7868,7891)],
    'Options derives Default and Copy; open_close: Option<bool>, change: Option<TextDocumentSyncKind> match candidate fields. Other fields default to None.')
add('TextDocumentSyncOptions -> TextDocumentSync conversion',[(LS,'.into(),')],[(EO,1746,1760),(ST,5961,5962)],
    'Explicit From<TextDocumentSyncOptions> for TextDocumentSync makes the .into() value fit ServerCapabilities.text_document_sync. The same line anchor also sees unrelated ordinary string conversions; this row addresses the options conversion only.')
add('TextDocumentSyncKind::Full',[(LS,'TextDocumentSyncKind::Full')],[(EN,888,913)],
    'Full exists and its Serialize conversion is numeric 1, matching the author-frozen initialize expectation.')
add('ServerInfo literal',[(LS,'server_info: Some(ServerInfo {')],[(ST,6171,6188)],
    'Public name: String and version: Option<String> are all fields. Its type name was added in the 3.18 metamodel but the candidate emits only the existing name/version wire fields.')
add('PublishDiagnosticsParams::new',[(LS,'PublishDiagnosticsParams::new(')],[(ST,2536,2558)],
    'The exact parameter order is (uri: Uri, version: Option<i32>, diagnostics: Vec<Diagnostic>) -> Self, which matches the candidate call and implements Serialize for Notification::new.')
add('Diagnostic::new nine-argument constructor',[(DG,'Diagnostic::new(')],[(ST,6265,6311),(ST,6312,6337)],
    'Both core and legacy calls supply exactly (Range, Option<DiagnosticSeverity>, Option<Code>, Option<CodeDescription>, Option<String>, Message, Option<Vec<DiagnosticTag>>, Option<Vec<DiagnosticRelatedInformation>>, Option<LspAny>). This is the real generated signature, not the older original-lsp-types constructor.')
add('Diagnostic code string conversions',[(DG,'Some(error.code.into())'),(DG,'code.to_string().into()')],[(EO,135,154)],
    'Code has From<String> and From<&str>. Actual mori_core::Error.code is String, and the legacy conversion also produces String; both .into() expressions match Code.')
add('Diagnostic message string conversion',[(DG,'error.message.into()'),(DG,'message.into()')],[(EO,1182,1196)],
    'The generated diagnostic Message is an untagged String/MarkupContent enum with From<String>; both candidate calls select the String conversion. This enum is distinct from lsp_server::Message and is inferred from Diagnostic::new, so no import-name collision occurs.')
add('DiagnosticSeverity variants',[(DG,'DiagnosticSeverity::Error'),(DG,'DiagnosticSeverity::Warning'),(DG,'DiagnosticSeverity::Information')],[(EN,2078,2102)],
    'The three named variants exist; Serialize maps Error=1, Warning=2, Information=3.')
add('DiagnosticRelatedInformation::new',[(DG,'DiagnosticRelatedInformation::new(')],[(ST,8000,8016)],
    'Signature (Location, String) -> Self matches the location and text.to_owned() arguments.')
add('Location::new',[(DG,'Location::new(')],[(ST,32,45)],
    'Signature (Uri, Range) -> Self matches cloned URI and label range.')
add('Range::new',[(DG,'Range::new(')],[(ST,4552,4563)],
    'Signature (Position, Position) -> Self matches both zero/default span construction and label endpoints.')
add('Position::new and Copy',[(DG,'Position::new('),(DG,'Range::new(at, at)')],[(ST,4744,4772)],
    'Position has public line/character: u32 and new(u32,u32). It derives Copy, so the same local at can be passed twice without a move error.')

imports = {'lsp_server':['ErrorCode','Message','Notification','Request','Response'],
           'lsp_types':['Diagnostic','DidChangeTextDocumentParams','DidCloseTextDocumentParams','DidOpenTextDocumentParams','InitializeParams','InitializeResult','PositionEncodingKind','PublishDiagnosticsParams','ServerCapabilities','ServerInfo','TextDocumentContentChangeEvent','TextDocumentSyncKind','TextDocumentSyncOptions','Uri','DiagnosticRelatedInformation','DiagnosticSeverity','Location','Position','Range']}
result = {'predicate':'Every newly used lsp-server/gen-lsp-types API in the seven-path snapshot has an exact public export/definition/signature/required trait conversion in the adopted version source under its stated cfg.',
          'method':'Manual ordinary source comparison with mechanically pinned full input bytes and exact excerpts; no Rust parser/compiler/product execution.',
          'result':'No API-definition or signature mismatch found in the inspected snapshot. Source-only, not a compiler result.',
          'directImportInventory':imports,'rows':rows,'mismatches':[],'patchJustified':False,
          'limitations':['No Cargo resolution, lock change, build or compiler check was run.','gen-lsp-types URI checks assume the adopted default-features=false/no optional URI features declaration; feature-unification selection remains unobserved.','Serde derives and standard trait contracts were read as source declarations, not executed.','This is not a protocol conformance, input/resource, security or full-language acceptance judgment.','Candidate test/fixture source is preserved; tests were not run and author-frozen expectations are not independent observations.']}
(R/'API-COMPATIBILITY.json').write_text(json.dumps(result,indent=2)+'\n')
md = ['# Source-only API compatibility', '', result['result'], '',
      'Seven current I candidate paths were snapshotted with repeated byte checks. Full adopted package text and original custody pins are preserved. This table binds only those exact bytes; it is not a compile-pass claim.', '',
      '| API | Candidate use | Exact public definition | Source conclusion |','|---|---|---|---|']
for row in rows:
    calls='; '.join(f"{Path(c['path']).name}:{','.join(map(str,c['lines']))}" for c in row['consumerReferences'])
    defs='; '.join(f"{d['path'].removeprefix('dependency/source/')}:{d['startLine']}–{d['endLine']}" for d in row['publicDefinitions'])
    md.append(f"| {row['id']} {row['api']} | {calls} | {defs} | {row['reason']}" + ((' Condition: '+row['conditionOrLimitation']) if row.get('conditionOrLimitation') else '')+' |')
md += ['', 'No source patch is justified by this bounded API comparison. Exact consumer anchors, complete excerpts and all limitations are in `API-COMPATIBILITY.json`; complete source bytes and original paths/hashes are in `INPUT-SOURCE-PINS.json`.', '',
       'No Cargo/resolver/lock/cache/worktree changes, acquired-source execution, server execution, dependency version selection, provider dispatch or security/resource judgments occurred. Actual compiler checking and resolved feature selection remain separate root steps.']
(R/'API-COMPATIBILITY.md').write_text('\n'.join(md)+'\n')
print(json.dumps({'rows':len(rows),'importedPublicSymbols':sum(len(v) for v in imports.values()),'mismatches':0,'patchJustified':False}))

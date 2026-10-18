#!/usr/bin/env python3
"""Static packet custody only. Reads pinned Git objects; never executes product/provider code."""
from pathlib import Path
import hashlib,json,re,subprocess
OUT=Path(__file__).resolve().parent
A=OUT.parent/'changed-method-full-review-a2'
K=Path('/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018')
R=Path('/home/charl/moriarty-c2-opus55-restored-20261017')
HEAD='ff4f718d8b1d9deccf40ff7597b6afbb1f1ccf75'
TREE='3b5d75a115712b155b68398d83ce18cef78a3d87'
BASE='e4d7a2b5709caa18a94f5256d03a0302364b8235'
def sha(b):return hashlib.sha256(b).hexdigest()
def jread(p):return json.loads(p.read_bytes())
def canonical(o):return json.dumps(o,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()
def dump(name,obj):(OUT/name).write_text(json.dumps(obj,indent=2,ensure_ascii=False)+'\n')
def git(*args):return subprocess.check_output(['git','-C',str(K),*args])
def blob(rev,path):return git('show',rev+':'+path)
manifest=jread(A/'MEMBERSHIP.json'); mb=(A/'MEMBERSHIP.json').read_bytes()
# Hash opaque archive files without decoding provider exports or unembedded logs.
def snapshot():
 rows=[]
 for row in manifest['files']:
  b=(A/row['path']).read_bytes();assert len(b)==row['bytes'] and sha(b)==row['sha256'],row['path']
  rows.append({'path':row['path'],'bytes':len(b),'sha256':sha(b)})
 return rows
before=snapshot();assert len(before)==72
p=(A/'REVIEW-INPUT.md').read_bytes();p.decode('utf-8')
assert len(p)==109881 and sha(p)=='965b6af2239b1714fd4192aa95e2626f7dfda1fc9fb2db28a96178226e03ff63'
assert git('rev-parse',HEAD+'^{tree}').decode().strip()==TREE
mapping=jread(A/'INPUT-DELIVERY-MAP.json')
fences=[]
for m in re.finditer(rb'^```([^\n]*)\n(.*?)^```[ \t]*$',p,re.M|re.S):
 fences.append({'language':m[1].decode(),'start':m.start(2),'end':m.end(2),'bytes':len(m[2]),'sha256':sha(m[2])})
assert len(fences)==54
# Each section must appear once; offsets are zero-based half-open UTF-8 byte positions.
index=[]
def block(heading,lang):
 h=heading.encode();assert p.count(h)==1,heading
 hs=p.index(h);n=p.find(b'\n## ',hs+len(h));n=len(p) if n<0 else n
 marker=b'```'+lang.encode()+b'\n';start=p.index(marker,hs,n)+len(marker)
 end=p.index(b'\n```',start,n)
 return start,end,p[start:end]
def record(kind,path,start,end,body,**extra):
 row={'kind':kind,'path':path,'start_byte_inclusive':start,'end_byte_exclusive':end,'bytes':len(body),'inline_sha256':sha(body),**extra}
 assert p[start:end]==body;index.append(row);return row
sources={}
for row in mapping['changed_file_pins']:
 path=row['path'];raw=blob(HEAD,path);physical=(A/'source/after'/path).read_bytes()
 assert raw==physical and sha(raw)==row['sha256'] and len(raw)==row['bytes']
 start,end,body=block('## FULL CHANGED FILE '+path,'rust')
 # Wrapper adds one newline after the complete raw file; exclude only that wrapper byte.
 assert body==raw, (path,len(body),len(raw))
 record('full-changed-source',path,start,end,body,revision=HEAD,git_and_physical_exact=True,source_lines=len(raw.splitlines()))
 sources[path]=raw
anchors_start,anchors_end,body=block('## Root-verified source/hash anchor index (full code above)','json')
anchors=json.loads(body);assert len(anchors)==4
for a in anchors:
 assert sha(sources[a['path']])==a['sha256']
 assert sources[a['path']].decode().splitlines()[a['anchor']['line']-1].strip()==a['anchor']['exact_text']
record('anchor-index','root-verified-index',anchors_start,anchors_end,body,all_four_symbols_and_hashes_exact=True)
jsonrows=[]
def jsonblock(heading,relative):
 start,end,body=block(heading,'json');actual=jread(A/relative);inline=json.loads(body)
 assert inline==actual and canonical(inline)==canonical(actual),relative
 raw=(A/relative).read_bytes()
 row=record('complete-json',relative,start,end,body,raw_sha256=sha(raw),raw_bytes=len(raw),semantic_sha256=sha(canonical(inline)),complete_parsed_equality=True)
 jsonrows.append(row);return inline
modes=[]
for mode in ['no-default-features','ordinary-optins']:
 actual=jsonblock('## ACTUAL CLI stdout '+mode+'/QUERY-STDOUT.json','actual-check-a1/'+mode+'/QUERY-STDOUT.json')
 expected=jsonblock('## SPECIFIED EXPECTED complete contract '+mode+'.json','specified-expected/'+mode+'.json')
 assert actual==expected and canonical(actual)==canonical(expected)
 step=jsonblock('## ACTUAL step result '+mode+'/RESULT.json','actual-check-a1/'+mode+'/RESULT.json')
 query=jsonblock('## ACTUAL direct-query receipt '+mode+'/QUERY-RESULT.json','actual-check-a1/'+mode+'/QUERY-RESULT.json')
 assert step['exitCode']==0 and step['headAfter']==HEAD and step['treeAfter']==TREE and step['statusAfter']==''
 assert query['exitCode']==0 and query['stderrEmpty'] and query['completeJSONEqualsSpecifiedContract']
 assert query['stdoutSha256']==sha((A/'actual-check-a1'/mode/'QUERY-STDOUT.json').read_bytes())
 start,end,body=block('## ACTUAL test stdout '+mode+'/STDOUT.log','text')
 raw=(A/'actual-check-a1'/mode/'STDOUT.log').read_bytes()
 assert body==raw
 record('complete-test-stdout','actual-check-a1/'+mode+'/STDOUT.log',start,end,body,raw_exact=True)
 assert step['stdoutSha256']==sha(raw)
 err=(A/'actual-check-a1'/mode/'QUERY-STDERR.log').read_bytes();assert err==b''
 # Original execution evidence remains independently stored; compare bytes, no rerun.
 original=R/'public-capabilities-check-cycle/actual-check-a1'/mode
 for name in ['QUERY-STDOUT.json','QUERY-RESULT.json','RESULT.json','STDOUT.log','QUERY-STDERR.log']:
  assert (A/'actual-check-a1'/mode/name).read_bytes()==(original/name).read_bytes()
 modes.append({'mode':mode,'complete_actual_equals_complete_expected':True,'top_level_keys':list(actual),'semantic_sha256':sha(canonical(actual)),'test_stdout':raw.decode(),'step':step,'query':query})
plan=jsonblock('## ACTUAL original CHECK-PLAN.json','actual-check-a1/CHECK-PLAN.json')
result=jsonblock('## ACTUAL original RESULT.json','actual-check-a1/RESULT.json')
cleanup=jsonblock('## ACTUAL original CLEANUP.json','actual-check-a1/CLEANUP.json')
for name in ['CHECK-PLAN.json','RESULT.json','CLEANUP.json']:
 assert (A/'actual-check-a1'/name).read_bytes()==(R/'public-capabilities-check-cycle/actual-check-a1'/name).read_bytes()
assert plan['head']==result['head']==HEAD and plan['tree']==result['tree']==TREE
assert result['allStepsCompleted'] and result['allExitZero'] and not result['temporaryRootExistsAfter'] and not cleanup['existsAfter']
query_name='public_query_distinguishes_profiles_build_consumers_and_unchecked_targets'
core_name='reported_core_profiles_select_real_source_only_cli_consumers'
tests=re.findall(rb'#\[test\]\s*fn\s+(\w+)\(',sources['crates/mori/tests/capabilities.rs'])
assert [x.decode() for x in tests]==[query_name,core_name]
cargo='/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo'
common=[cargo,'test','--manifest-path',str(K/'Cargo.toml'),'--frozen','-j1','-p','mori','--no-default-features']
assert plan['steps'][0]['argv']==common+['--test','capabilities','--','--test-threads=1']
assert plan['steps'][1]['argv']==common+['--features','source6-expansion,core2-disjoint-draft','--test','capabilities','--','--exact',query_name,'--test-threads=1']
for s in plan['steps']:assert s['wallSeconds']==90 and s['killGraceSeconds']==5
q=plan['actualQueryPerFeature'];assert q['wallSeconds']==10 and q['killGraceSeconds']==5
assert q['argv']==[plan['ownedTemporaryRoot']+'/target/debug/mori','--color','never','capabilities','--json']
for m in modes:assert m['query']['argv']==['/usr/bin/timeout','--signal=TERM','--kill-after=5s','10s']+q['argv']
assert '2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out' in modes[0]['test_stdout']
assert '1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out' in modes[1]['test_stdout']
contexts=[]
for row in mapping['full_physical_context_files']:
 path=row['path'];raw=blob(BASE,path)
 assert raw==blob(HEAD,path)==(A/'source/e4d-context'/path).read_bytes()
 assert sha(raw)==row['sha256']
 contexts.append({'path':path,'bytes':len(raw),'sha256':sha(raw),'git_base_equals_final_equals_physical':True})
# Verify every supplied physical source excerpt in order, including clamped requested end ranges.
pos=p.index(b'## Exact E4D curated-metadata context excerpts')
for row in mapping['embedded_context']:
 path=row['path'];raw=blob(BASE,path);lines=raw.splitlines(keepends=True)
 assert sha(raw)==row['full_sha256']
 for n,(a,b) in enumerate(row['embedded_line_ranges']):
  b=min(b,len(lines));expected=b''.join(lines[a-1:b])
  marker=(('### E4D '+path+'; full SHA256 '+row['full_sha256']+'; lines '+str(a)+'–'+str(b)) if n==0 else ('E4D lines '+str(a)+'–'+str(b)+' (same file/pin):')).encode()
  h=p.index(marker,pos);start=p.index(b'```text\n',h)+len(b'```text\n');end=p.index(b'\n```',start)
  assert p[start:end]==expected,path
  record('exact-e4d-excerpt',path,start,end,expected,revision=BASE,line_start=a,line_end=b,full_file_sha256=sha(raw))
  pos=end
# The source file bodies and all JSON objects cover every delivered fenced block.
assert len(index)==len(fences),(len(index),len(fences))
requirements=[]
for text in ['Analyze the full code and actual evidence; the anchor table and contract equality do not replace review.',
 'Each substantive finding must cite an existing supplied file/line or actual JSON pointer AND quote the supporting excerpt/value.',
 'Distinguish source fact, observed result, inference and unverified claim;',
 'Include concrete reasoned checks for each full changed file and each actual mode',
 'FileReads/coverage self-reports are not attestations.',
 'Current LSP/MCP are stubs.',
 'later actual editor integration requires explicit metadata/profile/output/limitation, inspection-basis and test updates with fresh evidence.',
 'catalogue.sourceRevision is E4D inspection provenance, not binary SHA; binarySourceRevision is null.',
 'Unembedded: original819-entry pin manifest, Cargo stderr and context outside stated excerpt ranges',
 'root decides adoption.']:
 b=text.encode();start=p.index(b);requirements.append({'text':text,'start_byte_inclusive':start,'end_byte_exclusive':start+len(b)})
schema=jread(A/'PROPOSED-OUTPUT-SCHEMA.json');schema_raw=(A/'PROPOSED-OUTPUT-SCHEMA.json').read_bytes()
assert schema['properties']['reviewToken']=={'type':'string'} and 'reviewToken' in schema['required']
assert schema['properties']['head']['const']==HEAD and schema['properties']['tree']['const']==TREE
assert schema['properties']['concreteChecks']['minItems']==6
assert set(schema['properties']['findings']['items']['required'])=={'severity','factOrInference','citation','finding','userVisibleConsequence','minimalCorrection'}
for property in ['concreteChecks','findings']:
 c=schema['properties'][property]['items']['properties']['citation']
 assert set(c['required'])=={'path','lineOrJsonPointer','exactExcerptOrValue','deliveryScope'}
after=snapshot();assert before==after and (A/'MEMBERSHIP.json').read_bytes()==mb
assert set(x['path'] for x in before)==set(str(x.relative_to(A)) for x in A.rglob('*') if x.is_file() and x.name!='MEMBERSHIP.json')
dump('OFFSETS-AND-HASHES.json',{'offset_unit':'UTF-8 bytes, zero based, [start,end)','input_sha256':sha(p),'input_bytes':len(p),'fenced_blocks':len(fences),'blocks':index,'instruction_anchors':requirements})
dump('FROZEN-MEMBERSHIP-RECHECK.json',{'membership_sha256':sha(mb),'frozen_file_count':72,'same_before_after':True,'files':after,'opaque_hash_only_scope':['preflight-provenance/*','unembedded Cargo logs','historical patches'],'no_provider_export_content_interpreted':True})
dump('SOURCE-CONTEXT-RECHECK.json',{'head':HEAD,'tree':TREE,'context_base':BASE,'complete_context_files':contexts})
dump('TESTS-AND-RESULTS.json',{'scope':'Static inspection of original captured execution evidence; no execution in this custody task','test_functions':[x.decode() for x in tests],'check_plan':plan,'modes':modes,'final_result':result,'cleanup':cleanup})
dump('EXTERNAL-SCHEMA-TOKEN-INSERTION.json',{'kind':'UNAPPLIED_TEMPLATE_NOT_A_DISPATCH','input_schema_sha256':sha(schema_raw),'precondition':{'pointer':'/properties/reviewToken','value':{'type':'string'}},'json_patch_template':[{'op':'add','path':'/properties/reviewToken/const','value':'<ROOT_CHOSEN_UNIQUE_PER_JOB_REVIEW_TOKEN>'}],'required_external_alignment':['Choose actual per-job token externally; do not use this placeholder literally.','In a separately derived dispatch schema set reviewToken.const to that token.','Use the identical token in runner task reviewToken and explicit dispatch instructions.','Pin/hash derived prompt and schema and preserve frozen A originals.'],'schema_limitations':['minItems=6 does not enforce six distinct scopes; root must inspect that checks cover four files plus two actual modes.','JSON Schema requires citation/reasoning strings but cannot verify truthful source anchoring or substantive reasoning.'],'source_original_modified':False})
dump('CUSTODY-REPORT.json',{'kind':'STATIC_CUSTODY_FACT_CHECK_NOT_SOURCE_OR_RESULT_APPROVAL','input':str(A/'REVIEW-INPUT.md'),'input_bytes':len(p),'input_sha256':sha(p),'membership_sha256':sha(mb),'head':HEAD,'tree':TREE,'complete_changed_sources_exact_git_physical_inline':4,'complete_actual_expected_json_pairs_semantically_equal':2,'complete_json_objects_verified':len(jsonrows),'all_fenced_blocks_checked':len(fences),'physical_context_files_exact_base_and_final':len(contexts),'frozen72_unchanged':True,'required_external_action':'Insert unique reviewToken const into separately derived dispatch schema, align task/prompt, pin derived artifacts.','reasoning_instructions_exist':True,'schema_distinct_scope_and_truthfulness_require_root_inspection':True,'runtime_or_provider_executed':False,'provider_or_model_identity_attested':False,'source_or_result_approval_given':False,'acceptance_exits_unchanged':'U/M/MC/SP/G','read_limitations':['Full prompt decoded and all fenced content compared.','28 physical context files compared bytewise to pinned Git; only supplied excerpt content is delivered inline.','Provider provenance exports and unembedded logs were hash-only membership checks, not semantic reads or review evidence.','Actual command results are captured root evidence, not fresh custody execution.']})
print(json.dumps({'static_custody':'MATCH','files':len(before),'fences':len(fences),'source_files':len(sources),'complete_json_objects':len(jsonrows),'output':str(OUT)},indent=2))

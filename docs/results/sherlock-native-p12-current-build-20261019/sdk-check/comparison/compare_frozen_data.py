from pathlib import Path
import json,hashlib,re,datetime,os
S=Path(__file__).resolve().parent.parent
D=Path(__file__).resolve().parent
R=S.parent
O=R/'native-literal-independent-positive-oracle-20261018'
C=R/'native-literal-independent-A102-context-rebind-40d41868-20261019'

def pin(p):
 b=p.read_bytes();return dict(path=str(p),bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
def check(row):
 a=pin(Path(row['path']));assert a['bytes']==row['bytes'] and a['sha256']==row['sha256'],row['path'];return a
def load(p):return json.loads(p.read_bytes())
def uint(s):
 assert isinstance(s,str) and re.fullmatch(r'0|[1-9][0-9]*',s),s
 n=int(s);assert 0<=n<2**128;return n
def normalize(v):
 if isinstance(v,str):return uint(v)
 if isinstance(v,list):return [normalize(x) for x in v]
 raise AssertionError('Only explicit unsigned decimal strings and arrays in actual images')
def same(a,b):return json.dumps(a,sort_keys=True,separators=(',',':'))==json.dumps(b,sort_keys=True,separators=(',',':'))

freeze=load(S/'ARTIFACT-FREEZE.json');assert pin(S/'ARTIFACT-FREEZE.json')['sha256']=='73df707b33412918cdb666f51ef13485f6eb9fe97ab5154d4dde362847f3afb9'
assert freeze['threeFilesOutdirAndParentFsyncCompletedBeforeManifest'] is True and (D/'ROOT-UNBLIND-ORDER.json').is_file()
actualpins=[check(r) for r in freeze['artifactFiles']]
safe=load(C/'SAFE-BINDINGS.json');assert pin(C/'SAFE-BINDINGS.json')['sha256']=='551924464b1dda58cfe096854b199202148d30abe66257893ed01e23d83a0c86'
for key in ['original_oracle_freeze','private_materialization_freeze','opaque_complete_statement','opaque_context_only','numeric_expectations_unchanged']:check(safe[key])
orig=load(O/'EXPECTATION-FREEZE.json');oldpins=[check(r) for r in orig['files']]
priv=load(C/'PRIVATE-MATERIALIZATION-FREEZE.json');privatepins=[check(r) for r in priv['private_files']]
a=load(S/'actual-output-01/a102_statement_and_transition.json');ir=load(S/'actual-output-01/a102_local_public.zkir.json')
expected=load(C/'private/expected-statement-2077.json');layout=load(O/'LAYOUT-2077.json');structure=load(O/'SDK-EXPECTED-STRUCTURE.json');context=load(C/'private/CONTEXT-MATERIALIZED.json');ident=load(O/'INPUT-IDENTITIES.json')
assert len(expected)==2077 and all(type(n) is int and 0<=n<2**128 for n in expected)
assert len(a['public_values_dec'])==len(a['lanes'])==len(layout['lanes'])==2077
actual=[uint(v) for v in a['public_values_dec']]
rows=[]
for i,(got,want,al,el) in enumerate(zip(actual,expected,a['lanes'],layout['lanes'],strict=True)):
 spec=el['lane'];maximum=None if spec['maximum'] is None else str(spec['maximum'])
 expectedmeta={'index':i,'path':el['path'],'name':spec['name'],'bits':spec['bits'],'boolean':spec['boolean'],'maximum_dec':maximum,'value_dec':str(got)}
 rows.append({'index':i,'expectedValueDec':str(want),'actualValueDec':str(got),'valueEqual':got==want,'metadataEqual':same(al,expectedmeta),'expectedMetadata':expectedmeta,'actualMetadata':al})
checks={'all2077StatementValues':all(r['valueEqual'] for r in rows),'all2077CompleteLaneMetadata':all(r['metadataEqual'] for r in rows),'contextTwo':same(a['context_limbs_dec'],load(C/'private/expected-context-two-lanes.json')),'noncontext2075':actual[2:]==load(O/'expectations/statement-noncontext-2075.json')}
images={}
for key in ['program','pre','post']:
 e=load(O/f'expectations/{key}-image.json');g=a[key];assert set(g)=={'root','tables'};assert len(g['tables'])==len(e['table_order'])
 got={'root':normalize(g['root']),'table_order':e['table_order'],'tables':dict(zip(e['table_order'],[normalize(x) for x in g['tables']],strict=True))}
 flat=got['root']+[cell for table in got['tables'].values() for row in table for cell in row]
 images[key]={'completeRootAndEveryTableEqual':same(got,{k:e[k] for k in ['root','table_order','tables']}),'wholeFlatEqual':same(flat,e['flat']),'lanes':len(flat)}
 checks[key+'CompleteImage']=all(images[key][k] for k in ['completeRootAndEveryTableEqual','wholeFlatEqual'])
checks['producerPostEnvelope685']=actual[1392:]==load(O/'expectations/return-projection-685.json')
program=(S/'actual-output-01/a102_program_l4.bin').read_bytes();checks['programAll707Bytes']=program==(O/'inputs/program-source.l4.bin').read_bytes() and program.hex()==a['program_l4_bytes_hex'];checks['artifactProgramDigest']=hashlib.sha256(program).hexdigest()==a['digests']['program_source_sha256']==ident['program_source_identity']['sha256'];checks['artifactZKIRDigest']=pin(S/'actual-output-01/a102_local_public.zkir.json')['sha256']==a['digests']['zkir_sha256'];checks['referenceClaimIdentity']=a['digests']['reference_source_sha256']==ident['reference_source']['sha256'];checks['KClaimIdentity']=a['digests']['k_source_sha256']==ident['K_SOURCE_SHA256']
checks['layoutDigest']=a['digests']['layout_hash']==layout['layout_hash_sha256']==context['layout_digest_sha256'];checks['relationDigest']=a['digests']['relation_hash']==context['relation_digest_sha256'];checks['contextDigest']=a['digests']['context_hash']==context['context_digest_sha256']
checks['programCapacities']=same(a['capacities']['program'],layout['program_capacities']);checks['stateCapacities']=same(a['capacities']['state'],layout['state_capacities']);checks['stackCapacity']=uint(a['capacities']['stack_limit'])==layout['stack_capacity']
public=[v for v in ir['instructions'] if v['op']=='public_input'];impacts=[v for v in ir['instructions'] if v['op']=='impact'];outputs=[v for v in ir['instructions'] if v['op']=='output'];ids=['%sherlock.a102.statement.'+str(i) for i in range(2077)]
checks['allPublicInputIDsInOrder']=len(public)==structure['PublicInput_instruction_count'] and all(v['output']==want and v['guard'] is None for v,want in zip(public,ids,strict=True));checks['allImpactIDsInOrder']=len(impacts)==structure['Impact_instruction_count'] and all(v['inputs']==[want] and v['guard']=='0x01' for v,want in zip(impacts,ids,strict=True));checks['oneOutputAll685IDs']=len(outputs)==structure['Output_instruction_count'] and outputs[0]['vals']==[ids[i] for i in structure['Output_operand_statement_indices']];checks['ordinaryIRInputsZero']=len(ir['inputs'])==structure['ordinary_ir_inputs_length'];checks['all685NativeOutputTypes']=len(ir['outputs'])==structure['ir_outputs_type_count'] and all(v=='Scalar<BLS12-381>' for v in ir['outputs']);checks['communicationsOff']=ir['do_communications_commitment'] is structure['do_communications_commitment']
checks['countsVsIR']=a['counts']['ir_total_instructions']==len(ir['instructions']) and a['counts']['ir_public_input_instructions']==len(public) and a['counts']['ir_impact_instructions']==len(impacts) and a['counts']['ir_outputs']==len(ir['outputs']);checks['schemaProfileBinding']=a['schema']=='moriarty-sherlock-a102-nonproving-check-output/1' and a['format']==context['fixed_strings'][0] and a['binding']==context['fixed_strings'][1] and a['profile']==context['fixed_strings'][2] and a['scope']==context['fixed_strings'][3] and a['ledger_revision']==context['fixed_strings'][4] and a['zkir_version']==context['fixed_strings'][5]
report={'status':'ALL_SCOPED_INDEPENDENT_EQUALITIES_PASS' if all(checks.values()) else 'SCOPED_COMPARISON_MISMATCH','UTC':datetime.datetime.now(datetime.timezone.utc).isoformat(),'actualFreeze':pin(S/'ARTIFACT-FREEZE.json'),'actualPins':actualpins,'originalOracleFreeze':check(safe['original_oracle_freeze']),'contextFreeze':check(safe['private_materialization_freeze']),'completeStatementExpected':check(safe['opaque_complete_statement']),'originalAll15PinsChecked':oldpins,'privateAllPinsChecked':privatepins,'checks':checks,'images':images,'all2077Rows':rows,'noDroppedRows':len(rows)==2077,'noSDKOrProducerOrSubjectExecution':True,'qualifications':['finite fixture equality, not general semantic correspondence or soundness','685 values are producer post/envelope projection, not SDK check outputs','source identity strings remain caller claims, not authenticated source proofs','notproof/noSRS/keygen/nativeverify/stagenet'],'mismatches':[r for r in rows if not r['valueEqual'] or not r['metadataEqual']]}
for r in actualpins+oldpins+privatepins:check(r)
with (D/'COMPLETE-REPORT.json').open('x') as f:json.dump(report,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
print(json.dumps({'status':report['status'],'allChecks':checks,'fullReport':pin(D/'COMPLETE-REPORT.json'),'statementRows':len(rows),'mismatchCount':len(report['mismatches']),'images':images},indent=2))
assert all(checks.values()) and not report['mismatches']

from pathlib import Path,PurePosixPath
import ast,copy,hashlib,json,collections
D=Path(__file__).resolve().parent;R=D.parent;S=R/'53-r2-integrated-source-candidate/source';OLD=Path('/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/a102-prerequisite-01/decision-packet-01');inputs=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def load(p,expected=None):
 b=p.read_bytes();h=sha(b)
 if expected:assert h==expected,(str(p),h,expected)
 inputs.append({'path':str(p),'bytes':len(b),'sha256':h});return json.loads(b)
def save(n,v):(D/n).write_text(json.dumps(v,indent=2)+'\n')
base=load(S/'config.example.json','4b12357a4af85553be2ca833760d3bd4a08bb9ad4f5585823d63e800f0ff42cb');draft=copy.deepcopy(base)
adoption=load(R/'157-prospective-native-policy-adoption/ROOT-PROSPECTIVE-ADOPTION.json');assert adoption['actualStatus']['nativeAuthority']==0
summary=load(OLD/'FACTUAL-MAPS-SUMMARY.json','587b0376c8d45f076e127a6b2659db27e2378290fbe0fd7f7084b152d180af5e');maps=load(OLD/'FACTUAL-MAPS.json','f123b2f4532cee2d53424bf9d5bd1d0c7191e3a2dd3419023915a212f71c3d43')
assert summary['bytes']==582425 and len(maps['registry_roots'])==423 and len(maps['includes'])==40
am_path=R/'21-native-build-path-review/snapshots/12-AMENDMENTS.proposed.json';am=load(am_path,'3676da94b6ad533a2276271c55c3c47b8c4473170b1200cf0dcae2b362fbda5d')
embedded_path=Path(summary['inputs']['embedded']['path']);emb=load(embedded_path,'ac397cfd5e0952b01ae1b892997808e95e13d3ac10ecff2e90418e484fab57d7')
resource=load(R/'21-native-build-path-review/snapshots/11-HISTORICAL-RESOURCE03-MEMBER-PROJECTION.json')['selected_original_members']
for p in [R/'98-host-readonly-facts/FACTS.json',R/'98-host-readonly-facts/SELF-CGROUP.json',R/'109-readonly-user-cgroup-facts/FACTS.json',R/'109-readonly-user-cgroup-facts/CONTROLLERS.json']:load(p)
for p in [R/'134-final-r2b-policy-freeze/SOURCE-FREEZE.json',R/'149-r2b-final-security-A-literal-correction/FINAL-RESPONSE.txt',R/'147-r2b-security-B-literal-correction/FINAL-RESPONSE.txt']:load(p)
gpath=S/'bounded_sdk_compile.py';graw=gpath.read_bytes();assert sha(graw)=='94810a65a2a68563300edefe34c76447ad9cc3118c07c8ae8f67607b88ac1fd7';inputs.append({'path':str(gpath),'bytes':len(graw),'sha256':sha(graw)});tree=ast.parse(graw);constants={}
for n in tree.body:
 if isinstance(n,ast.Assign) and len(n.targets)==1 and isinstance(n.targets[0],ast.Name) and n.targets[0].id in ('REQUIRED_MASKS','GIT_SYMLINKS'):
  constants[n.targets[0].id]=ast.literal_eval(n.value.args[0] if isinstance(n.value,ast.Call) else n.value)
git=maps['git_checkouts_all_blob_rows_including_mode_120000'];assert [len(g['files']) for g in git]==[1332,1174]
links={(g['commit'],x['relative'],x['blob']) for g in git for x in g['files'] if x['mode']=='120000'};assert links==set(constants['GIT_SYMLINKS']) and len(links)==16
for g in git:
 assert all(set(x)=={'relative','blob','mode'} for x in g['files']) and len({x['relative'] for x in g['files']})==len(g['files'])
 link_names={x['relative'] for x in g['files'] if x['mode']=='120000'};assert not any(str(parent) in link_names for x in g['files'] for parent in PurePosixPath(x['relative']).parents)
# Metadata-only row equality against current approved_includes() construction;
# never import the guard or open any archive/cache/Git/source/payload object.
roots={g['commit']:g['relative_root'] for g in git};expected={}
for item in emb['inputs']:
 if 'git_tree_entry' in item:
  commit=item['git_commit'];rel=item['git_relative_target'];path=str(PurePosixPath(roots[commit])/rel);expected[path]={'relative':path,'algorithm':'git-blob','digest':item['git_tree_entry']['blob'],'source_identifier':commit+':'+rel}
 elif 'archive_member' in item:
  member=item['archive_member'];parts=PurePosixPath(member).parts;matches=[x for x in maps['registry_roots'] if Path(x['source_relative']).name==parts[0] and x['package_checksum']==item['locked_archive_sha256']];assert len(matches)==1;x=matches[0];path=str(PurePosixPath(x['source_relative']).joinpath(*parts[1:]));expected[path]={'relative':path,'algorithm':'archive-member','digest':x['package_checksum'],'source_identifier':Path(x['archive']).name+':'+member}
 else:assert item.get('enclosing_item',{}).get('name')=='exptfile' and 'conservative_static_tree_entries' in item
for item in am['AM-3']['digest_files']:
 rel=item['path'];commit=am['AM-3']['git_commit'];path=str(PurePosixPath(roots[commit])/rel);expected[path]={'relative':path,'algorithm':'git-blob','digest':item['git_blob'],'source_identifier':commit+':'+rel}
assert len(expected)==40 and {x['relative']:x for x in maps['includes']}==expected
# Preserve every existing numeric cap/bound; do not choose operator paths or grants.
draft['bindings']['runner']={'path':str(gpath),'sha256':sha(graw)};draft['bindings']['amendments']={'path':str(am_path),'sha256':'3676da94b6ad533a2276271c55c3c47b8c4473170b1200cf0dcae2b362fbda5d'};draft['masks']=sorted(constants['REQUIRED_MASKS']);draft['git_checkouts']=copy.deepcopy(git);draft['registry_roots']=copy.deepcopy(maps['registry_roots']);draft['includes']=copy.deepcopy(maps['includes'])
for key in draft['paths']:
 if key!='worktree':draft['paths'][key]=None
for q in draft['quota_mounts'].values():q['mountpoint']=None
assert draft['source_authoring_only'] is True and draft['external_tier_consent']=={'tier1':False,'tier2':False}
assert draft['limits']==base['limits'] and draft['cgroup']==base['cgroup']
for key in ('source_home','target_tmp','packet'):
 assert {k:v for k,v in draft['quota_mounts'][key].items() if k!='mountpoint'}=={k:v for k,v in base['quota_mounts'][key].items() if k!='mountpoint'}
assert draft['total_wall_seconds']==base['total_wall_seconds'] and draft['supervisor_cpu_seconds']==base['supervisor_cpu_seconds'];assert draft['bindings']['config_inputs'] is None and draft['old_home_stat_manifest'] is None and draft['expected_generated_files'] is None
save('config.integration.proposed.json',draft)
rows=[]
def classify(pointer,value):
 if value is not None:
  if pointer in ('/git_checkouts','/registry_roots','/includes'):return 'FILLED_EXACT_SAVED_METADATA_NOT_LIVE_OBSERVATION','FACTUAL-MAPS member; static current-row equality checks only'
  if pointer.startswith('/bindings/runner') or pointer.startswith('/bindings/amendments') or pointer=='/masks':return 'FILLED_CURRENT_SOURCE_METADATA','Current frozen source/owner amendment; no runtime authority'
  return 'FILLED_EXISTING_SOURCE_VALUE_NOT_FRESHLY_REOBSERVED','Preserved config.example.json value; not current host/source-content observation or grant'
 if pointer=='/bindings/config_inputs':return 'MISSING_FINAL_ASSEMBLY_VALUE','Actual consent() projection removes review_receipts and bindings.config_inputs; not old FACTUAL-MAPS digest'
 if pointer.startswith('/review_receipts'):return 'MISSING_CONSUMABLE_RECEIPT','Prospective votes/adoption exist; exact actual-consumer receipt paths/hashes still unminted; no review rerun requested'
 if pointer=='/expected_generated_files':return 'MISSING_NONFABRICABLE_INVENTORY','Existing maps says OWNER_SELECTION_REQUIRED; exact saved GIT_INVENTORY/ARCHIVES_START metadata must establish membership or remain future observation'
 if pointer=='/old_home_stat_manifest':return 'MISSING_FRESH_STAT_ONLY_OBSERVATION','No old-home/cache stat/read/probe performed; fresh observed manifest path/hash required under applicable authority'
 if pointer.startswith('/paths/') or pointer.endswith('/mountpoint') or pointer=='/cgroup/envelope_path':return 'MISSING_FRESH_OWNER_ROOT_SELECTED_PATH','Old example/operator paths deliberately unselected; no consumed symlink grant or stopped attempt selected'
 if pointer.startswith('/quota_mounts/') and pointer.rsplit('/',1)[1] in ('mount_id','device'):return 'MISSING_ACTUAL_Q_OBS','Kernel mount identity after separately authorized real provisioning; never invent'
 if pointer.startswith('/cgroup/') or pointer.startswith('/limits/') or pointer.startswith('/quota_mounts/') or pointer in ('/total_wall_seconds','/supervisor_cpu_seconds'):return 'UNFILLED_EXISTING_OWNER_PROPOSAL_VALUE_NOT_SELECTED','Known historical proposal may supply value; no numeric/cap/bound selected or changed in this integration draft'
 return 'MISSING_REQUIRED_VALUE','No value invented'
def walk(value,pointer=''):
 if isinstance(value,dict):
  for k,x in value.items():walk(x,pointer+'/'+k)
 elif isinstance(value,list):
  status,reason=classify(pointer,value);rows.append({'field':pointer,'status':status,'value_or_entry_count':len(value),'reason':reason})
 else:
  status,reason=classify(pointer,value);row={'field':pointer,'status':status,'value':value,'reason':reason}
  if value is None and pointer in resource['missing_field_classification']:row['existing_historical_owner_source_text']=resource['missing_field_classification'][pointer]
  rows.append(row)
walk(draft);save('FIELD-STATUS.json',{'scope':'Single exact config-field status table; arrays collapsed with exact count and rows preserved verbatim in config draft; no ready/admission/security verdict','rows':rows})
counts=dict(collections.Counter(x['status'] for x in rows));save('METADATA-READBACK.json',{'status':'SOURCE_ONLY_METADATA_CONVERSIONS_CHECKED_NOT_GUARD_OR_SECURITY_REVIEW','git_total_rows':[1332,1174],'git_mode120000_rows':16,'git_literal_row_set_equals_current_guard':True,'registry_rows':423,'includes_rows':40,'include_row_equality_with_current_owner_metadata':True,'caps_bounds_limits_unchanged_from_current_example':True,'future_paths_not_selected':True,'source_authoring_only':True,'external_tier_consent':draft['external_tier_consent'],'config_inputs_binding':None,'final_config_projection_hash':'UNKNOWN_NOT_MINTED','field_status_counts':counts,'old_symlink_grant_selected_or_reused':False,'guard_imports_tests_host_actions_SDKpayload_reads_provider_native_operations':0})
save('INPUT-MEMBERSHIP.json',{'scope':'Existing public source/config/inventory metadata and saved facts only; no archive/cache/SDK/private payload/subject source bytes','files':inputs,'native_authority':0,'provider_calls':0})
print(json.dumps({'config_draft_bytes':(D/'config.integration.proposed.json').stat().st_size,'field_rows':len(rows),'status_counts':counts,'metadata_checks':'PASS; no guard import/runtime/host action'},indent=2))

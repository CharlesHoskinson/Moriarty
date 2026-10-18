"""Normal argv/config test SOURCE ONLY. Not imported or executed in authoring."""
import copy
import json
from pathlib import Path
import unittest
import bounded_sdk_compile as runner

class ConfigAndArgv(unittest.TestCase):
    def test_exact_backend_target_is_distinct_from_cli_obligation(self):
        self.assertEqual(runner.cargo_argv('F'), [runner.CARGO, 'test', '--no-run', '--frozen', '-p', 'mori-backend', '--lib', '-j4'])
        self.assertEqual(runner.CLI_OBLIGATION, ('test','--no-run','--frozen','-p','mori','--test','stage_test_public','-j4'))
        self.assertNotEqual(tuple(runner.cargo_argv('F')[1:]), runner.CLI_OBLIGATION)

    def test_exact_fetch_and_metadata_argv(self):
        self.assertEqual(runner.cargo_argv('B'), [runner.CARGO,'fetch','--frozen'])
        self.assertEqual(runner.cargo_argv('E'), [runner.CARGO,'metadata','--frozen','--format-version','1','--filter-platform','x86_64-unknown-linux-gnu'])

    def test_example_has_no_execution_authority(self):
        c=json.loads((Path(__file__).parent/'config.example.json').read_text())
        with self.assertRaises(runner.Refusal):runner.validate(c)
        self.assertTrue(c['source_authoring_only'])
        self.assertFalse(c['external_tier_consent']['tier1'])
        self.assertFalse(c['external_tier_consent']['tier2'])

    def test_unknown_top_level_field_is_rejected(self):
        c=json.loads((Path(__file__).parent/'config.example.json').read_text())
        c['invented_override']=True
        with self.assertRaisesRegex(runner.Refusal,'schema keys'):runner.validate(c)

    def test_null_and_unknown_nested_settings_are_refused(self):
        with self.assertRaises(runner.Refusal):runner.no_null({'wall_seconds':None})
        with self.assertRaises(runner.Refusal):runner.exact({'wall_seconds':1,'unlimited':True},'wall_seconds','limits')

    def test_environment_has_only_owner_keys(self):
        c={'paths':{'source_home':'/owned/source-home','empty_home':'/owned/empty-home','tmp':'/owned/tmp','target':'/owned/target'}}
        self.assertEqual(runner.cargo_env(c,'B'),{'PATH':runner.CARGO_PATH,'CARGO_HOME':'/owned/source-home','HOME':'/owned/empty-home','TMPDIR':'/owned/tmp','LANG':'C'})
        self.assertEqual(set(runner.cargo_env(c,'F')),{'PATH','CARGO_HOME','HOME','TMPDIR','LANG','CARGO_TARGET_DIR'})

class NormalAssociations(unittest.TestCase):
    def test_depinfo_retains_explicit_output_targets(self):
        text='/owned/target/debug/build/pkg-123/build_script_build-456: /owned/source/pkg/build.rs\n'
        rows=runner.parse_depinfo(text,Path('/checkout'))
        self.assertEqual(rows,[({'/owned/target/debug/build/pkg-123/build_script_build-456'}, {'/owned/source/pkg/build.rs'})])

    def test_depinfo_multiple_targets_and_escaped_space(self):
        text='/owned/a /owned/b: /source/a\\ b.rs /source/c.rs\n'
        self.assertEqual(runner.parse_depinfo(text,Path('/checkout')),[({'/owned/a','/owned/b'},{'/source/a b.rs','/source/c.rs'})])

    def test_staging_categories_keep_exact_destination_classes(self):
        prefix='index.crates.io-1949cf8c6b5b557f'
        rows=[
            {'category':'registry-index','source':'/home/charl/.cargo/registry/index/'+prefix+'/config.json','destination_relative_to_future_home':'registry/index/'+prefix+'/config.json'},
            {'category':'registry-archive-container','source':'/home/charl/.cargo/registry/cache/'+prefix+'/x-1.0.0.crate','destination_relative_to_future_home':'registry/cache/'+prefix+'/x-1.0.0.crate'},
            {'category':'git-db-metadata','source':'/home/charl/.cargo/git/db/midnight-ledger-b2f9c59d942dfdca/HEAD','destination_relative_to_future_home':'git/db/midnight-ledger-b2f9c59d942dfdca/HEAD'},
        ]
        for row in rows:runner.staging_destination(row,{'acquired_archives':[]})


class NormalCopyMountArguments(unittest.TestCase):
    def test_example_enumerates_old_home_and_all_five_acquired_files(self):
        c=json.loads((Path(__file__).parent/'config.example.json').read_text())
        self.assertEqual(len(c['source_copy_mounts']),6)
        self.assertEqual(set(c['source_copy_mounts']),{'/home/charl/.cargo'} | {p['path'] for p in c['acquired_archives']})

    def test_a0_and_a_restore_the_six_sources_read_only(self):
        from unittest.mock import patch
        c=json.loads((Path(__file__).parent/'config.example.json').read_text())
        c['masks']=sorted(runner.REQUIRED_MASKS)
        c['review_receipts']={}
        c['bindings']={'runner':c['bindings']['runner']}
        c['source_manifest']={'path':'/control/checkout.json','sha256':'0'*64}
        c['old_home_stat_manifest']={'path':'/control/old-home.json','sha256':'1'*64}
        for phase in ('A0','A'):
            with patch.object(runner,'owned_paths'),patch.object(runner,'jload',return_value={'head_files':[]}):
                args=runner.sandbox_args(c,phase)
            binds=[tuple(args[i+1:i+3]) for i,x in enumerate(args) if x=='--ro-bind']
            for path in c['source_copy_mounts']:self.assertEqual(binds.count((path,path)),1)
            self.assertEqual(binds.count((c['paths']['mask_empty'],'/home/charl/.cargo')),0)


# Owner-authored contract tests T1-T7 and K1-K3. SOURCE ONLY / ALL UNRUN.
# Fixture numbers are in-memory values only, not operator settings or grants.
from contextlib import ExitStack
from types import SimpleNamespace
from unittest.mock import patch
import inspect
import stat

class CgroupFixture:
    def __init__(self):
        self.uid=1000;self.euid=1000;self.pid=123
        self.cg={'envelope_path':'/review-envelope','supervisor_leaf_name':'supervisor',
                 'phase_parent_name':'phases','envelope_memory_max':8192,
                 'envelope_pids_max':32,'envelope_cpu_max':'10000 100000'}
        self.c={'cgroup':self.cg}
        self.e=Path('/sys/fs/cgroup/review-envelope');self.l=self.e/'supervisor';self.p=self.e/'phases'
        self.text={Path('/proc/self/status'):'CapPrm:\t00000000\nCapEff:\t00000000\nCapAmb:\t00000000\n',
                   Path('/proc/self/cgroup'):'0::/review-envelope/supervisor\n'}
        self.children={self.e:{'supervisor','phases'},self.l:set(),self.p:set()}
        self.stats={};self.writable={};self.writes=[];self.removed=[];self.mkdirs=[]
        self.mounts=[('/sys/fs/cgroup','cgroup2')]
        for p in [Path('/sys/fs/cgroup'),self.e,self.l,self.p]:self.setstat(p,stat.S_IFDIR|0o755,0)
        self.setstat(self.p,stat.S_IFDIR|0o755,self.uid);self.writable[self.p]=True
        for p in [self.e,self.l,self.p]:self.text[p/'cgroup.type']='domain\n'
        self.text[self.e/'cgroup.procs']='';self.text[self.l/'cgroup.procs']=str(self.pid)+'\n';self.text[self.p/'cgroup.procs']=''
        self.text[self.e/'cgroup.subtree_control']='cpu memory pids\n';self.text[self.l/'cgroup.subtree_control']='';self.text[self.p/'cgroup.subtree_control']='cpu memory pids\n'
        self.text[self.p/'cgroup.controllers']='cpu memory pids\n'
        for k,v in [('memory.max',8192),('memory.swap.max',0),('pids.max',32),('cpu.max','10000 100000')]:
            self.text[self.e/k]=str(v)+'\n';self.setstat(self.e/k,stat.S_IFREG|0o444,0)
        self.writable[self.e/'cgroup.procs']=True;self.setstat(self.p/'cgroup.kill',stat.S_IFREG|0o200,self.uid)
        self.new_type='domain';self.exists_on_mkdir=False
    def setstat(self,p,mode,uid):self.stats[p]=(SimpleNamespace(st_mode=mode,st_uid=uid,st_dev=17),p)
    def mkdir(self,p):
        if self.exists_on_mkdir:raise FileExistsError(str(p))
        self.mkdirs.append(p);self.setstat(p,stat.S_IFDIR|0o755,self.uid)
        self.text[p/'cgroup.type']=self.new_type;self.text[p/'cgroup.controllers']='cpu memory pids';self.text[p/'cgroup.procs']='';self.text[p/'cgroup.events']='populated 0\n'
        self.setstat(p/'cgroup.kill',stat.S_IFREG|0o200,self.uid)
    def write(self,p,value):self.writes.append((p,value));self.text[p]=value
    def patches(self):
        stack=ExitStack()
        for name,fn in [('_cg_text',lambda p:self.text[Path(p)]),('_cg_children',lambda p:self.children[Path(p)]),
                        ('_cg_writable',lambda p,search=False:self.writable.get(Path(p),False)),
                        ('_cg_dirstat',lambda p:self.stats[Path(p)]),('_cg_mkdir',self.mkdir),
                        ('_cg_rmdir',lambda p:self.removed.append(p)),('_cg_write',self.write),('_mount_rows',lambda:self.mounts)]:
            stack.enter_context(patch.object(runner,name,side_effect=fn))
        stack.enter_context(patch.object(runner.os,'getuid',return_value=self.uid));stack.enter_context(patch.object(runner.os,'geteuid',return_value=self.euid));stack.enter_context(patch.object(runner.os,'getpid',return_value=self.pid))
        return stack

class OwnerTopologyContract(unittest.TestCase):
    def test_t1_cgroup_v2_schema_and_owner_refusals(self):
        good=CgroupFixture().cg;runner.validate_cgroup(good)
        for key in ['parent','supervisor_expected_path','supervisor_memory_max','supervisor_pids_max','supervisor_cpu_max']:
            c=dict(good);c[key]='/old'
            with self.subTest(old_key=key),self.assertRaisesRegex(runner.Refusal,'schema keys: cgroup'):runner.validate_cgroup(c)
        for value in ['/','/a/../b','/a/','/cgroup.hidden']:
            c=dict(good,envelope_path=value)
            with self.subTest(path=value),self.assertRaises(runner.Refusal):runner.validate_cgroup(c)
        for field in ['supervisor_leaf_name','phase_parent_name']:
            for value in ['','a/b','a.b','sdk-once-a']:
                c=dict(good);c[field]=value
                with self.subTest(field=field,value=value),self.assertRaises(runner.Refusal):runner.validate_cgroup(c)
        with self.assertRaises(runner.Refusal):runner.validate_cgroup(dict(good,phase_parent_name=good['supervisor_leaf_name']))

    def test_t1_version_one_refuses_before_other_config_fields(self):
        c=json.loads((Path(__file__).parent/'config.example.json').read_text());c['version']=1
        with self.assertRaisesRegex(runner.Refusal,'mode/version'):runner.validate(c)

    def test_t2_shared_namespace_phase_derivation(self):
        c=CgroupFixture().c
        self.assertEqual(runner.cg_paths(c),('/review-envelope','/review-envelope/supervisor','/review-envelope/phases'))
        self.assertEqual(runner.phase_cgroup(c,'F'),'/review-envelope/phases/sdk-once-F')
        self.assertEqual(runner.cg_fs(runner.phase_cgroup(c,'F')),Path('/sys/fs/cgroup/review-envelope/phases/sdk-once-F'))
        # Source wiring inspection only: both consumers use the same helper.
        for fn in (runner.launch,runner.verify_worker_context):
            source=inspect.getsource(fn)
            self.assertIn("'0::'+phase_cgroup(c,phase)",source)
            self.assertNotIn("relative_to('/sys/fs/cgroup')",source)

    def test_t3_correct_process_free_envelope_and_phase_parent(self):
        f=CgroupFixture()
        with f.patches():runner.supervisor_envelope(f.c)
        self.assertEqual(f.writes,[]);self.assertEqual(f.mkdirs,[])

    def test_t4_all_owner_refusal_predicates_and_k1_k2_k3(self):
        cases=['supervisor_in_E','process_E','missing_memory','writable_cap','ancestor_not_writable',
               'extra_E_child','second_L_pid','L_child','process_P','threaded_P','invalid_P','residue_P',
               'cap_mismatch','two_cgroup_mounts','nested_mount','effective_cap','root_euid',
               'owned_readonly_cap_K1','permitted_cap_K2','ambient_cap_K2','alternate_mount_K3']
        for case in cases:
            f=CgroupFixture()
            if case=='supervisor_in_E':f.text[Path('/proc/self/cgroup')]='0::/review-envelope\n'
            elif case=='process_E':f.text[f.e/'cgroup.procs']='99\n'
            elif case=='missing_memory':f.text[f.e/'cgroup.subtree_control']='cpu pids\n'
            elif case=='writable_cap':f.writable[f.e/'memory.max']=True
            elif case=='ancestor_not_writable':f.writable[f.e/'cgroup.procs']=False
            elif case=='extra_E_child':f.children[f.e].add('other')
            elif case=='second_L_pid':f.text[f.l/'cgroup.procs']+='99\n'
            elif case=='L_child':f.children[f.l].add('other')
            elif case=='process_P':f.text[f.p/'cgroup.procs']='99\n'
            elif case=='threaded_P':f.text[f.p/'cgroup.type']='threaded\n'
            elif case=='invalid_P':f.text[f.p/'cgroup.type']='domain invalid\n'
            elif case=='residue_P':f.children[f.p].add('sdk-once-A')
            elif case=='cap_mismatch':f.text[f.e/'memory.max']='4096\n'
            elif case=='two_cgroup_mounts':f.mounts.append(('/sys/fs/cgroup','cgroup2'))
            elif case=='nested_mount':f.mounts.append(('/sys/fs/cgroup/nested','tmpfs'))
            elif case=='effective_cap':f.text[Path('/proc/self/status')]=f.text[Path('/proc/self/status')].replace('CapEff:\t00000000','CapEff:\t00000001')
            elif case=='root_euid':f.euid=0
            elif case=='owned_readonly_cap_K1':f.setstat(f.e/'memory.max',stat.S_IFREG|0o444,f.uid)
            elif case=='permitted_cap_K2':f.text[Path('/proc/self/status')]=f.text[Path('/proc/self/status')].replace('CapPrm:\t00000000','CapPrm:\t00000001')
            elif case=='ambient_cap_K2':f.text[Path('/proc/self/status')]=f.text[Path('/proc/self/status')].replace('CapAmb:\t00000000','CapAmb:\t00000001')
            elif case=='alternate_mount_K3':f.mounts.append(('/unrelated/cgroup','cgroup2'))
            with self.subTest(case=case),f.patches(),self.assertRaises(runner.Refusal):runner.supervisor_envelope(f.c)
            self.assertEqual(f.writes,[])

    def group_config(self,f):
        return dict(f.c,limits={'A':{'memory_bytes':8192,'pids':32,'cpu_quota_us':10000,'cpu_period_us':100000}})

    def test_t5_partial_create_removes_only_its_own_group(self):
        f=CgroupFixture();f.new_type='threaded'
        with f.patches():
            g=runner.Group(self.group_config(f),'A')
            with self.assertRaises(runner.Refusal):g.create()
            g.close()
        target=f.p/'sdk-once-A';self.assertEqual(f.mkdirs,[target]);self.assertEqual(f.removed,[target]);self.assertEqual(f.writes,[(target/'cgroup.kill','1')])

    def test_t5_eexist_never_kills_or_removes_foreign_group(self):
        f=CgroupFixture();f.exists_on_mkdir=True
        with f.patches():
            g=runner.Group(self.group_config(f),'A')
            with self.assertRaises(FileExistsError):g.create()
            g.close()
        self.assertFalse(g.created);self.assertEqual(f.writes,[]);self.assertEqual(f.removed,[])

    def test_t6_launch_closes_after_create_failure_without_launching(self):
        events=[]
        class FailedGroup:
            def __init__(self,c,phase):pass
            def create(self):events.append('create');raise runner.Refusal('fixture create failure')
            def close(self):events.append('close')
        c={'limits':{'A':{}},'paths':{'packet':'/fixture/packet','execution_root':'/fixture'}}
        with patch.object(runner,'Group',FailedGroup),patch.object(runner,'supervisor_envelope'),patch.object(runner,'bound_identities'),patch.object(runner,'quotas'),patch.object(Path,'is_file',return_value=True),patch.object(runner.signal,'setitimer'),patch.object(runner.subprocess,'Popen') as popen:
            with self.assertRaisesRegex(runner.Refusal,'fixture create failure'):runner.launch(c,'A','0'*64)
            popen.assert_not_called()
        self.assertEqual(events,['create','close'])

    def test_t7_cgroup_mask_in_both_loops_for_every_phase(self):
        self.assertIn('/sys/fs/cgroup',runner.REQUIRED_MASKS)
        c=json.loads((Path(__file__).parent/'config.example.json').read_text());c['masks']=sorted(runner.REQUIRED_MASKS)
        c['review_receipts']={};c['bindings']={'runner':c['bindings']['runner']};c['source_manifest']={'path':'/control/source','sha256':'0'*64};c['old_home_stat_manifest']={'path':'/control/home','sha256':'1'*64}
        for phase in runner.PHASES:
            for worker in ([True,False] if phase in ('B','E','F') else [True]):
                with patch.object(runner,'owned_paths'),patch.object(runner,'jload',return_value={'head_files':[]}):args=runner.sandbox_args(c,phase,worker)
                binds=[tuple(args[i+1:i+3]) for i,x in enumerate(args) if x=='--ro-bind']
                self.assertEqual(binds.count((c['paths']['mask_empty'],'/sys/fs/cgroup')),2)



class RegisteredA102Subject(unittest.TestCase):
    def test_exact_registered_a102_subject_and_inventory_bindings(self):
        c=json.loads((Path(__file__).parent/'config.example.json').read_text())
        self.assertEqual(runner.BASE, '5cf27d64fe91a3faef3ed1b4283284c50f660b24')
        self.assertEqual(c['paths']['worktree'], '/home/charl/Moriarty/.worktrees/sherlock-a102-build-20261016')
        self.assertEqual(c['bindings']['subject'], {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/astra-a102-compile-subject-prep-r1/COMPILE-SUBJECT.json', 'sha256': '68620a2a2462411050860503128193f32250f9165ce4ec80346da195cd4f157b'})
        self.assertEqual(c['source_manifest'], {'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/astra-a102-compile-subject-prep-r1/source-manifest.proposal.json', 'sha256': '9f48d1f227c2714bab374828b7fe4dd930aab660ea63dd2685f91d6250bcafce'})
        self.assertEqual(runner.LOCK, 'dfffb853b47a7b2d0d1b80a64bb3e826d64d66f247881603b6fc31c8e3e34003')

    def test_ten_reviewed_source_members_bind_through_actual_source_binding(self):
        # All paths/digests are independent frozen expectations. Every filesystem
        # access in source_binding is patched; this fixture does not hash a checkout.
        root=Path('/home/charl/Moriarty/.worktrees/sherlock-a102-build-20261016')
        selected={'crates/mori-backend/src/sherlock_machine/enabled_fetch.rs': '6c45ab9190ad829db29b039d91050bfd8b82d1ccb66b3dd990538c18415dc8c1', 'crates/mori-backend/src/sherlock_machine/enabled_fetch_tests.rs': 'b6afffebff3c3d1bed4e2a6b94baa2f410e25a573c31756e2e90c4c27a820973', 'crates/mori-backend/src/sherlock_machine/fetch.rs': '6e71e0dccf687bec051556eeefdc435cef1c61535323d62a3994dc7220cadc87', 'crates/mori-backend/src/sherlock_machine/identifier_rejection_tests.rs': '3c5be80e5104827b287cc4f310f88f4b2ae4a084a0518d517d421bb4e3374851', 'crates/mori-backend/src/sherlock_machine/literal_step.rs': 'b5d2ca73b0e6b561f554902c3f4e50ec7743cc7ca04b9e7e9acb0c3aca10b839', 'crates/mori-backend/src/sherlock_machine/literal_step_normal_tests.rs': '255408fdafb36b2ed771206c366232dea2b61d7345d79537121e556f38163ef6', 'crates/mori-backend/src/sherlock_machine/mod.rs': '5f193ee16986094b9706c7dedc7c8ac091d29496b83e46e71fa7c4553f6eea2f', 'crates/mori-backend/src/sherlock_machine/normal_tests.rs': '651a47138ab6a9bd37666508ca48de7c1ea9846840f1b56fee2b3a30e5400c8c', 'crates/mori-backend/src/lib.rs': 'dcc9f256a05c4d2d85b18aa8c8a20572440995807bc233c0adfc6e5b8d5c0263', 'models/sherlock/literal-step/MAPPING.md': '60d5c62650055ba6ed644786ac5146c60892c1b2dbbc81c313e6d24873377243'}
        all_files=dict(selected, **{'Cargo.lock':'dfffb853b47a7b2d0d1b80a64bb3e826d64d66f247881603b6fc31c8e3e34003'})
        manifest={'root':str(root),'head':'5cf27d64fe91a3faef3ed1b4283284c50f660b24','files':all_files,'excluded_directories':['.git'],'head_files':[]}
        subject={'candidate_files':[{'path':p,'sha256':h} for p,h in selected.items()]}
        c={'paths':{'worktree':str(root)},'source_manifest':{'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/astra-a102-compile-subject-prep-r1/source-manifest.proposal.json', 'sha256': '9f48d1f227c2714bab374828b7fe4dd930aab660ea63dd2685f91d6250bcafce'},'bindings':{'subject':{'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/astra-a102-compile-subject-prep-r1/COMPILE-SUBJECT.json', 'sha256': '68620a2a2462411050860503128193f32250f9165ce4ec80346da195cd4f157b'}}}
        grouped={}
        for name in all_files:
            p=Path(name);grouped.setdefault(str(root/p.parent),[]).append(p.name)
        walk=[(base,[],names) for base,names in grouped.items()]
        def data(path):return manifest if path=={'path': '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-reference-20261015/opus-sprint-loop-20261015/astra-a102-compile-subject-prep-r1/source-manifest.proposal.json', 'sha256': '9f48d1f227c2714bab374828b7fe4dd930aab660ea63dd2685f91d6250bcafce'}['path'] else subject
        with patch.object(runner,'check_pin') as pins,patch.object(runner,'jload',side_effect=data),patch.object(runner.os,'walk',return_value=walk),patch.object(runner,'digest',side_effect=lambda p:all_files[str(p.relative_to(root))]) as digests:
            runner.source_binding(c)
        self.assertEqual(pins.call_count,1)
        self.assertEqual(digests.call_count,11)
        self.assertEqual(len(selected),10)
        self.assertEqual(set(selected),set(row['path'] for row in subject['candidate_files']))

    def test_verify_raw_and_freeze_source_enforce_exact_generated_lockfile_membership(self):
        import os
        import tempfile
        with tempfile.TemporaryDirectory() as tmp:
            base=Path(tmp).resolve()
            source_home=base/'source-home';packet=base/'packet'
            source_home.mkdir();packet.mkdir()
            (packet/'A0_FREEZE.json').write_text('{"inputs":[]}\n')
            (source_home/'.global-cache').write_bytes(b'sqlite')
            (source_home/'.package-cache').write_bytes(b'')
            ok_rel='registry/src/index.crates.io-1949cf8c6b5b557f/adler2-2.0.1/.cargo-ok'
            ok_path=source_home/ok_rel;ok_path.parent.mkdir(parents=True);ok_path.write_bytes(b'{"v":1}')
            c={
                'paths':{'source_home':str(source_home),'packet':str(packet)},
                'limits':{'C':{'packet_bytes':65536},'D':{'packet_bytes':65536}},
                'registry_roots':[],'git_checkouts':[],'includes':[],
                'expected_generated_files':[
                    {'relative':'.global-cache','kind':'cargo-lockfile'},
                    {'relative':'.package-cache','kind':'cargo-lockfile'},
                    {'relative':'.package-cache-mutate','kind':'cargo-lockfile'},
                    {'relative':ok_rel,'kind':'cargo-ok'},
                ],
            }
            try:
                with patch.object(runner,'approved_includes',return_value=set()):
                    with self.assertRaises(FileNotFoundError):
                        runner.verify_raw(c)
                    c['expected_generated_files']=[
                        {'relative':'.global-cache','kind':'cargo-lockfile'},
                        {'relative':'.package-cache-mutate','kind':'cargo-lockfile'},
                        {'relative':ok_rel,'kind':'cargo-ok'},
                    ]
                    (source_home/'.package-cache-mutate').write_bytes(b'')
                    with self.assertRaisesRegex(runner.Refusal,'unexpected or missing source-home file membership'):
                        runner.verify_raw(c)
                    (source_home/'.package-cache-mutate').unlink()
                    c['expected_generated_files']=[
                        {'relative':'.global-cache','kind':'cargo-lockfile'},
                        {'relative':'.package-cache','kind':'cargo-lockfile'},
                        {'relative':ok_rel,'kind':'cargo-ok'},
                    ]
                    runner.verify_raw(c)
                raw_receipt=json.loads((packet/'C_RAW_IDENTITIES.json').read_text())
                self.assertEqual(set(raw_receipt['files']),{'.global-cache','.package-cache',ok_rel})
                self.assertEqual(len(raw_receipt['cargo_ok']),1)
                runner.freeze_source(c)
                frozen=runner.frozen_source(c)
                self.assertEqual(frozen['files'],raw_receipt['files'])
            finally:
                for root_dir,dirs,files in os.walk(source_home,topdown=False):
                    for f in files:os.chmod(Path(root_dir)/f,0o600)
                    for d in dirs:os.chmod(Path(root_dir)/d,0o700)
                os.chmod(source_home,0o700)

if __name__=='__main__':unittest.main()

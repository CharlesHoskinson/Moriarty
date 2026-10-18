#!/usr/bin/python3
"""Mock-only unit tests for operator_provisioner.py. Never executed here."""
import copy
import unittest
import operator_provisioner as op

def sample_valid_request() -> dict:
    root = '/home/charl/Moriarty/.moriarty-dev/runtime/fresh-op-root-unselected'
    return {
        'schema_version': 1,
        'operator_uid': 1000,
        'operator_gid': 1000,
        'paths': {
            'execution_root': root,
            'source_home': root + '/source-home',
            'target': root + '/target',
            'tmp': root + '/tmp',
            'packet': root + '/packet',
            'empty_home': root + '/empty-home',
            'mask_empty': root + '/mask-empty',
            'config_dir': '/home/charl/Moriarty/.moriarty-dev/runtime/fresh-cfg-dir-unselected',
            'observation_output': '/home/charl/Moriarty/.moriarty-dev/runtime/fresh-cfg-dir-unselected/q-obs.json',
            'evidence_dir': '/home/charl/Moriarty/.moriarty-dev/runtime/fresh-evidence-unselected',
        },
        'quota_mounts': {
            'source_home': {'mountpoint': root + '/source-home', 'max_bytes': 1287098368, 'max_inodes': 524288},
            'target_tmp': {'mountpoint': root, 'max_bytes': 8589934592, 'max_inodes': 524288},
            'packet': {'mountpoint': root + '/packet', 'max_bytes': 67108864, 'max_inodes': 4096},
        },
        'cgroup': {
            'envelope_path': '/moriarty-fresh-envelope-unselected',
            'supervisor_leaf_name': 'supervisor',
            'phase_parent_name': 'phases',
            'envelope_memory_max': 21474836480,
            'envelope_pids_max': 1088,
            'envelope_cpu_max': '500000 100000',
            'supervisor_leaf_limits': {'memory_max': 1610612736, 'pids_max': 64, 'cpu_max': '100000 100000'},
            'phase_parent_limits': {'memory_max': 18534076416, 'pids_max': 1024, 'cpu_max': '400000 100000'},
        },
    }

from pathlib import Path
import stat
from unittest import mock

class ProvisionerContractTests(unittest.TestCase):
    def test_valid_request_passes_schema_and_arithmetic(self):
        req = sample_valid_request()
        self.assertEqual(op.validate_request(req)['schema_version'], 1)

    def test_rejects_historical_paths_ancestors_disjointness_and_nested_cgroup(self):
        req = sample_valid_request()
        req['cgroup']['envelope_path'] = '/user.slice/user-1000.slice/user@1000.service/e'
        with self.assertRaises(op.Refusal):
            op.validate_request(req)
        req2 = sample_valid_request()
        req2['cgroup']['envelope_path'] = '/moriarty-a102-compile-r1'
        with self.assertRaises(op.Refusal):
            op.validate_request(req2)
        req3 = sample_valid_request()
        req3['paths']['evidence_dir'] = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016'
        with self.assertRaises(op.Refusal):
            op.validate_request(req3)
        req4 = sample_valid_request()
        req4['paths']['evidence_dir'] = req4['paths']['config_dir'] + '/sub-evidence'
        with self.assertRaises(op.Refusal):
            op.validate_request(req4)
        req5 = sample_valid_request()
        req5['paths']['source_home'] = req5['paths']['execution_root'] + '/wrong-child-name'
        req5['quota_mounts']['source_home']['mountpoint'] = req5['paths']['source_home']
        with self.assertRaises(op.Refusal):
            op.validate_request(req5)

    def test_rejects_lp_sum_exceeding_envelope_or_tmpfs_plus_f_exceeding_p_or_phase_f_floors(self):
        req = copy.deepcopy(sample_valid_request())
        req['cgroup']['phase_parent_limits']['pids_max'] = 1025
        with self.assertRaises(op.Refusal):
            op.validate_request(req)
        req_mem = copy.deepcopy(sample_valid_request())
        req_mem['cgroup']['supervisor_leaf_limits']['memory_max'] = 4294967296
        with self.assertRaises(op.Refusal):
            op.validate_request(req_mem)
        req_p = copy.deepcopy(sample_valid_request())
        req_p['cgroup']['phase_parent_limits']['memory_max'] = 18534076416 - 4096
        with self.assertRaises(op.Refusal):
            op.validate_request(req_p)
        req_tt = copy.deepcopy(sample_valid_request())
        req_tt['quota_mounts']['target_tmp']['max_bytes'] = 8589934592 + 4096
        with self.assertRaises(op.Refusal):
            op.validate_request(req_tt)
        req_cpu = copy.deepcopy(sample_valid_request())
        req_cpu['cgroup']['phase_parent_limits']['cpu_max'] = '300000 100000'
        req_cpu['cgroup']['supervisor_leaf_limits']['cpu_max'] = '200000 100000'
        with self.assertRaises(op.Refusal):
            op.validate_request(req_cpu)

    def test_mount_flags_preserve_exec_on_target_tmp_and_noexec_on_others(self):
        req = sample_valid_request()
        f_target = op.mount_flags(req['quota_mounts']['target_tmp'], noexec=False)
        f_source = op.mount_flags(req['quota_mounts']['source_home'], noexec=True)
        self.assertNotIn('noexec', f_target)
        self.assertIn('noexec', f_source)
        self.assertIn('huge=never', f_target)

    def test_check_host_predicates_succeeds_without_root_cgroup_kill_using_nonroot_witness(self):
        req_at = sample_valid_request()
        req_at['cgroup']['envelope_path'] = '/moriarty@fresh-envelope'
        self.assertEqual(op.validate_request(req_at)['cgroup']['envelope_path'], '/moriarty@fresh-envelope')
        text_map = {
            '/proc/self/meminfo': 'MemAvailable: 25000000 kB\n',
            '/proc/self/mountinfo': '29 1 0:26 / /sys/fs/cgroup rw,nosuid,nodev,noexec,relatime shared:9 - cgroup2 cgroup2 rw\n',
            '/sys/fs/cgroup/cgroup.controllers': 'cpu memory pids\n',
            '/sys/fs/cgroup/cgroup.subtree_control': 'cpu memory pids\n',
            '/proc/self/cgroup': '0::/user.slice/user-1000.slice/user@1000.service/app.slice\n',
            '/proc/sys/user/max_user_namespaces': '63000\n',
        }
        existing = set(text_map) | {
            '/sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/app.slice/cgroup.kill',
            '/sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/app.slice/cgroup.events',
        }
        dir_stat = mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=0, st_gid=0)
        reg_stat = mock.MagicMock(st_mode=stat.S_IFREG | 0o644, st_uid=0, st_gid=0)
        with mock.patch.object(Path, 'exists', autospec=True, side_effect=lambda p: str(p) in existing), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=lambda p, encoding='utf-8': text_map[str(p)]), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'resolve', autospec=True, side_effect=lambda p, strict=False: p), \
             mock.patch.object(Path, 'lstat', autospec=True, side_effect=lambda p: dir_stat if str(p) == '/sys/fs/cgroup' else reg_stat), \
             mock.patch('os.access', return_value=False):
            facts = op.check_host_predicates()
            self.assertEqual(facts['self_cgroup'], '0::/user.slice/user-1000.slice/user@1000.service/app.slice')
            existing.remove('/sys/fs/cgroup/user.slice/user-1000.slice/user@1000.service/app.slice/cgroup.kill')
            with self.assertRaises(op.Refusal):
                op.check_host_predicates()

    def test_pin_operator_tool_accepts_root_symlinked_python3_and_rejects_unowned_target(self):
        dir_st = mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=0, st_gid=0)
        lnk_st = mock.MagicMock(st_mode=stat.S_IFLNK | 0o777, st_uid=0, st_gid=0)
        reg_st = mock.MagicMock(st_mode=stat.S_IFREG | 0o755, st_uid=0, st_gid=0, st_dev=1, st_ino=2, st_size=4)
        def fake_lstat(p):
            s = str(p)
            if s in ('/', '/usr', '/usr/bin', '/usr/lib'):
                return dir_st
            if s == '/usr/bin/python3':
                return lnk_st
            if s in ('/usr/bin/python3.14', '/usr/lib/python3.14'):
                return reg_st
            raise FileNotFoundError(s)
        with mock.patch.object(Path, 'is_symlink', autospec=True, side_effect=lambda p: str(p) == '/usr/bin/python3'), \
             mock.patch.object(Path, 'resolve', autospec=True, return_value=Path('/usr/lib/python3.14')), \
             mock.patch('os.lstat', side_effect=fake_lstat), \
             mock.patch('os.readlink', return_value='../lib/python3.14'), \
             mock.patch('os.access', return_value=False), \
             mock.patch('os.geteuid', return_value=1000), \
             mock.patch('operator_provisioner.sha256_regular', return_value='a' * 64):
            pin = op.pin_operator_tool('/usr/bin/python3')
            self.assertEqual(pin, {'path': '/usr/bin/python3', 'realpath': '/usr/lib/python3.14', 'sha256': 'a' * 64})
            reg_st.st_mode = stat.S_IFREG | 0o775
            with self.assertRaises(op.Refusal):
                op.pin_operator_tool('/usr/bin/python3')
            reg_st.st_mode = stat.S_IFREG | 0o755
            reg_st.st_uid = 1000
            with self.assertRaises(op.Refusal):
                op.pin_operator_tool('/usr/bin/python3')

    def test_prelaunch_verify_detects_qobs_or_tool_drift(self):
        req = sample_valid_request()
        base_obs = {
            'schema': 'sherlock-operator-provisioning-observation/1',
            'request_sha256': '1' * 64,
            'provisioner_sha256': '2' * 64,
            'op0_facts': {'mem_available_bytes': 25000000000, 'self_cgroup': '0::/user.slice/user-1000.slice/user@1000.service/session-1.scope'},
            'quota_mounts': {'target_tmp': {'mount_id': 101}},
            'cgroup_verified': {'envelope_path': '/moriarty-fresh-envelope-unselected'},
            'operator_tool_sha256': {'/usr/bin/python3': {'path': '/usr/bin/python3', 'realpath': '/usr/bin/python3.14', 'sha256': '3' * 64}},
        }
        live_obs = copy.deepcopy(base_obs)
        live_obs['op0_facts']['mem_available_bytes'] = 24000000000
        live_obs['op0_facts']['self_cgroup'] = '0::/user.slice/user-1000.slice/user@1000.service/session-2.scope'
        with mock.patch('operator_provisioner.verify_and_observe', return_value=live_obs) as vo_mock, \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(copy.deepcopy(base_obs), '4' * 64)) as jl_mock:
            res = op.cmd_prelaunch_verify(req, request_sha256='1' * 64, provisioner_sha256='2' * 64)
            self.assertEqual(res, {
                'step': 'PRELAUNCH-VERIFY',
                'status': 'OK',
                'observation_output': req['paths']['observation_output'],
                'observation_sha256': '4' * 64,
                'request_sha256': '1' * 64,
                'provisioner_sha256': '2' * 64,
                'mem_available_bytes': 24000000000,
            })
            vo_mock.assert_called_once_with(req, request_sha256='1' * 64, provisioner_sha256='2' * 64)
            jl_mock.assert_called_once_with(Path(req['paths']['observation_output']))
        drifted = copy.deepcopy(base_obs)
        drifted['quota_mounts']['target_tmp']['mount_id'] = 999
        with mock.patch('operator_provisioner.verify_and_observe', return_value=live_obs), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(drifted, '4' * 64)):
            with self.assertRaises(op.Refusal):
                op.cmd_prelaunch_verify(req, request_sha256='1' * 64, provisioner_sha256='2' * 64)
        tool_drifted = copy.deepcopy(base_obs)
        tool_drifted['operator_tool_sha256']['/usr/bin/python3']['sha256'] = 'f' * 64
        with mock.patch('operator_provisioner.verify_and_observe', return_value=live_obs), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(tool_drifted, '4' * 64)):
            with self.assertRaises(op.Refusal):
                op.cmd_prelaunch_verify(req, request_sha256='1' * 64, provisioner_sha256='2' * 64)
        bad_op0 = copy.deepcopy(base_obs)
        bad_op0['op0_facts']['mem_available_bytes'] = op.OP0_MIN_MEM_AVAILABLE_BYTES - 1
        with mock.patch('operator_provisioner.verify_and_observe', return_value=live_obs), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(bad_op0, '4' * 64)):
            with self.assertRaises(op.Refusal):
                op.cmd_prelaunch_verify(req, request_sha256='1' * 64, provisioner_sha256='2' * 64)

    def test_teardown_polls_cgroup_events_and_validates_mountinfo(self):
        req = sample_valid_request()
        req_sha = op.default_request_sha256(req)
        prov_sha = '6' * 64
        ev_dir = Path(req['paths']['evidence_dir'])
        ev_manifest = ev_dir / 'EVIDENCE_SHA256_MANIFEST.json'
        events_seq = ['populated 1\nfrozen 0\n', 'populated 0\nfrozen 0\n']
        dir_st = mock.MagicMock(st_mode=stat.S_IFDIR | 0o755)
        rows = [
            (['102', '1', '0:51', '/', req['paths']['source_home'], 'rw,nosuid,nodev,noexec'], ['tmpfs', 'tmpfs', 'rw']),
            (['103', '1', '0:52', '/', req['paths']['execution_root'], 'rw,nosuid,nodev'], ['tmpfs', 'tmpfs', 'rw']),
        ]
        manifest_doc = {
            'request_sha256': req_sha,
            'provisioner_sha256': prov_sha,
            'observation_sha256': '5' * 64,
            'files': {'ONE_SHOT_STARTED': 'a' * 64, 'packet/RESULT.json': 'b' * 64},
        }
        sha_map = {
            req['paths']['observation_output']: '5' * 64,
            str(ev_dir / 'ONE_SHOT_STARTED'): 'a' * 64,
            str(ev_dir / 'packet/RESULT.json'): 'b' * 64,
        }
        def fake_read_text(p, encoding='utf-8'):
            if str(p).endswith('cgroup.events'):
                return events_seq.pop(0) if len(events_seq) > 1 else events_seq[0]
            return ''
        with mock.patch('operator_provisioner.require_root_operator'), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(copy.deepcopy(manifest_doc), '4' * 64)) as jl_mock, \
             mock.patch('operator_provisioner.sha256_regular', side_effect=lambda p: sha_map[str(p)]), \
             mock.patch('operator_provisioner.pin_operator_tool', return_value={'path': '/usr/bin/umount', 'realpath': '/usr/bin/umount', 'sha256': '7' * 64}), \
             mock.patch('operator_provisioner.parse_mountinfo', return_value=rows), \
             mock.patch.object(Path, 'exists', autospec=True, return_value=True), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'lstat', autospec=True, return_value=dir_st), \
             mock.patch.object(Path, 'iterdir', autospec=True, return_value=iter([])), \
             mock.patch.object(Path, 'write_text', autospec=True), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=fake_read_text), \
             mock.patch('os.rmdir') as rm_mock, \
             mock.patch('time.sleep') as sleep_mock, \
             mock.patch('subprocess.run') as run_mock:
            res = op.cmd_teardown(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(res, {'step': 'TEARDOWN', 'status': 'OK', 'evidence_manifest_sha256': '4' * 64})
            jl_mock.assert_called_once_with(ev_manifest)
            self.assertEqual(sleep_mock.call_count, 1)
            self.assertEqual(rm_mock.call_count, 3)
            self.assertEqual(run_mock.call_count, 2)
        bad_obs_manifest = copy.deepcopy(manifest_doc)
        bad_obs_manifest['observation_sha256'] = '0' * 64
        with mock.patch('operator_provisioner.require_root_operator'), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(bad_obs_manifest, '4' * 64)), \
             mock.patch('operator_provisioner.sha256_regular', side_effect=lambda p: sha_map[str(p)]), \
             mock.patch.object(Path, 'exists', autospec=True, return_value=True), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch('os.rmdir') as rm_mock, \
             mock.patch('subprocess.run') as run_mock:
            with self.assertRaises(op.Refusal):
                op.cmd_teardown(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(rm_mock.call_count, 0)
            self.assertEqual(run_mock.call_count, 0)
        bad_file_sha_map = dict(sha_map)
        bad_file_sha_map[str(ev_dir / 'packet/RESULT.json')] = 'c' * 64
        with mock.patch('operator_provisioner.require_root_operator'), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(copy.deepcopy(manifest_doc), '4' * 64)), \
             mock.patch('operator_provisioner.sha256_regular', side_effect=lambda p: bad_file_sha_map[str(p)]), \
             mock.patch.object(Path, 'exists', autospec=True, return_value=True), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch('os.rmdir') as rm_mock, \
             mock.patch('subprocess.run') as run_mock:
            with self.assertRaises(op.Refusal):
                op.cmd_teardown(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(rm_mock.call_count, 0)
            self.assertEqual(run_mock.call_count, 0)

    def test_ops_collect_evidence_abort_cleanup_and_main_anchor(self):
        req = sample_valid_request()
        req_sha = op.default_request_sha256(req)
        prov_sha = '8' * 64
        obs_doc = {
            'schema': 'sherlock-operator-provisioning-observation/1',
            'request_sha256': req_sha,
            'provisioner_sha256': prov_sha,
            'quota_mounts': {
                'target_tmp': {'mountpoint': req['paths']['execution_root'], 'mount_id': 103, 'device': '0:52'},
                'packet': {'mountpoint': req['paths']['packet'], 'mount_id': 101, 'device': '0:50'},
            },
        }
        rows = [
            (['101', '1', '0:50', '/', req['paths']['packet'], 'rw,nosuid,nodev,noexec'], ['tmpfs', 'tmpfs', 'rw,uid=1000,gid=1000,mode=700']),
            (['103', '1', '0:52', '/', req['paths']['execution_root'], 'rw,nosuid,nodev'], ['tmpfs', 'tmpfs', 'rw,uid=1000,gid=1000,mode=700']),
        ]
        with mock.patch('operator_provisioner.require_unprivileged_operator'), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch('operator_provisioner.parse_mountinfo', return_value=[]):
            with self.assertRaises(op.Refusal):
                op.cmd_op3(req)
        marker_path = str(Path(req['paths']['execution_root']) / 'ONE_SHOT_STARTED')
        pkt_item_path = str(Path(req['paths']['packet']) / 'A_RESULT.json')
        ev_sha_map = {marker_path: '9' * 64, pkt_item_path: 'b' * 64}
        with mock.patch('operator_provisioner.require_unprivileged_operator'), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(copy.deepcopy(obs_doc), 'a' * 64)) as jl_mock, \
             mock.patch('operator_provisioner.sha256_regular', side_effect=lambda p: ev_sha_map[str(p)]), \
             mock.patch('operator_provisioner.parse_mountinfo', return_value=rows), \
             mock.patch('operator_provisioner.require_absent_nonsymlink'), \
             mock.patch('operator_provisioner.copy_regular_verified', side_effect=lambda src, dst, expected: expected), \
             mock.patch('operator_provisioner.fsync_dir'), \
             mock.patch('operator_provisioner.write_new_json') as wjson_mock, \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'is_file', autospec=True, return_value=True), \
             mock.patch.object(Path, 'iterdir', autospec=True, return_value=iter([Path(pkt_item_path)])), \
             mock.patch('os.mkdir'), \
             mock.patch('os.chmod'):
            ev = op.cmd_collect_evidence(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(ev, {'step': 'COLLECT-EVIDENCE', 'status': 'OK', 'evidence_dir': req['paths']['evidence_dir'], 'count': 2})
            jl_mock.assert_called_once_with(Path(req['paths']['observation_output']))
            wjson_mock.assert_called_once_with(
                Path(req['paths']['evidence_dir']) / 'EVIDENCE_SHA256_MANIFEST.json',
                {
                    'request_sha256': req_sha,
                    'provisioner_sha256': prov_sha,
                    'observation_sha256': 'a' * 64,
                    'files': {'ONE_SHOT_STARTED': '9' * 64, 'packet/A_RESULT.json': 'b' * 64},
                },
            )
        bad_obs = copy.deepcopy(obs_doc)
        bad_obs['request_sha256'] = '0' * 64
        with mock.patch('operator_provisioner.require_unprivileged_operator'), \
             mock.patch('operator_provisioner.jload_with_sha256', return_value=(bad_obs, 'a' * 64)), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch('operator_provisioner.write_new_json') as wjson_mock:
            with self.assertRaises(op.Refusal):
                op.cmd_collect_evidence(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(wjson_mock.call_count, 0)
        with mock.patch('operator_provisioner.require_root_operator'), \
             mock.patch.object(Path, 'exists', autospec=True, return_value=True), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False):
            with self.assertRaises(op.Refusal):
                op.cmd_abort_prelaunch_cleanup(req)
        with mock.patch('operator_provisioner.jload_with_sha256', return_value=(req, req_sha)), \
             mock.patch('operator_provisioner.default_provisioner_sha256', return_value=prov_sha), \
             mock.patch('operator_provisioner.cmd_op0', return_value={'step': 'OP-0', 'status': 'OK'}), \
             mock.patch('builtins.print'):
            self.assertEqual(op.main(['--request', '/tmp/req.json', '--request-sha256', req_sha, 'op0']), 0)
            with self.assertRaises(op.Refusal):
                op.main(['--request', '/tmp/req.json', '--request-sha256', '0' * 64, 'op0'])

    def test_cmd_op0_direct_host_predicates_and_fresh_path_refusals(self):
        req = sample_valid_request()
        self_cg_rel = 'user.slice/user-1000.slice/user@1000.service/app.slice'
        text_map = {
            '/proc/self/status': 'Name:\tpython3\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000000\nCapAmb:\t0000000000000000\n',
            '/proc/self/meminfo': 'MemAvailable: 23622324 kB\n',
            '/proc/self/mountinfo': '29 1 0:26 / /sys/fs/cgroup rw,nosuid,nodev,noexec,relatime shared:9 - cgroup2 cgroup2 rw\n',
            '/sys/fs/cgroup/cgroup.controllers': 'cpu memory pids\n',
            '/sys/fs/cgroup/cgroup.subtree_control': 'cpu memory pids\n',
            '/proc/self/cgroup': f'0::/{self_cg_rel}\n',
            '/proc/sys/user/max_user_namespaces': '63000\n',
        }
        existing = set(text_map) | {
            f'/sys/fs/cgroup/{self_cg_rel}/cgroup.kill',
            f'/sys/fs/cgroup/{self_cg_rel}/cgroup.events',
        }
        dir_st = mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=0, st_gid=0)
        reg_st = mock.MagicMock(st_mode=stat.S_IFREG | 0o644, st_uid=0, st_gid=0)
        bwrap_st = mock.MagicMock(st_mode=stat.S_IFREG | 0o755, st_uid=0, st_gid=0)
        def fake_lstat(p):
            s = str(p)
            if s == '/sys/fs/cgroup':
                return dir_st
            if s == '/usr/bin/bwrap':
                return bwrap_st
            return reg_st
        with mock.patch('os.getuid', return_value=1000), \
             mock.patch('os.geteuid', return_value=1000), \
             mock.patch('os.getgid', return_value=1000), \
             mock.patch('os.getegid', return_value=1000), \
             mock.patch('os.access', return_value=False), \
             mock.patch.object(Path, 'exists', autospec=True, side_effect=lambda p: str(p) in existing), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=lambda p, encoding='utf-8': text_map[str(p)]), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'resolve', autospec=True, side_effect=lambda p, strict=False: p), \
             mock.patch.object(Path, 'lstat', autospec=True, side_effect=fake_lstat):
            res = op.cmd_op0(req)
            self.assertEqual(res, {
                'step': 'OP-0',
                'status': 'OK',
                'mem_available_bytes': 23622324 * 1024,
                'self_cgroup': f'0::/{self_cg_rel}',
            })
            text_map['/proc/self/meminfo'] = 'MemAvailable: 23068184 kB\n'
            with self.assertRaises(op.Refusal):
                op.cmd_op0(req)
            text_map['/proc/self/meminfo'] = 'MemAvailable: 23622324 kB\n'
            text_map['/proc/self/status'] = 'Name:\tpython3\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000001\nCapAmb:\t0000000000000000\n'
            with self.assertRaises(op.Refusal):
                op.cmd_op0(req)
            text_map['/proc/self/status'] = 'Name:\tpython3\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000000\nCapAmb:\t0000000000000000\n'
            bwrap_st.st_mode = stat.S_IFREG | stat.S_ISUID | 0o755
            with self.assertRaises(op.Refusal):
                op.cmd_op0(req)
            bwrap_st.st_mode = stat.S_IFREG | 0o755
            existing.add(req['paths']['execution_root'])
            with self.assertRaises(op.Refusal):
                op.cmd_op0(req)
            existing.remove(req['paths']['execution_root'])
            e_dir, _, _ = op.cg_paths(req)
            existing.add(str(e_dir))
            with self.assertRaises(op.Refusal):
                op.cmd_op0(req)

    def test_cmd_op1_direct_mkdir_chmod_and_refusals(self):
        req = sample_valid_request()
        root_p = Path(req['paths']['execution_root'])
        cfg_p = Path(req['paths']['config_dir'])
        status_text = 'Name:\tpython3\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000000\nCapAmb:\t0000000000000000\n'
        existing = set()
        symlinks = set()
        with mock.patch('os.getuid', return_value=1000), \
             mock.patch('os.geteuid', return_value=1000), \
             mock.patch('os.getgid', return_value=1000), \
             mock.patch('os.getegid', return_value=1000), \
             mock.patch.object(Path, 'read_text', autospec=True, return_value=status_text), \
             mock.patch.object(Path, 'exists', autospec=True, side_effect=lambda p: str(p) in existing), \
             mock.patch.object(Path, 'is_symlink', autospec=True, side_effect=lambda p: str(p) in symlinks), \
             mock.patch('os.mkdir') as mkdir_mock, \
             mock.patch('os.chmod') as chmod_mock:
            res = op.cmd_op1(req)
            self.assertEqual(res, {
                'step': 'OP-1',
                'status': 'OK',
                'execution_root': req['paths']['execution_root'],
                'config_dir': req['paths']['config_dir'],
            })
            self.assertEqual(mkdir_mock.call_args_list, [mock.call(root_p, 0o700), mock.call(cfg_p, 0o755)])
            self.assertEqual(chmod_mock.call_args_list, [mock.call(root_p, 0o700), mock.call(cfg_p, 0o755)])
            mkdir_mock.reset_mock()
            existing.add(req['paths']['execution_root'])
            with self.assertRaises(op.Refusal):
                op.cmd_op1(req)
            self.assertEqual(mkdir_mock.call_count, 0)
            existing.clear()
            symlinks.add(str(cfg_p.parent))
            with self.assertRaises(op.Refusal):
                op.cmd_op1(req)
            self.assertEqual(mkdir_mock.call_count, 0)
        with mock.patch('os.getuid', return_value=0), \
             mock.patch('os.geteuid', return_value=0), \
             mock.patch('os.getgid', return_value=0), \
             mock.patch('os.getegid', return_value=0), \
             mock.patch('os.mkdir') as mkdir_mock:
            with self.assertRaises(op.Refusal):
                op.cmd_op1(req)
            self.assertEqual(mkdir_mock.call_count, 0)

    def test_cmd_op2_direct_root_target_tmp_mount_and_refusals(self):
        req = sample_valid_request()
        root_str = req['paths']['execution_root']
        mountinfo_text = '29 1 0:26 / /sys/fs/cgroup rw,nosuid,nodev,noexec,relatime shared:9 - cgroup2 cgroup2 rw\n'
        root_st = mock.MagicMock(st_mode=stat.S_IFDIR | 0o700, st_uid=1000, st_gid=1000)
        children = []
        with mock.patch('os.getuid', return_value=0), \
             mock.patch('os.geteuid', return_value=0), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=lambda p, encoding='utf-8': mountinfo_text), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'lstat', autospec=True, return_value=root_st), \
             mock.patch.object(Path, 'iterdir', autospec=True, side_effect=lambda p: iter(children)), \
             mock.patch('operator_provisioner.pin_operator_tool', return_value={'path': '/usr/bin/mount', 'realpath': '/usr/bin/mount', 'sha256': 'a' * 64}), \
             mock.patch('subprocess.run') as run_mock:
            res = op.cmd_op2(req)
            self.assertEqual(res, {'step': 'OP-2', 'status': 'OK', 'mounted': root_str})
            expected_flags = 'rw,nosuid,nodev,huge=never,size=8589934592,nr_inodes=524288,mode=0700,uid=1000,gid=1000'
            self.assertEqual(run_mock.call_args_list, [
                mock.call(['/usr/bin/mount', '-t', 'tmpfs', '-o', expected_flags, 'tmpfs', root_str], check=True, shell=False),
                mock.call(['/usr/bin/mount', '--make-private', root_str], check=True, shell=False),
            ])
            run_mock.reset_mock()
            root_st.st_uid = 0
            with self.assertRaises(op.Refusal):
                op.cmd_op2(req)
            self.assertEqual(run_mock.call_count, 0)
            root_st.st_uid = 1000
            children.append(Path(root_str) / 'unexpected')
            with self.assertRaises(op.Refusal):
                op.cmd_op2(req)
            self.assertEqual(run_mock.call_count, 0)
            children.clear()
            mountinfo_text += f'101 20 0:50 / {root_str} rw,nosuid,nodev - tmpfs tmpfs rw\n'
            with self.assertRaises(op.Refusal):
                op.cmd_op2(req)
            self.assertEqual(run_mock.call_count, 0)
        with mock.patch('os.getuid', return_value=1000), \
             mock.patch('os.geteuid', return_value=1000), \
             mock.patch('subprocess.run') as run_mock:
            with self.assertRaises(op.Refusal):
                op.cmd_op2(req)
            self.assertEqual(run_mock.call_count, 0)

    def test_cmd_op4_direct_source_home_and_packet_noexec_mounts_and_refusals(self):
        req = sample_valid_request()
        root_str = req['paths']['execution_root']
        sh_str = req['paths']['source_home']
        pkt_str = req['paths']['packet']
        mountinfo_text = f'101 20 0:50 / {root_str} rw,nosuid,nodev - tmpfs tmpfs rw,uid=1000,gid=1000,mode=700\n'
        dev_map = {root_str: 50, sh_str: 50, pkt_str: 50}
        def fake_lstat(p):
            return mock.MagicMock(st_mode=stat.S_IFDIR | 0o700, st_uid=1000, st_gid=1000, st_dev=dev_map[str(p)])
        with mock.patch('os.getuid', return_value=0), \
             mock.patch('os.geteuid', return_value=0), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=lambda p, encoding='utf-8': mountinfo_text), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'lstat', autospec=True, side_effect=fake_lstat), \
             mock.patch.object(Path, 'iterdir', autospec=True, return_value=iter([])), \
             mock.patch('operator_provisioner.pin_operator_tool', return_value={'path': '/usr/bin/mount', 'realpath': '/usr/bin/mount', 'sha256': 'a' * 64}), \
             mock.patch('subprocess.run') as run_mock:
            res = op.cmd_op4(req)
            self.assertEqual(res, {'step': 'OP-4', 'status': 'OK', 'mounted': [sh_str, pkt_str]})
            sh_flags = 'rw,nosuid,nodev,noexec,huge=never,size=1287098368,nr_inodes=524288,mode=0700,uid=1000,gid=1000'
            pkt_flags = 'rw,nosuid,nodev,noexec,huge=never,size=67108864,nr_inodes=4096,mode=0700,uid=1000,gid=1000'
            self.assertEqual(run_mock.call_args_list, [
                mock.call(['/usr/bin/mount', '-t', 'tmpfs', '-o', sh_flags, 'tmpfs', sh_str], check=True, shell=False),
                mock.call(['/usr/bin/mount', '--make-private', sh_str], check=True, shell=False),
                mock.call(['/usr/bin/mount', '-t', 'tmpfs', '-o', pkt_flags, 'tmpfs', pkt_str], check=True, shell=False),
                mock.call(['/usr/bin/mount', '--make-private', pkt_str], check=True, shell=False),
            ])
            run_mock.reset_mock()
            dev_map[pkt_str] = 99
            with self.assertRaises(op.Refusal):
                op.cmd_op4(req)
            dev_map[pkt_str] = 50
            run_mock.reset_mock()
            mountinfo_text = '29 1 0:26 / /sys/fs/cgroup rw,nosuid,nodev,noexec - cgroup2 cgroup2 rw\n'
            with self.assertRaises(op.Refusal):
                op.cmd_op4(req)
            self.assertEqual(run_mock.call_count, 0)

    def test_cmd_op5_direct_cgroup_hierarchy_caps_and_selective_delegation(self):
        req = sample_valid_request()
        e_dir, l_dir, p_dir = op.cg_paths(req)
        mountinfo_text = '29 1 0:26 / /sys/fs/cgroup rw,nosuid,nodev,noexec,relatime shared:9 - cgroup2 cgroup2 rw\n'
        written_cg = {}
        owners = {}
        modes = {}
        readback_override = {}
        def fake_write_text(p, val, encoding='utf-8'):
            written_cg[str(p)] = val
            return len(val)
        def fake_read_text(p, encoding='utf-8'):
            s = str(p)
            if s == '/proc/self/mountinfo':
                return mountinfo_text
            if s in readback_override:
                return readback_override[s]
            return written_cg[s] + '\n'
        def fake_chown(p, uid, gid, follow_symlinks=True):
            self.assertFalse(follow_symlinks)
            owners[str(p)] = (uid, gid)
        def fake_chmod(p, mode):
            modes[str(p)] = mode
        with mock.patch('os.getuid', return_value=0), \
             mock.patch('os.geteuid', return_value=0), \
             mock.patch.object(Path, 'exists', autospec=True, return_value=False), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'write_text', autospec=True, side_effect=fake_write_text), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=fake_read_text), \
             mock.patch('os.mkdir') as mkdir_mock, \
             mock.patch('os.chown', side_effect=fake_chown), \
             mock.patch('os.chmod', side_effect=fake_chmod):
            res = op.cmd_op5(req)
            self.assertEqual(res, {'step': 'OP-5', 'status': 'OK', 'E': str(e_dir), 'L': str(l_dir), 'P': str(p_dir)})
            self.assertEqual(mkdir_mock.call_args_list, [mock.call(e_dir, 0o755), mock.call(l_dir, 0o755), mock.call(p_dir, 0o755)])
            self.assertEqual(written_cg[str(e_dir / 'memory.max')], '21474836480')
            self.assertEqual(written_cg[str(e_dir / 'memory.swap.max')], '0')
            self.assertEqual(written_cg[str(e_dir / 'pids.max')], '1088')
            self.assertEqual(written_cg[str(e_dir / 'cpu.max')], '500000 100000')
            self.assertEqual(written_cg[str(e_dir / 'cgroup.subtree_control')], '+cpu +memory +pids')
            self.assertEqual(written_cg[str(l_dir / 'memory.max')], '1610612736')
            self.assertEqual(written_cg[str(l_dir / 'memory.swap.max')], '0')
            self.assertEqual(written_cg[str(l_dir / 'pids.max')], '64')
            self.assertEqual(written_cg[str(l_dir / 'cpu.max')], '100000 100000')
            self.assertNotIn(str(l_dir / 'cgroup.subtree_control'), written_cg)
            self.assertEqual(written_cg[str(p_dir / 'memory.max')], '18534076416')
            self.assertEqual(written_cg[str(p_dir / 'memory.swap.max')], '0')
            self.assertEqual(written_cg[str(p_dir / 'pids.max')], '1024')
            self.assertEqual(written_cg[str(p_dir / 'cpu.max')], '400000 100000')
            self.assertEqual(written_cg[str(p_dir / 'cgroup.subtree_control')], '+cpu +memory +pids')
            self.assertFalse(any(k.endswith(('cgroup.max.depth', 'cgroup.max.descendants')) for k in written_cg))
            delegated = {
                str(e_dir / 'cgroup.procs'),
                str(p_dir),
                str(p_dir / 'cgroup.procs'),
                str(p_dir / 'cgroup.threads'),
                str(p_dir / 'cgroup.subtree_control'),
            }
            self.assertEqual({k for k, v in owners.items() if v == (1000, 1000)}, delegated)
            for node in (e_dir, l_dir, p_dir):
                for cap in op.CAP_FILES:
                    self.assertEqual(owners[str(node / cap)], (0, 0))
                    self.assertEqual(modes[str(node / cap)], 0o644)
            readback_override[str(p_dir / 'memory.max')] = '18534072320\n'
            with self.assertRaises(op.Refusal):
                op.cmd_op5(req)
            readback_override.clear()
            mountinfo_text += '30 29 0:27 / /sys/fs/cgroup/nested rw - cgroup2 cgroup2 rw\n'
            with self.assertRaises(op.Refusal):
                op.cmd_op5(req)

    def test_verify_and_observe_and_cmd_op6_direct_qobs_k_invariants_and_atomic_write(self):
        req = sample_valid_request()
        req_sha = op.default_request_sha256(req)
        prov_sha = 'c' * 64
        e_dir, l_dir, p_dir = op.cg_paths(req)
        self_cg_rel = 'user.slice/user-1000.slice/session-1.scope'
        root_str = req['paths']['execution_root']
        sh_str = req['paths']['source_home']
        pkt_str = req['paths']['packet']
        cfg_str = req['paths']['config_dir']
        obs_str = req['paths']['observation_output']
        base_mountinfo = (
            '29 1 0:26 / /sys/fs/cgroup rw,nosuid,nodev,noexec,relatime shared:9 - cgroup2 cgroup2 rw\n'
            f'101 20 0:50 / {root_str} rw,nosuid,nodev,relatime - tmpfs tmpfs rw,size=8388608k,nr_inodes=524288,mode=700,uid=1000,gid=1000\n'
            f'102 101 0:51 / {sh_str} rw,nosuid,nodev,noexec,relatime - tmpfs tmpfs rw,size=1256932k,nr_inodes=524288,mode=700,uid=1000,gid=1000\n'
            f'103 101 0:52 / {pkt_str} rw,nosuid,nodev,noexec,relatime - tmpfs tmpfs rw,size=65536k,nr_inodes=4096,mode=700,uid=1000,gid=1000\n'
        )
        text_map = {
            '/proc/self/status': 'Name:\tpython3\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000000\nCapAmb:\t0000000000000000\n',
            '/proc/self/meminfo': 'MemAvailable: 23622324 kB\n',
            '/proc/self/mountinfo': base_mountinfo,
            '/sys/fs/cgroup/cgroup.controllers': 'cpu memory pids\n',
            '/sys/fs/cgroup/cgroup.subtree_control': 'cpu memory pids\n',
            '/proc/self/cgroup': f'0::/{self_cg_rel}\n',
            '/proc/sys/user/max_user_namespaces': '63000\n',
            str(e_dir / 'memory.max'): '21474836480\n',
            str(e_dir / 'memory.swap.max'): '0\n',
            str(e_dir / 'pids.max'): '1088\n',
            str(e_dir / 'cpu.max'): '500000 100000\n',
            str(e_dir / 'cgroup.subtree_control'): 'cpu memory pids\n',
            str(l_dir / 'memory.max'): '1610612736\n',
            str(l_dir / 'memory.swap.max'): '0\n',
            str(l_dir / 'pids.max'): '64\n',
            str(l_dir / 'cpu.max'): '100000 100000\n',
            str(l_dir / 'cgroup.subtree_control'): '\n',
            str(p_dir / 'memory.max'): '18534076416\n',
            str(p_dir / 'memory.swap.max'): '0\n',
            str(p_dir / 'pids.max'): '1024\n',
            str(p_dir / 'cpu.max'): '400000 100000\n',
            str(p_dir / 'cgroup.subtree_control'): 'cpu memory pids\n',
        }
        for node in (e_dir, l_dir, p_dir):
            text_map[str(node / 'cgroup.type')] = 'domain\n'
            text_map[str(node / 'cgroup.procs')] = ''
            text_map[str(node / 'cgroup.events')] = 'populated 0\nfrozen 0\n'
            text_map[str(node / 'cgroup.controllers')] = 'cpu memory pids\n'
        existing = set(text_map) | {
            cfg_str,
            f'/sys/fs/cgroup/{self_cg_rel}/cgroup.kill',
            f'/sys/fs/cgroup/{self_cg_rel}/cgroup.events',
            *(str(n / 'cgroup.kill') for n in (e_dir, l_dir, p_dir)),
            *(str(n / 'cgroup.events') for n in (e_dir, l_dir, p_dir)),
        }
        delegated_files = {
            str(e_dir / 'cgroup.procs'),
            str(p_dir / 'cgroup.procs'),
            str(p_dir / 'cgroup.threads'),
            str(p_dir / 'cgroup.subtree_control'),
        }
        writable_set = {str(p_dir)} | set(delegated_files)
        dir_paths = {
            '/', '/usr', '/usr/bin', '/sys/fs/cgroup', cfg_str,
            str(e_dir), str(l_dir), str(p_dir),
            root_str, *(req['paths'][k] for k in op.CHILD_KEYS),
        }
        cg_entry_names = (*op.CAP_FILES, 'cgroup.procs', 'cgroup.threads', 'cgroup.subtree_control', 'cgroup.type', 'cgroup.controllers', 'cgroup.events', 'cgroup.kill')
        def fake_iterdir(p):
            s = str(p)
            if s == str(e_dir):
                return iter([l_dir, p_dir, *(e_dir / n for n in cg_entry_names)])
            if s in (str(l_dir), str(p_dir)):
                base = Path(s)
                return iter([*(base / n for n in cg_entry_names)])
            return iter([])
        def fake_lstat(p):
            s = str(p)
            if s in (root_str, req['paths']['target'], req['paths']['tmp'], req['paths']['empty_home'], req['paths']['mask_empty']):
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o700, st_uid=1000, st_gid=1000, st_dev=50)
            if s == sh_str:
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o700, st_uid=1000, st_gid=1000, st_dev=51)
            if s == pkt_str:
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o700, st_uid=1000, st_gid=1000, st_dev=52)
            if s == cfg_str:
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=1000, st_gid=1000, st_dev=1)
            if s in ('/', '/usr', '/usr/bin'):
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=0, st_gid=0, st_dev=1)
            if s in op.OPERATOR_TOOLS:
                return mock.MagicMock(st_mode=stat.S_IFREG | 0o755, st_uid=0, st_gid=0, st_dev=1, st_ino=10, st_size=128)
            if s in ('/sys/fs/cgroup', str(e_dir), str(l_dir)):
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=0, st_gid=0, st_dev=26)
            if s == str(p_dir):
                return mock.MagicMock(st_mode=stat.S_IFDIR | 0o755, st_uid=1000, st_gid=1000, st_dev=26)
            if s in delegated_files:
                return mock.MagicMock(st_mode=stat.S_IFREG | 0o644, st_uid=1000, st_gid=1000, st_dev=26)
            return mock.MagicMock(st_mode=stat.S_IFREG | 0o644, st_uid=0, st_gid=0, st_dev=26)
        statvfs_map = {
            root_str: mock.MagicMock(f_blocks=2097152, f_frsize=4096, f_files=524288),
            sh_str: mock.MagicMock(f_blocks=314233, f_frsize=4096, f_files=524288),
            pkt_str: mock.MagicMock(f_blocks=16384, f_frsize=4096, f_files=4096),
        }
        tool_hashes = {t: f'{idx + 1:064x}' for idx, t in enumerate(op.OPERATOR_TOOLS)}
        written_chunks = []
        def fake_sha256_regular(p):
            s = str(p)
            if s == obs_str:
                return op.hashlib.sha256(b''.join(written_chunks)).hexdigest()
            return tool_hashes[s]
        def fake_fdopen(fd, mode='rb'):
            fobj = mock.MagicMock()
            fobj.__enter__.return_value = fobj
            fobj.__exit__.return_value = False
            fobj.fileno.return_value = fd
            fobj.write.side_effect = written_chunks.append
            return fobj
        with mock.patch('os.getuid', return_value=1000), \
             mock.patch('os.geteuid', return_value=1000), \
             mock.patch('os.getgid', return_value=1000), \
             mock.patch('os.getegid', return_value=1000), \
             mock.patch('os.access', side_effect=lambda p, mode: str(p) in writable_set), \
             mock.patch('os.statvfs', side_effect=lambda mp: statvfs_map[str(mp)]), \
             mock.patch('os.lstat', side_effect=fake_lstat), \
             mock.patch.object(Path, 'lstat', autospec=True, side_effect=fake_lstat), \
             mock.patch.object(Path, 'exists', autospec=True, side_effect=lambda p: str(p) in existing), \
             mock.patch.object(Path, 'is_symlink', autospec=True, return_value=False), \
             mock.patch.object(Path, 'is_dir', autospec=True, side_effect=lambda p: str(p) in dir_paths), \
             mock.patch.object(Path, 'resolve', autospec=True, side_effect=lambda p, strict=False: p), \
             mock.patch.object(Path, 'read_text', autospec=True, side_effect=lambda p, encoding='utf-8': text_map[str(p)]), \
             mock.patch.object(Path, 'iterdir', autospec=True, side_effect=fake_iterdir), \
             mock.patch('operator_provisioner.sha256_regular', side_effect=fake_sha256_regular), \
             mock.patch('os.open', side_effect=[901, 902]) as open_mock, \
             mock.patch('os.fdopen', side_effect=fake_fdopen), \
             mock.patch('os.fsync') as fsync_mock, \
             mock.patch('os.close') as close_mock:
            obs = op.verify_and_observe(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(obs['schema'], 'sherlock-operator-provisioning-observation/1')
            self.assertEqual(obs['request_sha256'], req_sha)
            self.assertEqual(obs['provisioner_sha256'], prov_sha)
            self.assertEqual(obs['op0_facts'], {'mem_available_bytes': 23622324 * 1024, 'self_cgroup': f'0::/{self_cg_rel}'})
            self.assertEqual(obs['quota_mounts'], {
                'target_tmp': {'mountpoint': root_str, 'mount_id': 101, 'device': '0:50', 'max_bytes': 8589934592, 'max_inodes': 524288},
                'source_home': {'mountpoint': sh_str, 'mount_id': 102, 'device': '0:51', 'max_bytes': 1287098368, 'max_inodes': 524288},
                'packet': {'mountpoint': pkt_str, 'mount_id': 103, 'device': '0:52', 'max_bytes': 67108864, 'max_inodes': 4096},
            })
            self.assertEqual(obs['cgroup_verified']['k_invariants'], {'K1': True, 'K2': True, 'K3': True})
            self.assertEqual(obs['cgroup_verified']['envelope_memory_max'], 21474836480)
            self.assertEqual(obs['cgroup_verified']['supervisor_leaf_limits']['memory_max'], 1610612736)
            self.assertEqual(obs['cgroup_verified']['phase_parent_limits']['memory_max'], 18534076416)
            self.assertEqual(obs['cgroup_verified']['nodes']['L']['subtree_control'], [])
            self.assertEqual(obs['cgroup_verified']['nodes']['P']['subtree_control'], ['cpu', 'memory', 'pids'])
            self.assertEqual(set(obs['operator_tool_sha256']), set(op.OPERATOR_TOOLS))
            writable_set.add(str(e_dir / 'memory.max'))
            with self.assertRaises(op.Refusal):
                op.verify_and_observe(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            writable_set.remove(str(e_dir / 'memory.max'))
            writable_set.add(str(l_dir / 'cgroup.procs'))
            with self.assertRaises(op.Refusal):
                op.verify_and_observe(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            writable_set.remove(str(l_dir / 'cgroup.procs'))
            writable_set.remove(str(e_dir / 'cgroup.procs'))
            with self.assertRaises(op.Refusal):
                op.verify_and_observe(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            writable_set.add(str(e_dir / 'cgroup.procs'))
            text_map['/proc/self/mountinfo'] = base_mountinfo.replace('rw,nosuid,nodev,relatime -', 'rw,nosuid,nodev,relatime shared:1 -')
            with self.assertRaises(op.Refusal):
                op.verify_and_observe(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            text_map['/proc/self/mountinfo'] = base_mountinfo
            statvfs_map[pkt_str] = mock.MagicMock(f_blocks=16384, f_frsize=4096, f_files=8192)
            with self.assertRaises(op.Refusal):
                op.verify_and_observe(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            statvfs_map[pkt_str] = mock.MagicMock(f_blocks=16384, f_frsize=4096, f_files=4096)
            op6_res = op.cmd_op6(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            written_bytes = b''.join(written_chunks)
            expected_obs_sha = op.hashlib.sha256(written_bytes).hexdigest()
            self.assertEqual(op6_res, {
                'step': 'OP-6',
                'status': 'OK',
                'observation_output': obs_str,
                'observation_sha256': expected_obs_sha,
            })
            self.assertEqual(op.json.loads(written_bytes.decode('utf-8')), obs)
            self.assertEqual(open_mock.call_count, 2)
            self.assertEqual(open_mock.call_args_list[0][0][0], Path(obs_str))
            self.assertEqual(open_mock.call_args_list[0][0][2], 0o600)
            self.assertEqual(open_mock.call_args_list[1][0][0], Path(cfg_str))
            self.assertEqual(fsync_mock.call_args_list, [mock.call(901), mock.call(902)])
            self.assertEqual(close_mock.call_args_list, [mock.call(902)])
            open_mock.reset_mock()
            existing.add(obs_str)
            with self.assertRaises(op.Refusal):
                op.cmd_op6(req, request_sha256=req_sha, provisioner_sha256=prov_sha)
            self.assertEqual(open_mock.call_count, 0)

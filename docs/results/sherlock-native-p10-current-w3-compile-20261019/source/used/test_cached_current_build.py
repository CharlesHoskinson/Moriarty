#!/usr/bin/env python3
"""SDK-free synthetic unit tests for cached_current_build and bounded_sdk_compile hooks."""

import copy
import hashlib
import importlib.util
import io
import json
import os
import shutil
import stat
import sys
import tarfile
import tempfile
import unittest
from pathlib import Path

DIR = Path(__file__).resolve().parent
if str(DIR) not in sys.path:
    sys.path.insert(0, str(DIR))


def _load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    spec.loader.exec_module(mod)
    return mod


guard = _load_module('bounded_sdk_compile', DIR / 'bounded_sdk_compile.py')
ccb = _load_module('cached_current_build', DIR / 'cached_current_build.py')


def _sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def _git_blob_sha1(data):
    h = hashlib.sha1()
    h.update(('blob %d\0' % len(data)).encode('ascii'))
    h.update(data)
    return h.hexdigest()


class CachedCurrentBuildSyntheticTest(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.saved_base = guard.BASE
        self.saved_lock = guard.LOCK

    def tearDown(self):
        guard.BASE = self.saved_base
        guard.LOCK = self.saved_lock
        for base, dirs, _ in os.walk(self.root, topdown=True, followlinks=False):
            bpath = Path(base)
            if not bpath.is_symlink():
                os.chmod(bpath, 0o700)
            for d in dirs:
                dp = bpath / d
                if not dp.is_symlink():
                    os.chmod(dp, 0o700)
        self.tmp.cleanup()

    def _write(self, path, data=b'data', mode=0o644):
        p = Path(path)
        p.parent.mkdir(parents=True, exist_ok=True)
        raw = data.encode('utf-8') if isinstance(data, str) else data
        with open(p, 'wb') as f:
            f.write(raw)
        os.chmod(p, mode)
        return p

    def _make_synthetic_quarantine(self):
        q_root = self.root / 'quarantine'
        q_target = q_root / 'target'
        q_packet = q_root / 'packet'
        q_proposal = self.root / 'proposal'
        q_target.mkdir(parents=True, exist_ok=True)
        q_packet.mkdir(parents=True, exist_ok=True)
        q_proposal.mkdir(parents=True, exist_ok=True)

        # Workspace rows (must be classified & excluded from seeding)
        ws_fp = 'debug/.fingerprint/mori-backend-3a2f695515330e5a/dep-lib-mori_backend'
        ws_dep = 'debug/deps/libmori_backend-3a2f695515330e5a.rlib'
        ws_inc = 'debug/incremental/mori_backend-1a2b3c4d5e6f7a8b/s-1234-abcd/dep-graph.bin'

        # Third-party rows including non-debug (.rustc_info.json, generated/bincode/*.rs) and host_outputs
        tp_rustc_info = '.rustc_info.json'
        tp_bincode = 'generated/bincode/ZkStdLibArchV1_Decode.rs'
        tp_unit = 'serde-1111222233334444'
        tp_bs_bin1 = f'debug/build/{tp_unit}/build_script_build-aaaa1111bbbb2222'
        tp_bs_bin2 = f'debug/build/{tp_unit}/build-script-build'
        tp_pm_so1 = 'debug/deps/libserde_derive-cccc3333dddd4444.so'
        tp_pm_so2 = 'debug/libserde_derive-cccc3333dddd4444.so'
        tp_invoked = f'debug/build/{tp_unit}/invoked.timestamp'
        tp_out = f'debug/build/{tp_unit}/output'
        tp_root_out = f'debug/build/{tp_unit}/root-output'
        tp_stderr = f'debug/build/{tp_unit}/stderr'
        tp_rlib = 'debug/deps/libserde-5555666677778888.rlib'
        tp_depinfo = 'debug/deps/libserde-5555666677778888.d'

        target_home = self.root / 'exec' / 'target'
        source_home = self.root / 'exec' / 'source-home'
        worktree = self.root / 'worktree'
        for d in (target_home, source_home, worktree):
            d.mkdir(parents=True, exist_ok=True)
        dep_src = self._write(source_home / 'registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.0/src/lib.rs', b'// serde\n', 0o444)

        files_spec = {
            ws_fp: (b'ws-fp', False),
            ws_dep: (b'ws-rlib', False),
            ws_inc: (b'ws-inc', False),
            tp_rustc_info: (b'{"rustc":1}', False),
            tp_bincode: (b'// bincode decode\n', False),
            tp_bs_bin1: (b'\x7fELF-bs1', True),
            tp_bs_bin2: (b'\x7fELF-bs1', True),
            tp_pm_so1: (b'\x7fELF-pm1', True),
            tp_pm_so2: (b'\x7fELF-pm1', True),
            tp_invoked: (b'timestamp', False),
            tp_out: (b'cargo:rustc-cfg=feature="std"\n', False),
            tp_root_out: (str(target_home / f'debug/build/{tp_unit}').encode('utf-8'), False),
            tp_stderr: (b'', False),
            tp_rlib: (b'rlib-bytes', False),
            tp_depinfo: (f'{target_home / tp_rlib}: {dep_src}\n'.encode('utf-8'), False),
        }

        all_outputs_map = {}
        copy_map_rows = []
        host_outputs = []
        for rel, (payload, is_exec) in files_spec.items():
            p = self._write(q_target / rel, payload, mode=0o400)
            st = p.lstat()
            sha = _sha256_bytes(payload)
            all_outputs_map[rel] = sha
            copy_map_rows.append({
                'relative_path': 'target/' + rel,
                'origin': str(target_home / rel),
                'destination': str(p),
                'sha256': sha,
                'bytes': len(payload),
                'source_mode_octal': '755' if is_exec else '644',
                'device_decimal': str(st.st_dev),
                'inode': st.st_ino,
            })
            if is_exec:
                host_outputs.append({
                    'identity': ['serde 1.0.0', 'build-script' if rel.startswith('debug/build/') else 'proc-macro', 'build.rs'],
                    'kind': 'build-script' if rel.startswith('debug/build/') else 'proc-macro',
                    'name': 'serde',
                    'path': rel,
                    'sha256': sha,
                    'source_root': str(source_home / 'registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.0'),
                    'target_name': Path(rel).name,
                })

        copy_map_path = self._write(q_proposal / 'PRESERVATION-EXACT-COPY-MAP.proposed.json', json.dumps({'files': copy_map_rows}), 0o444)
        disp_path = self._write(q_proposal / 'ROOT-PRESERVATION-DISPOSITION.json', json.dumps({'status': 'PRESERVED'}), 0o444)
        self._write(
            q_packet / 'F_RETAINED_QUARANTINED_OUTPUTS_R3.json',
            json.dumps({
                'acceptance': 'quarantined',
                'all_outputs': all_outputs_map,
                'build_script_outputs': [],
                'host_fingerprints': [],
                'host_outputs': host_outputs,
            }),
            0o444,
        )
        self._write(
            q_packet / 'F_RETAINED_AUDIT_R3_RESULT.json',
            json.dumps({'native_outputs_accepted': False, 'phase': 'F'}),
            0o444,
        )

        expected_counts = {
            'total_preserved_files': len(copy_map_rows),
            'target_total': len(all_outputs_map),
            'packet_total': 2,
            'one_shot_total': 1,
            'owner_literal_workspace_fingerprint': 1,
            'owner_literal_workspace_deps': 1,
            'owner_literal_workspace_incremental': 1,
            'unadopted_other_cache_candidate': 12,
            'host_outputs_total': 4,
            'host_build_script_binaries': 2,
            'host_proc_macro_dylibs': 2,
            'build_script_run_units': 1,
        }
        c = {
            'paths': {
                'worktree': str(worktree),
                'execution_root': str(self.root / 'exec'),
                'source_home': str(source_home),
                'target': str(target_home),
                'packet': str(self.root / 'exec' / 'packet'),
            },
            'limits': {'F': {'packet_bytes': 67108864}},
            'readonly_tools': [],
        }
        Path(c['paths']['packet']).mkdir(parents=True, exist_ok=True)
        return c, q_root, copy_map_path, disp_path, expected_counts, all_outputs_map

    def test_target_partition_excludes_mori_workspace_and_preserves_quarantine_modes(self):
        c, q_root, copy_map_path, disp_path, expected_counts, all_outputs_map = self._make_synthetic_quarantine()
        saved_non_debug = copy.deepcopy(ccb.NON_DEBUG_CACHE_CANDIDATES)
        ccb.NON_DEBUG_CACHE_CANDIDATES = {
            '.rustc_info.json': all_outputs_map['.rustc_info.json'],
            'generated/bincode/ZkStdLibArchV1_Decode.rs': all_outputs_map['generated/bincode/ZkStdLibArchV1_Decode.rs'],
        }
        try:
            shutil.rmtree(c['paths']['target'])
            Path(c['paths']['target']).mkdir(parents=True, exist_ok=True)

            snapshot = ccb.verify_and_seed_cached_target(
                c,
                guard,
                quarantine_root=q_root,
                copy_map_file=copy_map_path,
                disposition_file=disp_path,
                expected_counts=expected_counts,
            )
            self.assertEqual(snapshot['partition_counts']['owner_literal_workspace_fingerprint'], 1)
            self.assertEqual(snapshot['partition_counts']['owner_literal_workspace_deps'], 1)
            self.assertEqual(snapshot['partition_counts']['owner_literal_workspace_incremental'], 1)
            self.assertEqual(snapshot['partition_counts']['unadopted_other_cache_candidate'], 12)

            dst_target = Path(c['paths']['target'])
            self.assertFalse((dst_target / 'debug/deps/libmori_backend-3a2f695515330e5a.rlib').exists())
            self.assertFalse((dst_target / 'debug/.fingerprint/mori-backend-3a2f695515330e5a').exists())
            self.assertTrue((dst_target / '.rustc_info.json').is_file())
            self.assertTrue((dst_target / 'generated/bincode/ZkStdLibArchV1_Decode.rs').is_file())

            for hrel in snapshot['host_outputs_seeded']:
                self.assertEqual(stat.S_IMODE((dst_target / hrel).lstat().st_mode), 0o755)
                self.assertEqual(stat.S_IMODE((q_root / 'target' / hrel).lstat().st_mode), 0o400)
            rlib_rel = 'debug/deps/libserde-5555666677778888.rlib'
            self.assertEqual(stat.S_IMODE((dst_target / rlib_rel).lstat().st_mode), 0o644)
            self.assertEqual(stat.S_IMODE((q_root / 'target' / rlib_rel).lstat().st_mode), 0o400)

            self._write(dst_target / 'debug/deps/mori_backend-9999888877776666', b'\x7fELF-new-backend', mode=0o755)
            post = ccb.verify_post_compile_no_script_reexec(c, guard, snapshot)
            self.assertEqual(post['host_outputs_verified_unchanged'], 4)
            self.assertEqual(post['build_script_run_units_verified_not_reexecuted'], 1)

            inv_p = dst_target / 'debug/build/serde-1111222233334444/invoked.timestamp'
            os.utime(inv_p, ns=(ccb.TARGET_MIN_EPOCH_NS + 9999, ccb.TARGET_MIN_EPOCH_NS + 9999))
            with self.assertRaises(guard.Refusal):
                ccb.verify_post_compile_no_script_reexec(c, guard, snapshot)
        finally:
            ccb.NON_DEBUG_CACHE_CANDIDATES = saved_non_debug

    def test_reconstitute_source_home_pure_extraction_and_generated_bookkeeping(self):
        q_root = self.root / 'q_reconst'
        q_packet = q_root / 'packet'
        q_packet.mkdir(parents=True, exist_ok=True)
        host_cargo = self.root / 'host_cargo'
        source_home = self.root / 'reconst_source_home'
        packet_dir = self.root / 'reconst_packet'
        source_home.mkdir(parents=True, exist_ok=True)
        packet_dir.mkdir(parents=True, exist_ok=True)

        crate_dir_name = 'objc2-0.6.5'
        lib_rs_bytes = b'pub fn hello() -> u32 { 42 }\n'
        lib_rs_sha = _sha256_bytes(lib_rs_bytes)
        archive_rel = f'registry/cache/index.crates.io-1949cf8c6b5b557f/{crate_dir_name}.crate'
        src_rel = f'registry/src/index.crates.io-1949cf8c6b5b557f/{crate_dir_name}'
        archive_src = self.root / 'acquired' / f'{crate_dir_name}.crate'
        archive_src.parent.mkdir(parents=True, exist_ok=True)
        with tarfile.open(archive_src, 'w:gz') as tf:
            info = tarfile.TarInfo(name=f'{crate_dir_name}/src/lib.rs')
            info.size = len(lib_rs_bytes)
            info.mode = 0o644
            tf.addfile(info, io.BytesIO(lib_rs_bytes))
        archive_sha = guard.digest(archive_src)

        git_root_04 = 'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d'
        git_root_a0 = 'git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea'
        git_file_bytes = b'// ledger lib\n'
        git_file_blob = _git_blob_sha1(git_file_bytes)
        git_file_sha = _sha256_bytes(git_file_bytes)
        git_sh_bytes = b'#!/bin/sh\necho ok\n'
        git_sh_blob = _git_blob_sha1(git_sh_bytes)
        git_sh_sha = _sha256_bytes(git_sh_bytes)
        sym_target = '.github/scripts/configure-cargo.sh'
        sym_blob = _git_blob_sha1(sym_target.encode('utf-8'))

        for g_root in (git_root_04, git_root_a0):
            self._write(host_cargo / g_root / 'src/lib.rs', git_file_bytes, 0o644)
            self._write(host_cargo / g_root / '.github/scripts/configure-cargo.sh', git_sh_bytes, 0o755)
            sym_p = host_cargo / g_root / 'link.rs'
            sym_p.parent.mkdir(parents=True, exist_ok=True)
            os.symlink(sym_target, sym_p)

        pack_rel_db = 'git/db/midnight-ledger-b2f9c59d942dfdca/objects/pack/pack-716c8728.pack'
        pack_rel_co = f'{git_root_a0}/.git/objects/pack/pack-716c8728.pack'
        pack_bytes = b'PACK-DATA'
        pack_sha = _sha256_bytes(pack_bytes)
        pack_src = self._write(host_cargo / pack_rel_db, pack_bytes, 0o644)
        head_rel_a0 = f'{git_root_a0}/.git/HEAD'
        head_bytes = b'ref: refs/heads/master\n'
        head_sha = _sha256_bytes(head_bytes)

        ok_rel = f'{src_rel}/.cargo-ok'
        ok_bytes = b'{"v":1}'
        ok_sha = _sha256_bytes(ok_bytes)
        lock_rel = '.global-cache'
        old_lock_bytes = b'old-global-cache'
        old_lock_sha = _sha256_bytes(old_lock_bytes)
        new_lock_bytes = b'new-global-cache-updated'
        new_lock_sha = _sha256_bytes(new_lock_bytes)
        snap_path = self._write(self.root / 'global-cache.immutable.snapshot', new_lock_bytes, 0o444)
        self._write(host_cargo / lock_rel, b'live-drifted-global-cache-after-snapshot', 0o644)

        pkg_cache_rel = '.package-cache'
        pkg_cache_sha = _sha256_bytes(b'')
        self._write(host_cargo / pkg_cache_rel, b'', 0o644)
        git_tag_rel = 'git/CACHEDIR.TAG'
        reg_tag_rel = 'registry/CACHEDIR.TAG'
        tag_bytes = b'Signature: 8a477f597d28d172789f06886806bc55\n'
        tag_sha = _sha256_bytes(tag_bytes)
        self._write(host_cargo / git_tag_rel, tag_bytes, 0o644)
        self._write(host_cargo / reg_tag_rel, tag_bytes, 0o644)

        # 8 git-control paths across 04c9c5d and a01a1ea (config, index, logs/HEAD, logs/refs/heads/master)
        cfg_bytes = (
            b'[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n\tlogallrefupdates = true\n'
            b'[remote "origin"]\n\turl = /home/charl/.cargo/git/db/midnight-ledger-b2f9c59d942dfdca\n'
            b'\tfetch = +refs/heads/*:refs/remotes/origin/*\n'
            b'[branch "master"]\n\tremote = origin\n\tmerge = refs/heads/master\n'
        )
        idx_04_bytes = b'DIRC' + (2).to_bytes(4, 'big') + (1).to_bytes(4, 'big') + b'index-04c9c5d-payload'
        idx_a0_bytes = b'DIRC' + (2).to_bytes(4, 'big') + (1).to_bytes(4, 'big') + b'index-a01a1ea-payload'
        log_04_bytes = b'0000000000000000000000000000000000000000 04c9c5d9bcebb8d4427d8589fb54d58a55599c14 reflog 04\n'
        log_a0_bytes = b'0000000000000000000000000000000000000000 a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1 reflog a0\n'

        eight_raw = [
            (f'{git_root_04}/.git/config', _sha256_bytes(b'old-cfg-04'), cfg_bytes),
            (f'{git_root_04}/.git/index', _sha256_bytes(b'old-idx-04'), idx_04_bytes),
            (f'{git_root_04}/.git/logs/HEAD', _sha256_bytes(b'old-log-04'), log_04_bytes),
            (f'{git_root_04}/.git/logs/refs/heads/master', _sha256_bytes(b'old-log-04'), log_04_bytes),
            (f'{git_root_a0}/.git/config', _sha256_bytes(b'old-cfg-a0'), cfg_bytes),
            (f'{git_root_a0}/.git/index', _sha256_bytes(b'old-idx-a0'), idx_a0_bytes),
            (f'{git_root_a0}/.git/logs/HEAD', _sha256_bytes(b'old-log-a0'), log_a0_bytes),
            (f'{git_root_a0}/.git/logs/refs/heads/master', _sha256_bytes(b'old-log-a0'), log_a0_bytes),
        ]
        snap_dir = self.root / 'eight_snapshots'
        eight_spec = {}
        eight_rows = []
        for rel_p, old_h, cur_b in eight_raw:
            cur_h = _sha256_bytes(cur_b)
            sp = self._write(snap_dir / rel_p, cur_b, 0o444)
            self._write(host_cargo / rel_p, b'live-drifted-' + cur_b, 0o644)
            eight_spec[rel_p] = {'historical_sha256': old_h, 'sha256': cur_h, 'bytes': len(cur_b), 'path': str(sp)}
            eight_rows.append({
                'historical_sha256': old_h,
                'kind': 'git-control',
                'proposed_current_snapshot': {'bytes': len(cur_b), 'path': str(sp), 'sha256': cur_h},
                'relative': rel_p,
            })
        eight_mf_path = self._write(self.root / 'EIGHT-CURRENT-SNAPSHOT-CANDIDATES.proposed.json', guard.json_bytes({'rows': eight_rows}), 0o444)
        eight_proj_obj = {
            'classification': 'synthetic validated',
            'full_manifest': {'bytes': eight_mf_path.stat().st_size, 'path': str(eight_mf_path), 'sha256': guard.digest(eight_mf_path)},
            'rows': eight_rows,
            'status': 'EIGHT_CURRENT_NONEXECUTED_SNAPSHOT_CANDIDATES_ONLY_NOT_HISTORICAL_BYTES_OR_ADOPTION',
            'total_snapshot_bytes': sum(len(b) for _, _, b in eight_raw),
        }
        eight_proj_path = self._write(self.root / 'EIGHT-SNAPSHOT-OWNER-PROJECTION.json', guard.json_bytes(eight_proj_obj), 0o444)

        old_files = {
            archive_rel: archive_sha,
            pack_rel_db: pack_sha,
            f'{src_rel}/src/lib.rs': lib_rs_sha,
            ok_rel: ok_sha,
            lock_rel: old_lock_sha,
            pkg_cache_rel: pkg_cache_sha,
            git_tag_rel: tag_sha,
            reg_tag_rel: tag_sha,
            f'{git_root_04}/src/lib.rs': git_file_sha,
            f'{git_root_04}/.github/scripts/configure-cargo.sh': git_sh_sha,
            f'{git_root_04}/link.rs': 'git-symlink:' + sym_blob,
            f'{git_root_a0}/src/lib.rs': git_file_sha,
            f'{git_root_a0}/.github/scripts/configure-cargo.sh': git_sh_sha,
            f'{git_root_a0}/link.rs': 'git-symlink:' + sym_blob,
            pack_rel_co: pack_sha,
            head_rel_a0: head_sha,
        }
        for rel_p, old_h, _ in eight_raw:
            old_files[rel_p] = old_h

        a0_obj = {
            'inputs': [
                {'destination': archive_rel, 'sha256': archive_sha, 'source': str(archive_src), 'stat': guard.stable_stat(archive_src)},
                {'destination': pack_rel_db, 'sha256': pack_sha, 'source': str(pack_src), 'stat': guard.stable_stat(pack_src)},
            ]
        }
        c_raw_obj = {
            'files': old_files,
            'cargo_ok': [{'path': ok_rel, 'sha256': ok_sha, 'bytes': len(ok_bytes)}],
            'raw_only': True,
        }
        d_freeze_obj = {'H_S': hashlib.sha256(guard.json_bytes(old_files)).hexdigest(), 'files': old_files}
        e_res_obj = {
            'host_units': [{'root': str(source_home / src_rel), 'name': 'objc2', 'target_name': 'build-script-build'}],
            'roots': {},
        }

        p_a0 = self._write(q_packet / 'A0_FREEZE.json', guard.json_bytes(a0_obj), 0o444)
        p_c = self._write(q_packet / 'C_RAW_IDENTITIES.json', guard.json_bytes(c_raw_obj), 0o444)
        p_d = self._write(q_packet / 'D_FREEZE.json', guard.json_bytes(d_freeze_obj), 0o444)
        p_e = self._write(q_packet / 'E_RESOLVED.json', guard.json_bytes(e_res_obj), 0o444)
        pins = {
            'A0_FREEZE.json': guard.digest(p_a0),
            'C_RAW_IDENTITIES.json': guard.digest(p_c),
            'D_FREEZE.json': guard.digest(p_d),
            'E_RESOLVED.json': guard.digest(p_e),
        }

        bk_check_obj = {
            'recordType': 'RootExactFourNonsecretCargoBookkeepingContentCheck',
            'utc': '2026-10-18T16:59:00.291913+00:00',
            'scope': 'synthetic',
            'files': [
                {
                    'path': str(host_cargo / lock_rel),
                    'bytes': len(new_lock_bytes),
                    'sha256': new_lock_sha,
                    'expected_D_FREEZE_sha256': old_lock_sha,
                    'matches': False,
                    'pre_post_stat_stable': True,
                    'immutable_snapshot': {'path': str(snap_path), 'bytes': len(new_lock_bytes), 'sha256': new_lock_sha},
                },
                {'path': str(host_cargo / pkg_cache_rel), 'bytes': 0, 'sha256': pkg_cache_sha, 'expected_D_FREEZE_sha256': pkg_cache_sha, 'matches': True, 'pre_post_stat_stable': True},
                {'path': str(host_cargo / git_tag_rel), 'bytes': len(tag_bytes), 'sha256': tag_sha, 'expected_D_FREEZE_sha256': tag_sha, 'matches': True, 'pre_post_stat_stable': True},
                {'path': str(host_cargo / reg_tag_rel), 'bytes': len(tag_bytes), 'sha256': tag_sha, 'expected_D_FREEZE_sha256': tag_sha, 'matches': True, 'pre_post_stat_stable': True},
            ],
            'all4_match': False,
        }
        bk_check_path = self._write(self.root / 'ROOT-FOUR-BOOKKEEPING-HASH-CHECK.json', guard.json_bytes(bk_check_obj), 0o444)

        c = {
            'paths': {'source_home': str(source_home), 'packet': str(packet_dir)},
            'limits': {'A0': {'packet_bytes': 67108864}, 'C': {'packet_bytes': 67108864}, 'D': {'packet_bytes': 67108864}},
            'source_copy_mounts': [str(host_cargo), str(archive_src)],
            'registry_roots': [{'archive': archive_rel, 'package_checksum': archive_sha, 'source_relative': src_rel}],
            'git_checkouts': [
                {
                    'commit': '04c9c5d9bcebb8d4427d8589fb54d58a55599c14',
                    'relative_root': git_root_04,
                    'files': [
                        {'blob': git_file_blob, 'mode': '100644', 'relative': 'src/lib.rs'},
                        {'blob': git_sh_blob, 'mode': '100755', 'relative': '.github/scripts/configure-cargo.sh'},
                        {'blob': sym_blob, 'mode': '120000', 'relative': 'link.rs'},
                    ],
                },
                {
                    'commit': 'a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1',
                    'relative_root': git_root_a0,
                    'files': [
                        {'blob': git_file_blob, 'mode': '100644', 'relative': 'src/lib.rs'},
                        {'blob': git_sh_blob, 'mode': '100755', 'relative': '.github/scripts/configure-cargo.sh'},
                        {'blob': sym_blob, 'mode': '120000', 'relative': 'link.rs'},
                    ],
                },
            ],
            'includes': [],
            'expected_generated_files': [
                {'relative': ok_rel, 'kind': 'cargo-ok'},
                {'relative': lock_rel, 'kind': 'cargo-lockfile'},
                {'relative': pkg_cache_rel, 'kind': 'cargo-lockfile'},
                {'relative': git_tag_rel, 'kind': 'cargo-lockfile'},
                {'relative': reg_tag_rel, 'kind': 'cargo-lockfile'},
                {'relative': pack_rel_co, 'kind': 'git-control'},
                {'relative': head_rel_a0, 'kind': 'git-control'},
                *[{'relative': rel_p, 'kind': 'git-control'} for rel_p, _, _ in eight_raw],
            ],
        }

        saved_approved_includes = guard.approved_includes
        saved_declared_links = guard.declared_links
        saved_consts = (
            ccb.GLOBAL_CACHE_OLD_D_SHA256,
            ccb.GLOBAL_CACHE_CURRENT_SHA256,
            ccb.GLOBAL_CACHE_CURRENT_BYTES,
            ccb.EIGHT_GIT_CONTROL_PROJECTION_PATH,
            ccb.EIGHT_GIT_CONTROL_PROJECTION_SHA256,
            ccb.EIGHT_GIT_CONTROL_PROJECTION_BYTES,
            ccb.EIGHT_GIT_CONTROL_MANIFEST_PATH,
            ccb.EIGHT_GIT_CONTROL_MANIFEST_SHA256,
            ccb.EIGHT_GIT_CONTROL_MANIFEST_BYTES,
            ccb.EIGHT_GIT_CONTROL_SNAPSHOT_SPEC,
        )
        guard.approved_includes = lambda _c: set()
        guard.declared_links = lambda _c: frozenset({source_home / git_root_04 / 'link.rs', source_home / git_root_a0 / 'link.rs'})
        ccb.GLOBAL_CACHE_OLD_D_SHA256 = old_lock_sha
        ccb.GLOBAL_CACHE_CURRENT_SHA256 = new_lock_sha
        ccb.GLOBAL_CACHE_CURRENT_BYTES = len(new_lock_bytes)
        ccb.EIGHT_GIT_CONTROL_PROJECTION_PATH = str(eight_proj_path)
        ccb.EIGHT_GIT_CONTROL_PROJECTION_SHA256 = guard.digest(eight_proj_path)
        ccb.EIGHT_GIT_CONTROL_PROJECTION_BYTES = eight_proj_path.stat().st_size
        ccb.EIGHT_GIT_CONTROL_MANIFEST_PATH = str(eight_mf_path)
        ccb.EIGHT_GIT_CONTROL_MANIFEST_SHA256 = guard.digest(eight_mf_path)
        ccb.EIGHT_GIT_CONTROL_MANIFEST_BYTES = eight_mf_path.stat().st_size
        ccb.EIGHT_GIT_CONTROL_SNAPSHOT_SPEC = eight_spec

        def _reset_dirs():
            for base, dirs, _ in os.walk(source_home, topdown=True, followlinks=False):
                os.chmod(base, 0o700)
                for d in dirs:
                    dp = Path(base) / d
                    if not dp.is_symlink():
                        os.chmod(dp, 0o700)
            shutil.rmtree(source_home)
            shutil.rmtree(packet_dir)
            source_home.mkdir(parents=True, exist_ok=True)
            packet_dir.mkdir(parents=True, exist_ok=True)

        try:
            rep = ccb.reconstitute_source_home(
                c,
                guard,
                quarantine_root=q_root,
                expected_pins=pins,
                bookkeeping_check_file=bk_check_path,
            )
            self.assertEqual(rep['authenticated_source_files_verified'], 9)
            self.assertEqual(rep['expected_generated_files_verified'], 15)
            self.assertEqual(len(rep['dated_bookkeeping_deltas']), 9)
            delta_by_rel = {d['relative']: d for d in rep['dated_bookkeeping_deltas']}
            self.assertEqual(set(delta_by_rel.keys()), {lock_rel} | set(eight_spec.keys()))
            self.assertEqual(guard.digest(source_home / lock_rel), new_lock_sha)
            for rel_p, spec in eight_spec.items():
                self.assertEqual(guard.digest(source_home / rel_p), spec['sha256'])
            sh_mode = stat.S_IMODE((source_home / git_root_a0 / '.github/scripts/configure-cargo.sh').lstat().st_mode)
            self.assertEqual(sh_mode, 0o555)
            guard.frozen_source(c)

            # Disallowed .git/config key (e.g. fsmonitor) in snapshot must refuse.
            cfg_snap_04 = Path(eight_spec[f'{git_root_04}/.git/config']['path'])
            bad_cfg_bytes = cfg_bytes + b'\tfsmonitor = /bin/sh\n'
            os.chmod(cfg_snap_04, 0o644)
            self._write(cfg_snap_04, bad_cfg_bytes, 0o444)
            eight_spec[f'{git_root_04}/.git/config']['sha256'] = _sha256_bytes(bad_cfg_bytes)
            eight_spec[f'{git_root_04}/.git/config']['bytes'] = len(bad_cfg_bytes)
            eight_proj_obj['rows'][0]['proposed_current_snapshot']['sha256'] = _sha256_bytes(bad_cfg_bytes)
            eight_proj_obj['rows'][0]['proposed_current_snapshot']['bytes'] = len(bad_cfg_bytes)
            eight_proj_obj['total_snapshot_bytes'] = sum(r['proposed_current_snapshot']['bytes'] for r in eight_proj_obj['rows'])
            os.chmod(eight_proj_path, 0o644)
            self._write(eight_proj_path, guard.json_bytes(eight_proj_obj), 0o444)
            ccb.EIGHT_GIT_CONTROL_PROJECTION_SHA256 = guard.digest(eight_proj_path)
            ccb.EIGHT_GIT_CONTROL_PROJECTION_BYTES = eight_proj_path.stat().st_size
            _reset_dirs()
            with self.assertRaises(guard.Refusal):
                ccb.reconstitute_source_home(c, guard, quarantine_root=q_root, expected_pins=pins, bookkeeping_check_file=bk_check_path)

            # Mutating the .global-cache immutable snapshot file to an unrecorded digest must also refuse.
            os.chmod(snap_path, 0o644)
            self._write(snap_path, b'unrecorded-global-cache-snapshot-mutation', 0o444)
            _reset_dirs()
            with self.assertRaises(guard.Refusal):
                ccb.reconstitute_source_home(c, guard, quarantine_root=q_root, expected_pins=pins, bookkeeping_check_file=bk_check_path)
        finally:
            guard.approved_includes = saved_approved_includes
            guard.declared_links = saved_declared_links
            (
                ccb.GLOBAL_CACHE_OLD_D_SHA256,
                ccb.GLOBAL_CACHE_CURRENT_SHA256,
                ccb.GLOBAL_CACHE_CURRENT_BYTES,
                ccb.EIGHT_GIT_CONTROL_PROJECTION_PATH,
                ccb.EIGHT_GIT_CONTROL_PROJECTION_SHA256,
                ccb.EIGHT_GIT_CONTROL_PROJECTION_BYTES,
                ccb.EIGHT_GIT_CONTROL_MANIFEST_PATH,
                ccb.EIGHT_GIT_CONTROL_MANIFEST_SHA256,
                ccb.EIGHT_GIT_CONTROL_MANIFEST_BYTES,
                ccb.EIGHT_GIT_CONTROL_SNAPSHOT_SPEC,
            ) = saved_consts

    def test_standing_beta_tier_grant_and_unsealed_base_refusal(self):
        beta_file = self._write(self.root / 'beta-assignment.md', b'standing beta automation\n', 0o444)
        beta_sha = guard.digest(beta_file)
        saved_g_path, saved_g_sha = guard.BETA_ASSIGNMENT_PATH, guard.BETA_ASSIGNMENT_SHA256
        saved_c_path, saved_c_sha = ccb.BETA_ASSIGNMENT_PATH, ccb.BETA_ASSIGNMENT_SHA256
        guard.BETA_ASSIGNMENT_PATH = str(beta_file)
        guard.BETA_ASSIGNMENT_SHA256 = beta_sha
        ccb.BETA_ASSIGNMENT_PATH = str(beta_file)
        ccb.BETA_ASSIGNMENT_SHA256 = beta_sha
        try:
            c = {
                'mode': 'tier1-and-compile',
                'bindings': {
                    'config_inputs': {'sha256': '11' * 32},
                    'amendments': {'sha256': '22' * 32},
                },
                'review_receipts': {
                    'root': {'sha256': '33' * 32},
                    'tier1': {'sha256': '44' * 32},
                },
            }
            records = {
                'tier1': {
                    'schema_version': 1,
                    'kind': 'root_standing_beta_tier1_grant',
                    'tier': 1,
                    'decision': 'GRANTED_STANDING_BETA_TIER1',
                    'authorization_mode': 'root_standing_beta_override',
                    'beta_assignment_path': str(beta_file),
                    'beta_assignment_sha256': beta_sha,
                    'root_receipt_sha256': '33' * 32,
                    'config_inputs_sha256': '11' * 32,
                    'amendments_sha256': '22' * 32,
                    'authority_granted_by_this_file_alone': False,
                },
                'tier2': {
                    'schema_version': 1,
                    'kind': 'root_standing_beta_tier2_grant',
                    'tier': 2,
                    'decision': 'GRANTED_STANDING_BETA_TIER2',
                    'authorization_mode': 'root_standing_beta_override',
                    'beta_assignment_path': str(beta_file),
                    'beta_assignment_sha256': beta_sha,
                    'root_receipt_sha256': '33' * 32,
                    'config_inputs_sha256': '11' * 32,
                    'amendments_sha256': '22' * 32,
                    'tier1_receipt_sha256': '44' * 32,
                    'authority_granted_by_this_file_alone': False,
                },
            }
            guard._validate_tier_grant(c, records, 1)
            guard._validate_tier_grant(c, records, 2)
            ccb.validate_standing_beta_tier_grant(c, guard, records, 1)
            ccb.validate_standing_beta_tier_grant(c, guard, records, 2)

            bad_records = copy.deepcopy(records)
            bad_records['tier1']['beta_assignment_sha256'] = 'ff' * 32
            with self.assertRaises(guard.Refusal):
                guard._validate_tier_grant(c, bad_records, 1)

            guard.BASE = ccb.DEFERRED_CURRENT_BASE
            with self.assertRaises(guard.Refusal):
                guard.source_binding({})
            self.assertIsNotNone(guard.HEX.fullmatch(guard.CACHED_CURRENT_BUILD_SHA256))
        finally:
            guard.BETA_ASSIGNMENT_PATH = saved_g_path
            guard.BETA_ASSIGNMENT_SHA256 = saved_g_sha
            ccb.BETA_ASSIGNMENT_PATH = saved_c_path
            ccb.BETA_ASSIGNMENT_SHA256 = saved_c_sha

    def test_post_r3_historical_debits_enforced(self):
        cg_base = self.root / 'cgroup_fs'
        env_cg = cg_base / 'moriarty-sherlock-ledger9-20261018-06'
        sup_cg = env_cg / 'supervisor'
        ph_cg = env_cg / 'phases'
        sup_cg.mkdir(parents=True, exist_ok=True)
        ph_cg.mkdir(parents=True, exist_ok=True)
        self._write(env_cg / 'cpu.stat', 'usage_usec 0\n', 0o644)
        self._write(sup_cg / 'cpu.stat', 'usage_usec 0\n', 0o644)
        self._write(ph_cg / 'cpu.stat', 'usage_usec 0\n', 0o644)

        saved_cg_fs = guard.cg_fs
        guard.cg_fs = lambda ns: cg_base / ns.lstrip('/')
        try:
            c = {
                'total_wall_seconds': 6038,
                'supervisor_cpu_seconds': 10576,
                'limits': {'F': {'wall_seconds': 2133, 'cpu_time_us': 10800000000}},
                'cgroup': {
                    'envelope_path': '/moriarty-sherlock-ledger9-20261018-06',
                    'supervisor_leaf_name': 'supervisor',
                    'phase_parent_name': 'phases',
                },
            }
            state = ccb.enforce_post_r3_historical_debits(c, guard, cleanup_receipt_file=None)
            self.assertEqual(state['effective_envelope_usec_at_start'], 962803819)
            self.assertEqual(state['effective_supervisor_usec_at_start'], 223293846)
            self.assertEqual(state['effective_phases_usec_at_start'], 739509969)
            self.assertEqual(state['historical_debits']['retired_p5_envelope_usec'], 37863695)
            self.assertEqual(state['historical_debits']['retired_p5_supervisor_usec'], 37863695)
            self.assertEqual(state['historical_debits']['retired_p5_phases_usec'], 0)
            self.assertEqual(state['historical_debits']['retired_p6_envelope_usec'], 39019876)
            self.assertEqual(state['historical_debits']['retired_p6_supervisor_usec'], 39019876)
            self.assertEqual(state['historical_debits']['retired_p6_phases_usec'], 0)
            self.assertEqual(state['historical_debits']['retired_cached_attempts_envelope_usec'], 616605107)
            self.assertEqual(state['historical_debits']['retired_cached_attempts_supervisor_usec'], 147378816)
            self.assertEqual(state['historical_debits']['retired_cached_attempts_phases_usec'], 469226288)
            self.assertEqual(state['historical_debits']['retired_p7_envelope_usec'], 187121653)
            self.assertEqual(state['historical_debits']['retired_p7_supervisor_usec'], 15162810)
            self.assertEqual(state['historical_debits']['retired_p7_phases_usec'], 171958842)
            self.assertEqual(state['historical_debits']['p5_refusal_wall_seconds_ceil'], 40)
            self.assertEqual(state['historical_debits']['p6_refusal_wall_seconds_ceil'], 44)
            self.assertEqual(state['historical_debits']['p7_outer_wall_seconds_ceil'], 135)
            self.assertEqual(state['historical_debits']['retired_p8_envelope_usec'], 198086463)
            self.assertEqual(state['historical_debits']['retired_p8_supervisor_usec'], 12326620)
            self.assertEqual(state['historical_debits']['retired_p8_phases_usec'], 185759842)
            self.assertEqual(state['historical_debits']['p8_outer_wall_seconds_ceil'], 133)
            self.assertEqual(state['historical_debits']['retired_p9_envelope_usec'], 127323100)
            self.assertEqual(state['historical_debits']['retired_p9_supervisor_usec'], 15815495)
            self.assertEqual(state['historical_debits']['retired_p9_phases_usec'], 111507604)
            self.assertEqual(state['historical_debits']['p9_outer_wall_seconds_ceil'], 121)
            self.assertEqual(state['historical_debits']['f_stdout_bytes'], 0)
            self.assertEqual(state['historical_debits']['f_stderr_bytes'], 43081)
            self.assertEqual(state['effective_envelope_usec_at_start'] - state['effective_supervisor_usec_at_start']
                             - state['effective_phases_usec_at_start'], 4)
            self.assertEqual(state['historical_debits']['supervisor_cpu_seconds_ceil'], 224)
            self.assertEqual(state['historical_debits']['remaining_total_wall_seconds_max'], 6038)
            self.assertEqual(state['historical_debits']['remaining_f_wall_seconds_max'], 2133)
            self.assertEqual(state['historical_debits']['remaining_supervisor_cpu_seconds_max'], 10576)

            self._write(env_cg / 'cpu.stat', 'usage_usec 400000000\n', 0o644)
            self._write(sup_cg / 'cpu.stat', 'usage_usec 100000000\n', 0o644)
            self._write(ph_cg / 'cpu.stat', 'usage_usec 300000000\n', 0o644)
            state_post = ccb.enforce_post_r3_historical_debits(c, guard, cleanup_receipt_file=None, at_start=False)
            self.assertEqual(state_post['effective_envelope_usec_at_start'], 962803819 + 400000000)
            self.assertEqual(state_post['effective_supervisor_usec_at_start'], 223293846 + 100000000)
            self.assertEqual(state_post['effective_phases_usec_at_start'], 739509969 + 300000000)
            state_again = ccb.enforce_post_r3_historical_debits(c, guard, cleanup_receipt_file=None, at_start=False)
            self.assertEqual(state_again['effective_phases_usec_at_start'], 739509969 + 300000000)
            self.assertEqual(state_again['historical_debits'], state_post['historical_debits'])

            c_over_wall = copy.deepcopy(c)
            c_over_wall['total_wall_seconds'] = 6039
            with self.assertRaises(guard.Refusal):
                ccb.enforce_post_r3_historical_debits(c_over_wall, guard, cleanup_receipt_file=None)

            c_over_f = copy.deepcopy(c)
            c_over_f['limits']['F']['wall_seconds'] = 2134
            with self.assertRaises(guard.Refusal):
                ccb.enforce_post_r3_historical_debits(c_over_f, guard, cleanup_receipt_file=None)

            c_over_sup = copy.deepcopy(c)
            c_over_sup['supervisor_cpu_seconds'] = 10577
            with self.assertRaises(guard.Refusal):
                ccb.enforce_post_r3_historical_debits(c_over_sup, guard, cleanup_receipt_file=None)

            for key, value in (('total_wall_seconds', 6427), ('supervisor_cpu_seconds', 10620), ('F', 2522)):
                c_old = copy.deepcopy(c)
                if key == 'F':
                    c_old['limits']['F']['wall_seconds'] = value
                else:
                    c_old[key] = value
                with self.assertRaises(guard.Refusal):
                    ccb.enforce_post_r3_historical_debits(c_old, guard, cleanup_receipt_file=None)

            for key, value in (('total_wall_seconds', 6292), ('supervisor_cpu_seconds', 10604), ('F', 2387)):
                c_old = copy.deepcopy(c)
                if key == 'F':
                    c_old['limits']['F']['wall_seconds'] = value
                else:
                    c_old[key] = value
                with self.assertRaises(guard.Refusal):
                    ccb.enforce_post_r3_historical_debits(c_old, guard, cleanup_receipt_file=None)

            for key, value in (('total_wall_seconds', 6159), ('supervisor_cpu_seconds', 10592), ('F', 2254)):
                c_old = copy.deepcopy(c)
                if key == 'F':
                    c_old['limits']['F']['wall_seconds'] = value
                else:
                    c_old[key] = value
                with self.assertRaises(guard.Refusal):
                    ccb.enforce_post_r3_historical_debits(c_old, guard, cleanup_receipt_file=None)

            saved_debits = ccb.HISTORICAL_POST_R3_DEBITS
            try:
                for key, value in (
                    ('envelope_usec', 835480719 + 187121653),
                    ('phases_usec', 628002365 + 171958842),
                    ('retired_p7_envelope_usec', 15162810 + 171958842),
                    ('envelope_usec', 835480719 + 198086463),
                    ('supervisor_usec', 207478351 + 12326620),
                    ('phases_usec', 628002365 + 185759842),
                    ('envelope_usec', 835480719 - 198086463),
                    ('supervisor_usec', 207478351 - 12326620),
                    ('phases_usec', 628002365 - 185759842),
                    ('envelope_usec', 207478351 + 628002365),
                    ('retired_p8_envelope_usec', 12326620 + 185759842),
                    ('retired_p8_supervisor_usec', 0),
                    ('retired_p8_phases_usec', 0),
                    ('retired_cached_attempts_envelope_usec', 489282007 - 198086463),
                    ('envelope_usec', 962803819 + 127323100),
                    ('phases_usec', 739509969 + 111507604),
                    ('envelope_usec', 962803819 - 127323100),
                    ('retired_p9_envelope_usec', 15815495 + 111507604),
                    ('retired_p9_supervisor_usec', 0),
                    ('retired_p9_phases_usec', 0),
                    ('p9_outer_wall_seconds_ceil', 0),
                    ('f_stderr_bytes', 43081 - 8880),
                    ('remaining_total_wall_seconds_max', 6159),
                    ('remaining_f_wall_seconds_max', 2254),
                    ('remaining_supervisor_cpu_seconds_max', 10592),
                    ('remaining_total_wall_seconds_max', 6292),
                    ('remaining_f_wall_seconds_max', 2387),
                    ('remaining_supervisor_cpu_seconds_max', 10604),
                    ('gross_supervisor_cpu_seconds_max', 10801),
                    ('p8_outer_wall_seconds_ceil', 0),
                    ('f_stderr_bytes', 34201 - 137),
                    ('f_stderr_bytes', 34201 - 12829),

                ):
                    ccb.HISTORICAL_POST_R3_DEBITS = dict(saved_debits, **{key: value})
                    with self.assertRaises(guard.Refusal):
                        ccb.enforce_post_r3_historical_debits(c, guard, cleanup_receipt_file=None)
            finally:
                ccb.HISTORICAL_POST_R3_DEBITS = saved_debits
        finally:
            guard.cg_fs = saved_cg_fs

    def test_p7_receipts_once_only_pinned_and_tamper_refused(self):
        names = ('ATTEMPT_RESULT', 'RAW_BEFORE_CLEANUP', 'PRESERVE_CLEANUP_RESULT')
        saved = {n + s: getattr(ccb, 'P7_' + n + s) for n in names for s in ('_PATH', '_SHA256', '_BYTES')}

        def pin(name, obj):
            p = self._write(self.root / 'p7' / (name + '.json'), guard.json_bytes(obj), 0o644)
            setattr(ccb, 'P7_' + name + '_PATH', str(p))
            setattr(ccb, 'P7_' + name + '_SHA256', guard.digest(p))
            setattr(ccb, 'P7_' + name + '_BYTES', p.stat().st_size)

        def node(usage, peak):
            events = 'low 0\nhigh 0\nmax 0\noom 0\noom_kill 0\noom_group_kill 0\n'
            return {
                'cpu.stat': 'usage_usec %d\nuser_usec 1\n' % usage,
                'cgroup.events': 'populated 0\nfrozen 0\n',
                'cgroup.procs': '',
                'memory.events': events,
                'memory.events.local': events,
                'memory.peak': '%d\n' % peak,
                'pids.current': '0\n',
            }

        attempt = {
            'id': 'P7-fresh-setup-current-stat-issued-DAG-and-changed-native-compile',
            'actual_exit': 2,
            'elapsed_seconds': 134.75891540400335,
            'controller_failure': None,
            'streams': [
                {'bytes': 2956, 'sha256': '593a73e0c81284b558ec8b07b4fad37bc3ac45da7ba4788121c162d91593cec5', 'full_stream_retained': True},
                {'bytes': 39, 'sha256': '9b4078eb89bbadd2597138fa1536d7732ccfa032574ab7218427a91f1a3a28bd', 'full_stream_retained': True},
            ],
        }
        raw = {
            'raw': {
                'E': node(187121653, 5701033984),
                'S': node(15162810, 80003072),
                'P': node(171958842, 5649174528),
                'marker': {'path': ccb.P7_MARKER_PATH, 'bytes': 9, 'sha256': '37cb06c8f02da9953c59a7186a4bbdca294494149fdf65f831d8b493ae5bcf67'},
            },
            'cleanupOwed': True,
            'nativeOutputsAccepted': False,
            'CargoDidStart': True,
        }
        steps = [{'step': s, 'actualExit': 0} for s in ccb.P7_CLEANUP_STEPS]
        steps[4] = {'step': ccb.P7_CLEANUP_STEPS[4], 'exactMembershipHashesSizesAndNonexecModes': True, 'files': 3299}
        steps[5]['streams'] = [{'bytes': 165, 'sha256': 'bc21db5edf04a7b73e3c813e0fc2d73db4610576ac39775953c3c08d99783a32'}]
        steps[6]['streams'] = [{'bytes': 128, 'sha256': '711c09e6d455edcc1a5c02ef433342689ce283699cc5ebf2541f4af664a18c41'}]
        cleanup = {
            'status': 'P7_PRESERVED_AND_CLEANED',
            'steps': steps,
            'cleanupOwed': False,
            'nativeOutputsAccepted': False,
            'ELFExecuted': False,
            'targetFilesPreserved': 3286,
            'allFilesPreserved': 3299,
            'quarantine': ccb.P7_PERSISTENT_QUARANTINE_ROOT,
            'totalBytesPreserved': 1985171956,
            'CargoDidStart': True,
            'ordinaryRustCompilerErrors': 2,
        }
        fixtures = {'ATTEMPT_RESULT': attempt, 'RAW_BEFORE_CLEANUP': raw, 'PRESERVE_CLEANUP_RESULT': cleanup}
        try:
            for name, obj in fixtures.items():
                pin(name, obj)
            ccb._verify_p7_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            for name, keys, value in (
                ('RAW_BEFORE_CLEANUP', ('raw', 'E', 'cpu.stat'), 'usage_usec %d\n' % (15162810 + 171958842)),
                ('RAW_BEFORE_CLEANUP', ('raw', 'P', 'memory.events'), 'low 0\noom 1\noom_kill 1\n'),
                ('ATTEMPT_RESULT', ('elapsed_seconds',), 135.5),
                ('PRESERVE_CLEANUP_RESULT', ('cleanupOwed',), True),
            ):
                bad = copy.deepcopy(fixtures[name])
                target = bad
                for k in keys[:-1]:
                    target = target[k]
                target[keys[-1]] = value
                pin(name, bad)
                with self.assertRaises(guard.Refusal):
                    ccb._verify_p7_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
                pin(name, fixtures[name])
            ccb._verify_p7_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            ccb.P7_PRESERVE_CLEANUP_RESULT_SHA256 = '00' * 32
            with self.assertRaises(guard.Refusal):
                ccb._verify_p7_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
        finally:
            for key, value in saved.items():
                setattr(ccb, 'P7_' + key, value)

    def test_p8_receipts_once_only_pinned_and_tamper_refused(self):
        names = ('ATTEMPT_RESULT', 'RAW_BEFORE_CLEANUP', 'PRESERVE_CLEANUP_RESULT', 'TERMINAL_FAILURE', 'LOG_BASIS')
        saved = {n + s: getattr(ccb, 'P8_' + n + s) for n in names for s in ('_PATH', '_SHA256', '_BYTES')}

        def pin(name, obj):
            p = self._write(self.root / 'p8' / (name + '.json'), guard.json_bytes(obj), 0o644)
            setattr(ccb, 'P8_' + name + '_PATH', str(p))
            setattr(ccb, 'P8_' + name + '_SHA256', guard.digest(p))
            setattr(ccb, 'P8_' + name + '_BYTES', p.stat().st_size)

        def node(usage, peak):
            events = 'low 0\nhigh 0\nmax 0\noom 0\noom_kill 0\noom_group_kill 0\n'
            return {'cpu.stat': 'usage_usec %d\nuser_usec 1\n' % usage,
                    'cgroup.events': 'populated 0\nfrozen 0\n', 'cgroup.procs': '',
                    'memory.events': events, 'memory.events.local': events,
                    'memory.peak': '%d\n' % peak, 'pids.current': '0\n'}

        attempt = {
            'id': 'P8-fresh-setup-current-stat-issued-DAG-and-two-test-fixed-native-compile',
            'actual_exit': 2, 'controller_failure': None, 'elapsed_seconds': 132.46132113799104,
            'streams': [
                {'bytes': 2990, 'sha256': '5b12a998da8bfa80df2ab35a51d761d3018fb7394b144178507e5c7c51c86ba9', 'full_stream_retained': True},
                {'bytes': 39, 'sha256': '9b4078eb89bbadd2597138fa1536d7732ccfa032574ab7218427a91f1a3a28bd', 'full_stream_retained': True},
            ],
        }
        raw = {
            'raw': {'E': node(198086463, 5911670784), 'S': node(12326620, 71958528),
                    'P': node(185759842, 5859758080),
                    'marker': {'path': ccb.P7_MARKER_PATH, 'bytes': 9,
                               'sha256': '37cb06c8f02da9953c59a7186a4bbdca294494149fdf65f831d8b493ae5bcf67'}},
            'cleanupOwed': True, 'nativeOutputsAccepted': False, 'CargoDidStart': True,
            'CargoCompileReportedFinished': True, 'ELFExecuted': False,
            'postAuditFailure': 'third-party host output mtime changed during cached Phase F: debug/build/aws-lc-sys-5469586605af0f08/build-script-main',
        }
        steps = [{'step': st, 'actualExit': 0} for st in ccb.P7_CLEANUP_STEPS]
        steps[4] = {'step': ccb.P7_CLEANUP_STEPS[4], 'exactMembershipHashesSizesAndNonexecModes': True, 'files': 3565}
        steps[5]['streams'] = [{'bytes': 165, 'sha256': '8527b3723c3feaee209027a88fbb123a1d7cf1f7ce39614eb7fa99b50477c7df'}]
        steps[6]['streams'] = [{'bytes': 128, 'sha256': '845999dd73a18885b43917a80a6cc7c77841126bc27a06c96f5a754e03acbc22'}]
        cleanup = {
            'status': 'P8_PRESERVED_AND_CLEANED', 'steps': steps, 'cleanupOwed': False,
            'nativeOutputsAccepted': False, 'ELFExecuted': False, 'CargoDidStart': True,
            'CargoCompileReportedSuccess': True, 'CargoBuildSecondsReported': 55.6,
            'guardPostAudit': 'FAILED aws-lc-sys build-script-main mtime changed',
            'targetFilesPreserved': 3548, 'allFilesPreserved': 3565, 'totalBytesPreserved': 2847417755,
            'quarantine': ccb.P8_PERSISTENT_QUARANTINE_ROOT, 'cgroupAbsent': True, 'executionRootEmpty': True,
        }
        terminal = {'message': 'phase exit nonzero F', 'native_outputs_accepted': False,
                    'route': 'cached-current-f', 'type': 'Refusal'}
        # Synthetic ordinary files have the independent historical byte counts, no SDK or captured payloads.
        specs = (
            ('r3', (('F', 10237), ('F_audit', 47), ('F_precheck', 0),
                    ('F_retained_audit', 47), ('F_retained_audit_r2', 75), ('F_retained_audit_r3', 0))),
            ('P7', (('F', 12829), ('F_precheck', 0))),
            ('P8', (('F', 10829), ('F_audit', 137), ('F_precheck', 0))),
        )
        rows = []
        for label, launches in specs:
            files = []
            for launch, size in launches:
                for kind, count in (('stdout', 0), ('stderr', size)):
                    p = self._write(self.root / 'streams' / label / (launch + '.' + kind), b'x' * count)
                    files.append({'path': str(p), 'bytes': count, 'sha256': guard.digest(p)})
            rows.append({'attempt': label, 'stream_files': files,
                         'subtotal': {'stdout': 0, 'stderr': sum(n for _, n in launches)}})
        basis = {'status': 'SAVED_PUBLIC_STREAM_BYTE_PINS_AND_ARITHMETIC_NOT_SOURCE_APPROVAL', 'rows': rows,
                 'new_cumulative_all_F_stdout': 0, 'new_cumulative_all_F_stderr': 34201}
        fixtures = {'ATTEMPT_RESULT': attempt, 'RAW_BEFORE_CLEANUP': raw, 'PRESERVE_CLEANUP_RESULT': cleanup,
                    'TERMINAL_FAILURE': terminal, 'LOG_BASIS': basis}
        try:
            for name, obj in fixtures.items():
                pin(name, obj)
            before = copy.deepcopy(ccb.HISTORICAL_POST_R3_DEBITS)
            ccb._verify_p8_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            ccb._verify_p8_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            self.assertEqual(ccb.HISTORICAL_POST_R3_DEBITS, before)
            # Re-pin altered receipts to exercise semantics independently of immutable production digests.
            for name, keys, value in (
                ('ATTEMPT_RESULT', ('actual_exit',), 0),
                ('ATTEMPT_RESULT', ('elapsed_seconds',), 131.5),
                ('ATTEMPT_RESULT', ('streams', 0, 'full_stream_retained'), False),
                ('RAW_BEFORE_CLEANUP', ('raw', 'E', 'cpu.stat'), 'usage_usec 198086462\n'),
                ('RAW_BEFORE_CLEANUP', ('raw', 'S', 'cpu.stat'), 'usage_usec 0\n'),
                ('RAW_BEFORE_CLEANUP', ('raw', 'P', 'cpu.stat'), 'usage_usec 0\n'),
                ('RAW_BEFORE_CLEANUP', ('raw', 'P', 'cgroup.events'), 'populated 1\n'),
                ('RAW_BEFORE_CLEANUP', ('raw', 'P', 'memory.events'), 'low 0\noom 1\noom_kill 1\n'),
                ('RAW_BEFORE_CLEANUP', ('raw', 'marker', 'sha256'), '00' * 32),
                ('RAW_BEFORE_CLEANUP', ('postAuditFailure',), None),
                ('RAW_BEFORE_CLEANUP', ('nativeOutputsAccepted',), True),
                ('PRESERVE_CLEANUP_RESULT', ('cleanupOwed',), True),
                ('PRESERVE_CLEANUP_RESULT', ('nativeOutputsAccepted',), True),
                ('PRESERVE_CLEANUP_RESULT', ('ELFExecuted',), True),
                ('PRESERVE_CLEANUP_RESULT', ('cgroupAbsent',), False),
                ('PRESERVE_CLEANUP_RESULT', ('executionRootEmpty',), False),
                ('PRESERVE_CLEANUP_RESULT', ('guardPostAudit',), 'PASS'),
                ('PRESERVE_CLEANUP_RESULT', ('steps', 6, 'actualExit'), 2),
                ('PRESERVE_CLEANUP_RESULT', ('steps', 4, 'exactMembershipHashesSizesAndNonexecModes'), False),
                ('TERMINAL_FAILURE', ('native_outputs_accepted',), True),
                ('TERMINAL_FAILURE', ('type',), 'Success'),
                ('LOG_BASIS', ('rows', 1, 'attempt'), 'P8'),
                ('LOG_BASIS', ('rows', 2, 'stream_files'), rows[2]['stream_files'][:-2]),
                ('LOG_BASIS', ('rows', 2, 'stream_files', 0), rows[1]['stream_files'][0]),
                ('LOG_BASIS', ('rows', 2, 'subtotal', 'stderr'), 10829),
                ('LOG_BASIS', ('new_cumulative_all_F_stderr',), 34201 - 137),
            ):
                with self.subTest(receipt=name, keys=keys):
                    bad = copy.deepcopy(fixtures[name])
                    target = bad
                    for key in keys[:-1]:
                        target = target[key]
                    target[keys[-1]] = value
                    pin(name, bad)
                    with self.assertRaises(guard.Refusal):
                        ccb._verify_p8_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
                    pin(name, fixtures[name])
            # A real retained-file byte change must reject without changing its receipt.
            audit_path = Path(rows[2]['stream_files'][3]['path'])
            old_bytes = audit_path.read_bytes()
            self._write(audit_path, b'y' * len(old_bytes))
            with self.assertRaises(guard.Refusal):
                ccb._verify_p8_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            self._write(audit_path, old_bytes)
            for name in names:
                with self.subTest(pin=name):
                    setattr(ccb, 'P8_' + name + '_SHA256', '00' * 32)
                    with self.assertRaises(guard.Refusal):
                        ccb._verify_p8_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
                    pin(name, fixtures[name])
            with self.assertRaises(guard.Refusal):
                ccb._verify_p8_receipts(dict(before, f_stderr_bytes=34201 - 12829), guard)
            with self.assertRaises(guard.Refusal):
                ccb._verify_p8_receipts(dict(before, retired_p8_envelope_usec=198086462), guard)
            ccb._verify_p8_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            self.assertEqual(ccb.HISTORICAL_POST_R3_DEBITS, before)
        finally:
            for key, value in saved.items():
                setattr(ccb, 'P8_' + key, value)

    def _legacy_two_tier_mtime_ns(self, rel):
        # Replica of the removed P8 tier expression, used only to show each fixture discriminates it.
        late = (
            rel.startswith('debug/.fingerprint/')
            or rel.startswith('debug/deps/')
            or (rel.startswith('debug/') and rel.count('/') == 1)
        )
        return ccb.TARGET_FINGERPRINT_EPOCH_NS if late else ccb.TARGET_MIN_EPOCH_NS

    def test_seed_reapplies_preserved_r3_mtimes_in_dependency_order(self):
        c, q_root, copy_map_path, disp_path, expected_counts, all_outputs_map = self._make_synthetic_quarantine()
        q_target = q_root / 'target'
        unit = 'debug/build/serde-1111222233334444/'
        rlib_rel = 'debug/deps/libserde-5555666677778888.rlib'
        bin_rels = (unit + 'build_script_build-aaaa1111bbbb2222', unit + 'build-script-build')
        workspace_rels = (
            'debug/.fingerprint/mori-backend-3a2f695515330e5a/dep-lib-mori_backend',
            'debug/deps/libmori_backend-3a2f695515330e5a.rlib',
            'debug/incremental/mori_backend-1a2b3c4d5e6f7a8b/s-1234-abcd/dep-graph.bin',
        )
        # Finite supplied mtimes: the rlib->script edge reuses actual r3 autocfg-rlib/num-traits-script values;
        # marker values follow the dirty build-script sequence (output reset to the invocation time).
        supplied = {
            '.rustc_info.json': 1_792_317_334_779_394_537,
            'generated/bincode/ZkStdLibArchV1_Decode.rs': 1_792_317_334_800_000_000,
            'debug/deps/libserde_derive-cccc3333dddd4444.so': 1_792_317_335_000_000_000,
            'debug/libserde_derive-cccc3333dddd4444.so': 1_792_317_335_000_000_000,
            'debug/deps/libserde-5555666677778888.d': 1_792_317_341_637_385_329,
            rlib_rel: 1_792_317_341_719_393_475,
            bin_rels[0]: 1_792_317_341_771_393_467,
            bin_rels[1]: 1_792_317_341_771_393_467,
            unit + 'invoked.timestamp': 1_792_317_341_790_000_000,
            unit + 'output': 1_792_317_341_790_000_000,
            unit + 'stderr': 1_792_317_341_800_000_000,
            unit + 'root-output': 1_792_317_341_800_000_000,
        }
        for rel in workspace_rels:
            supplied[rel] = 1_792_317_392_403_385_903
        self.assertEqual(set(supplied), set(all_outputs_map))
        for rel, t in supplied.items():
            p = q_target / rel
            # Read-only fixture: the owner sets explicit times without any write bit; no chmod is applied.
            self.assertEqual(stat.S_IMODE(p.lstat().st_mode), 0o400)
            os.utime(p, ns=(t, t), follow_symlinks=False)
            self.assertEqual(stat.S_IMODE(p.lstat().st_mode), 0o400)
        self.assertGreater(self._legacy_two_tier_mtime_ns(rlib_rel), self._legacy_two_tier_mtime_ns(bin_rels[0]))
        saved_non_debug = copy.deepcopy(ccb.NON_DEBUG_CACHE_CANDIDATES)
        ccb.NON_DEBUG_CACHE_CANDIDATES = {
            '.rustc_info.json': all_outputs_map['.rustc_info.json'],
            'generated/bincode/ZkStdLibArchV1_Decode.rs': all_outputs_map['generated/bincode/ZkStdLibArchV1_Decode.rs'],
        }
        try:
            snapshot = ccb.verify_and_seed_cached_target(
                c,
                guard,
                quarantine_root=q_root,
                copy_map_file=copy_map_path,
                disposition_file=disp_path,
                expected_counts=expected_counts,
            )
            dst_target = Path(c['paths']['target'])
            self.assertEqual(set(snapshot['seeded_files']), set(supplied) - set(workspace_rels))
            for rel, meta in snapshot['seeded_files'].items():
                self.assertEqual(meta['mtime_ns'], supplied[rel], rel)
                self.assertEqual((dst_target / rel).lstat().st_mtime_ns, supplied[rel], rel)
                self.assertEqual((q_target / rel).lstat().st_mtime_ns, supplied[rel], rel)
                self.assertEqual(stat.S_IMODE((q_target / rel).lstat().st_mode), 0o400, rel)
            seeded = {rel: (dst_target / rel).lstat().st_mtime_ns for rel in snapshot['seeded_files']}
            for b in bin_rels:
                self.assertLess(seeded[rlib_rel], seeded[b])
                self.assertLess(seeded[b], seeded[unit + 'invoked.timestamp'])
            self.assertEqual(seeded[unit + 'output'], seeded[unit + 'invoked.timestamp'])
            self.assertGreaterEqual(min(seeded[unit + 'stderr'], seeded[unit + 'root-output']), seeded[unit + 'invoked.timestamp'])
            run = snapshot['run_unit_snapshots']['serde-1111222233334444']
            self.assertEqual(run['invoked.timestamp']['mtime_ns'], supplied[unit + 'invoked.timestamp'])

            self._write(dst_target / 'debug/deps/mori_backend-9999888877776666', b'new-backend-elf', mode=0o755)
            post = ccb.verify_post_compile_no_script_reexec(c, guard, snapshot)
            self.assertEqual(post['host_outputs_verified_unchanged'], 4)
            self.assertEqual(post['build_script_run_units_verified_not_reexecuted'], 1)
            # A byte-identical script rewrite with a newer mtime (P8 class) is still refused by the unchanged postcheck.
            t_new = supplied[bin_rels[1]] + 1
            os.utime(dst_target / bin_rels[1], ns=(t_new, t_new), follow_symlinks=False)
            with self.assertRaises(guard.Refusal):
                ccb.verify_post_compile_no_script_reexec(c, guard, snapshot)
        finally:
            ccb.NON_DEBUG_CACHE_CANDIDATES = saved_non_debug

    def test_preserved_seed_mtime_bounds_and_actual_r3_build_dependency_edges(self):
        lo, hi = ccb.R3_ARCHIVE_MTIME_RANGE_NS
        self.assertEqual((lo, hi), (1_792_317_334_779_394_537, 1_792_317_392_403_385_903))
        probe = self._write(self.root / 'mtime-probe' / 'f', b'x', mode=0o400)

        def seed_mtime(t, mtime_range):
            os.utime(probe, ns=(t, t), follow_symlinks=False)
            self.assertEqual(stat.S_IMODE(probe.lstat().st_mode), 0o400)
            return ccb._preserved_seed_mtime_ns('probe', probe.lstat(), guard, mtime_range)

        for t in (lo, hi):
            self.assertEqual(seed_mtime(t, (lo, hi)), t)
        for t in (lo - 1, hi + 1):
            with self.assertRaises(guard.Refusal):
                seed_mtime(t, (lo, hi))
            self.assertEqual(seed_mtime(t, None), t)
        for t in (ccb.SOURCE_EPOCH_NS - 1, ccb.SOURCE_EPOCH_NS):
            with self.assertRaises(guard.Refusal):
                seed_mtime(t, None)
        self.assertEqual(seed_mtime(ccb.SOURCE_EPOCH_NS + 1, None), ccb.SOURCE_EPOCH_NS + 1)

        # Actual r3 archive mtimes from the all-3498 stat join: build-dependency rlib -> build-script binary.
        aws = ('debug/build/aws-lc-sys-5469586605af0f08/build_script_main-5469586605af0f08', 1_792_317_342_061_812_584)
        edges = (
            (('debug/deps/libversion_check-371b5f2dedfc9d12.rlib', 1_792_317_335_107_394_486),
             ('debug/build/generic-array-8fd0bbe89c84eca4/build_script_build-8fd0bbe89c84eca4', 1_792_317_335_371_394_444)),
            (('debug/deps/libautocfg-856175fbd5b6f428.rlib', 1_792_317_341_719_393_475),
             ('debug/build/num-traits-9996def55c5dad1c/build_script_build-9996def55c5dad1c', 1_792_317_341_771_393_467)),
            (('debug/deps/libcc-d11016940553e9ef.rlib', 1_792_317_337_383_394_126), aws),
            (('debug/deps/libcmake-02fd6461bc7a0673.rlib', 1_792_317_340_933_203_205), aws),
            (('debug/deps/libdunce-9d2697c384b5541b.rlib', 1_792_317_341_311_513_634), aws),
            (('debug/deps/libfs_extra-7470e21bc61d34fa.rlib', 1_792_317_341_427_393_514), aws),
        )
        for (dep_rel, dep_t), (bin_rel, bin_t) in edges:
            self.assertEqual(seed_mtime(dep_t, (lo, hi)), dep_t)
            self.assertEqual(seed_mtime(bin_t, (lo, hi)), bin_t)
            self.assertLess(dep_t, bin_t, dep_rel)
            self.assertGreater(self._legacy_two_tier_mtime_ns(dep_rel), self._legacy_two_tier_mtime_ns(bin_rel), dep_rel)
        # aws-lc-sys-382f6f2cbe30927c/invoked.timestamp follows its compiled script binary.
        self.assertLess(aws[1], 1_792_317_342_067_393_428)

    def test_seed_refuses_preserved_mtime_not_newer_than_source_epoch(self):
        c, q_root, copy_map_path, disp_path, expected_counts, all_outputs_map = self._make_synthetic_quarantine()
        rlib_rel = 'debug/deps/libserde-5555666677778888.rlib'
        q_rlib = q_root / 'target' / rlib_rel
        # Read-only fixture: explicit owner utime only; mode stays 0o400.
        os.utime(q_rlib, ns=(ccb.SOURCE_EPOCH_NS, ccb.SOURCE_EPOCH_NS), follow_symlinks=False)
        saved_non_debug = copy.deepcopy(ccb.NON_DEBUG_CACHE_CANDIDATES)
        ccb.NON_DEBUG_CACHE_CANDIDATES = {
            '.rustc_info.json': all_outputs_map['.rustc_info.json'],
            'generated/bincode/ZkStdLibArchV1_Decode.rs': all_outputs_map['generated/bincode/ZkStdLibArchV1_Decode.rs'],
        }
        try:
            with self.assertRaises(guard.Refusal) as cm:
                ccb.verify_and_seed_cached_target(
                    c,
                    guard,
                    quarantine_root=q_root,
                    copy_map_file=copy_map_path,
                    disposition_file=disp_path,
                    expected_counts=expected_counts,
                )
            self.assertIn('reconstituted source epoch', str(cm.exception))
            self.assertIn(rlib_rel, str(cm.exception))
            self.assertEqual(q_rlib.lstat().st_mtime_ns, ccb.SOURCE_EPOCH_NS)
            self.assertEqual(stat.S_IMODE(q_rlib.lstat().st_mode), 0o400)
            self.assertFalse((Path(c['paths']['packet']) / 'CACHED_TARGET_SEED.json').exists())
        finally:
            ccb.NON_DEBUG_CACHE_CANDIDATES = saved_non_debug

    def test_workspace_member_alias_partition_and_closed_counts(self):
        expected_members = (
            'mori',
            'mori-ast',
            'mori-backend',
            'mori-checker',
            'mori-core',
            'mori-diagnostics',
            'mori-fmt',
            'mori-ledger',
            'mori-lexer',
            'mori-parser',
            'mori-span',
        )
        self.assertEqual(ccb.WORKSPACE_MEMBER_PACKAGES, expected_members)
        for pkg in expected_members:
            u_pkg = pkg.replace('-', '_')
            self.assertEqual(
                ccb.classify_preserved_target_rel(
                    f'debug/.fingerprint/{pkg}-0123456789abcdef/lib-{u_pkg}',
                    guard,
                ),
                'owner_literal_workspace_fingerprint',
            )
            self.assertEqual(
                ccb.classify_preserved_target_rel(
                    f'debug/deps/{u_pkg}-0123456789abcdef.d',
                    guard,
                ),
                'owner_literal_workspace_deps',
            )
            self.assertEqual(
                ccb.classify_preserved_target_rel(
                    f'debug/deps/lib{u_pkg}-0123456789abcdef.rlib',
                    guard,
                ),
                'owner_literal_workspace_deps',
            )
            self.assertEqual(
                ccb.classify_preserved_target_rel(
                    f'debug/deps/lib{u_pkg}-0123456789abcdef.rmeta',
                    guard,
                ),
                'owner_literal_workspace_deps',
            )
            self.assertEqual(
                ccb.classify_preserved_target_rel(
                    f'debug/incremental/{u_pkg}-1f3x0ulm6pbn9/s-hhh2k2b19x-0812702-working/dep-graph.bin',
                    guard,
                ),
                'owner_literal_workspace_incremental',
            )
            self.assertEqual(
                ccb.classify_preserved_target_rel(f'debug/{u_pkg}', guard),
                'owner_literal_workspace_deps',
            )
            self.assertEqual(
                ccb.classify_preserved_target_rel(f'debug/lib{u_pkg}.rlib', guard),
                'owner_literal_workspace_deps',
            )

        r3_preserved_workspace_examples = (
            ('debug/.fingerprint/mori-diagnostics-3fff682cc749be12/lib-mori_diagnostics', 'owner_literal_workspace_fingerprint'),
            ('debug/.fingerprint/mori-lexer-e85e1a0131ecab3f/lib-mori_lexer', 'owner_literal_workspace_fingerprint'),
            ('debug/.fingerprint/mori-span-0998da0816f27b2c/lib-mori_span', 'owner_literal_workspace_fingerprint'),
            ('debug/deps/mori_diagnostics-3fff682cc749be12.d', 'owner_literal_workspace_deps'),
            ('debug/deps/libmori_diagnostics-3fff682cc749be12.rlib', 'owner_literal_workspace_deps'),
            ('debug/deps/libmori_diagnostics-3fff682cc749be12.rmeta', 'owner_literal_workspace_deps'),
            ('debug/deps/mori_lexer-e85e1a0131ecab3f.d', 'owner_literal_workspace_deps'),
            ('debug/deps/libmori_lexer-e85e1a0131ecab3f.rlib', 'owner_literal_workspace_deps'),
            ('debug/deps/libmori_lexer-e85e1a0131ecab3f.rmeta', 'owner_literal_workspace_deps'),
            ('debug/deps/mori_span-0998da0816f27b2c.d', 'owner_literal_workspace_deps'),
            ('debug/deps/libmori_span-0998da0816f27b2c.rlib', 'owner_literal_workspace_deps'),
            ('debug/deps/libmori_span-0998da0816f27b2c.rmeta', 'owner_literal_workspace_deps'),
            ('debug/incremental/mori_diagnostics-1f3x0ulm6pbn9/s-hhh2k2b19x-0812702-36dt7d79z8xh2/dep-graph.bin', 'owner_literal_workspace_incremental'),
            ('debug/incremental/mori_lexer-0f2l2fry7tng3/s-hhh2k2bc23-1d82vpx-1ycbcl0k9a0c8/dep-graph.bin', 'owner_literal_workspace_incremental'),
            ('debug/incremental/mori_span-19ygjfn93jgkc/s-hhh2k2b19v-035h7n9-33z95ebmsf857/dep-graph.bin', 'owner_literal_workspace_incremental'),
            ('debug/deps/mori_backend-3a2f695515330e5a', 'owner_literal_workspace_deps'),
        )
        for rel, expected_cls in r3_preserved_workspace_examples:
            self.assertEqual(ccb.classify_preserved_target_rel(rel, guard), expected_cls, rel)

        foreign_cache_candidates = (
            'debug/.fingerprint/mori-foreign-0123456789abcdef/lib-mori_foreign',
            'debug/deps/mori_foreign-0123456789abcdef.d',
            'debug/deps/libmori_foreign-0123456789abcdef.rlib',
            'debug/incremental/mori_foreign-1f3x0ulm6pbn9/s-work/dep-graph.bin',
            'debug/.fingerprint/serde-ca97fc5f093a8961/lib-serde',
            'debug/deps/libark_ff-25dd8ff3fd20b4c2.rlib',
            'debug/deps/libark_ff_macros-5382e467c4ba2fd5.so',
            'debug/build/poseidon-fe392bd1fb3165c7/build-script-build',
            '.rustc_info.json',
            'generated/bincode/ZkStdLibArchV1_Decode.rs',
            'generated/bincode/ZkStdLibArch_Decode.rs',
            'generated/bincode/ZkStdLibArch_Encode.rs',
        )
        for rel in foreign_cache_candidates:
            self.assertEqual(
                ccb.classify_preserved_target_rel(rel, guard),
                'unadopted_other_cache_candidate',
                rel,
            )

        refused_paths = (
            'generated/bincode/Midnightledger_DefaultDB_bincode.rs',
            'debug/build/mori-backend-3a2f695515330e5a/build-script-build',
            'debug/unknown/mori_core-8ebe0c13555e7bb4',
        )
        for bad_rel in refused_paths:
            with self.assertRaises(guard.Refusal):
                ccb.classify_preserved_target_rel(bad_rel, guard)

        counts = ccb.PRODUCTION_PARTITION_COUNTS
        self.assertEqual(counts['owner_literal_workspace_fingerprint'], 20)
        self.assertEqual(counts['owner_literal_workspace_deps'], 14)
        self.assertEqual(counts['owner_literal_workspace_incremental'], 621)
        self.assertEqual(counts['unadopted_other_cache_candidate'], 2843)
        self.assertEqual(
            counts['owner_literal_workspace_fingerprint']
            + counts['owner_literal_workspace_deps']
            + counts['owner_literal_workspace_incremental']
            + counts['unadopted_other_cache_candidate'],
            counts['target_total'],
        )
        self.assertEqual(counts['target_total'], 3498)
        self.assertEqual(
            counts['target_total'] + counts['packet_total'] + counts['one_shot_total'],
            counts['total_preserved_files'],
        )
        self.assertEqual(
            counts['host_build_script_binaries'] + counts['host_proc_macro_dylibs'],
            counts['host_outputs_total'],
        )

    def test_p9_receipts_once_only_pinned_and_tamper_refused(self):
        saved = {}
        for name in (
            'P9_PERSISTENT_QUARANTINE_ROOT', 'P9_COPY_MAP_PATH', 'P9_COPY_MAP_SHA256', 'P9_COPY_MAP_BYTES',
            'P9_PRESERVE_CLEANUP_RESULT_PATH', 'P9_PRESERVE_CLEANUP_RESULT_SHA256', 'P9_PRESERVE_CLEANUP_RESULT_BYTES',
            'P9_QUARANTINE_VERIFIED_PATH', 'P9_QUARANTINE_VERIFIED_SHA256', 'P9_QUARANTINE_VERIFIED_BYTES',
            'P9_RAW_COUNTERS_PATH', 'P9_RAW_COUNTERS_SHA256', 'P9_RAW_COUNTERS_BYTES',
            'P9_PACKET_PINS', 'P9_F_RESULT_BYTES',
        ):
            saved[name] = copy.deepcopy(getattr(ccb, name))
        q_root = self.root / 'p9-quarantine'
        pkt = q_root / 'packet'
        pkt.mkdir(parents=True, exist_ok=True)

        def set_receipt(prefix, obj, filename):
            p = self._write(self.root / 'p9' / filename, guard.json_bytes(obj), 0o644)
            setattr(ccb, prefix + '_PATH', str(p))
            setattr(ccb, prefix + '_SHA256', guard.digest(p))
            setattr(ccb, prefix + '_BYTES', p.stat().st_size)
            return p

        def set_packet(name, obj):
            if (pkt / name).exists():
                os.chmod(pkt / name, 0o644)
            p = self._write(pkt / name, guard.json_bytes(obj), 0o444)
            pins = dict(ccb.P9_PACKET_PINS)
            pins[name] = guard.digest(p)
            ccb.P9_PACKET_PINS = pins
            if name == 'F_CACHED_CURRENT_RESULT.json':
                ccb.P9_F_RESULT_BYTES = p.stat().st_size
            return p

        seeded_files = {}
        for i in range(2843):
            rel = 'debug/deps/libcrate%04d.rlib' % i
            seeded_files[rel] = {
                'sha256': _sha256_bytes(('candidate-%d' % i).encode('utf-8')),
                'bytes': 11,
                'mode': '0o644',
                'mtime_ns': 1792317392403385903,
                'ino': i + 1,
            }
        all_outputs = {rel: meta['sha256'] for rel, meta in seeded_files.items()}
        for i in range(704):
            all_outputs['debug/deps/libextra%04d.rlib' % i] = _sha256_bytes(('extra-%d' % i).encode('utf-8'))
        all_outputs[ccb.CURRENT_P9_BACKEND_ELF_REL] = ccb.CURRENT_P9_BACKEND_ELF_SHA256
        self.assertEqual(len(all_outputs), 3548)
        seed_obj = {
            'quarantine_root': ccb.PRESERVED_QUARANTINE_ROOT,
            'preserved_copy_map_sha256': ccb.P9_SEED_PACKET_PRESERVED_COPY_MAP_SHA256,
            'r3_quarantined_outputs_sha256': ccb.PRESERVED_PACKET_PINS['F_RETAINED_QUARANTINED_OUTPUTS_R3.json'],
            'partition_counts': dict(ccb.P9_HISTORICAL_SEED_PARTITION_COUNTS),
            'excluded_workspace_count': 655,
            'seeded_count': 2843,
            'host_outputs_count': 90,
            'build_script_run_units_count': 34,
            'host_outputs_seeded': {'debug/build/unit%02d/build-script-build' % i: 'ab' * 32 for i in range(90)},
            'run_unit_snapshots': {'unit%02d' % i: {} for i in range(34)},
            'seeded_files': seeded_files,
        }
        quarantined_obj = {
            'acceptance': 'quarantined',
            'all_outputs': all_outputs,
            'build_script_outputs': [],
            'host_fingerprints': [],
            'host_outputs': [{'path': 'debug/build/unit%02d/build-script-build' % i, 'sha256': 'ab' * 32} for i in range(90)],
        }
        f_obj = {
            'base_head': '539ab417d92cf7f4dfe8655fee810ea5d2a41c89',
            'route': 'cached-current-f',
            'cargo_lock_sha256': guard.LOCK,
            'f_result': {'log_bytes': {'stdout': 0, 'stderr': 8880}},
            'cached_target_seed_counts': dict(ccb.P9_HISTORICAL_SEED_PARTITION_COUNTS),
            'post_compile_check': {
                'host_outputs_verified_unchanged': 90,
                'build_script_run_units_verified_not_reexecuted': 34,
                'fresh_mori_backend_executables': [{
                    'relative': ccb.CURRENT_P9_BACKEND_ELF_REL,
                    'sha256': ccb.CURRENT_P9_BACKEND_ELF_SHA256,
                    'bytes': ccb.CURRENT_P9_BACKEND_ELF_BYTES,
                }],
            },
        }
        marker = {'path': ccb.P7_MARKER_PATH, 'bytes': ccb.P7_MARKER_BYTES, 'sha256': ccb.P7_MARKER_SHA256, 'content': 'no retry\n'}
        events = 'low 0\nhigh 0\nmax 0\noom 0\noom_kill 0\noom_group_kill 0\n'
        raw_obj = {
            'guardTerminal': True,
            'guardExit': 0,
            'noELFExecuted': True,
            'outputsAccepted': False,
            'raw': {
                'cpu.stat': 'usage_usec 127323100\nuser_usec 1\n',
                'supervisor/cpu.stat': 'usage_usec 15815495\nuser_usec 1\n',
                'phases/cpu.stat': 'usage_usec 111507604\nuser_usec 1\n',
                'cgroup.events': 'populated 0\nfrozen 0\n',
                'cgroup.procs': '',
                'memory.events': events,
                'marker': marker,
            },
        }
        steps = [{'step': st, 'actualExit': 0} for st in ccb.P7_CLEANUP_STEPS]
        steps[4] = {'step': ccb.P7_CLEANUP_STEPS[4], 'exactMembershipHashesSizesAndNonexecModes': True, 'files': 3566}
        steps[5]['streams'] = [{'bytes': 165, 'sha256': '326cc1061f54410d646200b669de60d789da9233896633afda799170704b63ba'}]
        steps[6]['streams'] = [{'bytes': 128, 'sha256': 'eb37eddb1cd7cd595e6797be89516814f7383f1bb1a8f7cb55689079af95e88a'}]
        cleanup_obj = {
            'status': 'P9_PRESERVED_AND_CLEANED',
            'steps': steps,
            'cleanupOwed': False,
            'nativeOutputsAccepted': False,
            'ELFExecuted': False,
            'CargoDidStart': True,
            'CargoCompileReportedSuccess': True,
            'CargoBuildSecondsReported': 25.69,
            'guardPostAudit': 'PASS 34buildscriptunitsnotreexecuted90hostoutputsunchanged',
            'targetFilesPreserved': 3548,
            'allFilesPreserved': 3566,
            'totalBytesPreserved': 2847459477,
            'quarantine': str(q_root),
            'cgroupAbsent': True,
            'executionRootEmpty': True,
        }
        fixtures = {
            'CACHED_TARGET_SEED.json': seed_obj,
            'F_QUARANTINED_OUTPUTS.json': quarantined_obj,
            'F_CACHED_CURRENT_RESULT.json': f_obj,
        }
        try:
            ccb.P9_PERSISTENT_QUARANTINE_ROOT = str(q_root)
            ccb.P9_PACKET_PINS = {name: '00' * 32 for name in saved['P9_PACKET_PINS']}
            for name in saved['P9_PACKET_PINS']:
                set_packet(name, fixtures.get(name, {'fixture': name}))
            set_receipt('P9_RAW_COUNTERS', raw_obj, 'RAW-CGROUP-AND-MARKER.json')
            set_receipt('P9_PRESERVE_CLEANUP_RESULT', cleanup_obj, 'RESULT.json')
            set_receipt('P9_QUARANTINE_VERIFIED', {'verified': True}, 'PERSISTENT-QUARANTINE-VERIFIED.json')
            set_receipt('P9_COPY_MAP', {'files': []}, 'PRESERVATION-EXACT-COPY-MAP.proposed.json')
            before = copy.deepcopy(ccb.HISTORICAL_POST_R3_DEBITS)
            ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            self.assertEqual(ccb.HISTORICAL_POST_R3_DEBITS, before)
            for kind, name, keys, value in (
                ('packet', 'F_CACHED_CURRENT_RESULT.json', ('base_head',), '229770592adbf3fffdf5fe77f8fa82c8c3aee194'),
                ('packet', 'F_CACHED_CURRENT_RESULT.json', ('post_compile_check', 'fresh_mori_backend_executables', 0, 'sha256'), '00' * 32),
                ('packet', 'F_CACHED_CURRENT_RESULT.json', ('cached_target_seed_counts', 'owner_literal_workspace_fingerprint'), 22),
                ('packet', 'CACHED_TARGET_SEED.json', ('partition_counts', 'owner_literal_workspace_fingerprint'), 22),
                ('packet', 'CACHED_TARGET_SEED.json', ('seeded_count',), 2842),
                ('packet', 'F_QUARANTINED_OUTPUTS.json', ('all_outputs', ccb.CURRENT_P9_BACKEND_ELF_REL), '00' * 32),
                ('receipt', 'P9_RAW_COUNTERS', ('raw', 'supervisor/cpu.stat'), 'usage_usec 0\n'),
                ('receipt', 'P9_PRESERVE_CLEANUP_RESULT', ('CargoBuildSecondsReported',), 25.70),
                ('receipt', 'P9_PRESERVE_CLEANUP_RESULT', ('allFilesPreserved',), 3565),
                ('receipt', 'P9_PRESERVE_CLEANUP_RESULT', ('steps', 4, 'files'), 3565),
            ):
                with self.subTest(kind=kind, name=name, keys=keys):
                    if kind == 'packet':
                        bad = copy.deepcopy(fixtures[name])
                        target = bad
                        for key in keys[:-1]:
                            target = target[key]
                        target[keys[-1]] = value
                        set_packet(name, bad)
                        with self.assertRaises(guard.Refusal):
                            ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
                        set_packet(name, fixtures[name])
                    else:
                        original = raw_obj if name == 'P9_RAW_COUNTERS' else cleanup_obj
                        filename = 'RAW-CGROUP-AND-MARKER.json' if name == 'P9_RAW_COUNTERS' else 'RESULT.json'
                        bad = copy.deepcopy(original)
                        target = bad
                        for key in keys[:-1]:
                            target = target[key]
                        target[keys[-1]] = value
                        set_receipt(name, bad, filename)
                        with self.assertRaises(guard.Refusal):
                            ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
                        set_receipt(name, original, filename)
            ccb.P9_COPY_MAP_SHA256 = '00' * 32
            with self.assertRaises(guard.Refusal):
                ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            set_receipt('P9_COPY_MAP', {'files': []}, 'PRESERVATION-EXACT-COPY-MAP.proposed.json')
            pins = dict(ccb.P9_PACKET_PINS)
            pins['A0_FREEZE.json'] = '00' * 32
            ccb.P9_PACKET_PINS = pins
            with self.assertRaises(guard.Refusal):
                ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            set_packet('A0_FREEZE.json', {'fixture': 'A0_FREEZE.json'})
            with self.assertRaises(guard.Refusal):
                ccb._verify_p9_receipts(dict(before, retired_p9_envelope_usec=127323099), guard)
            with self.assertRaises(guard.Refusal):
                ccb._verify_p9_receipts(dict(before, f_stderr_bytes=34201), guard)
            ccb._verify_p9_receipts(ccb.HISTORICAL_POST_R3_DEBITS, guard)
            self.assertEqual(ccb.HISTORICAL_POST_R3_DEBITS, before)
        finally:
            for name, value in saved.items():
                setattr(ccb, name, value)

    def test_p9_seed_branch_uses_p9_counts_and_excludes_current_elf(self):
        self.assertEqual(ccb.P9_PARTITION_COUNTS['target_total'], 3548)
        self.assertEqual(ccb.P9_PARTITION_COUNTS['packet_total'], 17)
        self.assertEqual(ccb.P9_PARTITION_COUNTS['total_preserved_files'], 3566)
        self.assertEqual(ccb.P9_HISTORICAL_SEED_PARTITION_COUNTS['owner_literal_workspace_fingerprint'], 20)
        self.assertEqual(ccb.P9_HISTORICAL_SEED_PARTITION_COUNTS['owner_literal_workspace_incremental'], 621)
        workspace = sum(ccb.P9_PARTITION_COUNTS[k] for k in (
            'owner_literal_workspace_fingerprint', 'owner_literal_workspace_deps',
            'owner_literal_workspace_incremental'))
        self.assertEqual(workspace, 705)
        self.assertEqual(3548 - workspace, ccb.P9_PARTITION_COUNTS['unadopted_other_cache_candidate'])
        self.assertEqual(ccb.classify_preserved_target_rel(ccb.CURRENT_P9_BACKEND_ELF_REL, guard),
                         'owner_literal_workspace_deps')
        self.assertNotEqual(ccb.CURRENT_P9_BACKEND_ELF_SHA256, ccb.OLD_BACKEND_ELF_SHA256)
        c, q_root, copy_map_path, disp_path, expected_counts, all_outputs_map = self._make_synthetic_quarantine()
        saved_non_debug = copy.deepcopy(ccb.NON_DEBUG_CACHE_CANDIDATES)
        saved_p9 = {}
        for name in (
            'P9_PERSISTENT_QUARANTINE_ROOT', 'P9_COPY_MAP_PATH', 'P9_COPY_MAP_SHA256',
            'P9_PRESERVE_CLEANUP_RESULT_PATH', 'P9_PRESERVE_CLEANUP_RESULT_SHA256',
            'P9_PACKET_PINS', 'P9_PARTITION_COUNTS',
        ):
            saved_p9[name] = copy.deepcopy(getattr(ccb, name))
        ccb.NON_DEBUG_CACHE_CANDIDATES = {
            '.rustc_info.json': all_outputs_map['.rustc_info.json'],
            'generated/bincode/ZkStdLibArchV1_Decode.rs': all_outputs_map['generated/bincode/ZkStdLibArchV1_Decode.rs'],
        }
        try:
            q_packet = q_root / 'packet'
            host_rels = sorted(
                rel for rel in all_outputs_map
                if rel.endswith('.so') or Path(rel).name.startswith('build-script') or Path(rel).name.startswith('build_script')
            )
            self.assertEqual(len(host_rels), 4)
            host_outputs = [{'path': rel, 'sha256': all_outputs_map[rel]} for rel in host_rels]
            outputs_path = self._write(
                q_packet / 'F_QUARANTINED_OUTPUTS.json',
                json.dumps({
                    'acceptance': 'quarantined',
                    'all_outputs': all_outputs_map,
                    'build_script_outputs': [],
                    'host_fingerprints': [],
                    'host_outputs': host_outputs,
                }),
                0o444,
            )
            result_path = self._write(
                q_packet / 'F_CACHED_CURRENT_RESULT.json',
                json.dumps({
                    'base_head': '539ab417d92cf7f4dfe8655fee810ea5d2a41c89',
                    'route': 'cached-current-f',
                    'cached_target_seed_counts': dict(ccb.P9_HISTORICAL_SEED_PARTITION_COUNTS),
                }),
                0o444,
            )
            ccb.P9_PERSISTENT_QUARANTINE_ROOT = str(q_root)
            ccb.P9_COPY_MAP_PATH = str(copy_map_path)
            ccb.P9_COPY_MAP_SHA256 = guard.digest(copy_map_path)
            ccb.P9_PRESERVE_CLEANUP_RESULT_PATH = str(disp_path)
            ccb.P9_PRESERVE_CLEANUP_RESULT_SHA256 = guard.digest(disp_path)
            ccb.P9_PACKET_PINS = {
                'F_QUARANTINED_OUTPUTS.json': guard.digest(outputs_path),
                'F_CACHED_CURRENT_RESULT.json': guard.digest(result_path),
            }
            ccb.P9_PARTITION_COUNTS = expected_counts
            shutil.rmtree(c['paths']['target'])
            Path(c['paths']['target']).mkdir(parents=True, exist_ok=True)
            snapshot = ccb.verify_and_seed_cached_target_p9(c, guard)
            self.assertEqual(snapshot['partition_counts']['owner_literal_workspace_fingerprint'], 1)
            self.assertEqual(snapshot['partition_counts']['owner_literal_workspace_deps'], 1)
            self.assertEqual(snapshot['partition_counts']['owner_literal_workspace_incremental'], 1)
            self.assertEqual(snapshot['partition_counts']['unadopted_other_cache_candidate'], 12)
            self.assertEqual(len(snapshot['seeded_files']), 12)
            dst_target = Path(c['paths']['target'])
            self.assertFalse((dst_target / 'debug/deps/libmori_backend-3a2f695515330e5a.rlib').exists())
            self.assertTrue((dst_target / '.rustc_info.json').is_file())
            self.assertTrue((dst_target / 'generated/bincode/ZkStdLibArchV1_Decode.rs').is_file())
            self.assertTrue((Path(c['paths']['packet']) / 'CACHED_TARGET_SEED.json').is_file())
        finally:
            ccb.NON_DEBUG_CACHE_CANDIDATES = saved_non_debug
            for name, value in saved_p9.items():
                setattr(ccb, name, value)

    def test_cached_current_f_phase_leaf_placement_and_lifecycle(self):
        cg_base = self.root / 'cgroup_fs'
        env_cg = cg_base / 'moriarty-sherlock-ledger9-20261018-06'
        sup_cg = env_cg / 'supervisor'
        ph_cg = env_cg / 'phases'
        sup_cg.mkdir(parents=True, exist_ok=True)
        ph_cg.mkdir(parents=True, exist_ok=True)
        self._write(env_cg / 'cpu.stat', 'usage_usec 0\n', 0o644)
        self._write(sup_cg / 'cpu.stat', 'usage_usec 0\n', 0o644)
        self._write(ph_cg / 'cpu.stat', 'usage_usec 0\n', 0o644)

        exec_root = self.root / 'exec'
        paths = {
            'execution_root': str(exec_root),
            'source_home': str(exec_root / 'source-home'),
            'target': str(exec_root / 'target'),
            'tmp': str(exec_root / 'tmp'),
            'packet': str(exec_root / 'packet'),
            'empty_home': str(exec_root / 'empty-home'),
            'mask_empty': str(exec_root / 'mask-empty'),
            'worktree': str(self.root / 'worktree'),
            'config': str(self.root / 'config.final.json'),
        }
        for k, v in paths.items():
            if k != 'config':
                Path(v).mkdir(parents=True, exist_ok=True)

        c = {
            'mode': 'tier1-and-compile',
            'total_wall_seconds': 6038,
            'supervisor_cpu_seconds': 10576,
            'paths': paths,
            'readonly_tools': ['/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu'],
            'source_copy_mounts': ['/home/charl/.cargo'],
            'masks': ['/home/charl', '/home/charl/.cargo'],
            'limits': {
                'D': {'packet_bytes': 67108864},
                'F': {
                    'wall_seconds': 2133,
                    'cpu_time_us': 10800000000,
                    'cpu_quota_us': 400000,
                    'cpu_period_us': 100000,
                    'memory_bytes': 8589934592,
                    'file_bytes': 4294967296,
                    'pids': 1024,
                    'packet_bytes': 67108864,
                    'stdout_bytes': 16777216,
                    'stderr_bytes': 16777216,
                    'namespace_tmpfs_bytes': 67108864,
                },
            },
            'cgroup': {
                'envelope_path': '/moriarty-sherlock-ledger9-20261018-06',
                'supervisor_leaf_name': 'supervisor',
                'phase_parent_name': 'phases',
            },
        }

        # 1. Before D_FREEZE.json exists, F precheck mount/predecessor contracts allow source_home write in phase F leaf.
        self.assertEqual(guard.child_mounts(c, 'F', False)[paths['source_home']], 'rw')
        self.assertEqual(guard.prior_receipts(c, 'F', True, False), {})

        saved_cg_fs = guard.cg_fs
        saved_cg_text = guard._cg_text
        saved_reconstitute = ccb.reconstitute_source_home
        saved_seed = ccb.verify_and_seed_cached_target_p9
        saved_enforce = ccb.enforce_post_r3_historical_debits
        saved_consent = getattr(guard, 'consent', None)
        saved_sup_env = guard.supervisor_envelope
        saved_bound = getattr(guard, 'bound_identities', None)
        saved_owned = getattr(guard, 'owned_paths', None)
        saved_quotas = getattr(guard, 'quotas', None)
        saved_launch = guard.launch
        import resource
        saved_setrlimit = resource.setrlimit
        rlimit_calls = []
        try:
            guard.cg_fs = lambda ns: cg_base / ns.lstrip('/')

            # 2. prepare_cached_current_f_worker refuses if called in supervisor leaf S before any write.
            guard._cg_text = lambda p: (
                '0::/moriarty-sherlock-ledger9-20261018-06/supervisor\n'
                if str(p) == '/proc/self/cgroup'
                else saved_cg_text(p)
            )
            with self.assertRaises(guard.Refusal):
                ccb.prepare_cached_current_f_worker(c, guard)
            self.assertEqual(list(Path(paths['source_home']).iterdir()), [])
            self.assertEqual(list(Path(paths['target']).iterdir()), [])

            # 3. Inside phase F leaf, prepare_cached_current_f_worker runs before first page-creating write.
            call_order = []

            def fake_reconstitute(cfg, g):
                self.assertEqual(
                    g._cg_text('/proc/self/cgroup').strip(),
                    '0::/moriarty-sherlock-ledger9-20261018-06/phases/sdk-once-F',
                )
                call_order.append('reconstitute')
                self._write(Path(cfg['paths']['source_home']) / 'marker.txt', 'ok\n', 0o444)
                for fn in (
                    'A0_FREEZE.json',
                    'C_RAW_IDENTITIES.json',
                    'D_FREEZE.json',
                    'E_RESOLVED.json',
                    'SOURCE_HOME_RECONSTITUTION.json',
                ):
                    self._write(Path(cfg['paths']['packet']) / fn, '{"ok":true}\n', 0o444)
                return {'status': 'reconstituted'}

            def fake_seed(cfg, g):
                self.assertEqual(
                    g._cg_text('/proc/self/cgroup').strip(),
                    '0::/moriarty-sherlock-ledger9-20261018-06/phases/sdk-once-F',
                )
                call_order.append('seed')
                self._write(Path(cfg['paths']['target']) / 'seeded.txt', 'ok\n', 0o644)
                snap = {
                    'partition_counts': dict(ccb.PRODUCTION_PARTITION_COUNTS),
                    'host_outputs_seeded': {},
                    'run_unit_snapshots': {},
                    'seeded_files': {},
                }
                self._write(Path(cfg['paths']['packet']) / 'CACHED_TARGET_SEED.json', json.dumps(snap) + '\n', 0o444)
                return snap

            ccb.reconstitute_source_home = fake_reconstitute
            ccb.verify_and_seed_cached_target_p9 = fake_seed
            ccb.enforce_post_r3_historical_debits = lambda cfg, g, cleanup_receipt_file=None, at_start=True: saved_enforce(
                cfg, g, cleanup_receipt_file=None, at_start=at_start
            )
            guard.BASE = 'f935febac3f9158c3ce02812ac3b3c4adb986714'
            guard.consent = lambda cfg: None
            guard.supervisor_envelope = lambda cfg: None
            guard.bound_identities = lambda cfg, host_originals=True: None
            guard.owned_paths = lambda cfg: None
            guard.quotas = lambda cfg, ph: None

            def fake_launch(cfg, ph, cfg_sha, retained_audit=False):
                self.assertEqual(ph, 'F')
                # Supervisor S must not have created any files in source_home or target before launch.
                self.assertEqual(list(Path(cfg['paths']['source_home']).iterdir()), [])
                self.assertEqual(list(Path(cfg['paths']['target']).iterdir()), [])
                self.assertTrue((Path(cfg['paths']['execution_root']) / 'ONE_SHOT_STARTED').is_file())
                grp = guard.Group(cfg, 'F')
                self.assertEqual(getattr(grp, 'prior_cpu', None), 739509969)
                guard._cg_text = lambda p: (
                    '0::/moriarty-sherlock-ledger9-20261018-06/phases/sdk-once-F\n'
                    if str(p) == '/proc/self/cgroup'
                    else saved_cg_text(p)
                )
                ccb.prepare_cached_current_f_worker(cfg, guard)
                # Once D_FREEZE and CACHED_TARGET_SEED exist, F mount mode is ro and predecessors include all 8 files.
                self._write(Path(cfg['paths']['packet']) / 'F.stdout', '', 0o444)
                self._write(Path(cfg['paths']['packet']) / 'F.stderr', '', 0o444)
                self.assertEqual(guard.child_mounts(cfg, 'F', False)[cfg['paths']['source_home']], 'ro')
                self.assertEqual(
                    set(guard.prior_receipts(cfg, 'F', False, False).keys()),
                    {
                        'A0_FREEZE.json',
                        'C_RAW_IDENTITIES.json',
                        'D_FREEZE.json',
                        'E_RESOLVED.json',
                        'SOURCE_HOME_RECONSTITUTION.json',
                        'CACHED_TARGET_SEED.json',
                        'F.stdout',
                        'F.stderr',
                    },
                )
                self._write(
                    Path(cfg['paths']['packet']) / 'CACHED_POST_COMPILE_CHECK.json',
                    '{"host_outputs_verified_unchanged":90}\n',
                    0o444,
                )
                self._write(env_cg / 'cpu.stat', 'usage_usec 60000000\n', 0o644)
                self._write(sup_cg / 'cpu.stat', 'usage_usec 10000000\n', 0o644)
                self._write(ph_cg / 'cpu.stat', 'usage_usec 50000000\n', 0o644)
                return {'phase': 'F', 'wall_seconds': 12.5, 'log_bytes': {'stdout': 0, 'stderr': 100}}

            resource.setrlimit = lambda kind, lims: rlimit_calls.append((kind, lims))
            guard.launch = fake_launch
            res = ccb.cached_current_f(c, 'a' * 64, guard)
            self.assertEqual(rlimit_calls, [(resource.RLIMIT_CPU, (10576, 10576))])
            self.assertEqual(call_order, ['reconstitute', 'seed'])
            self.assertEqual(res['historical_debit_state_start']['effective_phases_usec_at_start'], 739509969)
            self.assertEqual(res['historical_debit_state_end']['effective_phases_usec_at_start'], 739509969 + 50000000)
            self.assertEqual(res['historical_debit_state_end']['effective_supervisor_usec_at_start'], 223293846 + 10000000)
            self.assertEqual(res['historical_debit_state_end']['effective_envelope_usec_at_start'], 962803819 + 60000000)
            self.assertTrue((Path(paths['packet']) / 'F_CACHED_CURRENT_RESULT.json').is_file())
        finally:
            guard.cg_fs = saved_cg_fs
            guard._cg_text = saved_cg_text
            ccb.reconstitute_source_home = saved_reconstitute
            ccb.verify_and_seed_cached_target_p9 = saved_seed
            ccb.enforce_post_r3_historical_debits = saved_enforce
            if saved_consent is not None:
                guard.consent = saved_consent
            guard.supervisor_envelope = saved_sup_env
            if saved_bound is not None:
                guard.bound_identities = saved_bound
            if saved_owned is not None:
                guard.owned_paths = saved_owned
            if saved_quotas is not None:
                guard.quotas = saved_quotas
            guard.launch = saved_launch
            resource.setrlimit = saved_setrlimit


if __name__ == '__main__':
    unittest.main()

"""Offline launcher tests. Inert bytes only: no compiler/proofs/wallet/network."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

MODULE = Path(__file__).with_name('run-build-bounded.py')
spec = importlib.util.spec_from_file_location('bounded', MODULE)
bounded = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bounded)


class LauncherTests(unittest.TestCase):
    def fixture(self, root):
        parent = root / 'output-parent'
        parent.mkdir(mode=0o700)
        admission = dict(allowFullBuild=True, case='loan', outputDir=str(parent/'loan-build'),
                         maxAttempts=1, maxCompileMs=300000, deadlineMs=int(time.time()*1000)+200000,
                         candidateHash='a'*64, resourceId='allocation')
        resource = {k: admission[k] for k in ('case', 'outputDir', 'maxAttempts', 'maxCompileMs', 'deadlineMs')}
        resource.update(id='allocation', sourceCandidateSha='a'*64, attemptFile=str(root/'attempt.json'))
        refs = []
        for name, obj in [('resource', resource), ('gpt', dict(sourceCandidateSha='a'*64, verdict='APPROVED')),
                          ('opus', dict(sourceCandidateSha='a'*64, verdict='APPROVED'))]:
            path = root/(name+'.json'); path.write_text(json.dumps(obj))
            refs.append(dict(path=str(path), sha256=bounded.digest(path)))
        admission.update(resourceRecord=refs[0], reviews=refs[1:])
        request = dict(case='loan', outputDir=admission['outputDir'], admission=admission)
        path = root/'request.json'; path.write_text(json.dumps(request))
        return path, root/'result.json', parent, request

    def test_valid_request_and_durable_exclusion(self):
        with tempfile.TemporaryDirectory() as tmp:
            path, result, parent, _ = self.fixture(Path(tmp))
            self.assertEqual(bounded.validate(str(path), bounded.digest(path), str(result))[1], parent)
            bounded.exclusive(result, {'accepted': False})
            with self.assertRaises(FileExistsError): bounded.exclusive(result, {})
            self.assertEqual(json.loads(result.read_text()), {'accepted': False})

    def test_reject_changed_request_unsafe_path_existing_output(self):
        with tempfile.TemporaryDirectory() as tmp:
            path, result, parent, request = self.fixture(Path(tmp))
            with self.assertRaises(ValueError): bounded.validate(str(path), '0'*64, str(result))
            (parent/'loan-build').mkdir()
            with self.assertRaises(ValueError): bounded.validate(str(path), bounded.digest(path), str(result))
            link = Path(tmp)/'link'; link.symlink_to(path)
            with self.assertRaises(ValueError): bounded.safe_path(str(link))

    def test_reject_adapter_and_excess_budget(self):
        for field, value in [('commandAdapter', {}), ('maxCompileMs', 300001), ('deadlineMs', int(time.time()*1000)+400000)]:
            with tempfile.TemporaryDirectory() as tmp:
                path, result, _, request = self.fixture(Path(tmp))
                if field == 'commandAdapter': request[field] = value
                else: request['admission'][field] = value
                path.write_text(json.dumps(request))
                with self.assertRaises(ValueError): bounded.validate(str(path), bounded.digest(path), str(result))

    def test_reject_persistent_evidence_inside_quota(self):
        with tempfile.TemporaryDirectory() as tmp:
            path, _, parent, _ = self.fixture(Path(tmp))
            with self.assertRaises(ValueError): bounded.validate(str(path), bounded.digest(path), str(parent/'result.json'))

    def test_real_namespace_rejects_oversized_sparse_and_hardlinks(self):
        for payload in [
            "with (p/'sparse').open('wb') as f: f.truncate(262144)",
            "(p/'linked').write_bytes(b'x'*16384); os.link(p/'linked',p/'link')",
        ]:
            with tempfile.TemporaryDirectory(prefix='sp05-retain-test-') as tmp:
                root = Path(tmp); parent = root/'parent'; mirror = root/'mirror'
                parent.mkdir(mode=0o700)
                child = "import pathlib,sys,os\np=pathlib.Path(sys.argv[1])\n" + payload
                code = "import importlib.util,sys; from pathlib import Path; s=importlib.util.spec_from_file_location('b',sys.argv[1]); b=importlib.util.module_from_spec(s); s.loader.exec_module(b); sys.exit(b.quota_run(Path(sys.argv[2]),Path(sys.argv[3]),[sys.executable,'-c',sys.argv[4],sys.argv[2]],65536))"
                run = subprocess.run(['unshare', '--user', '--map-root-user', '--mount', '--fork', sys.executable,
                                      '-c', code, str(MODULE), str(parent), str(mirror), child],
                                     capture_output=True, text=True, timeout=10)
                self.assertNotEqual(run.returncode, 0, 'oversized/hardlinked output was accepted')
                self.assertEqual(list(parent.iterdir()), [], 'preflight must precede any host writes')

    def test_real_namespace_accepts_small_sparse_within_logical_limit(self):
        # Acceptance concerns logical bytes only, not host block allocation.
        with tempfile.TemporaryDirectory(prefix='sp05-small-sparse-') as tmp:
            root = Path(tmp); parent = root/'parent'; mirror = root/'mirror'
            parent.mkdir(mode=0o700)
            payload = "import pathlib,sys; p=pathlib.Path(sys.argv[1]); f=(p/'small').open('wb'); f.truncate(32); f.close()"
            code = "import importlib.util,sys; from pathlib import Path; s=importlib.util.spec_from_file_location('b',sys.argv[1]); b=importlib.util.module_from_spec(s); s.loader.exec_module(b); sys.exit(b.quota_run(Path(sys.argv[2]),Path(sys.argv[3]),[sys.executable,'-c',sys.argv[4],sys.argv[2]],65536))"
            run = subprocess.run(['unshare', '--user', '--map-root-user', '--mount', '--fork', sys.executable,
                                  '-c', code, str(MODULE), str(parent), str(mirror), payload],
                                 capture_output=True, text=True, timeout=10)
            self.assertEqual(run.returncode, 0, run.stderr)
            self.assertEqual((parent/'small').read_bytes(), bytes(32))
            self.assertFalse(mirror.exists())

    def test_mirror_appearing_after_overlay_prevents_child(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            parent = Path(tmp)/'parent'; parent.mkdir(); mirror = Path(tmp)/'mirror'
            commands = []
            def run(command, **kwargs):
                commands.append(command)
                if command[:3] == ['mount', '-t', 'tmpfs']: mirror.mkdir()
                return subprocess.CompletedProcess(command, 0)
            with patch.object(bounded.subprocess, 'run', run):
                with self.assertRaisesRegex(ValueError, 'mirror appeared after mount'):
                    bounded.quota_run(parent, mirror, ['inert'], 65536)
            self.assertFalse(any(command[0] == 'unshare' for command in commands))
            self.assertIn(['umount', str(parent)], commands)

    def test_real_namespace_kills_detached_descendants_before_retention(self):
        with tempfile.TemporaryDirectory(prefix='sp05-orphan-test-') as tmp:
            root = Path(tmp); parent = root/'parent'; mirror = root/'mirror'
            parent.mkdir(mode=0o700)
            payload = """import os,pathlib,sys,time
p=pathlib.Path(sys.argv[1]); ready=p.parent/'ready'; escaped=p.parent/'escaped'
if os.fork()==0:
 os.setsid()
 if os.fork()!=0: os._exit(0)
 ready.write_text('ready')
 time.sleep(0.2)
 escaped.write_text('surviving detached descendant')
 os._exit(0)
for _ in range(100):
 if ready.exists(): break
 time.sleep(0.001)
assert ready.exists()
(p/'retained').write_text('finished')
os._exit(0)
"""
            code = "import importlib.util,sys; from pathlib import Path; s=importlib.util.spec_from_file_location('b',sys.argv[1]); b=importlib.util.module_from_spec(s); s.loader.exec_module(b); sys.exit(b.quota_run(Path(sys.argv[2]),Path(sys.argv[3]),[sys.executable,'-c',sys.argv[4],sys.argv[2]],65536))"
            run = subprocess.run(['unshare', '--user', '--map-root-user', '--mount', '--fork', sys.executable,
                                  '-c', code, str(MODULE), str(parent), str(mirror), payload],
                                 capture_output=True, text=True, timeout=10)
            self.assertEqual(run.returncode, 0, run.stderr)
            time.sleep(0.25)
            self.assertTrue((root/'ready').exists())
            self.assertFalse((root/'escaped').exists(), 'detached descendant survived before retention')
            self.assertEqual((parent/'retained').read_text(), 'finished')

    def test_mirror_inside_output_rejected_before_mount(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            parent = Path(tmp)
            with patch.object(bounded.subprocess, 'run') as run:
                with self.assertRaises(ValueError): bounded.quota_run(parent, parent/'mirror', ['inert'], 65536)
                run.assert_not_called()

    def test_retention_and_both_unmount_failures_preserved(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            parent = Path(tmp)/'parent'; parent.mkdir()
            mirror = Path(tmp)/'mirror'
            original_stat = Path.stat
            def mounted_stat(path, *args, **kwargs):
                observed = original_stat(path, *args, **kwargs)
                return original_stat(parent) if path == mirror else observed
            def run(command, **kwargs):
                if command[0] == 'umount':
                    raise subprocess.CalledProcessError(9, command)
                return subprocess.CompletedProcess(command, 0)
            with patch.object(bounded.subprocess, 'run', run), patch.object(Path, 'stat', mounted_stat), patch.object(bounded, 'retain', side_effect=ValueError('primary-retention-failure')):
                with self.assertRaises(ExceptionGroup) as caught:
                    bounded.quota_run(parent, Path(tmp)/'mirror', ['inert'], 65536)
            errors = caught.exception.exceptions
            self.assertEqual(len(errors), 3)
            self.assertIn('primary-retention-failure', str(errors[0]))
            self.assertTrue(all(isinstance(error, subprocess.CalledProcessError) for error in errors[1:]))

    def test_successful_mount_with_wrong_target_rejects_before_retention(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            parent = Path(tmp)/'parent'; parent.mkdir(); mirror = Path(tmp)/'mirror'
            # A successful command status alone does not establish the bind target.
            with patch.object(bounded.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0)), patch.object(bounded, 'retain') as retain:
                with self.assertRaisesRegex(ValueError, 'retention bind target mismatch'):
                    bounded.quota_run(parent, mirror, ['inert'], 65536)
                retain.assert_not_called()
            self.assertFalse(mirror.exists())

    def test_growth_during_copy_cannot_exceed_shared_budget(self):
        from unittest.mock import patch
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); src = root/'src'; dst = root/'dst'
            src.mkdir(); dst.mkdir(); (src/'a').write_bytes(b'a'*32768)
            (src/'nested').mkdir(); growing = src/'nested'/'b'; growing.write_bytes(b'b'*32768)
            original_read = os.read
            changed = False
            def grow_after_read(fd, size):
                nonlocal changed
                chunk = original_read(fd, size)
                if chunk and not changed:
                    changed = True
                    with Path(os.readlink(f'/proc/self/fd/{fd}')).open('ab') as f: f.write(b'x'*65536)
                return chunk
            with patch.object(bounded.os, 'read', grow_after_read):
                with self.assertRaises(ValueError): bounded.retain(src, dst, 65536)
            self.assertTrue(changed)
            self.assertLessEqual(sum(p.stat().st_size for p in dst.rglob('*') if p.is_file()), 65536)

    def test_real_namespace_enospc_and_same_path_retention(self):
        with tempfile.TemporaryDirectory(prefix='sp05-quota-test-') as tmp:
            root = Path(tmp); parent = root/'parent'; mirror = root/'mirror'
            parent.mkdir(mode=0o700)
            payload = """import pathlib,errno,sys
p=pathlib.Path(sys.argv[1]); assert not (p.parent/'mirror').exists(); (p/'retained').write_bytes(b'same-path')
try:
 with (p/'overflow').open('wb') as f:
  for _ in range(32): f.write(b'x'*8192); f.flush()
except OSError as e:
 if e.errno != errno.ENOSPC: raise
 print('ENOSPC', flush=True)
 sys.exit(23)
sys.exit(99)
"""
            code = "import importlib.util,sys; from pathlib import Path; s=importlib.util.spec_from_file_location('b',sys.argv[1]); b=importlib.util.module_from_spec(s); s.loader.exec_module(b); sys.exit(b.quota_run(Path(sys.argv[2]),Path(sys.argv[3]),[sys.executable,'-c',sys.argv[4],sys.argv[2]],65536))"
            run = subprocess.run(['unshare', '--user', '--map-root-user', '--mount', '--fork', sys.executable,
                                  '-c', code, str(MODULE), str(parent), str(mirror), payload],
                                 capture_output=True, text=True, timeout=10)
            self.assertEqual(run.returncode, 23, run.stderr)
            self.assertIn('ENOSPC', run.stdout)
            self.assertEqual((parent/'retained').read_bytes(), b'same-path')
            self.assertLessEqual(sum(p.stat().st_size for p in parent.iterdir()), 65536)
            self.assertFalse(mirror.exists())


if __name__ == '__main__': unittest.main()

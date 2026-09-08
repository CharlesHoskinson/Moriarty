"""Observed Codex wire-format and honest coverage regressions."""
import contextlib
import io
import json
import os
from pathlib import Path
import subprocess
import sqlite3
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'scripts'))
from moriarty_dev import hook,store

class HostAdapter(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.root=Path(self.tmp.name)
        (self.root/'.git').mkdir();(self.root/'.moriarty-dev').mkdir()
        (self.root/'.moriarty-dev/actions.json').write_text('{}')
        self.action={'id':'blocked-action','requirement':'SP01','capability':'cap','kind':'implement','commandRef':'commands.json#driver'}

    def tearDown(self): self.tmp.cleanup()

    def test_snake_case_cmd_denied_and_file_edits_unaffected(self):
        snap={'sameDefectFailures':2};decision={'allow':False,'reasonCode':'REPRODUCE_BEFORE_RETRY','nextActionId':'reproduce'}
        with patch.object(hook,'read_actions',return_value=[self.action]),patch.object(hook,'load_snapshot',return_value=snap),patch.object(hook,'assess',return_value=decision):
            out=hook.handle_event('PreToolUse',{'tool_name':'Bash','tool_input':{'cmd':'python cli.py run --action blocked-action'}},self.root)
            self.assertEqual(out['hookSpecificOutput']['permissionDecision'],'deny')
            out=hook.handle_event('PreToolUse',{'tool_name':'apply_patch','tool_input':{'command':'Edit documentation about blocked-action'}},self.root)
            self.assertNotEqual(out['hookSpecificOutput'].get('permissionDecision'),'deny')

    def test_unknown_history_still_emits_session_context(self):
        with patch.object(hook,'read_actions',return_value=[self.action]),patch.object(hook,'load_snapshot',return_value={'sameDefectFailures':None}),patch.object(hook,'_last_result',return_value='No accepted result'):
            out=hook.handle_session_start(self.root)
            self.assertIn('SP01',out['hookSpecificOutput']['additionalContext'])

    def test_oversized_unicode_output_is_valid_json_and_keeps_denial(self):
        out={'hookSpecificOutput':{'hookEventName':'PreToolUse','permissionDecision':'deny','permissionDecisionReason':'界'*3000}}
        captured=io.StringIO()
        with patch.object(hook,'handle_event',return_value=out),patch.object(sys,'stdin',io.StringIO('{}')),patch.object(sys,'argv',['hook.py','PreToolUse']),contextlib.redirect_stdout(captured): hook.main()
        data=captured.getvalue();self.assertLessEqual(len(data.encode()),2048)
        self.assertEqual(json.loads(data)['hookSpecificOutput']['permissionDecision'],'deny')

    def test_malformed_typed_payload_does_not_crash(self):
        captured=io.StringIO()
        with patch.object(sys,'stdin',io.StringIO('[]')),patch.object(sys,'argv',['hook.py']),contextlib.redirect_stdout(captured): hook.main()
        self.assertIsInstance(json.loads(captured.getvalue()),dict)

    def test_post_tool_reports_pending_without_auto_continuation(self):
        store.enqueue_tx(store.get_db_path(self.root),str(self.root),'tx-hook-test','submitted',{'network':'preview'})
        out=hook.handle_event('PostToolUse',{},self.root)
        self.assertIn('tx-hook-test',out['hookSpecificOutput']['additionalContext'])
        stop=hook.handle_event('Stop',{},self.root)
        self.assertNotIn('decision',stop)
        self.assertNotIn('continue',stop)

    def test_doctor_cannot_infer_interception_from_file_existence(self):
        home=self.root/'codex';(home/'plugins/moriarty-dev').mkdir(parents=True)
        (home/'hooks.json').write_text(json.dumps({'hooks':{'PreToolUse':[{'hooks':[{'command':'/bin/true moriarty'}]}]}}))
        cli=Path(hook.__file__).with_name('cli.py')
        p=subprocess.run([sys.executable,str(cli),'--repo',str(self.root),'doctor','--json'],env={**os.environ,'CODEX_HOME':str(home)},capture_output=True,text=True,timeout=5)
        self.assertEqual(p.returncode,0,p.stderr)
        d=json.loads(p.stdout);self.assertNotEqual(d['hostCoverage'],'host-verified')
        self.assertIn('unverified',d['hostCoverage'])

    def test_registered_raw_argv_requires_guarded_cli(self):
        argv=['python3','driver.py']
        (self.root/'commands.json').write_text(json.dumps({'commands':{'driver':{'argv':argv}}}))
        with patch.object(hook,'read_actions',return_value=[self.action]):
            out=hook.handle_pre_tool_use({'tool_name':'Bash','tool_input':{'command':argv}},self.root)
            self.assertEqual(out['hookSpecificOutput']['permissionDecision'],'deny')
            self.assertIn('guarded CLI',out['hookSpecificOutput']['permissionDecisionReason'])

    def test_bad_records_produce_diagnostic_without_host_claim(self):
        with patch.object(hook,'read_actions',side_effect=ValueError('corrupt')):
            out=hook.handle_pre_tool_use({'tool_name':'Bash','tool_input':{'command':'true'}},self.root)
            self.assertIn('unverified',out['systemMessage'])

    def test_locked_store_returns_diagnostic_within_host_bound(self):
        db=store.get_db_path(self.root)
        store.enqueue_tx(db,str(self.root),'tx-locked','submitted',{'network':'preview'})
        with sqlite3.connect(db) as c:
            c.execute('PRAGMA journal_mode=DELETE')
            c.execute('BEGIN EXCLUSIVE')
            p=subprocess.run([sys.executable,hook.__file__,'PostToolUse'],input=json.dumps({'cwd':str(self.root)}),text=True,capture_output=True,timeout=0.95)
        self.assertEqual(p.returncode,0,p.stderr)
        self.assertIn('unverified',json.loads(p.stdout)['systemMessage'])

    def test_invalid_cwd_returns_json_diagnostic(self):
        p=subprocess.run([sys.executable,hook.__file__,'SessionStart'],input=json.dumps({'cwd':'\x00'}),text=True,capture_output=True,timeout=0.95)
        self.assertEqual(p.returncode,0,p.stderr)
        self.assertIn('systemMessage',json.loads(p.stdout))

if __name__=='__main__': unittest.main()

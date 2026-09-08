"""Public notification lifecycle and host-observed delivery regressions."""
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from datetime import datetime, timezone
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
from moriarty_dev import store

CLI = Path(__file__).resolve().parents[1] / 'scripts/moriarty_dev/cli.py'
THREAD = '00000000-0000-0000-0000-000000000001'

class Notifications(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        (self.root / '.git').mkdir()
        self.db = store.get_db_path(self.root)
        self.host = self.root / 'codex'
        self.session = self.host / 'sessions/2026/09/08' / ('rollout-' + THREAD + '.jsonl')
        self.session.parent.mkdir(parents=True)
        self.env = {**os.environ, 'CODEX_HOME':str(self.host), 'CODEX_THREAD_ID':THREAD, 'CODEX_SESSION_ID':THREAD}
        self.session.write_text(json.dumps({'type':'session_meta','payload':{'id':THREAD}})+'\n')
        store.enqueue_tx(self.db, str(self.root), 'tx-test-1', 'submitted', {'network':'preview'})

    def tearDown(self):
        self.tmp.cleanup()

    def pending(self):
        return store.get_undelivered_txs(self.db, str(self.root))

    def row(self):
        with sqlite3.connect(self.db) as c:
            return c.execute('SELECT status, delivered_at, enqueued_at FROM outbox').fetchone()

    def message(self, status='submitted', role='assistant', phase='commentary', timestamp=None, text=None):
        d={'timestamp':timestamp or datetime.now(timezone.utc).isoformat(), 'type':'response_item', 'payload':{
            'type':'message','role':role,'phase':phase,'id':'host-message-1', 'content':[{'type':'output_text',
            'text':text or f'Midnight Preview transaction tx-test-1: {status}.'}]}}
        with self.session.open('a') as f: f.write(json.dumps(d)+'\n')

    def cli(self, *args):
        return subprocess.run([sys.executable,str(CLI),'--repo',str(self.root),*args],env=self.env,capture_output=True,text=True,timeout=5)

    def test_duplicate_cannot_roll_back_confirmation(self):
        store.update_tx_status(self.db, 'tx-test-1', 'confirmed')
        store.enqueue_tx(self.db, str(self.root), 'tx-test-1','submitted', {'network':'preview'})
        self.assertEqual(self.row()[0], 'confirmed')

    def test_delivery_preserves_chain_status(self):
        self.message()
        p=self.cli('deliver','--tx-id','tx-test-1','--json')
        self.assertEqual(p.returncode,0,p.stderr)
        self.assertEqual(self.row()[0], 'submitted')
        self.assertIsNotNone(self.row()[1])
        self.assertEqual(self.pending(), [])

    def test_confirmation_after_delivery_becomes_pending_again(self):
        self.message()
        self.assertEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        before=self.row()[2]
        store.update_tx_status(self.db, 'tx-test-1', 'confirmed')
        self.assertEqual(self.pending()[0]['status'],'confirmed')
        self.assertNotEqual(self.row()[2], before)
        self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        self.message(status='confirmed',phase='final_answer')
        self.assertEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)

    def test_caller_json_is_not_delivery(self):
        ack=self.root/'ack.json'
        ack.write_text(json.dumps({'txId':'tx-test-1','deliveryId':'claimed'}))
        self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1','--acknowledgement',str(ack)).returncode,0)
        self.assertEqual(len(self.pending()),1)

    def test_user_tool_analysis_old_and_quoted_messages_do_not_deliver(self):
        for kw in [{'role':'user'},{'role':'tool'}, {'phase':'analysis'}, {'timestamp':'2000-01-01T00:00:00Z'},
                   {'text':'```\nMidnight Preview transaction tx-test-1: submitted.\n```'},
                   {'text':'````\n```\nMidnight Preview transaction tx-test-1: submitted.\n````'},
                   {'text':'```\n    ```\nMidnight Preview transaction tx-test-1: submitted.\n```'},
                   {'text':'```\n\t```\nMidnight Preview transaction tx-test-1: submitted.\n```'},
                   {'text':'<!--\nMidnight Preview transaction tx-test-1: submitted.\n-->'},
                   {'text':'Example:\nMidnight Preview transaction tx-test-1: submitted.'},
                   {'text':'Example: Midnight Preview transaction tx-test-1: submitted.'}]:
            with self.subTest(kw=kw):
                self.session.write_text(json.dumps({'type':'session_meta','payload':{'id':THREAD}})+'\n')
                self.message(**kw)
                self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
                self.assertEqual(len(self.pending()),1)

    def test_wrong_host_session_header_rejected(self):
        self.session.write_text(json.dumps({'type':'session_meta','payload':{'id':'different'}})+'\n')
        self.message()
        self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)

    def test_stale_observation_cannot_ack_new_status(self):
        from moriarty_dev.notifications import observe_delivery
        self.message()
        with patch.dict(os.environ,self.env): ack=observe_delivery(self.pending()[0])
        store.update_tx_status(self.db,'tx-test-1','confirmed')
        with self.assertRaises(store.StoreError): store.mark_delivered(self.db,'tx-test-1',ack)
        self.assertEqual(self.pending()[0]['status'],'confirmed')

    def test_public_cli_observation_reports_and_updates(self):
        p=self.cli('notify','--tx-id','tx-test-2','--status','submitted','--json')
        self.assertEqual(p.returncode,0,p.stderr)
        self.assertEqual(self.cli('notify','--tx-id','tx-test-2','--status','confirmed').returncode,0)
        p=self.cli('report','--json')
        data=json.loads(p.stdout)
        self.assertIn(('tx-test-2','confirmed'),[(t['txId'],t['status']) for t in data['pendingTransactions']])
        self.assertNotEqual(self.cli('notify','--tx-id','unknown','--status','confirmed').returncode,0)

    def test_newline_id_and_private_details_rejected(self):
        with self.assertRaises((ValueError,store.StoreError)):
            store.enqueue_tx(self.db,str(self.root),'tx\nforged','submitted',{})
        with self.assertRaises((ValueError,store.StoreError)):
            store.enqueue_tx(self.db,str(self.root),'tx-2','submitted',{'publicSink':{'witness':'secret'}})
        for details in ({'note':'arbitrary receipt'}, {'blockNumber':'secret'}, {'blockNumber':True}, {'network':'mainnet'}):
            with self.subTest(details=details), self.assertRaises((ValueError,store.StoreError)):
                store.enqueue_tx(self.db,str(self.root),'tx-2','submitted',details)
        with self.assertRaises(store.StoreError):
            store.update_tx_status(self.db,'tx-test-1','confirmed',note='private receipt')

    def test_duplicate_preserves_delivery_and_does_not_create_new_event(self):
        self.message()
        self.assertEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        row=self.row()
        store.enqueue_tx(self.db,str(self.root),'tx-test-1','submitted',{'network':'preview'})
        self.assertEqual(self.row(),row)
        with sqlite3.connect(self.db) as c:
            self.assertEqual(c.execute("SELECT count(*) FROM events WHERE event_kind='tx_notification'").fetchone()[0],1)

    def test_absent_or_ambiguous_host_stays_pending(self):
        self.message()
        other=self.session.with_name('rollout-other-'+THREAD+'.jsonl')
        other.write_bytes(self.session.read_bytes())
        self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        other.unlink()
        self.session.unlink()
        self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        self.assertEqual(len(self.pending()),1)

    def test_notification_visible_from_linked_worktree(self):
        linked=self.root/'linked'; linked.mkdir()
        gitdir=self.root/'.git/worktrees/linked';gitdir.mkdir(parents=True)
        (gitdir/'commondir').write_text('../..')
        (linked/'.git').write_text('gitdir: '+str(gitdir))
        self.assertEqual(store.get_undelivered_txs(store.get_db_path(linked),str(linked)),self.pending())

    def test_delivered_event_contains_no_conversation_text(self):
        self.message(text='Unrelated private conversation.\nMidnight Preview transaction tx-test-1: submitted.')
        self.assertNotEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        self.message()
        self.assertEqual(self.cli('deliver','--tx-id','tx-test-1').returncode,0)
        with sqlite3.connect(self.db) as c:
            data=c.execute("SELECT payload_json FROM events WHERE event_kind='tx_delivered'").fetchone()[0]
        self.assertNotIn('Unrelated private conversation',data)
        self.assertEqual(json.loads(data)['deliveryId'],'host-message-1')

    def test_dedicated_message_can_report_multiple_transactions(self):
        store.enqueue_tx(self.db,str(self.root),'tx-test-2','submitted',{'network':'preview'})
        self.message(text='Midnight Preview transaction tx-test-1: submitted.\n\nMidnight Preview transaction tx-test-2: submitted.')
        for tx in ('tx-test-1','tx-test-2'):
            self.assertEqual(self.cli('deliver','--tx-id',tx).returncode,0)
        self.assertEqual(self.pending(),[])

if __name__=='__main__': unittest.main()

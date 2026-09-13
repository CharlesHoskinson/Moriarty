"""Store connections close on both successful reads and failed initialization."""
import sqlite3
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "scripts"))
from moriarty_dev import store


class StoreConnections(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.path = Path(self.temp.name) / "state.sqlite3"
        self.connections = []
        self.fail_on = None
        owner = self

        class TrackedConnection(sqlite3.Connection):
            closed = False

            def execute(self, sql, *args):
                if sql == owner.fail_on:
                    raise sqlite3.OperationalError("injected SQL failure")
                return super().execute(sql, *args)

            def close(self):
                self.closed = True
                super().close()

        real_connect = sqlite3.connect

        def connect(*args, **kwargs):
            conn = real_connect(*args, **kwargs, factory=TrackedConnection)
            self.connections.append(conn)
            self.addCleanup(conn.close)
            return conn

        self.patcher = patch.object(store.sqlite3, "connect", connect)
        self.patcher.start()
        self.addCleanup(self.patcher.stop)

    def test_primary_query_closes_success_and_failure(self):
        store.init_db(self.path).close()
        self.assertFalse(store.has_other_primary(self.path, "none"))
        self.assertTrue(all(c.closed for c in self.connections))
        with self.assertRaises(sqlite3.OperationalError):
            store.has_other_primary(Path(self.temp.name) / "empty.sqlite3", "none")
        self.assertTrue(all(c.closed for c in self.connections))

    def test_failed_integrity_probe_closes_connection(self):
        store.init_db(self.path).close()
        self.fail_on = "PRAGMA schema_version;"
        with self.assertRaises(store.StoreError):
            store.init_db(self.path)
        self.assertTrue(all(c.closed for c in self.connections))

    def test_failed_initialization_closes_connection(self):
        self.fail_on = "PRAGMA journal_mode=WAL;"
        with self.assertRaises(sqlite3.OperationalError):
            store.init_db(self.path)
        self.assertTrue(all(c.closed for c in self.connections))

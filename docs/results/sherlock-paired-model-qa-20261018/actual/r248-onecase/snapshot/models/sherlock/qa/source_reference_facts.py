"""Summarize exact bytes already read by an ordinary checker.

No files are opened here. Membership describes explicitly recorded input reads,
not Python's full import closure, reviewer coverage or execution authenticity.
"""
import hashlib


class ReadInventory:
    def __init__(self):
        self._files = {}
        self._events = 0
        self._bytes = 0

    def record(self, path, raw):
        """Record the same byte object the caller consumes, without reopening it."""
        digest = hashlib.sha256(raw).hexdigest()
        snapshots = self._files.setdefault(str(path), {})
        key = (digest, len(raw))
        snapshots[key] = snapshots.get(key, 0) + 1
        self._events += 1
        self._bytes += len(raw)

    def report(self):
        files = []
        for path, snapshots in sorted(self._files.items()):
            files.append(dict(
                path=path, read_count=sum(snapshots.values()),
                changed_between_reads=len(snapshots) > 1,
                snapshots=[dict(sha256=digest, bytes=size, read_count=count)
                           for (digest, size), count in sorted(snapshots.items())],
            ))
        return dict(
            scope='explicit-checker-input-byte-reads',
            unique_paths=len(files), read_events=self._events,
            bytes_read=self._bytes, files=files,
            dependency_closure='not-established',
            reviewer_read_extent='not-observed', execution_evidence='none',
        )

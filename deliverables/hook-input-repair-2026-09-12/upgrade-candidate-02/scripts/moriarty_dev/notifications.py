"""Observe an emitted public message in the current Codex session.

Local session files are host evidence within the non-adversarial local-user
boundary. They establish an emitted message, not that a person read it or that
Midnight finalized a transaction. No transcript text is copied into the store.
"""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re

from moriarty_dev.store import StoreError

MAX_TAIL = 2 * 1024 * 1024


def notification_line(tx):
    return f"Midnight Preview transaction {tx['txId']}: {tx['status']}."


def _time(value):
    value = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if value.tzinfo is None:
        raise ValueError('Timestamp must include timezone')
    return value


def _contains_line(text, expected):
    # A closed message format avoids confusing examples, quotes, hidden HTML
    # or Markdown code blocks with an actual notification. No Markdown parser.
    lines = [line for line in text.splitlines() if line]
    pattern = r'Midnight Preview transaction [A-Za-z0-9:_-]{1,256}: (submitted|unknown-finality|failed|confirmed)\.'
    return expected in lines and all(re.fullmatch(pattern, line) for line in lines)


def observe_delivery(tx):
    thread = os.environ.get('CODEX_THREAD_ID') or os.environ.get('CODEX_SESSION_ID', '')
    if not re.fullmatch(r'[0-9a-fA-F-]{36}', thread):
        raise StoreError('Current Codex session identity unavailable; notification remains pending')
    home = Path(os.environ.get('CODEX_HOME', Path.home() / '.codex'))
    sessions = home / 'sessions'
    paths = list(sessions.glob(f'*/*/*/rollout-*{thread}.jsonl'))
    if len(paths) != 1:
        raise StoreError('Unique current host transcript unavailable; notification remains pending')
    path = paths[0]
    expected = notification_line(tx)
    since = _time(tx['enqueuedAt'])
    now = datetime.now(timezone.utc)
    with path.open('rb') as stream:
        header = json.loads(stream.readline(65536))
        if header.get('type') != 'session_meta' or header.get('payload', {}).get('id') != thread:
            raise StoreError('Host transcript identity mismatch')
        stream.seek(0, 2)
        size = stream.tell()
        start = max(0, size - MAX_TAIL)
        stream.seek(start)
        if start:
            stream.readline(MAX_TAIL)
        tail = stream.read(MAX_TAIL)
    for raw in reversed(tail.splitlines()):
        try:
            record = json.loads(raw)
            payload = record.get('payload', {})
            if (record.get('type') != 'response_item' or payload.get('type') != 'message'
                    or payload.get('role') != 'assistant' or payload.get('phase') not in ('commentary', 'final_answer')):
                continue
            observed = _time(record['timestamp'])
            if observed < since or observed > now:
                continue
            delivery_id = payload.get('id')
            if not isinstance(delivery_id, str) or not delivery_id:
                continue
            text = '\n'.join(item['text'] for item in payload.get('content', [])
                             if item.get('type') == 'output_text' and isinstance(item.get('text'), str))
            if not _contains_line(text, expected):
                continue
            return {'txId':tx['txId'], 'status':tx['status'], 'enqueuedAt':tx['enqueuedAt'],
                    'source':'codex-session-message', 'sessionId':thread, 'deliveryId':delivery_id,
                    'observedAt':record['timestamp'], 'contentSha256':hashlib.sha256(text.encode()).hexdigest()}
        except (ValueError, TypeError, KeyError, AttributeError):
            continue
    raise StoreError('No matching emitted host message in bounded transcript tail; notification remains pending')

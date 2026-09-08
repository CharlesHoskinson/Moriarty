import hashlib
import json
import os
import sqlite3
import time
import uuid
from datetime import datetime, timezone
from pathlib import Path

SCHEMA_SQL = """
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    repository TEXT NOT NULL,
    requirement TEXT NOT NULL,
    capability TEXT NOT NULL,
    candidate TEXT NOT NULL,
    action_id TEXT NOT NULL,
    event_kind TEXT NOT NULL,
    observed_at TEXT NOT NULL,
    payload_json TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS reservations (
    id TEXT PRIMARY KEY,
    repository TEXT NOT NULL,
    requirement TEXT NOT NULL,
    capability TEXT NOT NULL,
    candidate TEXT NOT NULL,
    action_id TEXT NOT NULL,
    action_kind TEXT NOT NULL,
    pid INTEGER,
    status TEXT NOT NULL,
    reserved_at TEXT NOT NULL,
    finished_at TEXT,
    receipt_json TEXT
);

CREATE TABLE IF NOT EXISTS admin_intervals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    repository TEXT NOT NULL,
    start_time REAL NOT NULL,
    end_time REAL NOT NULL,
    activity_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS outbox (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    repository TEXT NOT NULL,
    tx_id TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL,
    enqueued_at TEXT NOT NULL,
    delivered_at TEXT,
    details_json TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_lineage
    ON events (repository, requirement, capability);

CREATE INDEX IF NOT EXISTS idx_reservations_status
    ON reservations (repository, status, action_kind);
"""

FORBIDDEN_TX_KEYS = {
    "seed", "sk", "secret", "private", "witness", "private_key",
    "spending_key", "viewing_key", "auth_token", "password"
}


class StoreError(Exception):
    pass


class ReservationConflictError(StoreError):
    pass


def get_db_path(repo_root: str | Path) -> Path:
    repo_path = Path(repo_root).resolve()
    git_entry = repo_path / ".git"
    try:
        if git_entry.is_file():
            text = git_entry.read_text(encoding="utf-8").strip()
            if text.startswith("gitdir:"):
                gitdir = Path(text.split("gitdir:", 1)[1].strip())
                if not gitdir.is_absolute():
                    gitdir = (repo_path / gitdir).resolve()
                commondir_file = gitdir / "commondir"
                if commondir_file.is_file():
                    cd_text = commondir_file.read_text(encoding="utf-8").strip()
                    common = Path(cd_text)
                    if not common.is_absolute():
                        common = (gitdir / common).resolve()
                    target = common / "moriarty-dev" / "state.sqlite3"
                    return target
                return gitdir / "moriarty-dev" / "state.sqlite3"
        elif git_entry.is_dir():
            return git_entry / "moriarty-dev" / "state.sqlite3"
    except Exception:
        pass
    fallback = repo_path / ".moriarty-dev" / "state.sqlite3"
    return fallback


def init_db(db_path: Path) -> sqlite3.Connection:
    db_path.parent.mkdir(parents=True, exist_ok=True)
    conn = sqlite3.connect(str(db_path), timeout=10.0, isolation_level=None)
    conn.execute("PRAGMA journal_mode=WAL;")
    conn.execute("PRAGMA busy_timeout=5000;")
    conn.executescript(SCHEMA_SQL)
    return conn


def _now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()


def make_history_reader(db_path: Path):
    def history_reader(lineage: dict) -> dict | None:
        repo = lineage.get("repository", "")
        req = lineage.get("requirement", "")
        cap = lineage.get("capability", "")
        return get_history(db_path, repo, req, cap)
    return history_reader


def get_history(db_path: Path, repo: str, requirement: str, capability: str) -> dict | None:
    if not db_path.is_file():
        return None

    try:
        conn = sqlite3.connect(str(db_path), timeout=10.0)
    except Exception:
        return None
    try:
        cur = conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='events'")
        if not cur.fetchone():
            return None

        cur = conn.execute(
            """
            SELECT id, pid FROM reservations
            WHERE repository = ? AND status = 'active'
              AND action_kind IN ('implement', 'repair')
            """,
            (repo,),
        )
        active_rows = cur.fetchall()
        primary_active = False
        for res_id, pid in active_rows:
            if pid and _is_pid_alive(pid):
                primary_active = True
                break
            elif not pid:
                primary_active = True
                break

        cur = conn.execute(
            """
            SELECT event_kind, payload_json FROM events
            WHERE repository = ? AND requirement = ? AND capability = ?
            ORDER BY id ASC
            """,
            (repo, requirement, capability),
        )
        rows = cur.fetchall()

        unresolved_findings = set()
        reproducer_verified = False
        approach_changed = False

        for kind, payload_raw in rows:
            try:
                payload = json.loads(payload_raw)
            except Exception:
                payload = {}

            if kind in ("defect_failure", "result_review_failure"):
                fid = payload.get("findingId") or payload.get("defectClass") or payload.get("id") or str(len(unresolved_findings) + 1)
                unresolved_findings.add(fid)
                reproducer_verified = False
                approach_changed = False
            elif kind == "approach_changed":
                approach_changed = True
            elif kind == "reproducer_verified":
                reproducer_verified = True
            elif kind == "defect_resolved":
                fid = payload.get("findingId") or payload.get("defectClass")
                if fid and fid in unresolved_findings:
                    unresolved_findings.discard(fid)

        cur = conn.execute(
            """
            SELECT count(*) FROM events
            WHERE repository = ? AND event_kind = 'admin_cycle'
            """,
            (repo,),
        )
        admin_cycles = cur.fetchone()[0]

        cur = conn.execute(
            """
            SELECT start_time, end_time FROM admin_intervals
            WHERE repository = ? AND activity_type NOT IN ('test', 'testing', 'afk', 'idle')
            ORDER BY start_time ASC
            """,
            (repo,),
        )
        intervals = cur.fetchall()
        admin_seconds = int(_merge_intervals(intervals))

        return {
            "sameDefectFailures": len(unresolved_findings),
            "adminCycles": admin_cycles,
            "adminSeconds": admin_seconds,
            "primaryActive": primary_active,
            "reproducerVerified": reproducer_verified,
            "approachChanged": approach_changed,
        }
    finally:
        conn.close()


def bootstrap_store(db_path: Path, repo: str, initial_findings: list[dict] | None = None) -> sqlite3.Connection:
    conn = init_db(db_path)
    now = _now_iso()
    try:
        with conn:
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, '', '', '', 'bootstrap', 'bootstrap', ?, ?)
                """,
                (repo, now, json.dumps({"bootstrappedAt": now, "repository": repo})),
            )
            if initial_findings is None:
                initial_findings = [
                    {
                        "findingId": "SP05-DRIVER-DEFECT-RECURRENCE-01",
                        "defectClass": "driver-admitted-not-executed",
                        "requirement": "SP05.1",
                        "capability": "fixed-financial-driver",
                        "candidate": "daac83578f105b0a70ce44d88c579018881daae5",
                        "description": "September 6 post-mortem recurrence 1: ledger driver returns admitted-not-executed",
                    },
                    {
                        "findingId": "SP05-DRIVER-DEFECT-RECURRENCE-02",
                        "defectClass": "driver-admitted-not-executed",
                        "requirement": "SP05.1",
                        "capability": "fixed-financial-driver",
                        "candidate": "daac83578f105b0a70ce44d88c579018881daae5",
                        "description": "September 8 post-mortem recurrence 2: ledger driver returns admitted-not-executed",
                    },
                ]
            for f in initial_findings:
                req = f.get("requirement", "SP05.1")
                cap = f.get("capability", "fixed-financial-driver")
                cand = f.get("candidate", "daac83578f105b0a70ce44d88c579018881daae5")
                conn.execute(
                    """
                    INSERT INTO events (
                        repository, requirement, capability, candidate,
                        action_id, event_kind, observed_at, payload_json
                    ) VALUES (?, ?, ?, ?, 'bootstrap-finding', 'defect_failure', ?, ?)
                    """,
                    (repo, req, cap, cand, now, json.dumps(f)),
                )
    except Exception:
        pass
    finally:
        conn.close()


def _is_pid_alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
        return True
    except (OSError, ProcessLookupError):
        return False


def _merge_intervals(intervals: list[tuple[float, float]]) -> float:
    if not intervals:
        return 0.0
    merged = []
    for start, end in intervals:
        if end < start:
            continue
        if not merged:
            merged.append([start, end])
        else:
            prev = merged[-1]
            if start <= prev[1]:
                prev[1] = max(prev[1], end)
            else:
                merged.append([start, end])
    return sum(end - start for start, end in merged)


def reserve(db_path: Path, repo: str, action: dict, snapshot: dict) -> str:
    conn = init_db(db_path)
    try:
        conn.execute("BEGIN IMMEDIATE")
        action_kind = action.get("kind", "")
        if action_kind in ("implement", "repair"):
            cur = conn.execute(
                """
                SELECT id, pid FROM reservations
                WHERE repository = ? AND status = 'active'
                  AND action_kind IN ('implement', 'repair')
                """,
                (repo,),
            )
            rows = cur.fetchall()
            for res_id, pid in rows:
                if pid and not _is_pid_alive(pid):
                    conn.execute(
                        "UPDATE reservations SET status = 'crashed', finished_at = ? WHERE id = ?",
                        (_now_iso(), res_id),
                    )
                else:
                    raise ReservationConflictError(
                        f"Active primary implementation reservation already exists: {res_id}"
                    )

        res_id = str(uuid.uuid4())
        pid = os.getpid()
        now = _now_iso()
        conn.execute(
            """
            INSERT INTO reservations (
                id, repository, requirement, capability, candidate,
                action_id, action_kind, pid, status, reserved_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'active', ?)
            """,
            (
                res_id,
                repo,
                action.get("requirement", ""),
                action.get("capability", ""),
                action.get("candidate", ""),
                action.get("id", ""),
                action_kind,
                pid,
                now,
            ),
        )

        conn.execute(
            """
            INSERT INTO events (
                repository, requirement, capability, candidate,
                action_id, event_kind, observed_at, payload_json
            ) VALUES (?, ?, ?, ?, ?, 'action_start', ?, ?)
            """,
            (
                repo,
                action.get("requirement", ""),
                action.get("capability", ""),
                action.get("candidate", ""),
                action.get("id", ""),
                now,
                json.dumps({"action": action, "snapshot": snapshot}),
            ),
        )
        conn.commit()
        return res_id
    except Exception:
        conn.rollback()
        raise
    finally:
        conn.close()


def finish(db_path: Path, reservation_id: str, receipt: dict | None = None, status: str = "finished"):
    conn = init_db(db_path)
    try:
        conn.execute("BEGIN IMMEDIATE")
        now = _now_iso()
        cur = conn.execute(
            "SELECT repository, requirement, capability, candidate, action_id FROM reservations WHERE id = ?",
            (reservation_id,),
        )
        row = cur.fetchone()
        receipt_str = json.dumps(receipt or {})
        conn.execute(
            """
            UPDATE reservations
            SET status = ?, finished_at = ?, receipt_json = ?
            WHERE id = ?
            """,
            (status, now, receipt_str, reservation_id),
        )
        if row:
            repo, req, cap, cand, act_id = row
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, ?, ?, ?, ?, 'action_end', ?, ?)
                """,
                (repo, req, cap, cand, act_id, now, receipt_str),
            )
        conn.commit()
    except Exception:
        conn.rollback()
        raise
    finally:
        conn.close()


def record_event(
    db_path: Path,
    repo: str,
    requirement: str,
    capability: str,
    candidate: str,
    action_id: str,
    event_kind: str,
    payload: dict,
):
    conn = init_db(db_path)
    try:
        with conn:
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    repo,
                    requirement,
                    capability,
                    candidate,
                    action_id,
                    event_kind,
                    _now_iso(),
                    json.dumps(payload),
                ),
            )
    finally:
        conn.close()


def record_review(db_path: Path, repo: str, receipt_data: dict):
    if not isinstance(receipt_data, dict):
        raise StoreError("Receipt must be a JSON object")

    author = receipt_data.get("author")
    reviewer = (
        receipt_data.get("reviewer")
        or receipt_data.get("reviewerObserved")
        or receipt_data.get("reviewerRequested")
    )
    if not reviewer or not isinstance(reviewer, str):
        raise StoreError("Reviewer identity required in review receipt")
    if author and isinstance(author, str):
        a_low = author.lower().strip()
        r_low = reviewer.lower().strip()
        if a_low in r_low or r_low in a_low:
            raise StoreError("Author self-review is forbidden")

    cand = receipt_data.get("candidateHash", receipt_data.get("candidate", ""))
    if not cand or not isinstance(cand, str) or len(cand) < 16:
        raise StoreError("Valid candidateHash required in review receipt")

    scope = receipt_data.get("scope", receipt_data.get("auditScope", ""))
    if not scope or not isinstance(scope, str) or scope.strip() == "" or scope.strip().lower() == "none":
        raise StoreError("Valid non-empty scope required in review receipt")

    req = receipt_data.get("requirement", "")
    cap = receipt_data.get("capability", "")
    verdict = receipt_data.get("verdict", receipt_data.get("scopedVerdict", ""))
    findings = receipt_data.get("findings", [])

    conn = init_db(db_path)
    try:
        with conn:
            event_kind = "result_review" if verdict == "APPROVED" else "result_review_failure"
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, ?, ?, ?, 'review', ?, ?, ?)
                """,
                (
                    repo,
                    req,
                    cap,
                    cand,
                    event_kind,
                    _now_iso(),
                    json.dumps(receipt_data),
                ),
            )
            if verdict != "APPROVED":
                if not findings:
                    fid = receipt_data.get("findingId") or f"REVIEW-FAIL-{cand[:8]}"
                    findings = [{"findingId": fid, "defectClass": "review_rejected", "description": "Review not approved"}]
                for finding in findings:
                    if isinstance(finding, str):
                        finding = {"findingId": finding, "defectClass": finding}
                    conn.execute(
                        """
                        INSERT INTO events (
                            repository, requirement, capability, candidate,
                            action_id, event_kind, observed_at, payload_json
                        ) VALUES (?, ?, ?, ?, 'review-finding', 'defect_failure', ?, ?)
                        """,
                        (
                            repo,
                            req,
                            cap,
                            cand,
                            _now_iso(),
                            json.dumps(finding),
                        ),
                    )
    finally:
        conn.close()


def record_admin_interval(db_path: Path, repo: str, start_time: float, end_time: float, activity_type: str):
    conn = init_db(db_path)
    try:
        with conn:
            conn.execute(
                """
                INSERT INTO admin_intervals (repository, start_time, end_time, activity_type)
                VALUES (?, ?, ?, ?)
                """,
                (repo, start_time, end_time, activity_type),
            )
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, '', '', '', 'admin', 'admin_cycle', ?, ?)
                """,
                (
                    repo,
                    _now_iso(),
                    json.dumps({
                        "startTime": start_time,
                        "endTime": end_time,
                        "duration": end_time - start_time,
                        "type": activity_type,
                    }),
                ),
            )
    finally:
        conn.close()


def enqueue_tx(db_path: Path, repo: str, tx_id: str, status: str, details: dict):
    forbidden_subs = ("seed", "sk", "secret", "private", "witness", "password", "token", "auth", "spending_key", "signing_key")
    def _check_forbidden(d):
        if isinstance(d, dict):
            for k, v in d.items():
                k_low = k.lower()
                for sub in forbidden_subs:
                    if sub in k_low and k_low not in ("publicsink", "public_sink"):
                        raise ValueError(f"Forbidden private field in transaction details: {k}")
                _check_forbidden(v)
        elif isinstance(d, list):
            for item in d:
                _check_forbidden(item)

    _check_forbidden(details)

    conn = init_db(db_path)
    try:
        with conn:
            conn.execute(
                """
                INSERT INTO outbox (repository, tx_id, status, enqueued_at, details_json)
                VALUES (?, ?, ?, ?, ?)
                ON CONFLICT(tx_id) DO UPDATE SET
                    status = excluded.status,
                    details_json = excluded.details_json
                """,
                (repo, tx_id, status, _now_iso(), json.dumps(details)),
            )
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, '', '', '', 'tx', 'tx_notification', ?, ?)
                """,
                (
                    repo,
                    _now_iso(),
                    json.dumps({"txId": tx_id, "status": status, "details": details}),
                ),
            )
    finally:
        conn.close()


VALID_TX_STATUSES = {"submitted", "unknown-finality", "failed", "confirmed", "delivered"}


def update_tx_status(db_path: Path, tx_id: str, new_status: str, note: str | None = None):
    if new_status not in VALID_TX_STATUSES:
        raise StoreError(f"Invalid transaction status: {new_status}")
    conn = init_db(db_path)
    try:
        with conn:
            cur = conn.execute("SELECT repository, details_json FROM outbox WHERE tx_id = ?", (tx_id,))
            row = cur.fetchone()
            if not row:
                raise StoreError(f"Transaction not found in outbox: {tx_id}")
            repo, details_str = row
            try:
                details = json.loads(details_str)
            except Exception:
                details = {}
            if note:
                details["statusNote"] = note
            conn.execute(
                "UPDATE outbox SET status = ?, details_json = ? WHERE tx_id = ?",
                (new_status, json.dumps(details), tx_id),
            )
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, '', '', '', 'tx', 'tx_status_change', ?, ?)
                """,
                (repo, _now_iso(), json.dumps({"txId": tx_id, "newStatus": new_status, "note": note})),
            )
    finally:
        conn.close()


def get_undelivered_txs(db_path: Path, repo: str | None = None) -> list[dict]:
    if not db_path.is_file():
        return []
    conn = sqlite3.connect(str(db_path))
    try:
        if repo:
            cur = conn.execute(
                "SELECT tx_id, status, details_json, enqueued_at FROM outbox WHERE repository = ? AND delivered_at IS NULL ORDER BY id ASC",
                (repo,),
            )
        else:
            cur = conn.execute(
                "SELECT tx_id, status, details_json, enqueued_at FROM outbox WHERE delivered_at IS NULL ORDER BY id ASC"
            )
        rows = cur.fetchall()
        result = []
        for tx_id, status, details_str, enq in rows:
            try:
                details = json.loads(details_str)
            except Exception:
                details = {}
            result.append({
                "txId": tx_id,
                "status": status,
                "enqueuedAt": enq,
                "details": details,
            })
        return result
    finally:
        conn.close()


def mark_delivered(db_path: Path, tx_id: str):
    conn = init_db(db_path)
    now = _now_iso()
    try:
        with conn:
            conn.execute(
                "UPDATE outbox SET delivered_at = ?, status = 'delivered' WHERE tx_id = ?",
                (now, tx_id),
            )
            conn.execute(
                """
                INSERT INTO events (
                    repository, requirement, capability, candidate,
                    action_id, event_kind, observed_at, payload_json
                ) VALUES (?, '', '', '', 'tx', 'tx_delivered', ?, ?)
                """,
                ("", now, json.dumps({"txId": tx_id, "deliveredAt": now})),
            )
    finally:
        conn.close()

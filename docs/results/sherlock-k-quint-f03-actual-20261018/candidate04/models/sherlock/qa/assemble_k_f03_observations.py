"""Passive offline assembler for frozen F03 K standard-log captures.

Consumes supplied actual child receipts, stderr logs, Kore states, kprint
configurations, retained F03 input bytes, and the compiled main.k rule-span
inventory envelope; assembles the 95 post-startup captures plus separate
startup (exec-001/print-001) and terminal (exec-096/print-096) probes while
preserving pre-startup artifacts (kast, initializer.kore, exec-000, print-000)
outside typed post-startup state projection; and delegates decoding to
k_capture_observations.decode_capture(..., max_captures=F03_MAX_CAPTURES) and
k_capture_observations.observation_bundle(...).

No subprocesses, K tools, Quint runners, or synthetic receipt generators exist
in this module.

Author and provenance notice:
- Authored in a fresh designated source-author context (returned runtime model
  identity: Gemini 4 Argon (High), standing observed fallback for Claude Opus 5.5
  (High) quota outage; not a continuation of prior author CID
  7356947d-bd19-4cc5-a58b-643f0d3bf399, and not an independent audit vote).
- Addresses all six R303 source review findings (F03-ASM-01 through F03-ASM-06).
"""

from __future__ import annotations

import base64
import hashlib
import json
from pathlib import Path
import re
import stat

try:
    from models.sherlock.qa.k_capture_observations import (
        ADAPTER_SOURCES,
        BUNDLE_FORMAT,
        EMPTY_SHA256,
        ENGINE_RECEIPT_KEYS,
        F03_MAX_CAPTURES,
        FORMAT,
        LOG_FORMAT,
        MAX_ARG_CHARS,
        MAX_ARGV,
        MAX_CAPTURES,
        MAX_CONFIGURATION_BYTES,
        MAX_KORE_BYTES,
        MAX_LOG_BYTES,
        MAX_LOG_LINES,
        MAX_NESTING,
        MAX_RULE_SPANS,
        MAX_TERM_BYTES,
        PRINT_RECEIPT_KEYS,
        PRODUCER,
        decode_capture,
        observation_bundle,
        retained_run_input,
    )
except ImportError:  # pragma: no cover
    from k_capture_observations import (
        ADAPTER_SOURCES,
        BUNDLE_FORMAT,
        EMPTY_SHA256,
        ENGINE_RECEIPT_KEYS,
        F03_MAX_CAPTURES,
        FORMAT,
        LOG_FORMAT,
        MAX_ARG_CHARS,
        MAX_ARGV,
        MAX_CAPTURES,
        MAX_CONFIGURATION_BYTES,
        MAX_KORE_BYTES,
        MAX_LOG_BYTES,
        MAX_LOG_LINES,
        MAX_NESTING,
        MAX_RULE_SPANS,
        MAX_TERM_BYTES,
        PRINT_RECEIPT_KEYS,
        PRODUCER,
        decode_capture,
        observation_bundle,
        retained_run_input,
    )


ASSEMBLER_FORMAT = 'sherlock-k-f03-assembled-capture/1'
CASE_ID = 'F03'
F03_INITIAL_FUEL = 93
F03_STACK_LIMIT = 64
F03_CHARGED_STEPS = 93
F03_POST_STARTUP_CAPTURES = F03_MAX_CAPTURES
F03_STAGE_MAX_INDEX = 96
F03_TOTAL_CHILD_COMMANDS = 195
F03_RULE_SPAN_COUNT = 113

if F03_POST_STARTUP_CAPTURES != F03_CHARGED_STEPS + 2:
    raise AssertionError('F03 post-startup capture count must equal charged steps + 2 (95)')
if F03_STAGE_MAX_INDEX != F03_CHARGED_STEPS + 3:
    raise AssertionError('F03 max stage index must equal charged steps + 3 (96)')

F03_KAST_STDOUT_MAX_BYTES = 65536
MAX_RULE_SPANS_JSON_BYTES = 1024 * 1024
MAX_K_COMMANDS_JSON_BYTES = 1024 * 1024
_POST_STARTUP_OVERBOUND_SLACK_BYTES = 65536

F03_INPUT_RELPATH = 'input/F03-fuel093-stack064.ui'
F03_INPUT_BYTES = 359
F03_INPUT_SHA256 = 'c1ab51c62fc70ea5693e1d9046445cba6a03eb3222273c045b511913c0f6b91b'
F03_RUN_INPUT_TEXT = (
    'uiRun(uiFunction(uiFun(3, uiParam(0, uiListT(), uiNoParams()), uiUIntT(), '
    'uiMatch(uiVar(0), uiCase(uiPNil(), uiLiteral(uiU(0)), uiCase(uiPCons(1, 2), '
    'uiBinary(uiAdd(), uiVar(1), uiCall(3, uiExpr(uiVar(2), uiNoExprs()))), '
    'uiNoCases())))), uiNoFuns()), uiCall(3, uiExpr(uiLiteral(uiCons(uiU(1), '
    'uiCons(uiU(2), uiCons(uiU(3), uiNil())))), uiNoExprs())), 93, 64)\n'
)
F03_CODE_K = (
    'uiFunction(uiFun(3, uiParam(0, uiListT(), uiNoParams()), uiUIntT(), '
    'uiMatch(uiVar(0), uiCase(uiPNil(), uiLiteral(uiU(0)), uiCase(uiPCons(1, 2), '
    'uiBinary(uiAdd(), uiVar(1), uiCall(3, uiExpr(uiVar(2), uiNoExprs()))), '
    'uiNoCases())))), uiNoFuns())'
)
F03_ENTRY_K = (
    'uiCall(3, uiExpr(uiLiteral(uiCons(uiU(1), uiCons(uiU(2), '
    'uiCons(uiU(3), uiNil())))), uiNoExprs()))'
)

F03_COMPILED_MAIN_K_PATH = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/'
    'k-actual-01/compile-input/models/sherlock/k/main.k'
)
F03_COMPILED_MAIN_K_BYTES = 16249
F03_COMPILED_MAIN_K_SHA256 = (
    'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'
)

F03_DEFINITION_KORE_PATH = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/'
    'k-actual-01/attempt-compile-01/main-kompiled/definition.kore'
)
F03_DEFINITION_KORE_BYTES = 1338414
F03_DEFINITION_KORE_SHA256 = (
    'd795e32885e5651b49ea59eef7b57595a431a6908ef8e9c36901a5ec5e43b6e4'
)

F03_RULE_SPANS_SCHEMA = 'sherlock-k-compiled-source-attribute-inventory/1'
F03_RULE_SPANS_SCOPE = (
    'literal compiled attribute inventory only; '
    'no engine rule identities inferred from states'
)
F03_RULE_SPANS_BYTES = 85954
F03_RULE_SPANS_SHA256 = (
    '9e77ddfe74acaa1e2deccaa1fcde70794494dd3583362b3f9a6f58473d9e941c'
)

F03_INITIALIZER_RECIPE_BYTES = 1613
F03_INITIALIZER_RECIPE_SHA256 = (
    'f9e4d47debf8db513619317aac2115587042e7b284581c48393c3af65bb781cc'
)
F03_INITIALIZER_PREFIX = (
    'LblinitGeneratedTopCell{}(Lbl\'UndsPipe\'-\'-GT-Unds\'{}(inj{SortKConfigVar{}, '
    'SortKItem{}}(\\dv{SortKConfigVar{}}("$PGM")),inj{SortUIRun{}, SortKItem{}}('
)
F03_INITIALIZER_SUFFIX = ')))\n'
F03_INITIALIZER_MAX_BYTES = (
    len(F03_INITIALIZER_PREFIX) + F03_KAST_STDOUT_MAX_BYTES + len(F03_INITIALIZER_SUFFIX)
)

F03_K_COMMANDS_BYTES = 413065
F03_K_COMMANDS_SHA256 = (
    'db62fb14ce8d80e024dfb1d29e1b64dcbe264815690dee6c98619b6e90463110'
)

FORBIDDEN_ORACLE_SHA256S = frozenset({
    '71e780f207c862e6b7061b9b24cb51c3f5243874e256442d46d009e9fc937113',
    '13d00607c8eac53d6d607502e00395b50f6a2f2a9be386f2ba8bbc5bb167a49b',
})
FORBIDDEN_ORACLE_BASENAMES = frozenset({
    'fixture-normal-programs.json',
    'fixture-normal-prefixes.json',
})

EXEC_STAGE_IDS = tuple(f'exec-{idx:03d}' for idx in range(F03_STAGE_MAX_INDEX + 1))
PRINT_STAGE_IDS = tuple(f'print-{idx:03d}' for idx in range(F03_STAGE_MAX_INDEX + 1))
STAGE_IDS = ('kast', *EXEC_STAGE_IDS, *PRINT_STAGE_IDS)

_DIGEST_RE = re.compile(r'[0-9a-f]{64}')
_PRINTABLE_TEXT_RE = re.compile(r'[\x20-\x7e\n]+')
_KAST_STDOUT_LINE_RE = re.compile(r'[\x20-\x7e]{1,65536}\n?')
_SPAN_KEYS = ('start_line', 'start_column', 'end_line', 'end_column')
_ENGINE_VALUED = frozenset({
    '--module',
    '--pattern',
    '--output',
    '--depth',
    '--smt',
    '--log-format',
    '--log-entries',
})
_ENGINE_SWITCHES = frozenset({'--disable-log-timestamps'})
_ENGINE_REQUIRED = {
    '--log-format': 'standard',
    '--disable-log-timestamps': True,
    '--log-entries': 'DebugAppliedRewriteRules,InfoExecDepth',
}


def _compute_self_sha256() -> str:
    try:
        return hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    except OSError:  # pragma: no cover
        return EMPTY_SHA256


ASSEMBLER_SELF_SHA256 = _compute_self_sha256()
ASSEMBLER_ADAPTER_SOURCES = {
    **dict(ADAPTER_SOURCES),
    Path(__file__).name: ASSEMBLER_SELF_SHA256,
}

ASSEMBLER_AUTHOR_PROVENANCE = {
    'requested_role': (
        'Fresh designated security source-author under standing observed '
        'Claude Opus 5.5 (High) quota-outage fallback to Gemini 4 Argon (High)'
    ),
    'returned_model_identity': 'Gemini 4 Argon (High)',
    'fresh_context_non_continuation_of': '7356947d-bd19-4cc5-a58b-643f0d3bf399',
    'author_is_independent_audit_vote': False,
    'execution_authority': 0,
}

RAISE_VS_GAP_CONTRACT = {
    'always_raises_value_error': (
        'Malformed container/stage keys (not exact 195 STAGE_IDS + initializer), '
        'prospective=True or sherlock-normal-* oracle records, forbidden oracle '
        'basenames/SHA256s, unpinned or mismatched source_path / main.k / F03 run_input '
        '/ rule_span_inventory envelope or span rows, malformed receipt key sets / '
        'argv types / non-int exit_status / non-64-hex digest strings, any '
        'k_commands_bytes stage argv mismatch when k_commands_bytes is provided, '
        'any pre-startup (kast, initializer.kore, exec-000, print-000) validation '
        'or pre-startup path-chain (initializer.kore -> exec-000 --pattern -> '
        'print-000 argv[2] & exec-001 --pattern) failure, unfaithful auxiliary '
        'exec.stdout or print.stderr bytes that disagree with their receipt SHA256, '
        'invalid/non-printable/over-bound print configuration text, or on-disk '
        'missing/symlink/outside-root/oversize files.'
    ),
    'delegates_to_decoder_gaps_by_default': (
        'When strict_stage_validation=False (default), structurally well-formed '
        'post-startup stages (exec-001..096 and print-001..096) pass raw bytes '
        '(log/incoming_log and state_kore) and receipt dicts through to '
        'k_capture_observations.decode_capture(..., max_captures=95), so '
        'non-zero exit_status, stderr/output/configuration SHA256 mismatches, '
        'non-empty engine stdout / print stderr reflected in receipt digests, '
        '--depth != 1, unadmitted post-startup tool options, post-startup path-chain '
        'breaks, log size/line/ASCII/control-char/newline defects, state_kore > 1 MiB, '
        'logged On-configuration mismatches, rule-span misses, startup discrepancies, '
        'and terminal replay discrepancies produce comparison_ready=False with '
        'canonical decoder gap tokens.'
    ),
    'strict_stage_validation_mode': (
        'When strict_stage_validation=True, post-startup receipt exit_status != 0, '
        'digest mismatches, non-empty exec.stdout / print.stderr, --depth != 1, '
        'and over-bound/non-ASCII log or state_kore bytes also raise ValueError '
        'directly during assembly.'
    ),
}

TRUST_AND_SECURITY_LIMITS = {
    'adapter_source_binding': (
        'ASSEMBLER_ADAPTER_SOURCES hashes on-disk Python source files at module '
        'import time (TOCTOU); digests are not cryptographically bound to in-memory '
        'code objects or runtime execution.'
    ),
    'filesystem_and_toctou_limits': (
        'load_stage_artifacts_from_actual_dir rejects symlinks on base_dir, stage '
        'subdirectories, and stage files via lstat(), enforces resolve(strict=True) '
        'containment under base_dir, and checks st_size against per-artifact bounds '
        'prior to reading, but POSIX path operations remain subject to local '
        'filesystem TOCTOU races under concurrent mutation and provide no host-wide '
        'sandbox guarantee.'
    ),
    'oracle_isolation_qualification': (
        'FORBIDDEN_ORACLE_BASENAMES, FORBIDDEN_ORACLE_SHA256S, and '
        '_reject_prospective_or_oracle_dict block only unmodified whole oracle '
        'fixture files (fixture-normal-programs.json, fixture-normal-prefixes.json), '
        'sherlock-normal-* dicts, and prospective=True records. Fixed filenames in '
        'load_stage_artifacts_from_actual_dir only hit basename checks if base_dir '
        'itself uses a forbidden name; excerpted or reformatted oracle content '
        'cannot be ruled out by static source guards alone.'
    ),
    'unpinned_flag_qualification': (
        'Setting require_pinned_sha256=False / require_pinned_inventory_sha256=False '
        'or leaving k_commands_bytes=None skips exact SHA256 authentication against '
        'pinned COMPILED-MAIN-RULE-SPANS-01.json or K-COMMANDS.json.'
    ),
    'authenticity_and_gate_status': (
        'Passive offline assembly and comparison_ready=True verify internal '
        'consistency of caller-supplied bytes, receipts, and path chains only; '
        'they do not execute or authenticate child processes, do not prove K/Quint '
        'correspondence, and leave UIREQ-021, C4 OPEN, R2 UNSAT, 60460 HOLD, and '
        'stopped Amendment 10 unchanged.'
    ),
}


def _fail(message: str) -> None:
    raise ValueError(f'K F03 offline assembly error: {message}')


def _sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _validate_source_path(source_path: object) -> str:
    if type(source_path) is not str or source_path != F03_COMPILED_MAIN_K_PATH:
        _fail(
            f'source_path must match pinned compiled main.k path '
            f'{F03_COMPILED_MAIN_K_PATH!r}, got {source_path!r}'
        )
    return source_path


def _require_bytes(
    value: object,
    label: str,
    *,
    allow_empty: bool = False,
    max_bytes: int | None = None,
) -> bytes:
    if isinstance(value, bytes):
        if max_bytes is not None and len(value) > max_bytes:
            _fail(f'{label} exceeds bound ({len(value)} > {max_bytes})')
        raw = value
    elif isinstance(value, str):
        if max_bytes is not None and len(value) > max_bytes:
            _fail(f'{label} exceeds bound ({len(value)} > {max_bytes})')
        try:
            raw = value.encode('ascii')
        except UnicodeEncodeError as exc:
            raise ValueError(f'K F03 offline assembly error: {label} must be ASCII') from exc
        if max_bytes is not None and len(raw) > max_bytes:
            _fail(f'{label} exceeds bound ({len(raw)} > {max_bytes})')
    else:
        _fail(f'{label} must be bytes or ASCII str')
    if not allow_empty and not raw:
        _fail(f'{label} must be non-empty')
    if _sha256_bytes(raw) in FORBIDDEN_ORACLE_SHA256S:
        _fail(f'{label} matches a forbidden independent oracle artifact SHA256')
    return raw


def _require_ascii_text(
    value: object,
    label: str,
    *,
    max_bytes: int,
    require_printable: bool = False,
) -> tuple[str, bytes]:
    raw = _require_bytes(value, label, allow_empty=False, max_bytes=max_bytes)
    if b'\x00' in raw or b'\r' in raw:
        _fail(f'{label} contains NUL or CR bytes')
    try:
        text = raw.decode('ascii')
    except UnicodeDecodeError as exc:
        raise ValueError(f'K F03 offline assembly error: {label} is not ASCII') from exc
    if require_printable and _PRINTABLE_TEXT_RE.fullmatch(text) is None:
        _fail(f'{label} contains non-printable ASCII control characters')
    return text, raw


def _reject_prospective_or_oracle_dict(mapping: object, label: str) -> dict:
    if not isinstance(mapping, dict):
        _fail(f'{label} must be a dict')
    if mapping.get('prospective') is True:
        _fail(f'{label} is a prospective command record, not an actual child receipt')
    fmt = mapping.get('format')
    if isinstance(fmt, str) and fmt.startswith('sherlock-normal-'):
        _fail(f'{label} is an independent oracle artifact ({fmt}), not producer data')
    return mapping


def _engine_options(argv: list[str]) -> dict[str, str | bool] | None:
    """Tokenize admitted kore-exec options; return None if any token is unadmitted."""
    if len(argv) < 2 or argv[1].startswith('-'):
        return None
    options: dict[str, str | bool] = {}
    index = 2
    while index < len(argv):
        flag = argv[index]
        if flag in options:
            return None
        if flag in _ENGINE_SWITCHES:
            options[flag] = True
            index += 1
        elif (
            flag in _ENGINE_VALUED
            and index + 1 < len(argv)
            and not argv[index + 1].startswith('-')
        ):
            options[flag] = argv[index + 1]
            index += 2
        else:
            return None
    return options


def _engine_command_template(argv: list[str]) -> tuple[str, ...]:
    """Normalize kore-exec argv by replacing --pattern, --output, and --depth values."""
    return tuple(
        '<arg>' if idx > 0 and argv[idx - 1] in ('--pattern', '--output', '--depth') else token
        for idx, token in enumerate(argv)
    )


def _validate_receipt_shape(
    receipt: object,
    expected_keys: frozenset[str],
    label: str,
    *,
    expected_argv: list[str] | None = None,
    require_zero_exit: bool = True,
) -> dict:
    rec = _reject_prospective_or_oracle_dict(receipt, label)
    if set(rec) != expected_keys:
        _fail(f'{label} keys must be {sorted(expected_keys)}, got {sorted(rec)}')
    argv = rec.get('argv')
    if not isinstance(argv, list) or not (1 <= len(argv) <= MAX_ARGV):
        _fail(f'{label}.argv must be a non-empty list of at most {MAX_ARGV} strings')
    for idx, arg in enumerate(argv):
        if (
            type(arg) is not str
            or not arg
            or len(arg) > MAX_ARG_CHARS
            or '\x00' in arg
            or '\r' in arg
            or '\n' in arg
        ):
            _fail(f'{label}.argv[{idx}] is invalid')
    if expected_argv is not None and argv != expected_argv:
        _fail(f'{label}.argv does not match pinned K-COMMANDS.json stage argv')
    exit_status = rec.get('exit_status')
    if type(exit_status) is not int:
        _fail(f'{label}.exit_status must be an int, got {exit_status!r}')
    if require_zero_exit and exit_status != 0:
        _fail(f'{label}.exit_status must be int 0, got {exit_status!r}')
    for key in sorted(expected_keys - {'argv', 'exit_status'}):
        digest = rec.get(key)
        if type(digest) is not str or _DIGEST_RE.fullmatch(digest) is None:
            _fail(f'{label}.{key} must be lowercase 64-hex SHA256, got {digest!r}')
    return {key: list(rec[key]) if key == 'argv' else rec[key] for key in sorted(expected_keys)}


def validate_source_k_bytes(
    source_k_bytes: bytes | str,
    *,
    source_path: str = F03_COMPILED_MAIN_K_PATH,
) -> dict:
    """Verify that supplied main.k bytes match the pinned compiled K source."""
    _validate_source_path(source_path)
    raw = _require_bytes(
        source_k_bytes,
        'source_k_bytes',
        allow_empty=False,
        max_bytes=F03_COMPILED_MAIN_K_BYTES,
    )
    sha = _sha256_bytes(raw)
    if len(raw) != F03_COMPILED_MAIN_K_BYTES or sha != F03_COMPILED_MAIN_K_SHA256:
        _fail(
            f'source_k_bytes mismatch: expected ({F03_COMPILED_MAIN_K_BYTES}, '
            f'{F03_COMPILED_MAIN_K_SHA256}), got ({len(raw)}, {sha})'
        )
    return {
        'source_path': source_path,
        'byte_length': len(raw),
        'sha256': sha,
    }


def validate_f03_run_input(run_input_bytes: bytes | str) -> dict:
    """Verify frozen F03-fuel093-stack064.ui bytes and parse via retained_run_input."""
    raw = _require_bytes(
        run_input_bytes,
        'run_input_bytes',
        allow_empty=False,
        max_bytes=F03_INPUT_BYTES,
    )
    sha = _sha256_bytes(raw)
    if len(raw) != F03_INPUT_BYTES or sha != F03_INPUT_SHA256:
        _fail(
            f'run_input_bytes must match frozen {F03_INPUT_RELPATH} '
            f'({F03_INPUT_BYTES}, {F03_INPUT_SHA256}), got ({len(raw)}, {sha})'
        )
    try:
        text = raw.decode('ascii')
    except UnicodeDecodeError as exc:
        raise ValueError('K F03 offline assembly error: run_input_bytes must be ASCII') from exc
    if text != F03_RUN_INPUT_TEXT:
        _fail('run_input_bytes text does not match frozen F03 uiRun(...) input')
    parsed = retained_run_input(text)
    if (
        parsed['code_k'] != F03_CODE_K
        or parsed['entry_k'] != F03_ENTRY_K
        or parsed['initial_fuel'] != F03_INITIAL_FUEL
        or parsed['stack_limit'] != F03_STACK_LIMIT
        or parsed['raw_sha256'] != F03_INPUT_SHA256
    ):
        _fail('retained_run_input output diverged from frozen F03 input contract')
    return parsed


def _validate_rule_span_rows(rules: object, *, source_path: str) -> list[dict]:
    if (
        not isinstance(rules, list)
        or len(rules) != F03_RULE_SPAN_COUNT
        or len(rules) > MAX_RULE_SPANS
    ):
        _fail(
            f'rule_span_inventory.rules must be a list of {F03_RULE_SPAN_COUNT} rows'
        )
    required_row_keys = {'source_path', 'unique_id', *_SPAN_KEYS}
    normalized_rules: list[dict] = []
    seen_spans: set[tuple[int, int, int, int]] = set()
    seen_uniques: set[str] = set()
    for idx, row in enumerate(rules):
        if not isinstance(row, dict) or not (required_row_keys <= set(row)):
            _fail(f'rule_span_inventory.rules[{idx}] missing required span keys')
        if row.get('source_path') != source_path:
            _fail(
                f'rule_span_inventory.rules[{idx}].source_path '
                f'does not match {source_path!r}'
            )
        coords = tuple(row[k] for k in _SPAN_KEYS)
        if any(type(v) is not int or v < 1 for v in coords) or (
            coords[2],
            coords[3],
        ) < (coords[0], coords[1]):
            _fail(
                f'rule_span_inventory.rules[{idx}] requires positive ordered int span coordinates'
            )
        unique_id = row.get('unique_id')
        if type(unique_id) is not str or _DIGEST_RE.fullmatch(unique_id) is None:
            _fail(
                f'rule_span_inventory.rules[{idx}].unique_id must be lowercase 64-hex SHA256'
            )
        if coords in seen_spans or unique_id in seen_uniques:
            _fail(
                f'rule_span_inventory.rules[{idx}] repeats a span or unique_id'
            )
        seen_spans.add(coords)
        seen_uniques.add(unique_id)
        normalized_rules.append(dict(row))
    return normalized_rules


def load_rule_span_inventory(
    raw_inventory: bytes | str | dict,
    *,
    source_path: str = F03_COMPILED_MAIN_K_PATH,
    require_pinned_sha256: bool = True,
) -> tuple[list[dict], dict]:
    """Unwrap COMPILED-MAIN-RULE-SPANS-01.json envelope into the list required by _spans."""
    _validate_source_path(source_path)
    raw_sha256 = None
    raw_length = None
    max_raw_bytes = (
        F03_RULE_SPANS_BYTES if require_pinned_sha256 else MAX_RULE_SPANS_JSON_BYTES
    )
    if isinstance(raw_inventory, (bytes, str)):
        raw_bytes = _require_bytes(
            raw_inventory,
            'rule_span_inventory',
            allow_empty=False,
            max_bytes=max_raw_bytes,
        )
        raw_length = len(raw_bytes)
        raw_sha256 = _sha256_bytes(raw_bytes)
        if require_pinned_sha256 and (
            raw_length != F03_RULE_SPANS_BYTES or raw_sha256 != F03_RULE_SPANS_SHA256
        ):
            _fail(
                f'rule_span_inventory mismatch: expected ({F03_RULE_SPANS_BYTES}, '
                f'{F03_RULE_SPANS_SHA256}), got ({raw_length}, {raw_sha256})'
            )
        try:
            envelope = json.loads(raw_bytes.decode('utf-8'))
        except Exception as exc:
            raise ValueError(
                'K F03 offline assembly error: rule_span_inventory is not valid UTF-8 JSON'
            ) from exc
    elif isinstance(raw_inventory, dict):
        if require_pinned_sha256:
            _fail('raw_inventory must be raw bytes/str when require_pinned_sha256=True')
        envelope = raw_inventory
    else:
        _fail('rule_span_inventory must be raw JSON bytes/str or parsed envelope dict')

    envelope = _reject_prospective_or_oracle_dict(envelope, 'rule_span_inventory')
    expected_envelope_keys = {
        'schema',
        'scope',
        'definition_path',
        'definition_sha256',
        'main_sha256',
        'rules',
    }
    optional_envelope_keys = {'definition_bytes', 'definition_byte_length'}
    if not (
        expected_envelope_keys <= set(envelope)
        <= (expected_envelope_keys | optional_envelope_keys)
    ):
        _fail(
            f'rule_span_inventory envelope keys must be {sorted(expected_envelope_keys)}, '
            f'got {sorted(envelope)}'
        )
    if envelope['schema'] != F03_RULE_SPANS_SCHEMA:
        _fail(f'unexpected rule_span_inventory schema: {envelope["schema"]!r}')
    if envelope['scope'] != F03_RULE_SPANS_SCOPE:
        _fail(f'unexpected rule_span_inventory scope: {envelope["scope"]!r}')
    if envelope['definition_path'] != F03_DEFINITION_KORE_PATH:
        _fail(
            f'rule_span_inventory definition_path mismatch: '
            f'{envelope["definition_path"]!r}'
        )
    if envelope['definition_sha256'] != F03_DEFINITION_KORE_SHA256:
        _fail(
            f'rule_span_inventory definition_sha256 mismatch: '
            f'{envelope["definition_sha256"]!r}'
        )
    for opt_key in optional_envelope_keys & set(envelope):
        if (
            type(envelope[opt_key]) is not int
            or envelope[opt_key] != F03_DEFINITION_KORE_BYTES
        ):
            _fail(
                f'rule_span_inventory {opt_key} must equal {F03_DEFINITION_KORE_BYTES}'
            )
    if envelope['main_sha256'] != F03_COMPILED_MAIN_K_SHA256:
        _fail(f'rule_span_inventory main_sha256 mismatch: {envelope["main_sha256"]!r}')

    normalized_rules = _validate_rule_span_rows(
        envelope['rules'],
        source_path=source_path,
    )

    envelope_meta = {
        'schema': envelope['schema'],
        'scope': envelope['scope'],
        'definition_path': envelope['definition_path'],
        'definition_sha256': envelope['definition_sha256'],
        'pinned_definition_byte_length': F03_DEFINITION_KORE_BYTES,
        'main_sha256': envelope['main_sha256'],
        'rule_count': len(normalized_rules),
        'raw_byte_length': raw_length,
        'raw_sha256': raw_sha256,
        'pinned_sha256_verified': bool(require_pinned_sha256),
    }
    return normalized_rules, envelope_meta


def load_prospective_k_commands(
    k_commands_bytes: bytes | str,
    *,
    require_pinned_sha256: bool = True,
) -> dict[str, list[str]]:
    """Load immutable K-COMMANDS.json strictly for stage-ID and argv cross-checking."""
    max_raw_bytes = (
        F03_K_COMMANDS_BYTES if require_pinned_sha256 else MAX_K_COMMANDS_JSON_BYTES
    )
    raw = _require_bytes(
        k_commands_bytes,
        'k_commands_bytes',
        allow_empty=False,
        max_bytes=max_raw_bytes,
    )
    sha = _sha256_bytes(raw)
    if require_pinned_sha256 and (
        len(raw) != F03_K_COMMANDS_BYTES or sha != F03_K_COMMANDS_SHA256
    ):
        _fail(
            f'k_commands_bytes mismatch: expected ({F03_K_COMMANDS_BYTES}, '
            f'{F03_K_COMMANDS_SHA256}), got ({len(raw)}, {sha})'
        )
    try:
        doc = json.loads(raw.decode('utf-8'))
    except Exception as exc:
        raise ValueError(
            'K F03 offline assembly error: k_commands_bytes is not valid JSON'
        ) from exc
    if not isinstance(doc, dict):
        _fail('k_commands_bytes root must be a JSON object')
    commands = doc.get('commands')
    if not isinstance(commands, list) or len(commands) != F03_TOTAL_CHILD_COMMANDS:
        _fail(f'k_commands_bytes must contain exactly {F03_TOTAL_CHILD_COMMANDS} commands')
    argv_by_id: dict[str, list[str]] = {}
    for idx, entry in enumerate(commands):
        if not isinstance(entry, dict):
            _fail(f'k_commands_bytes.commands[{idx}] must be a dict')
        stage_id = entry.get('id')
        expected_id = STAGE_IDS[idx]
        if stage_id != expected_id:
            _fail(
                f'k_commands_bytes.commands[{idx}].id must be {expected_id!r}, '
                f'got {stage_id!r}'
            )
        argv = entry.get('argv')
        if (
            not isinstance(argv, list)
            or not (1 <= len(argv) <= MAX_ARGV)
            or not all(
                type(x) is str
                and bool(x)
                and len(x) <= MAX_ARG_CHARS
                and '\x00' not in x
                and '\r' not in x
                and '\n' not in x
                for x in argv
            )
        ):
            _fail(
                f'k_commands_bytes.commands[{idx}].argv must be a bounded list of valid strings'
            )
        argv_by_id[stage_id] = list(argv)
    return argv_by_id


def _validate_kast_stdout_kore(value: object) -> tuple[str, bytes]:
    """Enforce <=65536B printable-ASCII single-line ground SortUIRun{} KORE gate."""
    raw = _require_bytes(
        value,
        'kast.stdout',
        allow_empty=False,
        max_bytes=F03_KAST_STDOUT_MAX_BYTES,
    )
    if b'\x00' in raw or b'\r' in raw:
        _fail('kast.stdout contains NUL or CR bytes')
    try:
        text = raw.decode('ascii')
    except UnicodeDecodeError as exc:
        raise ValueError('K F03 offline assembly error: kast.stdout is not ASCII') from exc
    if _KAST_STDOUT_LINE_RE.fullmatch(text) is None:
        _fail(
            'kast.stdout must be a single newline-terminated printable-ASCII line '
            f'of at most {F03_KAST_STDOUT_MAX_BYTES} bytes'
        )
    term = text.removesuffix('\n')
    lbrace = term.find('{')
    lparen = term.find('(')
    if (
        lbrace <= 0
        or lparen <= lbrace + 1
        or term[lparen - 1] != '}'
        or not term.endswith(')')
    ):
        _fail('kast.stdout must be a KORE constructor or domain-value application')
    head = term[:lbrace]
    prefix_head = term[:lparen]
    has_uirun_root = (
        (head.startswith('Lbl') and ('uiRun' in head or 'UIRun' in head))
        or prefix_head == '\\dv{SortUIRun{}}'
        or (head == 'inj' and 'SortUIRun{}' in prefix_head)
    )
    if not has_uirun_root:
        _fail('kast.stdout root KORE constructor must be a SortUIRun{} / uiRun term')

    in_quotes = False
    escape = False
    brace_depth = 0
    paren_depth = 0
    idx = 0
    length = len(term)
    while idx < length:
        ch = term[idx]
        if in_quotes:
            if escape:
                escape = False
            elif ch == '\\':
                escape = True
            elif ch == '"':
                in_quotes = False
            idx += 1
            continue
        if ch == '"':
            if paren_depth < 1:
                _fail('kast.stdout string literal outside root KORE application')
            in_quotes = True
        elif ch == '{':
            brace_depth += 1
        elif ch == '}':
            brace_depth -= 1
            if brace_depth < 0:
                _fail('kast.stdout has unbalanced braces')
        elif ch == '(':
            if brace_depth != 0:
                _fail('kast.stdout has unclosed sort braces before parenthesis')
            paren_depth += 1
        elif ch == ')':
            if brace_depth != 0:
                _fail('kast.stdout has unclosed sort braces inside parentheses')
            paren_depth -= 1
            if paren_depth < 0:
                _fail('kast.stdout has unbalanced parentheses')
            if paren_depth == 0 and idx != length - 1:
                _fail(
                    'kast.stdout must be a single KORE term '
                    '(root application closed before end of line)'
                )
        elif ch == ':':
            _fail('kast.stdout must be a ground KORE term without free variable declarations')
        elif ch == '\\':
            if not term.startswith('\\dv{', idx):
                _fail(
                    'kast.stdout must be a ground KORE term '
                    '(only \\dv{...} backslash construct admitted)'
                )
        idx += 1

    if in_quotes or brace_depth != 0 or paren_depth != 0:
        _fail('kast.stdout has unbalanced quotes, braces, or parentheses')
    return text, raw


def _validate_exec_stage(
    stage_id: str,
    stage: object,
    *,
    expected_depth: str,
    expected_argv: list[str] | None = None,
    strict: bool = True,
) -> dict:
    mapping = _reject_prospective_or_oracle_dict(stage, stage_id)
    receipt = _validate_receipt_shape(
        mapping.get('receipt'),
        ENGINE_RECEIPT_KEYS,
        f'{stage_id}.receipt',
        expected_argv=expected_argv,
        require_zero_exit=strict,
    )
    argv = receipt['argv']
    if strict:
        if '--depth' not in argv:
            _fail(f'{stage_id}.receipt.argv must declare --depth')
        depth_idx = argv.index('--depth')
        if depth_idx + 1 >= len(argv) or argv[depth_idx + 1] != expected_depth:
            _fail(f'{stage_id}.receipt.argv must declare --depth {expected_depth}')

    has_stdout = 'stdout' in mapping and mapping.get('stdout') is not None
    stdout_bytes = _require_bytes(
        mapping.get('stdout', b'') if has_stdout else b'',
        f'{stage_id}.stdout',
        allow_empty=True,
        max_bytes=MAX_LOG_BYTES,
    )
    if strict:
        if stdout_bytes != b'' or receipt['stdout_sha256'] != EMPTY_SHA256:
            _fail(f'{stage_id} engine stdout must be empty ({EMPTY_SHA256})')
    elif has_stdout and _sha256_bytes(stdout_bytes) != receipt['stdout_sha256']:
        _fail(
            f'{stage_id}.stdout SHA256 does not match {stage_id}.receipt.stdout_sha256'
        )

    if strict:
        stderr_text, stderr_bytes = _require_ascii_text(
            mapping.get('stderr'),
            f'{stage_id}.stderr',
            max_bytes=MAX_LOG_BYTES,
        )
        if _sha256_bytes(stderr_bytes) != receipt['stderr_sha256']:
            _fail(f'{stage_id}.stderr SHA256 does not match receipt.stderr_sha256')
        kore_text, kore_bytes = _require_ascii_text(
            mapping.get('state_kore'),
            f'{stage_id}.state_kore',
            max_bytes=MAX_KORE_BYTES,
        )
        if _sha256_bytes(kore_bytes) != receipt['output_sha256']:
            _fail(f'{stage_id}.state_kore SHA256 does not match receipt.output_sha256')
    else:
        stderr_bytes = _require_bytes(
            mapping.get('stderr'),
            f'{stage_id}.stderr',
            allow_empty=True,
            max_bytes=MAX_LOG_BYTES + _POST_STARTUP_OVERBOUND_SLACK_BYTES,
        )
        stderr_text = stderr_bytes.decode('ascii', errors='replace')
        kore_bytes = _require_bytes(
            mapping.get('state_kore'),
            f'{stage_id}.state_kore',
            allow_empty=True,
            max_bytes=MAX_KORE_BYTES + _POST_STARTUP_OVERBOUND_SLACK_BYTES,
        )
        kore_text = kore_bytes.decode('ascii', errors='replace')

    return {
        'stage_id': stage_id,
        'receipt': receipt,
        'stderr_text': stderr_text,
        'stderr_bytes': stderr_bytes,
        'state_kore_text': kore_text,
        'state_kore_bytes': kore_bytes,
    }


def _validate_print_stage(
    stage_id: str,
    stage: object,
    *,
    expected_argv: list[str] | None = None,
    strict: bool = True,
) -> dict:
    mapping = _reject_prospective_or_oracle_dict(stage, stage_id)
    receipt = _validate_receipt_shape(
        mapping.get('receipt'),
        PRINT_RECEIPT_KEYS,
        f'{stage_id}.receipt',
        expected_argv=expected_argv,
        require_zero_exit=strict,
    )
    has_stderr = 'stderr' in mapping and mapping.get('stderr') is not None
    stderr_bytes = _require_bytes(
        mapping.get('stderr', b'') if has_stderr else b'',
        f'{stage_id}.stderr',
        allow_empty=True,
        max_bytes=MAX_LOG_BYTES,
    )
    if strict:
        if stderr_bytes != b'' or receipt['stderr_sha256'] != EMPTY_SHA256:
            _fail(f'{stage_id} print stderr must be empty ({EMPTY_SHA256})')
    elif has_stderr and _sha256_bytes(stderr_bytes) != receipt['stderr_sha256']:
        _fail(
            f'{stage_id}.stderr SHA256 does not match {stage_id}.receipt.stderr_sha256'
        )

    stdout_text, stdout_bytes = _require_ascii_text(
        mapping.get('stdout'),
        f'{stage_id}.stdout',
        max_bytes=MAX_CONFIGURATION_BYTES,
        require_printable=True,
    )
    if strict and _sha256_bytes(stdout_bytes) != receipt['stdout_sha256']:
        _fail(f'{stage_id}.stdout SHA256 does not match receipt.stdout_sha256')

    return {
        'stage_id': stage_id,
        'receipt': receipt,
        'configuration_text': stdout_text,
        'configuration_bytes': stdout_bytes,
    }


def _validate_prestartup_stages(
    stage_artifacts: dict,
    *,
    expected_argv_by_id: dict[str, list[str]] | None = None,
    exec_001_receipt: dict | None = None,
    print_001_receipt: dict | None = None,
) -> dict:
    """Validate kast, initializer.kore, exec-000, and print-000 outside post-startup states."""
    kast_stage = _reject_prospective_or_oracle_dict(stage_artifacts.get('kast'), 'kast')
    kast_receipt = _validate_receipt_shape(
        kast_stage.get('receipt'),
        PRINT_RECEIPT_KEYS,
        'kast.receipt',
        expected_argv=None if expected_argv_by_id is None else expected_argv_by_id['kast'],
        require_zero_exit=True,
    )
    kast_argv = kast_receipt['argv']
    kast_launcher = Path(kast_argv[0]).name
    if kast_launcher == 'java':
        if 'org.kframework.main.Main' not in kast_argv:
            _fail('kast.receipt.argv java launcher must invoke org.kframework.main.Main -kast')
        main_idx = kast_argv.index('org.kframework.main.Main')
        if (
            main_idx < 1
            or main_idx + 1 >= len(kast_argv)
            or kast_argv[main_idx + 1] != '-kast'
        ):
            _fail('kast.receipt.argv java launcher must invoke org.kframework.main.Main -kast')
        jvm_idx = 1
        while jvm_idx < main_idx:
            jvm_tok = kast_argv[jvm_idx]
            if jvm_tok in ('-cp', '-classpath'):
                if (
                    jvm_idx + 1 >= main_idx
                    or not kast_argv[jvm_idx + 1]
                    or kast_argv[jvm_idx + 1].startswith('-')
                ):
                    _fail('kast.receipt.argv java launcher has invalid -cp/-classpath argument')
                jvm_idx += 2
            elif jvm_tok.startswith(('-X', '-D')) or jvm_tok in ('-ea', '-enableassertions'):
                jvm_idx += 1
            else:
                _fail(f'kast.receipt.argv java launcher has unadmitted JVM token {jvm_tok!r}')
        kast_args = kast_argv[main_idx + 2 :]
    elif kast_launcher == 'kast':
        kast_args = kast_argv[1:]
    else:
        _fail(
            'kast.receipt.argv[0] must name the pinned java launcher '
            '(with org.kframework.main.Main -kast) or kast'
        )

    admitted_kast_valued = {
        '--definition',
        '--module',
        '--sort',
        '--input',
        '--output',
    }
    kast_options: dict[str, str] = {}
    kast_positionals: list[str] = []
    arg_idx = 0
    while arg_idx < len(kast_args):
        token = kast_args[arg_idx]
        if token in admitted_kast_valued:
            if (
                token in kast_options
                or arg_idx + 1 >= len(kast_args)
                or not kast_args[arg_idx + 1]
                or kast_args[arg_idx + 1].startswith('-')
            ):
                _fail(f'kast.receipt.argv has duplicate or missing value for {token}')
            kast_options[token] = kast_args[arg_idx + 1]
            arg_idx += 2
        elif token.startswith('-'):
            _fail(f'kast.receipt.argv contains unadmitted option {token!r}')
        else:
            kast_positionals.append(token)
            arg_idx += 1

    if set(kast_options) != admitted_kast_valued:
        missing_kast_opts = sorted(admitted_kast_valued - set(kast_options))
        _fail(f'kast.receipt.argv missing required kast options: {missing_kast_opts}')
    if kast_options['--module'] != 'MORIARTY-UI-SYNTAX':
        _fail('kast.receipt.argv --module must be MORIARTY-UI-SYNTAX')
    if kast_options['--sort'] != 'UIRun':
        _fail('kast.receipt.argv --sort must be UIRun')
    if kast_options['--input'] != 'program':
        _fail('kast.receipt.argv --input must be program')
    if kast_options['--output'] != 'kore':
        _fail('kast.receipt.argv --output must be kore')
    if len(kast_positionals) != 1 or Path(kast_positionals[0]).name != Path(F03_INPUT_RELPATH).name:
        _fail(
            f'kast.receipt.argv must name exactly one {Path(F03_INPUT_RELPATH).name!r} '
            'input file positional argument'
        )

    kast_stderr = _require_bytes(
        kast_stage.get('stderr', b''),
        'kast.stderr',
        allow_empty=True,
        max_bytes=MAX_LOG_BYTES,
    )
    if kast_stderr != b'' or kast_receipt['stderr_sha256'] != EMPTY_SHA256:
        _fail('kast.stderr must be empty')
    kast_stdout_text, kast_stdout_bytes = _validate_kast_stdout_kore(
        kast_stage.get('stdout')
    )
    if _sha256_bytes(kast_stdout_bytes) != kast_receipt['stdout_sha256']:
        _fail('kast.stdout SHA256 does not match kast.receipt.stdout_sha256')

    init_entry = stage_artifacts.get('initializer')
    recipe_bytes_checked = False
    if isinstance(init_entry, dict):
        init_dict = _reject_prospective_or_oracle_dict(init_entry, 'initializer')
        init_raw = init_dict.get('initializer_kore')
        if 'recipe_bytes' in init_dict and init_dict['recipe_bytes'] is not None:
            recipe_raw = _require_bytes(
                init_dict['recipe_bytes'],
                'initializer.recipe_bytes',
                allow_empty=False,
                max_bytes=F03_INITIALIZER_RECIPE_BYTES,
            )
            if (
                len(recipe_raw) != F03_INITIALIZER_RECIPE_BYTES
                or _sha256_bytes(recipe_raw) != F03_INITIALIZER_RECIPE_SHA256
            ):
                _fail('initializer.recipe_bytes does not match pinned INITIALIZER-RECIPE.json')
            recipe_bytes_checked = True
    else:
        init_raw = init_entry
    init_text, init_bytes = _require_ascii_text(
        init_raw,
        'initializer.initializer_kore',
        max_bytes=min(MAX_KORE_BYTES, F03_INITIALIZER_MAX_BYTES),
        require_printable=True,
    )
    expected_init_text = (
        F03_INITIALIZER_PREFIX + kast_stdout_text.removesuffix('\n') + F03_INITIALIZER_SUFFIX
    )
    if init_text != expected_init_text:
        _fail(
            'initializer.kore does not match deterministic INITIALIZER-RECIPE.json '
            'binding of kast/stdout'
        )

    exec_000 = _validate_exec_stage(
        'exec-000',
        stage_artifacts.get('exec-000'),
        expected_depth='0',
        expected_argv=None if expected_argv_by_id is None else expected_argv_by_id['exec-000'],
        strict=True,
    )
    exec_000_options = _engine_options(exec_000['receipt']['argv'])
    if exec_000_options is None:
        _fail('exec-000.receipt.argv is not an admitted kore-exec option set')
    if exec_000_options.get('--depth') != '0':
        _fail('exec-000.receipt.argv must declare --depth 0')
    for flag, expected_val in _ENGINE_REQUIRED.items():
        if exec_000_options.get(flag) != expected_val:
            _fail(f'exec-000.receipt.argv missing required kore-exec option {flag}={expected_val!r}')
    exec_000_pattern = exec_000_options.get('--pattern')
    exec_000_output = exec_000_options.get('--output')
    if (
        not isinstance(exec_000_pattern, str)
        or not isinstance(exec_000_output, str)
        or exec_000_pattern == exec_000_output
    ):
        _fail('exec-000.receipt.argv must declare distinct --pattern and --output paths')
    if Path(exec_000_pattern).name != 'initializer.kore':
        _fail(
            'exec-000.receipt.argv --pattern must reference initializer.kore, '
            f'got {exec_000_pattern!r}'
        )

    print_000 = _validate_print_stage(
        'print-000',
        stage_artifacts.get('print-000'),
        expected_argv=None if expected_argv_by_id is None else expected_argv_by_id['print-000'],
        strict=True,
    )
    p0_argv = print_000['receipt']['argv']
    if (
        len(p0_argv) != 5
        or p0_argv[3:] != ['false', 'false']
        or any(arg.startswith('-') for arg in p0_argv[1:3])
    ):
        _fail('print-000.receipt.argv is not an admitted uncoloured unfiltered kprint command')
    if p0_argv[2] != exec_000_output:
        _fail(
            'pre-startup path chain mismatch: print-000 input path does not match '
            'exec-000 --output path'
        )

    if exec_001_receipt is not None:
        e1_argv = exec_001_receipt['argv']
        e1_options = _engine_options(e1_argv)
        if e1_options is not None:
            if e1_options.get('--pattern') != exec_000_output:
                _fail(
                    'pre-startup path chain mismatch: exec-001 --pattern does not match '
                    'exec-000 --output path'
                )
            if e1_options.get('--output') == exec_000_output:
                _fail('pre-startup path chain mismatch: exec-001 --output duplicates exec-000 --output')
            if _engine_command_template(exec_000['receipt']['argv']) != _engine_command_template(
                e1_argv
            ):
                _fail('pre-startup command template mismatch between exec-000 and exec-001')
        elif '--pattern' in e1_argv:
            pat_idx = e1_argv.index('--pattern')
            if pat_idx + 1 >= len(e1_argv) or e1_argv[pat_idx + 1] != exec_000_output:
                _fail(
                    'pre-startup path chain mismatch: exec-001 --pattern does not match '
                    'exec-000 --output path'
                )

    if print_001_receipt is not None:
        p1_argv = print_001_receipt['argv']
        if (
            len(p1_argv) == 5
            and p1_argv[3:] == ['false', 'false']
            and not any(arg.startswith('-') for arg in p1_argv[1:3])
            and (p0_argv[:2] + p0_argv[3:]) != (p1_argv[:2] + p1_argv[3:])
        ):
            _fail('pre-startup command template mismatch between print-000 and print-001')

    return {
        'scope': 'pre-startup artifacts retained outside typed post-startup states[0..94]',
        'kast': {
            'receipt': kast_receipt,
            'stdout_kore': kast_stdout_text,
            'stdout_sha256': kast_receipt['stdout_sha256'],
            'stdout_byte_length': len(kast_stdout_bytes),
            'stdout_max_bytes_gate': F03_KAST_STDOUT_MAX_BYTES,
        },
        'initializer': {
            'recipe_sha256': F03_INITIALIZER_RECIPE_SHA256,
            'recipe_byte_length': F03_INITIALIZER_RECIPE_BYTES,
            'recipe_bytes_verified': recipe_bytes_checked,
            'initializer_kore': init_text,
            'initializer_sha256': _sha256_bytes(init_bytes),
            'initializer_byte_length': len(init_bytes),
        },
        'exec-000': {
            'receipt': exec_000['receipt'],
            'log': exec_000['stderr_text'],
            'log_sha256': exec_000['receipt']['stderr_sha256'],
            'state_kore_b64': base64.b64encode(exec_000['state_kore_bytes']).decode('ascii'),
            'state_kore_sha256': exec_000['receipt']['output_sha256'],
            'state_kore_byte_length': len(exec_000['state_kore_bytes']),
        },
        'print-000': {
            'receipt': print_000['receipt'],
            'configuration': print_000['configuration_text'],
            'configuration_sha256': print_000['receipt']['stdout_sha256'],
            'configuration_byte_length': len(print_000['configuration_bytes']),
        },
    }


def assemble_f03_capture_payload(
    stage_artifacts: dict,
    *,
    run_input_bytes: bytes | str,
    rule_span_inventory_bytes: bytes | str | dict,
    source_path: str = F03_COMPILED_MAIN_K_PATH,
    require_pinned_inventory_sha256: bool = True,
    k_commands_bytes: bytes | str | None = None,
    strict_stage_validation: bool = False,
) -> dict:
    """Assemble 95 post-startup captures, startup_probe, terminal_probe, and pre_startup record."""
    _validate_source_path(source_path)
    stage_map = _reject_prospective_or_oracle_dict(stage_artifacts, 'stage_artifacts')
    expected_stage_keys = set(STAGE_IDS) | {'initializer'}
    if set(stage_map) != expected_stage_keys:
        missing = sorted(expected_stage_keys - set(stage_map))
        extra = sorted(set(stage_map) - expected_stage_keys)
        _fail(
            f'stage_artifacts must contain exact stage keys (195 child stages + initializer); '
            f'missing={missing[:5]}, extra={extra[:5]}'
        )

    expected_argv_by_id = None
    if k_commands_bytes is not None:
        expected_argv_by_id = load_prospective_k_commands(k_commands_bytes)

    run_input = validate_f03_run_input(run_input_bytes)
    rule_spans, rule_span_envelope = load_rule_span_inventory(
        rule_span_inventory_bytes,
        source_path=source_path,
        require_pinned_sha256=require_pinned_inventory_sha256,
    )

    exec_stages: dict[int, dict] = {}
    print_stages: dict[int, dict] = {}
    for idx in range(1, F03_STAGE_MAX_INDEX + 1):
        exec_id = f'exec-{idx:03d}'
        print_id = f'print-{idx:03d}'
        exec_stages[idx] = _validate_exec_stage(
            exec_id,
            stage_map[exec_id],
            expected_depth='1',
            expected_argv=None if expected_argv_by_id is None else expected_argv_by_id[exec_id],
            strict=strict_stage_validation,
        )
        print_stages[idx] = _validate_print_stage(
            print_id,
            stage_map[print_id],
            expected_argv=None if expected_argv_by_id is None else expected_argv_by_id[print_id],
            strict=strict_stage_validation,
        )

    pre_startup = _validate_prestartup_stages(
        stage_map,
        expected_argv_by_id=expected_argv_by_id,
        exec_001_receipt=exec_stages[1]['receipt'],
        print_001_receipt=print_stages[1]['receipt'],
    )

    startup_probe = {
        'log': exec_stages[1]['stderr_bytes'],
        'state_kore': exec_stages[1]['state_kore_bytes'],
        'configuration': print_stages[1]['configuration_text'],
        'engine_receipt': exec_stages[1]['receipt'],
        'print_receipt': print_stages[1]['receipt'],
    }

    captures: list[dict] = [
        {
            'configuration': print_stages[1]['configuration_text'],
        }
    ]
    for stage_idx in range(2, F03_STAGE_MAX_INDEX):
        captures.append({
            'incoming_log': exec_stages[stage_idx]['stderr_bytes'],
            'state_kore': exec_stages[stage_idx]['state_kore_bytes'],
            'configuration': print_stages[stage_idx]['configuration_text'],
            'engine_receipt': exec_stages[stage_idx]['receipt'],
            'print_receipt': print_stages[stage_idx]['receipt'],
        })

    if len(captures) != F03_POST_STARTUP_CAPTURES or len(captures) - 1 != F03_CHARGED_STEPS + 1:
        _fail(
            f'assembled post-startup captures count must be {F03_POST_STARTUP_CAPTURES}, '
            f'got {len(captures)}'
        )

    terminal_probe = {
        'log': exec_stages[F03_STAGE_MAX_INDEX]['stderr_bytes'],
        'state_kore': exec_stages[F03_STAGE_MAX_INDEX]['state_kore_bytes'],
        'configuration': print_stages[F03_STAGE_MAX_INDEX]['configuration_text'],
        'engine_receipt': exec_stages[F03_STAGE_MAX_INDEX]['receipt'],
        'print_receipt': print_stages[F03_STAGE_MAX_INDEX]['receipt'],
    }

    return {
        'format': ASSEMBLER_FORMAT,
        'case_id': CASE_ID,
        'entry_k': F03_ENTRY_K,
        'source_path': source_path,
        'max_captures': F03_MAX_CAPTURES,
        'charged_steps': F03_CHARGED_STEPS,
        'run_input_text': run_input['raw_text'],
        'run_input': run_input,
        'rule_spans': rule_spans,
        'rule_span_inventory_envelope': rule_span_envelope,
        'pre_startup': pre_startup,
        'startup_probe': startup_probe,
        'captures': captures,
        'terminal_probe': terminal_probe,
    }


def decode_f03_observations(
    stage_artifacts: dict,
    *,
    run_input_bytes: bytes | str,
    rule_span_inventory_bytes: bytes | str | dict,
    source_k_bytes: bytes | str,
    source_path: str = F03_COMPILED_MAIN_K_PATH,
    require_pinned_inventory_sha256: bool = True,
    k_commands_bytes: bytes | str | None = None,
    strict_stage_validation: bool = False,
) -> dict:
    """Assemble F03 stage artifacts and decode via k_capture_observations at explicit 95 bound."""
    source_k_meta = validate_source_k_bytes(source_k_bytes, source_path=source_path)
    assembled = assemble_f03_capture_payload(
        stage_artifacts,
        run_input_bytes=run_input_bytes,
        rule_span_inventory_bytes=rule_span_inventory_bytes,
        source_path=source_path,
        require_pinned_inventory_sha256=require_pinned_inventory_sha256,
        k_commands_bytes=k_commands_bytes,
        strict_stage_validation=strict_stage_validation,
    )
    decoded_run = decode_capture(
        CASE_ID,
        F03_ENTRY_K,
        assembled['captures'],
        source_path=source_path,
        rule_spans=assembled['rule_spans'],
        terminal_probe=assembled['terminal_probe'],
        startup_probe=assembled['startup_probe'],
        run_input=assembled['run_input_text'],
        max_captures=F03_MAX_CAPTURES,
    )
    if decoded_run.get('format') != FORMAT:
        _fail(f'unexpected decode_capture format: {decoded_run.get("format")!r}')
    bundle = observation_bundle([decoded_run], source_sha256=source_k_meta['sha256'])
    if (
        bundle.get('format') != BUNDLE_FORMAT
        or bundle.get('observations', {}).get('producer') != PRODUCER
    ):
        _fail('unexpected observation_bundle format or producer')
    return {
        'bundle': bundle,
        'decoded_run': decoded_run,
        'assembled_provenance': {
            'assembler_format': ASSEMBLER_FORMAT,
            'case_id': CASE_ID,
            'default_decoder_max_captures': MAX_CAPTURES,
            'selected_f03_max_captures': F03_MAX_CAPTURES,
            'expected_charged_steps': F03_CHARGED_STEPS,
            'decoder_formats': {
                'capture_format': FORMAT,
                'bundle_format': BUNDLE_FORMAT,
                'log_format': LOG_FORMAT,
                'producer': PRODUCER,
            },
            'decoder_bounds': {
                'max_captures_default': MAX_CAPTURES,
                'max_captures_f03': F03_MAX_CAPTURES,
                'max_log_bytes': MAX_LOG_BYTES,
                'max_log_lines': MAX_LOG_LINES,
                'max_kore_bytes': MAX_KORE_BYTES,
                'max_configuration_bytes': MAX_CONFIGURATION_BYTES,
                'max_nesting': MAX_NESTING,
                'max_term_bytes': MAX_TERM_BYTES,
                'max_argv': MAX_ARGV,
                'max_arg_chars': MAX_ARG_CHARS,
                'max_rule_spans': MAX_RULE_SPANS,
                'kast_stdout_max_bytes': F03_KAST_STDOUT_MAX_BYTES,
            },
            'source_k': source_k_meta,
            'rule_span_inventory_envelope': assembled['rule_span_inventory_envelope'],
            'k_commands_pinned_verified': k_commands_bytes is not None,
            'strict_stage_validation': bool(strict_stage_validation),
            'pre_startup': assembled['pre_startup'],
            'adapter_sources': dict(ASSEMBLER_ADAPTER_SOURCES),
            'adapter_source_binding': TRUST_AND_SECURITY_LIMITS['adapter_source_binding'],
            'author_provenance': dict(ASSEMBLER_AUTHOR_PROVENANCE),
            'raise_vs_gap_contract': dict(RAISE_VS_GAP_CONTRACT),
            'trust_and_security_limits': dict(TRUST_AND_SECURITY_LIMITS),
        },
    }


def _verify_safe_subdir(base_dir: Path, resolved_root: Path, subdir_name: str) -> Path:
    if subdir_name in FORBIDDEN_ORACLE_BASENAMES:
        _fail(f'stage subdirectory uses forbidden oracle basename {subdir_name!r}')
    subdir = base_dir / subdir_name
    try:
        st = subdir.lstat()
    except OSError as exc:
        raise ValueError(
            f'K F03 offline assembly error: missing required stage directory {subdir}'
        ) from exc
    if stat.S_ISLNK(st.st_mode):
        _fail(f'stage directory must not be a symlink: {subdir}')
    if not stat.S_ISDIR(st.st_mode):
        _fail(f'stage directory path is not a directory: {subdir}')
    try:
        resolved_sub = subdir.resolve(strict=True)
    except OSError as exc:
        raise ValueError(
            f'K F03 offline assembly error: unable to resolve stage directory {subdir}'
        ) from exc
    if resolved_root not in resolved_sub.parents:
        _fail(f'stage directory escapes actual_k_dir root: {subdir}')
    return subdir


def _read_checked_file(
    path: Path,
    label: str,
    *,
    max_bytes: int = MAX_LOG_BYTES,
    allow_empty: bool = True,
    resolved_root: Path | None = None,
) -> bytes:
    if any(part in FORBIDDEN_ORACLE_BASENAMES for part in path.parts):
        _fail(f'{label} references forbidden independent oracle file {path.name!r}')
    try:
        st = path.lstat()
    except OSError as exc:
        raise ValueError(
            f'K F03 offline assembly error: missing required actual artifact file for '
            f'{label}: {path}'
        ) from exc
    if stat.S_ISLNK(st.st_mode):
        _fail(f'{label} must not be a symlink: {path}')
    if not stat.S_ISREG(st.st_mode):
        _fail(f'missing required actual artifact file for {label}: {path}')
    if not allow_empty and st.st_size == 0:
        _fail(f'{label} file must be non-empty: {path}')
    if st.st_size > max_bytes:
        _fail(f'{label} file size {st.st_size} exceeds bound {max_bytes}: {path}')
    if resolved_root is not None:
        try:
            resolved_path = path.resolve(strict=True)
        except OSError as exc:
            raise ValueError(
                f'K F03 offline assembly error: unable to resolve {label}: {path}'
            ) from exc
        if resolved_root not in resolved_path.parents:
            _fail(f'{label} escapes actual_k_dir root: {path}')
    with path.open('rb') as fh:
        raw = fh.read(max_bytes + 1)
    if len(raw) > max_bytes or len(raw) != st.st_size:
        _fail(f'{label} byte length {len(raw)} exceeds bound {max_bytes} or changed during read')
    if not allow_empty and not raw:
        _fail(f'{label} must be non-empty')
    if _sha256_bytes(raw) in FORBIDDEN_ORACLE_SHA256S:
        _fail(f'{label} content matches forbidden independent oracle SHA256')
    return raw


def load_stage_artifacts_from_actual_dir(
    actual_k_dir: str | Path,
    *,
    receipts_by_stage_id: dict[str, dict],
) -> dict:
    """Read materialized ${PLAN}/actual/k files and pair with actual child receipts.

    Never executes children and never synthesizes receipts from file hashes.
    """
    base_dir = Path(actual_k_dir)
    if any(part in FORBIDDEN_ORACLE_BASENAMES for part in base_dir.parts):
        _fail(f'actual_k_dir references forbidden oracle name: {base_dir}')
    try:
        base_st = base_dir.lstat()
    except OSError as exc:
        raise ValueError(
            f'K F03 offline assembly error: actual_k_dir does not exist or is not a '
            f'directory: {base_dir}'
        ) from exc
    if stat.S_ISLNK(base_st.st_mode):
        _fail(f'actual_k_dir must not be a symlink: {base_dir}')
    if not stat.S_ISDIR(base_st.st_mode):
        _fail(f'actual_k_dir does not exist or is not a directory: {base_dir}')
    resolved_root = base_dir.resolve(strict=True)

    receipt_map = _reject_prospective_or_oracle_dict(
        receipts_by_stage_id,
        'receipts_by_stage_id',
    )
    if set(receipt_map) != set(STAGE_IDS):
        missing = sorted(set(STAGE_IDS) - set(receipt_map))
        extra = sorted(set(receipt_map) - set(STAGE_IDS))
        _fail(
            f'receipts_by_stage_id must contain exact 195 stage IDs; '
            f'missing={missing[:5]}, extra={extra[:5]}'
        )

    kast_dir = _verify_safe_subdir(base_dir, resolved_root, 'kast')
    init_dir = _verify_safe_subdir(base_dir, resolved_root, 'initializer')
    stage_artifacts: dict[str, object] = {
        'kast': {
            'receipt': receipt_map['kast'],
            'stdout': _read_checked_file(
                kast_dir / 'stdout',
                'kast.stdout',
                max_bytes=F03_KAST_STDOUT_MAX_BYTES,
                allow_empty=False,
                resolved_root=resolved_root,
            ),
            'stderr': _read_checked_file(
                kast_dir / 'stderr',
                'kast.stderr',
                max_bytes=MAX_LOG_BYTES,
                allow_empty=True,
                resolved_root=resolved_root,
            ),
        },
        'initializer': {
            'initializer_kore': _read_checked_file(
                init_dir / 'initializer.kore',
                'initializer.initializer_kore',
                max_bytes=min(MAX_KORE_BYTES, F03_INITIALIZER_MAX_BYTES),
                allow_empty=False,
                resolved_root=resolved_root,
            ),
        },
    }
    for idx in range(F03_STAGE_MAX_INDEX + 1):
        exec_id = f'exec-{idx:03d}'
        print_id = f'print-{idx:03d}'
        exec_dir = _verify_safe_subdir(base_dir, resolved_root, exec_id)
        print_dir = _verify_safe_subdir(base_dir, resolved_root, print_id)
        stage_artifacts[exec_id] = {
            'receipt': receipt_map[exec_id],
            'stdout': _read_checked_file(
                exec_dir / 'stdout',
                f'{exec_id}.stdout',
                max_bytes=MAX_LOG_BYTES,
                allow_empty=True,
                resolved_root=resolved_root,
            ),
            'stderr': _read_checked_file(
                exec_dir / 'stderr',
                f'{exec_id}.stderr',
                max_bytes=MAX_LOG_BYTES,
                allow_empty=False,
                resolved_root=resolved_root,
            ),
            'state_kore': _read_checked_file(
                exec_dir / 'state.kore',
                f'{exec_id}.state_kore',
                max_bytes=MAX_KORE_BYTES,
                allow_empty=False,
                resolved_root=resolved_root,
            ),
        }
        stage_artifacts[print_id] = {
            'receipt': receipt_map[print_id],
            'stdout': _read_checked_file(
                print_dir / 'stdout',
                f'{print_id}.stdout',
                max_bytes=MAX_CONFIGURATION_BYTES,
                allow_empty=False,
                resolved_root=resolved_root,
            ),
            'stderr': _read_checked_file(
                print_dir / 'stderr',
                f'{print_id}.stderr',
                max_bytes=MAX_LOG_BYTES,
                allow_empty=True,
                resolved_root=resolved_root,
            ),
        }
    return stage_artifacts


__all__ = [
    'ASSEMBLER_ADAPTER_SOURCES',
    'ASSEMBLER_AUTHOR_PROVENANCE',
    'ASSEMBLER_FORMAT',
    'ASSEMBLER_SELF_SHA256',
    'CASE_ID',
    'EXEC_STAGE_IDS',
    'F03_CHARGED_STEPS',
    'F03_CODE_K',
    'F03_COMPILED_MAIN_K_BYTES',
    'F03_COMPILED_MAIN_K_PATH',
    'F03_COMPILED_MAIN_K_SHA256',
    'F03_DEFINITION_KORE_BYTES',
    'F03_DEFINITION_KORE_PATH',
    'F03_DEFINITION_KORE_SHA256',
    'F03_ENTRY_K',
    'F03_INITIALIZER_MAX_BYTES',
    'F03_INITIALIZER_PREFIX',
    'F03_INITIALIZER_RECIPE_BYTES',
    'F03_INITIALIZER_RECIPE_SHA256',
    'F03_INITIALIZER_SUFFIX',
    'F03_INITIAL_FUEL',
    'F03_INPUT_BYTES',
    'F03_INPUT_RELPATH',
    'F03_INPUT_SHA256',
    'F03_KAST_STDOUT_MAX_BYTES',
    'F03_K_COMMANDS_BYTES',
    'F03_K_COMMANDS_SHA256',
    'F03_POST_STARTUP_CAPTURES',
    'F03_RULE_SPANS_BYTES',
    'F03_RULE_SPANS_SCHEMA',
    'F03_RULE_SPANS_SCOPE',
    'F03_RULE_SPANS_SHA256',
    'F03_RULE_SPAN_COUNT',
    'F03_RUN_INPUT_TEXT',
    'F03_STACK_LIMIT',
    'F03_STAGE_MAX_INDEX',
    'F03_TOTAL_CHILD_COMMANDS',
    'FORBIDDEN_ORACLE_BASENAMES',
    'FORBIDDEN_ORACLE_SHA256S',
    'MAX_K_COMMANDS_JSON_BYTES',
    'MAX_RULE_SPANS_JSON_BYTES',
    'PRINT_STAGE_IDS',
    'RAISE_VS_GAP_CONTRACT',
    'STAGE_IDS',
    'TRUST_AND_SECURITY_LIMITS',
    'assemble_f03_capture_payload',
    'decode_f03_observations',
    'load_prospective_k_commands',
    'load_rule_span_inventory',
    'load_stage_artifacts_from_actual_dir',
    'validate_f03_run_input',
    'validate_source_k_bytes',
]

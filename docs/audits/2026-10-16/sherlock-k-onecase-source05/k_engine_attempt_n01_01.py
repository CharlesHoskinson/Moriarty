#!/usr/bin/env python3
"""Sherlock K-A attempt-engine-N01-01: first minimal N01 engine capture. No recompile.

Opus-authored security driver for the ordinary 30-second child class.

  python3 -I -B k_engine_attempt_n01_01.py --preflight-only
      Read-only gates and inventories. No directory, claim, ledger record or
      child. Prints driver_sha256 and identity_sha256 for review.
  python3 -I -B k_engine_attempt_n01_01.py --execute \\
          --approved-driver-sha256 HEX --approved-identity-sha256 HEX
      Exactly one terminal attempt-engine-N01-01. Never retried, never reset.

Fixed stages, strictly in order; each child runs in its own session under the
attempt-compile-01 limits (30 s wall, 3 s sweep, RLIMIT_CORE 0, RLIMIT_FSIZE
512 MiB, umask 077, minimal environment, no host PATH):

  01-kast            pinned JRE, org.kframework.main.Main -kast, frozen N01 input
  02-initializer     driver only: the parsed UIRun is bound as $PGM under the
                     compiled LblinitGeneratedTopCell; no cell or rule is written
  03..07             pinned .kore-exec-wrapped: one depth-0, then four depth-1
  08..12             pinned kprint on each of the five states, observation only

A stage is consumed once its ledger claim is appended. The first problem
(launch error, nonzero exit, signal, timeout, containment breach, missing
product, unexpected stage file, protected-tree change or captured-artifact
change) stops the attempt: no retry, no alternate method. Outputs are kept
raw and hashed; nothing is decoded, matched, labelled or certified.
"""
import argparse
import datetime
import hashlib
import json
import os
import re
import resource
import secrets
import signal
import stat
import subprocess
import sys
import time

WORKSPACE = '/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016'
RUNTIME_PARENT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016'
RUNTIME = RUNTIME_PARENT + '/k-actual-01'
ATTEMPT_NAME = 'attempt-engine-N01-01'
ATTEMPT = RUNTIME + '/' + ATTEMPT_NAME
STAGES_DIR = ATTEMPT + '/stages'
RECEIPT = ATTEMPT + '/receipt.json'
LEDGER = RUNTIME + '/k-engine-ledger.jsonl'

COMPILE_LEDGER = RUNTIME + '/k-attempt-ledger.jsonl'
COMPILE_ATTEMPT = RUNTIME + '/attempt-compile-01'
COMPILE_DRIVER_SHA256 = '05086da9bc2dc5f3f9d89f0f6734a57b57e408f9fef934351b990edc013011a3'
COMPILE_RECEIPT_SHA256 = '946c567c291edf13524a3641d14c7811e855b44073475f6770c9bb0e81399f69'
COMPILED = COMPILE_ATTEMPT + '/main-kompiled'
DEFINITION = COMPILED + '/definition.kore'
SYNTAX_DEFINITION = COMPILED + '/syntaxDefinition.kore'
MACROS = COMPILED + '/macros.kore'
COMPILE_INPUT = RUNTIME + '/compile-input'
CANDIDATE_SNAPSHOT = RUNTIME + '/candidate'
CANDIDATE_RECORD = RUNTIME + '/CANDIDATE-SNAPSHOT-AFTER-COMPILE-03.json'
CANDIDATE_RECORD_SHA256 = '2a7786ee3ac96788aa9ac389a3d4db93af15a7b7c08c9648de2dca08cd7e81c7'
INPUTS = RUNTIME + '/inputs'
INPUT = INPUTS + '/N01-fuel-001.ui'
INPUT_SHA256 = 'e5fe7b26e9565fdafdc97da0fb25fb81c6171afd69e99eaa6dfb38b5409a2e0f'
INPUT_BYTES = 43
INTERPRETER_SHA256 = '52e0a13e60a981d8c4b6478be2ba5176f69da07948a056bf49cf6f077e30cb41'

STORE = '/nix/store/'
K_ROOT = STORE + 'y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406'
JRE_ROOT = STORE + 'kjb3a2ip8ydb3zm604agim5ccjja3vfj-openjdk-headless-minimal-jre-21+35'
JAVA = JRE_ROOT + '/bin/java'
K_JARS = K_ROOT + '/lib/kframework/java'
K_BUILTIN = K_ROOT + '/include/kframework/builtin'
K_WRAPPER = K_ROOT + '/bin/kompile'
K_UNWRAPPED = K_ROOT + '/bin-unwrapped/kompile'
K_LIBRARY = K_ROOT + '/lib/kframework/k'
K_KAST_UNWRAPPED = K_ROOT + '/bin-unwrapped/kast'
K_KORE_PRINT_UNWRAPPED = K_ROOT + '/bin-unwrapped/kore-print'
K_KORE_PRINT_WRAPPER = K_ROOT + '/bin/kore-print'
TOOL_ROOTS = (  # PATH order for the kast JVM, identical to the approved compile.
    ('kore-parser', STORE + '5p6kb76qjrdwjvjmm6l7l2683k41ay1r-kore-38afc81'),
    ('gcc', STORE + '9bv7dcvmfcjnmg5mnqwqlq2wxfn8d7yi-gcc-wrapper-13.2.0'),
    ('flex', STORE + 'z1fkfc5ca0rra7m5zrh8i92mx18w9i00-flex-2.6.4'),
    ('z3', STORE + 'ih51sgk8g57fnkbd5r82ddi8k5vln8cl-z3-4.13.4'),
)
GCC_ROOT = dict(TOOL_ROOTS)['gcc']
FLEX = dict(TOOL_ROOTS)['flex'] + '/bin/flex'
KORE_ROOT = dict(TOOL_ROOTS)['kore-parser']
Z3_ROOT = dict(TOOL_ROOTS)['z3']
KORE_EXEC = STORE + 'qb0m8rd0c3b90dfbz8fwlczd3kc3828z-kore-exec/bin/.kore-exec-wrapped'
KORE_EXEC_SHA256 = '4ebe3cd5b32864352247b954c2d0fb90a7a5730e519a6c4516bb2b5a2c315429'
KORE_EXEC_WRAPPER = KORE_ROOT + '/bin/kore-exec'
LLVM_ROOT = STORE + 'p7hjh21dj4bc1s6kfcdwk45knp5n4cc0-llvm-backend-0.1.140'
KPRINT = LLVM_ROOT + '/bin/kprint'
JAVA_PATH = ':'.join(root + '/bin' for _, root in TOOL_ROOTS)
KORE_EXEC_PATH = Z3_ROOT + '/bin'  # Exactly what the installed kore-exec wrapper prepends.

CHILD_SECONDS = 30
SWEEP_SECONDS = 3
FILE_SIZE_LIMIT = 512 * 1024 * 1024
TOKEN_NAME = 'SHERLOCK_K_ATTEMPT_TOKEN'
STORE_ROOT = re.compile(rb'/nix/store/[0-9a-z]{32}-[0-9A-Za-z+._?=-]+')
EMPTY_SHA256 = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'

MAIN_REL = 'models/sherlock/k/main.k'
MAIN_SHA256 = 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'
MAIN_BYTES = 16249
SOURCE_FILES = (
    (MAIN_REL, MAIN_SHA256, MAIN_BYTES),
    ('models/sherlock/qa/k_capture_observations.py', '2f9d939ea847a61ecaa6c27058b00504c7d613642273767f79fcaa111fc2f8bd', 11336),
    ('models/sherlock/qa/test_k_capture_observations.py', 'e4b71ca2c249396e9ad262f5c43a16885193045da649d36a33069b686d898767', 5866),
    ('models/sherlock/qa/k_observations.py', '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07', 7686),
    ('models/sherlock/qa/k_terms.py', 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca', 5666),
    ('models/sherlock/qa/host_observations.py', '61b8258595a298231679362cb3974ae176b6970e7e4475388252d59da545fcdc', 4912),
    ('models/sherlock/qa/check.py', '7a6d1bbca881f5c2b012278989dac600834d610986694e640da51a40b5a412e0', 36429),
    ('models/sherlock/oracles/normal-programs.json', 'ef87f94f7e5d22a75b1c2e5e61af016b08175beee9a3599e3e69ddf92945d32f', 10202),
    ('models/sherlock/oracles/extra-normal-programs.json', 'dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153', 3721),
    ('models/sherlock/oracles/normal-prefixes.json', 'dca67b553d7483fdf7d9148e7f4bf1a6022b42b8843635f33f23c58a2415d56d', 53119),
    ('models/sherlock/oracles/extra-normal-prefixes.json', 'acf460094b258910bad0499fb8feda0cd85b680023939800ad714d2cb71bee16', 80130),
)
K_SRC = STORE + 'x3ybp2gyrf0vlzzf02s959skd08wn5r6-source'
KORE_PKG = STORE + 'pmjs0mz53bz7p8jzzqx4nba1pfwyhf39-kore'
KORE_SRC = KORE_PKG + '/src/Kore'
LLVM_SRC = STORE + '4k1n4sga1z2pp4iji230aisw4z6al044-source'
PRIMARY_SOURCES = (
    (K_ROOT + '/lib/kframework/checkJava', 'ca205adcbd0aaede282b0ee2251c15fd1dc0806bcd946d8a8971cdedeeed5571', 2257),
    (K_ROOT + '/lib/kframework/setenv', 'fa8d26333cf0d1ef90ba3c6cfc85cd9f63ff23088bfaa61f6a9c6c69c9629688', 891),
    (K_LIBRARY, 'd56ba8f42a320cab88708456ec2721ab71b0d49e8b1bad42d47b497b34fda66c', 1115),
    (K_ROOT + '/bin-unwrapped/krun', '80dea6c5e96142084fe7b4514a22f883db906daef63701b2bcad949d1ff5d743', 21662),
    (K_ROOT + '/bin-unwrapped/kparse', '72df487e5571f6db94beea880ce43ae87d3c1a3db13c5cd308c23ffdaf130484', 4543),
    (K_KORE_PRINT_UNWRAPPED, 'f1956617796796855e288760d88c524074b5da17b4da606dfd0ebff9565a0dc0', 4770),
    (KORE_EXEC_WRAPPER, 'f73768989745e8d108d80906c158fe406942173b104ad2e81b8e8349da6d504d', 395),
    (K_SRC + '/haskell-backend/src/main/java/org/kframework/backend/haskell/HaskellBackend.java',
     'ad044efecdb6679f0cc6ff9929717411f17c5f5f54b33dd3b87e25d938d5b9b4', 2977),
    (K_SRC + '/haskell-backend/src/main/java/org/kframework/backend/haskell/HaskellKompileOptions.java',
     '32534c3b27236d491091555fe053f7f147e9b4822f7faefea31a489f7d41a60a', 986),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/parser/inner/kernel/Scanner.java',
     '32451e633f48e573813e94726e7e9695dffd455aceb9b4665540cd20508fb142', 19129),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/parser/inner/disambiguation/TypeInferencer.java',
     '64ab4933ffeff6790cbadfbaccd31df569e0aaba457475f846dd1665e82042bc', 34641),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/utils/file/JarInfo.java',
     '0dad79688d7c283e013200db422ccf4aee1344fb160c76add78be24ff670f8d4', 3593),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/kast/KastOptions.java',
     'fb4af35d462b18577c47d2fd082fe094c159fd27ebccaf897bb0ddd5b82312fd', 6564),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/kast/KastFrontEnd.java',
     '1eee80a1c242754333f6cad6b83aa1e76f322bbfc3a3d58c627abc81e7621f7c', 10518),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/utils/options/DefinitionLoadingOptions.java',
     '45621d8dc601e1048903e9d568ac56cfe55e177cc0d2a3feba2e70ec6565f2b1', 403),
    (K_SRC + '/k-frontend/src/main/java/org/kframework/unparser/KPrint.java',
     '48ea5a40e2111c49cd7de2934962f170ab66d4b6da6aefe429b8c3a1edadfd76', 14966),
    (KORE_PKG + '/app/exec/Main.hs', 'd25081c4a4929e27ad37130963f16646503eb79f4350833f65dc740fecfab191', 31463),
    (KORE_PKG + '/app/share/GlobalMain.hs', '1084b9ce08eaf67006f87e0891a8ca214485ffa9e9d30e247b1de737d3aa5998', 22981),
    (KORE_PKG + '/src/Options/SMT.hs', '089081433908d5e8d708b15bbf0c69343aa1d447992bee9306d35a05bdf68a43', 7346),
    (KORE_SRC + '/Log.hs', '3b8f1f93a06bb3cb0b351dc76a61f2d4e8c4db8da6a1d0b8079374890224b545', 13206),
    (KORE_SRC + '/Log/DebugAppliedRewriteRules.hs', '6372c03b66df87d812dd3a07d05cf08ed3d1a4006b7ce3eca7da9d38bb4b08aa', 4599),
    (KORE_SRC + '/Log/DebugRewriteTrace.hs', 'a481f4d0d30646162317396af9995576ef8b807481b111df429b8abbd8e0b2ac', 8030),
    (KORE_SRC + '/Log/InfoExecDepth.hs', 'd7f94b700089936425f6d16f82101199176d9cb8df612fc59fa88c5cd0280091', 1428),
    (KORE_SRC + '/Log/WarnDepthLimitExceeded.hs', '097c76c49a00a8a6da763b85580fc16a8808defbb0746b5e4c399a961f896663', 1227),
    (KORE_SRC + '/Log/KoreLogOptions.hs', '58729e884c907d3d5a3c5f375e6be9e76aca45ce219bdd97acd28ef743378af0', 28018),
    (KORE_SRC + '/Attribute/SourceLocation.hs', '7f77fefdcbb3d464c2d35c1a28e1812ca7209a94d8439d8adb0c043da4908983', 2794),
    (LLVM_SRC + '/tools/kprint/main.cpp', '26a79f90483e57d8b2e6684148cbb6f7076de1da02d0af7f716cd61652321ec1', 1237),
    (LLVM_SRC + '/lib/printer/printer.cpp', 'b0c486fba7dd6cc7d55431662ae8fc7f6096bae83b747ae2e3c8f3ed7f4b2d52', 11616),
)
PINNED_FILES = (
    (JAVA, '2a065ca5589bc80168f45166653c6e58d97ce63bbd7407c85eb5fed6622a5740'),
    (JRE_ROOT + '/lib/server/libjvm.so', 'c0e8d7fff0018efb9f157593f0ce1bb646b30f3702e9149ed76bf13f6b56151c'),
    (K_JARS + '/k-frontend-1.0-SNAPSHOT.jar', '6505360d7b472d9802d857452f29722d09424985bb224585c68c37741b65d30a'),
    (Z3_ROOT + '/bin/z3', '2a5132e5f73510ab0ebb61fa8996bfa303f5473a85add71eedbd9c6819ff43ef'),
    (KORE_EXEC, KORE_EXEC_SHA256),
)
COMPILE_TREE = {  # Exact attempt-compile-01 tree: postflight inventory plus its receipt.
    '.': ('dir', '0o700', None, None),
    'compile.stderr': ('file', '0o600', EMPTY_SHA256, 0),
    'compile.stdout': ('file', '0o600', EMPTY_SHA256, 0),
    'home': ('dir', '0o700', None, None),
    'tmp': ('dir', '0o700', None, None),
    'main-kompiled': ('dir', '0o700', None, None),
    'receipt.json': ('file', '0o600', COMPILE_RECEIPT_SHA256, 146103),
    'main-kompiled/README.md': ('file', '0o600', '7202c455a322f60276c475075cc7f76c08065b6ef12b4312181038a9f8cb2120', 2886),
    'main-kompiled/allRules.txt': ('file', '0o600', '69b5134df15c11cd46479edfcae9e4a29f16617f9c1ff1facb0a054b1d10debd', 45466),
    'main-kompiled/backend.txt': ('file', '0o600', 'd11158b53adecdbfce1ad5c64d1adb628145cac84e958b1bf6fed68be5a6b64c', 7),
    'main-kompiled/cache.bin': ('file', '0o600', 'f9e86595d5fad9b57b34eeb8fe4877914e1b5fc6803d5866665baa125dd3029a', 1819101),
    'main-kompiled/compiled.bin': ('file', '0o600', '240b25ee04a90b2acab436a84337790db1bcb753bef133bb79949416f9b7ee18', 2269799),
    'main-kompiled/compiled.txt': ('file', '0o600', 'de7467e2f30d78ba430216cec744f2c27336b54f06cccd7d4fe3ac67cc135d36', 306675),
    'main-kompiled/configVars.sh': ('file', '0o600', '17f3b5a540bd9eec8af0e7d466f45216d790eeec1e2984a9c66ed538bf5ff1fc', 84),
    'main-kompiled/definition.kore': ('file', '0o600', 'd795e32885e5651b49ea59eef7b57595a431a6908ef8e9c36901a5ec5e43b6e4', 1338414),
    'main-kompiled/macros.kore': ('file', '0o600', '3585c084b81281ec26971b2e42c56b9968a802c5d645a81062296350493491cf', 10),
    'main-kompiled/mainModule.txt': ('file', '0o600', '992498a9233bcf21e517704684f47da3e756a39476e69a4bddc966631597b340', 11),
    'main-kompiled/mainSyntaxModule.txt': ('file', '0o600', 'c3da043df6c82c2bae1c1051598d9c67e64b10eda617b7027a28620d63b27697', 18),
    'main-kompiled/parsed.txt': ('file', '0o600', '956e8e6b96f414990c22d2f555353d81280599fc0f48be25283fba968b67aabb', 247593),
    'main-kompiled/scanner': ('file', '0o700', '6a60181404da4c20f8a34deecb8f2d5a48e9226e6f223e5913d241855405a432', 354784),
    'main-kompiled/syntaxDefinition.kore': ('file', '0o600', '39c69c19a5c8763d2aec7847fb603401c1f7f41cf0ef7a5dc4367e4c00d35ba8', 225240),
    'main-kompiled/timestamp': ('file', '0o600', '55800d7983a2a9904d34a306a176e303c5cd2b75dd99b8a761e92140ef9d4510', 7),
}
COMPILE_INPUT_SHAPE = [('.', 'dir'), ('models', 'dir'), ('models/sherlock', 'dir'),
                       ('models/sherlock/k', 'dir'), (MAIN_REL, 'file')]

# Literal declarations observed in the compiled definition; the binding uses nothing else.
UIRUN_OPEN = (b"LbluiRun'LParUndsCommUndsCommUndsCommUndsRParUnds'MORIARTY-UI-SYNTAX'Unds'UIRun"
              b"'Unds'UIFuns'Unds'UIExpr'Unds'Int'Unds'Int{}(")
DEFINITION_MARKERS = (
    b'module MORIARTY-UI',
    b'sort SortKConfigVar{}',
    b'symbol LblinitGeneratedTopCell{}(SortMap{}) : SortGeneratedTopCell{}',
    b"hooked-symbol Lbl'UndsPipe'-'-GT-Unds'{}(SortKItem{}, SortKItem{}) : SortMap{}",
    b'symbol ' + UIRUN_OPEN[:-1] + b'(SortUIFuns{}, SortUIExpr{}, SortInt{}, SortInt{}) : SortUIRun{}',
    b'\\dv{SortKConfigVar{}}("$PGM")',
    b'subsort{SortUIRun{}, SortKItem{}}()',
    b'subsort{SortKConfigVar{}, SortKItem{}}()',
)
INIT_PREFIX = (b"LblinitGeneratedTopCell{}(Lbl'UndsPipe'-'-GT-Unds'{}("
               b'inj{SortKConfigVar{}, SortKItem{}}(\\dv{SortKConfigVar{}}("$PGM")),'
               b'inj{SortUIRun{}, SortKItem{}}(')
INIT_SUFFIX = b')))\n'
PARSED_LIMIT = 64 * 1024
LOG_ENTRIES = 'DebugAppliedRewriteRules,InfoExecDepth'

STAGE_KAST = '01-kast'
STAGE_INIT = '02-initializer'
ENGINE_STAGES = (('03-exec-depth0', 0), ('04-exec-depth1-1', 1), ('05-exec-depth1-2', 1),
                 ('06-exec-depth1-3', 1), ('07-exec-depth1-4', 1))
PRINT_STAGES = ('08-kprint-03', '09-kprint-04', '10-kprint-05', '11-kprint-06', '12-kprint-07')
STAGE_NAMES = [STAGE_KAST, STAGE_INIT] + [name for name, _ in ENGINE_STAGES] + list(PRINT_STAGES)
LIMITS = dict(wall_seconds=CHILD_SECONDS, sweep_seconds=SWEEP_SECONDS, rlimit_core=0, umask='0o077',
              process_group='new session; SIGKILL to group at deadline and after exit; token sweep',
              memory_cpu='tool defaults, as for the admitted native compile children; only the JVM is bounded')
JVM_LIMITS = dict(jvm_heap='64m..512m', jvm_stack='32m', jvm_active_processors=2)
EXPECTATIONS = ('expectation only, never asserted, authored or used to branch: 03 normalises the '
                'initializer; 04, 05 and 06 are each expected to apply one rule (main.k 74, 102, 80); '
                '07 is expected to apply none and repeat 06 exactly. Identities are only what raw logs show.')
SCOPE = ('first minimal N01 capture: kast, driver-only $PGM binding, kore-exec depth 0 then four fixed '
         'depth-1 steps, kprint on five states; no recompile, kparse, krun, llvm-krun, SDK, native, '
         'decoder, adapter, comparison, retry, install or network')
IDENTITY_KEYS = ('interpreter', 'sources', 'primary_sources', 'pinned_files', 'tools', 'elf',
                 'packaging_evidence', 'kprint_linkage', 'native_refs', 'compiler_closure', 'inventory')


def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def environment(stage, token, path):
    env = {'HOME': stage + '/home', 'TMPDIR': stage + '/tmp', 'LANG': 'C', 'LC_ALL': 'C', TOKEN_NAME: token}
    if path is not None:
        env['PATH'] = path
    return env


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, 'rb') as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b''):
            digest.update(chunk)
    return digest.hexdigest()


def file_digest(path):
    try:
        return sha256_file(path) if stat.S_ISREG(os.lstat(path).st_mode) else None
    except OSError:
        return None


def read_exact(path):
    """Read a regular file reached through no symlinked or non-canonical component."""
    if os.path.realpath(path) != path:
        raise ValueError('symlinked or non-canonical path: ' + path)
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode):
            raise ValueError('not a regular file: ' + path)
        chunks = []
        while True:
            chunk = os.read(fd, 1 << 20)
            if not chunk:
                return b''.join(chunks)
            chunks.append(chunk)
    finally:
        os.close(fd)


def read_store(path):
    real = os.path.realpath(path)
    if not real.startswith(STORE):
        raise ValueError('resolves outside /nix/store: ' + path)
    with open(real, 'rb') as handle:
        return handle.read()


def open_exclusive(path, mode=0o600):
    return os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, mode)


def write_exclusive(path, data, mode=0o600):
    fd = open_exclusive(path, mode)
    try:
        view = memoryview(data)
        while view:
            view = view[os.write(fd, view):]
        os.fsync(fd)
    finally:
        os.close(fd)


def append_ledger(record):
    line = (json.dumps(record, sort_keys=True) + '\n').encode()
    fd = os.open(LEDGER, os.O_WRONLY | os.O_APPEND | os.O_CREAT | os.O_NOFOLLOW, 0o600)
    try:
        os.write(fd, line)
        os.fsync(fd)
    finally:
        os.close(fd)


def entry(path):
    info = os.lstat(path)
    row = dict(path=path, mode=oct(stat.S_IMODE(info.st_mode)))
    if stat.S_ISLNK(info.st_mode):
        real = os.path.realpath(path)
        row.update(kind='symlink', target=os.readlink(path), resolved=real)
        if os.path.isfile(real):
            row['resolved_sha256'] = sha256_file(real)
    elif stat.S_ISREG(info.st_mode):
        row.update(kind='file', size=info.st_size, sha256=sha256_file(path))
    elif stat.S_ISDIR(info.st_mode):
        row['kind'] = 'dir'
    else:
        row['kind'] = 'other'
    return row


def tree(root):
    if not os.path.lexists(root):
        return [dict(path=root, kind='absent')]
    rows = [entry(root)]
    if os.path.isdir(root) and not os.path.islink(root):
        for directory, dirnames, filenames in os.walk(root):
            dirnames.sort()
            for name in sorted(dirnames + filenames):
                rows.append(entry(os.path.join(directory, name)))
    return rows


def check_sources(blockers):
    rows, main = {}, None
    for rel, expected, size in SOURCE_FILES:
        try:
            raw = read_exact(WORKSPACE + '/' + rel)
        except (OSError, ValueError) as error:
            blockers.append(f'source-unreadable: {rel}: {error}')
            rows[rel] = None
            continue
        rows[rel] = dict(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
        if rows[rel] != dict(sha256=expected, bytes=size):
            blockers.append('source-hash-mismatch: ' + rel)
        elif rel == MAIN_REL:
            main = raw
    return rows, main


def check_primary(blockers):
    rows = {}
    for path, expected, size in PRIMARY_SOURCES:
        try:
            raw = read_store(path)
        except (OSError, ValueError) as error:
            blockers.append(f'primary-source-unreadable: {path}: {error}')
            rows[path] = None
            continue
        rows[path] = dict(sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))
        if rows[path] != dict(sha256=expected, bytes=size):
            blockers.append('primary-source-mismatch: ' + path)
    return rows


def check_pinned(blockers):
    rows = {}
    for path, expected in PINNED_FILES:
        real = os.path.realpath(path)
        try:
            if not real.startswith(STORE):
                raise ValueError('resolves outside /nix/store')
            rows[path] = sha256_file(real)
        except (OSError, ValueError) as error:
            blockers.append(f'pinned-file-unreadable: {path}: {error}')
            rows[path] = None
            continue
        if rows[path] != expected:
            blockers.append('pinned-file-mismatch: ' + path)
    return rows


def store_executable(path, blockers, label):
    real = os.path.realpath(path)
    if not real.startswith(STORE) or not os.path.isfile(real) or not os.access(real, os.X_OK):
        blockers.append(f'{label}-absent-or-outside-store: {path}')
        return None
    return real


def elf_magic(path, label, blockers):
    try:
        with open(os.path.realpath(path), 'rb') as handle:
            head = handle.read(4)
    except OSError as error:
        blockers.append(f'{label}-unreadable: {error}')
        return None
    if head != b'\x7fELF':
        blockers.append(label + '-not-elf: only the pinned binary itself may be invoked, not a script')
    return head.hex()


def store_refs(path):
    """Advisory: store roots named inside a binary, and whether each exists."""
    found, data = set(), b''
    with open(os.path.realpath(path), 'rb') as handle:
        while True:
            chunk = handle.read(1 << 20)
            data += chunk
            cut = max(0, len(data) - 512) if chunk else len(data)
            for match in STORE_ROOT.finditer(data):
                if match.start() < cut:
                    found.add(match.group().decode())
            if not chunk:
                break
            data = data[cut:]
    return {root: os.path.isdir(root) for root in sorted(found)}


def packaging_evidence(blockers):
    """The installed package, not this proposal, must evidence each direct invocation."""
    checks = (
        ('kast-unwrapped', K_KAST_UNWRAPPED, (b'../lib/kframework/k', b'-kast')),
        ('k-library', K_LIBRARY, (b'org.kframework.main.Main',)),
        ('kore-exec-wrapper', KORE_EXEC_WRAPPER, (KORE_EXEC.encode(), (Z3_ROOT + '/bin').encode())),
        ('kore-print-unwrapped', K_KORE_PRINT_UNWRAPPED,
         (b'kprint "$kompiledDir" "$input_file" "$color" "$filterSubst"',)),
        ('krun-unwrapped', K_ROOT + '/bin-unwrapped/krun',
         (b'--pattern "$final_input"', b'--pattern "$expanded_input_file" --output "$kore_output"')),
    )
    evidence = {}
    for label, path, markers in checks:
        try:
            text = read_store(path)
        except (OSError, ValueError) as error:
            blockers.append(f'{label}-unreadable: {path}: {error}')
            evidence[label] = None
            continue
        evidence[label] = hashlib.sha256(text).hexdigest()
        for marker in markers:
            if marker not in text:
                blockers.append(f'{label}-marker-not-observed: {marker.decode()}')
    return evidence


def kprint_linkage(blockers):
    """K must itself point at this kprint: via its kore-print wrapper or a bin symlink."""
    real = os.path.realpath(KPRINT)
    linkage = dict(kprint_realpath=real, wrapper_references_llvm_root=False, k_bin_links=[])
    try:
        linkage['wrapper_references_llvm_root'] = LLVM_ROOT.encode() in read_store(K_KORE_PRINT_WRAPPER)
    except (OSError, ValueError):
        pass
    for candidate in (K_ROOT + '/bin/kprint', K_ROOT + '/bin-unwrapped/kprint'):
        if os.path.lexists(candidate) and os.path.realpath(candidate) == real:
            linkage['k_bin_links'].append(candidate)
    if not linkage['wrapper_references_llvm_root'] and not linkage['k_bin_links']:
        blockers.append('kprint-linkage-not-evidenced-by-installed-k')
    return linkage


def nix_support_flags(root):
    prefixes, includes = [], []
    support = root + '/nix-support'
    if not os.path.isdir(support):
        return prefixes, includes
    for name in sorted(os.listdir(support)):
        path = os.path.join(support, name)
        if not os.path.isfile(path) or os.path.getsize(path) > 1 << 20:
            continue
        try:
            words = read_store(path).decode('utf-8').split()
        except (UnicodeDecodeError, ValueError):
            continue
        for index, word in enumerate(words):
            following = words[index + 1] if index + 1 < len(words) else ''
            if word == '-B':
                prefixes.append(following)
            elif word.startswith('-B'):
                prefixes.append(word[2:])
            elif word in ('-isystem', '-idirafter', '-I'):
                includes.append(following)
            elif word.startswith('-I'):
                includes.append(word[2:])
    return ([value for value in prefixes if value.startswith(STORE)],
            [value for value in includes if value.startswith(STORE)])


def locate(name, prefixes, executable):
    for prefix in prefixes:
        for path in (prefix + name, prefix.rstrip('/') + '/' + name):
            real = os.path.realpath(path)
            if (real.startswith(STORE) and os.path.isfile(real)
                    and (not executable or os.access(real, os.X_OK))):
                return path
    return None


def compiler_closure(blockers):
    """Only gcc's real search paths count: the minimal PATH and wrapper -B/include flags."""
    prefixes, includes = nix_support_flags(GCC_ROOT)
    path_dirs = [root + '/bin/' for _, root in TOOL_ROOTS]
    found = {}
    for name, where, executable in (
            ('as', path_dirs + prefixes, True),
            ('ld', path_dirs + prefixes, True),
            ('crt1.o', prefixes, False),
            ('stdio.h', includes, False),
            ('fcntl.h', includes, False)):
        found[name] = locate(name, where, executable)
        if found[name] is None:
            blockers.append('gcc-closure-not-established: ' + name)
    roots = sorted({root.decode() for root in STORE_ROOT.findall(read_store(FLEX))})
    found['m4'] = locate('m4', [root + '/bin/' for root in roots], True)
    if found['m4'] is None:
        blockers.append('flex-m4-closure-not-established')
    return found


def compile_attempt_problems():
    if os.path.realpath(COMPILE_ATTEMPT) != COMPILE_ATTEMPT:
        return ['compile-attempt-symlinked-or-non-canonical']
    actual = {}
    for row in tree(COMPILE_ATTEMPT):
        rel = os.path.relpath(row['path'], COMPILE_ATTEMPT)
        actual[rel] = (row['kind'], row.get('mode'), row.get('sha256'), row.get('size'))
    differing = sorted(key for key in set(actual) | set(COMPILE_TREE) if actual.get(key) != COMPILE_TREE.get(key))
    return ['compile-attempt-tree-differs: ' + ', '.join(differing)] if differing else []


def compile_input_problems():
    rows = tree(COMPILE_INPUT)
    shape = [(os.path.relpath(row['path'], COMPILE_INPUT), row['kind']) for row in rows]
    if shape != COMPILE_INPUT_SHAPE or (rows[-1].get('sha256'), rows[-1].get('size')) != (MAIN_SHA256, MAIN_BYTES):
        return ['compile-input-not-exactly-pinned-main.k']
    return []


def candidate_problems():
    try:
        raw = read_exact(CANDIDATE_RECORD)
        record = json.loads(raw)
    except (OSError, ValueError) as error:
        return [f'candidate-record-unreadable: {error}']
    if hashlib.sha256(raw).hexdigest() != CANDIDATE_RECORD_SHA256:
        return ['candidate-record-hash-mismatch']
    if record.get('root') != CANDIDATE_SNAPSHOT or record.get('unchanged') is not True:
        return ['candidate-record-not-the-unchanged-after-compile-snapshot']
    expected = {item['path']: item for item in record.get('entries', [])}
    actual = {}
    for row in tree(CANDIDATE_SNAPSHOT):
        if row['kind'] == 'absent':
            return ['candidate-snapshot-absent']
        rel = os.path.relpath(row['path'], CANDIDATE_SNAPSHOT)
        item = dict(path=rel, mode=stat.S_IMODE(os.lstat(row['path']).st_mode), kind=row['kind'])
        if row['kind'] == 'file':
            item.update(bytes=row['size'], sha256=row['sha256'])
        actual[rel] = item
    return [] if actual == expected else ['candidate-snapshot-differs-from-after-compile-record']


def input_problems():
    try:
        raw = read_exact(INPUT)
    except (OSError, ValueError) as error:
        return [f'input-unreadable: {error}']
    if (hashlib.sha256(raw).hexdigest(), len(raw)) != (INPUT_SHA256, INPUT_BYTES):
        return ['input-hash-mismatch']
    return []


def compiled_content_problems():
    """Content facts behind the decisions: LANG=C, no macro expansion, compiled binding labels."""
    try:
        texts = dict(definition=read_exact(DEFINITION), syntax_definition=read_exact(SYNTAX_DEFINITION),
                     macros=read_exact(MACROS), input=read_exact(INPUT))
    except (OSError, ValueError) as error:
        return [f'compiled-content-unreadable: {error}']
    problems = ['non-ascii-under-LANG=C: ' + label for label, raw in texts.items() if not raw.isascii()]
    if texts['macros'] != b'// macros\n':
        problems.append('macros-not-empty: kore-expand-macros would be required')
    for marker in DEFINITION_MARKERS:
        if marker not in texts['definition']:
            problems.append('definition-marker-absent: ' + marker.decode())
    return problems


def pinned_problems():
    return (compile_attempt_problems() + compile_input_problems() + candidate_problems()
            + input_problems() + compiled_content_problems())


def compile_ledger_problems():
    try:
        records = [json.loads(line) for line in read_exact(COMPILE_LEDGER).decode('utf-8').splitlines()]
    except (OSError, ValueError) as error:
        return [f'compile-ledger-unreadable: {error}']
    shape = [(r.get('event'), r.get('attempt')) if type(r) is dict else None for r in records]
    if shape != [('claimed', 'attempt-compile-01'), ('finished', 'attempt-compile-01')]:
        return ['compile-ledger-not-exactly-one-claim-and-finish']
    claimed, finished = records
    problems = []
    if claimed.get('driver_sha256') != COMPILE_DRIVER_SHA256:
        problems.append('compile-ledger-driver-mismatch')
    if finished.get('outcome') != 'exited' or finished.get('receipt_sha256') != COMPILE_RECEIPT_SHA256:
        problems.append('compile-ledger-finish-mismatch')
    return problems


def engine_ledger_problems():
    if not os.path.lexists(LEDGER):
        return []
    try:
        for line in read_exact(LEDGER).decode('utf-8').splitlines():
            record = json.loads(line)
            if type(record) is not dict:
                raise ValueError('non-object ledger record')
            if record.get('attempt') == ATTEMPT_NAME:
                return ['engine-ledger-records-prior-claim-no-retry']
    except (OSError, ValueError) as error:
        return [f'engine-ledger-unreadable-fail-closed: {error}']
    return []


def runtime_problems():
    if os.path.realpath(RUNTIME_PARENT) != RUNTIME_PARENT or not os.path.isdir(RUNTIME_PARENT):
        return ['runtime-parent-absent-or-symlinked: ' + RUNTIME_PARENT]
    if os.path.realpath(RUNTIME) != RUNTIME or not os.path.isdir(RUNTIME):
        return ['runtime-absent-or-symlinked: ' + RUNTIME]
    problems = []
    prior = sorted(name for name in os.listdir(RUNTIME) if name.startswith('attempt-engine-'))
    if prior:
        problems.append('prior-engine-attempt-present-no-retry: ' + ', '.join(prior))
    return problems + engine_ledger_problems() + compile_ledger_problems()


def protected_snapshot():
    return dict(compile_attempt=tree(COMPILE_ATTEMPT), compile_ledger=tree(COMPILE_LEDGER),
                compile_input=tree(COMPILE_INPUT), candidate=tree(CANDIDATE_SNAPSHOT),
                inputs=tree(INPUTS), runtime_entries=sorted(os.listdir(RUNTIME)))


def inventories(closure):
    roots = [JRE_ROOT, K_JARS, K_BUILTIN, K_WRAPPER, K_UNWRAPPED, K_LIBRARY, GCC_ROOT + '/nix-support']
    roots += [root + '/bin' for _, root in TOOL_ROOTS]
    roots += sorted(path for path in closure.values() if path)
    roots += [K_KAST_UNWRAPPED, K_KORE_PRINT_UNWRAPPED, K_KORE_PRINT_WRAPPER, KORE_EXEC_WRAPPER, KORE_EXEC, KPRINT]
    return {root: tree(os.path.realpath(root)) for root in dict.fromkeys(roots)}


def identity(report):
    body = {key: report.get(key) for key in IDENTITY_KEYS}
    protected = dict(report.get('protected') or {})
    protected.pop('runtime_entries', None)  # Review reports may be added; engine entries are gated.
    body['protected'] = protected
    return hashlib.sha256(json.dumps(body, sort_keys=True).encode()).hexdigest()


def preflight():
    blockers = []
    report = dict(schema='sherlock-k-engine-preflight/1', attempt=ATTEMPT_NAME,
                  driver_sha256=sha256_file(os.path.realpath(__file__)),
                  interpreter=dict(path=sys.executable, version=sys.version,
                                   sha256=sha256_file(os.path.realpath(sys.executable))),
                  blockers=blockers)
    try:
        if sys.platform != 'linux' or not os.path.isdir('/proc/self'):
            blockers.append('linux-procfs-required')
        if os.geteuid() == 0:
            blockers.append('refusing-root: no privilege beyond the ordinary user')
        if not sys.flags.isolated:
            blockers.append('python-isolated-mode-required: run with python3 -I')
        if report['interpreter']['sha256'] != INTERPRETER_SHA256:
            blockers.append('interpreter-differs-from-attempt-compile-01')
        report['sources'], _ = check_sources(blockers)
        report['primary_sources'] = check_primary(blockers)
        report['pinned_files'] = check_pinned(blockers)
        tools = {name: store_executable(root + '/bin/' + name, blockers, name) for name, root in TOOL_ROOTS}
        for label, path in (('java', JAVA), ('kore-exec', KORE_EXEC), ('kprint', KPRINT)):
            tools[label] = store_executable(path, blockers, label)
        report['tools'] = tools
        report['elf'] = {label: elf_magic(path, label, blockers)
                         for label, path in (('kore-exec', KORE_EXEC), ('kprint', KPRINT))}
        report['packaging_evidence'] = packaging_evidence(blockers)
        report['kprint_linkage'] = kprint_linkage(blockers)
        report['native_refs'] = {label: store_refs(path)
                                 for label, path in (('kore-exec', KORE_EXEC), ('kprint', KPRINT))}
        report['compiler_closure'] = closure = compiler_closure(blockers)
        blockers.extend(runtime_problems())
        blockers.extend(pinned_problems())
        report['protected'] = protected_snapshot()
        report['inventory'] = inventories(closure)
    except Exception as error:
        blockers.append(f'preflight-error: {type(error).__name__}: {error}')
    report['identity_sha256'] = identity(report)
    return report


def processes(pgid, token):
    marker = (TOKEN_NAME + '=' + token).encode()
    found = []
    for name in os.listdir('/proc'):
        if not name.isdigit() or int(name) == os.getpid():
            continue
        pid = int(name)
        try:
            with open(f'/proc/{pid}/stat', 'rb') as handle:
                fields = handle.read().rsplit(b')', 1)[1].split()
            state, group = fields[0].decode(), int(fields[2])
        except (OSError, IndexError, ValueError):
            continue
        tagged = False
        try:
            with open(f'/proc/{pid}/environ', 'rb') as handle:
                tagged = marker in handle.read().split(b'\0')
        except OSError:
            pass
        if group == pgid or tagged:
            try:
                with open(f'/proc/{pid}/cmdline', 'rb') as handle:
                    cmdline = [part.decode('utf-8', 'replace') for part in handle.read().split(b'\0') if part]
            except OSError:
                cmdline = []
            found.append(dict(pid=pid, pgid=group, state=state, tagged=tagged,
                              in_group=group == pgid, cmdline=cmdline))
    return found


def kill_group(pgid):
    try:
        os.killpg(pgid, signal.SIGKILL)
    except ProcessLookupError:
        pass


def sweep(pgid, token):
    killed = []
    deadline = time.monotonic() + SWEEP_SECONDS
    while True:
        live = [row for row in processes(pgid, token) if row['state'] != 'Z']
        if not live:
            return killed, []
        killed.extend(live)
        for row in live:
            try:
                os.kill(row['pid'], signal.SIGKILL)
            except ProcessLookupError:
                pass
        if time.monotonic() >= deadline:
            return killed, live
        time.sleep(0.1)


def launch(record, command, env, cwd, token, fsize):
    def limit_child():
        resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
        resource.setrlimit(resource.RLIMIT_FSIZE, (fsize, fsize))
        os.umask(0o077)

    out_fd, err_fd = open_exclusive(cwd + '/stdout'), open_exclusive(cwd + '/stderr')
    proc = None
    try:
        record['started_utc'] = utc()
        started = time.monotonic()
        try:
            proc = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                    stdout=out_fd, stderr=err_fd, close_fds=True,
                                    start_new_session=True, preexec_fn=limit_child)
        except (OSError, subprocess.SubprocessError) as error:
            record.update(outcome='launch-error', launch_error=f'{type(error).__name__}: {error}')
            return
        record['pid'] = proc.pid
        timed_out = False
        try:
            proc.wait(timeout=max(0.0, started + CHILD_SECONDS - time.monotonic()))
        except subprocess.TimeoutExpired:
            timed_out = True
            kill_group(proc.pid)
            proc.wait()
        code = proc.returncode
        record.update(elapsed_seconds=round(time.monotonic() - started, 3), ended_utc=utc(),
                      returncode=code, timed_out=timed_out,
                      outcome='timeout-killed' if timed_out else ('exited' if code >= 0 else 'signaled'))
    finally:
        if proc is not None:
            if proc.returncode is None:
                kill_group(proc.pid)
                proc.wait()
            record['group_members_after_exit'] = processes(proc.pid, token)
            kill_group(proc.pid)
            killed, remaining = sweep(proc.pid, token)
            record.update(swept=killed, remaining=remaining,
                          containment_breach=bool(remaining) or any(not row['in_group'] for row in killed))
        os.close(out_fd)
        os.close(err_fd)


def protected_problems(ctx):
    now = protected_snapshot()
    problems = ['protected-changed: ' + key for key in sorted(now) if now[key] != ctx['baseline'][key]]
    problems += ['captured-artifact-changed: ' + path
                 for path, digest in sorted(ctx['produced'].items()) if file_digest(path) != digest]
    return problems


def stage_problems(record, stage, product_name, ctx):
    problems = []
    if record.get('outcome') != 'exited' or record.get('returncode') != 0:
        problems.append(f"child-not-clean-exit: {record.get('outcome')} returncode={record.get('returncode')}")
    if record.get('containment_breach'):
        problems.append('containment-breach')
    allowed = {'stdout', 'stderr', 'home', 'tmp'} | ({product_name} if product_name else set())
    rows, unexpected, residue = tree(stage), [], []
    for row in rows[1:]:
        rel = os.path.relpath(row['path'], stage)
        if rel.split('/', 1)[0] in ('home', 'tmp') and '/' in rel:
            residue.append(row)
        elif rel not in allowed:
            unexpected.append(row)
    if unexpected:
        problems.append('unexpected-stage-files: ' + ', '.join(row['path'] for row in unexpected))
    artifacts = {name: (entry(stage + '/' + name) if os.path.lexists(stage + '/' + name)
                        else dict(path=stage + '/' + name, kind='absent'))
                 for name in sorted(allowed - {'home', 'tmp'})}
    record.update(stage_tree=rows, scratch_residue=residue, artifacts=artifacts)
    for name, row in artifacts.items():
        if row['kind'] != 'file':
            problems.append(f"artifact-not-regular-file: {row['path']}")
    if artifacts[product_name or 'stdout'].get('size', 0) == 0:
        problems.append('product-absent-or-empty: ' + artifacts[product_name or 'stdout']['path'])
    return problems + protected_problems(ctx)


def child_stage(receipt, ctx, name, build, path, product_name, extra_limits=None):
    stage = STAGES_DIR + '/' + name
    product = stage + '/' + (product_name or 'stdout')
    record = dict(stage=name, claimed_utc=utc(), outcome='driver-error-before-launch', problems=['driver-error'])
    receipt['stages'].append(record)
    append_ledger(dict(event='stage-claimed', attempt=ATTEMPT_NAME, stage=name, utc=record['claimed_utc']))
    try:
        for directory in (stage, stage + '/home', stage + '/tmp'):
            os.mkdir(directory, 0o700)
        token = secrets.token_hex(16)
        command, env = build(stage), environment(stage, token, path)
        record.update(command=list(command), environment=env, cwd=stage, stdin='/dev/null',
                      limits=dict(LIMITS, rlimit_fsize=ctx['fsize'], **(extra_limits or {})))
        problems = protected_problems(ctx)
        if problems:
            record['outcome'] = 'refused-prelaunch'
        else:
            launch(record, command, env, stage, token, ctx['fsize'])
            problems = stage_problems(record, stage, product_name, ctx)
        record['problems'] = problems
    finally:
        append_ledger(dict(event='stage-finished', attempt=ATTEMPT_NAME, stage=name, outcome=record.get('outcome'),
                           returncode=record.get('returncode'), problems=record['problems'], utc=utc()))
    if record['problems']:
        return False, product
    for row in record['artifacts'].values():
        ctx['produced'][row['path']] = row['sha256']
    return True, product


def parsed_problem(raw):
    """Bounded structural gate on the kast output; no expected outcome is consulted."""
    if not 0 < len(raw) <= PARSED_LIMIT:
        return 'parsed-size-out-of-bounds'
    if not raw.isascii() or any((byte < 0x20 and byte not in b'\t\n\r') or byte == 0x7f for byte in raw):
        return 'parsed-not-printable-ascii'
    term = raw.strip()
    if not term.startswith(UIRUN_OPEN):
        return 'parsed-root-is-not-the-compiled-uiRun-symbol'
    depth, index, closed = 0, 0, None
    while index < len(term):
        byte = term[index]
        if byte == 0x22:  # String literal: skip contents, honouring backslash escapes.
            index += 1
            while index < len(term) and term[index] != 0x22:
                index += 2 if term[index] == 0x5c else 1
            if index >= len(term):
                return 'parsed-unterminated-string-literal'
        elif byte == 0x5c and not term.startswith(b'\\dv{', index):
            return 'parsed-matching-logic-connective-other-than-dv'
        elif byte == 0x3a:
            return 'parsed-contains-variable'
        elif byte == 0x28:
            depth += 1
        elif byte == 0x29:
            depth -= 1
            if depth < 0:
                return 'parsed-unbalanced-parentheses'
            if depth == 0 and closed is None:
                closed = index
        index += 1
    if depth or closed != len(term) - 1:
        return 'parsed-not-one-closed-application'
    return None


def initializer_stage(receipt, ctx, parsed_path):
    """Driver-only: bind the parsed UIRun as $PGM; every cell comes from compiled equations."""
    stage = STAGES_DIR + '/' + STAGE_INIT
    target = stage + '/initializer.kore'
    record = dict(stage=STAGE_INIT, child='none: driver-only binding', claimed_utc=utc(), problems=['driver-error'])
    receipt['stages'].append(record)
    append_ledger(dict(event='stage-claimed', attempt=ATTEMPT_NAME, stage=STAGE_INIT, utc=record['claimed_utc']))
    try:
        os.mkdir(stage, 0o700)
        raw = read_exact(parsed_path)
        digest = hashlib.sha256(raw).hexdigest()
        record['parsed'] = dict(path=parsed_path, sha256=digest, bytes=len(raw))
        problems = protected_problems(ctx)
        if digest != ctx['produced'].get(parsed_path):
            problems.append('parsed-output-changed-since-capture')
        problem = parsed_problem(raw)
        if problem:
            problems.append(problem)
        if not problems:
            data = INIT_PREFIX + raw.strip() + INIT_SUFFIX
            write_exclusive(target, data, 0o400)
            if read_exact(target) != data:
                problems.append('initializer-readback-mismatch')
            written = hashlib.sha256(data).hexdigest()
            record['initializer'] = dict(path=target, sha256=written, bytes=len(data), mode='0o400',
                                         binding='LblinitGeneratedTopCell{}(|->($PGM, inj{UIRun,KItem}(parsed)))')
            ctx['produced'][target] = written
        record.update(problems=problems, outcome='refused' if problems else 'written')
    finally:
        append_ledger(dict(event='stage-finished', attempt=ATTEMPT_NAME, stage=STAGE_INIT,
                           outcome=record.get('outcome'), problems=record['problems'], utc=utc()))
    return not record['problems'], target


def kast_command(stage):
    return (JAVA, '-Xms64m', '-Xmx512m', '-Xss32m', '-XX:ActiveProcessorCount=2',
            '-XX:+ExitOnOutOfMemoryError', '-XX:-UsePerfData', '-XX:+DisableAttachMechanism',
            '-XX:ErrorFile=' + stage + '/hs_err_pid%p.log',
            '-Dfile.encoding=UTF-8', '-Djava.awt.headless=true',
            '-Djava.io.tmpdir=' + stage + '/tmp', '-Duser.home=' + stage + '/home',
            '-ea', '-cp', K_JARS + '/*', 'org.kframework.main.Main', '-kast',
            '--definition', COMPILED, '--module', 'MORIARTY-UI-SYNTAX', '--sort', 'UIRun',
            '--input', 'program', '--output', 'kore', INPUT)


def exec_command(pattern, depth):
    def build(stage):
        return (KORE_EXEC, DEFINITION, '--module', 'MORIARTY-UI', '--pattern', pattern,
                '--output', stage + '/state.kore', '--depth', str(depth), '--smt', 'none',
                '--log-format', 'standard', '--disable-log-timestamps', '--log-entries', LOG_ENTRIES)
    return build


def kprint_command(state):
    return lambda stage: (KPRINT, COMPILED, state, 'false', 'false')


def run_stages(receipt, ctx):
    _, hard = resource.getrlimit(resource.RLIMIT_FSIZE)
    ctx['fsize'] = FILE_SIZE_LIMIT if hard == resource.RLIM_INFINITY else min(hard, FILE_SIZE_LIMIT)
    ctx['baseline'] = protected_snapshot()
    problems = pinned_problems()
    if problems:
        receipt.update(outcome='refused-before-first-stage', problems=problems)
        return
    receipt['outcome'] = 'stopped'

    def stop(name):
        receipt.update(stopped_at=name, not_run=STAGE_NAMES[STAGE_NAMES.index(name) + 1:])

    ok, parsed = child_stage(receipt, ctx, STAGE_KAST, kast_command, JAVA_PATH, None, JVM_LIMITS)
    if not ok:
        return stop(STAGE_KAST)
    ok, state = initializer_stage(receipt, ctx, parsed)
    if not ok:
        return stop(STAGE_INIT)
    states = []
    for name, depth in ENGINE_STAGES:
        pattern = state
        ok, state = child_stage(receipt, ctx, name, exec_command(pattern, depth), KORE_EXEC_PATH, 'state.kore')
        if not ok:
            return stop(name)
        receipt['stages'][-1]['raw_fact_state_bytes_equal_pattern_bytes'] = (
            ctx['produced'][state] == ctx['produced'][pattern])
        states.append(state)
    for name, state in zip(PRINT_STAGES, states):
        ok, _ = child_stage(receipt, ctx, name, kprint_command(state), None, None)
        if not ok:
            return stop(name)
    receipt['outcome'] = 'all-stages-completed'


def postflight(receipt, before, ctx):
    try:
        blockers = []
        sources, _ = check_sources(blockers)
        primary = check_primary(blockers)
        pinned = check_pinned(blockers)
        inventory = inventories(before['compiler_closure'])
        changed = [name for name, now in (('sources', sources), ('primary_sources', primary),
                                          ('pinned_files', pinned), ('inventory', inventory)) if before[name] != now]
        baseline = ctx['baseline'] or before['protected']
        now = protected_snapshot()
        protected_changed = sorted(key for key in now if now[key] != baseline[key]
                                   and (ctx['baseline'] is not None or key != 'runtime_entries'))
        receipt['postflight'] = dict(
            blockers=blockers, changed=changed, protected_changed=protected_changed,
            pinned_problems=pinned_problems(),
            captured_artifacts_changed=[path for path, digest in sorted(ctx['produced'].items())
                                        if file_digest(path) != digest],
            attempt_tree=tree(ATTEMPT))
    except Exception as error:
        receipt['postflight'] = dict(error=f'{type(error).__name__}: {error}')


def persist(receipt):
    data = json.dumps(receipt, indent=2, sort_keys=True).encode()
    digest = hashlib.sha256(data).hexdigest()
    try:
        write_exclusive(RECEIPT, data)
    except OSError as error:
        digest += f' (receipt file not written: {error}; receipt follows on stdout)'
        sys.stdout.write(data.decode() + '\n')
    try:
        append_ledger(dict(event='finished', attempt=ATTEMPT_NAME, outcome=receipt.get('outcome'),
                           stopped_at=receipt.get('stopped_at'), receipt_sha256=digest, utc=utc()))
    except OSError as error:
        digest += f' (ledger finish not appended: {error})'
    return digest


def execute(approved_driver, approved_identity):
    own = sha256_file(os.path.realpath(__file__))
    if approved_driver.lower() != own:
        print(json.dumps(dict(status='refused', reason='approved-driver-sha256-mismatch', driver_sha256=own)))
        return 2
    before = preflight()
    if before['blockers']:
        print(json.dumps(dict(status='refused-preflight', blockers=before['blockers']), indent=2))
        return 2
    if approved_identity.lower() != before['identity_sha256']:
        print(json.dumps(dict(status='refused', reason='approved-identity-sha256-mismatch',
                              identity_sha256=before['identity_sha256'])))
        return 2
    os.mkdir(ATTEMPT, 0o700)  # Exclusive terminal claim: never removed, reset or retried.
    receipt = dict(schema='sherlock-k-engine-attempt-receipt/1', attempt=ATTEMPT_NAME, profile='sherlock-k-a/1',
                   driver_sha256=own, identity_sha256=before['identity_sha256'], claimed_utc=utc(),
                   preflight=before, stages=[], outcome='driver-error-before-first-stage',
                   scope=SCOPE, expectations=EXPECTATIONS,
                   certification='none: raw capture only; no decoding, label inference, match or acceptance',
                   retry='forbidden: attempt-engine-N01-01 and each claimed stage are terminal whatever the outcome')
    ctx = dict(produced={}, baseline=None, fsize=None)
    try:
        append_ledger(dict(event='claimed', attempt=ATTEMPT_NAME, driver_sha256=own,
                           identity_sha256=before['identity_sha256'], utc=receipt['claimed_utc'], pid=os.getpid()))
        os.mkdir(STAGES_DIR, 0o700)
        run_stages(receipt, ctx)
    except BaseException as error:
        receipt['driver_error'] = f'{type(error).__name__}: {error}'
    finally:
        postflight(receipt, before, ctx)
        digest = persist(receipt)
    post = receipt.get('postflight', {})
    ok = (receipt.get('outcome') == 'all-stages-completed' and 'driver_error' not in receipt
          and post.get('changed') == [] and not post.get('blockers') and post.get('protected_changed') == []
          and post.get('pinned_problems') == [] and post.get('captured_artifacts_changed') == [])
    print(json.dumps(dict(status='attempt-recorded', outcome=receipt.get('outcome'),
                          stopped_at=receipt.get('stopped_at'), not_run=receipt.get('not_run'),
                          stages=[(s['stage'], s.get('outcome'), s.get('returncode'), s.get('problems'))
                                  for s in receipt['stages']],
                          integrity=dict((k, post.get(k)) for k in ('changed', 'protected_changed',
                                                                     'pinned_problems', 'captured_artifacts_changed')),
                          receipt=RECEIPT, receipt_sha256=digest), indent=2))
    return 0 if ok else 1


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument('--preflight-only', action='store_true')
    mode.add_argument('--execute', action='store_true')
    parser.add_argument('--approved-driver-sha256')
    parser.add_argument('--approved-identity-sha256')
    args = parser.parse_args(argv)
    if args.preflight_only:
        report = preflight()
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0 if not report['blockers'] else 2
    if not args.approved_driver_sha256 or not args.approved_identity_sha256:
        parser.error('--execute requires --approved-driver-sha256 and --approved-identity-sha256')
    return execute(args.approved_driver_sha256, args.approved_identity_sha256)


if __name__ == '__main__':
    sys.exit(main())

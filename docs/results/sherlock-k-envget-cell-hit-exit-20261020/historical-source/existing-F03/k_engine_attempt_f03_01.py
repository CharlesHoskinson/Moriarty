#!/usr/bin/env python3
"""Sherlock K-A attempt-engine-F03-01: one-case F03 raw K engine capture. No recompile.

Derived from the consumed attempt-engine-N01-01 driver (53993B, SHA256
af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2) under
unchanged per-child controls (30 s wall, 3 s sweep, RLIMIT_CORE 0,
RLIMIT_FSIZE 512 MiB, umask 077, minimal environment, no host PATH).
Consumed attempt-engine-N01-01 and its ledger remain untouched.

  python3 -I -B k_engine_attempt_f03_01.py --preflight-only
      Read-only gates and inventories. No directory, claim, ledger record or
      child. Prints driver_sha256 and identity_sha256 for review.
  python3 -I -B k_engine_attempt_f03_01.py --execute \\
          --approved-driver-sha256 HEX --approved-identity-sha256 HEX
      Exactly one terminal attempt-engine-F03-01 in the R251 actual/k root.
      Never retried, never reset.

Fixed stages (195 child processes + 1 driver-only local initializer = 196 stages),
strictly in order:

  kast               pinned JRE, org.kframework.main.Main -kast, frozen F03 input
                     (F03-fuel093-stack064.ui, 359B, fuel 93, stack limit 64)
  initializer        driver only: the parsed UIRun is bound as $PGM under the
                     compiled LblinitGeneratedTopCell; no cell or rule is written
  exec-000           pinned .kore-exec-wrapped depth-0 pre-startup normalization
  exec-001           pinned .kore-exec-wrapped depth-1 startup transition
  exec-002..exec-094 pinned .kore-exec-wrapped depth-1 steps (93 charged transitions)
  exec-095           pinned .kore-exec-wrapped depth-1 step (free publication)
  exec-096           pinned .kore-exec-wrapped depth-1 separate terminal probe
  print-000..096     pinned kprint on each of the 97 states, observation only

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

CONSUMED_N01_ATTEMPT_NAME = 'attempt-engine-N01-01'
CONSUMED_N01_ATTEMPT = RUNTIME + '/' + CONSUMED_N01_ATTEMPT_NAME
CONSUMED_N01_LEDGER = RUNTIME + '/k-engine-ledger.jsonl'
HISTORICAL_N01_DRIVER_SHA256 = 'af9f49b07af4b8d16607fd7cbe01df79ff30302a782084ab502a17dda10e56d2'
HISTORICAL_N01_DRIVER_BYTES = 53993

PLAN_PARENT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017'
PLAN_ROOT = PLAN_PARENT + '/251-f03-paired-model-operation-proposal'
ACTUAL_ROOT = PLAN_ROOT + '/actual'
ATTEMPT_NAME = 'attempt-engine-F03-01'
ATTEMPT = ACTUAL_ROOT + '/k'
STAGES_DIR = ATTEMPT
RECEIPT = ATTEMPT + '/receipt.json'
LEDGER = PLAN_ROOT + '/k-engine-f03-ledger.jsonl'

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

N01_INPUTS = RUNTIME + '/inputs'
N01_INPUT = N01_INPUTS + '/N01-fuel-001.ui'
N01_INPUT_SHA256 = 'e5fe7b26e9565fdafdc97da0fb25fb81c6171afd69e99eaa6dfb38b5409a2e0f'
N01_INPUT_BYTES = 43

INPUTS = PLAN_ROOT + '/input'
INPUT = INPUTS + '/F03-fuel093-stack064.ui'
INPUT_SHA256 = 'c1ab51c62fc70ea5693e1d9046445cba6a03eb3222273c045b511913c0f6b91b'
INPUT_BYTES = 359
INITIAL_FUEL = 93
STACK_LIMIT = 64

PROSPECTIVE_K_COMMANDS = PLAN_ROOT + '/K-COMMANDS.json'
PROSPECTIVE_K_COMMANDS_SHA256 = 'db62fb14ce8d80e024dfb1d29e1b64dcbe264815690dee6c98619b6e90463110'
PROSPECTIVE_K_COMMANDS_BYTES = 413065
PROSPECTIVE_K_COMMANDS_COUNT = 195
INITIALIZER_RECIPE_SHA256 = 'f9e4d47debf8db513619317aac2115587042e7b284581c48393c3af65bb781cc'
INITIALIZER_RECIPE_BYTES = 1613
READ_IMPORT_PINS = PLAN_ROOT + '/READ-IMPORT-PINS.json'
READ_IMPORT_PINS_SHA256 = '2427379d20bb32c0c9a5d9f77b49c5bac39d1bd67fa39b7664c96cc85c2a6289'
READ_IMPORT_PINS_BYTES = 1274580

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
HISTORICAL_N01_SOURCE_FILES = (
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
SOURCE_FILES = (
    (MAIN_REL, MAIN_SHA256, MAIN_BYTES),
    ('models/sherlock/qa/k_capture_observations.py', '2260ebb2d2bbfc6d1cb3f6d2a6715f023637c6d76e401fe105c42616053ab8ac', 41478),
    ('models/sherlock/qa/test_k_capture_observations.py', '4e55872346ad58f9a21e67ef0917bf0cbe550c0a5f3e4f3a1e129b4cff78a091', 90731),
    ('models/sherlock/qa/k_observations.py', '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07', 7686),
    ('models/sherlock/qa/k_terms.py', 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca', 5666),
    ('models/sherlock/qa/host_observations.py', '61b8258595a298231679362cb3974ae176b6970e7e4475388252d59da545fcdc', 4912),
    ('models/sherlock/qa/check.py', '004646fb8a5dcd7666d209360bf932c67920226d50a638500d51a8f95b83b8c5', 81758),
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

STAGE_KAST = 'kast'
STAGE_INIT = 'initializer'
ENGINE_STAGE_MAX_INDEX = 96
ENGINE_STAGES = (('exec-000', 0),) + tuple(
    (f'exec-{idx:03d}', 1) for idx in range(1, ENGINE_STAGE_MAX_INDEX + 1)
)
PRINT_STAGES = tuple(f'print-{idx:03d}' for idx in range(ENGINE_STAGE_MAX_INDEX + 1))
CHILD_STAGE_NAMES = (STAGE_KAST,) + tuple(name for name, _ in ENGINE_STAGES) + PRINT_STAGES
STAGE_NAMES = [STAGE_KAST, STAGE_INIT] + [name for name, _ in ENGINE_STAGES] + list(PRINT_STAGES)

PROSPECTIVE_STAGE_TOKENS = {
    'kast': '99f8a3fe858fa21acf9b7871aad9d20e',
    'exec-000': '3e1a59c24a3fa726bb9c66f2ffe44ba7',
    'exec-001': 'fb2ff9bd36478857dc1318d3f1e947a2',
    'exec-002': 'b34de597fe2155a9aa856c26ce06ad04',
    'exec-003': '6c401b4f59496cba18d6f0b2d5c936a4',
    'exec-004': 'f6b698d35cfa9c29834118993e5bdb6d',
    'exec-005': '400c7482d3f79526d6137ad294fafbb7',
    'exec-006': 'c0b50375111cc2209c5af03aca1c10df',
    'exec-007': '3419d51b3e9c2ac048cc3289f437bdec',
    'exec-008': 'acd3720e4f176d8dec944bc3a5d9d00d',
    'exec-009': '8f98f6e307f716ea379f5d172820db31',
    'exec-010': 'd83af94b4f71f45e647b69402df79108',
    'exec-011': '10ebd99170149a001247642f2a500b44',
    'exec-012': '432bcf3babcc8b1eeaafa94dab6b48b2',
    'exec-013': 'b91abf5c3bbeae6111903d2c357bd9ae',
    'exec-014': '8094ae13a426013307b5fbea889b9b6b',
    'exec-015': '3595abeb94ca70506f3dc6f1fa3a8ffc',
    'exec-016': '3361fe81fc34c52613ed347aa9f66eee',
    'exec-017': '921d204e94cc59a114a709d8ec5acc56',
    'exec-018': '8e27cc7ab14436e4de434f8fcd9cce52',
    'exec-019': '9ef7361b20d85d7adfd93a6c86480bc6',
    'exec-020': '6f188aa0a5e5cf7f63c1cdd61d15efec',
    'exec-021': '9cf9f724796f653bc881fa1ade6e90e9',
    'exec-022': '9094958c51061a65aaad1e5bd4f33c61',
    'exec-023': '692ae63e3e3102323c033a3f8aa801a2',
    'exec-024': '0fc947f1510770c560628d995f66dcd0',
    'exec-025': 'b0e7cf25fece76f06d9015dc1de3e46f',
    'exec-026': '0d6901f8b97a553ef3190250eb9435a0',
    'exec-027': 'efa35213f574b0d43ac08f761fb845e4',
    'exec-028': '3931ca24d8f665b1ddc44c0f498e4614',
    'exec-029': '9e1e41f8d1317d5ad88658547dc90e16',
    'exec-030': 'be940b0119e1fb5516e7cf3ce8491d9c',
    'exec-031': '2d0e86565b87a234261d135c0aad3d5d',
    'exec-032': 'ff0cadbf8154e334b4d43d055aab9f33',
    'exec-033': '0a1b4a1af2316d05b88705755accba2e',
    'exec-034': 'cec922b78f571735cfcd303b150e8ad5',
    'exec-035': 'b04d932b93af5d70b927e5a3296d0397',
    'exec-036': '2fd978cd30bfe6169533d9218cad4397',
    'exec-037': '1fd6b9e286dbf94be630fc10d51f67b7',
    'exec-038': '572cdd0c7171fce0efb7a9b1756cebd5',
    'exec-039': '9793af06618232183981c6d93537505c',
    'exec-040': '0911dbed37750344566ef6dee076f6bf',
    'exec-041': '49c6305b6fe978ffa180678a4a2ee8e6',
    'exec-042': '40101768487f8f3a1998b6af464a10f0',
    'exec-043': 'aa6298037726d694eff594f9f25bee6b',
    'exec-044': '08a91587f262342ada3047d52fcda43e',
    'exec-045': 'aa1cf12b87097801268711a08c030142',
    'exec-046': 'c76ff9302357b8e614ddd376a8b91c54',
    'exec-047': '5117a37c4d5d332cc12660e150c937ca',
    'exec-048': '636e37abdefee5231f1796e363d6819b',
    'exec-049': 'c5d916f662de08e27206384485e42825',
    'exec-050': 'e1b53353ef94d395ea8d1bd5021cdef3',
    'exec-051': 'ea900256db0547327f577961132d7f96',
    'exec-052': 'b525cb798433c7ed584154010a13ebc3',
    'exec-053': '44949b5458e693eda36a3f2e1d901f9d',
    'exec-054': 'fb8d17e1910c989e4fb0e25872d57d4d',
    'exec-055': '7b34f7af58d96ea8220dcf1c34bbf9fa',
    'exec-056': 'dcab699359058f6a4648bac3a91092a9',
    'exec-057': 'e74f74fa679908586fbc20ef2be4f5d2',
    'exec-058': '473ebc8ac151b7ecdaf314543ac60f7d',
    'exec-059': 'ce33f3d1066f57e8c3741a461330141c',
    'exec-060': '8d4ebae8946744ee77271afca26c0c1b',
    'exec-061': '395ec3d9f58928ad5e6eac537b596598',
    'exec-062': 'cfefd9b42efd9d329fd76278453e1b9d',
    'exec-063': '23af250e2ab06b49559ff92422de1413',
    'exec-064': '67c593afcb38949e42e61a5184bc3437',
    'exec-065': 'eecc3ba133b58ba0d914231463df8279',
    'exec-066': '43e053a519dd54300e7f00538e18b687',
    'exec-067': 'a66765d276b35eb3f1edb8bfbeab5f97',
    'exec-068': '48241aaff5331e2ff7fbd3875fb7565c',
    'exec-069': 'dd936c00add067f9aa5974e7ff8389d5',
    'exec-070': 'a72f947d6f863452ea63c816d6770593',
    'exec-071': '38017dc4dd4e4313adafae3f0443e0a0',
    'exec-072': '64f7f9041b7a8f2b76bec326e2c8a86a',
    'exec-073': 'aedd69b6acab327c35efa4fcc1a4809a',
    'exec-074': '547d426af65b508fdf82ee45d28bdc10',
    'exec-075': '0a4423d79ae67fa6cb8f293aa0c1b291',
    'exec-076': '7d0598e6ede27e78bb619294ad2a6ac9',
    'exec-077': 'bb3ebb2151ee939ca5bddbcebc6a3082',
    'exec-078': '2a762183d1cea519f910934b0982b0c0',
    'exec-079': 'd07cbd11030d1dd345fe8310a4e6a420',
    'exec-080': '7add05940ae20a4e06e93a5684f3d9b6',
    'exec-081': '39405d7af1ea78974f190db43d3b1945',
    'exec-082': 'd33499008eafcc54afd71a7c0e341458',
    'exec-083': 'cce0adfc544056b8a5198a6e2d1b6cb3',
    'exec-084': 'b27e6257e07951f79baff1106479cb6c',
    'exec-085': '5be15088d29a7d5c13ebf2a309acf80a',
    'exec-086': '5343002b6216e2735960fe4c260c6165',
    'exec-087': 'c9d8f78cd213096d941490df1039efcb',
    'exec-088': 'c0c64bce53217a88919befa06dc5e31d',
    'exec-089': 'c39228a1d7281b66962346b45c26565e',
    'exec-090': '02d75a8cf61a572c9f85097ca1c4286c',
    'exec-091': '8e3caac6c3bfc87b0972d3bed8a55fb1',
    'exec-092': '9de489f541a23c00f55f85e7956aa0c2',
    'exec-093': '51bae582b7e93b25af0e69bb7e07692d',
    'exec-094': '310ef6ac5f0e0c807dc1fe981732a0a5',
    'exec-095': '59d3d72101dcf65f2b402d211cffe544',
    'exec-096': '03ceb29ea3ec88c7b6159a32d2a264d2',
    'print-000': 'a39defa106312bf091758781881ffd8c',
    'print-001': '9a69aab97847fbda11b83e50698a90a5',
    'print-002': '6e70eac69000e9c822669ad53454e296',
    'print-003': '4eb96461c7510c1c099d7518eb6e50b4',
    'print-004': 'a63f4c5061747d60c41fe624847a13f5',
    'print-005': 'acc935b585ab1a819bb7850a04edab4f',
    'print-006': '43827ae08daaed2c8db9595122bd5db8',
    'print-007': '2274ea4e467d98d65df7bd16a7312cd8',
    'print-008': 'c1159011fadade7e7dd8bddc2e97485a',
    'print-009': '388f9ba2e1d0c9ffb8b633ccc2fad4ac',
    'print-010': 'f5fd9f62dce4a486d5c0e0aac1da9309',
    'print-011': 'eeb8eae45f55f2850871fc5dfe0f8d42',
    'print-012': 'ab4d6f6fabfb9c130648685d74eb818e',
    'print-013': 'd79271521e70d1a48bb8e83c6922e683',
    'print-014': '42b0ed5b22a701d801c3df256db78d77',
    'print-015': 'ce83f0fdb9cab3faab4ea275ecf2b256',
    'print-016': '26bf146917e4e70544cb1d90177acc94',
    'print-017': 'dd30bfc6ae9ac25f3abf1cdeda968bd0',
    'print-018': '8f36cf31ae888a8cbf635da20847169e',
    'print-019': 'fced5231a97a980da8d29e5a06e8c788',
    'print-020': 'b96f4e09907206c0af5aa03af97db68a',
    'print-021': '28c23ceba0c329f099850a9d1ec1bf7a',
    'print-022': 'eaa16c070049435e17adeb8858e2b35f',
    'print-023': '038b67d711285a3981da7ed9beb0cac9',
    'print-024': '8c46fb1a786cc474ed6ebe34da06ad6b',
    'print-025': '97236b6f5871f39cf0a8ff87270cf987',
    'print-026': '48d905f2f3b7a12f39e4a27d25f1c18f',
    'print-027': '551f279458b5c6b5c01bc37ac3e2a43d',
    'print-028': '0bc25e46d989492d754973074f8f08c2',
    'print-029': 'b9f37fbb909d42d52fd1b587b9f5885f',
    'print-030': '74712e11a7876b0b2c28f1e4a8eabc85',
    'print-031': '823a5aebd4ff34ce38bc3a3a54881ec0',
    'print-032': 'ad8005ea54b0922a5bf48fb16fdfb6d8',
    'print-033': 'acfa54b3d48de18bddba2d5a82361214',
    'print-034': 'b3e1168ddeb027901a175dd6fc71b36e',
    'print-035': '47f0133efd934c4c1bcd0b1c3104b4df',
    'print-036': 'da22d38d18955905d9532ec180ce9871',
    'print-037': 'b4d186a10e08ba5512dba392734c9b75',
    'print-038': '2b9709df86daa3c9c37711bc8d0376ad',
    'print-039': '56be1e5375631de21dce04447100afa7',
    'print-040': '8443f0f7201b317c628f17bb8936873f',
    'print-041': '15758f0cfd2510106afebaa295e11012',
    'print-042': '865a9820958fde11dcf701799ce286bc',
    'print-043': '0cd55709f4c609029e6667e6e08f8e67',
    'print-044': '227011a264e3528ecb926e9785918a43',
    'print-045': '1b822632f2bfc8161fa11e05f866ec65',
    'print-046': '01f65c6ab97086dea553af94dfbff0e3',
    'print-047': 'efe6bd6c3f21804bb9c2cf56850cff60',
    'print-048': '539117e98f83c80199027a9272833fb2',
    'print-049': '421a37ef1a4ce2e994d1873832980060',
    'print-050': '731f9e740ade79dc2f63438cb9516b00',
    'print-051': '6ba2ab81c7504914384526c14cc711e0',
    'print-052': '6bd6343b71b1e9b5523bd69d08cff4f4',
    'print-053': '2d9e8c1c4c8f20e7153c8fccc4653d8f',
    'print-054': '0792dadf7e4d1a0a14e26291d4621cd2',
    'print-055': 'e472481c6b016b31cde9aa1a0e6300cb',
    'print-056': '220219880d1a7f17f756f478d0212bce',
    'print-057': '711c8938f7b55faa14ddd0ff934698b4',
    'print-058': '50846a85a6e85618af3c98baba7871e2',
    'print-059': '92b71191f18af99882c4250a574d8df0',
    'print-060': '79b304a052a475c0039c97ec7f666927',
    'print-061': '47a2de65862fcfcd4325479a2ad125d8',
    'print-062': 'acaf8403c8439e9d0a796e62417fc5e1',
    'print-063': 'ac00d2cc8a37f4ad3c54782d4a7d3259',
    'print-064': '1e23202187993079eb57a1e4f7aba711',
    'print-065': '13873605e376647f0c81d953d4c0d88f',
    'print-066': '5d46d73906bbf40ca8a1cce5ceb00cc0',
    'print-067': '4421c6fc6d64f2925fd25f9a5f4e9ea2',
    'print-068': '9a1c7b04b20d92b3c43d92abffb8d5b4',
    'print-069': '6efc1ed92f9d2536472b608d4ee98d64',
    'print-070': '44435ff689857d4b5f84156c529ac2fe',
    'print-071': '46dcbedcae514e546b18d11a2deb3e42',
    'print-072': '63fe1820524e391f3aaa47cdebd66123',
    'print-073': 'e185bd63a10924eb4ba2b281ef4f9851',
    'print-074': '95c60d57e52bd702e88141e0883f8f30',
    'print-075': '573128d2076e40c00724873a6d48409e',
    'print-076': 'd16a912b7c2f512e5943c286d0044710',
    'print-077': 'b65492e01dd0bdb3d6ee2b23002836e0',
    'print-078': '4b5bbd51af775f4610a2792d5bd56431',
    'print-079': 'e395c4be445ab0340dba6abad87cf9dc',
    'print-080': '12e5e7a392b87e0b51a44481c0eac461',
    'print-081': '8ab932bb1eff1d710f2d33e8c7fda1b7',
    'print-082': '3c90b53a17f53735290ce844d316b3d4',
    'print-083': '7fc6d59bf362bdb055b67e5eaa91ba85',
    'print-084': '63619cd6da341fd563e00077cc7d4d46',
    'print-085': '7176f258010caebeb08989e580416d15',
    'print-086': '34edb3df7bc84d18cb6f87d83aa6508c',
    'print-087': '539d021b54acf6dc8177e0d61abd78ff',
    'print-088': '002238199f548c1f42eae13ad0cb962d',
    'print-089': 'e2a6a941da8b9dfffba21a6535fb3cc6',
    'print-090': '12b7773051039763a6afa81a60da77c4',
    'print-091': 'a94af0430749a55a778a512774df7f16',
    'print-092': '792eaa2d783d4747b7d7ef2be7dedaa8',
    'print-093': '338ba3d13d142e596d9e0f5f106cb7bb',
    'print-094': '622ca05375669eaf885fd7c1d4bb1701',
    'print-095': '91d92818b78465923531f543e83afef0',
    'print-096': 'b5c0c728f8f510563aaacb6cc150944d',
}

STAGE_GEOMETRY = dict(
    total_child_commands=PROSPECTIVE_K_COMMANDS_COUNT,
    kast_children=1,
    local_initializer_stages=1,
    kore_exec_children=97,
    kprint_children=97,
    prestartup_raw=('kast', 'initializer', 'exec-000', 'print-000'),
    startup_probe=('exec-001', 'print-001'),
    post_startup_state_prints='print-001..print-095 (95 states)',
    charged_transitions='exec-002..exec-094 (93 charged steps)',
    free_publication_transition=('exec-095', 'print-095'),
    terminal_probe=('exec-096', 'print-096'),
    arithmetic_child_wall_cap_sum_seconds=PROSPECTIVE_K_COMMANDS_COUNT * CHILD_SECONDS,
)
LIMITS = dict(wall_seconds=CHILD_SECONDS, sweep_seconds=SWEEP_SECONDS, rlimit_core=0, umask='0o077',
              process_group='new session; SIGKILL to group at deadline and after exit; token sweep',
              memory_cpu='tool defaults, as for the admitted native compile children; only the JVM is bounded')
JVM_LIMITS = dict(jvm_heap='64m..512m', jvm_stack='32m', jvm_active_processors=2)
EXPECTATIONS = ('expectation only, never asserted, authored or used to branch: exec-000/print-000 '
                'normalises the initializer outside post-startup states; exec-001/print-001 is the '
                'startup transition (rule 74) and initial post-startup state; exec-002..exec-094 are '
                '93 charged transitions; exec-095/print-095 is the 0-charge publication transition '
                '(rule 80) completing 95 post-startup states (print-001..print-095); exec-096/print-096 '
                'is the separate terminal no-rule replay probe. Identities are only what raw logs show.')
SCOPE = ('one-case F03 raw K capture: kast, driver-only $PGM binding, kore-exec depth 0 (exec-000) '
         'then 96 fixed depth-1 steps (exec-001..exec-096), kprint on 97 states (print-000..print-096); '
         'no recompile, kparse, krun, llvm-krun, SDK, native, decoder, assembler, oracle, comparison, '
         'retry, install or network')
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
    problems = []
    try:
        raw = read_exact(INPUT)
        if (hashlib.sha256(raw).hexdigest(), len(raw)) != (INPUT_SHA256, INPUT_BYTES):
            problems.append('input-hash-mismatch')
    except (OSError, ValueError) as error:
        problems.append(f'input-unreadable: {error}')
    try:
        n01_raw = read_exact(N01_INPUT)
        if (hashlib.sha256(n01_raw).hexdigest(), len(n01_raw)) != (N01_INPUT_SHA256, N01_INPUT_BYTES):
            problems.append('n01-input-hash-mismatch')
    except (OSError, ValueError) as error:
        problems.append(f'n01-input-unreadable: {error}')
    return problems


def prospective_plan_problems():
    problems = []
    try:
        pins_raw = read_exact(READ_IMPORT_PINS)
        if (hashlib.sha256(pins_raw).hexdigest(), len(pins_raw)) != (
            READ_IMPORT_PINS_SHA256,
            READ_IMPORT_PINS_BYTES,
        ):
            problems.append('read-import-pins-hash-mismatch')
    except (OSError, ValueError) as error:
        problems.append(f'read-import-pins-unreadable: {error}')
    try:
        cmd_raw = read_exact(PROSPECTIVE_K_COMMANDS)
        if (hashlib.sha256(cmd_raw).hexdigest(), len(cmd_raw)) != (
            PROSPECTIVE_K_COMMANDS_SHA256,
            PROSPECTIVE_K_COMMANDS_BYTES,
        ):
            problems.append('prospective-k-commands-hash-mismatch')
        else:
            doc = json.loads(cmd_raw.decode('utf-8'))
            if type(doc) is not dict or doc.get('commands') != expected_prospective_k_commands():
                problems.append('prospective-k-commands-content-mismatch')
    except (OSError, ValueError, UnicodeDecodeError) as error:
        problems.append(f'prospective-k-commands-unreadable: {error}')
    return problems


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
            + input_problems() + prospective_plan_problems() + compiled_content_problems())


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
    if os.path.realpath(PLAN_PARENT) != PLAN_PARENT or not os.path.isdir(PLAN_PARENT):
        return ['plan-parent-absent-or-symlinked: ' + PLAN_PARENT]
    if os.path.realpath(PLAN_ROOT) != PLAN_ROOT or not os.path.isdir(PLAN_ROOT):
        return ['plan-root-absent-or-symlinked: ' + PLAN_ROOT]
    problems = []
    if os.path.lexists(ACTUAL_ROOT) and (
        os.path.realpath(ACTUAL_ROOT) != ACTUAL_ROOT or not os.path.isdir(ACTUAL_ROOT)
    ):
        problems.append('actual-root-symlinked-or-not-dir: ' + ACTUAL_ROOT)
    if os.path.lexists(ATTEMPT):
        problems.append('prior-f03-attempt-present-no-retry: ' + ATTEMPT)
    unexpected_runtime_attempts = sorted(
        name for name in os.listdir(RUNTIME)
        if name.startswith('attempt-engine-') and name != CONSUMED_N01_ATTEMPT_NAME
    )
    if unexpected_runtime_attempts:
        problems.append('unexpected-runtime-engine-attempt-present: ' + ', '.join(unexpected_runtime_attempts))
    return problems + engine_ledger_problems() + compile_ledger_problems()


def protected_snapshot():
    return dict(
        compile_attempt=tree(COMPILE_ATTEMPT),
        compile_ledger=tree(COMPILE_LEDGER),
        compile_input=tree(COMPILE_INPUT),
        candidate=tree(CANDIDATE_SNAPSHOT),
        inputs=tree(N01_INPUTS),
        consumed_n01_attempt=tree(CONSUMED_N01_ATTEMPT),
        consumed_n01_ledger=tree(CONSUMED_N01_LEDGER),
        f03_inputs=tree(INPUTS),
        prospective_k_commands=tree(PROSPECTIVE_K_COMMANDS),
        read_import_pins=tree(READ_IMPORT_PINS),
        runtime_entries=sorted(os.listdir(RUNTIME)),
    )


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
    child_receipt = dict(
        argv=list(command),
        exit_status=record.get('returncode'),
        stdout_sha256=record['artifacts']['stdout']['sha256'],
        stderr_sha256=record['artifacts']['stderr']['sha256'],
    )
    if product_name is not None and product_name in record['artifacts']:
        child_receipt['output_sha256'] = record['artifacts'][product_name]['sha256']
    record['prospective_plan_token'] = PROSPECTIVE_STAGE_TOKENS.get(name)
    record['child_receipt'] = child_receipt
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


def expected_prospective_k_commands():
    commands = []
    kast_stage = STAGES_DIR + '/' + STAGE_KAST
    commands.append(dict(
        id=STAGE_KAST,
        kind='K parser only, cached syntax/compiled definition; no K compilation',
        argv=list(kast_command(kast_stage)),
        cwd=kast_stage,
        environment=environment(kast_stage, PROSPECTIVE_STAGE_TOKENS[STAGE_KAST], JAVA_PATH),
        inherit_environment=False,
        stdin='DEVNULL',
        limits=dict(LIMITS, rlimit_fsize=FILE_SIZE_LIMIT),
        stdout=kast_stage + '/stdout',
        stderr=kast_stage + '/stderr',
        prospective=True,
    ))
    prev_state = STAGES_DIR + '/' + STAGE_INIT + '/initializer.kore'
    exec_outputs = []
    for name, depth in ENGINE_STAGES:
        stage = STAGES_DIR + '/' + name
        commands.append(dict(
            id=name,
            kind='raw K engine capture',
            argv=list(exec_command(prev_state, depth)(stage)),
            cwd=stage,
            environment=environment(stage, PROSPECTIVE_STAGE_TOKENS[name], KORE_EXEC_PATH),
            inherit_environment=False,
            stdin='DEVNULL',
            limits=dict(LIMITS, rlimit_fsize=FILE_SIZE_LIMIT),
            stdout=stage + '/stdout',
            stderr=stage + '/stderr',
            prospective=True,
        ))
        prev_state = stage + '/state.kore'
        exec_outputs.append(prev_state)
    for name, state_path in zip(PRINT_STAGES, exec_outputs):
        stage = STAGES_DIR + '/' + name
        commands.append(dict(
            id=name,
            kind='raw K pretty printer',
            argv=list(kprint_command(state_path)(stage)),
            cwd=stage,
            environment=environment(stage, PROSPECTIVE_STAGE_TOKENS[name], None),
            inherit_environment=False,
            stdin='DEVNULL',
            limits=dict(LIMITS, rlimit_fsize=FILE_SIZE_LIMIT),
            stdout=stage + '/stdout',
            stderr=stage + '/stderr',
            prospective=True,
        ))
    return commands


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
    if not os.path.lexists(ACTUAL_ROOT):
        os.mkdir(ACTUAL_ROOT, 0o700)
    elif os.path.realpath(ACTUAL_ROOT) != ACTUAL_ROOT or not os.path.isdir(ACTUAL_ROOT):
        print(json.dumps(dict(status='refused', reason='actual-root-symlinked-or-not-dir', actual_root=ACTUAL_ROOT)))
        return 2
    os.mkdir(ATTEMPT, 0o700)  # Exclusive terminal claim: never removed, reset or retried.
    receipt = dict(schema='sherlock-k-engine-attempt-receipt/1', attempt=ATTEMPT_NAME, case_id='F03',
                   profile='sherlock-k-a/1', driver_sha256=own, identity_sha256=before['identity_sha256'],
                   claimed_utc=utc(), preflight=before, stage_geometry=STAGE_GEOMETRY, stages=[],
                   outcome='driver-error-before-first-stage', scope=SCOPE, expectations=EXPECTATIONS,
                   certification='none: raw capture only; no decoding, label inference, match or acceptance',
                   retry='forbidden: attempt-engine-F03-01 and each claimed stage are terminal whatever the outcome')
    ctx = dict(produced={}, baseline=None, fsize=None)
    try:
        append_ledger(dict(event='claimed', attempt=ATTEMPT_NAME, driver_sha256=own,
                           identity_sha256=before['identity_sha256'], utc=receipt['claimed_utc'], pid=os.getpid()))
        if STAGES_DIR != ATTEMPT:
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

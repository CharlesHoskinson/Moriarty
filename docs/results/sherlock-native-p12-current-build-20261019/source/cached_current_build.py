#!/usr/bin/env python3
"""Bounded cached-Phase-F preparation, source_home reconstitution, and post-r3 accounting."""

import copy
import hashlib
import json
import os
import re
import shutil
import signal
import stat
import sys
import tarfile
from pathlib import Path

DEFERRED_CURRENT_BASE = '<ROOT_FINAL_CURRENT_HEAD_AFTER_ACTUAL_CONSUMER_FIX_MERGED>'
CURRENT_WORKTREE = '/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016'
LEGACY_WORKTREE = '/home/charl/Moriarty/.worktrees/sherlock-a102-build-20261016'
CURRENT_LOCK_SHA256 = '3fbc2453d641100bc34c9569116ccfddf5758c9e20375a7e036251828c9a0238'
LEGACY_LOCK_SHA256 = 'dfffb853b47a7b2d0d1b80a64bb3e826d64d66f247881603b6fc31c8e3e34003'

BETA_ASSIGNMENT_PATH = (
    '/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/'
    'raw/assignments/sherlock-beta-automated-provisioning-2026-10-18.md'
)
BETA_ASSIGNMENT_SHA256 = 'bcb1e016dd4a87ee31ddc1073405963d0860643667e2c259ebba4b706c26fabd'

PRESERVED_QUARANTINE_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-r3-persistent-quarantine-20261018-01'
)
PRESERVED_PROPOSAL_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-r3-preservation-cleanup-proposal-20261018'
)
PRESERVED_COPY_MAP_PATH = PRESERVED_PROPOSAL_ROOT + '/PRESERVATION-EXACT-COPY-MAP.proposed.json'
PRESERVED_COPY_MAP_SHA256 = '7f3127c34fd681326e13761234dc2f466b4bbe8fbdbbedbf65ab87ff38f6244f'
PRESERVED_DISPOSITION_PATH = PRESERVED_PROPOSAL_ROOT + '/actual-preservation-01/ROOT-PRESERVATION-DISPOSITION.json'
PRESERVED_DISPOSITION_SHA256 = 'd84adc1c07e1a6ded91ed480e6a8086a1c38ba5c93d0c14979cfc5c6451572df'
POST_CLEANUP_RESULT_PATH = PRESERVED_PROPOSAL_ROOT + '/actual-preservation-01/POST-CLEANUP-RESULT.json'
POST_CLEANUP_RESULT_SHA256 = 'e7804b2a59922d9ee58ffdec32d2017caae82aaeb0eb8061de86775eb845dea9'

GLOBAL_CACHE_STABILITY_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-global-cache-stability-correction-20261018'
)
ROOT_BOOKKEEPING_HASH_CHECK_PATH = GLOBAL_CACHE_STABILITY_ROOT + '/ROOT-FOUR-BOOKKEEPING-HASH-CHECK.json'
ROOT_BOOKKEEPING_HASH_CHECK_SHA256 = 'a0b64c884bf060b507ef48ee67ba124de2608e08068b918728b613ad6929b6a9'
GLOBAL_CACHE_SNAPSHOT_PATH = GLOBAL_CACHE_STABILITY_ROOT + '/global-cache.immutable.snapshot'
GLOBAL_CACHE_REL = '.global-cache'
GLOBAL_CACHE_OLD_D_SHA256 = 'f7138d717dcd66b67a55a88c41e2e80b7c1ed9c60e601a6bad1b2f0de42d6447'
GLOBAL_CACHE_CURRENT_SHA256 = '390ffa709f11c02f40e6484d8d58dd009dd23f572ab2987541b41068692154fa'
GLOBAL_CACHE_CURRENT_BYTES = 356352

P3_COUNTERS_DISPOSITION_PATH = GLOBAL_CACHE_STABILITY_ROOT + '/ROOT-ACTUAL-COUNTERS-AND-CLEANUP-DISPOSITION.json'
P3_COUNTERS_DISPOSITION_SHA256 = '8d6db3b44663c16320223e5d223e6d674aeafdd46e6fe341dd2ceb21c0e534e7'
P3_PRESERVE_CLEANUP_RESULT_PATH = GLOBAL_CACHE_STABILITY_ROOT + '/actual-preserve-and-cleanup-01/RESULT.json'
P3_PRESERVE_CLEANUP_RESULT_SHA256 = 'a824faaeadaef863388d1663db722d9f526239957a73db6f04e63c72f1705948'

PRESERVED_PACKET_PINS = {
    'A0_FREEZE.json': 'bf26f2d9b5db546ba0e5549e0cb8dfb3aabeee25e6c51ed135cbc9b2e35d973a',
    'C_RAW_IDENTITIES.json': '387b188d8f2a319dc972cf4727ff21219f5207c3bc74f93970590d708f62719e',
    'D_FREEZE.json': 'f8408c7ae5b5ccb495c129a9ba4a7a7a376ee9cb4b911b6f53a588b07cd839aa',
    'E_RESOLVED.json': 'fd3db35861d3119f3ef925b682e8c53fcbd322478b7be531b9241b9789f65e23',
    'F_RETAINED_QUARANTINED_OUTPUTS_R3.json': '92b24bcd754a1ebff68e5e9bfd051b11f4d69ffb0aae03627525e8ea439ec6f5',
    'F_RETAINED_AUDIT_R3_RESULT.json': '2d3001c7c2816a2a2dd9aa80406dbec4db5e6451d200f689b6bc504e07a9110f',
}

P9_PERSISTENT_QUARANTINE_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p9-persistent-quarantine-20261019-01'
)
P9_INTEGRATION_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p9-integration-prerequisites-20261019'
)
P9_COPY_MAP_PATH = P9_INTEGRATION_ROOT + '/post-terminal-preservation-guidance/actual-inventory-bound-02/PRESERVATION-EXACT-COPY-MAP.proposed.json'
P9_COPY_MAP_SHA256 = 'd095f2e39d3569894038d3075055fdd85abb01bb1c71e77e4262a10e5d4431e2'
P9_COPY_MAP_BYTES = 2486436
P9_PRESERVE_CLEANUP_RESULT_PATH = P9_INTEGRATION_ROOT + '/actual-p9-preserve-and-cleanup-01/RESULT.json'
P9_PRESERVE_CLEANUP_RESULT_SHA256 = '5639d7c25c40506420c66a1f4be4a3490855fd601960fa7bb3428819b688c331'
P9_PRESERVE_CLEANUP_RESULT_BYTES = 8873
P9_QUARANTINE_VERIFIED_PATH = P9_INTEGRATION_ROOT + '/actual-p9-preserve-and-cleanup-01/PERSISTENT-QUARANTINE-VERIFIED.json'
P9_QUARANTINE_VERIFIED_SHA256 = '8d8c05e4153213a58bc9e1e6c52460d56ca2900d8ea2cf4b89280e8eca2cb65c'
P9_QUARANTINE_VERIFIED_BYTES = 277
P9_RAW_COUNTERS_PATH = P9_INTEGRATION_ROOT + '/post-terminal-actual-01/RAW-CGROUP-AND-MARKER.json'
P9_RAW_COUNTERS_SHA256 = '1772c63f4d3231ba1518c314fe4a4e3ce7bfb19629e212549cf3e5e81c4a1865'
P9_RAW_COUNTERS_BYTES = 2045
P9_PACKET_PINS = {
    'A0_FREEZE.json': '1d25d87069d2455b711650a55722a73495f719ce319f90b3b602fe22b5cafd58',
    'CACHED_POST_COMPILE_CHECK.json': 'e8a6152a402741dc0086acca45b3da4bffa4a0acca5a939096fc1a184fa8f969',
    'CACHED_TARGET_SEED.json': '396dcdf38ebebede37f9a285643f7a0995a1a2aeaf733398eb4eb5ffe59fdf8b',
    'C_RAW_IDENTITIES.json': '08680bb121939654c74e6cc8ba01d7d88a9d248e9dfa424f095de63909a4791a',
    'D_FREEZE.json': '617d232e820e8973b856bc4d0ad1ade34f0599013524060f83172dd64d3282a7',
    'E_RESOLVED.json': 'fd3db35861d3119f3ef925b682e8c53fcbd322478b7be531b9241b9789f65e23',
    'F_CACHED_CURRENT_RESULT.json': 'e897099d8048bebb747d312c700aa87830b10f6170e8f157be68bf56c9f4c297',
    'F_QUARANTINED_OUTPUTS.json': '60a0744c1053533dbf9921984c3dc908132199e64d8f9f1d8bb1985548f467dc',
    'SOURCE_HOME_RECONSTITUTION.json': '8d2c22a9359e3a55a6a47b42f2954ede140f1a15e357b77d1ea53d64ec302805',
}
P9_F_RESULT_BYTES = 43668
CURRENT_P9_BACKEND_ELF_REL = 'debug/deps/mori_backend-3a2f695515330e5a'
CURRENT_P9_BACKEND_ELF_SHA256 = '65d818d5ba5d1f2ea9bdf0a8776f02d6e4c97d2514a1a8975a646ba98b5661be'
CURRENT_P9_BACKEND_ELF_BYTES = 454214504
P9_PARTITION_COUNTS = {
    'total_preserved_files': 3566,
    'target_total': 3548,
    'packet_total': 17,
    'one_shot_total': 1,
    'owner_literal_workspace_fingerprint': 22,
    'owner_literal_workspace_deps': 14,
    'owner_literal_workspace_incremental': 669,
    'unadopted_other_cache_candidate': 2843,
    'host_outputs_total': 90,
    'host_build_script_binaries': 68,
    'host_proc_macro_dylibs': 22,
    'build_script_run_units': 34,
}
P9_HISTORICAL_SEED_PARTITION_COUNTS = {
    'owner_literal_workspace_fingerprint': 20,
    'owner_literal_workspace_deps': 14,
    'owner_literal_workspace_incremental': 621,
    'unadopted_other_cache_candidate': 2843,
}
P9_SEED_PACKET_PRESERVED_COPY_MAP_SHA256 = '7f3127c34fd681326e13761234dc2f466b4bbe8fbdbbedbf65ab87ff38f6244f'

NON_DEBUG_CACHE_CANDIDATES = {
    '.rustc_info.json': '384fec7abe5b2fe2dd0005a44f4acad1dc30fd607fda4dd5403302f80e855347',
    'generated/bincode/ZkStdLibArchV1_Decode.rs': '2d5f3b285324c47f5ec331aeccd5dc23aa652f523a2d1544128ca71d123bb7d1',
    'generated/bincode/ZkStdLibArch_Decode.rs': '0915851dc01452a9b6447258a14295a822454295a95aa256ca9e39298412f984',
    'generated/bincode/ZkStdLibArch_Encode.rs': 'bf42b2c92b0700aa280c032282d8376d4a7202095d7abc26dba7c600e33e8ac6',
}

OLD_BACKEND_ELF_REL = 'debug/deps/mori_backend-3a2f695515330e5a'
OLD_BACKEND_ELF_SHA256 = '9fb9156862a5da2dfe19c2a5f4d09100a06f238871fb2674ec9b086a0c58e25a'
OLD_BACKEND_ELF_BYTES = 432019824

PRODUCTION_PARTITION_COUNTS = {
    'total_preserved_files': 3554,
    'target_total': 3498,
    'packet_total': 55,
    'one_shot_total': 1,
    'owner_literal_workspace_fingerprint': 20,
    'owner_literal_workspace_deps': 14,
    'owner_literal_workspace_incremental': 621,
    'unadopted_other_cache_candidate': 2843,
    'host_outputs_total': 90,
    'host_build_script_binaries': 68,
    'host_proc_macro_dylibs': 22,
    'build_script_run_units': 34,
}

HISTORICAL_POST_R3_DEBITS = {
    'prior_post_r3_envelope_usec': 346198712,
    'prior_post_r3_supervisor_usec': 75915030,
    'prior_post_r3_phases_usec': 270283681,
    'retired_p2_p3_envelope_usec': 13455030,
    'retired_p2_p3_supervisor_usec': 13455030,
    'retired_p2_p3_phases_usec': 0,
    'retired_p4_envelope_usec': 13735290,
    'retired_p4_supervisor_usec': 13735290,
    'retired_p4_phases_usec': 0,
    'retired_p5_envelope_usec': 37863695,
    'retired_p5_supervisor_usec': 37863695,
    'retired_p5_phases_usec': 0,
    'retired_p6_envelope_usec': 39019876,
    'retired_p6_supervisor_usec': 39019876,
    'retired_p6_phases_usec': 0,
    'retired_p7_envelope_usec': 187121653,
    'retired_p7_supervisor_usec': 15162810,
    'retired_p7_phases_usec': 171958842,
    'retired_p8_envelope_usec': 198086463,
    'retired_p8_supervisor_usec': 12326620,
    'retired_p8_phases_usec': 185759842,
    'retired_p9_envelope_usec': 127323100,
    'retired_p9_supervisor_usec': 15815495,
    'retired_p9_phases_usec': 111507604,
    'retired_cached_attempts_envelope_usec': 616605107,
    'retired_cached_attempts_supervisor_usec': 147378816,
    'retired_cached_attempts_phases_usec': 469226288,
    'envelope_usec': 962803819,
    'supervisor_usec': 223293846,
    'supervisor_cpu_seconds_ceil': 224,
    'phases_usec': 739509969,
    'f_stdout_bytes': 0,
    # Count each distinct attempt packet once, including the P8 audit stream.
    'f_stderr_bytes': 43081,
    'r3_outer_wall_seconds_ceil': 24,
    'r3_inner_wall_seconds_ceil': 13,
    'prior_a0_to_e_wall_seconds_ceil': 40,
    'p2_refusal_wall_seconds_ceil': 5,
    'p3_refusal_wall_seconds_ceil': 11,
    'p4_refusal_wall_seconds_ceil': 15,
    'p5_refusal_wall_seconds_ceil': 40,
    'p6_refusal_wall_seconds_ceil': 44,
    # P7: 135 is the full outer-sequence elapsed ceil (Root-chosen F debit), not a measured F launch wall.
    'p7_outer_wall_seconds_ceil': 135,
    # P8: 133 is the selected conservative outer-sequence ceil, not measured F launch wall.
    'p8_outer_wall_seconds_ceil': 133,
    # P9: 121 is the conservative outer-completion ceil, not Cargo 25.69s or F launch wall.
    'p9_outer_wall_seconds_ceil': 121,
    'remaining_total_wall_seconds_max': 6038,
    'remaining_f_wall_seconds_max': 2133,
    'remaining_supervisor_cpu_seconds_max': 10576,
    'gross_supervisor_cpu_seconds_max': 10800,
}


# Retired P10 (same06 groups) and the P11 read-only seed copy. Every P2-P9 store above stays byte-identical history.
P10_BASE_HEAD = '229770592adbf3fffdf5fe77f8fa82c8c3aee194'
P10_PERSISTENT_QUARANTINE_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p10-persistent-quarantine-20261019-01'
)
P10_INTEGRATION_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native-p10-integration-metadata-plan-20261019'
)
P11_PREPARATION_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native-p11-current-WW-two-leaf-metadata-preparation-20261019'
)
P10_ORIGINAL_MANIFEST_PATH = P10_PERSISTENT_QUARANTINE_ROOT + '/PRESERVED-SHA256-MANIFEST.json'
P10_ORIGINAL_MANIFEST_SHA256 = '93da1caa9d526bdf91aa0795ab1b3e8ebd212ceb7308c80ede2fe298d853c2bd'
P10_ORIGINAL_MANIFEST_BYTES = 779141
P10_F_RESULT_PATH = P10_PERSISTENT_QUARANTINE_ROOT + '/packet/F_CACHED_CURRENT_RESULT.json'
P10_F_RESULT_SHA256 = 'fad41c7ed0b3b31bce9f6e50c331c528b24c19e83e55f7b12c682f57de2457a0'
P10_F_RESULT_BYTES = 43450
P10_QUARANTINED_OUTPUTS_PATH = P10_PERSISTENT_QUARANTINE_ROOT + '/packet/F_QUARANTINED_OUTPUTS.json'
P10_QUARANTINED_OUTPUTS_SHA256 = 'fb4d07bc81583cf72de508438c385c25cb14a15b3e12b46c6d727bc54ddd081e'
P10_QUARANTINED_OUTPUTS_BYTES = 682933
P10_RAW_COUNTERS_PATH = P10_INTEGRATION_ROOT + '/future-reversible-maintenance-03/RAW-POST-TERMINAL-CGROUPS.json'
P10_RAW_COUNTERS_SHA256 = '048a82dbf70367a226d7be26d08639a0fce566d9bea3d5b8663da0782fcfd46a'
P10_RAW_COUNTERS_BYTES = 2856
P10_OUTER_RESULT_PATH = P10_INTEGRATION_ROOT + '/future-reversible-maintenance-03/one-changed-W3-cached-current-F/RESULT.json'
P10_OUTER_RESULT_SHA256 = '53d757737b852f07fd971c541b10f6b9308b86e04c179a34a16ad290b6d00ff2'
P10_OUTER_RESULT_BYTES = 114
P10_CLEANUP_PATH = P10_INTEGRATION_ROOT + '/actual-teardown-p10-root-01/ROOT-CLEANUP-DISCHARGED.json'
P10_CLEANUP_SHA256 = '7735c4eaf7c3e0b8a0bd741d5aca54cce324397f74896e57f7ca1991f307a0ba'
P10_CLEANUP_BYTES = 510
P10_ROW_PROJECTION_PATH = P11_PREPARATION_ROOT + '/P10-PRESERVATION-ROW-LIST.proposed.json'
P10_ROW_PROJECTION_SHA256 = '406f9f6b3e0fb5dfd6a5ecb56c5baf0d7504578d3ad69910bdf072513394910d'
P10_ROW_PROJECTION_BYTES = 1889042
P10_BACKEND_ELF_REL = 'debug/deps/mori_backend-3a2f695515330e5a'
P10_BACKEND_ELF_SHA256 = 'a70705f0f55a05bc000f480579cf20042cac69422d624d82e2e1191e6c2b839d'
P10_BACKEND_ELF_BYTES = 467781064
P10_POST_COMPILE_CHECK = {
    'build_script_run_units_verified_not_reexecuted': 34,
    'fresh_mori_backend_executables': [
        {
            'bytes': 467781064,
            'relative': 'debug/deps/mori_backend-3a2f695515330e5a',
            'sha256': 'a70705f0f55a05bc000f480579cf20042cac69422d624d82e2e1191e6c2b839d',
        },
    ],
    'host_outputs_verified_unchanged': 90,
}
P10_F_STDOUT_BYTES = 0
P10_F_STDERR_BYTES = 16326
P10_NATIVE_OUTER_SECONDS = 97.06481482301024
P10_PREPHASE_OUTER_SECONDS = 4.411494735992164
P10_MARKER_REL = 'ONE_SHOT_STARTED'
# Completed P10 OUTPUT (the P11 seed basis); distinct from the P10 producer's P9-basis INPUT counts below.
P10_PARTITION_COUNTS = {
    'total_preserved_files': 3574,
    'target_total': 3556,
    'packet_total': 17,
    'one_shot_total': 1,
    'owner_literal_workspace_fingerprint': 22,
    'owner_literal_workspace_deps': 22,
    'owner_literal_workspace_incremental': 669,
    'unadopted_other_cache_candidate': 2843,
    'host_outputs_total': 90,
    'host_build_script_binaries': 68,
    'host_proc_macro_dylibs': 22,
    'build_script_run_units': 34,
}
P10_PRODUCER_SEED_PARTITION_COUNTS = {
    'owner_literal_workspace_fingerprint': 22,
    'owner_literal_workspace_deps': 14,
    'owner_literal_workspace_incremental': 669,
    'unadopted_other_cache_candidate': 2843,
}
P11_SEED_COPY_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p11-readonly-target-seed-20261019-01'
)
P11_SEED_COPY_RECEIPT_PATH = P11_PREPARATION_ROOT + '/actual-readonly-seed-copy-01/SEED-COPY-RECEIPT.json'
# Unissued until Root's actual copy receipt exists. Non-hex/None placeholders refuse; they are not hashes.
P11_SEED_COPY_RECEIPT_SHA256 = 'b6aff969d06c63b4a927174f3b553bb4d553dce4097ed885f21fe50fb200cceb'
P11_SEED_COPY_RECEIPT_BYTES = 1375972
P11_SEED_COPY_TOTAL_BYTES = 2916032469


# Exact P10 original modes that are not 0644, as recorded by the pinned manifest and projection; all other originals are 0644.
P10_ORIGINAL_NON_0644_SOURCE_MODES = {
    'packet/A0_FREEZE.json': 0o600,
    'packet/CACHED_POST_COMPILE_CHECK.json': 0o600,
    'packet/CACHED_TARGET_SEED.json': 0o600,
    'packet/C_RAW_IDENTITIES.json': 0o600,
    'packet/D_FREEZE.json': 0o600,
    'packet/E_RESOLVED.json': 0o444,
    'packet/F_CACHED_CURRENT_RESULT.json': 0o600,
    'packet/F_QUARANTINED_OUTPUTS.json': 0o600,
    'packet/SOURCE_HOME_RECONSTITUTION.json': 0o600,
    'packet/USED_LAUNCH_28732fe46d861a5ed8293a810f1b241d5532ee1ff0563cd69b6519f1843c54a9': 0o600,
    'packet/USED_LAUNCH_8a8f0e843f1f5b7d3c439500013ee1e0898fbe1e38b29264fbe9eba12a7a07b9': 0o600,
    'target/debug/incremental/mori_backend-297g9ampaudl7/s-hncj0h1jpk-1blmeg1.lock': 0o600,
    'target/debug/incremental/mori_core-1cpvdxzbd3wk1/s-hncj0ccv2e-14tvpl2.lock': 0o600,
    'target/debug/incremental/mori_diagnostics-1f3x0ulm6pbn9/s-hncj0c7edw-1s05u8g.lock': 0o600,
    'target/debug/incremental/mori_lexer-0f2l2fry7tng3/s-hncj0c8vr2-0qof0dp.lock': 0o600,
    'target/debug/incremental/mori_span-19ygjfn93jgkc/s-hncj0c6oqb-1vj8gtt.lock': 0o600,
}

POST_P10_HISTORICAL_DEBITS = {
    'prior_post_p9_envelope_usec': 962803819,
    'prior_post_p9_supervisor_usec': 223293846,
    'prior_post_p9_phases_usec': 739509969,
    # Same-group live P10 retirement, already including the earlier pre-phase refusal; never added again.
    'retired_p10_envelope_usec': 116543574,
    'retired_p10_supervisor_usec': 19626044,
    'retired_p10_phases_usec': 96917530,
    'envelope_usec': 1079347393,
    'supervisor_usec': 242919890,
    'supervisor_cpu_seconds_ceil': 243,
    'phases_usec': 836427499,
    'independent_envelope_minus_supervisor_minus_phases_usec': 4,
    'p10_f_stdout_bytes': 0,
    'p10_f_stderr_bytes': 16326,
    'f_stdout_bytes': 0,
    'f_stderr_bytes': 59407,
    # 5 = ceil of the P10 pre-phase outer; it lowered whole wall 6038 -> 6033 once and is never debited again.
    'p10_prephase_outer_wall_seconds_ceil': 5,
    # 98 = ceil of the P10 native outer; the existing outer-ceil class debits it to both whole and F wall.
    'p10_outer_wall_seconds_ceil': 98,
    'remaining_total_wall_seconds_max': 5935,
    'remaining_f_wall_seconds_max': 2035,
    'remaining_supervisor_cpu_seconds_max': 10557,
    'gross_supervisor_cpu_seconds_max': 10800,
}
WORKSPACE_CRATE_PREFIXES = (
    'mori-ast-',
    'mori-backend-',
    'mori-checker-',
    'mori-core-',
    'mori-diagnostics-',
    'mori-fmt-',
    'mori-ledger-',
    'mori-lexer-',
    'mori-parser-',
    'mori-span-',
    'mori-',
    'mori_ast-',
    'mori_backend-',
    'mori_checker-',
    'mori_core-',
    'mori_diagnostics-',
    'mori_fmt-',
    'mori_ledger-',
    'mori_lexer-',
    'mori_parser-',
    'mori_span-',
    'libmori-ast-',
    'libmori-backend-',
    'libmori-checker-',
    'libmori-core-',
    'libmori-diagnostics-',
    'libmori-fmt-',
    'libmori-ledger-',
    'libmori-lexer-',
    'libmori-parser-',
    'libmori-span-',
    'libmori-',
    'libmori_ast-',
    'libmori_backend-',
    'libmori_checker-',
    'libmori_core-',
    'libmori_diagnostics-',
    'libmori_fmt-',
    'libmori_ledger-',
    'libmori_lexer-',
    'libmori_parser-',
    'libmori_span-',
)
WORKSPACE_EXACT_STEMS = (
    'mori',
    'mori-ast',
    'mori-backend',
    'mori-checker',
    'mori-core',
    'mori-diagnostics',
    'mori-fmt',
    'mori-ledger',
    'mori-lexer',
    'mori-parser',
    'mori-span',
    'mori_ast',
    'mori_backend',
    'mori_checker',
    'mori_core',
    'mori_diagnostics',
    'mori_fmt',
    'mori_ledger',
    'mori_lexer',
    'mori_parser',
    'mori_span',
    'libmori',
    'libmori-ast',
    'libmori-backend',
    'libmori-checker',
    'libmori-core',
    'libmori-diagnostics',
    'libmori-fmt',
    'libmori-ledger',
    'libmori-lexer',
    'libmori-parser',
    'libmori-span',
    'libmori_ast',
    'libmori_backend',
    'libmori_checker',
    'libmori_core',
    'libmori_diagnostics',
    'libmori_fmt',
    'libmori_ledger',
    'libmori_lexer',
    'libmori_parser',
    'libmori_span',
)
SOURCE_EPOCH_NS = 1_760_000_000_000_000_000
TARGET_MIN_EPOCH_NS = 1_760_000_010_000_000_000
TARGET_FINGERPRINT_EPOCH_NS = 1_760_000_020_000_000_000

# Verified span of successful-r3 per-file mtimes preserved in the read-only quarantine (Root all-3498 stat join).
R3_ARCHIVE_MTIME_RANGE_NS = (1_792_317_334_779_394_537, 1_792_317_392_403_385_903)
HEX40 = re.compile(r'[0-9a-f]{40}\Z')
BUILD_UNIT_RE = re.compile(r'debug/build/([^/]+-[0-9a-f]{16})/(.*)\Z')


def _is_workspace_segment(seg):
    if not seg:
        return False
    stem = seg.split('.', 1)[0]
    if stem in WORKSPACE_EXACT_STEMS:
        return True
    if '-' in stem:
        crate_stem, hash_suffix = stem.rsplit('-', 1)
        if (
            crate_stem in WORKSPACE_EXACT_STEMS
            and (crate_stem + '-') in WORKSPACE_CRATE_PREFIXES
            and hash_suffix
            and hash_suffix.isalnum()
            and hash_suffix.islower()
        ):
            return True
    return False


def classify_preserved_target_rel(rel, guard):
    """Classify a preserved target-relative path into workspace exclusions vs third-party cache."""
    norm = str(guard.relative(rel))
    if norm in NON_DEBUG_CACHE_CANDIDATES:
        return 'unadopted_other_cache_candidate'
    parts = norm.split('/')
    guard.need(len(parts) >= 2 and parts[0] == 'debug', 'unexpected target top-level path: ' + norm)
    if len(parts) >= 3 and parts[1] == '.fingerprint':
        if _is_workspace_segment(parts[2]):
            return 'owner_literal_workspace_fingerprint'
        return 'unadopted_other_cache_candidate'
    if len(parts) >= 3 and parts[1] == 'incremental':
        if _is_workspace_segment(parts[2]):
            return 'owner_literal_workspace_incremental'
        return 'unadopted_other_cache_candidate'
    if len(parts) == 3 and parts[1] == 'deps':
        if _is_workspace_segment(parts[2]):
            return 'owner_literal_workspace_deps'
        return 'unadopted_other_cache_candidate'
    if len(parts) == 2:
        if _is_workspace_segment(parts[1]):
            return 'owner_literal_workspace_deps'
        return 'unadopted_other_cache_candidate'
    if len(parts) >= 3 and parts[1] == 'build':
        guard.need(not _is_workspace_segment(parts[2]), 'unexpected workspace build-script unit: ' + norm)
        return 'unadopted_other_cache_candidate'
    guard.need(not any(_is_workspace_segment(p) for p in parts[1:]), 'unclassified workspace path: ' + norm)
    return 'unadopted_other_cache_candidate'


def _git_blob_bytes(raw_bytes):
    h = hashlib.sha1()
    h.update(('blob %d\0' % len(raw_bytes)).encode('ascii'))
    h.update(raw_bytes)
    return h.hexdigest()


def _extract_crate_archive(archive_path, dest_dir, source_relative, expected_files, guard):
    """Pure Python tar.gz extraction of a verified .crate archive into dest_dir."""
    dest_dir.mkdir(parents=True, exist_ok=True)
    extracted = {}
    with tarfile.open(archive_path, 'r:gz') as tf:
        for member in tf.getmembers():
            rel = str(guard.relative(member.name))
            parts = rel.split('/')
            guard.need(len(parts) >= 2 and parts[0] == dest_dir.name, 'crate member prefix mismatch: ' + rel)
            sub_rel = '/'.join(parts[1:])
            if member.isdir():
                (dest_dir / sub_rel).mkdir(parents=True, exist_ok=True)
                continue
            guard.need(member.isfile() and not member.issparse(), 'non-regular tar member in crate archive: ' + rel)
            out_path = dest_dir / sub_rel
            out_path.parent.mkdir(parents=True, exist_ok=True)
            guard.need(not out_path.exists() and not out_path.is_symlink(), 'duplicate crate member: ' + rel)
            fobj = tf.extractfile(member)
            guard.need(fobj is not None, 'missing tar member payload: ' + rel)
            h = hashlib.sha256()
            with open(out_path, 'wb') as out_f:
                while True:
                    chunk = fobj.read(guard.CHUNK)
                    if not chunk:
                        break
                    h.update(chunk)
                    out_f.write(chunk)
            sha = h.hexdigest()
            os.chmod(out_path, 0o644)
            os.utime(out_path, ns=(SOURCE_EPOCH_NS, SOURCE_EPOCH_NS), follow_symlinks=False)
            extracted[sub_rel] = sha
            full_rel = source_relative + '/' + sub_rel
            guard.need(full_rel in expected_files and expected_files[full_rel] == sha, 'extracted crate member hash mismatch: ' + full_rel)
    return extracted


def _set_tree_source_epoch(root):
    for base, dirs, files in os.walk(root, followlinks=False):
        bpath = Path(base)
        for name in files:
            p = bpath / name
            st = p.lstat()
            if stat.S_ISREG(st.st_mode):
                os.utime(p, ns=(SOURCE_EPOCH_NS, SOURCE_EPOCH_NS), follow_symlinks=False)
        for name in dirs:
            d = bpath / name
            if not d.is_symlink():
                os.utime(d, ns=(SOURCE_EPOCH_NS, SOURCE_EPOCH_NS), follow_symlinks=False)


def _verify_bookkeeping_hash_check_record(bookkeeping_check_file, old_home_root, old_d_files, guard):
    check_p = Path(bookkeeping_check_file)
    guard.regular(check_p)
    check_sha = guard.digest(check_p)
    if str(check_p) == ROOT_BOOKKEEPING_HASH_CHECK_PATH:
        guard.need(check_sha == ROOT_BOOKKEEPING_HASH_CHECK_SHA256, 'Root bookkeeping hash check receipt SHA-256 mismatch')
    obj = guard.jload(check_p)
    guard.need(
        isinstance(obj, dict)
        and obj.get('recordType') == 'RootExactFourNonsecretCargoBookkeepingContentCheck'
        and isinstance(obj.get('files'), list)
        and len(obj['files']) == 4,
        'invalid Root bookkeeping hash check schema',
    )
    by_rel = {}
    for row in obj['files']:
        guard.need(isinstance(row, dict) and row.get('pre_post_stat_stable') is True, 'unstable bookkeeping stat row')
        p_str = str(row.get('path', ''))
        prefix = str(old_home_root) + '/'
        guard.need(p_str.startswith(prefix), 'bookkeeping check row outside old_home_root: ' + p_str)
        rel = str(guard.relative(p_str[len(prefix):]))
        guard.need(rel not in by_rel, 'duplicate bookkeeping check row: ' + rel)
        guard.need(row.get('expected_D_FREEZE_sha256') == old_d_files.get(rel), 'bookkeeping check historical D_FREEZE hash mismatch: ' + rel)
        by_rel[rel] = row

    expected_four_rels = {GLOBAL_CACHE_REL, '.package-cache', 'git/CACHEDIR.TAG', 'registry/CACHEDIR.TAG'}
    guard.need(set(by_rel.keys()) == expected_four_rels, 'unexpected bookkeeping hash check path set')
    for rel, row in by_rel.items():
        if rel == GLOBAL_CACHE_REL:
            guard.need(
                row.get('expected_D_FREEZE_sha256') == GLOBAL_CACHE_OLD_D_SHA256
                and row.get('sha256') == GLOBAL_CACHE_CURRENT_SHA256
                and row.get('bytes') == GLOBAL_CACHE_CURRENT_BYTES
                and row.get('matches') is False,
                'unexpected .global-cache transition in bookkeeping hash check record',
            )
            snap = row.get('immutable_snapshot')
            guard.need(
                isinstance(snap, dict)
                and isinstance(snap.get('path'), str)
                and snap.get('bytes') == GLOBAL_CACHE_CURRENT_BYTES
                and snap.get('sha256') == GLOBAL_CACHE_CURRENT_SHA256,
                'missing or invalid immutable_snapshot descriptor in bookkeeping hash check record',
            )
            if str(check_p) == ROOT_BOOKKEEPING_HASH_CHECK_PATH:
                guard.need(
                    snap.get('path') == GLOBAL_CACHE_SNAPSHOT_PATH,
                    'immutable_snapshot path mismatch against pinned Root snapshot',
                )
            snap_p = Path(snap['path'])
            guard.regular(snap_p)
            guard.need(
                snap_p.stat().st_size == GLOBAL_CACHE_CURRENT_BYTES
                and guard.digest(snap_p) == GLOBAL_CACHE_CURRENT_SHA256,
                'immutable_snapshot file size or SHA-256 mismatch on disk',
            )
        else:
            guard.need(
                row.get('sha256') == row.get('expected_D_FREEZE_sha256') and row.get('matches') is True,
                'non-.global-cache cargo-lockfile entry unexpectedly differed: ' + rel,
            )
    return check_sha, by_rel


def reconstitute_source_home(
    c,
    guard,
    quarantine_root=PRESERVED_QUARANTINE_ROOT,
    expected_pins=None,
    symlink_targets=None,
    bookkeeping_check_file=ROOT_BOOKKEEPING_HASH_CHECK_PATH,
):
    """Reconstitute and freeze source_home from A0_FREEZE + preserved packet receipts without Cargo B/C/E."""
    pins = PRESERVED_PACKET_PINS if expected_pins is None else expected_pins
    q_root = Path(quarantine_root)
    q_packet = q_root / 'packet'
    for fname, exp_sha in pins.items():
        p = q_packet / fname
        guard.regular(p)
        guard.need(guard.digest(p) == exp_sha, 'preserved packet receipt digest mismatch: ' + fname)

    old_a0 = guard.jload(q_packet / 'A0_FREEZE.json')
    old_c_raw = guard.jload(q_packet / 'C_RAW_IDENTITIES.json')
    old_d_freeze = guard.jload(q_packet / 'D_FREEZE.json')
    old_e_resolved = guard.jload(q_packet / 'E_RESOLVED.json')

    old_c_files = old_c_raw['files']
    old_d_files = old_d_freeze['files']
    old_cargo_ok = {row['path']: row for row in old_c_raw.get('cargo_ok', [])}
    generated_rels = {row['relative'] for row in c['expected_generated_files']}

    guard.need(old_c_files == old_d_files, 'preserved C_RAW_IDENTITIES and D_FREEZE file map mismatch')

    source_home = Path(c['paths']['source_home'])
    packet_dir = Path(c['paths']['packet'])

    mount_sources = [Path(m) for m in c['source_copy_mounts']]
    old_home_root = Path('/home/charl/.cargo')
    for m_src in mount_sources:
        if m_src.is_dir():
            old_home_root = m_src
            break

    bookkeeping_check_sha = None
    bookkeeping_by_rel = {}
    if bookkeeping_check_file is not None:
        bookkeeping_check_sha, bookkeeping_by_rel = _verify_bookkeeping_hash_check_record(
            bookkeeping_check_file, old_home_root, old_d_files, guard
        )

    git_control_projection_sha = None
    git_control_by_rel = {}
    if EIGHT_GIT_CONTROL_PROJECTION_PATH is not None:
        git_control_projection_sha, git_control_by_rel = _verify_eight_git_control_snapshots(
            EIGHT_GIT_CONTROL_PROJECTION_PATH, old_d_files, guard
        )

    # 1. Stage all A0_FREEZE rows using actual 'destination' key.
    staged_rows = []
    for row in old_a0['inputs']:
        dest_rel = str(guard.relative(row['destination']))
        src = Path(row['source'])
        guard.regular(src)
        guard.need(guard.digest(src) == row['sha256'], 'A0 source digest changed: ' + dest_rel)
        dst = source_home / dest_rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src, dst)
        os.chmod(dst, 0o644)
        os.utime(dst, ns=(SOURCE_EPOCH_NS, SOURCE_EPOCH_NS), follow_symlinks=False)
        guard.need(guard.digest(dst) == row['sha256'], 'staged A0 copy digest mismatch: ' + dest_rel)
        staged_rows.append({
            'destination': dest_rel,
            'sha256': row['sha256'],
            'source': row['source'],
            'stat': guard.stable_stat(src),
        })
    guard.receipt(c, 'A0_FREEZE.json', {'inputs': staged_rows}, 'A0')

    # 2. Extract all registry_roots using actual 'source_relative', 'archive', and 'package_checksum' keys.
    for reg in c['registry_roots']:
        src_rel = str(guard.relative(reg['source_relative']))
        arch_rel = str(guard.relative(reg['archive']))
        pkg_cksum = reg['package_checksum']
        arch_path = source_home / arch_rel
        guard.regular(arch_path)
        guard.need(guard.digest(arch_path) == pkg_cksum, 'staged crate archive hash mismatch: ' + arch_rel)
        dest_dir = source_home / src_rel
        _extract_crate_archive(arch_path, dest_dir, src_rel, old_d_files, guard)

    # 3. Materialize Git checkouts using actual 'relative_root', 'commit', and 'files' (relative, blob, mode).
    checkout_commits = {}
    for g in c['git_checkouts']:
        g_root_rel = str(guard.relative(g['relative_root']))
        checkout_commits[g_root_rel] = str(g.get('commit', ''))
        g_dest = source_home / g_root_rel
        g_dest.mkdir(parents=True, exist_ok=True)
        src_checkout = old_home_root / g_root_rel
        guard.need(src_checkout.is_dir() and not src_checkout.is_symlink(), 'missing host git checkout source for: ' + g_root_rel)
        for f in g['files']:
            f_rel = str(guard.relative(f['relative']))
            full_rel = g_root_rel + '/' + f_rel
            f_mode = f['mode']
            src_f = src_checkout / f_rel
            dst_f = g_dest / f_rel
            dst_f.parent.mkdir(parents=True, exist_ok=True)
            if f_mode == '120000':
                if src_f.is_symlink():
                    target_text = os.readlink(src_f)
                elif symlink_targets and full_rel in symlink_targets:
                    target_text = symlink_targets[full_rel]
                else:
                    guard.need(False, 'missing host git symlink source for: ' + full_rel)
                guard.need(_git_blob_bytes(target_text.encode('utf-8')) == f['blob'], 'git symlink blob mismatch: ' + full_rel)
                guard.need(not dst_f.exists() and not dst_f.is_symlink(), 'existing path at git symlink: ' + full_rel)
                os.symlink(target_text, dst_f)
                guard.need(old_d_files.get(full_rel) == 'git-symlink:' + f['blob'], 'git symlink D_FREEZE entry mismatch: ' + full_rel)
            else:
                guard.need(f_mode in ('100644', '100755'), 'unsupported git file mode: ' + full_rel)
                guard.regular(src_f)
                guard.need(guard.digest(src_f, git=True) == f['blob'], 'git checkout blob mismatch: ' + full_rel)
                shutil.copyfile(src_f, dst_f)
                os.chmod(dst_f, 0o755 if f_mode == '100755' else 0o644)
                os.utime(dst_f, ns=(SOURCE_EPOCH_NS, SOURCE_EPOCH_NS), follow_symlinks=False)
                guard.need(guard.digest(dst_f) == old_d_files.get(full_rel), 'git checkout D_FREEZE hash mismatch: ' + full_rel)

    # 4. Materialize expected_generated_files (425 cargo-ok + 150 git-control + 4 cargo-lockfile).
    reconstructed_cargo_ok = []
    dated_bookkeeping_deltas = []
    expected_new_d_files = dict(old_d_files)
    for row in c['expected_generated_files']:
        rel = str(guard.relative(row['relative']))
        kind = row['kind']
        dst = source_home / rel
        dst.parent.mkdir(parents=True, exist_ok=True)
        expected_old_sha = old_d_files.get(rel)
        guard.need(expected_old_sha is not None, 'expected_generated_files path absent from preserved D_FREEZE: ' + rel)
        accepted_sha = expected_old_sha
        cand = old_home_root / rel
        copied = False
        if (
            kind == 'cargo-lockfile'
            and rel == GLOBAL_CACHE_REL
            and bookkeeping_check_sha is not None
            and expected_old_sha == GLOBAL_CACHE_OLD_D_SHA256
            and GLOBAL_CACHE_REL in bookkeeping_by_rel
        ):
            snap_info = bookkeeping_by_rel[GLOBAL_CACHE_REL]['immutable_snapshot']
            snap_p = Path(snap_info['path'])
            guard.regular(snap_p)
            guard.need(
                snap_p.stat().st_size == GLOBAL_CACHE_CURRENT_BYTES
                and guard.digest(snap_p) == GLOBAL_CACHE_CURRENT_SHA256,
                'immutable .global-cache snapshot modified before copy',
            )
            shutil.copyfile(snap_p, dst)
            accepted_sha = GLOBAL_CACHE_CURRENT_SHA256
            expected_new_d_files[rel] = GLOBAL_CACHE_CURRENT_SHA256
            dated_bookkeeping_deltas.append({
                'relative': rel,
                'kind': kind,
                'historical_d_sha256': expected_old_sha,
                'current_sha256': GLOBAL_CACHE_CURRENT_SHA256,
                'bytes': GLOBAL_CACHE_CURRENT_BYTES,
                'bookkeeping_check_sha256': bookkeeping_check_sha,
                'immutable_snapshot_path': str(snap_p),
            })
            copied = True
        elif (
            kind == 'git-control'
            and git_control_projection_sha is not None
            and rel in git_control_by_rel
        ):
            spec = EIGHT_GIT_CONTROL_SNAPSHOT_SPEC[rel]
            snap_info = git_control_by_rel[rel]['proposed_current_snapshot']
            snap_p = Path(snap_info['path'])
            guard.regular(snap_p)
            guard.need(
                stat.S_IMODE(snap_p.lstat().st_mode) == 0o444
                and expected_old_sha == spec['historical_sha256']
                and snap_p.stat().st_size == spec['bytes']
                and guard.digest(snap_p) == spec['sha256'],
                'immutable git-control snapshot modified before copy: ' + rel,
            )
            _validate_git_control_snapshot_payload(rel, snap_p, guard)
            shutil.copyfile(snap_p, dst)
            accepted_sha = spec['sha256']
            expected_new_d_files[rel] = spec['sha256']
            dated_bookkeeping_deltas.append({
                'relative': rel,
                'kind': kind,
                'historical_d_sha256': expected_old_sha,
                'current_sha256': spec['sha256'],
                'bytes': spec['bytes'],
                'git_control_projection_sha256': git_control_projection_sha,
                'immutable_snapshot_path': str(snap_p),
            })
            copied = True
        elif cand.exists() and not cand.is_symlink() and cand.is_file():
            cand_sha = guard.digest(cand)
            if cand_sha == expected_old_sha:
                shutil.copyfile(cand, dst)
                copied = True
        if not copied and kind == 'git-control' and '/.git/objects/' in rel:
            target_name = Path(rel).name
            for search_root in (source_home / 'git/db', old_home_root / 'git'):
                if not search_root.is_dir():
                    continue
                for db_obj in search_root.rglob(target_name):
                    if db_obj.is_file() and not db_obj.is_symlink() and guard.digest(db_obj) == expected_old_sha:
                        shutil.copyfile(db_obj, dst)
                        copied = True
                        break
                if copied:
                    break
        if not copied and kind == 'cargo-ok':
            ok_meta = old_cargo_ok.get(rel)
            guard.need(
                (ok_meta is not None and ok_meta['sha256'] == expected_old_sha) or expected_old_sha == hashlib.sha256(b'').hexdigest(),
                'missing cargo_ok metadata in C_RAW_IDENTITIES: ' + rel,
            )
            candidates = (b'', b'{"v":1}', b'{"v":1}\n')
            matched_payload = None
            for payload in candidates:
                if hashlib.sha256(payload).hexdigest() == expected_old_sha and (ok_meta is None or len(payload) == ok_meta['bytes']):
                    matched_payload = payload
                    break
            guard.need(matched_payload is not None, 'unrecognized cargo-ok payload digest for: ' + rel)
            with open(dst, 'wb') as f:
                f.write(matched_payload)
            reconstructed_cargo_ok.append({'relative': rel, 'bytes': len(matched_payload), 'sha256': expected_old_sha})
            copied = True
        if not copied and kind == 'cargo-lockfile' and rel != GLOBAL_CACHE_REL:
            for payload in (b'', b'Signature: 8a477f597d28d172789f06886806bc55\n'):
                if hashlib.sha256(payload).hexdigest() == expected_old_sha:
                    with open(dst, 'wb') as f:
                        f.write(payload)
                    copied = True
                    break
        if not copied and kind == 'git-control' and rel not in EIGHT_GIT_CONTROL_SNAPSHOT_SPEC:
            static_git_candidates = [b'', b'ref: refs/heads/master\n']
            for g_root_rel, g_commit in checkout_commits.items():
                if rel == g_root_rel + '/.git/refs/heads/master' and g_commit:
                    static_git_candidates.append((g_commit + '\n').encode('ascii'))
            for payload in static_git_candidates:
                if hashlib.sha256(payload).hexdigest() == expected_old_sha:
                    with open(dst, 'wb') as f:
                        f.write(payload)
                    copied = True
                    break
        guard.need(
            copied and dst.is_file() and guard.digest(dst) == accepted_sha,
            'missing or modified unrecoverable generated source_home input: ' + rel + ' (expected sha256=' + str(expected_old_sha) + ')',
        )
        os.chmod(dst, 0o644)
        os.utime(dst, ns=(SOURCE_EPOCH_NS, SOURCE_EPOCH_NS), follow_symlinks=False)

    _set_tree_source_epoch(source_home)

    # 5. Run guard.verify_raw(c) and guard.freeze_source(c) to write C_RAW_IDENTITIES.json & D_FREEZE.json
    # and enforce read-only modes across source_home.
    guard.verify_raw(c)
    guard.freeze_source(c)

    new_d_freeze = guard.jload(packet_dir / 'D_FREEZE.json')
    new_d_files = new_d_freeze['files']
    guard.need(set(new_d_files.keys()) == set(old_d_files.keys()), 'reconstituted D_FREEZE path membership differs from preserved r3 D_FREEZE')
    allowed_dated_rels = {GLOBAL_CACHE_REL} | set(git_control_by_rel.keys())
    for path_key, old_val in old_d_files.items():
        if path_key not in allowed_dated_rels:
            guard.need(new_d_files[path_key] == old_val, 'source_home entry differs from preserved r3 D_FREEZE: ' + path_key)
    guard.need(new_d_files == expected_new_d_files, 'reconstituted D_FREEZE file map differs from expected dated bookkeeping map')
    expected_new_h_s = hashlib.sha256(guard.json_bytes(expected_new_d_files)).hexdigest()
    expected_new_d_sha = hashlib.sha256(guard.json_bytes({'H_S': expected_new_h_s, 'files': expected_new_d_files})).hexdigest()
    guard.need(new_d_freeze.get('H_S') == expected_new_h_s, 'reconstituted D_FREEZE H_S mismatch')
    guard.need(guard.digest(packet_dir / 'D_FREEZE.json') == expected_new_d_sha, 'reconstituted D_FREEZE.json digest mismatch')

    # 6. Copy and verify E_RESOLVED.json into packet_dir, confirming all host_units live inside source_home.
    shutil.copyfile(q_packet / 'E_RESOLVED.json', packet_dir / 'E_RESOLVED.json')
    os.chmod(packet_dir / 'E_RESOLVED.json', 0o444)
    guard.need(guard.digest(packet_dir / 'E_RESOLVED.json') == pins['E_RESOLVED.json'], 'copied E_RESOLVED.json digest mismatch')
    guard.need(isinstance(old_e_resolved, dict), 'invalid E_RESOLVED structure')
    if 'host_units' in old_e_resolved:
        sh_prefix = str(source_home) + '/'
        guard.need(
            all(isinstance(u, dict) and str(u.get('root', '')).startswith(sh_prefix) for u in old_e_resolved['host_units']),
            'E_RESOLVED host_unit root outside source_home',
        )

    reconstitution_report = {
        'preserved_d_freeze_sha256': pins['D_FREEZE.json'],
        'new_d_freeze_sha256': guard.digest(packet_dir / 'D_FREEZE.json'),
        'preserved_h_s': old_d_freeze.get('H_S'),
        'new_h_s': new_d_freeze.get('H_S'),
        'authenticated_source_files_verified': len(old_d_files) - len(generated_rels),
        'expected_generated_files_verified': len(generated_rels),
        'reconstructed_cargo_ok_count': len(reconstructed_cargo_ok),
        'reconstructed_cargo_ok': reconstructed_cargo_ok,
        'bookkeeping_hash_check_sha256': bookkeeping_check_sha,
        'git_control_snapshot_projection_sha256': git_control_projection_sha,
        'dated_bookkeeping_deltas': dated_bookkeeping_deltas,
        'source_epoch_ns': SOURCE_EPOCH_NS,
    }
    guard.receipt(c, 'SOURCE_HOME_RECONSTITUTION.json', reconstitution_report, 'D')
    return reconstitution_report


def _extract_copy_map_rows(copy_map_obj, guard):
    if isinstance(copy_map_obj, list):
        return copy_map_obj
    if isinstance(copy_map_obj, dict):
        for v in copy_map_obj.values():
            if isinstance(v, list) and v and isinstance(v[0], dict) and 'relative_path' in v[0]:
                return v
    guard.need(False, 'unrecognized PRESERVATION-EXACT-COPY-MAP structure')


def _verify_depinfo_file(dep_file, c, guard):
    worktree = c['paths']['worktree']
    source_home = Path(c['paths']['source_home'])
    target_root = Path(c['paths']['target'])
    dep_roots = (
        [source_home / g['relative_root'] for g in c.get('git_checkouts', ())]
        + [source_home / r['source_relative'] for r in c.get('registry_roots', ())]
        + [source_home, target_root, *(Path(t) for t in c['readonly_tools']), Path('/usr')]
    )
    text = Path(dep_file).read_text(encoding='utf-8')
    guard.need(LEGACY_WORKTREE not in text, 'third-party .d file references legacy worktree: ' + str(dep_file))
    for lhs_paths, rhs_paths in guard.parse_depinfo(text, worktree, dep_roots):
        for norm in lhs_paths | rhs_paths:
            guard.need(not norm.startswith(LEGACY_WORKTREE + '/'), 'third-party .d dependency resolves to legacy worktree: ' + norm)


def _preserved_seed_mtime_ns(rel, src_st, guard, mtime_range):
    """Return the preserved successful-r3 mtime of a quarantine source file; never a normalized tier."""
    mtime_ns = src_st.st_mtime_ns
    guard.need(mtime_ns > SOURCE_EPOCH_NS, 'preserved target mtime not newer than reconstituted source epoch: ' + rel)
    if mtime_range is not None:
        guard.need(
            mtime_range[0] <= mtime_ns <= mtime_range[1],
            'preserved target mtime outside verified r3 archive range: ' + rel,
        )
    return mtime_ns


def verify_and_seed_cached_target(
    c,
    guard,
    quarantine_root=PRESERVED_QUARANTINE_ROOT,
    copy_map_file=PRESERVED_COPY_MAP_PATH,
    disposition_file=PRESERVED_DISPOSITION_PATH,
    expected_counts=None,
):
    """Verify preserved quarantine + r3 audit receipts and seed only the 2,843 non-workspace target files."""
    counts_cfg = PRODUCTION_PARTITION_COUNTS if expected_counts is None else expected_counts
    q_root = Path(quarantine_root)
    q_target = q_root / 'target'
    q_packet = q_root / 'packet'

    copy_map_path = Path(copy_map_file)
    disp_path = Path(disposition_file)
    is_p9 = expected_counts is P9_PARTITION_COUNTS
    is_p10 = expected_counts is P10_PARTITION_COUNTS
    r3_outputs_path = q_packet / ('F_QUARANTINED_OUTPUTS.json' if (is_p9 or is_p10) else 'F_RETAINED_QUARANTINED_OUTPUTS_R3.json')
    r3_result_path = q_packet / ('F_CACHED_CURRENT_RESULT.json' if (is_p9 or is_p10) else 'F_RETAINED_AUDIT_R3_RESULT.json')

    for p in (copy_map_path, disp_path, r3_outputs_path, r3_result_path):
        guard.regular(p)

    if is_p9:
        guard.need(guard.digest(copy_map_path) == P9_COPY_MAP_SHA256, 'P9 preservation copy-map SHA-256 mismatch')
        guard.need(guard.digest(disp_path) == P9_PRESERVE_CLEANUP_RESULT_SHA256, 'P9 preservation disposition SHA-256 mismatch')
        guard.need(guard.digest(r3_outputs_path) == P9_PACKET_PINS['F_QUARANTINED_OUTPUTS.json'], 'P9 quarantined outputs SHA-256 mismatch')
        guard.need(guard.digest(r3_result_path) == P9_PACKET_PINS['F_CACHED_CURRENT_RESULT.json'], 'P9 cached-current result SHA-256 mismatch')
    if is_p10:
        guard.need(q_root == Path(P11_SEED_COPY_ROOT), 'P10-basis seed must read the fresh P11 read-only copy, never the original P10 quarantine')
        guard.need(
            copy_map_path == Path(P10_ROW_PROJECTION_PATH)
            and copy_map_path.stat().st_size == P10_ROW_PROJECTION_BYTES
            and guard.digest(copy_map_path) == P10_ROW_PROJECTION_SHA256,
            'P10 preservation row projection SHA-256 or byte size mismatch',
        )
        guard.need(
            disp_path == Path(P11_SEED_COPY_RECEIPT_PATH)
            and guard.HEX.fullmatch(str(P11_SEED_COPY_RECEIPT_SHA256)) is not None
            and type(P11_SEED_COPY_RECEIPT_BYTES) is int
            and disp_path.stat().st_size == P11_SEED_COPY_RECEIPT_BYTES
            and guard.digest(disp_path) == P11_SEED_COPY_RECEIPT_SHA256,
            'P11 read-only seed copy receipt pin unissued or mismatched',
        )
        guard.need(guard.digest(r3_outputs_path) == P10_QUARANTINED_OUTPUTS_SHA256, 'P10 quarantined outputs SHA-256 mismatch')
        guard.need(guard.digest(r3_result_path) == P10_F_RESULT_SHA256, 'P10 cached-current result SHA-256 mismatch')
    if expected_counts is None:
        guard.need(guard.digest(copy_map_path) == PRESERVED_COPY_MAP_SHA256, 'preservation copy-map SHA-256 mismatch')
        guard.need(guard.digest(disp_path) == PRESERVED_DISPOSITION_SHA256, 'preservation disposition SHA-256 mismatch')
        guard.need(guard.digest(r3_outputs_path) == PRESERVED_PACKET_PINS['F_RETAINED_QUARANTINED_OUTPUTS_R3.json'], 'r3 outputs SHA-256 mismatch')
        guard.need(guard.digest(r3_result_path) == PRESERVED_PACKET_PINS['F_RETAINED_AUDIT_R3_RESULT.json'], 'r3 result SHA-256 mismatch')

    copy_map_rows = _extract_copy_map_rows(guard.jload(copy_map_path), guard)
    guard.need(len(copy_map_rows) == counts_cfg['total_preserved_files'], 'preservation copy-map total file count mismatch')

    copy_target_meta = {}
    for row in copy_map_rows:
        rpath = row['relative_path']
        if rpath.startswith('target/'):
            trel = str(guard.relative(rpath[len('target/'):]))
            copy_target_meta[trel] = row

    r3_outputs = guard.jload(r3_outputs_path)
    r3_result = guard.jload(r3_result_path)
    if is_p9:
        guard.need(
            r3_result.get('route') == 'cached-current-f'
            and r3_result.get('base_head') == '539ab417d92cf7f4dfe8655fee810ea5d2a41c89'
            and r3_result.get('cached_target_seed_counts') == P9_HISTORICAL_SEED_PARTITION_COUNTS,
            'P9 result must bind the historical R3-basis seed counts',
        )
    elif is_p10:
        _verify_p10_f_result(r3_result, c, guard)
    else:
        guard.need(r3_result.get('native_outputs_accepted') is False, 'r3 result must keep native_outputs_accepted=False')

    all_outputs_map = r3_outputs['all_outputs']
    host_outputs_list = r3_outputs['host_outputs']
    guard.need(len(all_outputs_map) == counts_cfg['target_total'], 'preserved all_outputs count mismatch')
    guard.need(set(copy_target_meta.keys()) == set(all_outputs_map.keys()), 'copy-map target keys differ from r3 all_outputs')
    guard.need(len(host_outputs_list) == counts_cfg['host_outputs_total'], 'preserved host_outputs count mismatch')

    for k_non_debug, exp_sha in NON_DEBUG_CACHE_CANDIDATES.items():
        if k_non_debug in all_outputs_map or expected_counts is None:
            guard.need(all_outputs_map.get(k_non_debug) == exp_sha, 'non-debug cached output digest mismatch: ' + k_non_debug)

    # Verify backend ELF identity in preserved quarantine before excluding it.
    if is_p9:
        if CURRENT_P9_BACKEND_ELF_REL in all_outputs_map:
            elf_sha = all_outputs_map[CURRENT_P9_BACKEND_ELF_REL]
            elf_meta = copy_target_meta[CURRENT_P9_BACKEND_ELF_REL]
            guard.need(
                elf_sha == CURRENT_P9_BACKEND_ELF_SHA256 and elf_meta['sha256'] == CURRENT_P9_BACKEND_ELF_SHA256 and elf_meta['bytes'] == CURRENT_P9_BACKEND_ELF_BYTES,
                'current P9 backend ELF identity mismatch against 3a2f695515330e5a',
            )
    elif is_p10:
        guard.need(P10_BACKEND_ELF_REL in all_outputs_map, 'missing retired P10 backend ELF row in P10 all_outputs')
        elf_sha = all_outputs_map[P10_BACKEND_ELF_REL]
        elf_meta = copy_target_meta[P10_BACKEND_ELF_REL]
        guard.need(
            elf_sha == P10_BACKEND_ELF_SHA256 and elf_meta['sha256'] == P10_BACKEND_ELF_SHA256 and elf_meta['bytes'] == P10_BACKEND_ELF_BYTES,
            'retired P10 backend ELF identity mismatch against 3a2f695515330e5a',
        )
    elif OLD_BACKEND_ELF_REL in all_outputs_map or expected_counts is None:
        guard.need(OLD_BACKEND_ELF_REL in all_outputs_map, 'missing old backend ELF row in r3 all_outputs')
        elf_sha = all_outputs_map[OLD_BACKEND_ELF_REL]
        elf_meta = copy_target_meta[OLD_BACKEND_ELF_REL]
        guard.need(
            elf_sha == OLD_BACKEND_ELF_SHA256 and elf_meta['sha256'] == OLD_BACKEND_ELF_SHA256 and elf_meta['bytes'] == OLD_BACKEND_ELF_BYTES,
            'old backend ELF identity mismatch against 3a2f695515330e5a',
        )

    # Verify the 90 host_outputs rows (68 build-script binaries + 22 proc-macro .so files) using actual 'path' key.
    host_output_map = {}
    bs_bin_count = 0
    pm_so_count = 0
    for hrow in host_outputs_list:
        hrel = str(guard.relative(hrow['path']))
        guard.need(hrel not in host_output_map, 'duplicate host_outputs row: ' + hrel)
        guard.need(hrel in all_outputs_map, 'host_outputs row absent from all_outputs: ' + hrel)
        guard.need(hrow['sha256'] == all_outputs_map[hrel], 'host_outputs sha256 mismatch vs all_outputs: ' + hrel)
        if hrel.endswith('.so'):
            pm_so_count += 1
        elif hrel.startswith('debug/build/'):
            bs_bin_count += 1
        else:
            guard.need(False, 'unexpected host_output kind: ' + hrel)
        host_output_map[hrel] = hrow['sha256']

    guard.need(bs_bin_count == counts_cfg['host_build_script_binaries'], 'host build-script binary count mismatch')
    guard.need(pm_so_count == counts_cfg['host_proc_macro_dylibs'], 'host proc-macro dylib count mismatch')

    # Walk preserved q_target to ensure no extra or missing files and intact quarantine modes.
    actual_q_target = set()
    for base, dirs, files in os.walk(q_target, followlinks=False):
        bpath = Path(base)
        for d in dirs:
            guard.need(not (bpath / d).is_symlink(), 'symlink directory in preserved quarantine target')
        for f in files:
            p = bpath / f
            guard.regular(p)
            mode = stat.S_IMODE(p.lstat().st_mode)
            guard.need(mode in (0o400, 0o444), 'preserved quarantine file mode not read-only: ' + str(p))
            actual_q_target.add(str(p.relative_to(q_target)))
    guard.need(actual_q_target == set(all_outputs_map.keys()), 'preserved target directory membership mismatch')

    # Partition all target rows into workspace exclusions vs third-party cache candidates.
    partitions = {
        'owner_literal_workspace_fingerprint': [],
        'owner_literal_workspace_deps': [],
        'owner_literal_workspace_incremental': [],
        'unadopted_other_cache_candidate': [],
    }
    for rel in sorted(all_outputs_map.keys()):
        cat = classify_preserved_target_rel(rel, guard)
        partitions[cat].append(rel)

    for cat in (
        'owner_literal_workspace_fingerprint',
        'owner_literal_workspace_deps',
        'owner_literal_workspace_incremental',
        'unadopted_other_cache_candidate',
    ):
        guard.need(len(partitions[cat]) == counts_cfg[cat], 'target partition count mismatch for ' + cat)

    # Ensure zero host_outputs belong to excluded workspace rows.
    candidate_set = set(partitions['unadopted_other_cache_candidate'])
    guard.need(set(host_output_map.keys()).issubset(candidate_set), 'host_output unexpectedly classified as workspace row')

    # Seed only unadopted_other_cache_candidate rows into fresh c['paths']['target'].
    dst_target = Path(c['paths']['target'])
    guard.need(dst_target.is_dir() and not any(dst_target.iterdir()), 'target must be fresh and empty before cache seeding')

    seeded_files = {}
    run_unit_snapshots = {}
    seeded_depinfo_paths = []
    for rel in partitions['unadopted_other_cache_candidate']:
        exp_sha = all_outputs_map[rel]
        cmeta = copy_target_meta[rel]
        guard.need(cmeta['sha256'] == exp_sha, 'copy-map sha256 mismatch vs all_outputs: ' + rel)
        src_p = q_target / rel
        guard.regular(src_p)
        src_st_before = src_p.lstat()
        src_sha = guard.digest(src_p)
        guard.need(src_sha == exp_sha and src_st_before.st_size == cmeta['bytes'], 'quarantine source hash/size mismatch: ' + rel)

        dst_p = dst_target / rel
        dst_p.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(src_p, dst_p)
        is_exec = rel in host_output_map
        os.chmod(dst_p, 0o755 if is_exec else 0o644)
        # Reapply the preserved successful-r3 mtime so dependency/output order matches the build that produced it.
        mtime_ns = _preserved_seed_mtime_ns(rel, src_st_before, guard, R3_ARCHIVE_MTIME_RANGE_NS if expected_counts is None else None)
        os.utime(dst_p, ns=(mtime_ns, mtime_ns), follow_symlinks=False)
        guard.need(guard.digest(dst_p) == exp_sha, 'seeded target copy hash mismatch: ' + rel)

        if rel.endswith('.d'):
            seeded_depinfo_paths.append(dst_p)

        # Confirm quarantine source file mode, inode and preserved mtime were not mutated.
        src_st_after = src_p.lstat()
        guard.need(
            src_st_after.st_ino == src_st_before.st_ino
            and stat.S_IMODE(src_st_after.st_mode) == stat.S_IMODE(src_st_before.st_mode)
            and src_st_after.st_mtime_ns == src_st_before.st_mtime_ns,
            'quarantine source mutated during seeding: ' + rel,
        )

        dst_st = dst_p.lstat()
        guard.need(dst_st.st_mtime_ns == mtime_ns, 'seeded target mtime differs from preserved r3 mtime: ' + rel)
        seeded_files[rel] = {
            'sha256': exp_sha,
            'bytes': cmeta['bytes'],
            'mode': oct(stat.S_IMODE(dst_st.st_mode)),
            'mtime_ns': dst_st.st_mtime_ns,
            'ino': dst_st.st_ino,
        }

        m = BUILD_UNIT_RE.fullmatch(rel)
        if m and m.group(2) in ('invoked.timestamp', 'output', 'root-output', 'stderr'):
            unit_dir = m.group(1)
            run_unit_snapshots.setdefault(unit_dir, {})[m.group(2)] = {
                'relative': rel,
                'sha256': exp_sha,
                'mtime_ns': dst_st.st_mtime_ns,
                'ino': dst_st.st_ino,
            }

    for dep_p in seeded_depinfo_paths:
        _verify_depinfo_file(dep_p, c, guard)

    guard.need(len(run_unit_snapshots) == counts_cfg['build_script_run_units'], 'build-script run-unit snapshot count mismatch')
    for unit_dir, markers in run_unit_snapshots.items():
        guard.need(
            {'invoked.timestamp', 'output', 'root-output', 'stderr'}.issubset(markers.keys()),
            'incomplete build-script run-unit markers in: ' + unit_dir,
        )

    seed_snapshot = {
        'quarantine_root': str(q_root),
        'preserved_copy_map_sha256': guard.digest(copy_map_path),
        'r3_quarantined_outputs_sha256': guard.digest(r3_outputs_path),
        'partition_counts': {k: len(v) for k, v in partitions.items()},
        'excluded_workspace_paths': (
            partitions['owner_literal_workspace_fingerprint']
            + partitions['owner_literal_workspace_deps']
            + partitions['owner_literal_workspace_incremental']
        ),
        'host_outputs_seeded': host_output_map,
        'run_unit_snapshots': run_unit_snapshots,
        'seeded_files': seeded_files,
    }
    guard.receipt(
        c,
        'CACHED_TARGET_SEED.json',
        {
            'quarantine_root': seed_snapshot['quarantine_root'],
            'preserved_copy_map_sha256': seed_snapshot['preserved_copy_map_sha256'],
            'r3_quarantined_outputs_sha256': seed_snapshot['r3_quarantined_outputs_sha256'],
            'partition_counts': seed_snapshot['partition_counts'],
            'excluded_workspace_count': len(seed_snapshot['excluded_workspace_paths']),
            'seeded_count': len(seeded_files),
            'host_outputs_count': len(host_output_map),
            'build_script_run_units_count': len(run_unit_snapshots),
            'host_outputs_seeded': host_output_map,
            'run_unit_snapshots': run_unit_snapshots,
            'seeded_files': seeded_files,
        },
        'F',
    )
    return seed_snapshot


def verify_and_seed_cached_target_p9(c, guard):
    """Seed the new source from P9 quarantine, classifying P9 outputs rather than the historical seed packet."""
    return verify_and_seed_cached_target(
        c,
        guard,
        quarantine_root=P9_PERSISTENT_QUARANTINE_ROOT,
        copy_map_file=P9_COPY_MAP_PATH,
        disposition_file=P9_PRESERVE_CLEANUP_RESULT_PATH,
        expected_counts=P9_PARTITION_COUNTS,
    )




def _verify_p10_f_result(f_res, c, guard):
    """Bind the retired P10 F result: P9-basis producer seed, unaccepted P10 ELF, exact streams and unchanged Cargo paths."""
    f_result = f_res.get('f_result') if isinstance(f_res, dict) else None
    start = f_res.get('historical_debit_state_start') if isinstance(f_res, dict) else None
    env = f_result.get('environment') if isinstance(f_result, dict) else None
    current_env = guard.cargo_env(c, 'F')
    guard.need(
        isinstance(f_res, dict)
        and f_res.get('phase') == 'F'
        and f_res.get('route') == 'cached-current-f'
        and f_res.get('base_head') == P10_BASE_HEAD
        and f_res.get('cargo_lock_sha256') == guard.LOCK
        and f_res.get('cached_target_seed_counts') == P10_PRODUCER_SEED_PARTITION_COUNTS
        and f_res.get('post_compile_check') == P10_POST_COMPILE_CHECK
        # P10 started from exactly the post-P9 totals, so its own E/S/P are a disjoint once-only layer.
        and isinstance(start, dict)
        and start.get('historical_debits') == HISTORICAL_POST_R3_DEBITS
        and isinstance(f_result, dict)
        and f_result.get('phase') == 'F'
        and f_result.get('log_bytes') == {'stdout': P10_F_STDOUT_BYTES, 'stderr': P10_F_STDERR_BYTES}
        and type(f_result.get('wall_seconds')) is float
        and 0 < f_result['wall_seconds'] <= P10_NATIVE_OUTER_SECONDS
        # Seeded dep-info and fingerprints are path-bound: Cargo home and target dir must be unchanged.
        and isinstance(env, dict)
        and env.get('CARGO_HOME') == current_env['CARGO_HOME']
        and env.get('CARGO_TARGET_DIR') == current_env['CARGO_TARGET_DIR'],
        'P10 cached-current F result receipt content mismatch',
    )


def _verify_p11_readonly_seed_copy(guard):
    """Verify the fresh P11 read-only copy of the complete retired P10 target, packet and marker before any seeding."""
    guard.need(
        guard.HEX.fullmatch(str(P11_SEED_COPY_RECEIPT_SHA256)) is not None and type(P11_SEED_COPY_RECEIPT_BYTES) is int,
        'P11 read-only seed copy receipt pin is still unissued',
    )
    rec_p = Path(P11_SEED_COPY_RECEIPT_PATH)
    guard.regular(rec_p)
    guard.need(
        rec_p.stat().st_size == P11_SEED_COPY_RECEIPT_BYTES and guard.digest(rec_p) == P11_SEED_COPY_RECEIPT_SHA256,
        'P11 read-only seed copy receipt SHA-256 or byte size mismatch',
    )
    proj_p = Path(P10_ROW_PROJECTION_PATH)
    guard.regular(proj_p)
    guard.need(
        proj_p.stat().st_size == P10_ROW_PROJECTION_BYTES and guard.digest(proj_p) == P10_ROW_PROJECTION_SHA256,
        'P10 preservation row projection SHA-256 or byte size mismatch',
    )
    rec = guard.jload(rec_p)
    guard.exact(
        rec,
        'recordType source_root copy_root source_manifest row_projection file_count total_bytes files source_stat_stable directories_fsynced native_outputs_accepted authority_granted_by_this_file_alone',
        'P11 read-only seed copy receipt',
    )
    guard.need(
        rec['recordType'] == 'P11ReadonlySeedCopyReceipt'
        and rec['source_root'] == P10_PERSISTENT_QUARANTINE_ROOT
        and rec['copy_root'] == P11_SEED_COPY_ROOT
        and rec['source_manifest'] == {'path': P10_ORIGINAL_MANIFEST_PATH, 'bytes': P10_ORIGINAL_MANIFEST_BYTES, 'sha256': P10_ORIGINAL_MANIFEST_SHA256}
        and rec['row_projection'] == {'path': P10_ROW_PROJECTION_PATH, 'bytes': P10_ROW_PROJECTION_BYTES, 'sha256': P10_ROW_PROJECTION_SHA256}
        and rec['file_count'] == P10_PARTITION_COUNTS['total_preserved_files']
        and rec['total_bytes'] == P11_SEED_COPY_TOTAL_BYTES
        and rec['source_stat_stable'] is True
        and rec['directories_fsynced'] is True
        and rec['native_outputs_accepted'] is False
        and rec['authority_granted_by_this_file_alone'] is False
        and isinstance(rec['files'], list),
        'P11 read-only seed copy receipt header mismatch',
    )
    rows = {}
    total = 0
    for row in rec['files']:
        guard.exact(row, 'relative_path sha256 bytes source_mode copy_mode source_mtime_ns copy_mtime_ns fsynced', 'P11 seed copy row')
        guard.need(type(row['relative_path']) is str, 'P11 seed copy row path type')
        rel = str(guard.relative(row['relative_path']))
        guard.need(rel not in rows, 'duplicate P11 seed copy row: ' + rel)
        guard.need(
            type(row['sha256']) is str and guard.HEX.fullmatch(row['sha256']) is not None
            and type(row['bytes']) is int and row['bytes'] >= 0
            # Original P10 modes stay exactly as preserved per path (0644, 0600 or 0444); only the copy is read-only and never executable.
            and type(row['source_mode']) is int and row['source_mode'] == P10_ORIGINAL_NON_0644_SOURCE_MODES.get(rel, 0o644)
            and row['copy_mode'] in (0o400, 0o444)
            and type(row['source_mtime_ns']) is int
            and type(row['copy_mtime_ns']) is int
            and row['source_mtime_ns'] == row['copy_mtime_ns']
            and row['fsynced'] is True,
            'P11 seed copy row hash/mode/mtime/fsync invalid: ' + rel,
        )
        rows[rel] = row
        total += row['bytes']
    guard.need(
        len(rows) == P10_PARTITION_COUNTS['total_preserved_files'] and total == P11_SEED_COPY_TOTAL_BYTES,
        'P11 seed copy receipt membership or byte total mismatch',
    )
    guard.need(set(P10_ORIGINAL_NON_0644_SOURCE_MODES) <= set(rows), 'P10 non-0644 original mode row missing from P11 seed copy receipt')
    proj = {}
    for prow in _extract_copy_map_rows(guard.jload(proj_p), guard):
        guard.need(isinstance(prow, dict) and type(prow.get('relative_path')) is str, 'P10 projection row invalid')
        guard.need(prow['relative_path'] not in proj, 'duplicate P10 projection row: ' + prow['relative_path'])
        proj[prow['relative_path']] = prow
    guard.need(set(proj) == set(rows), 'P11 seed copy membership differs from P10 preservation row projection')
    for rel, row in rows.items():
        guard.need(
            proj[rel].get('sha256') == row['sha256'] and proj[rel].get('bytes') == row['bytes']
            and proj[rel].get('source_mode_octal') == '%o' % row['source_mode'],
            'P11 seed copy row differs from P10 projection: ' + rel,
        )
    copy_root = Path(P11_SEED_COPY_ROOT)
    files, links, _ = guard.walk_tree(copy_root)
    guard.need(not links, 'symlink in P11 read-only seed copy')
    actual = {str(p.relative_to(copy_root)): p for p in files}
    guard.need(set(actual) == set(rows), 'P11 read-only seed copy on-disk membership differs from receipt')
    for rel, p in actual.items():
        st = p.lstat()
        guard.need(
            stat.S_IMODE(st.st_mode) == rows[rel]['copy_mode']
            and st.st_size == rows[rel]['bytes']
            and st.st_mtime_ns == rows[rel]['copy_mtime_ns'],
            'P11 read-only seed copy mode/size/mtime differs from receipt: ' + rel,
        )
    target_rels = sorted(r for r in rows if r.startswith('target/'))
    packet_rels = sorted(r for r in rows if r.startswith('packet/'))
    other_rels = sorted(r for r in rows if not r.startswith(('target/', 'packet/')))
    guard.need(
        len(target_rels) == P10_PARTITION_COUNTS['target_total']
        and len(packet_rels) == P10_PARTITION_COUNTS['packet_total']
        and other_rels == [P10_MARKER_REL],
        'P11 read-only seed copy target/packet/marker partition mismatch',
    )
    for rel in packet_rels + other_rels:
        guard.need(guard.digest(actual[rel]) == rows[rel]['sha256'], 'P11 read-only seed copy packet/marker hash mismatch: ' + rel)
    guard.need(
        rows[P10_MARKER_REL]['sha256'] == P7_MARKER_SHA256 and rows[P10_MARKER_REL]['bytes'] == P7_MARKER_BYTES,
        'P10 one-shot marker content mismatch',
    )
    seed_rel = 'packet/CACHED_TARGET_SEED.json'
    guard.need(seed_rel in rows, 'P10 producer seed packet missing from P11 copy')
    producer = guard.jload(actual[seed_rel])
    seeded = producer.get('seeded_files') if isinstance(producer, dict) else None
    guard.need(
        isinstance(producer, dict)
        and producer.get('quarantine_root') == P9_PERSISTENT_QUARANTINE_ROOT
        and producer.get('preserved_copy_map_sha256') == P9_COPY_MAP_SHA256
        and producer.get('r3_quarantined_outputs_sha256') == P9_PACKET_PINS['F_QUARANTINED_OUTPUTS.json']
        and producer.get('partition_counts') == P10_PRODUCER_SEED_PARTITION_COUNTS
        and producer.get('seeded_count') == P10_PRODUCER_SEED_PARTITION_COUNTS['unadopted_other_cache_candidate']
        and isinstance(seeded, dict),
        'P10 producer seed packet provenance mismatch',
    )
    # The reused cache must be exactly what P10 seeded and verified: same members, bytes and mtimes.
    candidates = {
        rel[len('target/'):]
        for rel in target_rels
        if classify_preserved_target_rel(rel[len('target/'):], guard) == 'unadopted_other_cache_candidate'
    }
    guard.need(
        len(candidates) == P10_PARTITION_COUNTS['unadopted_other_cache_candidate'] and candidates == set(seeded),
        'P10 third-party cache candidates differ from the P10 producer seed membership',
    )
    for trel in sorted(candidates):
        meta = seeded[trel]
        row = rows['target/' + trel]
        guard.need(
            isinstance(meta, dict) and meta.get('sha256') == row['sha256'] and meta.get('mtime_ns') == row['copy_mtime_ns'],
            'P10 third-party cache candidate changed hash or mtime after P10 seeding: ' + trel,
        )
    return rows


def verify_and_seed_cached_target_p11(c, guard):
    """Seed the frozen-WW compile from the read-only copy of the retired P10 target; workspace rows and the P10 ELF stay excluded."""
    _verify_p11_readonly_seed_copy(guard)
    return verify_and_seed_cached_target(
        c,
        guard,
        quarantine_root=P11_SEED_COPY_ROOT,
        copy_map_file=P10_ROW_PROJECTION_PATH,
        disposition_file=P11_SEED_COPY_RECEIPT_PATH,
        expected_counts=P10_PARTITION_COUNTS,
    )
def verify_post_compile_no_script_reexec(c, guard, seed_snapshot=None):
    """Verify after Phase F that no third-party build script or proc-macro host output was re-executed or rewritten."""
    if seed_snapshot is None:
        seed_snapshot = guard.jload(Path(c['paths']['packet']) / 'CACHED_TARGET_SEED.json')
    dst_target = Path(c['paths']['target'])

    # 1. Verify all 90 host_outputs kept their exact pre-compile SHA-256 and mtime_ns.
    for hrel, exp_sha in seed_snapshot['host_outputs_seeded'].items():
        p = dst_target / hrel
        guard.regular(p)
        guard.need(guard.digest(p) == exp_sha, 'third-party host output was rewritten during cached Phase F: ' + hrel)
        pre_meta = seed_snapshot['seeded_files'][hrel]
        st = p.lstat()
        guard.need(st.st_mtime_ns == pre_meta['mtime_ns'], 'third-party host output mtime changed during cached Phase F: ' + hrel)

    # 2. Verify all 34 build-script run-unit marker files kept their exact pre-compile SHA-256, mtime_ns, and inode,
    # and no new debug/build/<unit> directories were created.
    current_build_root = dst_target / 'debug' / 'build'
    if current_build_root.is_dir():
        actual_build_dirs = {d.name for d in current_build_root.iterdir() if d.is_dir()}
        expected_build_dirs = {
            rel.split('/')[2]
            for rel in seed_snapshot['seeded_files']
            if rel.startswith('debug/build/') and len(rel.split('/')) >= 4
        }
        guard.need(actual_build_dirs == expected_build_dirs, 'unexpected new build-script directories created during cached Phase F')

    for unit_dir, markers in seed_snapshot['run_unit_snapshots'].items():
        for mname, pre in markers.items():
            p = dst_target / pre['relative']
            guard.regular(p)
            st = p.lstat()
            guard.need(guard.digest(p) == pre['sha256'], 'third-party build-script marker content changed: ' + pre['relative'])
            guard.need(
                st.st_mtime_ns == pre['mtime_ns'] and st.st_ino == pre['ino'],
                'third-party build-script was re-executed (timestamp/inode changed): ' + pre['relative'],
            )

    # 3. Verify that old excluded backend binary 3a2f695515330e5a is absent or distinct and a fresh mori_backend test binary exists.
    deps_dir = dst_target / 'debug' / 'deps'
    fresh_backend_bins = []
    if deps_dir.is_dir():
        for p in sorted(deps_dir.iterdir()):
            if p.is_file() and not p.is_symlink() and p.name.startswith('mori_backend-') and '.' not in p.name:
                rel = 'debug/deps/' + p.name
                sha = guard.digest(p)
                guard.need(
                    not (rel == OLD_BACKEND_ELF_REL and sha == OLD_BACKEND_ELF_SHA256),
                    'cached Phase F produced stale un-rebuilt old backend ELF 3a2f695515330e5a',
                )
                guard.need(
                    not (rel == CURRENT_P9_BACKEND_ELF_REL and sha == CURRENT_P9_BACKEND_ELF_SHA256),
                    'cached Phase F produced stale un-rebuilt current P9 backend ELF 3a2f695515330e5a',
                )
                guard.need(
                    not (rel == P10_BACKEND_ELF_REL and sha == P10_BACKEND_ELF_SHA256),
                    'cached Phase F produced stale un-rebuilt retired P10 backend ELF 3a2f695515330e5a',
                )
                fresh_backend_bins.append({'relative': rel, 'sha256': sha, 'bytes': p.stat().st_size})
    guard.need(len(fresh_backend_bins) >= 1, 'cached Phase F did not produce a fresh mori_backend test executable')

    report = {
        'host_outputs_verified_unchanged': len(seed_snapshot['host_outputs_seeded']),
        'build_script_run_units_verified_not_reexecuted': len(seed_snapshot['run_unit_snapshots']),
        'fresh_mori_backend_executables': fresh_backend_bins,
    }
    pkt_check = Path(c['paths']['packet']) / 'CACHED_POST_COMPILE_CHECK.json'
    if not pkt_check.exists():
        guard.receipt(c, 'CACHED_POST_COMPILE_CHECK.json', report, 'F')
    return report


def validate_standing_beta_tier_grant(c, guard, records, tier):
    """Delegate directly to guard._validate_tier_grant so production and helper share one implementation."""
    return guard._validate_tier_grant(c, records, tier)


def enforce_post_r3_historical_debits(c, guard, cleanup_receipt_file=POST_CLEANUP_RESULT_PATH, at_start=True):
    """Carry forward post-r3 and retired P2 through P8 debits once, enforcing remaining ceilings."""
    debits = HISTORICAL_POST_R3_DEBITS
    if cleanup_receipt_file is not None:
        cleanup_p = Path(cleanup_receipt_file)
        guard.regular(cleanup_p)
        guard.need(guard.digest(cleanup_p) == POST_CLEANUP_RESULT_SHA256, 'post-cleanup receipt SHA-256 mismatch')
        p3_cnt_p = Path(P3_COUNTERS_DISPOSITION_PATH)
        guard.regular(p3_cnt_p)
        guard.need(guard.digest(p3_cnt_p) == P3_COUNTERS_DISPOSITION_SHA256, 'P3 counters disposition SHA-256 mismatch')
        cnt_obj = guard.jload(p3_cnt_p)
        cgs = cnt_obj.get('current_cgroups', {}) if isinstance(cnt_obj, dict) else {}
        guard.need(
            isinstance(cnt_obj, dict)
            and cnt_obj.get('one_shot_marker_exists') is True
            and cnt_obj.get('cargo_F_never_started') is True
            and cnt_obj.get('remaining_total_wall_seconds') == 6526
            and cnt_obj.get('retained_F_wall_seconds') == 2522
            and 'usage_usec 13455030\n' in str(cgs.get('envelope', {}).get('cpu.stat', ''))
            and 'usage_usec 13455030\n' in str(cgs.get('supervisor', {}).get('cpu.stat', ''))
            and 'usage_usec 0\n' in str(cgs.get('phases', {}).get('cpu.stat', '')),
            'P3 counters disposition content mismatch against historical debits',
        )
        p3_cln_p = Path(P3_PRESERVE_CLEANUP_RESULT_PATH)
        guard.regular(p3_cln_p)
        guard.need(guard.digest(p3_cln_p) == P3_PRESERVE_CLEANUP_RESULT_SHA256, 'P3 preserve/cleanup receipt SHA-256 mismatch')
        cln_obj = guard.jload(p3_cln_p)
        guard.need(
            isinstance(cln_obj, dict)
            and cln_obj.get('status') == 'PRESERVED_AND_CLEANED'
            and cln_obj.get('cleanupOwed') is False
            and cln_obj.get('native_outputs_accepted') is False
            and cln_obj.get('cumulative_CPU_preserved_in_Root_record') == P3_COUNTERS_DISPOSITION_PATH,
            'P3 preserve/cleanup receipt status invalid',
        )
        p4_cnt_p = Path(P4_COUNTERS_BEFORE_RETIRE_PATH)
        guard.regular(p4_cnt_p)
        guard.need(
            p4_cnt_p.stat().st_size == P4_COUNTERS_BEFORE_RETIRE_BYTES
            and guard.digest(p4_cnt_p) == P4_COUNTERS_BEFORE_RETIRE_SHA256,
            'P4 counters before-retire receipt SHA-256 or byte size mismatch',
        )
        p4_cnt_obj = guard.jload(p4_cnt_p)
        p4_cgs = p4_cnt_obj.get('currentCgroups', {}) if isinstance(p4_cnt_obj, dict) else {}
        guard.need(
            isinstance(p4_cnt_obj, dict)
            and p4_cnt_obj.get('cargoNeverStarted') is True
            and p4_cnt_obj.get('markerExisted') is True
            and p4_cnt_obj.get('priorLogicalDebits') == {'envelope': 359653742, 'supervisor': 89370060, 'phases': 270283681}
            and p4_cnt_obj.get('retiredP4UsageUsec') == {'envelope': 13735290, 'supervisor': 13735290, 'phases': 0}
            and p4_cnt_obj.get('nextLogicalDebits') == {
                'envelope': 373389032,
                'supervisor': 103105350,
                'phases': 270283681,
            }
            and p4_cnt_obj.get('nextWholeWallSeconds') == 6511
            and p4_cnt_obj.get('FwallSeconds') == 2522
            and p4_cnt_obj.get('nextSupervisorCPUSeconds') == 10696
            and 'usage_usec 13735290\n' in str(p4_cgs.get('envelope', {}).get('cpu.stat', ''))
            and 'usage_usec 13735290\n' in str(p4_cgs.get('supervisor', {}).get('cpu.stat', ''))
            and 'usage_usec 0\n' in str(p4_cgs.get('phases', {}).get('cpu.stat', '')),
            'P4 counters receipt content mismatch against historical debits',
        )
        p4_cln_p = Path(P4_PRESERVE_CLEANUP_RESULT_PATH)
        guard.regular(p4_cln_p)
        guard.need(
            p4_cln_p.stat().st_size == P4_PRESERVE_CLEANUP_RESULT_BYTES
            and guard.digest(p4_cln_p) == P4_PRESERVE_CLEANUP_RESULT_SHA256,
            'P4 preserve/cleanup receipt SHA-256 or byte size mismatch',
        )
        p4_cln_obj = guard.jload(p4_cln_p)
        guard.need(
            isinstance(p4_cln_obj, dict)
            and p4_cln_obj.get('status') == 'P4_PRESERVED_AND_CLEANED'
            and p4_cln_obj.get('cleanupOwed') is False
            and p4_cln_obj.get('nativeAccepted') is False
            and p4_cln_obj.get('cargoNeverStarted') is True
            and p4_cln_obj.get('counters') == {
                'path': P4_COUNTERS_BEFORE_RETIRE_PATH,
                'bytes': P4_COUNTERS_BEFORE_RETIRE_BYTES,
                'sha256': P4_COUNTERS_BEFORE_RETIRE_SHA256,
            },
            'P4 preserve/cleanup receipt status invalid',
        )
        p5_res_p = Path(P5_ACTUAL_RESULT_PATH)
        guard.regular(p5_res_p)
        guard.need(
            p5_res_p.stat().st_size == P5_ACTUAL_RESULT_BYTES
            and guard.digest(p5_res_p) == P5_ACTUAL_RESULT_SHA256,
            'P5 actual compile result receipt SHA-256 or byte size mismatch',
        )
        p5_cnt_p = Path(P5_COUNTERS_BEFORE_RETIRE_PATH)
        guard.regular(p5_cnt_p)
        guard.need(
            p5_cnt_p.stat().st_size == P5_COUNTERS_BEFORE_RETIRE_BYTES
            and guard.digest(p5_cnt_p) == P5_COUNTERS_BEFORE_RETIRE_SHA256,
            'P5 counters before-retire receipt SHA-256 or byte size mismatch',
        )
        p5_cnt_obj = guard.jload(p5_cnt_p)
        p5_nodes = p5_cnt_obj.get('nodes', {}) if isinstance(p5_cnt_obj, dict) else {}
        guard.need(
            isinstance(p5_cnt_obj, dict)
            and p5_cnt_obj.get('CargoNeverStarted') is True
            and p5_cnt_obj.get('targetEmpty') is True
            and p5_cnt_obj.get('sourceHomeReconstitutionPassed') is True
            and p5_cnt_obj.get('priorLogicalDebitsUsec') == {'E': 373389032, 'S': 103105350, 'P': 270283681}
            and p5_cnt_obj.get('retiredP5UsageUsec') == {
                'E': debits['retired_p5_envelope_usec'],
                'S': debits['retired_p5_supervisor_usec'],
                'P': debits['retired_p5_phases_usec'],
            }
            and p5_cnt_obj.get('nextLogicalDebitsUsec') == {
                'E': 411252727,
                'S': 140969045,
                'P': 270283681,
            }
            and p5_cnt_obj.get('P5wallDebitCeil') == debits['p5_refusal_wall_seconds_ceil']
            and p5_cnt_obj.get('nextWholeWallMax') == 6471
            and p5_cnt_obj.get('FwallMax') == 2522
            and p5_cnt_obj.get('supervisorCPUMax') == 10659
            and 'usage_usec 37863695\n' in str(p5_nodes.get('E', {}).get('cpu.stat', ''))
            and 'usage_usec 37863695\n' in str(p5_nodes.get('S', {}).get('cpu.stat', ''))
            and 'usage_usec 0\n' in str(p5_nodes.get('P', {}).get('cpu.stat', '')),
            'P5 counters receipt content mismatch against historical debits',
        )
        p5_cln_p = Path(P5_PRESERVE_CLEANUP_RESULT_PATH)
        guard.regular(p5_cln_p)
        guard.need(
            p5_cln_p.stat().st_size == P5_PRESERVE_CLEANUP_RESULT_BYTES
            and guard.digest(p5_cln_p) == P5_PRESERVE_CLEANUP_RESULT_SHA256,
            'P5 preserve/cleanup receipt SHA-256 or byte size mismatch',
        )
        p5_cln_obj = guard.jload(p5_cln_p)
        guard.need(
            isinstance(p5_cln_obj, dict)
            and p5_cln_obj.get('status') == 'P5_PRESERVED_AND_CLEANED'
            and p5_cln_obj.get('cleanupOwed') is False
            and p5_cln_obj.get('counterRecord') == {
                'path': P5_COUNTERS_BEFORE_RETIRE_PATH,
                'bytes': P5_COUNTERS_BEFORE_RETIRE_BYTES,
                'sha256': P5_COUNTERS_BEFORE_RETIRE_SHA256,
            }
            and p5_cln_obj.get('nextLogicalDebitsUsec') == {
                'E': 411252727,
                'S': 140969045,
                'P': 270283681,
            }
            and p5_cln_obj.get('wholeWallMax') == 6471
            and p5_cln_obj.get('FwallMax') == 2522
            and p5_cln_obj.get('supervisorCPUMax') == 10659,
            'P5 preserve/cleanup receipt status invalid',
        )
        p6_qmf_p = Path(P6_QUARANTINE_MANIFEST_PATH)
        guard.regular(p6_qmf_p)
        guard.need(
            p6_qmf_p.stat().st_size == P6_QUARANTINE_MANIFEST_BYTES
            and guard.digest(p6_qmf_p) == P6_QUARANTINE_MANIFEST_SHA256,
            'P6 quarantine manifest SHA-256 or byte size mismatch',
        )
        p6_out_p = Path(P6_COMPILE_STDOUT_PATH)
        guard.regular(p6_out_p)
        guard.need(
            p6_out_p.stat().st_size == P6_COMPILE_STDOUT_BYTES
            and guard.digest(p6_out_p) == P6_COMPILE_STDOUT_SHA256,
            'P6 compile stdout stream SHA-256 or byte size mismatch',
        )
        p6_cnt_p = Path(P6_DEBITS_BEFORE_RETIRE_PATH)
        guard.regular(p6_cnt_p)
        guard.need(
            p6_cnt_p.stat().st_size == P6_DEBITS_BEFORE_RETIRE_BYTES
            and guard.digest(p6_cnt_p) == P6_DEBITS_BEFORE_RETIRE_SHA256,
            'P6 debits before-retire receipt SHA-256 or byte size mismatch',
        )
        p6_cnt_obj = guard.jload(p6_cnt_p)
        p6_raw = p6_cnt_obj.get('rawCgroupTerminal', {}) if isinstance(p6_cnt_obj, dict) else {}
        guard.need(
            isinstance(p6_cnt_obj, dict)
            and p6_cnt_obj.get('CargoNeverStarted') is True
            and p6_cnt_obj.get('targetFilesPreserved') == 2171
            and p6_cnt_obj.get('allFilesPreserved') == 2177
            and p6_cnt_obj.get('quarantine') == P6_PERSISTENT_QUARANTINE_ROOT
            and p6_cnt_obj.get('nativeOutputsAccepted') is False
            and p6_cnt_obj.get('priorLogicalDebitsUsec') == {'E': 411252727, 'S': 140969045, 'P': 270283681}
            and p6_cnt_obj.get('retiredP6UsageUsec') == {
                'E': debits['retired_p6_envelope_usec'],
                'S': debits['retired_p6_supervisor_usec'],
                'P': debits['retired_p6_phases_usec'],
            }
            # Frozen post-P6 values; never compared against the changed post-P7 totals.
            and p6_cnt_obj.get('nextLogicalDebitsUsec') == {'E': 450272603, 'S': 179988921, 'P': 270283681}
            and p6_cnt_obj.get('P6wallDebitCeil') == debits['p6_refusal_wall_seconds_ceil']
            and p6_cnt_obj.get('nextWholeWallMax') == 6427
            and p6_cnt_obj.get('FwallMax') == 2522
            and p6_cnt_obj.get('supervisorCPUMax') == 10620
            and 'usage_usec 39019876\n' in str(p6_raw.get('E', {}).get('cpu.stat', ''))
            and 'usage_usec 39019876\n' in str(p6_raw.get('S', {}).get('cpu.stat', ''))
            and 'usage_usec 0\n' in str(p6_raw.get('P', {}).get('cpu.stat', ''))
            and str(p6_raw.get('S', {}).get('memory.peak', '')).strip() == '1610612736'
            and 'oom_kill 1\n' in str(p6_raw.get('S', {}).get('memory.events.local', ''))
            and str(p6_raw.get('P', {}).get('memory.peak', '')).strip() == '0',
            'P6 debits receipt content mismatch against historical debits',
        )
        p6_cln_p = Path(P6_PRESERVE_CLEANUP_RESULT_PATH)
        guard.regular(p6_cln_p)
        p6_cln_obj = guard.jload(p6_cln_p)
        p6_steps = p6_cln_obj.get('steps', []) if isinstance(p6_cln_obj, dict) else []
        guard.need(
            isinstance(p6_cln_obj, dict)
            and p6_cln_obj.get('status') == 'P6_PRESERVED_AND_CLEANED'
            and p6_cln_obj.get('cleanupOwed') is False
            and p6_cln_obj.get('quarantineManifest') == {
                'path': P6_QUARANTINE_MANIFEST_PATH,
                'bytes': P6_QUARANTINE_MANIFEST_BYTES,
                'sha256': P6_QUARANTINE_MANIFEST_SHA256,
            }
            and p6_cln_obj.get('debitReceipt') == {
                'path': P6_DEBITS_BEFORE_RETIRE_PATH,
                'bytes': P6_DEBITS_BEFORE_RETIRE_BYTES,
                'sha256': P6_DEBITS_BEFORE_RETIRE_SHA256,
            }
            and isinstance(p6_steps, list)
            and len(p6_steps) == 2
            and p6_steps[0].get('id') == 'collect-evidence'
            and p6_steps[0].get('actual_exit') == 0
            and p6_steps[0].get('streams', [{}])[0].get('sha256') == P6_COLLECT_STDOUT_SHA256
            and p6_steps[0].get('streams', [{}])[0].get('bytes') == P6_COLLECT_STDOUT_BYTES
            and p6_steps[1].get('id') == 'teardown'
            and p6_steps[1].get('actual_exit') == 0
            and p6_steps[1].get('streams', [{}])[0].get('sha256') == P6_TEARDOWN_STDOUT_SHA256
            and p6_steps[1].get('streams', [{}])[0].get('bytes') == P6_TEARDOWN_STDOUT_BYTES,
            'P6 preserve/cleanup receipt status invalid',
        )
        # P7 Cargo did start: bind its exact receipts; its independent E/S/P are debited exactly once.
        _verify_p7_receipts(debits, guard)
        # P8 remains failed and unaccepted; its consumed resources cannot be refunded.
        _verify_p8_receipts(debits, guard)
        # P9 succeeded once; its independent E/S/P are debited exactly once, never refunded.
        _verify_p9_receipts(debits, guard)

    guard.need(
        debits['retired_cached_attempts_envelope_usec']
        == (
            debits['retired_p2_p3_envelope_usec']
            + debits['retired_p4_envelope_usec']
            + debits['retired_p5_envelope_usec']
            + debits['retired_p6_envelope_usec']
            + debits['retired_p7_envelope_usec']
            + debits['retired_p8_envelope_usec']
            + debits['retired_p9_envelope_usec']
        )
        and debits['retired_cached_attempts_supervisor_usec']
        == (
            debits['retired_p2_p3_supervisor_usec']
            + debits['retired_p4_supervisor_usec']
            + debits['retired_p5_supervisor_usec']
            + debits['retired_p6_supervisor_usec']
            + debits['retired_p7_supervisor_usec']
            + debits['retired_p8_supervisor_usec']
            + debits['retired_p9_supervisor_usec']
        )
        and debits['retired_cached_attempts_phases_usec']
        == (
            debits['retired_p2_p3_phases_usec']
            + debits['retired_p4_phases_usec']
            + debits['retired_p5_phases_usec']
            + debits['retired_p6_phases_usec']
            + debits['retired_p7_phases_usec']
            + debits['retired_p8_phases_usec']
            + debits['retired_p9_phases_usec']
        )
        and debits['envelope_usec'] == debits['prior_post_r3_envelope_usec'] + debits['retired_cached_attempts_envelope_usec']
        and debits['supervisor_usec'] == debits['prior_post_r3_supervisor_usec'] + debits['retired_cached_attempts_supervisor_usec']
        and debits['phases_usec'] == debits['prior_post_r3_phases_usec'] + debits['retired_cached_attempts_phases_usec']
        # Post-P9 totals retain frozen post-P8, post-P7 and post-P6 readings; E is never normalized to S+P.
        and debits['envelope_usec'] - debits['retired_p9_envelope_usec'] - debits['retired_p8_envelope_usec'] == 637394256
        and debits['supervisor_usec'] - debits['retired_p9_supervisor_usec'] - debits['retired_p8_supervisor_usec'] == 195151731
        and debits['phases_usec'] - debits['retired_p9_phases_usec'] - debits['retired_p8_phases_usec'] == 442242523
        and debits['envelope_usec'] - debits['retired_p9_envelope_usec'] - debits['retired_p8_envelope_usec'] - debits['retired_p7_envelope_usec'] == 450272603
        and debits['supervisor_usec'] - debits['retired_p9_supervisor_usec'] - debits['retired_p8_supervisor_usec'] - debits['retired_p7_supervisor_usec'] == 179988921
        and debits['phases_usec'] - debits['retired_p9_phases_usec'] - debits['retired_p8_phases_usec'] - debits['retired_p7_phases_usec'] == 270283681
        and debits['envelope_usec'] - debits['retired_p9_envelope_usec'] == 835480719
        and debits['supervisor_usec'] - debits['retired_p9_supervisor_usec'] == 207478351
        and debits['phases_usec'] - debits['retired_p9_phases_usec'] == 628002365
        and debits['retired_p8_envelope_usec'] == 198086463
        and debits['retired_p8_supervisor_usec'] == 12326620
        and debits['retired_p8_phases_usec'] == 185759842
        and debits['retired_p9_envelope_usec'] == 127323100
        and debits['retired_p9_supervisor_usec'] == 15815495
        and debits['retired_p9_phases_usec'] == 111507604
        and debits['envelope_usec'] == 962803819
        and debits['supervisor_usec'] == 223293846
        and debits['phases_usec'] == 739509969
        and debits['f_stdout_bytes'] == 0
        and debits['f_stderr_bytes'] == 10406 + 12829 + 10829 + 137 + 8880
        and debits['p7_outer_wall_seconds_ceil'] == 135
        and debits['p8_outer_wall_seconds_ceil'] == 133
        and debits['p9_outer_wall_seconds_ceil'] == 121
        and debits['gross_supervisor_cpu_seconds_max'] == 10800
        and debits['supervisor_cpu_seconds_ceil'] == (debits['supervisor_usec'] + 999999) // 1000000
        and debits['remaining_total_wall_seconds_max'] == 6159 - debits['p9_outer_wall_seconds_ceil']
        and debits['remaining_f_wall_seconds_max'] == 2254 - debits['p9_outer_wall_seconds_ceil']
        and debits['remaining_supervisor_cpu_seconds_max']
        == debits['gross_supervisor_cpu_seconds_max'] - debits['supervisor_cpu_seconds_ceil'],
        'internal historical debit sum invariant violated',
    )
    guard.need(
        c['total_wall_seconds'] <= debits['remaining_total_wall_seconds_max'],
        'total_wall_seconds exceeds post-P9 remaining wall ceiling (6038s)',
    )
    guard.need(
        c['limits']['F']['wall_seconds'] <= debits['remaining_f_wall_seconds_max'],
        'limits.F.wall_seconds exceeds post-P9 remaining F wall ceiling (2133s)',
    )
    guard.need(
        c['supervisor_cpu_seconds'] <= debits['remaining_supervisor_cpu_seconds_max'],
        'supervisor_cpu_seconds exceeds post-P9 remaining supervisor CPU ceiling (10576s)',
    )

    e_ns, l_ns, p_ns = guard.cg_paths(c)
    e_dir, l_dir, p_dir = (guard.cg_fs(ns) for ns in (e_ns, l_ns, p_ns))
    live_env_usec = guard.read_numbers(e_dir / 'cpu.stat').get('usage_usec', 0)
    live_sup_usec = guard.read_numbers(l_dir / 'cpu.stat').get('usage_usec', 0)
    live_phases_usec = guard.read_numbers(p_dir / 'cpu.stat').get('usage_usec', 0)

    # Post-cleanup cgroups start at 0 in the kernel; all historical debits through P9 are always added.
    effective_env_usec = debits['envelope_usec'] + live_env_usec
    effective_sup_usec = debits['supervisor_usec'] + live_sup_usec
    effective_phases_usec = debits['phases_usec'] + live_phases_usec
    gross_sup_cap_usec = debits['gross_supervisor_cpu_seconds_max'] * 1_000_000

    guard.need(
        live_sup_usec <= c['supervisor_cpu_seconds'] * 1_000_000 and effective_sup_usec <= gross_sup_cap_usec,
        'cumulative supervisor CPU (including all historical debits through P9) exceeds cap',
    )
    if at_start:
        guard.need(
            effective_phases_usec < c['limits']['F']['cpu_time_us'],
            'cumulative phases CPU (including post-r3 historical debit) exceeds F cpu_time_us cap',
        )
    else:
        guard.need(
            effective_phases_usec <= c['limits']['F']['cpu_time_us'],
            'cumulative phases CPU (including post-r3 historical debit) exceeds F cpu_time_us cap',
        )
    guard.need(
        effective_env_usec <= (gross_sup_cap_usec + c['limits']['F']['cpu_time_us']),
        'cumulative envelope CPU (including all historical debits through P9) exceeds combined CPU cap',
    )
    return {
        'historical_debits': copy.deepcopy(debits),
        'live_envelope_usec_at_start': live_env_usec,
        'live_supervisor_usec_at_start': live_sup_usec,
        'live_phases_usec_at_start': live_phases_usec,
        'effective_envelope_usec_at_start': effective_env_usec,
        'effective_supervisor_usec_at_start': effective_sup_usec,
        'effective_phases_usec_at_start': effective_phases_usec,
    }




def _json_leaves(value):
    """Yield every scalar leaf of a parsed JSON value; used only for schema-independent pinned-receipt cross-checks."""
    if isinstance(value, dict):
        for v in value.values():
            yield from _json_leaves(v)
    elif isinstance(value, list):
        for v in value:
            yield from _json_leaves(v)
    else:
        yield value


def _verify_p10_receipts(c, guard):
    """Bind the retired P10 once-only charge to its exact F result, raw counters, outer wall, cleanup and original manifest bytes."""
    pinned = {}
    for key, path, size, sha in (
        ('F result', P10_F_RESULT_PATH, P10_F_RESULT_BYTES, P10_F_RESULT_SHA256),
        ('quarantined outputs', P10_QUARANTINED_OUTPUTS_PATH, P10_QUARANTINED_OUTPUTS_BYTES, P10_QUARANTINED_OUTPUTS_SHA256),
        ('raw post-terminal cgroups', P10_RAW_COUNTERS_PATH, P10_RAW_COUNTERS_BYTES, P10_RAW_COUNTERS_SHA256),
        ('outer result', P10_OUTER_RESULT_PATH, P10_OUTER_RESULT_BYTES, P10_OUTER_RESULT_SHA256),
        ('Root cleanup', P10_CLEANUP_PATH, P10_CLEANUP_BYTES, P10_CLEANUP_SHA256),
        ('original preservation manifest', P10_ORIGINAL_MANIFEST_PATH, P10_ORIGINAL_MANIFEST_BYTES, P10_ORIGINAL_MANIFEST_SHA256),
    ):
        p = Path(path)
        guard.regular(p)
        guard.need(p.stat().st_size == size and guard.digest(p) == sha, 'P10 ' + key + ' receipt SHA-256 or byte size mismatch')
        pinned[key] = p
    _verify_p10_f_result(guard.jload(pinned['F result']), c, guard)
    debits = POST_P10_HISTORICAL_DEBITS
    raw_lines = {s.splitlines()[0] for s in _json_leaves(guard.jload(pinned['raw post-terminal cgroups'])) if type(s) is str and s}
    guard.need(
        all(
            'usage_usec ' + str(debits[k]) in raw_lines
            for k in ('retired_p10_envelope_usec', 'retired_p10_supervisor_usec', 'retired_p10_phases_usec')
        ),
        'P10 raw post-terminal cgroup receipt lacks the once-only E/S/P usage',
    )
    outer = [x for x in _json_leaves(guard.jload(pinned['outer result'])) if type(x) is float]
    guard.need(
        P10_NATIVE_OUTER_SECONDS in outer
        and int(-(-P10_NATIVE_OUTER_SECONDS // 1)) == debits['p10_outer_wall_seconds_ceil'],
        'P10 outer result receipt lacks the debited native outer wall',
    )


def enforce_post_p10_historical_debits(c, guard, at_start=True):
    """Layer the retired P10 charge exactly once over the unchanged post-P9 accounting; never reset, refund or raise a ceiling."""
    base_state = enforce_post_r3_historical_debits(c, guard, at_start=at_start)
    prior = HISTORICAL_POST_R3_DEBITS
    debits = POST_P10_HISTORICAL_DEBITS
    _verify_p10_receipts(c, guard)
    guard.need(
        debits['prior_post_p9_envelope_usec'] == prior['envelope_usec']
        and debits['prior_post_p9_supervisor_usec'] == prior['supervisor_usec']
        and debits['prior_post_p9_phases_usec'] == prior['phases_usec']
        and debits['envelope_usec'] == debits['prior_post_p9_envelope_usec'] + debits['retired_p10_envelope_usec']
        and debits['supervisor_usec'] == debits['prior_post_p9_supervisor_usec'] + debits['retired_p10_supervisor_usec']
        and debits['phases_usec'] == debits['prior_post_p9_phases_usec'] + debits['retired_p10_phases_usec']
        and debits['retired_p10_envelope_usec'] == 116543574
        and debits['retired_p10_supervisor_usec'] == 19626044
        and debits['retired_p10_phases_usec'] == 96917530
        and debits['envelope_usec'] == 1079347393
        and debits['supervisor_usec'] == 242919890
        and debits['phases_usec'] == 836427499
        # E is never normalized to S+P; the independent +4us difference is carried forward unchanged.
        and prior['envelope_usec'] - prior['supervisor_usec'] - prior['phases_usec'] == 4
        and debits['envelope_usec'] - debits['supervisor_usec'] - debits['phases_usec'] == debits['independent_envelope_minus_supervisor_minus_phases_usec'] == 4
        and debits['f_stdout_bytes'] == prior['f_stdout_bytes'] + debits['p10_f_stdout_bytes'] == 0
        and debits['f_stderr_bytes'] == prior['f_stderr_bytes'] + debits['p10_f_stderr_bytes'] == 59407
        and debits['p10_prephase_outer_wall_seconds_ceil'] == int(-(-P10_PREPHASE_OUTER_SECONDS // 1)) == 5
        and debits['p10_outer_wall_seconds_ceil'] == int(-(-P10_NATIVE_OUTER_SECONDS // 1)) == 98
        and debits['gross_supervisor_cpu_seconds_max'] == prior['gross_supervisor_cpu_seconds_max'] == 10800
        and debits['supervisor_cpu_seconds_ceil'] == (debits['supervisor_usec'] + 999999) // 1000000 == 243
        and debits['remaining_supervisor_cpu_seconds_max'] == debits['gross_supervisor_cpu_seconds_max'] - debits['supervisor_cpu_seconds_ceil'] == 10557
        and debits['remaining_total_wall_seconds_max'] == prior['remaining_total_wall_seconds_max'] - debits['p10_prephase_outer_wall_seconds_ceil'] - debits['p10_outer_wall_seconds_ceil'] == 5935
        and debits['remaining_f_wall_seconds_max'] == prior['remaining_f_wall_seconds_max'] - debits['p10_outer_wall_seconds_ceil'] == 2035,
        'internal post-P10 historical debit invariant violated',
    )
    guard.need(
        c['total_wall_seconds'] <= debits['remaining_total_wall_seconds_max'],
        'total_wall_seconds exceeds post-P10 remaining wall ceiling (5935s)',
    )
    guard.need(
        c['limits']['F']['wall_seconds'] <= debits['remaining_f_wall_seconds_max'],
        'limits.F.wall_seconds exceeds post-P10 remaining F wall ceiling (2035s)',
    )
    guard.need(
        c['supervisor_cpu_seconds'] <= debits['remaining_supervisor_cpu_seconds_max'],
        'supervisor_cpu_seconds exceeds post-P10 remaining supervisor CPU ceiling (10557s)',
    )

    e_ns, l_ns, p_ns = guard.cg_paths(c)
    e_dir, l_dir, p_dir = (guard.cg_fs(ns) for ns in (e_ns, l_ns, p_ns))
    live_env_usec = guard.read_numbers(e_dir / 'cpu.stat').get('usage_usec', 0)
    live_sup_usec = guard.read_numbers(l_dir / 'cpu.stat').get('usage_usec', 0)
    live_phases_usec = guard.read_numbers(p_dir / 'cpu.stat').get('usage_usec', 0)
    guard.need(
        live_env_usec >= base_state['live_envelope_usec_at_start']
        and live_sup_usec >= base_state['live_supervisor_usec_at_start']
        and live_phases_usec >= base_state['live_phases_usec_at_start'],
        'live cgroup CPU counter reset during post-P10 accounting',
    )

    # Fresh post-cleanup groups start at 0; every historical debit through P10 is always added, never refunded.
    effective_env_usec = debits['envelope_usec'] + live_env_usec
    effective_sup_usec = debits['supervisor_usec'] + live_sup_usec
    effective_phases_usec = debits['phases_usec'] + live_phases_usec
    gross_sup_cap_usec = debits['gross_supervisor_cpu_seconds_max'] * 1_000_000

    guard.need(
        live_sup_usec <= c['supervisor_cpu_seconds'] * 1_000_000 and effective_sup_usec <= gross_sup_cap_usec,
        'cumulative supervisor CPU (including all historical debits through P10) exceeds cap',
    )
    if at_start:
        guard.need(
            effective_phases_usec < c['limits']['F']['cpu_time_us'],
            'cumulative phases CPU (including post-P10 historical debit) exceeds F cpu_time_us cap',
        )
    else:
        guard.need(
            effective_phases_usec <= c['limits']['F']['cpu_time_us'],
            'cumulative phases CPU (including post-P10 historical debit) exceeds F cpu_time_us cap',
        )
    guard.need(
        effective_env_usec <= (gross_sup_cap_usec + c['limits']['F']['cpu_time_us']),
        'cumulative envelope CPU (including all historical debits through P10) exceeds combined CPU cap',
    )
    return {
        'historical_debits': copy.deepcopy(HISTORICAL_POST_R3_DEBITS),
        'post_p10_debits': copy.deepcopy(POST_P10_HISTORICAL_DEBITS),
        'live_envelope_usec_at_start': live_env_usec,
        'live_supervisor_usec_at_start': live_sup_usec,
        'live_phases_usec_at_start': live_phases_usec,
        'effective_envelope_usec_at_start': effective_env_usec,
        'effective_supervisor_usec_at_start': effective_sup_usec,
        'effective_phases_usec_at_start': effective_phases_usec,
    }
P11_ACTUAL_ROOT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native-p11-actual-host-provision-20261019'

P11_EVIDENCE_ROOT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-sdk-evidence-20261018-06-cached-current-p11-01'

P11_OUTER_DIR = P11_ACTUAL_ROOT + '/actual-one-current-WW-cached-current-F-01'

P11_RECEIPT_PINS = {
    'outer result': (P11_OUTER_DIR + '/RESULT.json', 922, 'c8cb1e1b518ec0ea78067e62d2852ce2ccd00d3de1dccff7b9e800e3b970269e'),
    'outer stderr': (P11_OUTER_DIR + '/stderr.bin', 39, '9b4078eb89bbadd2597138fa1536d7732ccfa032574ab7218427a91f1a3a28bd'),
    'Root cleanup': (P11_ACTUAL_ROOT + '/ROOT-P11-CLEANUP-RESULT.json', 1670, '82e136a224b39984fb53405dcd9286a9306a50a6b6377c4f3bbbe744f62210dd'),
    'teardown result': (P11_ACTUAL_ROOT + '/actual-teardown-01/RESULT.json', 859, '171e95ac96312ab3d941127df9bf8ee9cd78d3f0234f53ab243bdc98f6a32aad'),
    'evidence manifest': (P11_EVIDENCE_ROOT + '/EVIDENCE_SHA256_MANIFEST.json', 797, '131309ba547568bfdbc5e28a417e557f2b71eb3b022716f51a97c6a499100e6f'),
}

# Preserved attempt files (manifest-relative); each SHA-256 comes from the pinned manifest.
P11_EVIDENCE_FILE_BYTES = {
    'ONE_SHOT_STARTED': 9,
    'packet/F_precheck.stderr': 112,
    'packet/F_precheck.stdout': 0,
    'packet/TERMINAL_FAILURE.json': 111,
    'packet/USED_LAUNCH_ac016eccddc7f851f05f244b81ac1a86e9e2a0fcfc8bee86ab0195f7826611e8': 9,
}

# Raw terminal cpu.stat (usage, user, system, nr_periods).
P11_RAW_CPU_STAT = {
    'E': (12906928, 9991688, 2915240, 135),
    'S': (9308089, 7234742, 2073347, 130),
    'P': (3598838, 2756981, 841857, 42),
}

P11_OUTER_SECONDS = 13.15461667800264

POST_P11_HISTORICAL_DEBITS = {
    # Failed P11 F_precheck: one-shot consumed, no Cargo or ELF; never refunded.
    'retired_p11_envelope_usec': 12906928,
    'retired_p11_supervisor_usec': 9308089,
    'retired_p11_phases_usec': 3598838,
    'retired_p11_independent_residual_usec': 1,
    'envelope_usec': 1092254321,
    'supervisor_usec': 252227979,
    'supervisor_cpu_seconds_ceil': 253,
    'phases_usec': 840026337,
    'independent_envelope_minus_supervisor_minus_phases_usec': 5,
    'p11_f_stdout_bytes': 0,
    'p11_f_stderr_bytes': 112,
    # Separate outer controller stream; never also counted as F stderr.
    'p11_outer_controller_stderr_bytes_separate': 39,
    'f_stdout_bytes': 0,
    'f_stderr_bytes': 59519,
    # ceil(13.15s actual outer), debited once to both whole and F wall.
    'p11_outer_wall_seconds_ceil': 14,
    'remaining_total_wall_seconds_max': 5921,
    'remaining_f_wall_seconds_max': 2021,
    'remaining_supervisor_cpu_seconds_max': 10547,
    'gross_supervisor_cpu_seconds_max': 10800,
}

def _verify_p11_receipts(guard):
    """Bind the failed P11 F_precheck charge to its exact pinned actual records."""
    debits = POST_P11_HISTORICAL_DEBITS
    pinned = {}
    for key, (path, size, sha) in P11_RECEIPT_PINS.items():
        p = Path(path)
        guard.regular(p)
        guard.need(p.stat().st_size == size and guard.digest(p) == sha, 'P11 ' + key + ' SHA-256 or byte size mismatch')
        pinned[key] = p
    files = guard.jload(pinned['evidence manifest'])['files']
    guard.need(set(files) == set(P11_EVIDENCE_FILE_BYTES), 'P11 evidence manifest file set mismatch')
    for rel, size in P11_EVIDENCE_FILE_BYTES.items():
        p = Path(P11_EVIDENCE_ROOT + '/' + rel)
        guard.regular(p)
        guard.need(p.stat().st_size == size and guard.digest(p) == files[rel], 'P11 ' + rel + ' SHA-256 or byte size mismatch')
        pinned[rel] = p
    esp = (('E', 'envelope_usec'), ('S', 'supervisor_usec'), ('P', 'phases_usec'))
    for node, k in esp:
        p = Path(P11_ACTUAL_ROOT + '/actual-terminal-preservation-01/' + node + '-cpu.stat')
        guard.regular(p)
        raw = (
            'usage_usec %d\nuser_usec %d\nsystem_usec %d\nnice_usec 0\nnr_periods %d\n'
            'nr_throttled 0\nthrottled_usec 0\nnr_bursts 0\nburst_usec 0\n' % P11_RAW_CPU_STAT[node]
        )
        guard.need(
            p.read_bytes() == raw.encode() and P11_RAW_CPU_STAT[node][0] == debits['retired_p11_' + k],
            'P11 raw ' + node + ' cpu.stat mismatch',
        )
    outer = guard.jload(pinned['outer result'])
    guard.exact(outer, 'id actual_exit elapsed_seconds controller_failure streams', 'P11 outer result')
    cleanup = guard.jload(pinned['Root cleanup'])
    counts = cleanup['counts']
    guard.need(
        counts.get('phase') == 'F_precheck'
        and counts.get('result') == dict(zip(('path', 'bytes', 'sha256'), P11_RECEIPT_PINS['outer result']))
        and counts.get('rawUsageUsec') == {n: debits['retired_p11_' + k] for n, k in esp}
        and counts.get('previousHistory') == {n: POST_P10_HISTORICAL_DEBITS[k] for n, k in esp}
        and counts.get('cumulativeUsec') == {n: debits[k] for n, k in esp}
        and counts.get('currentResidualUsec') == debits['retired_p11_independent_residual_usec']
        and all(counts.get(k) is False for k in ('CargoStarted', 'SDKbuild', 'newELF', 'nativeOutputsAccepted'))
        and counts.get('oneShotConsumed') is True
        # Distinct F stream: the exact A0 digest refusal, never the outer controller stderr.
        and pinned['packet/F_precheck.stderr'].read_bytes() == str(counts.get('failure')).encode()
        and guard.jload(pinned['packet/TERMINAL_FAILURE.json']).get('type') == 'Refusal'
        and outer['id'] == 'P11-one-current-WW-cached-current-F'
        and outer['actual_exit'] == 2
        and outer['controller_failure'] is None
        and outer['elapsed_seconds'] == P11_OUTER_SECONDS
        # Current status is this outer cleanup record; counts.cleanupOwed is a historical snapshot.
        and cleanup.get('status') == 'DONE'
        and cleanup.get('cleanupOwed') is False
        and all(cleanup.get(k) is True for k in ('all3MountsAbsent', 'EandSupervisorAndPhasesAbsent', 'noCargoStarted', 'noELFProducedOrExecuted'))
        and cleanup.get('evidence') == P11_EVIDENCE_ROOT
        and cleanup.get('teardown') == dict(zip(('path', 'bytes', 'sha256'), P11_RECEIPT_PINS['teardown result']))
        and guard.jload(pinned['teardown result']).get('actual_exit') == 0,
        'P11 failed F_precheck receipt content mismatch',
    )

def enforce_post_p11_historical_debits(c, guard, at_start=True):
    """Layer the retired failed P11 F_precheck charge once over unchanged post-P10 accounting."""
    base_state = enforce_post_p10_historical_debits(c, guard, at_start=at_start)
    prior = POST_P10_HISTORICAL_DEBITS
    debits = POST_P11_HISTORICAL_DEBITS
    _verify_p11_receipts(guard)
    resid = 'independent_envelope_minus_supervisor_minus_phases_usec'
    wall = debits['p11_outer_wall_seconds_ceil']
    guard.need(
        all(debits[k] == prior[k] + debits['retired_p11_' + k] for k in ('envelope_usec', 'supervisor_usec', 'phases_usec'))
        # E is never normalized to S+P: the independent residual carries 4 + 1 = 5.
        and debits['retired_p11_envelope_usec'] - debits['retired_p11_supervisor_usec'] - debits['retired_p11_phases_usec']
        == debits['retired_p11_independent_residual_usec'] == 1
        and debits['envelope_usec'] - debits['supervisor_usec'] - debits['phases_usec']
        == debits[resid] == prior[resid] + debits['retired_p11_independent_residual_usec'] == 5
        and debits['f_stdout_bytes'] == prior['f_stdout_bytes'] + debits['p11_f_stdout_bytes']
        == prior['f_stdout_bytes'] + P11_EVIDENCE_FILE_BYTES['packet/F_precheck.stdout'] == 0
        and debits['f_stderr_bytes'] == prior['f_stderr_bytes'] + debits['p11_f_stderr_bytes'] == 59519
        and debits['p11_f_stderr_bytes'] == P11_EVIDENCE_FILE_BYTES['packet/F_precheck.stderr']
        and debits['p11_outer_controller_stderr_bytes_separate'] == P11_RECEIPT_PINS['outer stderr'][1] == 39
        and wall == int(-(-P11_OUTER_SECONDS // 1)) == 14
        and debits['gross_supervisor_cpu_seconds_max'] == prior['gross_supervisor_cpu_seconds_max'] == 10800
        and debits['supervisor_cpu_seconds_ceil'] == (debits['supervisor_usec'] + 999999) // 1000000 == 253
        and debits['remaining_supervisor_cpu_seconds_max'] == 10800 - debits['supervisor_cpu_seconds_ceil'] == 10547
        and debits['remaining_total_wall_seconds_max'] == prior['remaining_total_wall_seconds_max'] - wall == 5921
        and debits['remaining_f_wall_seconds_max'] == prior['remaining_f_wall_seconds_max'] - wall == 2021,
        'internal post-P11 historical debit invariant violated',
    )
    guard.need(
        c['total_wall_seconds'] <= debits['remaining_total_wall_seconds_max']
        and c['limits']['F']['wall_seconds'] <= debits['remaining_f_wall_seconds_max']
        and c['supervisor_cpu_seconds'] <= debits['remaining_supervisor_cpu_seconds_max'],
        'config exceeds post-P11 remaining wall, F wall or supervisor CPU ceiling',
    )
    live_e, live_s, live_p = (guard.read_numbers(guard.cg_fs(ns) / 'cpu.stat').get('usage_usec', 0) for ns in guard.cg_paths(c))
    guard.need(
        live_e >= base_state['live_envelope_usec_at_start']
        and live_s >= base_state['live_supervisor_usec_at_start']
        and live_p >= base_state['live_phases_usec_at_start'],
        'live cgroup CPU counter reset during post-P11 accounting',
    )
    # Every historical debit through P11 is always added, never refunded.
    eff_e, eff_s, eff_p = debits['envelope_usec'] + live_e, debits['supervisor_usec'] + live_s, debits['phases_usec'] + live_p
    sup_cap = debits['gross_supervisor_cpu_seconds_max'] * 1_000_000
    f_cap = c['limits']['F']['cpu_time_us']
    guard.need(
        live_s <= c['supervisor_cpu_seconds'] * 1_000_000 and eff_s <= sup_cap,
        'cumulative supervisor CPU through P11 exceeds cap',
    )
    guard.need(eff_p < f_cap if at_start else eff_p <= f_cap, 'cumulative phases CPU through P11 exceeds F cpu_time_us cap')
    guard.need(eff_e <= sup_cap + f_cap, 'cumulative envelope CPU through P11 exceeds combined CPU cap')
    return dict(
        base_state,
        post_p11_debits=copy.deepcopy(debits),
        live_envelope_usec_at_start=live_e,
        live_supervisor_usec_at_start=live_s,
        live_phases_usec_at_start=live_p,
        effective_envelope_usec_at_start=eff_e,
        effective_supervisor_usec_at_start=eff_s,
        effective_phases_usec_at_start=eff_p,
    )

def cached_current_f(c, config_sha, guard):
    """Supervisor entry point for --cached-current-f (preparation + Cargo + post-audit run inside phase F leaf)."""
    guard.need(c['mode'] == 'tier1-and-compile', 'cached-current-f requires tier1-and-compile mode')
    guard.need(HEX40.fullmatch(guard.BASE) is not None, 'current candidate BASE commit is still unsealed placeholder')
    guard.consent(c)
    guard.supervisor_envelope(c)
    debit_state = enforce_post_p11_historical_debits(c, guard, at_start=True)

    def expired(signum, frame):
        raise guard.Refusal('whole-job wall cap')

    signal.signal(signal.SIGALRM, expired)
    signal.setitimer(signal.ITIMER_REAL, c['total_wall_seconds'])
    import resource
    max_rem_sup_sec = (
        POST_P11_HISTORICAL_DEBITS['gross_supervisor_cpu_seconds_max'] * 1_000_000
        - POST_P11_HISTORICAL_DEBITS['supervisor_usec']
    ) // 1_000_000
    rlim_cpu = min(c['supervisor_cpu_seconds'], max_rem_sup_sec)
    resource.setrlimit(resource.RLIMIT_CPU, (rlim_cpu, rlim_cpu))

    guard.bound_identities(c)
    p = c['paths']
    root = Path(p['execution_root'])
    packet_dir = Path(p['packet'])
    guard.owned_paths(c)
    for k in ('source_home', 'target', 'tmp', 'packet', 'empty_home', 'mask_empty'):
        guard.need(Path(p[k]).is_dir() and not any(Path(p[k]).iterdir()), 'required fresh empty owned path: ' + k)
        guard.need(Path(p[k]).stat().st_uid == os.getuid(), 'output ownership')
    guard.write_new(root / 'ONE_SHOT_STARTED', b'no retry\n', 64)

    orig_group_init = guard.Group.__init__

    def debited_group_init(self_group, cfg, phase):
        orig_group_init(self_group, cfg, phase)
        if phase == 'F':
            self_group.prior_cpu = debit_state['effective_phases_usec_at_start']

    guard.Group.__init__ = debited_group_init
    try:
        guard.bound_identities(c)
        guard.quotas(c, 'F')
        f_result = guard.launch(c, 'F', config_sha)
        reconstitution = guard.jload(packet_dir / 'SOURCE_HOME_RECONSTITUTION.json')
        seed_snapshot = guard.jload(packet_dir / 'CACHED_TARGET_SEED.json')
        log_bytes = f_result['log_bytes']
        guard.need(
            POST_P11_HISTORICAL_DEBITS['f_stderr_bytes'] + log_bytes['stderr'] <= c['limits']['F']['stderr_bytes'],
            'cumulative F stderr bytes (including post-P11 historical debit) exceeded cap',
        )
        guard.need(
            POST_P11_HISTORICAL_DEBITS['f_stdout_bytes'] + log_bytes['stdout'] <= c['limits']['F']['stdout_bytes'],
            'cumulative F stdout bytes (including post-P11 historical debit) exceeded cap',
        )
        post_debit_state = enforce_post_p11_historical_debits(c, guard, at_start=False)
        if (packet_dir / 'CACHED_POST_COMPILE_CHECK.json').is_file():
            post_check = guard.jload(packet_dir / 'CACHED_POST_COMPILE_CHECK.json')
        else:
            post_check = verify_post_compile_no_script_reexec(c, guard, seed_snapshot)
        guard.bound_identities(c)
        combined = {
            'phase': 'F',
            'route': 'cached-current-f',
            'base_head': guard.BASE,
            'cargo_lock_sha256': guard.LOCK,
            'historical_debit_state_start': debit_state,
            'historical_debit_state_end': post_debit_state,
            'source_home_reconstitution': reconstitution,
            'cached_target_seed_counts': seed_snapshot['partition_counts'],
            'post_compile_check': post_check,
            'f_result': f_result,
        }
        guard.receipt(c, 'F_CACHED_CURRENT_RESULT.json', combined, 'F')
        signal.setitimer(signal.ITIMER_REAL, 0)
        return combined
    except BaseException as e:
        signal.setitimer(signal.ITIMER_REAL, 0)
        try:
            guard.receipt(
                c,
                'TERMINAL_FAILURE.json',
                {'type': type(e).__name__, 'message': str(e), 'native_outputs_accepted': False, 'route': 'cached-current-f'},
                'F',
            )
        except Exception:
            pass
        raise
    finally:
        guard.Group.__init__ = orig_group_init


GENERATED_INPUT_CLASS_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-generated-input-class-correction-request-20261018'
)
GENERATED_INPUT_SNAPSHOT_DIR = GENERATED_INPUT_CLASS_ROOT + '/proposed-current-generated-snapshots'
EIGHT_GIT_CONTROL_PROJECTION_PATH = GENERATED_INPUT_CLASS_ROOT + '/EIGHT-SNAPSHOT-OWNER-PROJECTION.json'
EIGHT_GIT_CONTROL_PROJECTION_SHA256 = 'a36fd57bd8f09dfa05a478584ba4010a6773204518c6dd0d00357755633aa61c'
EIGHT_GIT_CONTROL_PROJECTION_BYTES = 6069
EIGHT_GIT_CONTROL_MANIFEST_PATH = GENERATED_INPUT_CLASS_ROOT + '/EIGHT-CURRENT-SNAPSHOT-CANDIDATES.proposed.json'
EIGHT_GIT_CONTROL_MANIFEST_SHA256 = 'bedc415cc183227833f7c55bcb4c62b5fa8b7390f90b0a84eaf6a2ddda4d6337'
EIGHT_GIT_CONTROL_MANIFEST_BYTES = 10917
EIGHT_GIT_CONTROL_TOTAL_SNAPSHOT_BYTES = 281004

EIGHT_GIT_CONTROL_SNAPSHOT_SPEC = {
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/config': {
        'historical_sha256': '1a4515b2faa2cffbdef25c1649c29bb601fa33adcf51d5e5e599463477ba5cba',
        'sha256': 'fcd80a3f9d90d7b6315ce15c5f3ca05ecbe4fb526059debc4d44ae54bcf88055',
        'bytes': 308,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/config',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/index': {
        'historical_sha256': 'cac9f99d5d1c3f30bd50c0d21bd622e699f1aa2d0c917c0058193a7c187ac252',
        'sha256': '964d9372f03a7777308c94ccc9c8c108272be91328872551f61efd8b3f67aaf2',
        'bytes': 148865,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/index',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/logs/HEAD': {
        'historical_sha256': 'df335cebbd4fc8ca6e7c45afb674f974f11d3b7ff0008f7c71bb493bae3cfd49',
        'sha256': '0e700a6f5728fe2fa851ec7ffb8872b545a9bfb0eb2e41a78c3dc319a3e18326',
        'bytes': 205,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/logs/HEAD',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/logs/refs/heads/master': {
        'historical_sha256': 'df335cebbd4fc8ca6e7c45afb674f974f11d3b7ff0008f7c71bb493bae3cfd49',
        'sha256': '0e700a6f5728fe2fa851ec7ffb8872b545a9bfb0eb2e41a78c3dc319a3e18326',
        'bytes': 205,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/04c9c5d/.git/logs/refs/heads/master',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/config': {
        'historical_sha256': '1a4515b2faa2cffbdef25c1649c29bb601fa33adcf51d5e5e599463477ba5cba',
        'sha256': 'fcd80a3f9d90d7b6315ce15c5f3ca05ecbe4fb526059debc4d44ae54bcf88055',
        'bytes': 308,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/config',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/index': {
        'historical_sha256': 'a24ec9918c19345bd9d2c50977323700fcf544080076ec9709bdf1b64b5a07ad',
        'sha256': 'bcf080592b48c8d40ed7e9752556ddfae02facc648e1af137b01fb075ebf071c',
        'bytes': 130703,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/index',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/logs/HEAD': {
        'historical_sha256': 'fdd45aac4d043dffc87051d6e9f914cdde0243667a82fae4be6c5e3292cda057',
        'sha256': 'ec53f18d9d9f18421d3747847307af6f3fb582169b291d5bff658dc8b9d67600',
        'bytes': 205,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/logs/HEAD',
    },
    'git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/logs/refs/heads/master': {
        'historical_sha256': 'fdd45aac4d043dffc87051d6e9f914cdde0243667a82fae4be6c5e3292cda057',
        'sha256': 'ec53f18d9d9f18421d3747847307af6f3fb582169b291d5bff658dc8b9d67600',
        'bytes': 205,
        'path': GENERATED_INPUT_SNAPSHOT_DIR + '/git/checkouts/midnight-ledger-b2f9c59d942dfdca/a01a1ea/.git/logs/refs/heads/master',
    },
}

ALLOWED_GIT_CONFIG_SECTION_KEYS = {
    'core': frozenset({'repositoryformatversion', 'filemode', 'bare', 'logallrefupdates', 'autocrlf'}),
    'remote "origin"': frozenset({'url', 'fetch'}),
    'branch "master"': frozenset({'remote', 'merge'}),
}

P4_INTEGRATION_METADATA_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-p4-integration-metadata-20261018'
)
P4_COUNTERS_BEFORE_RETIRE_PATH = P4_INTEGRATION_METADATA_ROOT + '/actual-p4-preserve-and-cleanup-01/ROOT-ACTUAL-COUNTERS-BEFORE-RETIRE.json'
P4_COUNTERS_BEFORE_RETIRE_SHA256 = '02246e4a6466a4cbbcb1c11d386bc997a8c48e85e8175ebf174a583c92712205'
P4_COUNTERS_BEFORE_RETIRE_BYTES = 1753
P4_PRESERVE_CLEANUP_RESULT_PATH = P4_INTEGRATION_METADATA_ROOT + '/actual-p4-preserve-and-cleanup-01/RESULT.json'
P4_PRESERVE_CLEANUP_RESULT_SHA256 = '38c797aef6e6d3dfc5db3597e0d9be0662fdf709590a129a5c94aab6f2fb43e7'
P4_PRESERVE_CLEANUP_RESULT_BYTES = 2632
P4_ACTUAL_RESULT_PATH = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-compile-final-literal-integration-20261018/'
    'prospective-final-04/metadata-refresh-02/actual-cached-compile-01/RESULT.json'
)
P4_ACTUAL_RESULT_SHA256 = '4eb2c34a23ba8401c3ad74824d720f2242a26766a8fd9a099f6a7f08cfb73c95'

def _validate_git_control_snapshot_payload(rel, snap_p, guard):
    """Validate non-secret format and INI/header safety of a pinned git-control snapshot."""
    raw = Path(snap_p).read_bytes()
    if rel.endswith('/.git/config'):
        guard.need(b'\x00' not in raw, 'NUL byte in .git/config snapshot: ' + rel)
        try:
            text = raw.decode('utf-8')
        except UnicodeDecodeError:
            guard.need(False, 'non-UTF8 .git/config snapshot: ' + rel)
        current_sec = None
        seen_sections = set()
        for raw_line in text.splitlines():
            line = raw_line.strip()
            if not line or line.startswith('#') or line.startswith(';'):
                continue
            if line.startswith('[') and line.endswith(']'):
                current_sec = line[1:-1].strip()
                guard.need(
                    current_sec in ALLOWED_GIT_CONFIG_SECTION_KEYS,
                    'disallowed .git/config section in snapshot: ' + str(current_sec),
                )
                seen_sections.add(current_sec)
                continue
            guard.need(current_sec is not None and '=' in line, 'malformed .git/config line in snapshot: ' + rel)
            k, v = line.split('=', 1)
            key_norm = k.strip().lower()
            val_norm = v.strip()
            guard.need(
                key_norm in ALLOWED_GIT_CONFIG_SECTION_KEYS[current_sec],
                'disallowed .git/config key in snapshot: ' + key_norm,
            )
            if current_sec == 'remote "origin"' and key_norm == 'url':
                guard.need(
                    (val_norm.startswith('/home/charl/.cargo/git/db/') or val_norm.startswith('file:///home/charl/.cargo/git/db/'))
                    and '@' not in val_norm
                    and '?' not in val_norm
                    and '#' not in val_norm,
                    'disallowed remote origin url in .git/config snapshot: ' + rel,
                )
        guard.need('core' in seen_sections, 'missing [core] section in .git/config snapshot: ' + rel)
    elif rel.endswith('/.git/index'):
        guard.need(len(raw) >= 12 and raw[:4] == b'DIRC', 'invalid .git/index DIRC header in snapshot: ' + rel)
        ver = int.from_bytes(raw[4:8], 'big')
        guard.need(ver in (2, 3, 4), 'unsupported .git/index version in snapshot: ' + rel)
    elif rel.endswith('/.git/logs/HEAD') or rel.endswith('/.git/logs/refs/heads/master'):
        guard.need(len(raw) > 0 and b'\x00' not in raw and raw.endswith(b'\n'), 'invalid git reflog snapshot: ' + rel)
        try:
            raw.decode('utf-8')
        except UnicodeDecodeError:
            guard.need(False, 'non-UTF8 git reflog snapshot: ' + rel)
    else:
        guard.need(False, 'unlisted git-control snapshot relative path: ' + rel)

def _verify_eight_git_control_snapshots(projection_file, old_d_files, guard):
    """Verify the 8 pinned git-control snapshot descriptors and on-disk mode-0444 snapshot files."""
    proj_p = Path(projection_file)
    guard.regular(proj_p)
    proj_sha = guard.digest(proj_p)
    if str(proj_p) == EIGHT_GIT_CONTROL_PROJECTION_PATH:
        guard.need(
            proj_sha == EIGHT_GIT_CONTROL_PROJECTION_SHA256
            and proj_p.stat().st_size == EIGHT_GIT_CONTROL_PROJECTION_BYTES,
            'eight git-control snapshot projection SHA-256 or byte size mismatch',
        )
        if EIGHT_GIT_CONTROL_MANIFEST_PATH is not None:
            mf_p = Path(EIGHT_GIT_CONTROL_MANIFEST_PATH)
            guard.regular(mf_p)
            guard.need(
                mf_p.stat().st_size == EIGHT_GIT_CONTROL_MANIFEST_BYTES
                and guard.digest(mf_p) == EIGHT_GIT_CONTROL_MANIFEST_SHA256,
                'eight git-control full manifest SHA-256 or byte size mismatch',
            )
    obj = guard.jload(proj_p)
    guard.need(
        isinstance(obj, dict)
        and obj.get('status') == 'EIGHT_CURRENT_NONEXECUTED_SNAPSHOT_CANDIDATES_ONLY_NOT_HISTORICAL_BYTES_OR_ADOPTION'
        and isinstance(obj.get('rows'), list)
        and len(obj['rows']) == len(EIGHT_GIT_CONTROL_SNAPSHOT_SPEC),
        'invalid eight git-control snapshot projection schema',
    )
    if str(proj_p) == EIGHT_GIT_CONTROL_PROJECTION_PATH and EIGHT_GIT_CONTROL_MANIFEST_PATH is not None:
        fm = obj.get('full_manifest')
        guard.need(
            isinstance(fm, dict)
            and fm.get('path') == EIGHT_GIT_CONTROL_MANIFEST_PATH
            and fm.get('bytes') == EIGHT_GIT_CONTROL_MANIFEST_BYTES
            and fm.get('sha256') == EIGHT_GIT_CONTROL_MANIFEST_SHA256,
            'eight git-control full_manifest descriptor mismatch',
        )

    by_rel = {}
    total_bytes = 0
    for row in obj['rows']:
        guard.need(isinstance(row, dict) and row.get('kind') == 'git-control', 'invalid git-control snapshot row kind')
        rel = str(guard.relative(str(row.get('relative', ''))))
        guard.need(rel in EIGHT_GIT_CONTROL_SNAPSHOT_SPEC, 'unallowlisted git-control snapshot path: ' + rel)
        guard.need(rel not in by_rel, 'duplicate git-control snapshot row: ' + rel)
        spec = EIGHT_GIT_CONTROL_SNAPSHOT_SPEC[rel]
        guard.need(
            row.get('historical_sha256') == spec['historical_sha256']
            and old_d_files.get(rel) == spec['historical_sha256'],
            'git-control snapshot historical D_FREEZE hash mismatch: ' + rel,
        )
        snap = row.get('proposed_current_snapshot')
        guard.need(
            isinstance(snap, dict)
            and isinstance(snap.get('path'), str)
            and snap.get('bytes') == spec['bytes']
            and snap.get('sha256') == spec['sha256']
            and snap.get('sha256') != spec['historical_sha256'],
            'invalid proposed_current_snapshot descriptor for: ' + rel,
        )
        if str(proj_p) == EIGHT_GIT_CONTROL_PROJECTION_PATH:
            guard.need(snap.get('path') == spec['path'], 'pinned git-control snapshot path mismatch: ' + rel)
        snap_p = Path(snap['path'])
        guard.regular(snap_p)
        snap_mode = stat.S_IMODE(snap_p.lstat().st_mode)
        guard.need(snap_mode == 0o444, 'git-control snapshot file mode must be 0444: ' + str(snap_p))
        guard.need(
            snap_p.stat().st_size == spec['bytes'] and guard.digest(snap_p) == spec['sha256'],
            'git-control snapshot file size or SHA-256 mismatch on disk: ' + rel,
        )
        _validate_git_control_snapshot_payload(rel, snap_p, guard)
        total_bytes += spec['bytes']
        by_rel[rel] = row

    guard.need(
        set(by_rel.keys()) == set(EIGHT_GIT_CONTROL_SNAPSHOT_SPEC.keys()),
        'incomplete eight git-control snapshot path coverage',
    )
    guard.need(
        obj.get('total_snapshot_bytes') == total_bytes,
        'eight git-control total_snapshot_bytes mismatch',
    )
    return proj_sha, by_rel


WORKSPACE_MEMBER_PACKAGES = (
    'mori',
    'mori-ast',
    'mori-backend',
    'mori-checker',
    'mori-core',
    'mori-diagnostics',
    'mori-fmt',
    'mori-ledger',
    'mori-lexer',
    'mori-parser',
    'mori-span',
)

P5_INTEGRATION_METADATA_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-p5-integration-metadata-20261018'
)

P5_ACTUAL_RESULT_PATH = (
    P5_INTEGRATION_METADATA_ROOT
    + '/actual-p5-config-and-cached-compile-01/actual-cached-compile-01/RESULT.json'
)
P5_ACTUAL_RESULT_SHA256 = '41e1aecc57bb0c3da158cc211ea6201d6924cfc8c538c756bcfac1b77c7f8da7'
P5_ACTUAL_RESULT_BYTES = 994

P5_COUNTERS_BEFORE_RETIRE_PATH = (
    P5_INTEGRATION_METADATA_ROOT
    + '/actual-p5-preserve-and-cleanup-01/ROOT-ACTUAL-COUNTERS-BEFORE-RETIRE.json'
)
P5_COUNTERS_BEFORE_RETIRE_SHA256 = '779e66a3fbdea13babd800d51c238950cf5575b01a5f6ce1ecf9ead142e846f3'
P5_COUNTERS_BEFORE_RETIRE_BYTES = 1715

P5_PRESERVE_CLEANUP_RESULT_PATH = (
    P5_INTEGRATION_METADATA_ROOT + '/actual-p5-preserve-and-cleanup-01/RESULT.json'
)
P5_PRESERVE_CLEANUP_RESULT_SHA256 = 'ff5be61638759e2a8d9f132e9721e40b2ee1ffcfbd4d136853ffc5f0631ef9fe'
P5_PRESERVE_CLEANUP_RESULT_BYTES = 2762


P6_INTEGRATION_METADATA_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-p6-integration-metadata-20261018'
)
P6_PERSISTENT_QUARANTINE_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p6-persistent-quarantine-20261018-01'
)
P6_QUARANTINE_MANIFEST_PATH = P6_PERSISTENT_QUARANTINE_ROOT + '/PRESERVATION-MANIFEST.json'
P6_QUARANTINE_MANIFEST_SHA256 = '5527280a0dbf0c1ce173473183ac5b2d93724b168ea77ef6eaba65c67716a1da'
P6_QUARANTINE_MANIFEST_BYTES = 462088

P6_DEBITS_BEFORE_RETIRE_PATH = (
    P6_INTEGRATION_METADATA_ROOT
    + '/actual-p6-config-and-cached-compile-02/ROOT-P6-DEBITS-BEFORE-RETIRE.json'
)
P6_DEBITS_BEFORE_RETIRE_SHA256 = 'c854610dbf5cb67450b38f6cac623034fd39327d64e1d814511ef2da17f5c2a1'
P6_DEBITS_BEFORE_RETIRE_BYTES = 5364

P6_PRESERVE_CLEANUP_RESULT_PATH = (
    P6_INTEGRATION_METADATA_ROOT + '/actual-p6-preserve-and-cleanup-01/RESULT.json'
)
P6_COMPILE_STDOUT_PATH = (
    P6_INTEGRATION_METADATA_ROOT
    + '/actual-p6-config-and-cached-compile-02/actual-cached-compile-01/stdout.bin'
)
P6_COMPILE_STDOUT_SHA256 = '465fa7f1008e49c3aa6544e9cd5898bb0ece96623170f273bb14ac86986d708e'
P6_COMPILE_STDOUT_BYTES = 1588
P6_COLLECT_STDOUT_SHA256 = 'ab7ce665e76fdf86e4b8450037e8c23f784f549aa13bb180bc4110ae65949f4c'
P6_COLLECT_STDOUT_BYTES = 164
P6_TEARDOWN_STDOUT_SHA256 = '5dc221d267cc10b16db4d8a355a05cbbecde0198922b03bbdbe4e8426f4627f4'
P6_TEARDOWN_STDOUT_BYTES = 128
P6_EMPTY_STREAM_SHA256 = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'

P7_INTEGRATION_METADATA_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-p7-integration-metadata-20261018'
)
P7_PERSISTENT_QUARANTINE_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p7-persistent-quarantine-20261018-01'
)
P7_ATTEMPT_RESULT_PATH = (
    P7_INTEGRATION_METADATA_ROOT
    + '/actual-p7-config-and-cached-compile-01/actual-sequence-01/RESULT.json'
)
P7_ATTEMPT_RESULT_SHA256 = '24b6547af10fba6527beae986d76adabb150e6bfda841036f92a599e155e75d2'
P7_ATTEMPT_RESULT_BYTES = 1024
P7_ATTEMPT_ELAPSED_SECONDS = 134.75891540400335
P7_ATTEMPT_STDOUT_SHA256 = '593a73e0c81284b558ec8b07b4fad37bc3ac45da7ba4788121c162d91593cec5'
P7_ATTEMPT_STDOUT_BYTES = 2956
P7_ATTEMPT_STDERR_SHA256 = '9b4078eb89bbadd2597138fa1536d7732ccfa032574ab7218427a91f1a3a28bd'
P7_ATTEMPT_STDERR_BYTES = 39
P7_RAW_BEFORE_CLEANUP_PATH = P7_INTEGRATION_METADATA_ROOT + '/post-terminal-actual-01/RAW-CGROUP-AND-MARKER.json'
P7_RAW_BEFORE_CLEANUP_SHA256 = 'a557677833cac929ef292953cefb7ba90d8f961c8aaf11959851f125d07d2a5b'
P7_RAW_BEFORE_CLEANUP_BYTES = 2225
P7_MARKER_PATH = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-sdk-compile-20261018-06/ONE_SHOT_STARTED'
P7_MARKER_SHA256 = '37cb06c8f02da9953c59a7186a4bbdca294494149fdf65f831d8b493ae5bcf67'
P7_MARKER_BYTES = 9
P7_PRESERVE_CLEANUP_RESULT_PATH = P7_INTEGRATION_METADATA_ROOT + '/actual-p7-preserve-and-cleanup-01/RESULT.json'
P7_PRESERVE_CLEANUP_RESULT_SHA256 = 'a16dada3c50cbad959b10e82e81b33a0d94c0ef9474772cf88af1e64d27403c5'
P7_PRESERVE_CLEANUP_RESULT_BYTES = 5823
P7_COLLECT_STDOUT_SHA256 = 'bc21db5edf04a7b73e3c813e0fc2d73db4610576ac39775953c3c08d99783a32'
P7_COLLECT_STDOUT_BYTES = 165
P7_TEARDOWN_STDOUT_SHA256 = '711c09e6d455edcc1a5c02ef433342689ce283699cc5ebf2541f4af664a18c41'
P7_TEARDOWN_STDOUT_BYTES = 128
P7_CLEANUP_STEPS = (
    'create_fresh_persistent_destination',
    'copy_exact_current_regular_files',
    'strip_all_write_execute_bits',
    'verify_every_copied_hash',
    'Root_verifies_persistent_copy_metadata',
    'collect_evidence_existing_operator_ordinary',
    'teardown_existing_operator_sudo',
)


def _verify_p7_receipts(debits, guard):
    """Verify exact P7 attempt, pre-cleanup raw cgroup and cleanup receipts against the once-only P7 debits."""
    objs = []
    for path, size, sha, what in (
        (P7_ATTEMPT_RESULT_PATH, P7_ATTEMPT_RESULT_BYTES, P7_ATTEMPT_RESULT_SHA256, 'P7 attempt result'),
        (P7_RAW_BEFORE_CLEANUP_PATH, P7_RAW_BEFORE_CLEANUP_BYTES, P7_RAW_BEFORE_CLEANUP_SHA256, 'P7 raw cgroup/marker'),
        (P7_PRESERVE_CLEANUP_RESULT_PATH, P7_PRESERVE_CLEANUP_RESULT_BYTES, P7_PRESERVE_CLEANUP_RESULT_SHA256, 'P7 preserve/cleanup'),
    ):
        p = Path(path)
        guard.regular(p)
        guard.need(p.stat().st_size == size and guard.digest(p) == sha, what + ' receipt SHA-256 or byte size mismatch')
        objs.append(guard.jload(p))
    res, raw_obj, cln = objs
    streams = res.get('streams') if isinstance(res, dict) else None
    elapsed = res.get('elapsed_seconds') if isinstance(res, dict) else None
    guard.need(
        isinstance(res, dict)
        and res.get('id') == 'P7-fresh-setup-current-stat-issued-DAG-and-changed-native-compile'
        and res.get('actual_exit') == 2
        and 'controller_failure' in res
        and res['controller_failure'] is None
        and type(elapsed) is float
        and elapsed == P7_ATTEMPT_ELAPSED_SECONDS
        and int(-(-elapsed // 1)) == debits['p7_outer_wall_seconds_ceil']
        and isinstance(streams, list)
        and len(streams) == 2
        and all(isinstance(s, dict) and s.get('full_stream_retained') is True for s in streams)
        and [(s.get('bytes'), s.get('sha256')) for s in streams] == [
            (P7_ATTEMPT_STDOUT_BYTES, P7_ATTEMPT_STDOUT_SHA256),
            (P7_ATTEMPT_STDERR_BYTES, P7_ATTEMPT_STDERR_SHA256),
        ],
        'P7 attempt result receipt content mismatch',
    )
    raw = raw_obj.get('raw') if isinstance(raw_obj, dict) else None
    nodes = [raw.get(k) for k in ('E', 'S', 'P')] if isinstance(raw, dict) else []
    usages = (debits['retired_p7_envelope_usec'], debits['retired_p7_supervisor_usec'], debits['retired_p7_phases_usec'])
    guard.need(
        isinstance(raw_obj, dict)
        and raw_obj.get('CargoDidStart') is True
        and raw_obj.get('nativeOutputsAccepted') is False
        and len(nodes) == 3
        and all(isinstance(n, dict) for n in nodes)
        and raw.get('marker') == {'path': P7_MARKER_PATH, 'bytes': P7_MARKER_BYTES, 'sha256': P7_MARKER_SHA256}
        and all(
            str(n.get('cpu.stat', '')).splitlines()[:1] == ['usage_usec ' + str(u)]
            and str(n.get('cgroup.events', '')).splitlines()[:1] == ['populated 0']
            and n.get('cgroup.procs') == ''
            and n.get('pids.current') == '0\n'
            and '\noom 0\n' in str(n.get('memory.events', ''))
            and '\noom_kill 0\n' in str(n.get('memory.events', ''))
            and '\noom_kill 0\n' in str(n.get('memory.events.local', ''))
            for n, u in zip(nodes, usages)
        )
        and str(nodes[1].get('memory.peak', '')).strip() == '80003072'
        and str(nodes[2].get('memory.peak', '')).strip() == '5649174528',
        'P7 raw cgroup/marker receipt content mismatch against historical debits',
    )
    steps = cln.get('steps') if isinstance(cln, dict) else None
    guard.need(
        isinstance(cln, dict)
        and cln.get('status') == 'P7_PRESERVED_AND_CLEANED'
        and cln.get('cleanupOwed') is False
        and cln.get('nativeOutputsAccepted') is False
        and cln.get('ELFExecuted') is False
        and cln.get('CargoDidStart') is True
        and cln.get('ordinaryRustCompilerErrors') == 2
        and cln.get('targetFilesPreserved') == 3286
        and cln.get('allFilesPreserved') == 3299
        and cln.get('totalBytesPreserved') == 1985171956
        and cln.get('quarantine') == P7_PERSISTENT_QUARANTINE_ROOT
        and isinstance(steps, list)
        and len(steps) == len(P7_CLEANUP_STEPS)
        and all(isinstance(s, dict) for s in steps)
        and tuple(s.get('step') for s in steps) == P7_CLEANUP_STEPS
        and all(s.get('actualExit') == 0 for i, s in enumerate(steps) if i != 4)
        and steps[4].get('exactMembershipHashesSizesAndNonexecModes') is True
        and steps[4].get('files') == 3299
        and steps[5].get('streams', [{}])[0].get('sha256') == P7_COLLECT_STDOUT_SHA256
        and steps[5].get('streams', [{}])[0].get('bytes') == P7_COLLECT_STDOUT_BYTES
        and steps[6].get('streams', [{}])[0].get('sha256') == P7_TEARDOWN_STDOUT_SHA256
        and steps[6].get('streams', [{}])[0].get('bytes') == P7_TEARDOWN_STDOUT_BYTES,
        'P7 preserve/cleanup receipt status invalid',
    )


P8_INTEGRATION_METADATA_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-current-cached-p8-integration-metadata-20261018'
)
P8_PERSISTENT_QUARANTINE_ROOT = (
    '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/'
    'native06-p8-persistent-quarantine-20261018-01'
)
P8_ATTEMPT_RESULT_PATH = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-current-cached-p8-integration-metadata-20261018/actual-sequence-01/RESULT.json'
P8_ATTEMPT_RESULT_SHA256 = '1747fbf1091b4096c4ea276e3e143ebc474f08f60e535aba072022889d01d38a'
P8_ATTEMPT_RESULT_BYTES = 953
P8_RAW_BEFORE_CLEANUP_PATH = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-current-cached-p8-integration-metadata-20261018/post-terminal-actual-01/RAW-CGROUP-AND-MARKER.json'
P8_RAW_BEFORE_CLEANUP_SHA256 = 'c1195a3551e9ba4119a37cb2a9549462f7509d3e3bff01e0cfce0db382350f44'
P8_RAW_BEFORE_CLEANUP_BYTES = 5899
P8_PRESERVE_CLEANUP_RESULT_PATH = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-current-cached-p8-integration-metadata-20261018/actual-p8-preserve-and-cleanup-01/RESULT.json'
P8_PRESERVE_CLEANUP_RESULT_SHA256 = 'cb7febe2e98c2a600a273cc2817b5a02e99b2ff497594c184db9354742ca952b'
P8_PRESERVE_CLEANUP_RESULT_BYTES = 9014
P8_TERMINAL_FAILURE_PATH = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p8-persistent-quarantine-20261018-01/packet/TERMINAL_FAILURE.json'
P8_TERMINAL_FAILURE_SHA256 = '4fe0c644ddeeeb6c9919311ba1cef8355f8de87539c5b20d5f7818c458c94a52'
P8_TERMINAL_FAILURE_BYTES = 111
P8_LOG_BASIS_PATH = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/native06-p8-once-only-accounting-completion-opus-source-request-20261018/inputs/ACTUAL-CUMULATIVE-F-LOG-BYTE-BASIS.json'
P8_LOG_BASIS_SHA256 = '2359c49bca5d66383cab427efdfd26246c0d415a4aab94e48a7408eff7f94ccc'
P8_LOG_BASIS_BYTES = 7739
P8_ATTEMPT_ELAPSED_SECONDS = 132.46132113799104
P8_ATTEMPT_STDOUT_SHA256 = '5b12a998da8bfa80df2ab35a51d761d3018fb7394b144178507e5c7c51c86ba9'
P8_ATTEMPT_STDOUT_BYTES = 2990
P8_ATTEMPT_STDERR_SHA256 = '9b4078eb89bbadd2597138fa1536d7732ccfa032574ab7218427a91f1a3a28bd'
P8_ATTEMPT_STDERR_BYTES = 39
P8_COLLECT_STDOUT_SHA256 = '8527b3723c3feaee209027a88fbb123a1d7cf1f7ce39614eb7fa99b50477c7df'
P8_TEARDOWN_STDOUT_SHA256 = '845999dd73a18885b43917a80a6cc7c77841126bc27a06c96f5a754e03acbc22'
P8_POST_AUDIT_FAILURE = 'third-party host output mtime changed during cached Phase F: debug/build/aws-lc-sys-5469586605af0f08/build-script-main'


def _verify_p8_receipts(debits, guard):
    """Bind the failed P8 once-only charge to its immutable terminal, CPU, cleanup and stream receipts."""
    objs = []
    for path, size, sha, what in (
        (P8_ATTEMPT_RESULT_PATH, P8_ATTEMPT_RESULT_BYTES, P8_ATTEMPT_RESULT_SHA256, 'P8 attempt result'),
        (P8_RAW_BEFORE_CLEANUP_PATH, P8_RAW_BEFORE_CLEANUP_BYTES, P8_RAW_BEFORE_CLEANUP_SHA256, 'P8 raw cgroup/marker'),
        (P8_PRESERVE_CLEANUP_RESULT_PATH, P8_PRESERVE_CLEANUP_RESULT_BYTES, P8_PRESERVE_CLEANUP_RESULT_SHA256, 'P8 preserve/cleanup'),
        (P8_TERMINAL_FAILURE_PATH, P8_TERMINAL_FAILURE_BYTES, P8_TERMINAL_FAILURE_SHA256, 'P8 terminal failure'),
        (P8_LOG_BASIS_PATH, P8_LOG_BASIS_BYTES, P8_LOG_BASIS_SHA256, 'P8 cumulative F log basis'),
    ):
        p = Path(path)
        guard.regular(p)
        guard.need(p.stat().st_size == size and guard.digest(p) == sha, what + ' receipt SHA-256 or byte size mismatch')
        objs.append(guard.jload(p))
    res, raw_obj, cln, terminal, basis = objs
    streams = res.get('streams') if isinstance(res, dict) else None
    elapsed = res.get('elapsed_seconds') if isinstance(res, dict) else None
    guard.need(
        isinstance(res, dict)
        and res.get('id') == 'P8-fresh-setup-current-stat-issued-DAG-and-two-test-fixed-native-compile'
        and res.get('actual_exit') == 2
        and 'controller_failure' in res
        and res['controller_failure'] is None
        and type(elapsed) is float
        and elapsed == P8_ATTEMPT_ELAPSED_SECONDS
        and int(-(-elapsed // 1)) == debits['p8_outer_wall_seconds_ceil']
        and isinstance(streams, list)
        and len(streams) == 2
        and all(isinstance(st, dict) and st.get('full_stream_retained') is True for st in streams)
        and [(st.get('bytes'), st.get('sha256')) for st in streams] == [
            (P8_ATTEMPT_STDOUT_BYTES, P8_ATTEMPT_STDOUT_SHA256),
            (P8_ATTEMPT_STDERR_BYTES, P8_ATTEMPT_STDERR_SHA256),
        ],
        'P8 failed attempt result receipt content mismatch',
    )
    guard.need(
        terminal == {'message': 'phase exit nonzero F', 'native_outputs_accepted': False,
                     'route': 'cached-current-f', 'type': 'Refusal'},
        'P8 terminal failure receipt content mismatch',
    )
    raw = raw_obj.get('raw') if isinstance(raw_obj, dict) else None
    nodes = [raw.get(k) for k in ('E', 'S', 'P')] if isinstance(raw, dict) else []
    usages = (debits['retired_p8_envelope_usec'], debits['retired_p8_supervisor_usec'], debits['retired_p8_phases_usec'])
    guard.need(
        isinstance(raw_obj, dict)
        and raw_obj.get('cleanupOwed') is True
        and raw_obj.get('CargoDidStart') is True
        and raw_obj.get('CargoCompileReportedFinished') is True
        and raw_obj.get('nativeOutputsAccepted') is False
        and raw_obj.get('ELFExecuted') is False
        and raw_obj.get('postAuditFailure') == P8_POST_AUDIT_FAILURE
        and len(nodes) == 3
        and all(isinstance(n, dict) for n in nodes)
        and raw.get('marker') == {'path': P7_MARKER_PATH, 'bytes': P7_MARKER_BYTES, 'sha256': P7_MARKER_SHA256}
        and all(
            str(n.get('cpu.stat', '')).splitlines()[:1] == ['usage_usec ' + str(u)]
            and str(n.get('cgroup.events', '')).splitlines()[:1] == ['populated 0']
            and n.get('cgroup.procs') == ''
            and n.get('pids.current') == '0\n'
            and '\noom 0\n' in str(n.get('memory.events', ''))
            and '\noom_kill 0\n' in str(n.get('memory.events', ''))
            and '\noom_kill 0\n' in str(n.get('memory.events.local', ''))
            for n, u in zip(nodes, usages)
        )
        and [str(n.get('memory.peak', '')).strip() for n in nodes] == ['5911670784', '71958528', '5859758080'],
        'P8 raw cgroup/marker receipt content mismatch against historical debits',
    )
    steps = cln.get('steps') if isinstance(cln, dict) else None
    guard.need(
        isinstance(cln, dict)
        and cln.get('status') == 'P8_PRESERVED_AND_CLEANED'
        and cln.get('cleanupOwed') is False
        and cln.get('nativeOutputsAccepted') is False
        and cln.get('ELFExecuted') is False
        and cln.get('CargoDidStart') is True
        and cln.get('CargoCompileReportedSuccess') is True
        and cln.get('CargoBuildSecondsReported') == 55.6
        and cln.get('guardPostAudit') == 'FAILED aws-lc-sys build-script-main mtime changed'
        and cln.get('targetFilesPreserved') == 3548
        and cln.get('allFilesPreserved') == 3565
        and cln.get('totalBytesPreserved') == 2847417755
        and cln.get('quarantine') == P8_PERSISTENT_QUARANTINE_ROOT
        and cln.get('cgroupAbsent') is True
        and cln.get('executionRootEmpty') is True
        and isinstance(steps, list)
        and len(steps) == len(P7_CLEANUP_STEPS)
        and all(isinstance(st, dict) for st in steps)
        and tuple(st.get('step') for st in steps) == P7_CLEANUP_STEPS
        and all(st.get('actualExit') == 0 for i, st in enumerate(steps) if i != 4)
        and steps[4].get('exactMembershipHashesSizesAndNonexecModes') is True
        and steps[4].get('files') == 3565
        and steps[5].get('streams', [{}])[0].get('sha256') == P8_COLLECT_STDOUT_SHA256
        and steps[5].get('streams', [{}])[0].get('bytes') == 165
        and steps[6].get('streams', [{}])[0].get('sha256') == P8_TEARDOWN_STDOUT_SHA256
        and steps[6].get('streams', [{}])[0].get('bytes') == 128,
        'P8 preserve/cleanup receipt status invalid',
    )
    rows = basis.get('rows') if isinstance(basis, dict) else None
    guard.need(
        isinstance(basis, dict)
        and basis.get('status') == 'SAVED_PUBLIC_STREAM_BYTE_PINS_AND_ARITHMETIC_NOT_SOURCE_APPROVAL'
        and isinstance(rows, list)
        and len(rows) == 3
        and all(isinstance(row, dict) for row in rows)
        and [row.get('attempt') for row in rows] == ['r3', 'P7', 'P8'],
        'P8 cumulative F stream basis attempts invalid',
    )
    totals = {'stdout': 0, 'stderr': 0}
    seen = set()
    for row, expected_count, stderr_bytes in zip(rows, (12, 4, 6), (10406, 12829, 10966)):
        files = row.get('stream_files')
        guard.need(isinstance(files, list) and len(files) == expected_count, 'P8 F stream membership count mismatch')
        subtotal = {'stdout': 0, 'stderr': 0}
        for stream in files:
            guard.need(isinstance(stream, dict) and isinstance(stream.get('path'), str), 'P8 F stream row invalid')
            sp = Path(stream['path'])
            guard.regular(sp)
            resolved = str(sp.resolve(strict=True))
            guard.need(resolved not in seen, 'P8 F stream counted more than once')
            seen.add(resolved)
            kind = sp.suffix.lstrip('.')
            guard.need(kind in subtotal, 'P8 F stream kind invalid')
            size = stream.get('bytes')
            guard.need(type(size) is int and size >= 0 and sp.stat().st_size == size
                       and guard.digest(sp) == stream.get('sha256'), 'P8 F stream SHA-256 or byte size mismatch')
            subtotal[kind] += size
        guard.need(subtotal == {'stdout': 0, 'stderr': stderr_bytes} and row.get('subtotal') == subtotal,
                   'P8 F stream subtotal mismatch')
        for kind in totals:
            totals[kind] += subtotal[kind]
    guard.need(
        totals == {'stdout': 0, 'stderr': 34201}
        and basis.get('new_cumulative_all_F_stdout') == totals['stdout'] == debits['f_stdout_bytes']
        and basis.get('new_cumulative_all_F_stderr') == totals['stderr'] == debits['f_stderr_bytes'] - 8880,
        'P8 cumulative F stream arithmetic mismatch against historical debits',
    )


def _verify_p9_receipts(debits, guard):
    """Bind the successful P9 once-only charge to its F, seed, raw-counter and preserve/cleanup receipts."""
    f_path = Path(P9_PERSISTENT_QUARANTINE_ROOT) / 'packet' / 'F_CACHED_CURRENT_RESULT.json'
    for p, size, sha, what in (
        (f_path, P9_F_RESULT_BYTES, P9_PACKET_PINS['F_CACHED_CURRENT_RESULT.json'], 'P9 F result'),
        (Path(P9_RAW_COUNTERS_PATH), P9_RAW_COUNTERS_BYTES, P9_RAW_COUNTERS_SHA256, 'P9 raw cgroup/marker'),
        (Path(P9_PRESERVE_CLEANUP_RESULT_PATH), P9_PRESERVE_CLEANUP_RESULT_BYTES, P9_PRESERVE_CLEANUP_RESULT_SHA256, 'P9 preserve/cleanup'),
        (Path(P9_QUARANTINE_VERIFIED_PATH), P9_QUARANTINE_VERIFIED_BYTES, P9_QUARANTINE_VERIFIED_SHA256, 'P9 quarantine verified'),
        (Path(P9_COPY_MAP_PATH), P9_COPY_MAP_BYTES, P9_COPY_MAP_SHA256, 'P9 preservation copy-map'),
    ):
        guard.regular(p)
        guard.need(p.stat().st_size == size and guard.digest(p) == sha, what + ' receipt SHA-256 or byte size mismatch')
    pkt = Path(P9_PERSISTENT_QUARANTINE_ROOT) / 'packet'
    for name, sha in P9_PACKET_PINS.items():
        pp = pkt / name
        guard.regular(pp)
        guard.need(guard.digest(pp) == sha, 'P9 packet SHA-256 mismatch: ' + name)
    seed_pkt = guard.jload(pkt / 'CACHED_TARGET_SEED.json')
    seeded = seed_pkt.get('seeded_files') if isinstance(seed_pkt, dict) else None
    hosted = seed_pkt.get('host_outputs_seeded') if isinstance(seed_pkt, dict) else None
    units = seed_pkt.get('run_unit_snapshots') if isinstance(seed_pkt, dict) else None
    guard.need(
        isinstance(seed_pkt, dict)
        and seed_pkt.get('quarantine_root') == PRESERVED_QUARANTINE_ROOT
        and seed_pkt.get('preserved_copy_map_sha256') == P9_SEED_PACKET_PRESERVED_COPY_MAP_SHA256
        and seed_pkt.get('r3_quarantined_outputs_sha256') == PRESERVED_PACKET_PINS['F_RETAINED_QUARANTINED_OUTPUTS_R3.json']
        and seed_pkt.get('partition_counts') == P9_HISTORICAL_SEED_PARTITION_COUNTS
        and seed_pkt.get('excluded_workspace_count') == 655
        and seed_pkt.get('seeded_count') == P9_HISTORICAL_SEED_PARTITION_COUNTS['unadopted_other_cache_candidate']
        and seed_pkt.get('host_outputs_count') == P9_PARTITION_COUNTS['host_outputs_total']
        and seed_pkt.get('build_script_run_units_count') == P9_PARTITION_COUNTS['build_script_run_units']
        and isinstance(seeded, dict)
        and len(seeded) == P9_HISTORICAL_SEED_PARTITION_COUNTS['unadopted_other_cache_candidate']
        and isinstance(hosted, dict)
        and len(hosted) == P9_PARTITION_COUNTS['host_outputs_total']
        and isinstance(units, dict)
        and len(units) == P9_PARTITION_COUNTS['build_script_run_units']
        and CURRENT_P9_BACKEND_ELF_REL not in seeded
        and all(isinstance(v, dict) and v.get('sha256') != CURRENT_P9_BACKEND_ELF_SHA256 for v in seeded.values()),
        'P9 historical R3-basis seed packet content mismatch',
    )
    quarantined = guard.jload(pkt / 'F_QUARANTINED_OUTPUTS.json')
    all_outputs = quarantined.get('all_outputs') if isinstance(quarantined, dict) else None
    host_outputs = quarantined.get('host_outputs') if isinstance(quarantined, dict) else None
    guard.need(
        isinstance(quarantined, dict)
        and isinstance(all_outputs, dict)
        and len(all_outputs) == P9_PARTITION_COUNTS['target_total']
        and isinstance(host_outputs, list)
        and len(host_outputs) == P9_PARTITION_COUNTS['host_outputs_total']
        and all_outputs.get(CURRENT_P9_BACKEND_ELF_REL) == CURRENT_P9_BACKEND_ELF_SHA256
        and all(all_outputs.get(rel) == meta.get('sha256') for rel, meta in seeded.items()),
        'P9 quarantined outputs differ from the historical seeded candidate keys or hashes',
    )
    f_res = guard.jload(f_path)
    post = f_res.get('post_compile_check', {}) if isinstance(f_res, dict) else {}
    fresh = post.get('fresh_mori_backend_executables') if isinstance(post, dict) else None
    f_result = f_res.get('f_result', {}) if isinstance(f_res, dict) else {}
    log_bytes = f_result.get('log_bytes', {}) if isinstance(f_result, dict) else {}
    guard.need(
        isinstance(f_res, dict)
        and f_res.get('base_head') == '539ab417d92cf7f4dfe8655fee810ea5d2a41c89'
        and f_res.get('route') == 'cached-current-f'
        and f_res.get('cargo_lock_sha256') == guard.LOCK
        and f_res.get('cached_target_seed_counts') == P9_HISTORICAL_SEED_PARTITION_COUNTS
        and isinstance(f_result, dict)
        and isinstance(log_bytes, dict)
        and log_bytes == {'stdout': 0, 'stderr': 8880}
        and debits.get('f_stdout_bytes') == log_bytes['stdout']
        and debits.get('f_stderr_bytes') == 34201 + log_bytes['stderr']
        and debits.get('p9_outer_wall_seconds_ceil') == 121
        and post.get('host_outputs_verified_unchanged') == 90
        and post.get('build_script_run_units_verified_not_reexecuted') == 34
        and isinstance(fresh, list)
        and len(fresh) == 1
        and fresh[0].get('relative') == CURRENT_P9_BACKEND_ELF_REL
        and fresh[0].get('sha256') == CURRENT_P9_BACKEND_ELF_SHA256
        and fresh[0].get('bytes') == CURRENT_P9_BACKEND_ELF_BYTES,
        'P9 successful F result receipt content mismatch',
    )
    raw_obj = guard.jload(Path(P9_RAW_COUNTERS_PATH))
    raw = raw_obj.get('raw') if isinstance(raw_obj, dict) else None
    guard.need(
        isinstance(raw_obj, dict)
        and raw_obj.get('guardTerminal') is True
        and raw_obj.get('guardExit') == 0
        and raw_obj.get('noELFExecuted') is True
        and raw_obj.get('outputsAccepted') is False
        and isinstance(raw, dict)
        and str(raw.get('cpu.stat', '')).splitlines()[:1] == ['usage_usec ' + str(debits['retired_p9_envelope_usec'])]
        and str(raw.get('supervisor/cpu.stat', '')).splitlines()[:1] == ['usage_usec ' + str(debits['retired_p9_supervisor_usec'])]
        and str(raw.get('phases/cpu.stat', '')).splitlines()[:1] == ['usage_usec ' + str(debits['retired_p9_phases_usec'])]
        and raw.get('marker') == {'path': P7_MARKER_PATH, 'bytes': P7_MARKER_BYTES, 'sha256': P7_MARKER_SHA256, 'content': 'no retry\n'}
        and str(raw.get('cgroup.events', '')).splitlines()[:1] == ['populated 0']
        and raw.get('cgroup.procs') == ''
        and '\noom_kill 0\n' in str(raw.get('memory.events', '')),
        'P9 raw cgroup/marker receipt content mismatch against once-only debits',
    )
    cln = guard.jload(Path(P9_PRESERVE_CLEANUP_RESULT_PATH))
    steps = cln.get('steps') if isinstance(cln, dict) else None
    guard.need(
        isinstance(cln, dict)
        and cln.get('status') == 'P9_PRESERVED_AND_CLEANED'
        and cln.get('cleanupOwed') is False
        and cln.get('nativeOutputsAccepted') is False
        and cln.get('ELFExecuted') is False
        and cln.get('CargoDidStart') is True
        and cln.get('CargoCompileReportedSuccess') is True
        and cln.get('CargoBuildSecondsReported') == 25.69
        and cln.get('guardPostAudit') == 'PASS 34buildscriptunitsnotreexecuted90hostoutputsunchanged'
        and cln.get('targetFilesPreserved') == P9_PARTITION_COUNTS['target_total']
        and cln.get('allFilesPreserved') == P9_PARTITION_COUNTS['total_preserved_files']
        and cln.get('totalBytesPreserved') == 2847459477
        and cln.get('quarantine') == P9_PERSISTENT_QUARANTINE_ROOT
        and cln.get('cgroupAbsent') is True
        and cln.get('executionRootEmpty') is True
        and isinstance(steps, list)
        and len(steps) == len(P7_CLEANUP_STEPS)
        and tuple(s.get('step') for s in steps) == P7_CLEANUP_STEPS
        and all(s.get('actualExit') == 0 for i, s in enumerate(steps) if i != 4)
        and steps[4].get('exactMembershipHashesSizesAndNonexecModes') is True
        and steps[4].get('files') == 3566
        and steps[5].get('streams', [{}])[0].get('sha256') == '326cc1061f54410d646200b669de60d789da9233896633afda799170704b63ba'
        and steps[5].get('streams', [{}])[0].get('bytes') == 165
        and steps[6].get('streams', [{}])[0].get('sha256') == 'eb37eddb1cd7cd595e6797be89516814f7383f1bb1a8f7cb55689079af95e88a'
        and steps[6].get('streams', [{}])[0].get('bytes') == 128,
        'P9 preserve/cleanup receipt status invalid',
    )


def prepare_cached_current_f_worker(c, guard):
    """Execute source_home reconstitution and P10-basis read-only-copy target seeding inside the bounded F_precheck phase leaf."""
    expected_cg = '0::' + guard.phase_cgroup(c, 'F')
    actual_cg = guard._cg_text('/proc/self/cgroup').strip()
    guard.need(
        actual_cg == expected_cg,
        'cached F preparation must run inside phase F leaf cgroup, not supervisor: ' + actual_cg,
    )
    p = c['paths']
    for k in ('source_home', 'target'):
        d = Path(p[k])
        guard.need(d.is_dir() and not any(d.iterdir()), 'cached F worker requires empty ' + k + ' before first write')
    # Refuse a bad seed copy before any source_home write; seeding re-verifies it immediately before use.
    _verify_p11_readonly_seed_copy(guard)
    reconstitution = reconstitute_source_home(c, guard)
    seed_snapshot = verify_and_seed_cached_target_p11(c, guard)
    return {
        'reconstitution': reconstitution,
        'seed_snapshot': seed_snapshot,
    }

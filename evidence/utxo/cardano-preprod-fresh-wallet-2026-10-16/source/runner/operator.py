#!/usr/bin/env python3
"""Create exactly one fresh Cardano Preprod payment keypair.

Security boundary:
- Python standard library only.
- No network, signing, transaction, query, submission, or reuse of old keys.
- The native cardano-cli is run only under a separately supplied, exact,
  one-run AUTH file. AUTH-CANDIDATE.json is never accepted as authority.
- The signing key is never opened, read, copied, hashed, or printed by this
  operator. Only its presence and file mode are checked with lstat.
- Raw native stdout/stderr are retained only under the fresh private root.
"""

import hashlib
import json
import os
import re
import resource
import stat
import subprocess
import sys
import tarfile
import time
from pathlib import Path

BASE = Path("/home/charl/moriarty-eutxo-validation-2026-10-12")
PUBLIC_ROOT = BASE / "current-cardano-preprod-fresh-wallet-public-20261016"
PRIVATE_ROOT = Path(
    "/home/charl/.local/share/moriarty/cardano-preprod/"
    "moriarty-utxo-20261016"
)
ARCHIVE = (
    BASE
    / "cardano-cli-release-data-20261015"
    / "cardano-cli-11.3.1.0-x86_64-linux.tar.gz"
)
ARCHIVE_BYTES = 24057685
ARCHIVE_SHA256 = "330d20a904d5f7526a0ba22edec6ba8fe5670978e8a37bf83c27d4b63faeeebe"
SOURCE_ARCHIVE_SHA256 = "20ba6e5711a1f8b8aa916cc87001baa04862c19358b9bf2aaebdb9425dc536e2"
SOURCE_COMMIT = "fc466f22ca5c03c0e373e2d37053b7f1884c50c9"
MEMBER_NAME = "cardano-cli-x86_64-linux"
MEMBER_BYTES = 102556168
CLI_VERSION = "11.3.1.0"
NETWORK_NAME = "Cardano Preprod"
NETWORK_MAGIC = 1
MAX_CLI_CALLS = 4
TOTAL_WALL_SECONDS = 120
PER_CALL_TIMEOUT_SECONDS = 30
PER_CALL_CPU_SECONDS = 15
HEX64 = re.compile(r"^[0-9a-f]{64}$")
HEX56 = re.compile(r"^[0-9a-f]{56}$")
ADDR_RE = re.compile(r"^addr_test1[023456789acdefghjklmnpqrstuvwxyz]+$")

AUTH_KEYS = {
    "format",
    "status",
    "operatorSha256",
    "archivePath",
    "archiveBytes",
    "archiveSha256",
    "sourceArchiveSha256",
    "sourceCommit",
    "cliVersion",
    "memberName",
    "memberBytes",
    "expectedStagedBinarySha256",
    "binaryLinkage",
    "runtimeBinds",
    "publicRoot",
    "privateRoot",
    "networkName",
    "networkMagic",
    "mainnet",
    "attemptsAuthorized",
    "retriesAuthorized",
    "maxCliCalls",
    "totalWallSeconds",
    "perCallTimeoutSeconds",
    "signingAuthorized",
    "transactionAuthorized",
    "networkAccessAuthorized",
    "oldKeyUseAuthorized",
}


class Fail(Exception):
    def __init__(self, category):
        super().__init__(category)
        self.category = category


def is_int(value):
    return type(value) is int


def is_bool(value):
    return type(value) is bool


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        while True:
            chunk = handle.read(1024 * 1024)
            if not chunk:
                break
            digest.update(chunk)
    return digest.hexdigest()


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def fsync_dir(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def write_exclusive(path, data, mode):
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, mode)
    try:
        view = memoryview(data)
        while view:
            written = os.write(fd, view)
            view = view[written:]
        os.fchmod(fd, mode)
        os.fsync(fd)
    finally:
        os.close(fd)
    fsync_dir(path.parent)


def write_json_exclusive(path, value, mode):
    data = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    write_exclusive(path, data, mode)
    return sha256_bytes(data)


def replace_json(path, value, mode):
    data = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    tmp = path.with_name(path.name + ".tmp")
    write_exclusive(tmp, data, mode)
    os.replace(tmp, path)
    fsync_dir(path.parent)
    return sha256_bytes(data)


def require_dir_nolink(path, category):
    try:
        info = os.lstat(path)
    except FileNotFoundError:
        raise Fail(category)
    if stat.S_ISLNK(info.st_mode) or not stat.S_ISDIR(info.st_mode):
        raise Fail(category)
    return info


def require_file_nolink(path, category):
    try:
        info = os.lstat(path)
    except FileNotFoundError:
        raise Fail(category)
    if stat.S_ISLNK(info.st_mode) or not stat.S_ISREG(info.st_mode):
        raise Fail(category)
    return info


def ensure_private_parents():
    current = Path(PRIVATE_ROOT.anchor)
    for part in PRIVATE_ROOT.parts[1:-1]:
        current = current / part
        try:
            info = os.lstat(current)
        except FileNotFoundError:
            os.mkdir(current, 0o700)
            os.chmod(current, 0o700)
            continue
        if stat.S_ISLNK(info.st_mode) or not stat.S_ISDIR(info.st_mode):
            raise Fail("PRIVATE_PARENT_UNSAFE")


def create_fresh_roots():
    require_dir_nolink(BASE, "PUBLIC_PARENT_UNSAFE")
    ensure_private_parents()
    try:
        os.mkdir(PUBLIC_ROOT, 0o755)
    except FileExistsError:
        raise Fail("PUBLIC_ROOT_ALREADY_EXISTS")
    os.chmod(PUBLIC_ROOT, 0o755)
    try:
        os.mkdir(PRIVATE_ROOT, 0o700)
    except FileExistsError:
        # The public root now records a spent, non-retryable attempt.
        raise Fail("PRIVATE_ROOT_ALREADY_EXISTS")
    os.chmod(PRIVATE_ROOT, 0o700)
    os.mkdir(PUBLIC_ROOT / "bin", 0o755)
    os.mkdir(PRIVATE_ROOT / "logs", 0o700)
    os.chmod(PRIVATE_ROOT / "logs", 0o700)
    fsync_dir(PUBLIC_ROOT)
    fsync_dir(PRIVATE_ROOT)


def validate_runtime_binds(value):
    if type(value) is not list:
        raise Fail("AUTH_RUNTIME_BINDS_INVALID")
    binds = []
    destinations = set()
    for item in value:
        if type(item) is not dict or set(item.keys()) != {"source", "destination"}:
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        source = item["source"]
        destination = item["destination"]
        if type(source) is not str or type(destination) is not str:
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        if not source.startswith("/") or not destination.startswith("/"):
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        if source.startswith("/home/") or destination.startswith("/home/"):
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        if ".." in Path(source).parts or ".." in Path(destination).parts:
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        allowed = ("/lib/", "/lib64/", "/usr/lib/", "/usr/lib64/")
        if not destination.startswith(allowed):
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        if destination in destinations:
            raise Fail("AUTH_RUNTIME_BINDS_INVALID")
        destinations.add(destination)
        info = require_file_nolink(Path(source), "AUTH_RUNTIME_BIND_UNSAFE")
        if info.st_size <= 0:
            raise Fail("AUTH_RUNTIME_BIND_UNSAFE")
        binds.append((source, destination))
    return binds


def load_auth(path_text):
    if type(path_text) is not str or not path_text:
        raise Fail("AUTH_PATH_INVALID")
    auth_path = Path(path_text)
    require_file_nolink(auth_path, "AUTH_FILE_UNSAFE")
    try:
        auth = json.loads(auth_path.read_text(encoding="utf-8"))
    except Exception:
        raise Fail("AUTH_JSON_INVALID")
    if type(auth) is not dict or set(auth.keys()) != AUTH_KEYS:
        raise Fail("AUTH_SCHEMA_INVALID")
    if auth["format"] != "cardano-preprod-fresh-wallet-auth/1":
        raise Fail("AUTH_FORMAT_INVALID")
    if auth["status"] != "AUTHORIZED_ONE_RUN":
        raise Fail("AUTH_NOT_AUTHORIZED")
    if type(auth["operatorSha256"]) is not str or not HEX64.fullmatch(auth["operatorSha256"]):
        raise Fail("AUTH_OPERATOR_HASH_INVALID")
    expected_strings = {
        "archivePath": str(ARCHIVE),
        "sourceArchiveSha256": SOURCE_ARCHIVE_SHA256,
        "sourceCommit": SOURCE_COMMIT,
        "cliVersion": CLI_VERSION,
        "memberName": MEMBER_NAME,
        "publicRoot": str(PUBLIC_ROOT),
        "privateRoot": str(PRIVATE_ROOT),
        "networkName": NETWORK_NAME,
    }
    for key, expected in expected_strings.items():
        if auth[key] != expected:
            raise Fail("AUTH_SCOPE_INVALID")
    if auth["archiveBytes"] != ARCHIVE_BYTES or not is_int(auth["archiveBytes"]):
        raise Fail("AUTH_ARCHIVE_INVALID")
    if auth["archiveSha256"] != ARCHIVE_SHA256:
        raise Fail("AUTH_ARCHIVE_INVALID")
    if auth["memberBytes"] != MEMBER_BYTES or not is_int(auth["memberBytes"]):
        raise Fail("AUTH_MEMBER_INVALID")
    if type(auth["expectedStagedBinarySha256"]) is not str or not HEX64.fullmatch(
        auth["expectedStagedBinarySha256"]
    ):
        raise Fail("AUTH_MEMBER_HASH_UNRESOLVED")
    if auth["binaryLinkage"] not in ("static", "dynamic"):
        raise Fail("AUTH_LINKAGE_UNRESOLVED")
    binds = validate_runtime_binds(auth["runtimeBinds"])
    if auth["binaryLinkage"] == "static" and binds:
        raise Fail("AUTH_STATIC_BINDS_INVALID")
    if auth["binaryLinkage"] == "dynamic" and not binds:
        raise Fail("AUTH_DYNAMIC_BINDS_MISSING")
    if auth["networkMagic"] != NETWORK_MAGIC or not is_int(auth["networkMagic"]):
        raise Fail("AUTH_NETWORK_INVALID")
    if not is_bool(auth["mainnet"]) or auth["mainnet"] is not False:
        raise Fail("AUTH_NETWORK_INVALID")
    counters = {
        "attemptsAuthorized": 1,
        "retriesAuthorized": 0,
        "maxCliCalls": MAX_CLI_CALLS,
        "totalWallSeconds": TOTAL_WALL_SECONDS,
        "perCallTimeoutSeconds": PER_CALL_TIMEOUT_SECONDS,
    }
    for key, expected in counters.items():
        if not is_int(auth[key]) or auth[key] != expected:
            raise Fail("AUTH_LIMIT_INVALID")
    for key in (
        "signingAuthorized",
        "transactionAuthorized",
        "networkAccessAuthorized",
        "oldKeyUseAuthorized",
    ):
        if not is_bool(auth[key]) or auth[key] is not False:
            raise Fail("AUTH_FORBIDDEN_SCOPE")
    return auth, binds, sha256_file(auth_path)


def parse_elf_linkage(path):
    info = require_file_nolink(path, "STAGED_BINARY_UNSAFE")
    if info.st_size != MEMBER_BYTES:
        raise Fail("STAGED_BINARY_SIZE_INVALID")
    with open(path, "rb") as handle:
        header = handle.read(64)
        if len(header) != 64 or header[:4] != b"\x7fELF":
            raise Fail("ELF_HEADER_INVALID")
        if header[4] != 2 or header[5] != 1:
            raise Fail("ELF_CLASS_OR_ENDIAN_INVALID")
        machine = int.from_bytes(header[18:20], "little")
        if machine != 62:
            raise Fail("ELF_MACHINE_INVALID")
        phoff = int.from_bytes(header[32:40], "little")
        phentsize = int.from_bytes(header[54:56], "little")
        phnum = int.from_bytes(header[56:58], "little")
        if phentsize < 56 or phnum <= 0 or phnum > 128:
            raise Fail("ELF_PROGRAM_HEADERS_INVALID")
        interp = None
        dynamic = False
        for index in range(phnum):
            handle.seek(phoff + index * phentsize)
            ph = handle.read(56)
            if len(ph) != 56:
                raise Fail("ELF_PROGRAM_HEADERS_INVALID")
            p_type = int.from_bytes(ph[0:4], "little")
            p_offset = int.from_bytes(ph[8:16], "little")
            p_filesz = int.from_bytes(ph[32:40], "little")
            if p_type == 2:
                dynamic = True
            if p_type == 3:
                if p_filesz <= 1 or p_filesz > 4096:
                    raise Fail("ELF_INTERP_INVALID")
                handle.seek(p_offset)
                raw = handle.read(p_filesz)
                if not raw.endswith(b"\x00"):
                    raise Fail("ELF_INTERP_INVALID")
                try:
                    interp = raw[:-1].decode("ascii")
                except Exception:
                    raise Fail("ELF_INTERP_INVALID")
                if not interp.startswith("/") or ".." in Path(interp).parts:
                    raise Fail("ELF_INTERP_INVALID")
    return {"dynamic": dynamic, "interp": interp}


def extract_binary(auth):
    info = require_file_nolink(ARCHIVE, "ARCHIVE_UNSAFE")
    if info.st_size != ARCHIVE_BYTES:
        raise Fail("ARCHIVE_SIZE_INVALID")
    if sha256_file(ARCHIVE) != ARCHIVE_SHA256:
        raise Fail("ARCHIVE_HASH_INVALID")
    staged_final = PUBLIC_ROOT / "bin" / MEMBER_NAME
    staged_tmp = PUBLIC_ROOT / "bin" / (MEMBER_NAME + ".tmp")
    digest = hashlib.sha256()
    total = 0
    try:
        with tarfile.open(ARCHIVE, "r:gz") as archive:
            members = archive.getmembers()
            if len(members) != 1:
                raise Fail("ARCHIVE_MEMBER_COUNT_INVALID")
            member = members[0]
            if member.name != MEMBER_NAME or not member.isfile():
                raise Fail("ARCHIVE_MEMBER_INVALID")
            if member.issym() or member.islnk() or member.size != MEMBER_BYTES:
                raise Fail("ARCHIVE_MEMBER_INVALID")
            source = archive.extractfile(member)
            if source is None:
                raise Fail("ARCHIVE_MEMBER_UNREADABLE")
            fd = os.open(
                staged_tmp,
                os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
                0o500,
            )
            try:
                while True:
                    chunk = source.read(1024 * 1024)
                    if not chunk:
                        break
                    total += len(chunk)
                    if total > MEMBER_BYTES:
                        raise Fail("ARCHIVE_MEMBER_SIZE_INVALID")
                    digest.update(chunk)
                    view = memoryview(chunk)
                    while view:
                        written = os.write(fd, view)
                        view = view[written:]
                os.fchmod(fd, 0o500)
                os.fsync(fd)
            finally:
                os.close(fd)
    except Fail:
        raise
    except Exception:
        raise Fail("ARCHIVE_EXTRACTION_FAILED")
    if total != MEMBER_BYTES:
        raise Fail("ARCHIVE_MEMBER_SIZE_INVALID")
    measured = digest.hexdigest()
    if measured != auth["expectedStagedBinarySha256"]:
        raise Fail("STAGED_BINARY_HASH_MISMATCH")
    os.replace(staged_tmp, staged_final)
    fsync_dir(staged_final.parent)
    linkage = parse_elf_linkage(staged_final)
    if auth["binaryLinkage"] == "static":
        if linkage["dynamic"] or linkage["interp"] is not None:
            raise Fail("ELF_LINKAGE_MISMATCH")
    else:
        if not linkage["dynamic"] or linkage["interp"] is None:
            raise Fail("ELF_LINKAGE_MISMATCH")
    return staged_final, measured, linkage


def set_limits():
    resource.setrlimit(resource.RLIMIT_CPU, (PER_CALL_CPU_SECONDS, PER_CALL_CPU_SECONDS + 1))
    resource.setrlimit(resource.RLIMIT_FSIZE, (32 * 1024 * 1024, 32 * 1024 * 1024))
    resource.setrlimit(resource.RLIMIT_NOFILE, (128, 128))
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024 * 1024 * 1024, 4 * 1024 * 1024 * 1024))


class Runner:
    def __init__(self, auth, binds, staged, linkage):
        self.auth = auth
        self.binds = binds
        self.staged = staged
        self.linkage = linkage
        self.started = time.monotonic()
        self.calls = []
        self.intent_recorded = False
        self.ledger = {
            "format": "cardano-preprod-fresh-wallet-attempt-ledger/1",
            "attemptsStarted": 0,
            "retries": 0,
            "cliCallsStarted": 0,
            "status": "CREATED_NOT_STARTED",
        }
        write_json_exclusive(PUBLIC_ROOT / "attempt-ledger.json", self.ledger, 0o644)

    def save_ledger(self, status):
        self.ledger["status"] = status
        replace_json(PUBLIC_ROOT / "attempt-ledger.json", self.ledger, 0o644)

    def record_intent(self):
        if not self.intent_recorded:
            self.ledger["attemptsStarted"] = 1
            self.save_ledger("INTENT_RECORDED_BEFORE_FIRST_PROCESS_SPAWN")
            self.intent_recorded = True

    def run_cli(self, name, args):
        if len(self.calls) >= MAX_CLI_CALLS:
            raise Fail("CLI_CALL_LIMIT_EXCEEDED")
        if time.monotonic() - self.started > TOTAL_WALL_SECONDS:
            raise Fail("TOTAL_WALL_LIMIT_EXCEEDED")
        self.record_intent()
        self.ledger["cliCallsStarted"] = len(self.calls) + 1
        self.save_ledger("NATIVE_CALL_STARTED")
        command = [
            "/usr/bin/bwrap",
            "--unshare-all",
            "--die-with-parent",
            "--new-session",
            "--clearenv",
            "--ro-bind",
            str(self.staged),
            "/cli/cardano-cli",
        ]
        for source, destination in self.binds:
            command.extend(["--ro-bind", source, destination])
        command.extend(
            [
                "--bind",
                str(PRIVATE_ROOT),
                "/private",
                "--tmpfs",
                "/tmp",
                "--dev",
                "/dev",
                "--chdir",
                "/private",
                "--setenv",
                "HOME",
                "/private",
                "--setenv",
                "TMPDIR",
                "/tmp",
                "--setenv",
                "LANG",
                "C.UTF-8",
                "--setenv",
                "PATH",
                "/usr/bin:/bin",
                "/cli/cardano-cli",
            ]
        )
        command.extend(args)
        remaining = TOTAL_WALL_SECONDS - (time.monotonic() - self.started)
        timeout = min(PER_CALL_TIMEOUT_SECONDS, remaining)
        if timeout <= 0:
            raise Fail("TOTAL_WALL_LIMIT_EXCEEDED")
        process = subprocess.Popen(
            command,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            start_new_session=True,
            preexec_fn=set_limits,
        )
        timed_out = False
        try:
            stdout, stderr = process.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
            try:
                os.killpg(process.pid, 9)
            except Exception:
                try:
                    process.kill()
                except Exception:
                    pass
            try:
                stdout, stderr = process.communicate(timeout=5)
            except Exception:
                stdout, stderr = b"", b""
                try:
                    process.wait(timeout=5)
                except Exception:
                    pass
        index = len(self.calls) + 1
        write_exclusive(PRIVATE_ROOT / "logs" / ("call-%d.stdout.bin" % index), stdout, 0o600)
        write_exclusive(PRIVATE_ROOT / "logs" / ("call-%d.stderr.bin" % index), stderr, 0o600)
        record = {
            "call": name,
            "exitCode": process.returncode,
            "timedOut": timed_out,
        }
        self.calls.append(record)
        self.save_ledger("NATIVE_CALL_FINISHED")
        if timed_out:
            raise Fail("NATIVE_TIMEOUT")
        if process.returncode != 0:
            raise Fail("NATIVE_NONZERO_EXIT")
        return record


def private_file_info(name, category):
    return require_file_nolink(PRIVATE_ROOT / name, category)


def read_private_public_file(name, category, max_bytes):
    info = private_file_info(name, category)
    if info.st_size <= 0 or info.st_size > max_bytes:
        raise Fail(category)
    path = PRIVATE_ROOT / name
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        chunks = []
        total = 0
        while True:
            chunk = os.read(fd, 65536)
            if not chunk:
                break
            total += len(chunk)
            if total > max_bytes:
                raise Fail(category)
            chunks.append(chunk)
    finally:
        os.close(fd)
    return b"".join(chunks)


def main(argv):
    os.umask(0o077)
    if len(argv) != 3 or argv[1] != "--auth":
        raise Fail("ARGUMENTS_INVALID")
    auth, binds, auth_hash = load_auth(argv[2])
    operator_path = Path(__file__)
    require_file_nolink(operator_path, "OPERATOR_FILE_UNSAFE")
    if sha256_file(operator_path) != auth["operatorSha256"]:
        raise Fail("OPERATOR_HASH_MISMATCH")
    bwrap_info = require_file_nolink(Path("/usr/bin/bwrap"), "BWRAP_UNAVAILABLE")
    if not (bwrap_info.st_mode & 0o111):
        raise Fail("BWRAP_UNAVAILABLE")
    create_fresh_roots()
    runner = None
    try:
        staged, measured_hash, linkage = extract_binary(auth)
        if auth["binaryLinkage"] == "dynamic":
            destinations = {destination for _, destination in binds}
            if linkage["interp"] not in destinations:
                raise Fail("ELF_INTERPRETER_BIND_MISSING")
        runner = Runner(auth, binds, staged, linkage)
        # Source structure: EraIndependent Address/Option.hs mounts "address";
        # Address/Command.hs and Address/Run.hs define key-gen, key-hash,
        # build, and info. Leaf option spellings below are exact proposed argv
        # and remain subject to the fresh review recorded in the proposal.
        runner.run_cli(
            "address-key-gen",
            [
                "address",
                "key-gen",
                "--verification-key-file",
                "/private/payment.vkey",
                "--signing-key-file",
                "/private/payment.skey",
            ],
        )
        skey_info = private_file_info("payment.skey", "SIGNING_KEY_MISSING")
        if skey_info.st_size <= 0:
            raise Fail("SIGNING_KEY_MISSING")
        # Never open the signing key. Only normalize/query its mode by path
        # after an lstat regular-file and non-link check.
        os.chmod(PRIVATE_ROOT / "payment.skey", 0o600)
        skey_info = private_file_info("payment.skey", "SIGNING_KEY_MISSING")
        signing_key_present = True
        signing_key_mode_0600 = stat.S_IMODE(skey_info.st_mode) == 0o600
        if not signing_key_mode_0600:
            raise Fail("SIGNING_KEY_MODE_INVALID")
        vkey_bytes = read_private_public_file("payment.vkey", "VERIFICATION_KEY_INVALID", 65536)
        try:
            vkey_json = json.loads(vkey_bytes.decode("utf-8"))
        except Exception:
            raise Fail("VERIFICATION_KEY_NOT_TEXT_ENVELOPE")
        if type(vkey_json) is not dict:
            raise Fail("VERIFICATION_KEY_NOT_TEXT_ENVELOPE")
        cbor_hex = vkey_json.get("cborHex")
        if type(cbor_hex) is not str or not re.fullmatch(r"[0-9a-fA-F]+", cbor_hex):
            raise Fail("VERIFICATION_KEY_NOT_TEXT_ENVELOPE")
        if len(cbor_hex) % 2 != 0:
            raise Fail("VERIFICATION_KEY_NOT_TEXT_ENVELOPE")
        runner.run_cli(
            "address-key-hash",
            [
                "address",
                "key-hash",
                "--payment-verification-key-file",
                "/private/payment.vkey",
                "--out-file",
                "/private/payment.vkey.hash",
            ],
        )
        key_hash_bytes = read_private_public_file(
            "payment.vkey.hash", "PAYMENT_KEY_HASH_INVALID", 4096
        )
        try:
            key_hash = key_hash_bytes.decode("ascii").strip().lower()
        except Exception:
            raise Fail("PAYMENT_KEY_HASH_INVALID")
        if not HEX56.fullmatch(key_hash):
            raise Fail("PAYMENT_KEY_HASH_INVALID")
        runner.run_cli(
            "address-build",
            [
                "address",
                "build",
                "--payment-verification-key-file",
                "/private/payment.vkey",
                "--testnet-magic",
                "1",
                "--out-file",
                "/private/payment.addr",
            ],
        )
        address_bytes = read_private_public_file("payment.addr", "ADDRESS_INVALID", 4096)
        try:
            address = address_bytes.decode("ascii").strip()
        except Exception:
            raise Fail("ADDRESS_INVALID")
        if address_bytes != (address + "\n").encode("ascii") and address_bytes != address.encode("ascii"):
            raise Fail("ADDRESS_NOT_SINGLE_LINE")
        if not ADDR_RE.fullmatch(address):
            raise Fail("ADDRESS_INVALID")
        runner.run_cli(
            "address-info",
            [
                "address",
                "info",
                "--address",
                address,
                "--out-file",
                "/private/address-info.json",
            ],
        )
        info_bytes = read_private_public_file("address-info.json", "ADDRESS_INFO_INVALID", 65536)
        try:
            address_info = json.loads(info_bytes.decode("utf-8"))
        except Exception:
            raise Fail("ADDRESS_INFO_INVALID")
        if type(address_info) is not dict:
            raise Fail("ADDRESS_INFO_INVALID")
        if address_info.get("type") != "payment":
            raise Fail("ADDRESS_INFO_TYPE_INVALID")
        if address_info.get("era") != "shelley":
            raise Fail("ADDRESS_INFO_ERA_INVALID")
        if address_info.get("encoding") != "bech32":
            raise Fail("ADDRESS_INFO_ENCODING_INVALID")
        if address_info.get("address") != address:
            raise Fail("ADDRESS_INFO_ADDRESS_MISMATCH")
        base16 = address_info.get("base16")
        if type(base16) is not str:
            raise Fail("ADDRESS_INFO_BASE16_INVALID")
        try:
            address_raw = bytes.fromhex(base16)
        except Exception:
            raise Fail("ADDRESS_INFO_BASE16_INVALID")
        # Proposed public invariant: a Shelley enterprise testnet address is
        # one header byte 0x60 followed by the 28-byte payment key hash.
        # This literal check is disclosed for fresh-reviewer agreement; it is
        # not represented as a ledger or ownership proof.
        if len(address_raw) != 29 or address_raw[0] != 0x60:
            raise Fail("ADDRESS_ENTERPRISE_HEADER_INVALID")
        if address_raw[1:].hex() != key_hash:
            raise Fail("ADDRESS_KEY_HASH_MISMATCH")
        if sha256_file(ARCHIVE) != ARCHIVE_SHA256:
            raise Fail("ARCHIVE_HASH_CHANGED")
        if sha256_file(staged) != measured_hash:
            raise Fail("STAGED_BINARY_HASH_CHANGED")
        if sha256_file(staged) != auth["expectedStagedBinarySha256"]:
            raise Fail("STAGED_BINARY_HASH_CHANGED")
        # Export public material only after every validation above succeeds.
        write_exclusive(PUBLIC_ROOT / "payment.vkey", vkey_bytes, 0o644)
        write_exclusive(PUBLIC_ROOT / "payment.vkey.hash", (key_hash + "\n").encode("ascii"), 0o644)
        write_exclusive(PUBLIC_ROOT / "payment.addr", (address + "\n").encode("ascii"), 0o644)
        write_exclusive(PUBLIC_ROOT / "address-info.json", info_bytes, 0o644)
        result = {
            "format": "cardano-preprod-fresh-wallet-result/1",
            "status": "WALLET_CREATED_PUBLIC_EXPORT_COMPLETE",
            "walletCreated": True,
            "networkName": NETWORK_NAME,
            "networkMagic": NETWORK_MAGIC,
            "mainnet": False,
            "address": address,
            "addressFileSha256": sha256_bytes((address + "\n").encode("ascii")),
            "paymentVerificationKeyFileSha256": sha256_bytes(vkey_bytes),
            "paymentVerificationKeyHash": key_hash,
            "paymentVerificationKeyHashFileSha256": sha256_bytes((key_hash + "\n").encode("ascii")),
            "addressInfoFileSha256": sha256_bytes(info_bytes),
            "addressType": "payment",
            "addressEra": "shelley",
            "addressEncoding": "bech32",
            "enterprise": True,
            "stakeCredentialPresent": False,
            "signingKeyRetainedPrivate": signing_key_present,
            "signingKeyMode0600": signing_key_mode_0600,
            "signingKeyExported": False,
            "signingKeyHashedByOperator": False,
            "privateRootMode0700": stat.S_IMODE(os.lstat(PRIVATE_ROOT).st_mode) == 0o700,
            "archivePath": str(ARCHIVE),
            "archiveBytes": ARCHIVE_BYTES,
            "archiveSha256": ARCHIVE_SHA256,
            "sourceCommit": SOURCE_COMMIT,
            "sourceArchiveSha256": SOURCE_ARCHIVE_SHA256,
            "cliVersion": CLI_VERSION,
            "stagedBinarySha256": measured_hash,
            "binaryLinkageDeclared": auth["binaryLinkage"],
            "authFileSha256": auth_hash,
            "operatorSha256": auth["operatorSha256"],
            "attemptsStarted": 1,
            "retries": 0,
            "cliCalls": runner.calls,
            "networkAccessUsed": False,
            "signingPerformed": False,
            "transactionBuiltOrSubmitted": False,
            "ledgerObserved": False,
            "ownershipSignatureTestPerformed": False,
        }
        result_hash = write_json_exclusive(PUBLIC_ROOT / "RESULT.json", result, 0o644)
        runner.save_ledger("COMPLETE")
        print(json.dumps({"status": "WALLET_CREATED", "resultSha256": result_hash}, sort_keys=True))
        return 0
    except Fail as failure:
        if runner is not None:
            try:
                runner.save_ledger("FAILED_ATTEMPT_SPENT")
            except Exception:
                pass
        if PUBLIC_ROOT.exists():
            result_path = PUBLIC_ROOT / "RESULT.json"
            if not result_path.exists():
                failure_result = {
                    "format": "cardano-preprod-fresh-wallet-result/1",
                    "status": "FAILED_ATTEMPT_SPENT",
                    "walletCreated": False,
                    "category": failure.category,
                    "attemptsStarted": 1 if runner is not None and runner.intent_recorded else 0,
                    "retries": 0,
                    "cliCalls": runner.calls if runner is not None else [],
                    "fundingAddress": None,
                    "addressAdvice": None,
                    "networkAccessUsed": False,
                    "signingPerformed": False,
                    "transactionBuiltOrSubmitted": False,
                }
                try:
                    write_json_exclusive(result_path, failure_result, 0o644)
                except Exception:
                    pass
        print(json.dumps({"status": "FAILED", "category": failure.category}, sort_keys=True))
        return 2


if __name__ == "__main__":
    try:
        sys.exit(main(sys.argv))
    except Fail as failure:
        print(json.dumps({"status": "FAILED", "category": failure.category}, sort_keys=True))
        sys.exit(2)
    except Exception:
        print(json.dumps({"status": "FAILED", "category": "INTERNAL_FIXED"}, sort_keys=True))
        sys.exit(2)

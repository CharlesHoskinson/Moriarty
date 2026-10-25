"""Holdout engine adapter: drives the REAL `mori simulate` Sherlock host route.

Evaluator-owned. Python standard library only. The binary is built from the working
tree (`cargo build -p mori --no-default-features --locked`) unless DW_MORI_BIN names
an existing executable. Nothing here reads driver-editable tests or expectations.
"""
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
HOLDOUT = HERE.parent
# The system under test is the checkout holding this holdout. DW_SUT_ROOT (evaluator use only)
# points the same sealed checks at another worktree, e.g. an implementer's uncommitted tree.
REPO = Path(os.environ.get("DW_SUT_ROOT") or HOLDOUT.parent.parent).resolve()


class Fail(Exception):
    """A holdout assertion failed against the system under test."""

PROFILE = "sherlock-helper-microstep/1"
SOURCE_PROFILES = {
    "draft": "moriarty-sherlock-draft/1",
    "nominal": "moriarty-sherlock-nominal-body-typed/1",
}
CONFIG_FILES = {
    "draft": HOLDOUT / "config" / "draft-host-config.json",
    "nominal": HOLDOUT / "config" / "nominal-host-config.json",
}

EXIT_PASS, EXIT_FAIL, EXIT_UNIMPL, EXIT_UNFIT = 0, 1, 2, 3


class HostUnfit(Exception):
    pass


class BuildFailed(Exception):
    pass


_BIN = None


def mori_bin():
    """Locate or build the non-native `mori` binary. Raises HostUnfit / BuildFailed."""
    global _BIN
    if _BIN:
        return _BIN
    override = os.environ.get("DW_MORI_BIN")
    if override:
        if not os.access(override, os.X_OK):
            raise HostUnfit(f"DW_MORI_BIN is not executable: {override}")
        _BIN = override
        return _BIN
    for tool in ("cargo",):
        if subprocess.run(["bash", "-c", f"command -v {tool}"], capture_output=True).returncode:
            raise HostUnfit(f"{tool} not found on PATH")
    free = os.statvfs(REPO).f_bavail * os.statvfs(REPO).f_frsize / 2**30
    if free < 5:
        raise HostUnfit(f"only {free:.1f} GiB free on the repository filesystem")
    env = dict(os.environ)
    env.setdefault("CARGO_BUILD_JOBS", "6")
    r = subprocess.run(
        ["cargo", "build", "-p", "mori", "--no-default-features", "--locked", "-q"],
        cwd=REPO, env=env, capture_output=True, text=True, timeout=590,
    )
    if r.returncode != 0:
        raise BuildFailed("cargo build -p mori --no-default-features --locked failed:\n" + r.stderr[-2000:])
    meta = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps", "--locked"],
        cwd=REPO, env=env, capture_output=True, text=True,
    )
    target = json.loads(meta.stdout)["target_directory"] if meta.returncode == 0 else str(REPO / "target")
    _BIN = str(Path(target) / "debug" / "mori")
    if not os.access(_BIN, os.X_OK):
        raise BuildFailed(f"built binary not found at {_BIN}")
    return _BIN


class Run:
    """One `mori simulate` invocation, parsed."""

    def __init__(self, returncode, stdout, stderr):
        self.returncode = returncode
        self.stderr = stderr
        try:
            self.doc = json.loads(stdout)
        except Exception:
            self.doc = None
        self.raw = stdout

    @property
    def executed(self):
        return bool(self.doc) and "execution" in self.doc and self.doc["execution"].get("completed")

    @property
    def execution(self):
        return self.doc["execution"]

    @property
    def summary(self):
        return self.execution["summary"]

    @property
    def outcome(self):
        return self.summary["outcome"]

    @property
    def failure_stage(self):
        if self.doc and "stage" in self.doc and "execution" not in self.doc:
            return self.doc["stage"]
        return None

    @property
    def program_view(self):
        d = self.doc
        if d is None:
            return None
        prep = d.get("preparation", {})
        return prep.get("admission", {}).get("program_view")


def run_source(kind, source, fuel=None, stack=None, workdir=None, extra=None, bounds=None):
    """Run `mori simulate`. `extra` argv is appended to the call (e.g. ["--chunk", "1"]);
    `bounds` overrides config bounds entries (string values), applied after the fuel adjustments."""
    cfg = json.loads(CONFIG_FILES[kind].read_text())
    if fuel is not None:
        cfg["fuel"]["value"] = str(fuel)
        for k in ("max_reference_work", "max_helper_work", "step_cap"):
            if int(cfg["bounds"][k]) < fuel:
                cfg["bounds"][k] = str(fuel)
        vs = int(cfg["layout"]["capacities"]["value_slots"])
        if vs < fuel + 64:
            cfg["layout"]["capacities"]["value_slots"] = str(fuel + 64)
    if stack is not None:
        cfg["stack"]["value"] = str(stack)
    for k, v in (bounds or {}).items():
        assert k in cfg["bounds"], f"unknown config bound {k}"
        cfg["bounds"][k] = str(v)
    with tempfile.TemporaryDirectory(prefix="dw-holdout-") as d:
        p = Path(d)
        (p / "program.mori").write_text(source)
        (p / "config.json").write_text(json.dumps(cfg))
        (p / "target.json").write_text("{}")
        r = subprocess.run(
            [mori_bin(), "simulate", str(p / "program.mori"),
             "--execution-profile", PROFILE,
             "--source-profile", SOURCE_PROFILES[kind],
             "--entry",
             "--execution-config", str(p / "config.json"),
             "--target", str(p / "target.json"), *[str(x) for x in (extra or [])]],
            capture_output=True, text=True, timeout=120,
        )
    return Run(r.returncode, r.stdout, r.stderr)


# ---- value decoding -------------------------------------------------------------

def _slice(x):
    return int(x["start"]), int(x["len"])


def decode_value(run, ref=None):
    """Decode the halted result (or ref) to plain Python: int, bool, list, ('leaf',n), ('branch',l,r)."""
    pv = run.program_view
    img = run.execution["image"]
    consts = pv["constants"]
    cfields = pv["constant_fields"]
    data = img["data"]
    dfields = img["data_fields"]
    if ref is None:
        ref = run.outcome["Halted"]["value"]

    def scalar(node):
        if "U128" in node:
            return int(node["U128"]["n"])
        if "Bool" in node:
            return bool(node["Bool"]["b"])
        return None

    def ctor(name, fields, fetch):
        s, n = _slice(fields)
        kids = [dec(fetch[s + i]) for i in range(n)]
        return name, kids

    def dec(r):
        (kind, idx), = r.items()
        idx = int(idx)
        if kind == "Const":
            node = consts[idx]
            src_fields = cfields
        else:
            node = data[idx]
            src_fields = dfields
        v = scalar(node)
        if v is not None:
            return v
        (name, body), = node.items()
        fields = body.get("fields", {"start": "0", "len": "0"})
        s, n = _slice(fields)
        kids = [dec(src_fields[s + i]) for i in range(n)]
        if name == "Nil":
            return []
        if name == "Cons":
            head, tail = kids
            return [head] + tail
        if name == "Leaf":
            return ("leaf", kids[0])
        if name == "Branch":
            return ("branch", kids[0], kids[1])
        return (name, *kids)

    return dec(ref)


def outcome_kind(run):
    (k, v), = run.outcome.items()
    return k, v


# ---- raw invocation (profile /2 suspension and finance routes) ------------------------

PROFILE2 = "sherlock-helper-microstep/2"
SOURCE_PROFILE2 = {"draft": "moriarty-sherlock-draft/2", "fin": "moriarty/2"}


def config2_file(workdir, kind="draft"):
    """The draft host config with the profile switched to /2 (nothing else is assumed)."""
    cfg = json.loads(CONFIG_FILES[kind].read_text())
    cfg["profile"] = PROFILE2
    path = Path(workdir) / "config2.json"
    path.write_text(json.dumps(cfg))
    return path


def mori_raw(args, cwd, timeout=120):
    """Run `mori <args>` in `cwd` (files persist there between calls). Returns a Run."""
    r = subprocess.run([mori_bin(), *[str(a) for a in args]], cwd=str(cwd),
                       capture_output=True, text=True, timeout=timeout)
    return Run(r.returncode, r.stdout, r.stderr)

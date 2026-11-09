#!/usr/bin/env python3
"""Skeleton-coding pilot driver (research/skeleton-coding-20261108/pilot-protocol.md).

Subcommands:
  spec COMPONENT                 print the condition-A spec (task, reference machine
                                 pointers, prose) for a component
  sandbox COMPONENT OUT [--stub F] [--with-reference]
                                 export HEAD (crates + workspace files, no .git) to OUT,
                                 hide the component's reference circuit and the test
                                 files that name its rules, install the stub
  lane VENDOR CWD PROMPT LOGDIR  run one model lane (claude | codex) and record
                                 wall, exit, tokens and the event log
  audit LOGDIR SANDBOX COMPONENT blindness audit of a lane's event log
  grade COMPONENT CANDIDATE OUT [--target candidate|reference]
                                 install CANDIDATE in a grader export and run the
                                 Rust grader through `scripts/doctorwatson heavy`
  skeleton COMPONENT KIND RUNDIR one skeleton-author lane (blind -> B, derived -> C),
                                 always Claude; writes RUNDIR/skeletons/COMPONENT-B|C.rs
  cell COMPONENT CONDITION VENDOR SEED RUNDIR
                                 one build: sandbox, lane, extract, audit
  grade-cell CELLDIR             grade a built cell (one grade at a time)

Model ids and flags are fixed here and in the protocol; change both together.
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
PILOT = REPO / "research" / "skeleton-coding-20261108" / "pilot"
SM = Path("crates/mori-backend/src/sherlock_machine")
TARGET_ROOT = Path(os.environ.get("SKELETON_PILOT_TARGET_ROOT",
                                  "/home/hoskinson/Projects/Moriarty/target"))
EXPORT_PATHS = ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "crates"]

COMPONENTS = {
    "c6_admin_publish": ("C6AdminPublishEmitter", ["KAdminPublishValue"]),
    "c6_call_bind": ("C6CallBindEmitter",
                     ["KMachineInvokeFind", "KMachineEnterBind", "KMachineBoundValid"]),
    "c6_scalar_construct": ("C6ScalarConstructEmitter", ["BHelperScalarConstruct"]),
    "c6_append_helper": ("C6AppendHelperEmitter",
                         ["BHelperAppendEntry", "BHelperAppendEmpty", "BHelperAppendTail",
                          "BHelperAppendExit"]),
    "c6_call_helper": ("C6CallHelperEmitter",
                       ["BHelperFindEntry", "BHelperFindRow", "BHelperFindExit",
                        "BHelperEnvGetEntry", "BHelperEnvGetCell", "BHelperEnvGetExit"]),
    "c6_eval_schedule": ("C6EvalScheduleEmitter",
                         ["KMachineEvalLiteralValid", "KMachineEvalVariablePresent",
                          "KMachineEvalBinary", "KMachineEvalUnary", "KMachineEvalIf",
                          "KMachineEvalMatch", "KMachineEvalCallArguments"]),
}

CLAUDE_MODEL = "claude-opus-5-5"
CODEX_MODEL = "gpt-6.1-sol"
CODEX_EFFORT = "high"
LANE_TIMEOUT = int(os.environ.get("SKELETON_PILOT_LANE_TIMEOUT", "3600"))


def dotted(rule: str) -> str:
    """KAdminPublishValue -> k.admin.publish.value (lower-case row name)."""
    return ".".join(w.lower() for w in re.findall(r"[A-Z][a-z]*", rule))


def rule_patterns(component: str) -> list[str]:
    rules = COMPONENTS[component][1]
    return rules + [dotted(r) for r in rules]


def mentions(text: str, pats: list[str]) -> bool:
    low = text.lower()
    return any((p in text) if p[0].isupper() else (p in low) for p in pats)


# ---------------------------------------------------------------- export


def export_head(out: Path) -> str:
    """Extract HEAD's workspace files into `out` (no .git). Returns the commit."""
    head = subprocess.run(["git", "-C", str(REPO), "rev-parse", "HEAD"], check=True,
                          capture_output=True, text=True).stdout.strip()
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    def extract(paths: list[str]) -> None:
        blob = subprocess.run(["git", "-C", str(REPO), "archive", "--format=tar", head, "--",
                               *paths], check=True, capture_output=True).stdout
        with tarfile.open(fileobj=io.BytesIO(blob)) as t:
            t.extractall(out, filter="data")

    extract(EXPORT_PATHS)
    extra = sorted(included_outside(out))
    if extra:
        extract(extra)
    return head


INCLUDE_RE = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]+)"')


def included_outside(root: Path) -> set[str]:
    """Files outside the exported paths that crate sources `include_str!`/`include_bytes!`,
    as repo-relative paths that exist at HEAD (only those files, not their folders)."""
    out = set()
    for f in (root / "crates").rglob("*.rs"):
        for rel in INCLUDE_RE.findall(f.read_text(errors="replace")):
            p = os.path.normpath(f.parent / rel)
            try:
                r = Path(p).relative_to(root)
            except ValueError:
                continue
            if not (root / r).exists() and (REPO / r).is_file():
                out.add(str(r))
    return out


MOD_RE = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;\s*$")
ATTR_RE = re.compile(r"^\s*#\[.*\]\s*$")


def strip_mods(src: str, removed: set[str]) -> str:
    """Delete `mod x;` declarations (and their attribute lines) that point at a
    removed file, by module name or by `#[path = "file"]`."""
    lines = src.split("\n")
    out: list[str] = []
    for line in lines:
        m = MOD_RE.match(line)
        if m:
            attrs = []
            while out and ATTR_RE.match(out[-1]):
                attrs.insert(0, out.pop())
            paths = {p for a in attrs for p in re.findall(r'path\s*=\s*"([^"]+)"', a)}
            hit = (m.group(1) + ".rs") in removed or any(Path(p).name in removed for p in paths)
            if hit:
                continue
            out.extend(attrs)
        out.append(line)
    return "\n".join(out)


def hide_files(root: Path, component: str) -> list[str]:
    """Hide the reference circuit, its tests, every mori-backend test file that names
    a component rule, and the pilot grader. Returns the hidden paths."""
    pats = rule_patterns(component)
    sm = root / SM
    hidden: list[Path] = [sm / f"{component}_step.rs", sm / f"{component}_tests.rs",
                          sm / "skeleton_pilot.rs", sm / "skeleton_pilot_cand.rs"]
    backend = root / "crates/mori-backend"
    for f in sorted(backend.rglob("*.rs")):
        if f in hidden:
            continue
        is_test = f.name.endswith("_tests.rs") or "/tests/" in str(f.relative_to(backend))
        if is_test and mentions(f.read_text(errors="replace"), pats):
            hidden.append(f)
    names = {h.name for h in hidden if h.name != f"{component}_step.rs"}
    for h in hidden:
        if h.exists() and h.name != f"{component}_step.rs":
            h.unlink()
    for f in backend.rglob("*.rs"):
        s = f.read_text(errors="replace")
        t = strip_mods(s, names)
        if t != s:
            f.write_text(t)
    return [str(h.relative_to(root)) for h in hidden]


def cmd_sandbox(component: str, out: Path, stub: Path | None, with_reference: bool) -> dict:
    head = export_head(out)
    ref_src = (out / SM / f"{component}_step.rs").read_text()
    hidden = hide_files(out, component)
    target = out / SM / f"{component}_step.rs"
    target.write_text(stub.read_text() if stub else
                      "//! Pilot target: implement this file (see PILOT_TASK.md).\n")
    if with_reference:
        (out / "reference").mkdir(exist_ok=True)
        (out / "reference" / f"{component}_step.rs").write_text(ref_src)
    meta = {"component": component, "head": head, "hidden": hidden,
            "target": str(SM / f"{component}_step.rs"), "with_reference": with_reference,
            "stub_sha256": hashlib.sha256(target.read_bytes()).hexdigest()}
    (out / ".pilot-sandbox.json").write_text(json.dumps(meta, indent=2) + "\n")
    return meta


# ---------------------------------------------------------------- spec


def reference_doc(component: str) -> str:
    """The module doc header (//! lines) of the reference step file: the rule's
    prose description. Identical in every condition."""
    src = (REPO / SM / f"{component}_step.rs").read_text()
    doc = []
    for line in src.split("\n"):
        if line.startswith("//!"):
            doc.append(line[3:].strip())
        elif doc:
            break
    return "\n".join(doc)


def machine_pointers(component: str) -> list[str]:
    """Every non-test line of mori-core that names a component rule."""
    pats = rule_patterns(component)
    core = REPO / "crates/mori-core/src"
    hits = []
    for f in sorted(core.rglob("*.rs")):
        if f.name.endswith("_tests.rs") or f.name == "c6_stream_event.rs":
            continue
        for i, line in enumerate(f.read_text(errors="replace").split("\n"), 1):
            if mentions(line, pats):
                hits.append(f"{f.relative_to(REPO)}:{i}: {line.strip()[:160]}")
    return hits


def cmd_spec(component: str) -> str:
    emitter, rules = COMPONENTS[component]
    stem = component.removeprefix("c6_")
    t = (PILOT / "prompts" / "spec-template.md").read_text()
    return (t.replace("{component}", component)
             .replace("{target}", str(SM / f"{component}_step.rs"))
             .replace("{consumer}", str(SM / f"{component}_consumer.rs"))
             .replace("{emitter}", emitter)
             .replace("{rules_const}", stem.upper() + "_RULES")
             .replace("{supported_fn}", stem + "_supported")
             .replace("{rules}", ", ".join(f"`RuleIdC::{r}`" for r in rules))
             .replace("{pointers}", "\n".join(f"- `{p}`" for p in machine_pointers(component)))
             .replace("{prose}", reference_doc(component)))


# ---------------------------------------------------------------- lanes


def lane_target_dir(cwd: Path) -> Path:
    tag = hashlib.sha256(str(cwd).encode()).hexdigest()[:8]
    return TARGET_ROOT / f"skeleton-pilot-lane-{tag}"


def lane_cmd(vendor: str, cwd: Path, prompt: str, logdir: Path) -> list[str]:
    if vendor == "claude":
        return ["claude", "-p", "--restricted", "--safe-mode", "--strict-mcp-config",
                "--tools", "Bash,Read,Edit,Write,Glob,Grep",
                "--allowedTools",
                "Read,Edit,Write,Glob,Grep,Bash(cargo check:*),Bash(cargo clippy:*),"
                "Bash(cargo fmt:*)",
                "--permission-mode", "dontAsk", "--model", CLAUDE_MODEL,
                "--output-format", "stream-json", "--verbose", "--no-session-persistence",
                prompt]
    if vendor == "codex":
        tdir = lane_target_dir(cwd)
        cargo_home = Path.home() / ".cargo"
        return ["codex", "exec", "-m", CODEX_MODEL, "-c", f"model_reasoning_effort={CODEX_EFFORT}",
                "-s", "workspace-write", "--ephemeral", "--skip-git-repo-check",
                "-c", f'sandbox_workspace_write.writable_roots=["{tdir}","{cargo_home}"]',
                "--json", "-C", str(cwd), "-o", str(logdir / "final.md"), prompt]
    raise SystemExit(f"unknown vendor {vendor}")


def usage_of(vendor: str, events: str) -> dict:
    u: dict = {}
    for line in events.splitlines():
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        if vendor == "claude" and e.get("type") == "result":
            u = {"usage": e.get("usage"), "total_cost_usd": e.get("total_cost_usd"),
                 "num_turns": e.get("num_turns"), "is_error": e.get("is_error"),
                 "result": (e.get("result") or "")[:2000]}
        if vendor == "codex" and e.get("type") == "turn.completed":
            usage = e.get("usage") or {}
            for k, v in usage.items():
                if isinstance(v, int):
                    u[k] = u.get(k, 0) + v
    return u


def cmd_lane(vendor: str, cwd: Path, prompt_file: Path, logdir: Path) -> dict:
    logdir.mkdir(parents=True, exist_ok=True)
    prompt = prompt_file.read_text()
    argv = lane_cmd(vendor, cwd, prompt, logdir)
    env = dict(os.environ)
    for k in ("OPENAI_API_KEY", "CODEX_API_KEY", "XAI_API_KEY", "ANTHROPIC_API_KEY"):
        env.pop(k, None)
    env["CARGO_TARGET_DIR"] = str(lane_target_dir(cwd))
    env["CARGO_NET_OFFLINE"] = "true"
    t0 = time.time()
    try:
        p = subprocess.run(argv, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                           capture_output=True, text=True, timeout=LANE_TIMEOUT)
        code, out, err = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired as e:
        code = "timeout"
        out = e.stdout.decode() if isinstance(e.stdout, bytes) else (e.stdout or "")
        err = e.stderr.decode() if isinstance(e.stderr, bytes) else (e.stderr or "")
    wall = time.time() - t0
    (logdir / "events.jsonl").write_text(out)
    (logdir / "stderr.log").write_text(err)
    shown = [a if a != prompt else f"<prompt:{prompt_file}>" for a in argv]
    meta = {"vendor": vendor, "model": CLAUDE_MODEL if vendor == "claude" else CODEX_MODEL,
            "argv": shown, "cwd": str(cwd), "exit": code, "wall_seconds": round(wall, 1),
            "prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
            "usage": usage_of(vendor, out)}
    (logdir / "lane.json").write_text(json.dumps(meta, indent=2) + "\n")
    return meta


# ---------------------------------------------------------------- audit

FORBIDDEN_CMD = re.compile(r"\bgit\b|\bcurl\b|\bwget\b|cargo\s+test|\bgh\b")


def audit_events(text: str, sandbox: Path, component: str,
                 meta: Path | None = None) -> list[dict]:
    """Blindness audit. A violation is a tool call that names a path outside the
    sandbox (other than the cargo home and lane target dirs), names the
    reference repository, runs git/network/cargo test, or names a hidden file.
    `meta` is the sandbox record saved with the cell (sandboxes are reused, so the
    live `.pilot-sandbox.json` may belong to a later build)."""
    allowed = (str(sandbox), str(Path.home() / ".cargo"), str(TARGET_ROOT / "skeleton-pilot-lane"),
               "/dev/null",
               # The vendor CLIs' own global instructions and skills (same in every
               # condition; no repository content).
               str(Path.home() / ".agents/skills"), str(Path.home() / ".codex"))
    hidden = [f"{component}_tests.rs", "skeleton_pilot.rs"]
    meta = meta or sandbox / ".pilot-sandbox.json"
    with_ref = (sandbox / "reference").exists()
    if meta.exists():
        with_ref = bool(json.loads(meta.read_text()).get("with_reference"))
        hidden += [Path(h).name for h in json.loads(meta.read_text())["hidden"]
                   if not h.endswith(f"{component}_step.rs")]
    out = []
    for n, line in enumerate(text.splitlines()):
        try:
            e = json.loads(line)
        except json.JSONDecodeError:
            continue
        calls = []
        if e.get("type") == "assistant":  # claude stream-json
            for c in (e.get("message") or {}).get("content") or []:
                if c.get("type") == "tool_use":
                    calls.append(json.dumps(c.get("input")))
        item = e.get("item") or {}
        if item.get("type") == "command_execution":  # codex --json
            calls.append(item.get("command") or "")
        if item.get("type") == "file_change":
            calls.append(json.dumps(item.get("changes")))
        for c in calls:
            why = []
            for p in re.findall(r"(/[\w.\-/]+)", c):
                own_result = p.startswith(str(Path.home() / ".claude/projects/")) and "/tool-results/" in p
                if (p.startswith(("/home/", "/tmp/claude-")) and not p.startswith(allowed)
                        and not own_result):
                    why.append(f"outside path {p}")
            if FORBIDDEN_CMD.search(c):
                why.append("forbidden command")
            for h in hidden:
                if h in c:
                    why.append(f"hidden file {h}")
            if "reference/" in c and not with_ref:
                why.append("reference dir")
            if why:
                out.append({"event": n, "call": c[:300], "why": why})
    return out


# ---------------------------------------------------------------- grade


def adaptor(component: str) -> str:
    emitter = COMPONENTS[component][0]
    return f"""//! Pilot grader adaptor (throwaway grader export only).
use midnight_zkir_v3::Instruction;
use mori_core::sherlock::helper_machine::shared::RuleIdC;

use super::c6_step_kit::C6StepInputs;

pub(super) fn emit(
    rule: RuleIdC,
    inputs: &C6StepInputs,
    scope: &str,
) -> Result<Vec<Instruction>, String> {{
    let mut e = super::skeleton_pilot_cand_step::{emitter}::new(scope).map_err(|e| e.to_string())?;
    Ok(e.emit(rule, inputs).map_err(|e| e.to_string())?.instructions)
}}
"""


def install_candidate(root: Path, component: str, candidate: Path) -> None:
    sm = root / SM
    src = candidate.read_text()
    # A candidate's own test-module declarations point at files that do not exist here.
    src = re.sub(r"(?m)^\s*#\[cfg\(test\)\]\s*\n(\s*#\[path[^\n]*\n)?\s*mod\s+\w+\s*;\s*$", "", src)
    (sm / "skeleton_pilot_cand_step.rs").write_text(src)
    (sm / "skeleton_pilot_cand.rs").write_text(adaptor(component))
    mod = sm / "mod.rs"
    m = mod.read_text()
    decl = "#[cfg(test)]\nmod skeleton_pilot_cand_step;\n"
    if decl not in m:
        m = m.replace("#[cfg(test)]\nmod skeleton_pilot_cand;\n",
                      "#[cfg(test)]\nmod skeleton_pilot_cand;\n" + decl)
        mod.write_text(m)


def cmd_grade(component: str, candidate: Path | None, out: Path, target: str,
              grader: Path, max_wall: int, extra_env: dict[str, str]) -> int:
    export_head(grader)
    if candidate is not None:
        install_candidate(grader, component, candidate)
    out.mkdir(parents=True, exist_ok=True)
    env = {"RUST_TEST_THREADS": "1", "CARGO_TARGET_DIR": str(TARGET_ROOT / "skeleton-pilot-grader"),
           "SKELETON_PILOT_COMPONENT": component, "SKELETON_PILOT_OUT": str(out),
           "SKELETON_PILOT_TARGET": target, **extra_env}
    cell = out.parent.name if out.name == "grade" else out.name
    argv = [str(REPO / "scripts/doctorwatson"), "heavy", "--id", f"SKP-{cell}",
            "--max-wall", str(max_wall), "--hypothesis",
            f"skeleton pilot grade {component} {target}: IR raw-statement mutants vs reference",
            "--", "env", *[f"{k}={v}" for k, v in env.items()],
            "cargo", "test", "--manifest-path", str(grader / "Cargo.toml"), "-p", "mori-backend",
            "--lib", "--locked", "sherlock_machine::skeleton_pilot::skeleton_pilot_grade",
            "--", "--ignored", "--exact", "--nocapture"]
    p = subprocess.run(argv, cwd=REPO, capture_output=True, text=True)
    (out / "heavy.log").write_text(p.stdout + "\n--- stderr ---\n" + p.stderr)
    return p.returncode


def cmd_grade_batch(cells: list[Path], scratch: Path, max_wall: int, per_cell: int,
                    self_test: bool = True) -> int:
    """Grade several built cells in ONE heavy run (one queue slot). One grader export;
    between cells the script copies in that cell's candidate and adaptor. The batch
    export (`grader-batch`) is separate from the single-cell export (`grader`), so a
    queued single grade is not overwritten."""
    grader = scratch / "grader-batch"
    export_head(grader)
    sm = grader / SM
    lines = ["#!/usr/bin/env bash", "set -u"]
    for cell in cells:
        rec = json.loads((cell / "cell.json").read_text())
        out = (cell / "grade").resolve()
        inst = out / "install"
        inst.mkdir(parents=True, exist_ok=True)
        install_candidate(grader, rec["component"], cell / "candidate.rs")
        shutil.copy(sm / "skeleton_pilot_cand_step.rs", inst / "skeleton_pilot_cand_step.rs")
        shutil.copy(sm / "skeleton_pilot_cand.rs", inst / "skeleton_pilot_cand.rs")
        env = {"RUST_TEST_THREADS": "1",
               "CARGO_TARGET_DIR": str(TARGET_ROOT / "skeleton-pilot-grader"),
               "SKELETON_PILOT_COMPONENT": rec["component"], "SKELETON_PILOT_OUT": str(out),
               "SKELETON_PILOT_TARGET": "candidate", "SKELETON_PILOT_SEED": str(rec["seed"]),
               "SKELETON_PILOT_MAX_MUTANTS": MAX_MUTANTS}
        lines += [f"cp {inst}/*.rs {sm}/"]
        if self_test and cell is cells[0]:
            # Grader self-check after the first install (reference clean, weakened emitter caught), so a
            # grader defect is visible before any candidate verdict is read.
            log = (cells[0].parent / "grader-selftest.log").resolve()
            lines += ["echo '== grader self-test'", "start=$(date +%s)",
                      f"timeout {per_cell} env RUST_TEST_THREADS=1"
                      f" CARGO_TARGET_DIR={TARGET_ROOT / 'skeleton-pilot-grader'}"
                      f" cargo test --manifest-path {grader}/Cargo.toml -p mori-backend --lib --locked"
                      " sherlock_machine::skeleton_pilot:: -- --nocapture"
                      f" > {log} 2>&1",
                      "echo \"self-test exit $? wall $(( $(date +%s) - start ))s\""]
        lines += [
                  f"echo '== {cell.name}'",
                  "start=$(date +%s)",
                  f"timeout {per_cell} env " + " ".join(f"{k}={v}" for k, v in env.items())
                  + f" cargo test --manifest-path {grader}/Cargo.toml -p mori-backend --lib --locked"
                  " sherlock_machine::skeleton_pilot::skeleton_pilot_grade -- --ignored --exact"
                  f" --nocapture > {out}/heavy.log 2>&1",
                  f"echo \"{cell.name} exit $? wall $(( $(date +%s) - start ))s\""]
    script = scratch / "grade-batch.sh"
    script.write_text("\n".join(lines) + "\n")
    argv = [str(REPO / "scripts/doctorwatson"), "heavy", "--id", f"SKP-batch-{len(cells)}",
            "--max-wall", str(max_wall), "--hypothesis",
            f"skeleton pilot batch grade of {len(cells)} cells: IR raw-statement mutants vs reference",
            "--", "bash", str(script)]
    p = subprocess.run(argv, cwd=REPO, capture_output=True, text=True)
    (scratch / "grade-batch.log").write_text(p.stdout + "\n--- stderr ---\n" + p.stderr)
    shutil.copy(scratch / "grade-batch.log", cells[0].parent / "grade-batch.log")
    for cell in cells:
        rec = json.loads((cell / "cell.json").read_text())
        g = cell / "grade" / "grade.json"
        rec["grade_exit"] = 0 if g.exists() else "no receipt"
        if g.exists():
            rec["verdict"] = json.loads(g.read_text())["verdict"]
        (cell / "cell.json").write_text(json.dumps(rec, indent=2) + "\n")
    print(p.stdout[-3000:])
    return p.returncode


# ---------------------------------------------------------------- cell


def cmd_cell(component: str, condition: str, vendor: str, seed: int, rundir: Path,
             scratch: Path) -> dict:
    """Build one cell (sandbox, lane, extract, audit). Grade it with grade-cell."""
    cell = rundir / f"{component}-{condition}-{vendor}-s{seed}"
    cell.mkdir(parents=True, exist_ok=True)
    stub = None
    if condition in ("B", "C"):
        stub = rundir / "skeletons" / f"{component}-{condition}.rs"
        if not stub.exists():
            raise SystemExit(f"missing skeleton {stub}")
    sandbox = scratch / f"lane-{vendor}"  # stable path per vendor: warm cargo cache
    meta = cmd_sandbox(component, sandbox, stub, False)
    task = build_prompt(component, condition, stub)
    (sandbox / "PILOT_TASK.md").write_text(task)
    pf = cell / "prompt.md"
    pf.write_text(task)
    lane = cmd_lane(vendor, sandbox, pf, cell / "lane")
    built = sandbox / meta["target"]
    shutil.copy(built, cell / "candidate.rs")
    (cell / "sandbox.json").write_text(json.dumps(meta, indent=2) + "\n")
    violations = audit_events((cell / "lane" / "events.jsonl").read_text(), sandbox, component,
                              cell / "sandbox.json")
    (cell / "audit.json").write_text(json.dumps(violations, indent=2) + "\n")
    rec = {"component": component, "condition": condition, "vendor": vendor, "seed": seed,
           "sandbox_head": meta["head"], "lane_exit": lane["exit"],
           "lane_wall_seconds": lane["wall_seconds"], "usage": lane["usage"],
           "blind_violations": len(violations),
           "candidate_sha256": hashlib.sha256((cell / "candidate.rs").read_bytes()).hexdigest()}
    (cell / "cell.json").write_text(json.dumps(rec, indent=2) + "\n")
    return rec


MAX_MUTANTS = "4000"


def cmd_grade_cell(cell: Path, scratch: Path, max_wall: int) -> dict:
    """Grade a built cell. Run grades one at a time: they share one grader export."""
    rec = json.loads((cell / "cell.json").read_text())
    rc = cmd_grade(rec["component"], cell / "candidate.rs", cell / "grade", "candidate",
                   scratch / "grader", max_wall,
                   {"SKELETON_PILOT_SEED": str(rec["seed"]),
                    "SKELETON_PILOT_MAX_MUTANTS": MAX_MUTANTS})
    rec["grade_exit"] = rc
    g = cell / "grade" / "grade.json"
    if g.exists():
        rec["verdict"] = json.loads(g.read_text())["verdict"]
    (cell / "cell.json").write_text(json.dumps(rec, indent=2) + "\n")
    return rec


def cmd_skeleton(component: str, kind: str, rundir: Path, scratch: Path) -> dict:
    cond = {"blind": "B", "derived": "C"}[kind]
    cell = rundir / "skeletons" / f"{component}-{cond}"
    cell.mkdir(parents=True, exist_ok=True)
    sandbox = scratch / f"lane-skeleton-{cond}"
    meta = cmd_sandbox(component, sandbox, None, kind == "derived")
    task = skeleton_prompt(component, kind)
    (sandbox / "PILOT_TASK.md").write_text(task)
    pf = cell / "prompt.md"
    pf.write_text(task)
    lane = cmd_lane("claude", sandbox, pf, cell / "lane")
    out = rundir / "skeletons" / f"{component}-{cond}.rs"
    shutil.copy(sandbox / meta["target"], out)
    (cell / "sandbox.json").write_text(json.dumps(meta, indent=2) + "\n")
    violations = audit_events((cell / "lane" / "events.jsonl").read_text(), sandbox, component,
                              cell / "sandbox.json")
    (cell / "audit.json").write_text(json.dumps(violations, indent=2) + "\n")
    text = out.read_text()
    rec = {"component": component, "condition": cond, "kind": kind, "lane_exit": lane["exit"],
           "lane_wall_seconds": lane["wall_seconds"], "usage": lane["usage"],
           "blind_violations": len(violations), "lines": text.count("\n"),
           "todo_bodies": text.count("todo!("),
           "sha256": hashlib.sha256(out.read_bytes()).hexdigest()}
    (cell / "skeleton.json").write_text(json.dumps(rec, indent=2) + "\n")
    return rec


def build_prompt(component: str, condition: str, stub: Path | None) -> str:
    t = (PILOT / "prompts" / "builder.md").read_text()
    sk = ""
    if condition in ("B", "C"):
        sk = (PILOT / "prompts" / "builder-skeleton-section.md").read_text()
    return (t.replace("{spec}", cmd_spec(component))
             .replace("{skeleton_section}", sk)
             .replace("{target}", str(SM / f"{component}_step.rs")))


def skeleton_prompt(component: str, kind: str) -> str:
    t = (PILOT / "prompts" / f"skeleton-{kind}.md").read_text()
    return (t.replace("{spec}", cmd_spec(component))
             .replace("{target}", str(SM / f"{component}_step.rs"))
             .replace("{component}", component))


def tokens(usage: dict) -> str:
    """One token summary for either vendor's usage record."""
    u = usage.get("usage") if isinstance(usage.get("usage"), dict) else usage
    if not u:
        return "-"
    inp = (u.get("input_tokens") or 0) + (u.get("cache_read_input_tokens") or 0) + \
        (u.get("cache_creation_input_tokens") or 0)
    out = u.get("output_tokens") or 0
    cost = usage.get("total_cost_usd")
    s = f"{inp / 1000:.0f}k in / {out / 1000:.1f}k out"
    return s + (f" (${cost:.2f})" if cost else "")


def cmd_report(rundir: Path) -> str:
    """Markdown tables of the skeleton lanes and the builds in a run directory."""
    rows = ["| Skeleton | Lane exit | Wall s | Tokens | Lines | Blind violations |",
            "|---|---|---|---|---|---|"]
    for f in sorted((rundir / "skeletons").glob("*/skeleton.json")):
        d = json.loads(f.read_text())
        v = json.loads((f.parent / "audit.json").read_text())
        rows.append(f"| {f.parent.name} | {d['lane_exit']} | {d['lane_wall_seconds']} | "
                    f"{tokens(d['usage'])} | {d['lines']} | {len(v)} |")
    rows += ["", "| Build | Lane exit | Wall s | Tokens | Blind | Built | Complete | Static | "
             "Soundness mutants | Mutants run / space | Instr. cand/ref | Primary |",
             "|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for f in sorted(rundir.glob("*/cell.json")):
        d = json.loads(f.read_text())
        v = json.loads((f.parent / "audit.json").read_text())
        g = f.parent / "grade" / "grade.json"
        if g.exists():
            s = json.loads(g.read_text())
            ins = s["build"]["instructions"]
            ratio = ", ".join(f"{x['candidate']}/{x['reference']}" for x in ins.values())
            vd = s["verdict"]
            st = len(s["static"]["lanes_read_by_reference_not_candidate"])
            cols = [vd["built"], vd["complete"], st, vd["soundness_mutants"],
                    f"{s['mutants']['run']} / {s['mutants']['space']}", ratio,
                    "PASS" if vd["primary_pass"] else "fail"]
        else:
            cols = ["not graded"] + [""] * 6
        rows.append(f"| {f.parent.name} | {d['lane_exit']} | {d['lane_wall_seconds']} | "
                    f"{tokens(d['usage'])} | {len(v)} | " + " | ".join(str(c) for c in cols) + " |")
    return "\n".join(rows)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    sp = ap.add_subparsers(dest="cmd", required=True)
    a = sp.add_parser("spec"); a.add_argument("component")
    a = sp.add_parser("builder-prompt"); a.add_argument("component"); a.add_argument("condition")
    a = sp.add_parser("skeleton-prompt"); a.add_argument("component"); a.add_argument("kind", choices=["blind", "derived"])
    a = sp.add_parser("sandbox"); a.add_argument("component"); a.add_argument("out", type=Path)
    a.add_argument("--stub", type=Path); a.add_argument("--with-reference", action="store_true")
    a = sp.add_parser("lane"); a.add_argument("vendor"); a.add_argument("cwd", type=Path)
    a.add_argument("prompt", type=Path); a.add_argument("logdir", type=Path)
    a = sp.add_parser("audit"); a.add_argument("logdir", type=Path); a.add_argument("sandbox", type=Path)
    a.add_argument("component"); a.add_argument("--meta", type=Path)
    a = sp.add_parser("grade"); a.add_argument("component"); a.add_argument("out", type=Path)
    a.add_argument("--candidate", type=Path); a.add_argument("--target", default="candidate")
    a.add_argument("--grader", type=Path, required=True); a.add_argument("--max-wall", type=int, default=3600)
    a.add_argument("--max-mutants"); a.add_argument("--seed", default="1")
    a = sp.add_parser("skeleton"); a.add_argument("component"); a.add_argument("kind", choices=["blind", "derived"])
    a.add_argument("rundir", type=Path); a.add_argument("--scratch", type=Path, required=True)
    a = sp.add_parser("cell"); a.add_argument("component"); a.add_argument("condition", choices="ABC")
    a.add_argument("vendor"); a.add_argument("seed", type=int); a.add_argument("rundir", type=Path)
    a.add_argument("--scratch", type=Path, required=True)
    a = sp.add_parser("grade-cell"); a.add_argument("cell", type=Path)
    a.add_argument("--scratch", type=Path, required=True); a.add_argument("--max-wall", type=int, default=3600)
    a = sp.add_parser("grade-batch"); a.add_argument("cells", type=Path, nargs="+")
    a.add_argument("--scratch", type=Path, required=True); a.add_argument("--max-wall", type=int, default=14400)
    a.add_argument("--per-cell", type=int, default=3600)
    a = sp.add_parser("report"); a.add_argument("rundir", type=Path)
    x = ap.parse_args()
    if x.cmd == "report":
        print(cmd_report(x.rundir))
        return 0
    if x.cmd == "grade-batch":
        return cmd_grade_batch(x.cells, x.scratch, x.max_wall, x.per_cell)
    if getattr(x, "component", None) and x.component not in COMPONENTS:
        raise SystemExit(f"unknown component {x.component}")
    if x.cmd == "spec":
        print(cmd_spec(x.component))
    elif x.cmd == "builder-prompt":
        print(build_prompt(x.component, x.condition, None))
    elif x.cmd == "skeleton-prompt":
        print(skeleton_prompt(x.component, x.kind))
    elif x.cmd == "sandbox":
        print(json.dumps(cmd_sandbox(x.component, x.out, x.stub, x.with_reference), indent=2))
    elif x.cmd == "lane":
        print(json.dumps(cmd_lane(x.vendor, x.cwd, x.prompt, x.logdir), indent=2))
    elif x.cmd == "audit":
        v = audit_events((x.logdir / "events.jsonl").read_text(), x.sandbox, x.component, x.meta)
        print(json.dumps(v, indent=2))
        return 1 if v else 0
    elif x.cmd == "grade":
        env = {"SKELETON_PILOT_SEED": x.seed}
        if x.max_mutants:
            env["SKELETON_PILOT_MAX_MUTANTS"] = x.max_mutants
        return cmd_grade(x.component, x.candidate, x.out, x.target, x.grader, x.max_wall, env)
    elif x.cmd == "skeleton":
        print(json.dumps(cmd_skeleton(x.component, x.kind, x.rundir, x.scratch), indent=2))
    elif x.cmd == "cell":
        print(json.dumps(cmd_cell(x.component, x.condition, x.vendor, x.seed, x.rundir,
                                  x.scratch), indent=2))
    elif x.cmd == "grade-cell":
        print(json.dumps(cmd_grade_cell(x.cell, x.scratch, x.max_wall), indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())

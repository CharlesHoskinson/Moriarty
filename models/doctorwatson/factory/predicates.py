"""Load, validate and order qa/sherlock/predicates.yaml."""

import heapq
import json
import os
from dataclasses import dataclass, field

import yaml

from . import repo_root, sherlock_path

SETS = ("A", "B", "C", "U", "F")
TIERS = ("T0", "T1", "T2", "T3", "T4")
# E6/SWF-008: deployment target of a row. Stagenet is retired (dissent.md, Resolved 2026-11-08 (5)):
# it stays legal only on retired rows, as history.
NETWORKS = ("undeployed", "stagenet", "preview", "mainnet")
RETIRED_NETWORKS = ("stagenet",)
# E15/SWF-012/SWF-013: milestone row sets, as the roots of a dependency closure.
MILESTONES = {
    "sherlock-m6": ["C4"],         # Sherlock Preview qualification (tasks.md M6)
    "synthetic-rollup-preview": ["B9p"],     # synthetic-rollup Preview gate (task 6.6): B1-B3, B6, B7, B4p, B5p, B8p, B9p
    "synthetic-rollup-mainnet": ["B9m"],     # synthetic-rollup Mainnet gate (task 7.4): adds B4m, B5m, B8m, B9m
}


class PredicateError(Exception):
    def __init__(self, errors):
        self.errors = list(errors)
        super().__init__("; ".join(self.errors))


@dataclass
class Predicate:
    id: str
    set: str
    title: str
    tier: str
    check: str
    heavy: bool = False
    budget: str | None = None
    depends: list = field(default_factory=list)
    source: str | None = None
    holdout: bool = False
    retired: str | None = None   # E10/SWF-011: reason or anchor; sweeps and FIRST FAILING skip the row
    optional: bool = False       # E14/SWF-012: reported, never gating unless --include-optional
    network: str | None = None   # E6/SWF-008: undeployed | stagenet (retired rows only) | preview | mainnet
    deps: dict | None = None     # E7/SAS-002: declared dependency closure; None -> default closure
    index: int = 0

    @property
    def mandatory(self):
        return self.retired is None and not self.optional


def swept(preds, include_optional=False):
    """Rows a sweep or FIRST FAILING considers: never retired; optional only on request."""
    return [p for p in preds if p.retired is None and (include_optional or not p.optional)]


def closure(preds, roots):
    """The roots plus every row they depend on, transitively, in file order."""
    by_id = {p.id: p for p in preds}
    seen = set()
    stack = [r for r in roots if r in by_id]
    while stack:
        pid = stack.pop()
        if pid in seen:
            continue
        seen.add(pid)
        stack.extend(d for d in by_id[pid].depends if d in by_id)
    return [p for p in preds if p.id in seen]


def lineage(pid):
    """Key lineage stem of a network row id: `A8m`/`A8p` -> `A8` (SWF-008)."""
    return pid[:-1] if len(pid) > 1 and pid[-1] in "pm" else pid


def pins_networks(repo):
    """Network names in qa/sherlock/target-pins.json, or None when the file is absent or unreadable."""
    path = sherlock_path("target-pins.json", repo)
    try:
        with open(path, encoding="utf-8") as fh:
            nets = json.load(fh).get("networks")
    except (OSError, ValueError, AttributeError):
        return None
    return set(nets) if isinstance(nets, dict) else None


def yaml_path(repo=None):
    return sherlock_path("predicates.yaml", repo)


def _read_raw(repo):
    path = yaml_path(repo)
    if not os.path.exists(path):
        return None, ["qa/sherlock/predicates.yaml: not found"]
    try:
        with open(path, encoding="utf-8") as fh:
            data = yaml.safe_load(fh)
    except yaml.YAMLError as exc:
        return None, [f"qa/sherlock/predicates.yaml: invalid YAML: {exc}"]
    if not isinstance(data, dict) or not isinstance(data.get("predicates"), list):
        return None, ["qa/sherlock/predicates.yaml: top level must be a mapping with a `predicates` list"]
    return data, []


def _nonempty_str(v):
    return isinstance(v, str) and v.strip() != ""


def _find_cycles(ids, deps):
    """Return a list of ids that sit on a dependency cycle."""
    state = {}
    on_cycle = set()

    def visit(node, stack):
        state[node] = 1
        stack.append(node)
        for d in deps.get(node, []):
            if d not in ids:
                continue
            if state.get(d) == 1:
                on_cycle.update(stack[stack.index(d):])
            elif d not in state:
                visit(d, stack)
        stack.pop()
        state[node] = 2

    for i in ids:
        if i not in state:
            visit(i, [])
    return sorted(on_cycle)


DEPS_LISTS = ("paths", "manifests", "receipts")   # repo-relative paths / globs
DEPS_ROWS = ("pins", "chain")                      # target-pins.json network row names


def _deps_errors(where, deps):
    """E7: `deps:` is a mapping of the keys above; every value names something to fingerprint."""
    if not isinstance(deps, dict):
        return [f"{where}: deps must be a mapping"]
    errors = []
    for k in sorted(deps, key=str):
        if k not in DEPS_LISTS + DEPS_ROWS:
            errors.append(f"{where}: deps has unknown key {k!r}")
    for k in DEPS_LISTS:
        v = deps.get(k)
        if k in deps and not (isinstance(v, list) and all(_nonempty_str(x) for x in v)):
            errors.append(f"{where}: deps.{k} must be a list of non-empty strings")
    for k in DEPS_ROWS:
        v = deps.get(k)
        if k in deps and not (_nonempty_str(v) or (isinstance(v, list) and v and all(_nonempty_str(x) for x in v))):
            errors.append(f"{where}: deps.{k} must be a network name or a list of them")
    return errors


def validate_data(data, networks=None):
    """`networks`: names allowed by the pins file (None = not checked against a pins file)."""
    errors = []
    items = data["predicates"]
    seen = {}
    ids_list = []
    for n, item in enumerate(items):
        where = f"predicates[{n}]"
        if not isinstance(item, dict):
            errors.append(f"{where}: must be a mapping")
            continue
        pid = item.get("id")
        if not _nonempty_str(pid):
            errors.append(f"{where}: id must be a non-empty string")
            continue
        where = f"{pid}"
        if pid in seen:
            errors.append(f"{pid}: duplicate id")
        seen[pid] = item
        ids_list.append(pid)
        if item.get("set") not in SETS:
            errors.append(f"{where}: set must be one of {', '.join(SETS)} (got {item.get('set')!r})")
        if item.get("tier") not in TIERS:
            errors.append(f"{where}: tier must be one of {', '.join(TIERS)} (got {item.get('tier')!r})")
        if not _nonempty_str(item.get("title")):
            errors.append(f"{where}: title must be a non-empty string")
        if not _nonempty_str(item.get("check")):
            errors.append(f"{where}: check must be a non-empty string")
        if "retired" in item and not _nonempty_str(item.get("retired")):
            errors.append(f"{where}: retired must be a non-empty reason or anchor string")
        for flag in ("heavy", "holdout", "optional"):
            if flag in item and not isinstance(item[flag], bool):
                errors.append(f"{where}: {flag} must be true or false")
        if item.get("heavy") is True and not _nonempty_str(item.get("budget")):
            errors.append(f"{where}: heavy predicate needs a budget key")
        if "deps" in item:
            errors.extend(_deps_errors(where, item["deps"]))
        dep = item.get("depends")
        if dep is not None and not isinstance(dep, list):
            errors.append(f"{where}: depends must be a list of ids")
        net = item.get("network")
        if "network" in item:
            if net not in NETWORKS:
                errors.append(f"{where}: network must be one of {', '.join(NETWORKS)} (got {net!r})")
            elif net in RETIRED_NETWORKS and "retired" not in item:
                errors.append(f"{where}: network {net} is retired; only a retired row may carry it")
            elif networks is not None and net not in networks:
                errors.append(f"{where}: network {net!r} has no row in qa/sherlock/target-pins.json")
        if "retired" not in item:
            if item.get("tier") == "T4" and net != "mainnet":
                errors.append(f"{where}: a T4 row must carry network: mainnet")
            elif net == "mainnet" and item.get("tier") != "T4":
                errors.append(f"{where}: a network: mainnet row must be tier T4")
    idset = set(ids_list)
    deps = {}
    for pid, item in seen.items():
        dl = item.get("depends") or []
        if not isinstance(dl, list):
            continue
        deps[pid] = dl
        for d in dl:
            if d == pid:
                errors.append(f"{pid}: depends on itself")
            elif d not in idset:
                errors.append(f"{pid}: depends on unknown id {d!r}")
    for pid, item in seen.items():
        if "retired" in item:
            continue  # retired rows may keep depending on retired or optional rows (history)
        for d in deps.get(pid, []):
            target = seen.get(d)
            if d == pid or not isinstance(target, dict):
                continue
            if "retired" in target:
                errors.append(f"{pid}: active row depends on retired row {d!r}")
            elif target.get("optional") is True and item.get("optional") is not True:
                errors.append(f"{pid}: mandatory row depends on optional row {d!r}")
    # SWF-008: a T4 (mainnet) row depends on the T3 Preview row of the same key lineage
    for pid, item in seen.items():
        if item.get("tier") != "T4" or "retired" in item:
            continue
        stem = lineage(pid)
        ok = any(isinstance(seen.get(d), dict) and lineage(d) == stem and d != pid
                 and seen[d].get("tier") == "T3" and seen[d].get("network") == "preview"
                 for d in deps.get(pid, []))
        if not ok:
            errors.append(f"{pid}: a T4 row must depend on its T3 Preview counterpart "
                          f"{stem + 'p'!r} (same key lineage)")
    cyc = _find_cycles(idset, {k: [d for d in v if d != k] for k, v in deps.items()})
    if cyc:
        errors.append("dependency cycle among: " + ", ".join(cyc))
    return errors


def validate(repo=None):
    """Return a list of error strings (empty = ok)."""
    repo = repo or repo_root()
    data, errors = _read_raw(repo)
    if data is None:
        return errors
    return validate_data(data, pins_networks(repo))


def load(repo=None):
    """Return predicates in file order; raise PredicateError if the file is invalid."""
    repo = repo or repo_root()
    data, errors = _read_raw(repo)
    if data is None:
        raise PredicateError(errors)
    errors = validate_data(data, pins_networks(repo))
    if errors:
        raise PredicateError(errors)
    out = []
    for n, item in enumerate(data["predicates"]):
        out.append(Predicate(
            id=item["id"], set=item["set"], title=item["title"], tier=item["tier"],
            check=item["check"], heavy=bool(item.get("heavy", False)),
            budget=item.get("budget"), depends=list(item.get("depends") or []),
            source=item.get("source"), holdout=bool(item.get("holdout", False)),
            retired=item.get("retired"), optional=item.get("optional") is True,
            network=item.get("network"), deps=item.get("deps"), index=n,
        ))
    return out


def topo_order(preds):
    """Dependencies first; among ready predicates the earliest in file order goes first."""
    by_id = {p.id: p for p in preds}
    indeg = {p.id: len([d for d in p.depends if d in by_id]) for p in preds}
    children = {p.id: [] for p in preds}
    for p in preds:
        for d in p.depends:
            if d in by_id:
                children[d].append(p.id)
    heap = [(p.index, p.id) for p in preds if indeg[p.id] == 0]
    heapq.heapify(heap)
    out = []
    while heap:
        _, pid = heapq.heappop(heap)
        out.append(by_id[pid])
        for c in children[pid]:
            indeg[c] -= 1
            if indeg[c] == 0:
                heapq.heappush(heap, (by_id[c].index, c))
    return out

"""Load, validate and order qa/sherlock/predicates.yaml."""

import heapq
import os
from dataclasses import dataclass, field

import yaml

from . import repo_root, sherlock_path

SETS = ("A", "B", "C", "U", "F")
TIERS = ("T0", "T1", "T2", "T3")


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
    index: int = 0


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


def validate_data(data):
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
        for flag in ("heavy", "holdout"):
            if flag in item and not isinstance(item[flag], bool):
                errors.append(f"{where}: {flag} must be true or false")
        if item.get("heavy") is True and not _nonempty_str(item.get("budget")):
            errors.append(f"{where}: heavy predicate needs a budget key")
        dep = item.get("depends")
        if dep is not None and not isinstance(dep, list):
            errors.append(f"{where}: depends must be a list of ids")
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
    return validate_data(data)


def load(repo=None):
    """Return predicates in file order; raise PredicateError if the file is invalid."""
    repo = repo or repo_root()
    data, errors = _read_raw(repo)
    if data is None:
        raise PredicateError(errors)
    errors = validate_data(data)
    if errors:
        raise PredicateError(errors)
    out = []
    for n, item in enumerate(data["predicates"]):
        out.append(Predicate(
            id=item["id"], set=item["set"], title=item["title"], tier=item["tier"],
            check=item["check"], heavy=bool(item.get("heavy", False)),
            budget=item.get("budget"), depends=list(item.get("depends") or []),
            source=item.get("source"), holdout=bool(item.get("holdout", False)),
            index=n,
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

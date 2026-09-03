"""Structural resource bounds for the experimental Moriarty Core."""

from __future__ import annotations

from dataclasses import dataclass

from moriarty.core import Case, Choice, Close, Contract, Deposit, If, Pay, When


@dataclass(frozen=True)
class Bounds:
    max_inputs: int
    max_reductions: int
    max_payments: int
    max_live_accounts: int
    max_timeout: int
    syntax_nodes: int


@dataclass(frozen=True)
class _Path:
    inputs: int
    reductions: int
    payments: int
    max_live_accounts: int


def _maximum(paths: tuple[_Path, ...]) -> _Path:
    return _Path(
        inputs=max(path.inputs for path in paths),
        reductions=max(path.reductions for path in paths),
        payments=max(path.payments for path in paths),
        max_live_accounts=max(path.max_live_accounts for path in paths),
    )


def _path(contract: Contract, live_accounts: frozenset) -> _Path:
    live_count = len(live_accounts)
    if isinstance(contract, Close):
        return _Path(0, live_count, live_count, live_count)
    if isinstance(contract, Pay):
        remaining = live_accounts - {contract.account}
        continuation = _path(contract.continuation, remaining)
        return _Path(
            continuation.inputs,
            1 + continuation.reductions,
            1 + continuation.payments,
            max(live_count, continuation.max_live_accounts),
        )
    if isinstance(contract, If):
        return _maximum(
            (
                _path(contract.then_contract, live_accounts),
                _path(contract.else_contract, live_accounts),
            )
        )
    if isinstance(contract, When):
        paths: list[_Path] = []
        for candidate in contract.cases:
            next_live = live_accounts
            if isinstance(candidate.action, Deposit):
                next_live = live_accounts | {candidate.action.account}
            child = _path(candidate.continuation, next_live)
            paths.append(
                _Path(
                    1 + child.inputs,
                    child.reductions,
                    child.payments,
                    max(live_count, child.max_live_accounts),
                )
            )
        timeout = _path(contract.timeout_continuation, live_accounts)
        paths.append(
            _Path(
                timeout.inputs,
                1 + timeout.reductions,
                timeout.payments,
                max(live_count, timeout.max_live_accounts),
            )
        )
        return _maximum(tuple(paths))
    raise TypeError(f"unsupported contract: {type(contract).__name__}")


def _syntax_nodes(contract: Contract) -> int:
    if isinstance(contract, Close):
        return 1
    if isinstance(contract, Pay):
        return 2 + _syntax_nodes(contract.continuation)
    if isinstance(contract, If):
        return (
            2
            + _syntax_nodes(contract.then_contract)
            + _syntax_nodes(contract.else_contract)
        )
    if isinstance(contract, When):
        cases = sum(_case_nodes(candidate) for candidate in contract.cases)
        return 1 + cases + _syntax_nodes(contract.timeout_continuation)
    raise TypeError(f"unsupported contract: {type(contract).__name__}")


def _case_nodes(candidate: Case) -> int:
    action_nodes = 2 if isinstance(candidate.action, Deposit) else 1
    if not isinstance(candidate.action, (Deposit, Choice)):
        raise TypeError(f"unsupported action: {type(candidate.action).__name__}")
    return 1 + action_nodes + _syntax_nodes(candidate.continuation)


def _max_timeout(contract: Contract) -> int:
    if isinstance(contract, Close):
        return 0
    if isinstance(contract, Pay):
        return _max_timeout(contract.continuation)
    if isinstance(contract, If):
        return max(
            _max_timeout(contract.then_contract),
            _max_timeout(contract.else_contract),
        )
    if isinstance(contract, When):
        children = [
            _max_timeout(candidate.continuation) for candidate in contract.cases
        ]
        children.append(_max_timeout(contract.timeout_continuation))
        return max(contract.timeout, *children)
    raise TypeError(f"unsupported contract: {type(contract).__name__}")


def analyze_bounds(contract: Contract) -> Bounds:
    path = _path(contract, frozenset())
    return Bounds(
        max_inputs=path.inputs,
        max_reductions=path.reductions,
        max_payments=path.payments,
        max_live_accounts=path.max_live_accounts,
        max_timeout=_max_timeout(contract),
        syntax_nodes=_syntax_nodes(contract),
    )

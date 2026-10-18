"""Join complete actual observation windows; no evaluator or expected access.

The existing CompleteKWire.complete_states already decodes each window and
checks cumulative event-prefix continuity even with nonempty initial events.
Only a byte-verbatim K final state may seed the subsequent actual query.
"""


TERMINAL = {"ngHalted", "ngOutsideModel"}


def _same(left, right):
    if type(left) is not type(right):
        return False
    if type(left) is dict:
        return left.keys() == right.keys() and all(_same(left[key], right[key]) for key in left)
    if type(left) is list:
        return len(left) == len(right) and all(_same(a, b) for a, b in zip(left, right))
    return left == right


def join_complete_chunks(chunks, *, overall_bound):
    """Return original decoded state objects, dropping one equal boundary row.

    chunks contains (selected_bound, complete_actual_states) pairs in actual
    execution order. It is not a function from a source program to any state.
    """
    if type(overall_bound) is not int or overall_bound < 0:
        raise ValueError("Invalid overall observation bound")
    joined, used = [], 0
    program = None
    for bound, states in chunks:
        if type(bound) is not int or bound < 0 or bound > overall_bound - used:
            raise ValueError("Window exceeds remaining observation bound")
        if bound == 0 and (overall_bound != 0 or joined):
            raise ValueError("Zero window is only the complete zero-bound observation")
        if not states or len(states) > bound + 1:
            raise ValueError("Missing or oversized actual window")
        if joined:
            if next(iter(joined[-1]["control"])) in TERMINAL:
                raise ValueError("An actual query followed a terminal state")
            if not _same(joined[-1], states[0]):
                raise ValueError("Complete actual continuation boundary differs")
        elif states[0]["events"]:
            raise ValueError("Whole trace must start with empty initial events")
        if program is None:
            program = states[0]["program"]
        for state in states:
            if not _same(state["program"], program):
                raise ValueError("Immutable complete Program changed")
        for prior, current in zip(states, states[1:]):
            if next(iter(prior["control"])) in TERMINAL:
                raise ValueError("State followed terminal control")
            if len(current["events"]) != len(prior["events"]) + 1 or not _same(current["events"][:-1], prior["events"]):
                raise ValueError("Cumulative actual events do not extend by one")
        advancing = len(states) - 1
        if advancing < bound and next(iter(states[-1]["control"])) not in TERMINAL:
            raise ValueError("Short window without actual terminal control")
        joined.extend(states[1:] if joined else states)
        used += advancing
    if not joined:
        raise ValueError("No actual windows supplied")
    if used < overall_bound and next(iter(joined[-1]["control"])) not in TERMINAL:
        raise ValueError("Whole observation is still incomplete")
    if len(joined) != used + 1:
        raise ValueError("Unexpected state loss or duplication")
    return joined

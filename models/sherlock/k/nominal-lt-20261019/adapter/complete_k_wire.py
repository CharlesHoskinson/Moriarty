"""Complete structural observation export; no evaluator, parser or file access.

The selected consumer supplies a complete typed term from the existing parser,
the approved public payload API, and that parser's constructor registry.  This
module does not import the parser or consume independent expected data.
"""

from typing import Any, Iterable, Mapping, Protocol, Sequence


class Term(Protocol):
    sort: str
    name: str
    args: tuple


class ObservationFormatError(ValueError):
    """The supplied object is not the complete declared observation format."""


class CompleteKWire:
    SCHEMA = "independent-nominal-full-state/1"
    GROUPS = {
        "NGControl": "control_payload_order",
        "NGFrame": "frame_payload_order",
        "NGEvent": "event_payload_order",
    }
    SEQUENCES = {
        "NGValues": ("ngNoValues", "ngValue"),
        "NGTypes": ("ngNoTypes", "ngType"),
        "NGExprIds": ("ngNoExprs", "ngExprId"),
        "NGPatterns": ("ngNoPatterns", "ngPattern"),
        "NGFrames": ("ngNoFrames", "ngFrame"),
        "NGEvents": ("ngNoEvents", "ngEvent"),
        "NGStates": ("ngNoStates", "ngStateRow"),
        "NGCtors": ("ngNoCtors", "ngCtorRow"),
        "NGFunctions": ("ngNoFunctions", "ngFunctionRow"),
        "NGPath": ("ngRoot", "ngAt"),
    }
    EXPRESSIONS = {
        "ngLiteral": ("Literal", ("value",)),
        "ngVariable": ("Variable", ("name_id",)),
        "ngAdd": ("Add", ("left", "right")),
        "ngLt": ("Lt", ("left", "right")),
        "ngConstruct": ("Construct", ("type_id", "ctor_id", "fields")),
        "ngCall": ("Call", ("name_id", "arguments")),
        "ngMatch": ("Match", ("scrutinee", "arms")),
        "ngLet": ("Let", ("binder", "bound_expr", "body_expr")),
        "ngIf": ("If", ("condition_expr", "yes_expr", "no_expr")),
    }

    def __init__(
        self,
        public_api: Mapping[str, Any],
        registry: Mapping[str, tuple[str, Sequence[str]]],
    ) -> None:
        # Copy declarations once: decoding does not mutate shared codec globals.
        self.registry = {
            name: (sort, tuple(children))
            for name, (sort, children) in registry.items()
        }
        self.fields = {
            sort: {
                name: tuple(fields)
                for name, fields in public_api[group].items()
            }
            for sort, group in self.GROUPS.items()
        }
        for sort, constructors in self.fields.items():
            for name, fields in constructors.items():
                declaration = self.registry.get(name)
                if declaration is None or declaration[0] != sort:
                    raise ObservationFormatError("Missing declared variant: " + name)
                if len(fields) != len(declaration[1]):
                    raise ObservationFormatError("Payload arity differs: " + name)
                if len(set(fields)) != len(fields):
                    raise ObservationFormatError("Repeated payload field: " + name)

    def _args(
        self, term: Term, sort: str, name: str | None = None
    ) -> tuple:
        if term.sort != sort or (name is not None and term.name != name):
            raise ObservationFormatError(
                "Expected " + str((sort, name))
                + ", received " + str((term.sort, term.name))
            )
        declaration = self.registry.get(term.name)
        if declaration is None or declaration[0] != sort:
            raise ObservationFormatError("Unknown constructor: " + term.name)
        children = declaration[1]
        if len(term.args) != len(children):
            raise ObservationFormatError("Constructor arity differs: " + term.name)
        for argument, child_sort in zip(term.args, children):
            if argument.sort != child_sort:
                raise ObservationFormatError("Child sort differs: " + term.name)
        return term.args

    def _record(self, keys: Sequence[str], values: Sequence[Any]) -> dict:
        if len(keys) != len(values):
            raise ObservationFormatError("Record field count differs")
        return dict(zip(keys, values))

    def _rows(self, term: Term, empty: str, row: str) -> list[tuple]:
        sort = term.sort
        rows = []
        current = term
        while True:
            args = self._args(current, sort)
            if current.name == empty:
                if args:
                    raise ObservationFormatError("Nonempty list terminator")
                return rows
            if current.name != row or not args:
                raise ObservationFormatError("Unexpected list constructor")
            rows.append(args[:-1])
            current = args[-1]

    def _decode_rows(
        self, term: Term, empty: str, row: str, fields: Sequence[str]
    ) -> list[dict]:
        return [
            self._record(fields, [self.decode(child) for child in payload])
            for payload in self._rows(term, empty, row)
        ]

    def decode(self, term: Term) -> Any:
        """Decode all supplied fields; function expressions remain unacceptable."""
        sort, name, args = term.sort, term.name, term.args
        scalar_types = {"Int": int, "Bool": bool, "String": str}
        if sort in scalar_types:
            if (
                name != "@" + sort
                or len(args) != 1
                or type(args[0]) is not scalar_types[sort]
            ):
                raise ObservationFormatError("Unexpected scalar representation")
            return args[0]
        if sort == "Map":
            if name != "@Map":
                raise ObservationFormatError("Expected complete parsed Map")
            rows = []
            for pair in args:
                if len(pair) != 2 or pair[0].sort != "Int" or pair[1].sort != "NGValue":
                    raise ObservationFormatError("Map binding shape differs")
                rows.append({
                    "name_id": self.decode(pair[0]),
                    "value": self.decode(pair[1]),
                })
            # Preserve every row.  In particular there is no dict overwrite.
            return sorted(rows, key=lambda binding: binding["name_id"])
        if sort in self.SEQUENCES:
            empty, row = self.SEQUENCES[sort]
            result = []
            for payload in self._rows(term, empty, row):
                if len(payload) != 1:
                    raise ObservationFormatError("Sequence element arity differs")
                result.append(self.decode(payload[0]))
            return result
        if sort in self.GROUPS:
            actual = self._args(term, sort)
            keys = self.fields[sort].get(name)
            if keys is None:
                raise ObservationFormatError("Variant absent from public API: " + name)
            # Exact arity is checked before pairing; no payload tail disappears.
            payload = self._record(keys, [self.decode(child) for child in actual])
            return {name: payload}
        if sort == "NGValue":
            actual = self._args(term, sort)
            values = [self.decode(child) for child in actual]
            if name == "ngUInt":
                return {"UInt": self._single(values)}
            if name == "ngBool":
                return {"Bool": self._single(values)}
            if name == "ngNominal":
                return {"Nominal": self._record(("type_id", "ctor_id", "fields"), values)}
        elif sort == "NGPattern":
            values = [self.decode(child) for child in self._args(term, sort)]
            if name == "ngWildcard" and not values:
                return {"Wildcard": {}}
            if name == "ngBind":
                return {"Bind": self._single(values)}
            if name == "ngConstructorPattern":
                return {"Constructor": self._record(("type_id", "ctor_id", "fields"), values)}
        elif sort == "NGExpr":
            actual = self._args(term, sort)
            declaration = self.EXPRESSIONS.get(name)
            if declaration is None or not actual:
                raise ObservationFormatError("Expression form is not declared")
            tag, fields = declaration
            occurrence = self.decode(actual[0])
            values = [self.decode(child) for child in actual[1:]]
            payload = self._record(fields, values)
            return {
                "occurrence": occurrence,
                "kind": {tag: payload["value"] if tag == "Literal" else payload},
            }
        elif sort == "NGNodes":
            nodes = []
            for payload in self._rows(term, "ngNoNodes", "ngNode"):
                if len(payload) != 2:
                    raise ObservationFormatError("Expression row arity differs")
                nodes.append({"id": self.decode(payload[0]), **self.decode(payload[1])})
            return nodes
        elif sort == "NGDecls":
            return self._decode_rows(
                term, "ngNoDecls", "ngDecl", ("type_id", "recursive_group", "ctors")
            )
        elif sort == "NGParams":
            return self._decode_rows(
                term, "ngNoParams", "ngParam", ("name_id", "type_id")
            )
        elif sort == "NGArms":
            return self._decode_rows(
                term, "ngNoArms", "ngArm", ("source_arm_id", "pattern", "body")
            )
        elif sort == "NGBindings":
            return self._decode_rows(
                term, "ngNoBindings", "ngBinding",
                ("name_id", "match_expr_id", "source_arm_id", "field_path", "value"),
            )
        else:
            records = {
                "NGCtor": ("ngCtor", ("type_id", "ctor_id", "source_order", "field_types")),
                "NGFunction": ("ngFunction", ("name_id", "parameters", "return_type_id", "body")),
                "NGProgram": ("ngProgram", ("declarations", "expressions", "functions", "entry")),
                "NGState": ("ngState", ("program", "control", "environment", "frames_top_first", "events")),
                "NGTape": ("ngTape", ("initial", "after_each_event", "final")),
            }
            if sort in records:
                constructor, fields = records[sort]
                values = [self.decode(child) for child in self._args(term, sort, constructor)]
                return self._record(fields, values)
        raise ObservationFormatError("Undecoded declared datum: " + str((sort, name)))

    @staticmethod
    def _single(values: Sequence[Any]) -> Any:
        if len(values) != 1:
            raise ObservationFormatError("Expected exactly one payload")
        return values[0]

    def complete_states(self, tape: Term) -> list[dict]:
        """Decode and validate every actual snapshot without expected metadata."""
        decoded = self.decode(tape)
        if tape.sort != "NGTape" or tape.name != "ngTape":
            raise ObservationFormatError("Expected a fully evaluated ngTape")
        states = [decoded["initial"], *decoded["after_each_event"]]
        if decoded["final"] != states[-1]:
            raise ObservationFormatError("Final differs from last returned state")
        program = states[0]["program"]
        for current in states:
            if current["program"] != program:
                raise ObservationFormatError("Observed immutable Program changed")
        for previous, current in zip(states, states[1:]):
            prior_tag = next(iter(previous["control"]))
            if (prior_tag in ("ngHalted", "ngOutsideModel")
                    or (prior_tag == "ngLtError" and previous["control"][prior_tag]["code"] == 11)):
                raise ObservationFormatError("An observation followed a terminal state")
            prior_events, events = previous["events"], current["events"]
            if len(events) != len(prior_events) + 1 or events[:-1] != prior_events:
                raise ObservationFormatError("Cumulative event continuity differs")
        return states

    def bind_shared_wire(
        self,
        states: list[dict],
        *,
        case_id: str,
        profile: str,
        terminal_step_contract: str,
        observer_cuts: Iterable[int],
    ) -> dict:
        """Bind public metadata to already decoded complete actual states."""
        if not states:
            raise ObservationFormatError("Complete observations include an initial state")
        cuts = list(observer_cuts)
        if any(type(cut) is not int or cut < 0 or cut >= len(states) for cut in cuts):
            raise ObservationFormatError("Cut ordinal is outside the observed prefix")
        if any(type(text) is not str for text in (case_id, profile, terminal_step_contract)):
            raise ObservationFormatError("Public wire metadata must be strings")
        return {
            "schema": self.SCHEMA,
            "profile": profile,
            "caseId": case_id,
            "states": states,
            "terminalStepContract": terminal_step_contract,
            "observerCuts": cuts,
        }

    def to_shared_wire(
        self,
        tape: Term,
        *,
        case_id: str,
        profile: str,
        terminal_step_contract: str,
        observer_cuts: Iterable[int],
    ) -> dict:
        """Export complete observations with explicitly supplied public metadata."""
        return self.bind_shared_wire(
            self.complete_states(tape),
            case_id=case_id,
            profile=profile,
            terminal_step_contract=terminal_step_contract,
            observer_cuts=observer_cuts,
        )

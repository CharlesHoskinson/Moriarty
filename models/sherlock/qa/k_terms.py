"""Normalize the finite Sherlock diagnostic TERM vocabulary, without evaluation.

The admitted syntax is the concrete constructors in main.k, decimal Ints,
true/false, .K, .Map, and finite distinct Int-key map bindings. Ordinary
parentheses around terms/maps are accepted. Constructor/list/frame/source
order stays intact. Map display follows reference_display.rs: ascending host
String order of canonical signed-decimal keys, rather than numeric order.

This is diagnostic syntax normalization, not a K sort checker, rule evaluator,
state importer or proof. K helpers, symbolic terms, KAST, Kore and logs are
outside this vocabulary. No runtime output format is established by these
hand-authored syntax examples. Callers retain original text separately.

parse_term returns Ints as Python ints, literal atoms as strings, constructors
as (name, argument_list), and maps as ('map', [(int_key, value), ...]). The
parsed map retains supplied binding order; render_term alone orders its display.
"""
import re


ARITIES = {
    'uiU': 1, 'uiNil': 0, 'uiCons': 2, 'uiLeaf': 1, 'uiBranch': 2, 'uiB': 1,
    'uiUIntT': 0, 'uiBoolT': 0, 'uiListT': 0, 'uiTreeT': 0,
    'uiAdd': 0, 'uiSub': 0, 'uiMul': 0, 'uiEq': 0, 'uiLt': 0,
    'uiConsOp': 0, 'uiBranchOp': 0, 'uiLeafOp': 0, 'uiNotOp': 0,
    'uiLiteral': 1, 'uiVar': 1, 'uiLet': 3, 'uiBinary': 3, 'uiUnary': 2,
    'uiIf': 3, 'uiCall': 2, 'uiMatch': 2, 'uiNoExprs': 0, 'uiExpr': 2,
    'uiNoValues': 0, 'uiValue': 2, 'uiPVar': 1, 'uiPNil': 0,
    'uiPCons': 2, 'uiPLeaf': 1, 'uiPBranch': 2, 'uiNoCases': 0, 'uiCase': 3,
    'uiNoParams': 0, 'uiParam': 3, 'uiFun': 4, 'uiNoFuns': 0, 'uiFunction': 2,
    'uiRun': 4, 'uiEval': 1, 'uiRet': 1, 'uiPush': 3, 'uiInvoke': 2,
    'uiEnter': 2, 'uiBound': 3, 'uiCases': 3, 'uiCaseResult': 5,
    'uiPrimResult': 1, 'uiError': 1, 'uiLetF': 3, 'uiLeftF': 3,
    'uiRightF': 2, 'uiUnaryF': 1, 'uiIfF': 3, 'uiMatchF': 2,
    'uiArgsF': 4, 'uiReturnF': 2, 'uiScopeF': 1, 'uiNoFrames': 0, 'uiFrame': 2,
    'uiPending': 0, 'uiDone': 1, 'uiRejected': 1, 'uiExhausted': 0,
    'uiMissingFun': 0, 'uiBindings': 1, 'uiBadBinding': 0,
    'uiMatched': 1, 'uiNoMatch': 0, 'uiPrimValue': 1, 'uiPrimError': 1,
}
_INTEGER = re.compile(r'-?[0-9]+\Z')
_TOKEN = re.compile(r'\s*(\|->|\.Map\b|\.K\b|-?[0-9]+|[A-Za-z][A-Za-z0-9]*|[(),])')


class _Terms:
    def __init__(self, text):
        if type(text) is not str:
            raise ValueError('diagnostic term must be text')
        self.tokens = []
        position = 0
        while text[position:].strip():
            match = _TOKEN.match(text, position)
            if match is None:
                raise ValueError('unsupported concrete Sherlock term syntax')
            self.tokens.append(match[1])
            position = match.end()
        self.position = 0

    def peek(self):
        return self.tokens[self.position] if self.position < len(self.tokens) else None

    def take(self, expected=None):
        value = self.peek()
        if value is None or (expected is not None and value != expected):
            raise ValueError('incomplete or unsupported concrete Sherlock term')
        self.position += 1
        return value

    def term(self):
        node = self.atom()
        if self.peek() != '|->':
            return node
        pairs = []
        keys = set()
        while True:
            if type(node) is not int:
                raise ValueError('diagnostic vocabulary requires Int map keys')
            if node in keys:
                raise ValueError('finite diagnostic map requires distinct Int keys')
            keys.add(node)
            self.take('|->')
            pairs.append((node, self.atom()))
            if (self.position + 1 < len(self.tokens)
                    and self.tokens[self.position + 1] == '|->'):
                node = self.atom()
            else:
                break
        return ('map', pairs)

    def atom(self):
        name = self.take()
        if name == '(':
            value = self.term()
            self.take(')')
            return value
        if _INTEGER.fullmatch(name):
            return int(name)
        if name in ('.K', '.Map', 'true', 'false'):
            return name
        if name not in ARITIES:
            raise ValueError('unsupported constructor or unresolved K helper: ' + name)
        self.take('(')
        arguments = []
        if self.peek() != ')':
            arguments.append(self.term())
            while self.peek() == ',':
                self.take(',')
                arguments.append(self.term())
        self.take(')')
        if len(arguments) != ARITIES[name]:
            raise ValueError('constructor arity differs from main.k: ' + name)
        return (name, arguments)


def parse_term(text: str):
    """Parse one complete concrete diagnostic term; unsupported syntax raises ValueError."""
    parser = _Terms(text)
    result = parser.term()
    if parser.peek() is not None:
        raise ValueError('multiple or unsupported concrete Sherlock terms')
    return result


def render_term(node) -> str:
    """Render a parse_term tree with Rust KDisplay spacing and map key order."""
    if type(node) is int or type(node) is str:
        return str(node)
    name, arguments = node
    if name == 'map':
        return ' '.join(str(key) + ' |-> ' + render_term(value)
                        for key, value in sorted(arguments, key=lambda pair: str(pair[0])))
    return name + '(' + ', '.join(render_term(value) for value in arguments) + ')'


def normalize_term(text: str) -> str:
    """Canonicalize only diagnostic syntax; preserve every supplied term's meaning."""
    return render_term(parse_term(text))

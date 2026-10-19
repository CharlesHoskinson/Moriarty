"""Exact existing F03 parsed_problem carrier gate; no new policy or launcher."""

UIRUN_OPEN = (b"LbluiRun'LParUndsCommUndsCommUndsCommUndsRParUnds'MORIARTY-UI-SYNTAX'Unds'UIRun"
              b"'Unds'UIFuns'Unds'UIExpr'Unds'Int'Unds'Int{}(")

PARSED_LIMIT = 64 * 1024

def parsed_problem(raw):
    """Bounded structural gate on the kast output; no expected outcome is consulted."""
    if not 0 < len(raw) <= PARSED_LIMIT:
        return 'parsed-size-out-of-bounds'
    if not raw.isascii() or any((byte < 0x20 and byte not in b'\t\n\r') or byte == 0x7f for byte in raw):
        return 'parsed-not-printable-ascii'
    term = raw.strip()
    if not term.startswith(UIRUN_OPEN):
        return 'parsed-root-is-not-the-compiled-uiRun-symbol'
    depth, index, closed = 0, 0, None
    while index < len(term):
        byte = term[index]
        if byte == 0x22:  # String literal: skip contents, honouring backslash escapes.
            index += 1
            while index < len(term) and term[index] != 0x22:
                index += 2 if term[index] == 0x5c else 1
            if index >= len(term):
                return 'parsed-unterminated-string-literal'
        elif byte == 0x5c and not term.startswith(b'\\dv{', index):
            return 'parsed-matching-logic-connective-other-than-dv'
        elif byte == 0x3a:
            return 'parsed-contains-variable'
        elif byte == 0x28:
            depth += 1
        elif byte == 0x29:
            depth -= 1
            if depth < 0:
                return 'parsed-unbalanced-parentheses'
            if depth == 0 and closed is None:
                closed = index
        index += 1
    if depth or closed != len(term) - 1:
        return 'parsed-not-one-closed-application'
    return None

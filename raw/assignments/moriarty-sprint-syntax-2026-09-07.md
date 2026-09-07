# Sprint syntax requirement

User clarification, 2026-09-07:

> make sure we have in the plans Syntax: Backus–Naur Form (BNF) and its variants. BNF describes a context-free grammar as production rules; EBNF (standardized as ISO/IEC 14977) adds notation for repetition and optionality, and ABNF (RFC 5234) is the flavor used in internet protocol specs. Nearly every language spec uses one of these for the grammar. Lexical structure is usually given separately as regular expressions or a small grammar of its own.

The sprint specification records BNF and both variants, keeps Moriarty's selected grammar notation EBNF, and requires a separate lexical specification. K operational semantics remains selected.

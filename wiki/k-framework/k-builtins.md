---
id: k.framework.builtins
type: language
title: K builtin domains
status: active
updated_at: 2026-09-03T14:35:20Z
sources:
  - SRC-0023
---

# K Builtin Domains

This document provides a comprehensive technical reference for the builtin domains of the K Framework standard library. It covers the core datatypes declared in `domains.md`, the internal AST representation constructs in `kast.md`, the prelude infrastructure, and auxiliary modules including substitution, unification, rational arithmetic, JSON parsing, the foreign function interface (FFI), and execution timers. The document concludes with a dedicated analysis labelled inference detailing the mathematical and operational representation of prime fields, fixed-width words, byte sequences, elliptic curve points, and circuit vectors for ZKIR formal semantics.

## The Builtin Library Architecture and Prelude

The K standard library is installed under `include/kframework/builtin/` (CLM-0255; SRC-0023 include/kframework/README.md; source fact; not reproduced; high; S6). Every K definition automatically includes `prelude.md` unless the `--no-prelude` flag is explicitly passed to `kompile` (CLM-0255; SRC-0023 include/kframework/builtin/prelude.md; source fact; not reproduced; high; S6).

`prelude.md` requires two root files:
```k
requires "kast.md"
requires "domains.md"
```

The standard library separates surface lexical syntax from operational signatures (CLM-0255; SRC-0023 include/kframework/builtin/domains.md #default-modules; source fact; not reproduced; high; S6):
- `DOMAINS-SYNTAX`: contains only program-level token syntax for identifiers (`Id`), integers (`Int`), booleans (`Bool`), and strings (`String`), suitable for importation by user syntax modules.
- `DOMAINS`: imports `DOMAINS-SYNTAX` alongside functional operator declarations for `INT`, `BOOL`, `STRING`, `ID`, `LIST`, `MAP`, `SET`, `K-IO`, and `K-EQUAL`.
- Non-default modules: Specialized modules are not included in `DOMAINS` and must be imported explicitly by name. These comprise `ARRAY`, `COLLECTIONS`, `FLOAT`, `STRING-BUFFER`, `BYTES`, `K-REFLECTION`, `MINT`, and `RANGEMAP` (CLM-0255; SRC-0023 include/kframework/builtin/domains.md lines 34-37; source fact; not reproduced; high; S6).

Partial functions in the standard library are not defined on all inputs; invoking a partial function on an unhandled input results in matching logic bottom (`#Bottom`), which causes LLVM backend interpreters to terminate or abort (CLM-0255; SRC-0023 include/kframework/builtin/domains.md lines 14-17; source fact; not reproduced; high; S6).

## Integer Arithmetic (`INT`)

The `INT` module provides arbitrary-precision integer arithmetic backed natively by the GNU Multiple Precision Arithmetic Library (GMP) in the LLVM backend runtime (CLM-0256; SRC-0023 docs/ktools.md line 291; repository observation; not reproduced; high; S6). Integers do not overflow or underflow.

### Sort and Lexical Syntax

```k
module UNSIGNED-INT-SYNTAX
  syntax Int [hook(INT.Int)]
  syntax Int ::= r"[0-9]+" [prefer, token, prec(2)]
endmodule

module INT-SYNTAX
  imports UNSIGNED-INT-SYNTAX
  syntax Int ::= r"[\\+\\-]?[0-9]+" [prefer, token, prec(2)]
endmodule
```

### Operators, Hooks, and Division Semantics

K provides two distinct division and remainder conventions:
1. *Truncated division (`/Int`, `%Int`)*: rounds quotients towards zero ($t$-division). SMT translation maps these via conditional expressions converting to Euclidean division (CLM-0256; SRC-0023 include/kframework/builtin/domains.md lines 1256-1261; source fact; not reproduced; high; S6).
2. *Euclidean division (`divInt`, `modInt`)*: satisfies the Euclidean division theorem where the remainder is strictly non-negative ($0 \le r < |d|$) (CLM-0256; SRC-0023 include/kframework/builtin/domains.md lines 1262-1265; source fact; not reproduced; high; S6).

```k
syntax Int ::= "~Int" Int                     [function, total, hook(INT.not)]
             > left:
               Int "^Int" Int                 [function, hook(INT.pow)]
             | Int "^%Int" Int Int            [function, hook(INT.powmod)]
             > left:
               Int "*Int" Int                 [function, total, hook(INT.mul)]
             | Int "/Int" Int                 [function, hook(INT.tdiv)]
             | Int "%Int" Int                 [function, hook(INT.tmod)]
             | Int "divInt" Int               [function, hook(INT.ediv)]
             | Int "modInt" Int               [function, hook(INT.emod)]
             > left:
               Int "+Int" Int                 [function, total, hook(INT.add)]
             | Int "-Int" Int                 [function, total, hook(INT.sub)]
             > left:
               Int ">>Int" Int                [function, hook(INT.shr)]
             | Int "<<Int" Int                [function, hook(INT.shl)]
             > left:
               Int "&Int" Int                 [function, total, hook(INT.and)]
             > left:
               Int "xorInt" Int               [function, total, hook(INT.xor)]
             > left:
               Int "|Int" Int                 [function, total, hook(INT.or)]
```

Notable specialized integer functions include:
- `A ^%Int B C`: modular exponentiation computing $(A^B) \pmod C$ with logarithmic asymptotic complexity (CLM-0256; SRC-0023 include/kframework/builtin/domains.md lines 1229-1231; source fact; not reproduced; high; S6).
- `minInt(I1, I2)` and `maxInt(I1, I2)`: minimum and maximum of two integers (CLM-0256; SRC-0023 include/kframework/builtin/domains.md line 1284; source fact; not reproduced; high; S6).
- `absInt(I)`: absolute value (CLM-0256; SRC-0023 include/kframework/builtin/domains.md line 1293; source fact; not reproduced; high; S6).
- `log2Int(I)`: floor of binary logarithm; returns `#Bottom` if $I \le 0$ (CLM-0256; SRC-0023 include/kframework/builtin/domains.md lines 1298-1305; source fact; not reproduced; high; S6).
- `bitRangeInt(I, Off, Len)`: extracts `Len` bits starting at offset `Off` in little-endian order (CLM-0256; SRC-0023 include/kframework/builtin/domains.md lines 1318-1320; source fact; not reproduced; high; S6).
- `signExtendBitRangeInt(I, Off, Len)`: extracts and sign-extends a bit field into two's complement representation (CLM-0256; SRC-0023 include/kframework/builtin/domains.md line 1320; source fact; not reproduced; high; S6).
- `freshInt(I)`: builtin generator for unique integer constants (CLM-0257; SRC-0023 include/kframework/builtin/domains.md lines 1456-1458; source fact; not reproduced; high; S6).
- `randInt(Limit)` and `srandInt(Seed)`: pseudorandom integer generation for concrete execution (CLM-0257; SRC-0023 include/kframework/builtin/domains.md lines 1346-1354; source fact; not reproduced; high; S6).

## Boolean Algebra (`BOOL`)

The `BOOL` module defines two-valued propositional logic:

```k
module SORT-BOOL
  syntax Bool [hook(BOOL.Bool)]
endmodule

module BOOL-SYNTAX
  imports SORT-BOOL
  syntax Bool ::= "true"  [token]
                | "false" [token]
endmodule
```

### Operators and Short-Circuit Evaluation

K distinguishes non-short-circuiting operators from short-circuiting operators (CLM-0258; SRC-0023 include/kframework/builtin/domains.md lines 1115-1118; source fact; not reproduced; high; S6):
- Strict conjunction and disjunction: `andBool`, `orBool`, `xorBool`, `impliesBool`. In symbolic backends, both arguments are evaluated eagerly (CLM-0258; SRC-0023 include/kframework/builtin/domains.md lines 1115-1118; source fact; not reproduced; high; S6).
- Short-circuiting conjunction and disjunction: `andThenBool` and `orElseBool`. If the left-hand operand determines the truth value, the right-hand operand is not evaluated (CLM-0258; SRC-0023 include/kframework/builtin/domains.md lines 1115-1118; source fact; not reproduced; high; S6).

```k
syntax Bool ::= "notBool" Bool          [function, total, hook(BOOL.not)]
              > Bool "andBool" Bool     [function, total, hook(BOOL.and)]
              | Bool "andThenBool" Bool [function, total, hook(BOOL.andThen)]
              | Bool "xorBool" Bool     [function, total, hook(BOOL.xor)]
              | Bool "orBool" Bool      [function, total, hook(BOOL.or)]
              | Bool "orElseBool" Bool  [function, total, hook(BOOL.orElse)]
              | Bool "impliesBool" Bool [function, total, hook(BOOL.implies)]
              > left:
                Bool "==Bool" Bool      [function, total, hook(BOOL.eq)]
              | Bool "=/=Bool" Bool     [function, total, hook(BOOL.ne)]
```

Boolean truth in `BOOL` is distinct from matching logic satisfaction; a boolean term $B$ is embedded into matching logic via the constraint $\{B \text{ \#Equals true}\}$ (CLM-0258; SRC-0023 include/kframework/builtin/domains.md lines 1080-1083; source fact; not reproduced; high; S6).

## Strings (`STRING`)

The `STRING` module implements character string manipulation:

```k
module STRING-SYNTAX
  syntax String [hook(STRING.String)]
  syntax String ::= r"[\\\"](([^\\\"\\n\\r\\\\])|([\\\\][nrtf\\\"\\\\])|([\\\\][x][0-9a-fA-F]{2})|([\\\\][u][0-9a-fA-F]{4})|([\\\\][U][0-9a-fA-F]{8}))*[\\\"]" [token]
endmodule
```

Escape sequences include `\"`, `\\`, `\n`, `\r`, `\t`, `\f`, alongside hex escapes `\xFF` (1 byte), unicode escapes `\uFFFF` (2 bytes), and code point escapes `\UFFFFFFFF` (4 bytes) (CLM-0259; SRC-0023 include/kframework/builtin/domains.md lines 1690-1704; source fact; not reproduced; high; S6).

Core operations include:
- `S1 +String S2`: string concatenation ($O(N)$ time) (CLM-0259; SRC-0023 include/kframework/builtin/domains.md line 1725; source fact; not reproduced; high; S6).
- `lengthString(S)`: character count ($O(1)$ time) (CLM-0259; SRC-0023 include/kframework/builtin/domains.md line 1733; source fact; not reproduced; high; S6).
- `chrChar(I)` and `ordChar(S)`: integer character code conversion (CLM-0259; SRC-0023 include/kframework/builtin/domains.md lines 1742-1743; source fact; not reproduced; high; S6).
- `substrString(S, Start, End)`: half-open range $[Start, End)$ substring; partial function requiring $0 \le Start \le End \le lengthString(S)$ (CLM-0259; SRC-0023 include/kframework/builtin/domains.md lines 1747-1759; source fact; not reproduced; high; S6).
- `findString(Haystack, Needle, Offset)`: index of substring or $-1$ if absent (CLM-0259; SRC-0023 include/kframework/builtin/domains.md lines 1768-1770; source fact; not reproduced; high; S6).
- `String2Int(S)`, `Int2String(I)`: radix-10 integer parsing and formatting (CLM-0259; SRC-0023 include/kframework/builtin/domains.md lines 1834-1835; source fact; not reproduced; high; S6).
- `String2Base(S, Base)`, `Base2String(I, Base)`: radix conversions for bases $2 \le Base \le 36$ (CLM-0259; SRC-0023 include/kframework/builtin/domains.md lines 1825-1838; source fact; not reproduced; high; S6).

## Program Identifiers (`ID`)

The `ID` module provides standard programming language identifiers:

```k
module ID-SYNTAX
  syntax Id [token]
endmodule

module ID-COMMON
  syntax String ::= Id2String ( Id ) [function, total, hook(STRING.token2string)]
  syntax Id ::= String2Id (String)   [function, total, hook(STRING.string2token)]
  syntax Id ::= freshId(Int)         [freshGenerator, function, total, private]
  rule freshId(I:Int) => String2Id("_" +String Int2String(I))
endmodule
```

`freshId(I)` formats unique identifiers by prefixing the integer counter with an underscore (CLM-0260; SRC-0023 include/kframework/builtin/domains.md lines 2297-2300; source fact; not reproduced; high; S6).

## Maps (`MAP`)

The `MAP` module implements immutable, associative-commutative finite maps mapping `KItem` to `KItem` (CLM-0261; SRC-0023 include/kframework/builtin/domains.md #maps; source fact; not reproduced; high; S6):

```k
syntax Map [hook(MAP.Map)]
syntax Map ::= Map Map             [left, function, hook(MAP.concat), assoc, comm, unit(.Map), element(_|->_)]
             | ".Map"              [function, total, hook(MAP.unit)]
syntax Map ::= KItem "|->" KItem   [function, total, hook(MAP.element)]
```

### Operations and Complexity

- Lookup: `M[K]` ($O(\log N)$ time, effectively constant). If key $K$ is not present, evaluating `M[K]` yields `#False` / `#Bottom` (CLM-0261; SRC-0023 include/kframework/builtin/domains.md lines 265-272; source fact; not reproduced; high; S6).
- Total lookup: `M[K] orDefault DefaultValue` returns `DefaultValue` if $K \notin \text{keys}(M)$ (CLM-0261; SRC-0023 include/kframework/builtin/domains.md lines 276-282; source fact; not reproduced; high; S6).
- Update: `M[K <- V]` ($O(\log N)$ time) inserts or updates the binding (CLM-0261; SRC-0023 include/kframework/builtin/domains.md lines 286-291; source fact; not reproduced; high; S6).
- Deletion: `M[K <- undef]` ($O(\log N)$ time) removes the key $K$ (CLM-0261; SRC-0023 include/kframework/builtin/domains.md lines 294-300; source fact; not reproduced; high; S6).
- Difference: `M1 -Map M2` removes pairs present in both maps with identical values (CLM-0262; SRC-0023 include/kframework/builtin/domains.md lines 304-312; source fact; not reproduced; high; S6).
- Bulk update: `updateMap(M1, M2)` overwrites bindings in `M1` with all pairs from `M2` ($O(N \log M)$ time) (CLM-0262; SRC-0023 include/kframework/builtin/domains.md lines 316-325; source fact; not reproduced; high; S6).
- Bulk removal: `removeAll(M, Set)` removes a set of keys ($O(N \log M)$ time) (CLM-0262; SRC-0023 include/kframework/builtin/domains.md lines 328-334; source fact; not reproduced; high; S6).
- Key query: `keys(M)` returns keys as a `Set`; `keys_list(M)` returns keys as a `List`; `K in_keys(M)` performs $O(1)$ membership testing (CLM-0262; SRC-0023 include/kframework/builtin/domains.md lines 336-359; source fact; not reproduced; high; S6).
- Size and choice: `size(M)` returns binding count in $O(1)$ time; `choice(M)` deterministically returns an arbitrary key (CLM-0262; SRC-0023 include/kframework/builtin/domains.md lines 370-394; source fact; not reproduced; high; S6).

## Sets (`SET`)

The `SET` module implements immutable, associative-commutative mathematical sets of `KItem`:

```k
syntax Set [hook(SET.Set)]
syntax Set ::= Set Set         [left, function, hook(SET.concat), assoc, comm, unit(.Set), idem, element(SetItem)]
             | ".Set"          [function, total, hook(SET.unit)]
syntax Set ::= SetItem(KItem)  [function, total, hook(SET.element)]
```

Concatenation of two sets with overlapping elements is nilpotent and yields `#False` in symbolic execution (CLM-0263; SRC-0023 include/kframework/builtin/domains.md lines 705-710; source fact; not reproduced; high; S6). For safe addition of elements that may already be present, the union operator `|Set` must be used (CLM-0263; SRC-0023 include/kframework/builtin/domains.md lines 740-750; source fact; not reproduced; high; S6).

Operations comprise:
- Union: `S1 |Set S2` ($O(N \log M)$ time) (CLM-0263; SRC-0023 include/kframework/builtin/domains.md line 748; source fact; not reproduced; high; S6).
- Intersection: `intersectSet(S1, S2)` (CLM-0263; SRC-0023 include/kframework/builtin/domains.md line 759; source fact; not reproduced; high; S6).
- Relative complement: `S1 -Set S2` (CLM-0263; SRC-0023 include/kframework/builtin/domains.md line 769; source fact; not reproduced; high; S6).
- Membership: `Item in Set` ($O(1)$ time) (CLM-0263; SRC-0023 include/kframework/builtin/domains.md line 777; source fact; not reproduced; high; S6).
- Inclusion: `S1 <=Set S2` tests subset inclusion ($O(N)$ time) (CLM-0263; SRC-0023 include/kframework/builtin/domains.md line 786; source fact; not reproduced; high; S6).
- Size and choice: `size(Set)` in $O(1)$ time; `choice(Set)` selects an arbitrary element (CLM-0263; SRC-0023 include/kframework/builtin/domains.md lines 794-805; source fact; not reproduced; high; S6).

## Lists (`LIST`)

The `LIST` module implements immutable, associative sequences of `KItem`:

```k
syntax List [hook(LIST.List)]
syntax List ::= List List       [left, function, total, hook(LIST.concat), assoc, unit(.List), element(ListItem)]
             | ".List"          [function, total, hook(LIST.unit)]
syntax List ::= ListItem(KItem) [function, total, hook(LIST.element)]
```

Lists are backed by relaxed radix balanced (RRB) trees, providing efficient concatenation, prepending, appending, and index updates (CLM-0264; SRC-0023 include/kframework/builtin/domains.md lines 920-923; source fact; not reproduced; high; S6).

List operations comprise:
- Indexing: `L[I:Int]` supports zero-based indexing from the front and negative indexing from the back (e.g. `L[-1]` is the final element) ($O(\log N)$ time) (CLM-0264; SRC-0023 include/kframework/builtin/domains.md lines 960-968; source fact; not reproduced; high; S6).
- Machine integer indexing: `L[M:MInt{Width}]` is supported for 64-bit and 256-bit unsigned widths (CLM-0264; SRC-0023 include/kframework/builtin/domains.md lines 963-970; source fact; not reproduced; high; S6).
- Element update: `L[Index <- Value]` ($O(\log N)$ time) (CLM-0264; SRC-0023 include/kframework/builtin/domains.md line 978; source fact; not reproduced; high; S6).
- Prepend: `pushList(KItem, List)` (CLM-0264; SRC-0023 include/kframework/builtin/domains.md line 954; source fact; not reproduced; high; S6).
- Bulk creation: `makeList(Length, Value)` ($O(N)$ time) (CLM-0264; SRC-0023 include/kframework/builtin/domains.md line 988; source fact; not reproduced; high; S6).
- Slicing: `range(List, FromFront, FromBack)` drops `FromFront` elements from the head and `FromBack` elements from the tail (CLM-0264; SRC-0023 include/kframework/builtin/domains.md line 1017; source fact; not reproduced; high; S6).
- Membership: `Item in List` ($O(N)$ linear scan) (CLM-0264; SRC-0023 include/kframework/builtin/domains.md line 1026; source fact; not reproduced; high; S6).
- Size: `size(List)` returns cardinality as `Int` or as `MInt{64}` / `MInt{256}` in $O(1)$ time (CLM-0264; SRC-0023 include/kframework/builtin/domains.md lines 1039-1041; source fact; not reproduced; high; S6).

Bidirectional conversion between `List` and `Set` is provided by `COLLECTIONS`:
```k
syntax List ::= Set2List(Set) [function, total, hook(SET.set2list)]
syntax Set  ::= List2Set(List) [function, total, hook(SET.list2set)]
```
Converting a set to a list yields an arbitrary deterministic ordering (CLM-0265; SRC-0023 include/kframework/builtin/domains.md lines 1050-1072; source fact; not reproduced; high; S6).

## Byte Sequences (`BYTES`)

The `BYTES` module provides arbitrary-length arrays of 8-bit octets:

```k
syntax Bytes [hook(BYTES.Bytes)]
syntax Bytes ::= r"b[\\\"](([ !#-\\[\\]-~])|([\\\\][tnfr\\\"\\\\])|([\\\\][x][0-9a-fA-F]{2}))*[\\\"]" [token]
syntax Bytes ::= ".Bytes" [function, total, hook(BYTES.empty)]
```

### Conversions with Endianness and Signedness

Integers and byte sequences convert explicitly using algebraic enumeration flags:

```k
syntax Endianness ::= "LE" [symbol(littleEndianBytes)]
                    | "BE" [symbol(bigEndianBytes)]
syntax Signedness ::= "Signed"   [symbol(signedBytes)]
                    | "Unsigned" [symbol(unsignedBytes)]

syntax Int ::= Bytes2Int(Bytes, Endianness, Signedness) [function, total, hook(BYTES.bytes2int)]
syntax Bytes ::= Int2Bytes(length: Int, Int, Endianness) [function, total, hook(BYTES.int2bytes)]
               | Int2Bytes(Int, Endianness, Signedness) [function, total, symbol(Int2BytesNoLen)]
```

When length is omitted, `Int2Bytes` computes the minimal byte count necessary to represent the integer magnitude without truncation (CLM-0266; SRC-0023 include/kframework/builtin/domains.md lines 2074-2085; source fact; not reproduced; high; S6).

Additional byte operations include:
- `Bytes2String(B)`, `String2Bytes(S)`: raw character copying (CLM-0267; SRC-0023 include/kframework/builtin/domains.md lines 2094-2096; source fact; not reproduced; high; S6).
- `Bytes2Hex(B)`: hexadecimal encoding (CLM-0267; SRC-0023 include/kframework/builtin/domains.md line 2095; source fact; not reproduced; high; S6).
- `B[Index <- Val]`: updates byte at offset in $O(1)$ time; `Val` must be in $[0..255]$ (CLM-0267; SRC-0023 include/kframework/builtin/domains.md lines 2101-2110; source fact; not reproduced; high; S6).
- `B[Index]`: reads byte at offset in $O(1)$ time (CLM-0267; SRC-0023 include/kframework/builtin/domains.md line 2118; source fact; not reproduced; high; S6).
- `substrBytes(B, Start, End)`: extracts sub-array; undefined if indices are out of bounds (CLM-0267; SRC-0023 include/kframework/builtin/domains.md lines 2132-2140; source fact; not reproduced; high; S6).
- `replaceAtBytes(Dest, Index, Src)`: overwrites slice of `Dest` with `Src` ($O(N)$ time) (CLM-0267; SRC-0023 include/kframework/builtin/domains.md line 2152; source fact; not reproduced; high; S6).
- `padRightBytes(B, Len, Val)` and `padLeftBytes(B, Len, Val)`: pads byte buffer to length (CLM-0267; SRC-0023 include/kframework/builtin/domains.md lines 2183-2187; source fact; not reproduced; high; S6).

## Machine Integers (`MINT`)

The `MINT` module defines width-parametric machine integers in two's complement form:

```k
syntax {Width} MInt{Width} [hook(MINT.MInt)]
syntax {Width} MInt{Width} ::= r"[\\+\\-]?[0-9]+[pP][0-9]+" [token, prec(2), hook(MINT.literal)]
```

Literals take the form `Value p Width` (e.g. `0p32`, `-1p64`) (CLM-0268; SRC-0023 include/kframework/builtin/domains.md lines 2880-2892; source fact; not reproduced; high; S6).

### Bitwidth, Conversion, and Signedness Interpretation

```k
syntax {Width} Int ::= bitwidthMInt(MInt{Width})   [function, total, hook(MINT.bitwidth)]
syntax {Width} Int ::= MInt2Signed(MInt{Width})     [function, total, hook(MINT.svalue)]
                     | MInt2Unsigned(MInt{Width})   [function, total, hook(MINT.uvalue)]
syntax {Width} MInt{Width} ::= Int2MInt(Int)        [function, total, hook(MINT.integer)]
```

`MInt` values do not possess intrinsic sign bits; the sign depends entirely on whether operations treat the high-order bit as signed or unsigned (CLM-0269; SRC-0023 include/kframework/builtin/domains.md lines 2911-2927; source fact; not reproduced; high; S6).

Full two's complement arithmetic and bitwise operations are provided:
- Arithmetic: `+MInt`, `-MInt`, `*MInt`, `/sMInt` (signed division), `/uMInt` (unsigned division), `%sMInt`, `%uMInt`, `^MInt` (unsigned power) (CLM-0269; SRC-0023 include/kframework/builtin/domains.md lines 3000-3011; source fact; not reproduced; high; S6).
- Bitwise: `~MInt`, `--MInt` (negation), `&MInt`, `|MInt`, `xorMInt` (CLM-0269; SRC-0023 include/kframework/builtin/domains.md lines 3000-3022; source fact; not reproduced; high; S6).
- Shifts: `<<MInt`, `>>aMInt` (arithmetic right shift), `>>lMInt` (logical right shift) (CLM-0269; SRC-0023 include/kframework/builtin/domains.md lines 3013-3015; source fact; not reproduced; high; S6).
- Comparisons: `<sMInt`, `<=sMInt`, `>sMInt`, `>=sMInt` versus `<uMInt`, `<=uMInt`, `>uMInt`, `>=uMInt` (CLM-0269; SRC-0023 include/kframework/builtin/domains.md lines 3032-3042; source fact; not reproduced; high; S6).
- Width resizing: `roundMInt` (truncates high bits or zero-extends) and `signExtendMInt` (sign-extends) (CLM-0269; SRC-0023 include/kframework/builtin/domains.md lines 3066-3068; source fact; not reproduced; high; S6).

## Auxiliary Builtin Domains in `domains.md`

### Floating Point (`FLOAT`)

The `FLOAT` module implements arbitrary-precision floating point numbers based on IEEE 754 (CLM-0270; SRC-0023 include/kframework/builtin/domains.md #ieee-754-floating-point-numbers; source fact; not reproduced; high; S6). Literals use suffixes `f` (binary32), `d` (binary64), or `pNxM` ($N$ precision bits, $M$ exponent bits). Mathematical transcendental functions (`sinFloat`, `cosFloat`, `expFloat`, `logFloat`) and rounding operators are backed by GNU MPFR (CLM-0270; SRC-0023 include/kframework/builtin/domains.md lines 1607-1627; source fact; not reproduced; high; S6).

### Range Maps (`RANGEMAP`)

The `RANGEMAP` module implements immutable range maps mapping intervals `[Low, High)` of `Int` to values:

```k
syntax Range ::= "[" KItem "," KItem ")" [symbol(RangeMap:Range)]
syntax RangeMap [hook(RANGEMAP.RangeMap)]
syntax RangeMap ::= Range "r|->" KItem   [function, hook(RANGEMAP.elementRng)]
```

Contiguous or overlapping intervals mapping to identical values are merged automatically (CLM-0271; SRC-0023 include/kframework/builtin/domains.md lines 498-501; source fact; not reproduced; high; S6). Concatenating range maps with overlapping keys throws an exception during concrete execution (CLM-0271; SRC-0023 include/kframework/builtin/domains.md lines 503-505; source fact; not reproduced; high; S6). Range maps are currently supported only on the LLVM backend (CLM-0271; SRC-0023 include/kframework/builtin/domains.md lines 474-475; source fact; not reproduced; high; S6).

### Arrays (`ARRAY`)

The `ARRAY` module implements fixed-size, contiguous maps from `Int` to `KItem` backed by `List` (CLM-0272; SRC-0023 include/kframework/builtin/domains.md #arrays; source fact; not reproduced; high; S6). Operations include `makeArray(Length, Default)`, `Arr[Index]`, `Arr[Index <- Value]`, `updateArray(Arr, Index, List)`, and `fillArray(Arr, Index, Len, Val)`, operating with effectively constant $O(\log N)$ or linear time complexity (CLM-0272; SRC-0023 include/kframework/builtin/domains.md lines 86-134; source fact; not reproduced; high; S6).

### String Buffers (`STRING-BUFFER`)

`STRING-BUFFER` provides mutable string accumulators for concrete execution, avoiding $O(N^2)$ concatenation costs via `+String` and converting to immutable strings via `StringBuffer2String` (CLM-0273; SRC-0023 include/kframework/builtin/domains.md #string-buffers; source fact; not reproduced; high; S6).

## Core AST Representation Modules (`kast.md` and `prelude.md`)

`kast.md` defines the primitive syntactic categories of the K abstract machine:
- `K`: the top sort of all computation sequences (CLM-0274; SRC-0023 include/kframework/builtin/kast.md #basic-k-sorts; source fact; not reproduced; high; S6).
- `KItem`: the sort of individual computation steps or values. Every sort is a subsort of `KItem` (CLM-0274; SRC-0023 include/kframework/builtin/kast.md lines 38-42; source fact; not reproduced; high; S6).
- `KBott`: the bottom sort, used internally for AST typing (CLM-0274; SRC-0023 include/kframework/builtin/kast.md lines 68-71; source fact; not reproduced; high; S6).
- `.K`: the empty computation sequence (unit of `~>`) (CLM-0274; SRC-0023 include/kframework/builtin/kast.md line 98; source fact; not reproduced; high; S6).
- `~>`: the associative computation sequencing operator (left-associative) (CLM-0274; SRC-0023 include/kframework/builtin/kast.md line 100; source fact; not reproduced; high; S6).
- `#configuration`: meta-function returning the full configuration term as a `K` item (CLM-0274; SRC-0023 include/kframework/builtin/domains.md line 2380; source fact; not reproduced; high; S6).

## Auxiliary Standard Library Modules

### Capture-Aware Substitution (`substitution.md`)

`substitution.md` defines capture-avoiding substitution for lambda calculi:
- `KVar`: hooked identifier sort representing variables subject to alpha-conversion (CLM-0275; SRC-0023 include/kframework/builtin/substitution.md #the-kvar-sort; source fact; not reproduced; high; S6).
- `[binder]`: production attribute marking variable-binding constructs where the first nonterminal is of sort `KVar` and the last nonterminal is the scope body (CLM-0275; SRC-0023 include/kframework/builtin/substitution.md #the-binder-attribute; source fact; not reproduced; high; S6).
- Operators: `Term[Val / Var]` and `Term[Map]` execute capture-avoiding substitution backed by de Bruijn indices (CLM-0275; SRC-0023 include/kframework/builtin/substitution.md lines 95-97; source fact; not reproduced; high; S6).

### Unification (`unification.k`)

`unification.k` exposes backend unification hooks:
- `#unifiable(K1, K2)`: returns boolean indicating syntactic unifiability (CLM-0276; SRC-0023 include/kframework/builtin/unification.k line 18; source fact; not reproduced; high; S6).
- `#variables(K)`: extracts free variables as a `Set` (CLM-0276; SRC-0023 include/kframework/builtin/unification.k line 8; source fact; not reproduced; high; S6).
- `#renameVariables(K)`: alpha-renames variables within a term (CLM-0276; SRC-0023 include/kframework/builtin/unification.k line 6; source fact; not reproduced; high; S6).

### Rational Numbers (`rat.md`)

`rat.md` defines arbitrary-precision rational arithmetic of sort `Rat`:
- `Int` is a subsort of `Rat` (CLM-0276; SRC-0023 include/kframework/builtin/rat.md line 21; source fact; not reproduced; high; S6).
- Non-integer rationals are normalized to canonical irreducible pairs `I /Rat J` such that $J \ge 2$ and $\gcd(I, J) = 1$ (CLM-0276; SRC-0023 include/kframework/builtin/rat.md lines 89-94; source fact; not reproduced; high; S6).
- Standard field operations `+Rat`, `-Rat`, `*Rat`, `/Rat`, and power `^Rat` are supported (CLM-0276; SRC-0023 include/kframework/builtin/rat.md lines 34-42; source fact; not reproduced; high; S6).

### JSON Parsing (`json.md`)

`json.md` provides bi-directional serialization between structured JSON terms and K strings via hooks `String2JSON` and `JSON2String` (CLM-0276; SRC-0023 include/kframework/builtin/json.md lines 40-47; source fact; not reproduced; high; S6).

### Foreign Function Interface (`ffi.md`)

`ffi.md` provides native C ABI calling capabilities based on `libffi`:
- `#ffiCall(Addr, Args, ArgTypes, RetType)`: invokes native functions and returns result as `Bytes` (CLM-0276; SRC-0023 include/kframework/builtin/ffi.md line 85; source fact; not reproduced; high; S6).
- Direct memory management: `#alloc(Key, Size, Align)`, `#free(Key)`, `#nativeRead`, and `#nativeWrite` allow allocating fixed-address buffers for native C interactions (CLM-0276; SRC-0023 include/kframework/builtin/ffi.md lines 146-207; source fact; not reproduced; high; S6).

### Execution Timers (`timer.md`)

`timer.md` exports `timerStart()` and `timerStop()` hooks for profiling execution durations within semantics (CLM-0276; SRC-0023 include/kframework/builtin/timer.md lines 7-10; source fact; not reproduced; high; S6).

## Modelling Cryptographic Primitives in K (Inference)

Defining an executable formal semantics of the ZKIR intermediate representation in K requires establishing representations for prime fields, fixed-width words, byte sequences, elliptic curve group elements, and circuit wire vectors. The builtin domains described above dictate specific design patterns.

### Prime Field Arithmetic

It is an inference that prime-field elements $\mathbb{F}_p$ should be modelled as values of sort `Int` constrained to the range $[0, p-1]$, rather than declaring an uninterpreted domain (CLM-0277; inference based on CLM-0256; SRC-0023 include/kframework/builtin/domains.md #integers). Because K's `Int` is backed by GMP arbitrary-precision integers, it natively accommodates cryptographic prime moduli exceeding 250 bits, such as the Midnight BLS12-381 scalar field or Pasta curve fields (CLM-0277; inference based on CLM-0256; SRC-0023 include/kframework/builtin/domains.md #integers).

Field operations can be specified using total helper functions with Euclidean modulo reduction:
```k
syntax Int ::= addMod(Int, Int, Int) [function, total]
             | mulMod(Int, Int, Int) [function, total]
             | invMod(Int, Int)      [function]

rule addMod(A, B, P) => (A +Int B) modInt P
rule mulMod(A, B, P) => (A *Int B) modInt P
rule invMod(A, P)    => A ^%Int (P -Int 2) P requires A =/=Int 0
```
Modular inversion exploits Fermat's Little Theorem via K's builtin `^%Int` modular exponentiation operator, ensuring logarithmic evaluation time without requiring an external Euclidean algorithm implementation (CLM-0277; inference based on CLM-0256; SRC-0023 include/kframework/builtin/domains.md line 1251).

### Fixed-Width Integers and Bit Vectors

It is an inference that fixed-width words (such as 8, 16, 32, and 64-bit integer values in ZKIR circuits) should be modelled using `MInt{Width}` when bitwise slicing, signed/unsigned shifts, and two's complement overflow are primary (CLM-0278; inference based on CLM-0268, CLM-0269; SRC-0023 include/kframework/builtin/domains.md #machine-integers). Alternatively, if the circuit representation treats integer values as bit-constrained field elements, authoring operations over sort `Int` with explicit range constraints `0 <=Int X andBool X <Int (1 <<Int Width)` preserves algebraic transparency across Z3 SMT solver queries (CLM-0278; inference based on CLM-0256, CLM-0268; SRC-0023 docs/user_manual.md #smt-translation).

### Byte Strings and Hashes

It is an inference that cryptographic byte strings (such as transaction digests, challenge seeds, and witness commitments) must be represented using sort `Bytes` (CLM-0279; inference based on CLM-0266, CLM-0267; SRC-0023 include/kframework/builtin/domains.md #byte-arrays). Cryptographic endianness transformations are expressed directly through `Int2Bytes(Len, Val, BE)` and `Bytes2Int(B, BE, Unsigned)` (CLM-0279; inference based on CLM-0266; SRC-0023 include/kframework/builtin/domains.md lines 2082-2085).

### Elliptic Curve Points and Straight-Line Vectors

It is an inference that elliptic curve points should be represented as constructed terms over field coordinate pairs:
```k
syntax Point ::= point(x: Int, y: Int) | "infinity"
```
In straight-line ZKIR circuits, execution environments mapping wire identifiers to assigned field elements are modelled using `Map` (`WireId |-> FieldElement`), while ordered instruction vectors are held in `List` (CLM-0279; inference based on CLM-0261, CLM-0264; SRC-0023 include/kframework/builtin/domains.md #maps, #lists).

For detailed user manual attribute rules, see [k-user-manual](k-user-manual.md). For compiler backend execution pipelines, see [k-backends-and-tools](k-backends-and-tools.md). For ZKIR-specific instruction sets and circuit VM rules, see [zkir-instruction-set](../zkir/zkir-instruction-set.md) and [zkir-vm-semantics](../zkir/zkir-vm-semantics.md).

* [Home](index.shtml)
* [About](about.shtml)
* [News](news.shtml)
* [Standard](standard.shtml)
* [Benchmarks](benchmarks.shtml)
* [Software](software.shtml)
* [Credits](credits.shtml)

[SMT-LIB  The Satisfiability Modulo Theories Library](index.shtml)
==================================================================

AUFLIA
------

```
(logic AUFLIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-30"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "

 :theories (Ints ArraysEx)

 :language 
 "Closed formulas built over arbitrary expansions of the Ints and ArraysEx
  signatures with free sort and function symbols, but with the following 
  restrictions:
  - all terms of sort Int are linear, that is, have no occurrences of the
    function symbols *, **, /, div, mod, and abs, except as specified in the 
    :extensions attributes;
  - all array terms have sort (Array Int Int).
 "

 :extensions
 "As in the logic QF_AUFLIA."

:notes
 "This logic properly extends the logic QF_AUFLIA by allowing quantifiers."
)
```

([raw file](Logics/AUFLIA.smt2))
  
  

AUFLIRA
-------

```
(logic AUFLIRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli and Clark Barrett"
 :date "2010-05-05"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2011-06-03 Replaced "(* c x), or (* x c)" with "c, (* c x), or (* x c)" 
             in :extensions.
             (The missing case was had been left out unintentionally.)
 "

 :theories (Reals_Ints ArraysEx)

:language
 "Closed formulas built over arbitrary expansions of the Reals_Ints and
  ArraysEx signatures with free sort and function symbols, but with the
  following restrictions:
  - all terms of sort Int are linear, that is, have no occurrences of the
    function symbols *, /, div, mod, and abs, except as specified in the 
    :extensions attributes;
  - all terms of sort Real are linear, that is, have no occurrences of the
    function symbols * and /, except as specified in the 
    :extensions attribute;
  - all array terms have sort 
    (Array Int Real) or 
    (Array Int (Array Int Real)).
 "

:extensions
 "For every operator op with declaration (op Real Real s) for some sort s,
  and every term t1, t2 of sort Int and t of sort Real, the expression 
  - (op t1 t) is syntactic sugar for (op (to_real t1) t)
  - (op t t1) is syntactic sugar for (op t (to_real t1))
  - (/ t1 t2) is syntactic sugar for (/ (to_real t1) (to_real t2))
 "

 :extensions
 "Real or Int terms with _concrete_ coefficients are also allowed, that is,
  terms of the form c, (* c x), or (* x c) where
  x is a free constant of sort Int or Real and 
  c is an integer or rational coefficient, respectively. 
  - An integer coefficient is a term of the form m or (- m) for some
    numeral m.
  - A rational coefficient is a term of the form d, (- d) or (/ c n) 
    for some decimal d, integer coefficient c and numeral n other than 0.
 "
)
```

([raw file](Logics/AUFLIRA.smt2))
  
  

AUFNIRA
-------

```
(logic AUFNIRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli and Clark Barrett"
 :date "2010-05-12"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2012-09-26 Clarified in :notes in what way AUFNIRA extendes AUFLIRA.
 "

 :theories (Reals_Ints ArraysEx)

 :language
 "Closed formulas built over arbitrary expansions of the Reals_Ints and
  ArraysEx signatures with free sort and function symbols.
 "

 :extensions
 "For every operator op with declaration (op Real Real s) for some sort s,
  and every term t1, t2 of sort Int and t of sort Real, the expression 
  - (op t1 t) is syntactic sugar for (op (to_real t1) t)
  - (op t t1) is syntactic sugar for (op t (to_real t1))
  - (/ t1 t2) is syntactic sugar for (/ (to_real t1) (to_real t2))
 "

 :notes
 "This logic properly extends the logic AUFLIRA by allowing 
  - non-linear (integer/real) operators such as  *, /, div, mod, and abs, and
  - allowing terms with an arbitrary array sort (as opposed to just
    (Array Int Real) and (Array Int (Array Int Real)) ).
 "
)
```

([raw file](Logics/AUFNIRA.smt2))
  
  

LIA
---

```
(logic LIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2016-02-07"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
 "
 :theories (Ints)

 :language 
 "Closed formulas built over an arbitrary expansion of the
  Ints signature with free constant symbols, but whose terms of sort Int 
  are all linear, that is, have no occurrences of the function symbols
  *, **, /, div, mod, and abs, except as specified the :extensions attribute.
 "

 :extensions
 "Terms with _concrete_ coefficients are also allowed, that is, terms
  of the form c, (* c x), or (* x c)  where x is a free constant and 
  c is a term of the form n or (- n) for some numeral n.
 "
)
```

([raw file](Logics/LIA.smt2))
  
  

LRA
---

```
(logic LRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-05-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2011-06-03 Replaced ''(* c x), or (* x c)'' with ''c, (* c x), or (* x c)'' 
             in :extensions.
             (The missing case was had been left out unintentionally.)
 "

 :theories (Reals)

 :language 
 "Closed formulas built over arbitrary expansions of the Reals signature
  with free constant symbols, but containing only linear atoms, that is, 
  atoms with no occurrences of the function symbols * and /, except as 
  specified the :extensions attribute.
 "

 :extensions
 "Terms with _concrete_ coefficients are also allowed, that is, terms
  of the form c, (* c x), or (* x c)  where x is a free constant and 
  c is an integer or rational coefficient. 
  - An integer coefficient is a term of the form m or (- m) for some
    numeral m.
  - A rational coefficient is a term of the form d, (- d) or (/ c n) 
    for some decimal d, integer coefficient c and numeral n other than 0.
 "

:notes
 "This logic properly extends the logic QF_LRA by allowing quantifiers."
)
```

([raw file](Logics/LRA.smt2))
  
  

QF\_ABV
-------

```
(logic QF_ABV

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli and Clark Barrett"
 :date "2010-07-05"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2013-06-24 Changed references to Fixed_Size_Bitvectors to FixedSizeBitVectors.
 "

 :theories (FixedSizeBitVectors ArraysEx)

 :language 
 "Closed quantifier-free formulas built over the FixedSizeBitVectors and
  ArraysEx signatures, with the restriction that all array terms have sort of
  the form (Array (_ BitVec i) (_ BitVec j)) for some i, j > 0.
 "

 :extensions "As in the logic QF_BV."
)
```

([raw file](Logics/QF_ABV.smt2))
  
  

QF\_AUFBV
---------

```
(logic QF_AUFBV

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli and Clark Barrett"
 :date "2010-05-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2013-06-24 Changed references to Fixed_Size_Bitvectors to FixedSizeBitVectors.
 "

 :theories (FixedSizeBitVectors ArraysEx)

 :language 
 "Closed quantifier-free formulas built over an arbitrary expansion of the
  FixedSizeBitVectors and ArraysEx signatures with free sort and function
  symbols, but with the restriction that all array terms have sort of the 
  form (Array (_ BitVec i) (_ BitVec j)) for some i, j > 0.
 "

 :extensions "As in the logic QF_BV."
)
```

([raw file](Logics/QF_AUFBV.smt2))
  
  

QF\_AUFLIA
----------

```
(logic QF_AUFLIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-30"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2011-06-03 Allowed x to be more than a free constants in terms of the form 
             (* c x) or (*x c)   in :extensions.
 "

 :theories (Ints ArraysEx)

 :language 
 "Closed quantifier-free formulas built over arbitrary expansions of the
  Ints and ArraysEx signatures with free sort and function symbols, but
  with the following restrictions:
  - all terms of sort Int are linear, that is, have no occurrences of the
    function symbols *, **, /, div, mod, and abs, except as specified in the 
    :extensions attributes;
  - all array terms have sort (Array Int Int).
 "

 :extensions
 "Terms with _concrete_ integer coefficients are also allowed, that is, terms
  of the form c, (* c x), or (* x c)  where 
  - x is a free constant or a term with top symbol not in Ints, and 
  - c is a term of the form n or (- n) for some numeral n.
 ")
```

([raw file](Logics/QF_AUFLIA.smt2))
  
  

QF\_AX
------

```
(logic QF_AX

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-30"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "

 :theories (ArraysEx)
 
 :language 
 "Closed quantifier-free formulas built over an arbitrary expansion of
  the ArraysEx signature with free sort and constant symbols.
 "

 :notes
 "Formulas can contain variables as long as they are bound by a let binder."
)
```

([raw file](Logics/QF_AX.smt2))
  
  

QF\_BV
------

```
(logic QF_BV

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Clark Barrett, Pascal Fontaine, Silvio Ranise, and Cesare Tinelli"
 :date "2010-05-02"
 :last-updated "2025-02-25"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2025-02-25 Renamed and updated conversion operators to/from integers.
  2024-07-21 Updated to Version 2.7.
  2024-07-15 Added conversion operators between bitvectors and integers.
  2024-07-14 Minor disambiguation.
  2023-11-29 Added bvnego bvuaddo bvsaddo bvumulo bvsmulo bvusubo bvssubo bvsdivo
  2020-05-20 bvxnor is no longer marked as left associative, as that is
             inconsistent with its meaning as the negation of bvxor.
  2017-06-13 Added that bvxor and bvxnor are left associative
  2017-05-03 Updated to Version 2.6.  Division and remainder operations are no
             longer undefiend when the second operand is 0.  See
             the FixedSizeBitVectors theory definition for details.
  2015-04-25 Updated to Version 2.5.
  2013-06-24 Changed references to Fixed_Size_Bitvectors to FixedSizeBitVectors.
  2011-06-15 Fixed bug in definition of bvsmod.  Previously, it gave an incorrect
             answer when the divisor is negative and goes into the dividend evenly.
 "

:theories (FixedSizeBitVectors)

:language
 "Closed quantifier-free formulas built over an arbitrary expansion of the
  FixedSizeBitVectors signature with free constant symbols over the sorts
  (_ BitVec m) for 0 < m.  Formulas in ite terms must satisfy the same
  restriction as well, with the exception that they need not be closed 
  (because they may be in the scope of a let binder).
 "

:notes
 "For quick reference, the following is a brief and informal summary of the
  legal symbols in this logic and their meaning (formal definitions are found
  either in the FixedSizeBitvectors theory, or in the extensions below).

  Defined in theory FixedSizeBitvectors:

    Bitvector constants:

      - #bX where X is a binary numeral of length m defines the
        bitvector constant with value X and size m.
      - #xX where X is a hexadecimal numeral of length m defines the
        bitvector constant with value X and size 4*m.
 
   Functions:
 
    (concat (_ BitVec i) (_ BitVec j) (_ BitVec m))
      - concatenation of bitvectors of size i and j to get a new bitvector of
        size m, where m = i + j
    ((_ extract i j) (_ BitVec m) (_ BitVec n))
      - extraction of bits i down to j from a bitvector of size m to yield a
        new bitvector of size n, where n = i - j + 1
    (bvnot (_ BitVec m) (_ BitVec m))
      - bitwise negation
    (bvand (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - bitwise and
    (bvor (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - bitwise or
    (bvneg (_ BitVec m) (_ BitVec m))
      - 2's complement unary minus
    (bvadd (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - addition modulo 2^m
    (bvmul (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - multiplication modulo 2^m
    (bvudiv (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - unsigned division, truncating towards 0
    (bvurem (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - unsigned remainder from truncating division
    (bvshl (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - shift left (equivalent to multiplication by 2^x where x is the value of
        the second argument)
    (bvlshr (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - logical shift right (equivalent to unsigned division by 2^x where x is
        the value of the second argument)
    (bvult (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for unsigned less-than
    (bvnego (_ BitVec m) Bool)
      - overflow predicate for 2's complement unary minus
    (bvuaddo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for unsigned addition modulo 2^m
    (bvsaddo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for signed addition on m-bit 2's complement
    (bvumulo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for unsigned multiplication modulo 2^m
    (bvsmulo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for signed multiplication on m-bit 2's complement
    (ubv_to_int (_ BitVec m) Int)
      - convert bitvector, interpreted as unsigned, to its integer value
    (sbv_to_int (_ BitVec m) Int)
      - convert a bitvector, interpreted as signed, to its integer value
    ((_ int_to_bv m) (Int) (_ BitVec m))
      - convert an Integer to a bitvector

  Defined below:

    Functions:

    (bvnand (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - bitwise nand (negation of and)
    (bvnor (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - bitwise nor (negation of or)
    (bvxor (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - bitwise exclusive or
    (bvxnor (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - bitwise equivalence (equivalently, negation of bitwise exclusive or)
    (bvcomp (_ BitVec m) (_ BitVec m) (_ BitVec 1))
      - bit comparator: equals #b1 iff all bits are equal
    (bvsub (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - 2's complement subtraction modulo 2^m
    (bvsdiv (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - 2's complement signed division
    (bvsrem (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - 2's complement signed remainder (sign follows dividend)
    (bvsmod (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - 2's complement signed remainder (sign follows divisor)
    (bvashr (_ BitVec m) (_ BitVec m) (_ BitVec m))
      - Arithmetic shift right, like logical shift right except that the most
        significant bits of the result always copy the most significant
        bit of the first argument.
    (bvusubo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for unsigned subtraction modulo 2^m
    (bvssubo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for signed 2's complement m-bit subtraction
    (bvsdivo (_ BitVec m) (_ BitVec m) Bool)
      - overflow predicate for 2's complement signed division

    The following symbols are parameterized by the numeral i, where i >= 1.

    ((_ repeat i) (_ BitVec m) (_ BitVec i*m))
      - ((_ repeat i) x) means concatenate i copies of x

    The following symbols are parameterized by the numeral i, where i >= 0.

    ((_ zero_extend i) (_ BitVec m) (_ BitVec m+i))
      - ((_ zero_extend i) x) means extend x with zeroes to the (unsigned)
        equivalent bitvector of size m+i
    ((_ sign_extend i) (_ BitVec m) (_ BitVec m+i))
      - ((_ sign_extend i) x) means extend x to the (signed) equivalent bitvector
        of size m+i
    ((_ rotate_left i) (_ BitVec m) (_ BitVec m))
      - ((_ rotate_left i) x) means rotate bits of x to the left i times
    ((_ rotate_right i) (_ BitVec m) (_ BitVec m))
      - ((_ rotate_right i) x) means rotate bits of x to the right i times

    (bvule (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for unsigned less than or equal
    (bvugt (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for unsigned greater than
    (bvuge (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for unsigned greater than or equal
    (bvslt (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for signed less than
    (bvsle (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for signed less than or equal
    (bvsgt (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for signed greater than
    (bvsge (_ BitVec m) (_ BitVec m) Bool)
      - binary predicate for signed greater than or equal
 "

:extensions
 "Below, let |exp| denote the integer resulting from the evaluation
  of the arithmetic expression exp.

  - Bitvector Constants:
    (_ bvX n) where X and n are numerals, i.e. (_ bv13 32),
    abbreviates the term #bY of sort (_ BitVec n) such that

    [[#bY]] = nat2bv[n](X) for X=0, ..., 2^n - 1.

    See the specification of the theory's semantics for a definition
    of the functions [[_]] and nat2bv.  Note that this convention implicitly
    considers the numeral X as a number written in decimal.  It is
    an error to write (_ bvX n) with X >= 2^n.

  - Bitwise operators

    For all terms s,t of sort (_ BitVec m), where 0 < m,

    (bvnand s t) abbreviates (bvnot (bvand s t))
    (bvnor s t) abbreviates (bvnot (bvor s t))
    (bvxor s t) abbreviates (bvor (bvand s (bvnot t)) (bvand (bvnot s) t))
    (bvxnor s t) abbreviates (bvor (bvand s t) (bvand (bvnot s) (bvnot t)))
    (bvcomp s t) abbreviates (bvxnor s t) if m = 1, and
       (bvand (bvxnor ((_ extract |m-1| |m-1|) s) ((_ extract |m-1| |m-1|) t))
              (bvcomp ((_ extract |m-2| 0) s) ((_ extract |m-2| 0) t))) otherwise.

    Additionally, bvxor is left associative, so:

    (bvxor s_1 s_2 ... s_n) abbreviates (bvxor (bvxor s_1 s_2 ...) s_n), and

  - Arithmetic operators

    For all terms s,t of sort (_ BitVec m), where 0 < m,

    (bvsub s t) abbreviates (bvadd s (bvneg t))
    (bvsdiv s t) abbreviates
      (let ((?msb_s ((_ extract |m-1| |m-1|) s))
            (?msb_t ((_ extract |m-1| |m-1|) t)))
        (ite (and (= ?msb_s #b0) (= ?msb_t #b0))
             (bvudiv s t)
        (ite (and (= ?msb_s #b1) (= ?msb_t #b0))
             (bvneg (bvudiv (bvneg s) t))
        (ite (and (= ?msb_s #b0) (= ?msb_t #b1))
             (bvneg (bvudiv s (bvneg t)))
             (bvudiv (bvneg s) (bvneg t))))))
    (bvsrem s t) abbreviates
      (let ((?msb_s ((_ extract |m-1| |m-1|) s))
            (?msb_t ((_ extract |m-1| |m-1|) t)))
        (ite (and (= ?msb_s #b0) (= ?msb_t #b0))
             (bvurem s t)
        (ite (and (= ?msb_s #b1) (= ?msb_t #b0))
             (bvneg (bvurem (bvneg s) t))
        (ite (and (= ?msb_s #b0) (= ?msb_t #b1))
             (bvurem s (bvneg t)))
             (bvneg (bvurem (bvneg s) (bvneg t))))))
    (bvsmod s t) abbreviates
      (let ((?msb_s ((_ extract |m-1| |m-1|) s))
            (?msb_t ((_ extract |m-1| |m-1|) t)))
        (let ((abs_s (ite (= ?msb_s #b0) s (bvneg s)))
              (abs_t (ite (= ?msb_t #b0) t (bvneg t))))
          (let ((u (bvurem abs_s abs_t)))
            (ite (= u (_ bv0 m))
                 u
            (ite (and (= ?msb_s #b0) (= ?msb_t #b0))
                 u
            (ite (and (= ?msb_s #b1) (= ?msb_t #b0))
                 (bvadd (bvneg u) t)
            (ite (and (= ?msb_s #b0) (= ?msb_t #b1))
                 (bvadd u t)
                 (bvneg u))))))))
    (bvule s t) abbreviates (or (bvult s t) (= s t))
    (bvugt s t) abbreviates (bvult t s)
    (bvuge s t) abbreviates (or (bvult t s) (= s t))
    (bvslt s t) abbreviates:
      (or (and (= ((_ extract |m-1| |m-1|) s) #b1)
               (= ((_ extract |m-1| |m-1|) t) #b0))
          (and (= ((_ extract |m-1| |m-1|) s) ((_ extract |m-1| |m-1|) t))
               (bvult s t)))
    (bvsle s t) abbreviates:
      (or (and (= ((_ extract |m-1| |m-1|) s) #b1)
               (= ((_ extract |m-1| |m-1|) t) #b0))
          (and (= ((_ extract |m-1| |m-1|) s) ((_ extract |m-1| |m-1|) t))
               (bvule s t)))
    (bvsgt s t) abbreviates (bvslt t s)
    (bvsge s t) abbreviates (bvsle t s)
    (bvusubo s t) stands for (bvult s t)
    (bvssubo s t) abbreviates (ite (bvnego t) (bvsge s (_ bv0 m)) (bvsaddo s (bvneg t)))
    (bvsdivo s t) abbreviates (and (bvnego s) (= t (bvnot (_ bv0 m))))

  - Other operations

    For all numerals i > 0, j > 1 and 0 < m, and all terms s and t of
    sort (_ BitVec m),

    (bvashr s t) abbreviates 
      (ite (= ((_ extract |m-1| |m-1|) s) #b0)
           (bvlshr s t)
           (bvnot (bvlshr (bvnot s) t)))

    ((_ repeat 1) t) stands for t
    ((_ repeat j) t) abbreviates (concat t ((_ repeat |j-1|) t))

    ((_ zero_extend 0) t) stands for t
    ((_ zero_extend i) t) abbreviates (concat ((_ repeat i) #b0) t)

    ((_ sign_extend 0) t) stands for t
    ((_ sign_extend i) t) abbreviates
      (concat ((_ repeat i) ((_ extract |m-1| |m-1|) t)) t)

    ((_ rotate_left 0) t) stands for t
    ((_ rotate_left i) t) abbreviates t if m = 1, and
      ((_ rotate_left |i-1|)
        (concat ((_ extract |m-2| 0) t) ((_ extract |m-1| |m-1|) t))
      otherwise

    ((_ rotate_right 0) t) stands for t
    ((_ rotate_right i) t) abbreviates t if m = 1, and
      ((_ rotate_right |i-1|)
        (concat ((_ extract 0 0) t) ((_ extract |m-1| 1) t)))
      otherwise
 "
)
```

([raw file](Logics/QF_BV.smt2))
  
  

QF\_EIA
-------

```
(logic QF_EIA

 :smt-lib-version 2.7
 :smt-lib-release "2026-03-13"
 :written-by "Cesare Tinelli and Florian Frohn"
 :date "2016-03-18"
 :last-updated "2026-03-18"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2026-03-18 Creation of QF_EIA on the basis of QF_NIA
 "

 :theories ( Ints )

 :language
 "Closed quantifier-free formulas built over an arbitrary expansion of the
  Ints signature with free constant symbols.
 "
)
```

([raw file](Logics/QF_EIA.smt2))
  
  

QF\_IDL
-------

```
(logic QF_IDL

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-30"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "

 :theories ( Ints )

 :language
 "Closed quantifier-free formulas with atoms of the form:
  - q
  - (op (- x y) n),
  - (op (- x y) (- n)), or
  - (op x y)
  where
    - q is a variable or free constant symbol of sort Bool,
    - op is <, <=, >, >=, =, or distinct,
    - x, y are free constant symbols of sort Int, 
    - n is a numeral. 
 "
)
```

([raw file](Logics/QF_IDL.smt2))
  
  

QF\_LIA
-------

```
(logic QF_LIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-30"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2021-01-21 Clarified wording around what terms are included in extensions.
  2015-04-25 Updated to Version 2.5.
 "

 :theories (Ints)

 :language 
 "Closed quantifier-free formulas built over an arbitrary expansion of the
  Ints signature with free constant symbols, but whose terms of sort Int 
  are all linear, that is, have no occurrences of the function symbols
  **, /, div, mod, and abs, and no occurrences of the function symbol *,
  except as specified in the :extensions attribute.
 "

 :extensions
 "Terms containing * with _concrete_ coefficients are also allowed, that
  is, terms of the form c, (* c x), or (* x c)  where x is a free constant
  and c is a term of the form n or (- n) for some numeral n.
 "
)
```

([raw file](Logics/QF_LIA.smt2))
  
  

QF\_LRA
-------

```
(logic QF_LRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-14"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5. 
  2011-06-03 Replaced ''(* c x), or (* x c)'' with ''c, (* c x), or (* x c)'' 
             in :extensions. (The missing case had been left out unintentionally.)
 "

 :theories (Reals)

 :language 
 "Closed quantifier-free formulas built over arbitrary expansions of 
  the Reals signature with free constant symbols, but containing only
  linear atoms, that is, atoms with no occurrences of the function
  symbols * and /, except as specified the :extensions attribute.
 "

 :extensions
 "Terms with _concrete_ coefficients are also allowed, that is, terms
  of the form c, (* c x), or (* x c)  where x is a free constant and 
  c is an integer or rational coefficient. 
  - An integer coefficient is a term of the form m or (- m) for some
    numeral m.
  - A rational coefficient is a term of the form d, (- d) or (/ c n) 
    for some decimal d, integer coefficient c and numeral n other than 0.
 "
)
```

([raw file](Logics/QF_LRA.smt2))
  
  

QF\_NIA
-------

```
(logic QF_NIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-05-12"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "

 :theories ( Ints )

 :language
 "Closed quantifier-free formulas built over an arbitrary expansion of the
  Ints signature with free constant symbols, but whose terms of sort Int 
  have no occurrences of the function symbol **.
 "
)
```

([raw file](Logics/QF_NIA.smt2))
  
  

QF\_NRA
-------

```
(logic QF_NRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "
 
 :theories ( Reals )

 :language 
 "Closed quantifier-free formulas built over arbitrary expansions of 
  the Reals signature with free constant symbols."
)
```

([raw file](Logics/QF_NRA.smt2))
  
  

QF\_RDL
-------

```
(logic QF_RDL

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-30"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2010-12-16 Replaced erroneous ''n > 0'' with ''n > 1'' in language attribute.
 "

 :theories (Reals)

 :language 
 "Closed quantifier-free formulas with atoms of the form:
  - p
  - (op (- x y) c),
  - (op x y),
  - (op (- (+ x ... x) (+ y ... y)) c) with n > 1 occurrences of x and of y,
  where
    - p is a variable or free constant symbol of sort Bool,
    - c is an expression of the form m or (- m) for some numeral m,
    - op is <, <=, >, >=, =, or distinct,
    - x, y are free constant symbols of sort Real. 
 "

 :extensions
 "The expression (op (- x y) (/ c n)) where n is a numeral other than 0 and
  c is an expression of the form m or (- m) for some numeral m, 
  abbreviates the expression
  (op (- (+ x ... x) (+ y ... y)) c) with n occurrences of x and y.
 "
)
```

([raw file](Logics/QF_RDL.smt2))
  
  

QF\_UF
------

```
(logic QF_UF

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2010-04-14"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "
 
 :theories ()  ; includes just Core theory

 :language 
 "Closed quantifier-free formulas built over an arbitrary expansion of
  the Core signature with free sort and function symbols.
 "
 :values
 "For each sort other than Bool the set of values is abstract.
  For Bool it is defined as in the Core theory declaration.
 "
 :notes
 "Formulas can contain variables as long as they are bound by a let binder.
 "
 :notes
 "All the free symbols used in scripts for this logic must be declared in
  the scripts themselves.
 "
)
```

([raw file](Logics/QF_UF.smt2))
  
  

QF\_UFBV
--------

```
(logic QF_UFBV

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2013-06-24 Changed references to Fixed_Size_Bitvectors to FixedSizeBitVectors.
 "

:theories ( FixedSizeBitVectors )

:language
 "Closed quantifier-free formulas built over arbitrary expansions of
  the FixedSizeBitVectors signature with free sort and function
  symbols.
 "

:extensions "As in the logic QF_BV."
)
```

([raw file](Logics/QF_UFBV.smt2))
  
  

QF\_UFIDL
---------

```
(logic QF_UFIDL

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Clark Barrett, Cesare Tinelli"
 :date "2010-05-12"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
  2011-06-03 Added :note field."

 :theories (Ints)

 :language
 "Closed quantifier-free formulas built over an arbitrary expansion with 
  free sort and function symbols of the signature consisting of 
  - all the sort and function symbols of Core and
  - the following symbols of Int:

    :sorts ((Int 0))
    :funs ((NUMERAL Int) 
           (- Int Int Int)
           (+ Int Int Int) 
           (<= Int Int Bool)
           (< Int Int Bool)
           (>= Int Int Bool)
           (> Int Int Bool)
          )

  Additionally, for every term of the form (op t1 t2) with op in {+, -}, 
  at least one of t1 and t2 is a numeral. 
 "

 :note 
 "For historical reasons, the syntax of this logic is *not* an extension 
  of QF_IDL's syntax. However, any formula in this logic with no free sorts 
  and no free function symbol of arity > 0 can be converted into an 
  equisatisfiable formula in QF_IDL.
 "
)
```

([raw file](Logics/QF_UFIDL.smt2))
  
  

QF\_UFLIA
---------

```
(logic QF_UFLIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "

 :theories ( Ints )

 :language 
 "Closed quantifier-free formulas built over arbitrary expansions of the
  Ints signatures with free sort and function symbols, but with the 
  following restrictions:
  - all terms of sort Int are linear, that is, have no occurrences of the
    function symbols *, **, /, div, mod, and abs, except as specified in the 
    :extensions attributes;
 "

 :extensions
 "As in the logic QF_LIA."
)
```

([raw file](Logics/QF_UFLIA.smt2))
  
  

QF\_UFLRA
---------

```
(logic QF_UFLRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "
 
 :theories ( Reals )

 :language 
 "Closed quantifier-free formulas built over arbitrary expansions of the 
  Reals signature with free sort and function symbols, but containing 
  only linear atoms, that is, atoms with no occurrences of the function
  symbols * and /, except as specified the :extensions attribute.
 "

 :extensions
 "Terms with _concrete_ coefficients are also allowed, that is, terms
  of the form c, (* c x), or (* x c)  where x is a free constant and 
  c is an integer or rational coefficient. 
  - An integer coefficient is a term of the form m or (- m) for some
    numeral m.
  - A rational coefficient is a term of the form d, (- d) or (/ c n) 
    for some decimal d, integer coefficient c and numeral n other than 0.
 "
)
```

([raw file](Logics/QF_UFLRA.smt2))
  
  

QF\_UFNRA
---------

```
(logic QF_UFNRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "
 
 :theories ( Reals )

 :language 
 "Closed quantifier-free formulas built over arbitrary expansions of 
  the Reals signature with free sort and function symbols.
 "
)
```

([raw file](Logics/QF_UFNRA.smt2))
  
  

UFLRA
-----

```
(logic UFLRA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "
 
 :theories ( Reals )

 :language 
 "Closed formulas built over arbitrary expansions of the Reals signature 
  with free sort and function symbols, but containing only linear atoms, 
  that is, atoms with no occurrences of the function symbols * and /,
  except as specified the :extensions attribute.
 "

 :extensions
 "Terms with _concrete_ coefficients are also allowed, that is, terms
  of the form c, (* c x), or (* x c)  where x is a free constant and 
  c is an integer or rational coefficient. 
  - An integer coefficient is a term of the form m or (- m) for some
    numeral m.
  - A rational coefficient is a term of the form d, (- d) or (/ c n) 
    for some decimal d, integer coefficient c and numeral n other than 0.
 "
)
```

([raw file](Logics/UFLRA.smt2))
  
  

UFNIA
-----

```
(logic UFNIA

 :smt-lib-version 2.7
 :smt-lib-release "2024-07-21"
 :written-by "Cesare Tinelli"
 :date "2011-06-11"
 :last-updated "2024-07-21"
 :update-history
 "Note: history only accounts for content changes, not release changes.
  2024-07-21 Updated to Version 2.7.
  2015-04-25 Updated to Version 2.5.
 "

 :theories ( Ints )

 :language 
 "Closed formulas built over an arbitrary expansion of the Ints signature
  with free sort and function symbols, but whose terms of sort Int 
  have no occurrences of the function symbol **."
)
```

([raw file](Logics/UFNIA.smt2))

* [Home](index.shtml)
* [About](about.shtml)
* [News](news.shtml)
* Standard
  + [Language](language.shtml)
  + [Theories](theories.shtml)
  + [Logics](logics.shtml)
  + [Examples](examples.shtml)
* [Benchmarks](benchmarks.shtml)
* Software
  + [Solvers](solvers.shtml)
  + [Utilities](utilities.shtml)
* [Contact](contact.shtml)
* [Related](related.shtml)
* [Credits](credits.shtml)

© Copyright The SMT-LIB Initiative   
Based on a design by
Blue [Web Templates](http://www.bluewebtemplates.com)

* [Home](index.shtml)
* [About](about.shtml)
* [News](news.shtml)
* [Standard](standard.shtml)
* [Benchmarks](benchmarks.shtml)
* [Software](software.shtml)
* [Credits](credits.shtml)
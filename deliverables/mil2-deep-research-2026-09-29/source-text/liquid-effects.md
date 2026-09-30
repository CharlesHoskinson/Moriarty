Deterministic Parallelism via Liquid Effects
--------------------------------------------

*[Ming Kawaguchi](http://www.cs.ucsd.edu/~mwookawa),
[Patrick M. Rondon](http://www.cs.ucsd.edu/~prondon),
[Alexander Bakst](http://www.cs.ucsd.edu/~abakst), and
[Ranjit Jhala](http://www.cs.ucsd.edu/~rjhala).*

hared memory multithreading is a popular approach to parallel programming,
but also fiendishly hard to get right. We present Liquid Effects, a
type-and-effect system based on refinement types which allows for
fine-grained, low-level, shared memory multithreading while statically
guaranteeing that a program is deterministic. Liquid Effects records the
effect of an expression as a formula in first-order logic, making our
type-and-effect system highly expressive. Further, effects like Read and
Write are recorded in Liquid Effects as ordinary uninterpreted predicates,
leaving the effect system open to extension by the user. By building our
system as an extension to an existing dependent refinement type system,
our system gains precise value- and branch-sensitive reasoning about
effects. Finally, our system exploits the Liquid Types refinement type
inference technique to automatically infer refinement types and effects. We
have implemented our type-and-effect checking techniques in CSOLVE, a
refinement type inference system for C programs. We demonstrate how CSOLVE
uses Liquid Effects to prove the determinism of a variety of benchmarks

*In the Proceedings of the
34th ACM Conference on Programming Language Design and Implementation, 2012.*
(PLDI 2012).

[PostScript](deterministic_parallelism_via_liquid_effects.ps)
[PDF](deterministic_parallelism_via_liquid_effects.pdf)

---

---

© 2012.  
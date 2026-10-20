### Requirement: UIREQ-007 Typed inductive values
WHEN a declared constructor is evaluated, the interpreter SHALL produce a finite immutable value of its declared nominal type.

#### Scenario: Recursive list value
- **WHEN** Cons(1,Cons(2,Nil)) is built under ListUInt128 declarations.
- **THEN** The result has the stated ordered fields and recursive list type.


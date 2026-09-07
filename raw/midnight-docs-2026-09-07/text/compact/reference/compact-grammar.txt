> For the complete documentation index, see [llms.txt](/llms.txt)

# Compact grammar

Compact language version 0.26.0.

Notational note: In the grammar below, keywords and punctuation are in `monospaced` font. Terminal and nonterminal names are in *emphasized* font. Alternation is indicated by a vertical bar (`|`). Optional items are indicated by the superscript opt. Repetition is specified by ellipses. The notation *X* ⋯ *X*, where *X* is a grammar symbol, represents zero or more occurrences of *X*. The notation *X* `,` ⋯ `,` *X*, where *X* is a grammar symbol and `,` is a literal comma, represents zero or more occurrences of *X* separated by commas. In either case, when the ellipsis is marked with the superscript 1, the notation represents a sequence containing at least one *X*. When such a sequence is followed by *,*&#x6F;pt, an optional trailing comma is allowed, but only if there is at least one *X*. For example, *id* ⋯ *id* represents zero or more *id*s, and *expr* `,` ⋯¹`,` *expr* `,`opt represents one or more comma-separated *expr*s possibly followed by an extra comma. The rules involving commas apply equally to semicolons, i.e., apply when `,` is replaced by `;`.

#### identifier[​](#identifier "Direct link to identifier")

*id*, *module-name*, *function-name*, *struct-name*, *enum-name*, *contract-name*, *tvar-name*, *type-name*

Identifiers have the same syntax as Typescript identifiers.

#### field-literal[​](#field-literal "Direct link to field-literal")

*nat*

A field literal is 0 or a natural number formed from a sequence of digits starting with 1-9, e.g. 723, whose value does not exceed the maximum field value.

#### string-literal[​](#string-literal "Direct link to string-literal")

*str*, *file*

A string literal has the same syntax as a Typescript string.

#### version-literal[​](#version-literal "Direct link to version-literal")

*version*

A version literal takes the form nat.nat representing major and minor versions or nat.nat.nat representing major, minor, and bugfix versions. Where version literals are allowed, a plain nat representing just the major version is also allowed.

#### Compact[​](#compact "Direct link to Compact")

|           |   |                                                                               |
| --------- | - | ----------------------------------------------------------------------------- |
| *program* | ⟶ | [*program-element*](#program-element) ⋯ [*program-element*](#program-element) |

#### Program-element[​](#program-element "Direct link to Program-element")

|                   |    |                                                              |
| ----------------- | -- | ------------------------------------------------------------ |
| *program-element* | ⟶  | [*pragma-form*](#pragma)                                     |
|                   | \| | [*module-definition*](#module-definition)                    |
|                   | \| | [*import-form*](#import-declaration)                         |
|                   | \| | [*export-form*](#export-declaration)                         |
|                   | \| | [*include-form*](#include)                                   |
|                   | \| | [*struct-declaration*](#structure-declaration)               |
|                   | \| | [*enum-declaration*](#enum-declaration)                      |
|                   | \| | [*contract-declaration*](#external-contract-declaration)     |
|                   | \| | [*implements-declaration*](#contract-implements-declaration) |
|                   | \| | [*type-alias-declaration*](#type-declaration)                |
|                   | \| | [*ledger-declaration*](#ledger-declaration)                  |
|                   | \| | [*witness-declaration*](#witness-declaration)                |
|                   | \| | [*constructor-definition*](#constructor)                     |
|                   | \| | [*circuit-definition*](#circuit-definition)                  |

#### Pragma[​](#pragma "Direct link to Pragma")

|               |   |                                                                        |
| ------------- | - | ---------------------------------------------------------------------- |
| *pragma-form* | ⟶ | `pragma` [*id*](#identifier) [*version-expr*](#version-expression) `;` |

#### Version-expression[​](#version-expression "Direct link to Version-expression")

|                |    |                                                                                      |
| -------------- | -- | ------------------------------------------------------------------------------------ |
| *version-expr* | ⟶  | [*version-expr*](#version-expression) `\|\|` [*version-expr0*](#version-expression0) |
|                | \| | [*version-expr0*](#version-expression0)                                              |

#### Version-expression0[​](#version-expression0 "Direct link to version-expression0")

|                 |    |                                                                              |
| --------------- | -- | ---------------------------------------------------------------------------- |
| *version-expr0* | ⟶  | [*version-expr0*](#version-expression0) `&&` [*version-term*](#version-term) |
|                 | \| | [*version-term*](#version-term)                                              |

#### Version-Term[​](#version-term "Direct link to Version-Term")

|                |    |                                               |
| -------------- | -- | --------------------------------------------- |
| *version-term* | ⟶  | [*version-atom*](#version-atom)               |
|                | \| | `!` [*version-atom*](#version-atom)           |
|                | \| | `<` [*version-atom*](#version-atom)           |
|                | \| | `<=` [*version-atom*](#version-atom)          |
|                | \| | `>=` [*version-atom*](#version-atom)          |
|                | \| | `>` [*version-atom*](#version-atom)           |
|                | \| | `(` [*version-expr*](#version-expression) `)` |

#### Version-atom[​](#version-atom "Direct link to Version-atom")

|                |    |                               |
| -------------- | -- | ----------------------------- |
| *version-atom* | ⟶  | [*nat*](#field-literal)       |
|                | \| | [*version*](#version-literal) |

#### Include[​](#include "Direct link to Include")

|                |   |                                         |
| -------------- | - | --------------------------------------- |
| *include-form* | ⟶ | `include` [*file*](#string-literal) `;` |

#### Module-definition[​](#module-definition "Direct link to Module-definition")

|                     |   |                                                                                                                                                                                 |
| ------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| *module-definition* | ⟶ | `export`opt `module` [*module-name*](#identifier) [*gparams*](#generic-parameter-list)opt `{` [*program-element*](#program-element) ⋯ [*program-element*](#program-element) `}` |

#### Generic-parameter-list[​](#generic-parameter-list "Direct link to Generic-parameter-list")

|           |   |                                                                                                      |
| --------- | - | ---------------------------------------------------------------------------------------------------- |
| *gparams* | ⟶ | `<` [*generic-param*](#generic-parameter) `,` ⋯ `,` [*generic-param*](#generic-parameter) `,`opt `>` |

#### Generic-parameter[​](#generic-parameter "Direct link to Generic-parameter")

|                 |    |                                |
| --------------- | -- | ------------------------------ |
| *generic-param* | ⟶  | `#` [*tvar-name*](#identifier) |
|                 | \| | [*tvar-name*](#identifier)     |

#### Import-declaration[​](#import-declaration "Direct link to Import-declaration")

|               |   |                                                                                                                                                                 |
| ------------- | - | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| *import-form* | ⟶ | `import` [*import-selection*](#import-selection)opt [*import-name*](#import-name) [*gargs*](#generic-argument-list)opt [*import-prefix*](#import-prefix)opt `;` |

#### Import-selection[​](#import-selection "Direct link to Import-selection")

|                    |   |                                                                                                         |
| ------------------ | - | ------------------------------------------------------------------------------------------------------- |
| *import-selection* | ⟶ | `{` [*import-element*](#import-element) `,` ⋯ `,` [*import-element*](#import-element) `,`opt `}` `from` |

#### Import-element[​](#import-element "Direct link to Import-element")

|                  |    |                                              |
| ---------------- | -- | -------------------------------------------- |
| *import-element* | ⟶  | [*id*](#identifier)                          |
|                  | \| | [*id*](#identifier) `as` [*id*](#identifier) |

#### Import-name[​](#import-name "Direct link to Import-name")

|               |    |                           |
| ------------- | -- | ------------------------- |
| *import-name* | ⟶  | [*id*](#identifier)       |
|               | \| | [*file*](#string-literal) |

#### Import-prefix[​](#import-prefix "Direct link to Import-prefix")

|                 |   |                              |
| --------------- | - | ---------------------------- |
| *import-prefix* | ⟶ | `prefix` [*id*](#identifier) |

#### Generic-argument-list[​](#generic-argument-list "Direct link to Generic-argument-list")

|         |   |                                                                                  |
| ------- | - | -------------------------------------------------------------------------------- |
| *gargs* | ⟶ | `<` [*garg*](#generic-argument) `,` ⋯ `,` [*garg*](#generic-argument) `,`opt `>` |

#### Generic-argument[​](#generic-argument "Direct link to Generic-argument")

|        |    |                         |
| ------ | -- | ----------------------- |
| *garg* | ⟶  | [*nat*](#field-literal) |
|        | \| | [*type*](#type)         |

#### Export-declaration[​](#export-declaration "Direct link to Export-declaration")

|               |   |                                                                                  |
| ------------- | - | -------------------------------------------------------------------------------- |
| *export-form* | ⟶ | `export` `{` [*id*](#identifier) `,` ⋯ `,` [*id*](#identifier) `,`opt `}` `;`opt |

#### Ledger-declaration[​](#ledger-declaration "Direct link to Ledger-declaration")

|                      |   |                                                                              |
| -------------------- | - | ---------------------------------------------------------------------------- |
| *ledger-declaration* | ⟶ | `export`opt `sealed`opt `ledger` [*id*](#identifier) `:` [*type*](#type) `;` |

#### Witness-declaration[​](#witness-declaration "Direct link to Witness-declaration")

|                       |   |                                                                                                                                                             |
| --------------------- | - | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| *witness-declaration* | ⟶ | `export`opt `witness` [*id*](#identifier) [*gparams*](#generic-parameter-list)opt [*simple-parameter-list*](#simple-parameter-list) `:` [*type*](#type) `;` |

#### Constructor[​](#constructor "Direct link to Constructor")

|                          |   |                                                                                     |
| ------------------------ | - | ----------------------------------------------------------------------------------- |
| *constructor-definition* | ⟶ | `constructor` [*pattern-parameter-list*](#pattern-parameter-list) [*block*](#block) |

#### Circuit-definition[​](#circuit-definition "Direct link to Circuit-definition")

|                      |   |                                                                                                                                                                                                  |
| -------------------- | - | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| *circuit-definition* | ⟶ | `export`opt `pure`opt `circuit` [*function-name*](#identifier) [*gparams*](#generic-parameter-list)opt [*pattern-parameter-list*](#pattern-parameter-list) `:` [*type*](#type) [*block*](#block) |

#### Structure-declaration[​](#structure-declaration "Direct link to Structure-declaration")

|                      |    |                                                                                                                                                                                           |
| -------------------- | -- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| *struct-declaration* | ⟶  | `export`opt `struct` [*struct-name*](#identifier) [*gparams*](#generic-parameter-list)opt `{` [*typed-id*](#typed-identifier) `;` ⋯ `;` [*typed-id*](#typed-identifier) `;`opt `}` `;`opt |
|                      | \| | `export`opt `struct` [*struct-name*](#identifier) [*gparams*](#generic-parameter-list)opt `{` [*typed-id*](#typed-identifier) `,` ⋯ `,` [*typed-id*](#typed-identifier) `,`opt `}` `;`opt |

#### Enum-declaration[​](#enum-declaration "Direct link to Enum-declaration")

|                    |   |                                                                                                                        |
| ------------------ | - | ---------------------------------------------------------------------------------------------------------------------- |
| *enum-declaration* | ⟶ | `export`opt `enum` [*enum-name*](#identifier) `{` [*id*](#identifier) `,` ⋯¹ `,` [*id*](#identifier) `,`opt `}` `;`opt |

#### Contract-Implements-declaration[​](#contract-implements-declaration "Direct link to Contract-Implements-declaration")

|                          |   |                                             |
| ------------------------ | - | ------------------------------------------- |
| *implements-declaration* | ⟶ | `contract` `implements` [*type*](#type) `;` |

#### External-contract-declaration[​](#external-contract-declaration "Direct link to External-contract-declaration")

|                        |    |                                                                                                                                                                                               |
| ---------------------- | -- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| *contract-declaration* | ⟶  | `export`opt `contract` [*contract-name*](#identifier) `{` [*circuit-declaration*](#external-contract-circuit) `;` ⋯ `;` [*circuit-declaration*](#external-contract-circuit) `;`opt `}` `;`opt |
|                        | \| | `export`opt `contract` [*contract-name*](#identifier) `{` [*circuit-declaration*](#external-contract-circuit) `,` ⋯ `,` [*circuit-declaration*](#external-contract-circuit) `,`opt `}` `;`opt |

#### External-contract-circuit[​](#external-contract-circuit "Direct link to External-contract-circuit")

|                       |   |                                                                                                               |
| --------------------- | - | ------------------------------------------------------------------------------------------------------------- |
| *circuit-declaration* | ⟶ | `pure`opt `circuit` [*id*](#identifier) [*simple-parameter-list*](#simple-parameter-list) `:` [*type*](#type) |

#### Type-declaration[​](#type-declaration "Direct link to Type-declaration")

|                          |   |                                                                                                                        |
| ------------------------ | - | ---------------------------------------------------------------------------------------------------------------------- |
| *type-alias-declaration* | ⟶ | `export`opt `new`opt `type` [*type-name*](#identifier) [*gparams*](#generic-parameter-list)opt `=` [*type*](#type) `;` |

#### Typed-identifier[​](#typed-identifier "Direct link to Typed-identifier")

|            |   |                                         |
| ---------- | - | --------------------------------------- |
| *typed-id* | ⟶ | [*id*](#identifier) `:` [*type*](#type) |

#### Simple-parameter-list[​](#simple-parameter-list "Direct link to Simple-parameter-list")

|                         |   |                                                                                          |
| ----------------------- | - | ---------------------------------------------------------------------------------------- |
| *simple-parameter-list* | ⟶ | `(` [*typed-id*](#typed-identifier) `,` ⋯ `,` [*typed-id*](#typed-identifier) `,`opt `)` |

#### Typed-pattern[​](#typed-pattern "Direct link to Typed-pattern")

|                 |   |                                           |
| --------------- | - | ----------------------------------------- |
| *typed-pattern* | ⟶ | [*pattern*](#pattern) `:` [*type*](#type) |

#### Pattern-parameter-list[​](#pattern-parameter-list "Direct link to Pattern-parameter-list")

|                          |   |                                                                                              |
| ------------------------ | - | -------------------------------------------------------------------------------------------- |
| *pattern-parameter-list* | ⟶ | `(` [*typed-pattern*](#typed-pattern) `,` ⋯ `,` [*typed-pattern*](#typed-pattern) `,`opt `)` |

#### Type[​](#type "Direct link to Type")

|        |    |                                                                 |
| ------ | -- | --------------------------------------------------------------- |
| *type* | ⟶  | [*tref*](#type-reference)                                       |
|        | \| | `Boolean`                                                       |
|        | \| | `Field`                                                         |
|        | \| | `Uint` `<` [*tsize*](#type-size) `>`                            |
|        | \| | `Uint` `<` [*tsize*](#type-size) `..` [*tsize*](#type-size) `>` |
|        | \| | `Bytes` `<` [*tsize*](#type-size) `>`                           |
|        | \| | `Opaque` `<` [*str*](#string-literal) `>`                       |
|        | \| | `Vector` `<` [*tsize*](#type-size) `,` [*type*](#type) `>`      |
|        | \| | `[` [*type*](#type) `,` ⋯ `,` [*type*](#type) `,`opt `]`        |

#### Type-reference[​](#type-reference "Direct link to Type-reference")

|        |   |                                                          |
| ------ | - | -------------------------------------------------------- |
| *tref* | ⟶ | [*id*](#identifier) [*gargs*](#generic-argument-list)opt |

#### Type-size[​](#type-size "Direct link to Type-size")

|         |    |                         |
| ------- | -- | ----------------------- |
| *tsize* | ⟶  | [*nat*](#field-literal) |
|         | \| | [*id*](#identifier)     |
| *start* | ⟶  | [*tsize*](#type-size)   |
| *end*   | ⟶  | [*tsize*](#type-size)   |

#### Block[​](#block "Direct link to Block")

|         |   |                                                     |
| ------- | - | --------------------------------------------------- |
| *block* | ⟶ | `{` [*stmt*](#statement) ⋯ [*stmt*](#statement) `}` |

#### Statement[​](#statement "Direct link to Statement")

|        |    |                                                                      |
| ------ | -- | -------------------------------------------------------------------- |
| *stmt* | ⟶  | `if` `(` [*expr-seq*](#expression-sequence) `)` [*stmt*](#statement) |
|        | \| | [*stmt0*](#statement0)                                               |

#### Statement0[​](#statement0 "Direct link to statement0")

|         |    |                                                                                                                    |
| ------- | -- | ------------------------------------------------------------------------------------------------------------------ |
| *stmt0* | ⟶  | [*expr-seq*](#expression-sequence) `;`                                                                             |
|         | \| | `const` [*cbinding*](#const-binding) `,` ⋯¹ `,` [*cbinding*](#const-binding) `;`                                   |
|         | \| | `if` `(` [*expr-seq*](#expression-sequence) `)` [*stmt0*](#statement0) `else` [*stmt*](#statement)                 |
|         | \| | `for` `(` `const` [*id*](#identifier) `of` [*start*](#type-size) `..` [*end*](#type-size) `)` [*stmt*](#statement) |
|         | \| | `for` `(` `const` [*id*](#identifier) `of` [*expr-seq*](#expression-sequence) `)` [*stmt*](#statement)             |
|         | \| | `return` [*expr-seq*](#expression-sequence) `;`                                                                    |
|         | \| | `return` `;`                                                                                                       |
|         | \| | [*block*](#block)                                                                                                  |

#### Pattern[​](#pattern "Direct link to Pattern")

|           |    |                                                                                                                          |
| --------- | -- | ------------------------------------------------------------------------------------------------------------------------ |
| *pattern* | ⟶  | [*id*](#identifier)                                                                                                      |
|           | \| | `[` [*pattern*](#pattern)opt `,` ⋯ `,` [*pattern*](#pattern)opt `,`opt `]`                                               |
|           | \| | `{` [*pattern-struct-elt*](#pattern-struct-element) `,` ⋯ `,` [*pattern-struct-elt*](#pattern-struct-element) `,`opt `}` |

#### Pattern-struct-element[​](#pattern-struct-element "Direct link to Pattern-struct-element")

|                      |    |                                               |
| -------------------- | -- | --------------------------------------------- |
| *pattern-struct-elt* | ⟶  | [*id*](#identifier)                           |
|                      | \| | [*id*](#identifier) `:` [*pattern*](#pattern) |

#### Expression-sequence[​](#expression-sequence "Direct link to Expression-sequence")

|            |    |                                                                                  |
| ---------- | -- | -------------------------------------------------------------------------------- |
| *expr-seq* | ⟶  | [*expr*](#expression)                                                            |
|            | \| | [*expr*](#expression) `,` ⋯¹ `,` [*expr*](#expression) `,` [*expr*](#expression) |

#### Expression[​](#expression "Direct link to Expression")

|        |    |                                                                             |
| ------ | -- | --------------------------------------------------------------------------- |
| *expr* | ⟶  | [*expr0*](#expression0) `?` [*expr*](#expression) `:` [*expr*](#expression) |
|        | \| | [*expr0*](#expression0) `=` [*expr*](#expression)                           |
|        | \| | [*expr0*](#expression0) `+=` [*expr*](#expression)                          |
|        | \| | [*expr0*](#expression0) `-=` [*expr*](#expression)                          |
|        | \| | [*expr0*](#expression0)                                                     |

#### Expression0[​](#expression0 "Direct link to expression0")

|         |    |                                                        |
| ------- | -- | ------------------------------------------------------ |
| *expr0* | ⟶  | [*expr0*](#expression0) `\|\|` [*expr1*](#expression1) |
|         | \| | [*expr1*](#expression1)                                |

#### Expression1[​](#expression1 "Direct link to expression1")

|         |    |                                                      |
| ------- | -- | ---------------------------------------------------- |
| *expr1* | ⟶  | [*expr1*](#expression1) `&&` [*expr2*](#expression2) |
|         | \| | [*expr2*](#expression2)                              |

#### Expression2[​](#expression2 "Direct link to expression2")

|         |    |                                                      |
| ------- | -- | ---------------------------------------------------- |
| *expr2* | ⟶  | [*expr2*](#expression2) `==` [*expr3*](#expression3) |
|         | \| | [*expr2*](#expression2) `!=` [*expr3*](#expression3) |
|         | \| | [*expr3*](#expression3)                              |

#### Expression3[​](#expression3 "Direct link to expression3")

|         |    |                                                      |
| ------- | -- | ---------------------------------------------------- |
| *expr3* | ⟶  | [*expr4*](#expression4) `<` [*expr4*](#expression4)  |
|         | \| | [*expr4*](#expression4) `<=` [*expr4*](#expression4) |
|         | \| | [*expr4*](#expression4) `>=` [*expr4*](#expression4) |
|         | \| | [*expr4*](#expression4) `>` [*expr4*](#expression4)  |
|         | \| | [*expr4*](#expression4)                              |

#### Expression4[​](#expression4 "Direct link to expression4")

|         |    |                                              |
| ------- | -- | -------------------------------------------- |
| *expr4* | ⟶  | [*expr4*](#expression4) `as` [*type*](#type) |
|         | \| | [*expr5*](#expression5)                      |

#### Expression5[​](#expression5 "Direct link to expression5")

|         |    |                                                     |
| ------- | -- | --------------------------------------------------- |
| *expr5* | ⟶  | [*expr5*](#expression5) `+` [*expr6*](#expression6) |
|         | \| | [*expr5*](#expression5) `-` [*expr6*](#expression6) |
|         | \| | [*expr6*](#expression6)                             |

#### Expression6[​](#expression6 "Direct link to expression6")

|         |    |                                                     |
| ------- | -- | --------------------------------------------------- |
| *expr6* | ⟶  | [*expr6*](#expression6) `*` [*expr7*](#expression7) |
|         | \| | [*expr7*](#expression7)                             |

#### Expression7[​](#expression7 "Direct link to expression7")

|         |    |                             |
| ------- | -- | --------------------------- |
| *expr7* | ⟶  | `!` [*expr7*](#expression7) |
|         | \| | [*expr8*](#expression8)     |

#### Expression8[​](#expression8 "Direct link to expression8")

|         |    |                                                                                                                      |
| ------- | -- | -------------------------------------------------------------------------------------------------------------------- |
| *expr8* | ⟶  | [*expr8*](#expression8) `[` [*expr*](#expression) `]`                                                                |
|         | \| | [*expr8*](#expression8) `.` [*id*](#identifier)                                                                      |
|         | \| | [*expr8*](#expression8) `.` [*id*](#identifier) `(` [*expr*](#expression) `,` ⋯ `,` [*expr*](#expression) `,`opt `)` |
|         | \| | [*expr9*](#expression9)                                                                                              |

#### Expression9[​](#expression9 "Direct link to expression9")

|         |    |                                                                                                                               |
| ------- | -- | ----------------------------------------------------------------------------------------------------------------------------- |
| *expr9* | ⟶  | [*fun*](#function) `(` [*expr*](#expression) `,` ⋯ `,` [*expr*](#expression) `,`opt `)`                                       |
|         | \| | `map` `(` [*fun*](#function) `,` [*expr*](#expression) `,` ⋯¹ `,` [*expr*](#expression) `,`opt `)`                            |
|         | \| | `fold` `(` [*fun*](#function) `,` [*expr*](#expression) `,` [*expr*](#expression) `,` ⋯¹ `,` [*expr*](#expression) `,`opt `)` |
|         | \| | `slice` `<` [*tsize*](#type-size) `>` `(` [*expr*](#expression) `,` [*expr*](#expression) `)`                                 |
|         | \| | `[` [*tuple-arg*](#tuple-argument) `,` ⋯ `,` [*tuple-arg*](#tuple-argument) `,`opt `]`                                        |
|         | \| | `Bytes` `[` [*bytes-arg*](#tuple-argument) `,` ⋯ `,` [*bytes-arg*](#tuple-argument) `,`opt `]`                                |
|         | \| | [*tref*](#type-reference) `{` [*struct-arg*](#structure-argument) `,` ⋯ `,` [*struct-arg*](#structure-argument) `,`opt `}`    |
|         | \| | `assert` `(` [*expr*](#expression) `,` [*str*](#string-literal) `)`                                                           |
|         | \| | `emit` `(` [*expr*](#expression) `)`                                                                                          |
|         | \| | `disclose` `(` [*expr*](#expression) `)`                                                                                      |
|         | \| | [*term*](#term)                                                                                                               |

#### Term[​](#term "Direct link to Term")

|        |    |                                                                    |
| ------ | -- | ------------------------------------------------------------------ |
| *term* | ⟶  | [*id*](#identifier)                                                |
|        | \| | `true`                                                             |
|        | \| | `false`                                                            |
|        | \| | [*nat*](#field-literal)                                            |
|        | \| | [*str*](#string-literal)                                           |
|        | \| | `pad` `(` [*nat*](#field-literal) `,` [*str*](#string-literal) `)` |
|        | \| | `default` `<` [*type*](#type) `>`                                  |
|        | \| | `(` [*expr-seq*](#expression-sequence) `)`                         |

#### Tuple-argument[​](#tuple-argument "Direct link to Tuple-argument")

|             |    |                                |
| ----------- | -- | ------------------------------ |
| *tuple-arg* | ⟶  | [*expr*](#expression)          |
|             | \| | `...` [*expr*](#expression)    |
| *bytes-arg* | ⟶  | [*tuple-arg*](#tuple-argument) |

#### Structure-argument[​](#structure-argument "Direct link to Structure-argument")

|              |    |                                               |
| ------------ | -- | --------------------------------------------- |
| *struct-arg* | ⟶  | [*expr*](#expression)                         |
|              | \| | [*id*](#identifier) `:` [*expr*](#expression) |
|              | \| | `...` [*expr*](#expression)                   |

#### Function[​](#function "Direct link to Function")

|       |    |                                                                                                             |
| ----- | -- | ----------------------------------------------------------------------------------------------------------- |
| *fun* | ⟶  | [*id*](#identifier) [*gargs*](#generic-argument-list)opt                                                    |
|       | \| | [*arrow-parameter-list*](#arrow-parameter-list) [*return-type*](#return-type)opt `=>` [*block*](#block)     |
|       | \| | [*arrow-parameter-list*](#arrow-parameter-list) [*return-type*](#return-type)opt `=>` [*expr*](#expression) |
|       | \| | `(` [*fun*](#function) `)`                                                                                  |

#### Return-type[​](#return-type "Direct link to Return-type")

|               |   |                     |
| ------------- | - | ------------------- |
| *return-type* | ⟶ | `:` [*type*](#type) |

#### Optionally-typed-pattern[​](#optionally-typed-pattern "Direct link to Optionally-typed-pattern")

|                            |    |                                   |
| -------------------------- | -- | --------------------------------- |
| *optionally-typed-pattern* | ⟶  | [*pattern*](#pattern)             |
|                            | \| | [*typed-pattern*](#typed-pattern) |

#### Const-Binding[​](#const-binding "Direct link to Const-Binding")

|            |   |                                                                                   |
| ---------- | - | --------------------------------------------------------------------------------- |
| *cbinding* | ⟶ | [*optionally-typed-pattern*](#optionally-typed-pattern) `=` [*expr*](#expression) |

#### Arrow-parameter-list[​](#arrow-parameter-list "Direct link to Arrow-parameter-list")

|                        |   |                                                                                                                                          |
| ---------------------- | - | ---------------------------------------------------------------------------------------------------------------------------------------- |
| *arrow-parameter-list* | ⟶ | `(` [*optionally-typed-pattern*](#optionally-typed-pattern) `,` ⋯ `,` [*optionally-typed-pattern*](#optionally-typed-pattern) `,`opt `)` |

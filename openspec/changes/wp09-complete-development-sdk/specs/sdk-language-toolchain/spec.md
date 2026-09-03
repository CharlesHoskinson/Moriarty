# SDK language-toolchain specification

## ADDED Requirements

### Requirement: canonical source pipeline

The SDK SHALL provide a textual parser, formatter, linter, type checker,
visibility checker, capability checker, bound checker, elaborator, Core
normalizer, serializer, source maps, and Compact generator.

#### Scenario: formatted source is parsed again

- WHEN valid source is parsed, formatted, and parsed again
- THEN both parses elaborate to identical canonical Core bytes.

### Requirement: typed domains and visibility

The checker SHALL distinguish amounts, tokens, parties, authorities,
timestamps, durations, choices, hashes, attestations, capabilities, and
visibility states.

#### Scenario: two token quantities are added incorrectly

- WHEN their token identities differ
- THEN type checking fails before Core elaboration.

### Requirement: finite elaboration

Modules, templates, functions, folds, loops, schedules, and comprehensions SHALL
elaborate to finite Core with explicit lifetime and resource bounds.

#### Scenario: a compile-time expansion exceeds its limit

- WHEN expansion exceeds the declared bound
- THEN elaboration returns a typed bound error and emits no deployment artifact.

### Requirement: shared service contracts

The CLI and language server SHALL use the same parser, checker, elaborator, and
diagnostic library. Diagnostics SHALL include stable codes and source spans.

#### Scenario: editor and CLI check identical bytes

- WHEN both use the same version and configuration
- THEN they return the same semantic diagnostics in the same stable order.

### Requirement: AI remains outside the compiler

AI assistance MAY propose source or explanations. Its output SHALL pass every
human-source compiler and approval gate without privileged bypass.

#### Scenario: an LLM asserts that a contract is safe

- WHEN no analyzer or proof artifact supports the assertion
- THEN the SDK does not display it as verified evidence.

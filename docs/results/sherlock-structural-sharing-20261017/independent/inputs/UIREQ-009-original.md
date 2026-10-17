### Requirement: UIREQ-009 Structural value meaning
WHEN an implementation shares an immutable subvalue physically, the interpreter SHALL preserve its structural observations and declared traversal accounting.

#### Scenario: Repeated subtree
- **WHEN** A tree has two occurrences of Leaf(4), represented either separately or by one shared node.
- **THEN** Its fold returns 8 and charges both semantic visits under the same traversal schedule.


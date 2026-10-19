Independent finite expected-trace derivation

Authority: only the five complete safe bodies in the pinned author request. No implementation or actual state supplies an expectation.

Initial: ngInitial places the supplied whole program in the state, ngEval names entry 10, the supplied environment binds name 900 to UInt 7, and frames/events are empty.

First observation: source expression 10 is Add occurrence 10010 with left expression 11 and right expression 12. Strict left-before-right evaluation plus a separate Add-start event starts evaluation of expression 11. The ngAddLeft continuation saves expression 10, occurrence 10010, right expression 12 and the complete environment. ngAddStarted records both child expression IDs and the saved environment in its declared order. This step does not evaluate a child. Program and environment are unchanged.

Second observation: active source expression 11 is a literal at occurrence 10011 with supplied value UInt 17. It returns exactly that typed input value and appends ngLiteralEvaluated with that expression ID and occurrence. Producing Return is separate from consuming its continuation. The ngAddLeft continuation therefore remains unchanged; the Add-start event remains before the literal event. Program and environment remain unchanged. The right operand and arithmetic completion are beyond the observation bound.

Each window retains its initial state and single afterstate, with final represented once. Window 2 starts at the identical whole middle boundary, retaining its frame and cumulative event. Joining removes only the duplicated shared middle state. Boundary identity compares program, control, environment, top-first frames and cumulative events with integer/Bool distinction. Actual boundary identity remains Root's separate obligation.

The packet calls observerCuts an independently authored ordinal array, without separately declaring numeric base or locality. These files explicitly choose zero-based local state positions for each wire; joined positions are derived from the own joined states. This representational convention is recorded for review without actual exposure. No semantic input fact is missing for the two observations.

This finite expectation establishes no actual runtime result, implementation-validity verdict, security verdict, charged-work identity, terminal publication, native/proof/ledger acceptance, or deployment permission.

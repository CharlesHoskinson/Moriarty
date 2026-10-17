# Groth16 section notes

## Display replacement
In technical.sourceVersusDesign, replace
`The executed source differs.`
with
`The S12 source differs from this design.`
S12 was source-read, not executed, in this packet.

## Facts checked
- Shared phase 1, per-circuit phase 2 (G02, G04)
- Setup(R), trapdoor and Sim (G05)
- Statement/witness boundary (G05; V2 proofRoleExplanation)
- Unknown Cardano total; current test setup (V2 technical)

## Scope
- The main copy is in the present tense. Status is kept behind the technical link.
- The counter is hypothetical arithmetic for K compatible circuits.
- No size, gas or cost claims.
- Book statistics and the layer names are NOT_CHECKED.
- This is not a certification of the whole book.
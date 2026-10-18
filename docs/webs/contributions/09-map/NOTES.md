# Role 09 diagram and map

`WebDiagram.jsx` is a controlled React component. It takes `scenario`, `selectedStrand`, `onSelect`, `omittedStrands`, and `onToggleOmit`. It renders the scenario’s three future role specifications as connected SVG geometry. The caption calls these three required future proof-role specifications and separates role identity from independent producers. HTML buttons control selection and omission.

The SVG is decorative. Each role’s complete claim, statement, authentication context, and limits remain in HTML. Controls use visible text, `aria-pressed`, and inline 44 px minimum targets. Selection uses a thicker outline and the word “SELECTED.” Omission uses a dashed outline and the word “OMITTED.” The status text does not rely on color or hover.

The caption states that the roles are future requirements, not existing proof producers, concrete Strands, proof instances, verified assembly, or settlement. It also states that one native whole-execution proof may cover several arithmetic conditions. The omitted state describes only an incomplete illustration.

`map.json` maps the five frozen scenarios and fifteen role records by exact JSON pointers. It also maps all twenty S02 retained targets through the read-only study-09-r2 concept graph. Every family-to-case connection is marked `INFERRED` and `PROPOSED`. These destinations are not support claims. Two families have no link to this five-case set.

The source references copied from the frozen scenario file remain verbatim. Role 09 did not reopen every linked claims-register or security-errata file. `SOURCE-PRECONDITION-CLARIFICATION.json` corrects the source partial-payment range to amounts 1–99. The fixture map keeps amount 100 refused and any related arithmetic hypothetical.

## CSS integration

Role 03 can style `.web-diagram`, `.web-diagram__visual`, `.web-diagram__svg`, `.web-diagram__edge`, `.web-diagram__hub`, `.web-diagram__caption`, `.web-diagram__state`, `.web-diagram__controls`, `.web-diagram__role`, `.web-diagram__role-shape`, and `.strand-node` descendants. The component does not modify or assume a particular CSS implementation. Keep SVG lines and text visible in forced-colors mode. Preserve the button minimum target size and visible keyboard focus.

The component owns no build entry, dependency, CSS file, browser state, network request, storage access, wallet, or ledger behavior. It was not built or rendered in this role. Root/role 07 owns integration and actual render verification.

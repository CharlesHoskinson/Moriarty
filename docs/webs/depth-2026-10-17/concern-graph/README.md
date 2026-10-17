# ConcernGraph

The component replaces the existing seven-concern selector with one graph view.
It accepts the existing `sevenLayers.layers` array unchanged:

```jsx
import ConcernGraph from './ConcernGraph.jsx';

<ConcernGraph layers={c.layers} selected={selected} onSelect={select}/>
```

Copy `ConcernGraph.jsx` and `concern-graph.css` together; the component imports its
stylesheet. Replace the old `.layer-selector` and its button loop with this
component. Keep the existing selected-concern explanation panel, source copy and
relationship prose. This view changes selection through `onSelect(number)` and
does not render or rewrite their technical content.

Each layer uses its existing `number`, `name`, optional `subtitle`, and optional
`dependencies` number array. Names, numbering and subtitles display unchanged.
There is exactly one native `button[data-layer]` per supplied concern. The parent
owns `selected`; ordinary Tab, Enter and Space navigation work without custom key
handling. The existing parent `select` handler can continue updating the selected
explanation panel. There is no duplicate hidden selector.

The selected concern is the central top node. The other concerns form a
two-column field of wrapped buttons. Only relationships touching the selected
concern are drawn. Curves connect it through the central gutter to related
concerns, avoiding unrelated cards. This neighbourhood view keeps labels readable
at mobile widths. Cards say “Selected concern”, “Related concern” or “Other
concern”, so colour is not the only signal. The selected relationships also appear
in a live text summary for screen readers.

Edges are **undirected related-concern pairs** from the union of supplied
`dependencies`: a link declared by either endpoint creates one pair, and
reciprocal declarations do not create duplicate edges. There are no arrowheads.
The supplied relation set is not described as an execution sequence or a DAG.
For the current seven-layer data this gives 12 distinct pairs:

```text
1–4  1–5  1–6  1–7
2–3  2–4
3–4
4–5  4–6
5–6  5–7
6–7
```

For example, selecting concern 6, Cryptographic Bedrock, shows relationships to
1, 4, 5 and 7. Its own dependency list is empty, but those links are supplied by
the other concerns. This is a rendering of the supplied relations, not an added
technical claim. Unknown references and self-links add no edges; missing arrays
act as empty arrays. An unknown `selected` value shows no selected node or edges.

SVG supplies decorative connecting strokes only. All names and labels remain
HTML text. A ResizeObserver recomputes stroke positions from the wrapped native
buttons; no fixed text-size assumption is used. Button targets exceed 44px,
body text is at least 15px, and no animation is needed to understand the graph.
The component calls no network, storage, crypto, SDK, wallet or ledger API.

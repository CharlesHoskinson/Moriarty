# TransactionDiagram

A React display component for a single settlement transaction. It renders spent
asset/state inputs, separate evidence inputs, the transaction boundary, and
produced outputs. At component widths below 900px the same reading order becomes
a vertical flow. No packages beyond React are required.

Copy `TransactionDiagram.jsx` and `transaction-diagram.css` together into the
consumer's source directory. The component imports its stylesheet. Use it at the
full available content width; putting it inside the old narrow diagram/inspector
split will correctly select the vertical layout.

```jsx
import React, {useState} from 'react';
import TransactionDiagram from './TransactionDiagram.jsx';

const invoice = {
  id: 'invoice-display-35',
  title: 'Pay 35 CASH and carry the remaining duties forward',
  inputs: [
    {
      id: 'escrow-before', label: 'Escrow before payment', kind: 'state',
      lines: ['Asset: 112 CASH', 'Datum: invoice owed 100 CASH', 'Datum: reserve duty 10 CASH'],
      checks: ['payment', 'fee', 'remaining'],
    },
    {
      id: 'claim-a', label: 'Illustrative claim output A', kind: 'evidence',
      lines: [
        'Chosen packaging: 1 Strand covers all 3 checks',
        '1 illustrative Issue transaction created this input',
        'Evidence funding is separate from the 112 CASH balance',
      ],
      checks: ['payment', 'fee', 'remaining'],
    },
  ],
  outputs: [
    {
      id: 'seller', label: 'Seller receives payment', kind: 'asset',
      lines: ['Asset: 35 CASH'], checks: ['payment'],
    },
    {
      id: 'program-fee', label: 'Program fee recipient', kind: 'asset',
      lines: ['Asset: 2 CASH'], checks: ['fee'],
    },
    {
      id: 'escrow-after', label: 'Successor escrow', kind: 'state',
      lines: ['Asset: 75 CASH', 'Datum: invoice owed 65 CASH', 'Datum: reserve duty 10 CASH'],
      checks: ['payment', 'fee', 'remaining'],
    },
  ],
  checks: [
    {
      id: 'payment', label: 'Payment calculation',
      explanation: 'The seller receives 35 CASH. The invoice owed falls from 100 to 65 CASH. The fee does not reduce the invoice duty.',
    },
    {
      id: 'fee', label: 'Separate program fee',
      explanation: 'The program fee recipient receives 2 CASH. Payment plus fee debits 37 CASH: 112 − 35 − 2 = 75 CASH in the successor escrow.',
    },
    {
      id: 'remaining', label: 'Remaining obligations',
      explanation: 'The successor records 65 CASH of unpaid invoice duty and the unchanged 10 CASH reserve duty. In this illustration those duties total 75 CASH.',
    },
  ],
  note: 'Illustrative display, not live ledger execution. The 112 CASH opening balance is a chosen backing example, not a demonstrated funding transaction. The 2 CASH program fee is separate from network fees; network fees and evidence min-ADA are not included in these CASH amounts. The reserve duty remains 10 CASH. One Strand and one Issue transaction are chosen evidence-packaging assumptions for this example; the three check buttons do not establish those counts.',
};

export default function InvoiceExample() {
  const [selectedCheck, setSelectedCheck] = useState('payment');
  return <TransactionDiagram model={invoice} selectedCheck={selectedCheck} onSelect={setSelectedCheck}/>;
}
```

The model contract is:

```ts
type DisplayNode = {
  id: string;       // Unique within its input or output list.
  label: string;
  kind: string;     // 'state', 'asset', or 'evidence'; other strings display literally.
  lines: string[];  // Values and datum fields supplied by the caller.
  checks: string[]; // IDs from model.checks associated with this entire node.
};
type TransactionModel = {
  id: string;
  title: string;
  inputs: DisplayNode[];
  outputs: DisplayNode[];
  checks: {id: string; label: string; explanation: string}[];
  note: string;     // Visible context below the figure: source status, assumptions,
                   // excluded fees/funding, and any example-specific limitations.
};
```

`selectedCheck` is the selected check ID. `onSelect(id)` requests a selection
change; the parent owns the state. Check buttons are native buttons with
`aria-pressed`, visible focus, and ordinary Tab/Enter/Space operation. Selection
updates a polite text explanation and marks matching cards with a visible
“Selected check” label, border and background. No information requires hover.

`checks` maps a **whole card** to a condition. Keep only related fields in that
card or split fields into separate nodes if you need finer highlighting. The
diagram does not derive statements, proofs, balances or acceptance from these
associations. Missing or unknown selection highlights nothing. Empty lists
render explicit empty messages. Check IDs must be unique, and node references
should name IDs actually present in `model.checks`.

Inputs are spent inputs of the displayed settlement, including evidence inputs.
This component does not model reference inputs, earlier Issue transactions,
mint/burn events, signatures or reclaim paths. Add those facts only through an
appropriate separate view or explanatory note. The example's selected evidence
packaging is an author-supplied illustration, not an asserted protocol rule.

The diagram calls no network, storage, wallet, signing, proving or ledger API.
Its input/output counts count supplied display nodes only. Its proof note stays
visible and never equates check count with proof or Issue-transaction count.
The source/status note is outside the figure and is always expanded.

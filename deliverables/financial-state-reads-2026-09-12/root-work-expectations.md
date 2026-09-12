# Independent work oracle for root's source

These counts apply to the source constructed by independent-probes.mjs, not to
an author's fixture with possibly different guard bodies.

The reviewed multiple-action fixture spends E41/N2 for repay and E43/N2 for
installment. Root replaces `pre.due` (one ReadPre) with
`magnitude(outstanding<Cash>("Due100"))` (scalar extraction, financial read,
Text identity: three nodes). Thus root repay E43/N2 uses45 and installment
E45/N2 uses47. Remaining work from256 is211 then164.

Root repay_remaining counts: nominal Let+read+identity3; payment Let+scalar+
ReadLocal3; positive Require+comparison+ReadLocal+literal4; balance guard6;
allowance guard6; NextWrite+Add+ReadPre+ReadLocal4; Transfer Emit8; Repay Emit7;
Ensure ordinary paid6. E47 plus N2 uses49, leaving115. Total spent141.

Root scalar read probe: NextWrite+scalar+read+ReadArg4, Transfer Emit+record+
four Text fields+ConstructAmount+literal8. E12 plus N1 uses13, leaving243.
A missing identity faults after the first four reductions and publishes no
candidate result; its source span is the financial read node.

Admission/index creation is bounded validation rather than expression runtime
work. No lookup/declaration/order-dependent debit is assumed. Separate reserve16
must survive every successful transition.

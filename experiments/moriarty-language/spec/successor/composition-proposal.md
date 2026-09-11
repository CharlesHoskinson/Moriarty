# Proposed composition of expression descriptors with the funded kernel

Status: **PROPOSAL**, not registered. No runtime implements this document, no
profile digest includes it, and no evidence record cites it. It closes one named
gap: the [expression contract](semantic-contract.md) ends at `ExpressionPrepared`
with typed operation descriptors, and the [funded repayment kernel](repayment-kernel.md)
begins at a closed packet of state and actions. Neither document states how a
descriptor becomes an action. This proposal states a mapping, one composition-layer
rejection code and a precedence order, and nothing else: it invents no financial
behavior, no new effect and no authority. Every financial rule remains the
kernel's.

## Domains

From the expression contract: a successful action yields
$\mathsf{ExpressionPrepared}(\mathit{post}_e,\ D,\ w_r)$ where $\mathit{post}_e$
is the pre-state overridden by staged ordinary writes, $D = d_1 \cdots d_n$ is
the descriptor list in emission order, and $w_r$ is the expression work remaining
after the statements and the Ensure suffix. This document also names $w_s$, the
expression work remaining after the last ordinary statement and before the
suffix; $w_0$ is the initial budget.
Each $d_i = \mathsf{Operation}_{O_i}(r_i)$ carries a declared operation name
$O_i$ and an operand record $r_i : \mathrm{Record}\langle R_{O_i}\rangle$.

From the kernel: an input is $\langle S, \vec a \rangle$ with $S$ the closed
`RepaymentState` (balances, allowances, obligations, used ids, work) and
$\vec a$ a nonempty action list of at most 128 `Transfer` or `Repay` actions.
The kernel returns $\mathsf{Prepared}(S', E)$ with post-state and one effect per
action, or $\mathsf{Rejected}(\mathit{code}, i)$ with $i$ the failing action
index or null.

Financial-class fields belong to the field registry $F$ like ordinary fields:
they are readable through `ReadPre` and only their writes are refused, and the
expression contract retains every unwritten field, financial records included,
in $\mathit{pre} \oplus W$. The two post-states therefore overlap wherever a
financial field exists in $\mathit{pre}$. In the post view of the composition,
$S'$ takes precedence: write $(\mathit{pre} \oplus W) \lhd S'$ for the record
whose ordinary fields come from $\mathit{pre} \oplus W$ and whose financial-class
fields are replaced by their projection from $S'$. The projection from the
kernel's balance, allowance and obligation records onto declared financial-class
fields is not defined by either component and is listed as open below.

## Descriptor mapping

Let $\mu$ map one descriptor to one kernel action. It is defined exactly on the
two operation names whose operand records match the kernel's closed action
shapes field for field, and undefined elsewhere.

```math
\begin{array}{rcl}
\mu\bigl(\mathsf{Operation}_{\mathsf{Transfer}}\{\mathit{id},\mathit{from},\mathit{to},\mathit{asset},\mathit{amount}\}\bigr)
 & = & \mathsf{Transfer}(\mathit{id},\mathit{from},\mathit{to},\mathit{asset},\mathit{amount}) \\[6pt]
\mu\bigl(\mathsf{Operation}_{\mathsf{Repay}}\{\mathit{allocationId},\mathit{transferId},\mathit{obligationId},\mathit{payer},\mathit{nominalAmount}\}\bigr)
 & = & \mathsf{Repay}(\mathit{allocationId},\mathit{transferId},\mathit{obligationId},\mathit{payer},\mathit{nominalAmount}) \\[6pt]
\mu(d) & = & \bot \qquad \text{otherwise}
\end{array}
```

Field values pass through unchanged. The Core has no identifier value type, so
the record fields that the kernel reads as identifiers (`id`, `from`, `to`,
`asset`, `allocationId`, `transferId`, `obligationId`, `payer`) must be declared
as `Text` whose decoded value satisfies the kernel's identifier syntax
`[A-Za-z][A-Za-z0-9_]{0,63}`; a `Text` value outside that syntax reaches the
kernel and rejects `INVALID_IDENTIFIER` with null index. Whether a dedicated
identifier type should be added to the Core instead is listed as open below.
`amount` and `nominalAmount` are `UInt128` and pass as canonical decimal text. The operand record types $R_{\mathsf{Transfer}}$
and $R_{\mathsf{Repay}}$ must be declared in the trusted schema with exactly
those field names and with the types the kernel admits; a schema that declares
an operation of either name with a different record shape makes $\mu$ undefined
for it. $\mu$ extends pointwise to lists and is undefined if any element is.

## Composition rule

The combined judgment runs the expression statements, hands the mapped
descriptors to the kernel, then runs the Ensure suffix against the combined
post-state. $\sigma_s$ is the frame after the last ordinary statement and
$\vec q$ the Ensure suffix.
$\sigma_s[\mathit{post} := X]$ abbreviates the frame in which $\mathsf{ReadPre}(\mathit{post}, f)$
reads $X[f]$; $\sigma$ has no separate post component, so the bracket names
only what the Ensure suffix sees.

```math
\frac{
\begin{array}{c}
\langle s_1;\ldots;s_m,\ \sigma_0,\ w_0\rangle \to^{*} \langle \varepsilon,\ \sigma_s,\ w_s\rangle
\qquad \sigma_s = \langle \mathit{pre}, W, L, A, O, D\rangle
\qquad \mu(D) = \vec a \neq \varepsilon \\[4pt]
\mathsf{kernel}(S, \vec a) = \mathsf{Prepared}(S', E)
\qquad \langle \vec q,\ \sigma_s[\mathit{post} := (\mathit{pre} \oplus W) \lhd S'],\ w_s\rangle \to^{*} \langle \varepsilon,\ \_,\ w_r\rangle
\end{array}
}{
\mathsf{Compose}(\mathit{pre}, S, \mathcal{A}) \to \mathsf{Prepared}\bigl((\mathit{pre} \oplus W) \lhd S',\ E,\ w_r\bigr)
}\ \text{(COMPOSE)}
```

```math
\frac{
\begin{array}{c}
\langle s_1;\ldots;s_m,\ \sigma_0,\ w_0\rangle \to^{*} \langle \varepsilon,\ \sigma_s,\ w_s\rangle
\qquad D = \varepsilon \\[4pt]
\langle \vec q,\ \sigma_s[\mathit{post} := \mathit{pre} \oplus W],\ w_s\rangle \to^{*} \langle \varepsilon,\ \_,\ w_r\rangle
\end{array}
}{
\mathsf{Compose}(\mathit{pre}, S, \mathcal{A}) \to \mathsf{ExpressionPrepared}(\mathit{pre} \oplus W,\ \varepsilon,\ w_r)}\ \text{(COMPOSE-PURE)}
```

An action that emits nothing never reaches the kernel and keeps today's
`ExpressionPrepared` result, with its Ensure suffix run exactly as the expression
contract specifies; the kernel's nonempty-actions rule is not violated by an
empty list because the kernel is not called. The kernel's post-state $S'$ carries
its own updated `work` record; the composed result adds no separate work field.

**Ordering.** Ordinary statements run first, in lexical order, exactly as the
expression contract specifies. Descriptors reach the kernel in emission order and
the kernel executes them in that order under its own transition rules. Only
after the kernel returns does the Ensure suffix run, and it sees a post view in
which ordinary fields come from $\mathit{pre} \oplus W$ and financial-class fields
from $S'$. This resolves the postcondition-visibility question the expression contract
leaves open: an Ensure can observe the financial outcome, but through `ReadPre`
with the `post` view only, so it still cannot write. Nothing is visible outside
the candidate at any intermediate point.

**Work.** Two budgets are charged separately and neither converts into the other.
Expression work is charged by E-ENTER as the contract specifies: $w_s$ remains
after the statements and $w_r$ after the Ensure suffix, which is entered with
$w_s$. Kernel work is charged
by the kernel's own rule: exactly one action-work unit per mapped action, taken
from `work.remaining` and added to `work.spent`, with `closureReserve` untouched.
A descriptor therefore costs one expression unit at its Emit occurrence plus one
kernel unit at its action, and the two never net against each other.

**Rejection precedence.** Failures resolve in this order, and every failure
publishes nothing: no post-state, no effects, no descriptors.

```math
\begin{array}{ll}
\langle s_1;\ldots;s_m,\ \sigma_0,\ w_0\rangle \to^{*} \mathsf{Rejected}(c, \mathit{sp}, p, w) & \text{(C-EXPR)} \\
\quad \Longrightarrow\ \mathsf{Compose} \to \mathsf{Rejected}(c, \mathit{sp}, p, w) & \\[6pt]
\mu(d_i) = \bot & \text{(C-UNMAPPED)} \\
\quad \Longrightarrow\ \mathsf{Compose} \to \mathsf{Rejected}(\mathtt{UNMAPPED\_OPERATION},\ \mathrm{span}(d_i),\ \mathrm{path}(d_i),\ w_0 - w_s) & \\[6pt]
\mathsf{kernel}(S, \vec a) = \mathsf{Rejected}(c, i) & \text{(C-KERNEL)} \\
\quad \Longrightarrow\ \mathsf{Compose} \to \mathsf{Rejected}(c,\ \mathrm{span}(d_i),\ \mathrm{path}(d_i),\ w_0 - w_s) & \\[6pt]
\langle \vec q,\ \ldots,\ w_s\rangle \to^{*} \mathsf{Rejected}(c, \mathit{sp}, p, w) & \text{(C-ENSURE)} \\
\quad \Longrightarrow\ \mathsf{Compose} \to \mathsf{Rejected}(c, \mathit{sp}, p, w) &
\end{array}
```

An expression rejection is reported before any kernel work, so a statically or
dynamically failing action never charges kernel work. An unmapped descriptor is
found before the kernel is called; its provenance is the Emit occurrence. A
kernel rejection with action index $i$ is attributed to the $i$-th descriptor's
Emit occurrence, so the source diagnostic points at the emission that produced
the failing action; a kernel rejection with null index, which the kernel gives to
input, state, work and action-list admission failures, is attributed to the
action root. Any rejection raised while the Ensure suffix reduces, whether
`ENSURES_FAILED`, `WORK_EXHAUSTED`, an arithmetic or index code or a bound,
keeps its own code and provenance and discards the kernel's tentative post-state
and effects exactly as it discards staged writes today.

`UNMAPPED_OPERATION` is the only new code and belongs to this composition layer,
not to either component. Kernel codes keep their kernel meaning and kernel index.

## Worked shape

A source action that stages one ordinary write, emits one Transfer descriptor
and then emits one Repay descriptor referencing that transfer's id maps to the
kernel's two-action funded repayment packet in exactly the order the
[K definition](../../formal/k/README.md) executes: Transfer then Repay. Its Ensure
suffix can then require, for example, that the obligation's outstanding amount
in the post view equals the expected remainder. That Ensure reads the kernel's
result; it does not recompute the discharge.

## What remains open

- The projection from kernel balance, allowance and obligation records onto
  declared financial-class fields, which $\lhd$ presupposes, is defined by
  neither component.
- Identifier-valued operand fields are carried as `Text` under the kernel's
  syntax; a dedicated Core identifier type is an alternative not chosen here.
- Kernel packets today carry two balance rows, one allowance, one obligation and
  empty used-id lists; a composed action touching more state needs the full
  kernel state shape, which the K projection does not yet cover.
- The trusted schema must declare `Transfer` and `Repay` operations with the
  kernel's exact operand records. No schema in the repository does so yet, and
  the source profile's roster confirmations do not authenticate one.
- Which party's authority signs the composed candidate, how effects enter
  history, and what the proof relation states about the composed transition are
  outside this document, as they are outside both components.
- Whether expression work and kernel work should share a single registered
  budget is a profile decision; this proposal keeps them separate because the
  two contracts define them separately.
- No correspondence is claimed between this rule and the TypeScript evaluator,
  the K definition or the Compact compiler. Adopting it requires a reviewed
  contract revision of both components and its own executable evidence.

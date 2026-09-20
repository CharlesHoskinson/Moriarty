from pathlib import Path
C=Path(__file__).parent/'candidate'
def patch(rel,pairs):
 p=C/rel;s=p.read_text()
 for a,b in pairs:
  assert a in s,(rel,a[:80]);s=s.replace(a,b)
 p.write_text(s)
patch('ROADMAP.md',[
('Current interfaces can support an early scoped milestone; their limitations do not reduce MC03/MC06.','U1 will determine whether currently released interfaces support an early scoped milestone; their limitations do not reduce MC03/MC06.'),
('numeric/price conventions; actual compiler','canonical price orientation, unit dimensions, per-primitive rounding direction and beneficiary policy in the numeric profile; K-reference reconciliation status; actual compiler'),
('caller preconditions; measured signature','caller preconditions and declared logical/target cost relation; measured signature'),
('separate-party private handoff and every retained composition operator;','separate-party private handoff and every retained composition operator demonstrated without the federated kernel;'),
('common ZK/MPC/TEE statement under separate assumptions; OWS delegation;','common ZK/MPC/TEE statement and authorized federation policy naming membership, threshold, ordering, equivocation, availability and epoch-change rules; epoch-migration test preserving duties and consumed authority (ZR15/UNI-015); OWS delegation;'),
('Two independent solvers may supply different authorized routes.','Two independently produced candidates may supply different authorized routes through direct local/manual authoring or standalone tools; this checks objective route neutrality without U5 infrastructure. U3 needs no federation. U5 separately qualifies live coordinated solver services on the same program.'),
])
patch('docs/MORIARTY-CONSOLIDATED-DESIGN.md',[
('Midnight will have comprehensive recursion in about six months,','Midnight is assumed by the user to have comprehensive recursion in about six months,'),
('These are logical obligations, not a claim','Source, Core, emitted ZKIR and verifier keys have distinct identities. Equality is required only for the same canonical representation; transformations are linked by pinned correspondence relations and commitments, not literal equality of their hashes. The enforcement map must also state whether signed-intent authentication occurs in the circuit, a bound ledger primitive or another explicitly justified native boundary.\n\nThese are logical obligations, not a claim'),
('Checked finite-width arithmetic, overflow, rounding and prices are explicit.','Checked finite-width arithmetic, overflow, rounding and prices are explicit. U0 freezes canonical price orientation, unit dimensions, per-primitive rounding direction and beneficiary policy in the numeric profile; U1 certificates bind that profile.'),
('Admission must establish a viable supported closure/recovery path','Under the signed policy, the program/ledger acceptance predicate for any claimed recovery guarantee must establish a viable supported closure/recovery path'),
('Ordinary authority can expire while a narrowly scoped recovery authority remains usable under its signed conditions.','Ordinary authority can expire while a narrowly scoped recovery authority remains usable under its signed conditions. Recovery grants declare their own signed termination rule: an expiry or owner-chosen indefinite duration with scoped revocation. Exclusive terminal outcomes consume/tombstone the applicable authority. Revocation cannot erase outstanding duties; unavailable recovery remains explicit or uses a separately authorized remedy. Optional kernel service eligibility can refuse service but is not program validity and cannot override Midnight acceptance.'),
('Native proof aggregation alone does not establish historical compliance.','The target bounds each stage and predecessor fan-in (at least two for MC06), not all history to one fixed-size DAG. It supports growing finite histories across stages under well-founded composition. Each signed episode retains its declared finite global work/authority limits; longer-lived continuation requires authenticated authorized successor episodes without resetting an existing signed lifetime budget. No literally infinite execution, unbounded per-stage work or automatic unlimited-depth machine counter is promised.\n\nNative proof aggregation alone does not establish historical compliance.'),
('For portable/off-ledger segments, private handoff, bounded split/join and imported histories, require','MC06 private handoff and split/join must be demonstrable between independently controlled participants without the federated kernel. Optional kernel-assisted joint proving is an additional profile with UNI-011 assumptions, not the sole language evidence.\n\nFor portable/off-ledger segments, private handoff, bounded split/join and imported histories, require'),
])
patch('docs/MORIARTY-BACKEND-REQUIREMENTS.md',[
('U0 freezes this contract; U1 investigates compatibility/cost;','U0 adopts this versioned requested contract; U1/U4 reconcile it with the released compatible tuple without silently weakening mandatory scope; U1 investigates compatibility/cost;'),
('the required equality and refinement links.','equality within the same canonical representation and pinned correspondence/refinement links where source, Core, ZKIR and ledger representations differ. Distinct representation hashes are expected; their authorized linkage is enforced.'),
('Mutate each component individually while retaining other valid objects: every unauthorized mismatch rejects.','Mutate each component individually while retaining other valid objects: every unauthorized mismatch of identity or its correspondence linkage rejects; mere inequality of different representation hashes is not a failure.'),
('Head/absence checks, entitlement uniqueness and complete state; U3/U4','Head/absence checks and unique single-lineage consumption at U2; completeness/concurrency/composition extensions at U3/U4'),
('Privacy policy, witness transfer and availability, separate-party tests; U4','Privacy policy, witness transfer and availability, separate-party tests without the federated kernel at U4; optional kernel-assisted profile under UNI-011 at U5'),
('and aggregate budgets without double-counting common ancestry.','and aggregate budgets without double-counting common ancestry. The per-stage fan-in bound does not impose a single fixed global DAG depth; growing finite well-founded histories preserve declared episode/lifetime budgets through authorized continuation.'),
])
# Target policy in EARS.
patch('openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md',[
('including exact units, price orientation and rounding.','including exact units, price orientation, rounding direction and beneficiary rules bound to the U0 numeric profile.'),
('A native port converts an exact positive quote/base ratio before directed rounding.','A native port converts an exact positive quote/base ratio before directed rounding under the bound U0 numeric profile.'),
])
p=C/'openspec/changes/consolidated-language-kernel/traceability.md';s=p.read_text()
s=s.replace('All five retained composition operators remain U4 scope.','All five retained composition operators remain U4 scope: sequential, disjoint parallel, shared-state interleaving, atomic synchronization, and asynchronous messaging, as defined in [SP10.1](../../sprints/sp10-private-handoff-and-bounded-composition.md). These are the inherited operator names, not a replacement list inferred from DeFiFormal.')
s=s.replace('| UNI-009 | specified-only |','| UNI-007,008,009 | specified-only |',1) if False else s
lines=s.splitlines()
for i,line in enumerate(lines):
 if '| [MPLR-007]' in line:lines[i]=line.replace('UNI-009','UNI-007,008,009')
 if any(f'| [MPLR-{n:03}]' in line for n in [11,13,27,28,29]):
  parts=lines[i].split('|');parts[-3]=parts[-3].strip()+',017 ';lines[i]='|'.join(parts)
s='\n'.join(lines)+'\n'
zr={1:('U0/U1','003,017','014,022','MC04'),2:('U4','009,017','027','MC03'),3:('U2/U4','002,003,017','018,022,023','MC04/MC05/SP09'),4:('U4','009,017','027','MC03'),5:('U2/U4','003,009,017','022,027','MC03/MC04'),6:('U1/U2/U4','003,014,017','014,020,023','MC01/MC04'),7:('U2/U4','002,009,017','023,027','MC05/SP09'),8:('U4','009,017','027','MC03/MC05'),9:('U4','009,017','007,011,027','MC06'),10:('U2/U3/U4','005,009,017','011,029','MC04/MC06'),11:('U2/U3/U4','003,006,017','014,017,024','MC04/MC05/SP09'),12:('U1/U3/U4','008,014,017','009,021','MC01/MC06'),13:('U1/U4','014,016,017','009,021','SP06/SP09'),14:('U4; optional U5','010,017','013,029,030','MC06'),15:('U4/U5','015,017','028','MC04/MC06'),16:('U4/U7','009,016,017','012,027','MC03/MC06/SP06/SP09')}
s+='''\n## Backend requirement crosswalk\n\nEvery ZR row is normative through UNI-017 and the specific UNI clauses below. MPLR numbers denote the existing full notes. No row closes through a forecast, opcode name or advisor vote.\n\n| ZR | U owner | UNI | MPLR | Retained acceptance | Status |\n|---|---|---|---|---|---|\n'''
for n,(u,uni,mplr,legacy) in zr.items():s+=f'| ZR{n:02} | {u} | UNI-{uni} | {mplr} | {legacy} | specified-only |\n'
p.write_text(s)
print('all final-review corrections applied to candidate')

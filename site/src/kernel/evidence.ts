/**
 * Evidence inspector fixtures. Each mechanism names what it establishes, the
 * assumptions that qualify it and the bypass a reader must keep in view.
 * Bindings are the fields every mechanism must agree on before combining
 * them adds anything. Destination profiles show where enforcement happens.
 */

export interface Mechanism {
  readonly id: 'zk' | 'threshold' | 'tee';
  readonly name: string;
  readonly claim: string;
  readonly assumptions: readonly string[];
  readonly doesNot: string;
}

export const MECHANISMS: readonly Mechanism[] = [
  {
    id: 'zk',
    name: 'Zero-knowledge proof',
    claim: 'The encoded relation holds for the bound public statement. Under the proof system’s zero-knowledge assumptions, the proof discloses no witness information beyond that statement and its intended disclosure.',
    assumptions: [
      'The proof system is sound under its cryptographic assumptions.',
      'Zero knowledge is a separate property from soundness. It hides the witness only relative to the public statement; public effects, metadata and authorized outputs remain visible as specified, and this is not an end-to-end confidentiality or witness-handoff guarantee.',
      'The circuit and verifier key are the ones the policy pinned.',
      'The relation actually expresses the financial meaning the author intended.',
    ],
    doesNot: 'It does not establish that an external chain executed, that a document is legally true, or that anyone will supply the next witness.',
  },
  {
    id: 'threshold',
    name: 'Threshold signing or MPC',
    claim: 'A signature over exactly these bytes verifies under the group key bound to this membership epoch. That is what the verifier checks. A signing threshold is not a threshold of honest signers: with t required shares and at most f corrupt, a valid signature needs at least t − f honest shares under the protocol assumptions, not t. That the honest participation the protocol requires occurred, and that those participants checked the policy first, is an inference under the named protocol and corruption bound, not a fact the signature itself records.',
    assumptions: [
      'At most the corruption bound of signers are corrupt or coerced, so a valid signature implies the honest participation the protocol requires, which may be fewer than the threshold.',
      'Key generation and refresh were honest and the membership epoch is the one bound here.',
      'Each honest signer checked the policy before signing rather than signing on request; generic MPC does not supply identifiable per-signer approvals.',
    ],
    doesNot: 'It does not prove the signed statement is true, and it does not by itself attribute an informed approval to any individual signer. It distributes signing power under a corruption model and nothing more.',
  },
  {
    id: 'tee',
    name: 'TEE attestation',
    claim: 'A report signed under the vendor root identifies the measured code and configuration that produced it, together with the report data the enclave chose to bind. That is the authenticated content.',
    assumptions: [
      'The hardware and its attestation root are not compromised.',
      'The attestation is fresh for this epoch and stage.',
      'Sealed state was not rolled back to an earlier snapshot.',
      'Any claim that the environment ran on this input and produced this output depends on the measured code binding its input and output into the report data, and on the verifier trusting that measured code and its verification policy.',
    ],
    doesNot: 'It does not by itself prove what was computed. It does not replace a missing program proof, and it says nothing about the code’s correctness beyond identifying it.',
  },
];

export interface Binding {
  readonly id: string;
  readonly field: string;
  readonly why: string;
  readonly ifMissing: string;
}

export const BINDINGS: readonly Binding[] = [
  { id: 'intention', field: 'Signed intention', why: 'Ties the evidence to what the owner authorized.', ifMissing: 'Evidence for a different intention could be presented for this one.' },
  { id: 'program', field: 'Program identity', why: 'Names the source, Core, ZKIR and verifier key by their distinct identities and their required correspondence.', ifMissing: 'A proof of a weaker relation could stand in for the pinned one.' },
  { id: 'domain', field: 'Settlement domain', why: 'Says which chain or issuer the effect concerns.', ifMissing: 'An effect on one domain could be mistaken for the same effect on another.' },
  { id: 'stage', field: 'Stage and logical request', why: 'Identifies which fill or attempt this evidence resolves.', ifMissing: 'One result could discharge two stages, or be replayed against a later one.' },
  { id: 'epoch', field: 'Epoch and membership', why: 'Fixes which signer set and which freshness window apply.', ifMissing: 'A retired signer set or a stale attestation could still count.' },
  { id: 'effects', field: 'Complete effects', why: 'Binds the actual gross debit, fees, recipient and receipt, not a convenient projection.', ifMissing: 'A prover could bind only the flattering part of what happened.' },
];

export interface Destination {
  readonly id: 'signature-only' | 'checking';
  readonly name: string;
  readonly enforces: readonly string[];
  readonly bypass: string;
  readonly localLimit: string;
  readonly hypothetical: boolean;
}

export const DESTINATIONS: readonly Destination[] = [
  {
    id: 'signature-only',
    name: 'Signature-only foreign account',
    enforces: ['A valid threshold signature over the transfer bytes.'],
    bypass: 'Compromise the toy 3-of-5 threshold and any transfer the corrupt signers assemble is honored. The honest signers’ policy checks never ran at the destination.',
    localLimit: 'A local exclusive-state rule on Midnight cannot retroactively stop that foreign transfer. It can only refuse to treat it as this agreement’s accepted stage.',
    hypothetical: false,
  },
  {
    id: 'checking',
    name: 'Hypothetical destination with named checks',
    enforces: [
      'A valid threshold signature over the transfer bytes.',
      'A verifier for one named relation over the bound statement.',
      'A consumption rule that refuses a second effect for the same stage identifier.',
    ],
    bypass: 'Anything the three named checks do not cover. There is no generic proof-enforcing chain that checks every Moriarty property; each destination enforces exactly what it enforces.',
    localLimit: 'Midnight still checks only its own relation and its own ledger rules. The destination’s checks are the destination’s.',
    hypothetical: true,
  },
];

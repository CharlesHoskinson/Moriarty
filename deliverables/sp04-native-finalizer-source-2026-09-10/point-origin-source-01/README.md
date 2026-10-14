# Dynamic point origins and canonical input boundary

This bounded source investigation resolves the point-origin categories and count
formulas for two explicitly separate source versions. It does not select a
backend, SRS, circuit architecture, proof allocation or accepted finalizer.
The previous [subgroup relation](../SUBGROUP-RESULT.md) remains unchanged; its
368,188 per-point conditional subtotal is reused only as a conditional unit cost.

Sources are retained with original paths and digests in [source-evidence.json](source-evidence.json).
`native/` below means pinned Git `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`
(retained under sources/native-695351); `7.2.4/` means the exact cached
midnight-circuits archive, paired for source inspection with proofs0.8.2 and
curves0.3.1. These are different implementations: the native Git circuits package
calls itself7.1.0 and uses LogUp/multi-commitment interfaces. Package version names
alone must not select a point count. All inspected installed members matched
cached archives; Git sources were read at the pinned commit.

## Closed proof-point roster

Let A be advice columns, L lookup arguments, P permutation columns, d the exact
constraint-system degree (d>2), T trash polynomials and H actual quotient
commitments. Let c_i be native LogUp batch i's num_chunks(cs_degree). These are
fixed by the exact architecture/VK/features, not by prover-supplied list lengths.

| Origin | Installed7.2.4 count | Pinned native count | First source boundary |
| --- | ---: | ---: | --- |
| Advice commitments | A | A | verifier_gadget.rs:334–338 / native:350–355 |
| Lookup permuted input/table | 2L | 0 | 7.2.4 lookup.rs:56–66 |
| Lookup product | L | 0 | 7.2.4 lookup.rs:69–79 |
| LogUp multiplicities | 0 | L | native lookup.rs:62–74 |
| LogUp helpers and aggregator | 0 | sum(c_i)+L | native lookup.rs:77–106 |
| Permutation products | ceil(P/(d−2)) | same | permutation.rs:56–74 / native:56–78 |
| Trash commitments | T | T | 7.2.4 trash.rs:42–48; native argument.rs:79–89 + transcript_gadget.rs:149–173 |
| Random vanishing commitment | 1 | 0 | 7.2.4 vanishing.rs:47–55 |
| Quotient pieces | H | H | 7.2.4 vanishing.rs:59–72; native verifier_gadget.rs:531–561 |
| KZG batch commitment f | 1 | 1 | 7.2.4 kzg.rs:385; native kzg.rs:561–563 |
| KZG opening proof π | 1 | 1 | 7.2.4 kzg.rs:457–458; native kzg.rs:640–642 |

Thus N_724=A+3L+ceil(P/(d−2))+T+H+3 and
N_native=A+2L+sum(c_i)+ceil(P/(d−2))+T+H+2.
Native read_commitment reads one point per label, **not one per batch object**;
the trash group therefore still contributes T, including0 for an empty group.
Native H is1 under single-h-commitment, otherwise the exact domain quotient
polynomial degree. Native c_i is the count of chunks of input_expressions at the
positive chunk size computed in proofs/src/plonk/logup.rs:223–238,316–319; absent
or invalid architecture parameters do not justify a numerical count. The installed
7.2.4 parse_trace additionally asserts one advice phase (verifier_gadget.rs:330–332).
No formula here replaces that implementation precondition.

## Other point origins and checks

| Point source | Dynamic check count under the conservative subgroup-domain contract | Required binding |
| --- | --- | --- |
| Every proof point above | N for the selected exact version | Same assigned point cells must feed transcript absorption, query commitment, resulting MSM and subgroup relation. Host proof parsing alone is not a constraint. |
| Caller committed-instance points | C, the number of fresh untrusted constituent points | Each must bind to the actual public instance commitment and enter the transcript before advice commitments. The pinned IVC wrapper uses a circuit-constructed zero commitment, so C=0 there; this is not a general assumption for other callers. |
| Incoming carried accumulator variable bases | R_lhs+R_rhs | Exact ordered bases, corresponding scalars, fixed-base identities and state/VK instance must be bound. The pinned IVC collapsed shape has one variable base per side, so R=2. |
| VK fixed commitments, permutation fixed commitments, −G | 0 repeated dynamic checks **only if** exact immutable constants have independently established membership and identity/label binding | Fixed map includes each actual VK commitment and −G; replacing named constants by caller points invalidates this exemption. No selected SRS/VK constant manifest is established here. |
| Two evaluated final MSM results | No additional fresh-input subgroup checks when derived by constrained correct arithmetic from checked bases/constants | Both results must be the actual fully combined accumulator, not two unrelated supplied points. If assigned as independent witnesses instead, both need domain checks **and** exact MSM equality constraints. |
| Intermediate addition/MSM points | No separate membership checks when correct complete operations propagate subgroup membership | This is conditional on constrained arithmetic and its preconditions; the existing incomplete arithmetic is not newly certified by this origin table. |
| Application T::State points | Outside this generic verifier's closed interface | T supplies format_public_input/decider. No complete application-point count follows without the actual application schema. Their values cannot be silently counted as checked proof points. |

A conservative input-domain schedule therefore has N+C+R membership checks, with
R=2,C=0 for the pinned collapsed IVC step, plus any separately assigned application
points. It is a sufficient proposed strategy for the reviewed complete-group
premises, **not a proof that upstream PLONK inherently needs every subgroup check**.
The upstream SelfEmulation documentation claims out-of-subgroup inputs give no
advantage but retains a formal-analysis TODO (7.2.4 types.rs:127–138). This report
does not resolve that cryptographic claim by assumption. Multiplying N+C+R by
368,188 gives only the inherited conditional membership subtotal; it does not give
whole-verifier fit, include transcript/canonical decoding, or authorize F0 go.

## Byte decoding, identity and public representation

The host Poseidon Hashable reader consumes exactly48 compressed G1 bytes through
G1Affine::from_bytes (7.2.4 hash/poseidon/poseidon_cpu.rs:207–225). Curves0.3.1
routes that to BLST decompression and then is_torsion_free
(g1.rs:370–386,401–403,763–768). BLST0.3.16 and cached0.3.17 have the inspected
same decisive rules at e1.c:237–294: compression bit set; infinity exactly
`c0` followed by47 zero bytes; finite x strictly below p; on-curve square root;
encoded x=0 rejected; then the Rust checked wrapper enforces subgroup membership.
Identity is allowed. A host length/error rejection is distinct from circuit
acceptance.

The gadget does not constrain that byte decoder: read_point/read_commitment
turns a host read error into default identity and assigns without subgroup check
(7.2.4 transcript_gadget.rs:134–160; native:142–173). Its initialization explicitly
does not constrain trailing proof bytes. The native external IVC verifier does
call transcript.assert_empty (aggregation/src/ivc/verifier.rs:75), after preparing
the proof. A faithful byte-bound finalizer must specify strict decoding and
exhaustion, or explicitly bind a decoded abstract proof under a separately checked
canonical encoding. It cannot silently claim that the existing gadget proves a
raw proof-byte hash or rejects malformed bytes.

Poseidon absorbs the point's field representation, not compressed bytes. For the
seven56-bit shifted limbs, Fq capacity permits four limbs per packed word, so each
Fp coordinate exports two words and a point exports five Fq elements:
`pack((x−1) mod p), pack((y−1) mod p), is_id`.
The canonical identity uses x=y=0, hence both packed coordinates encode p−1,
followed by1. The circuit point type explicitly allows arbitrary x/y when is_id
(7.2.4 weierstrass_chip.rs:174–179); as_public_input exports those coordinates
without masking them (399–411), although each field is normalized (field_chip.rs:
509–528). Therefore subgroup identity acceptance alone does not enforce the
canonical identity used by the host transcript. The reviewed subgroup relation's
explicit identity-coordinate constraints are relevant here.

The convenience from_public_input returns identity whenever the last field is1
**before length/coordinate validation** (weierstrass_chip.rs:250–270). In addition,
the field decoder reconstructs integers modulo p after limb-width checks
(field_chip.rs:141–167); p−1 and2p−1 both reconstruct zero. It is not a strict
canonical export validator. This is a source-level discriminator, not a claim of
an observed deployed exploit or that these convenience decoders are on a public
network input path.

## Transcript, carried state and final pairing

The proof transcript starts with assigned VK transcript_repr, committed instances,
then lengths and normal instance fields before advice points. KZG combines the
actual queried commitments and f into RHS C−vG+zπ, with π as LHS
(7.2.4 kzg.rs:457–476; native kzg.rs:640 onward). Do not subgroup-check just π
and ignore the other bases because the final pairing has two terms.

Native IVC format_instance includes vk_repr, application state and the full
accumulator public-field encoding (circuit.rs:128–135). Its in-circuit step assigns
the collapsed previous accumulator, includes it in the previous proof's public
input, prepares that proof, conditionally zeros only the proof accumulator for
genesis, accumulates with prev_acc, collapses and exports the result
(circuit.rs:165–222). External verification checks canonical VK equality and the
application decider, prepares the new proof, requires transcript exhaustion, then
accumulates proof_acc with instance.acc before final pairing (verifier.rs:48–87).

Accumulator accumulation hashes ordered accumulator public fields to derive its
combining scalar. MSM public encoding contains ordered point fields and scalars;
it does not serialize the structural labels needed to reconstruct the MSM
(native msm.rs:351–368; 7.2.4 msm.rs:230–245). Native Fixed bases export no point
fields (native msm.rs:96–110). Thus exact shape/label order must come from the
committed verifier/IVC ABI; a flat Fq vector cannot be treated as a self-describing
carried accumulator. The generic IvcInstance is a typed object, and no canonical
standalone byte decoder for it was found in these inspected entry files. This is
the precise remaining wire-interface obligation, not permission to invent one.

Finally DualMSM::check evaluates both sides and pairs with s_g2_prepared and
n_g2_prepared, accepting final-exponentiation identity (proofs0.8.2 msm.rs:294–309).
These are inner SRS constants. Subgroup/byte checks neither bind those constants
to the intended SRS nor replace the existing app decider, semantic or Midnight
acceptance gates.

## Actual bounded checks and next concrete input

`python3 check-boundaries.py` verified37 retained source pins and passed eight
compressed-prefix boundary cases, three malformed public-identity aliases,
foreign-field modulo aliases and two independently expanded count examples.
Those examples are illustrative shapes, not observed circuit metadata. No Rust,
BLST, circuit, proof, backend or network process ran. The previous complete-add
relation was not re-derived or changed.

The next concrete requirement is an exact source-bound architecture/VK/features
and carried-accumulator ABI manifest: A,L,P,d,T,H,c_i,C,R; label/point/scalar order;
canonical point/identity exports; strict proof-byte exhaustion; same-cell bindings
from decoding through transcript/MSM to the two final results. Without that
specific interface, a numerical whole-verifier count or canonical IvcInstance
byte conformance claim is unavailable. This result closes the category/formula
question and identifies the missing input precisely; it creates no new runner
or resource gate.

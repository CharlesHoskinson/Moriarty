# Pinned public crate memory-layout excerpts
> Source: https://docs.rs/crate/midnight-curves/0.3.1/source/src/bls12_381/g1.rs
> Collected: 2026-10-26
> Published: Unknown

**SOURCE:** Read from the local Cargo registry cache of public crates; filenames and line spans below identify each excerpt.

## midnight-curves-0.3.1/src/bls12_381/mod.rs:24-36
SHA-256 `b4a834244017829ff066d26797f0b61f223e7f4d7f3e33ab2a7ceccd658bebe2`
```rust
use pairing::{Engine, MultiMillerLoop};

/// BLS12-381 pairing engine.
#[derive(Debug, Copy, Clone)]
pub struct Bls12;

impl Engine for Bls12 {
    type Fr = Fq;
    type G1 = G1Projective;
    type G1Affine = G1Affine;
    type G2 = G2Projective;
    type G2Affine = G2Affine;
    type Gt = Gt;
```

## midnight-curves-0.3.1/src/bls12_381/g1.rs:463-473
SHA-256 `37fedeaa9d4624428dace605bd616a41cafa85751f60557e55f0c490fba495b0`
```rust
    }
}

/// This is an element of $\mathbb{G}_1$ represented in the projective
/// coordinate space.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct G1Projective(pub(crate) blst_p1);

impl fmt::Debug for G1Projective {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
```

## midnight-curves-0.3.1/src/bls12_381/fq.rs:22-31
SHA-256 `f12967e1d672175304ca4cac302a19e1e50d13d030759bd4a81003660174e874`
```rust
/// Represents an element of the scalar field Fq of the BLS12-381 elliptic
/// curve construction.
///
/// The inner representation `blst_fr` is stored in Montgomery form as
/// little-endian `u64` limbs.
#[derive(Default, Clone, Copy)]
#[repr(transparent)]
pub struct Fq(pub(crate) blst_fr);

// GENERATOR = 7 (multiplicative generator of r-1 order, that is also quadratic
```

## midnight-proofs-0.8.3/src/plonk/permutation.rs:136-170
SHA-256 `dda74e51e5713f4766cbe6796dc51a9f3906dfd5f4189f37460d728900fba374`
```rust

/// The proving key for a single permutation argument.
#[derive(Clone, Debug)]
pub(crate) struct ProvingKey<F: PrimeField> {
    pub(crate) permutations: Vec<Polynomial<F, LagrangeCoeff>>,
    pub(crate) polys: Vec<Polynomial<F, Coeff>>,
    pub(crate) cosets: Vec<Polynomial<F, ExtendedLagrangeCoeff>>,
}

impl<F: WithSmallOrderMulGroup<3> + SerdeObject> ProvingKey<F> {
    /// Reads proving key for a single permutation argument from buffer using
    /// `Polynomial::read`.
    pub(super) fn read<R: io::Read>(
        reader: &mut R,
        format: SerdeFormat,
        domain: &EvaluationDomain<F>,
        p: &Argument,
    ) -> io::Result<Self> {
        let permutations = read_polynomial_vec(reader, format)?;
        let (polys, cosets) = compute_polys_and_cosets::<F>(domain, p, &permutations);
        Ok(ProvingKey {
            permutations,
            polys,
            cosets,
        })
    }

    /// Writes proving key for a single permutation argument to buffer using
    /// `Polynomial::write`.
    pub(super) fn write<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        write_polynomial_slice(&self.permutations, writer)?;
        Ok(())
    }
}

```

**SOURCE:** `blst-0.3.17/src/bindings.rs` defines blst_fr as four u64 limbs, blst_fp as six u64 limbs, blst_p1 as x/y/z blst_fp, blst_p1_affine as x/y blst_fp, and blst_p2 as x/y/z blst_fp2. SHA-256 `8b0c44ba87b3493fb47f8aeacafa0d5aa278a2a8b5932a735f2d5cbc888bc386`.

**ESTIMATE:** Scalar storage 32 B, projective G1 144 B, affine G1 96 B, projective G2 288 B from those layouts; allocator/capacity overhead excluded.

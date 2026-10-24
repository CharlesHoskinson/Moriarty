//! Private host materialization of the numeric portion of the selected A aux/2
//! design. This computes honest columns; it emits no constraints or wire ABI.
//! The aux/1 prefix, G4 requirements, IO1–8 and C4 are unchanged.

/// Only the arithmetic tags of ARH-0 (0, 1, 2, 4), not the language operator ABI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Lt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct AddWitness {
    pub r: [u8; 16],
    /// c[0] is the fixed zero endpoint; supplied columns are c[1..17].
    pub c: [bool; 17],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SubWitness {
    pub r: [u8; 16],
    /// bo[0] is the fixed zero endpoint; supplied columns are bo[1..17].
    pub bo: [bool; 17],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct MulWitness {
    pub r: [u8; 16],
    pub h: [u8; 16],
    /// Fixed c[0]=c[32]=0 endpoints; supplied columns are c[1..32].
    /// Honest carries are at most 4079 (the design admits unsigned 12 bits).
    pub c: [u16; 33],
    pub h_sum: u16,
    pub h_zero: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ArithmeticWitness {
    pub sel_add: bool,
    pub sel_sub: bool,
    pub sel_mul: bool,
    pub sel_lt: bool,
    pub uu: bool,
    pub ovf: bool,
    pub lt_out: bool,
    pub add: AddWitness,
    pub sub: SubWitness,
    pub mul: MulWitness,
}

/// Compute ARH-0/3/5/6 and ADD/SUB/MUL numeric columns from guarded inputs.
///
/// The caller supplies Some(op) exactly when en_bin holds and FS0.binaryOp is
/// Add/Sub/Mul/Lt; otherwise None (including Eq/Cons/Branch). Under INV-TY,
/// Some(bytes) means the fetched node's typeId is 0. Left is the Right-frame
/// FS0.valueRef operand; right is CR0.valueRef. Fetch, range/tag validity,
/// ARH-1/2 header materialization and binding remain the consumer's obligations.
/// These options are host inputs, not additional supplied witness columns.
pub(super) fn materialize_arithmetic(
    op: Option<ArithmeticOp>,
    left: Option<&[u8; 16]>,
    right: Option<&[u8; 16]>,
) -> ArithmeticWitness {
    let mut out = ArithmeticWitness {
        sel_add: op == Some(ArithmeticOp::Add),
        sel_sub: op == Some(ArithmeticOp::Sub),
        sel_mul: op == Some(ArithmeticOp::Mul),
        sel_lt: op == Some(ArithmeticOp::Lt),
        uu: false,
        ovf: false,
        lt_out: false,
        add: AddWitness {
            r: [0; 16],
            c: [false; 17],
        },
        sub: SubWitness {
            r: [0; 16],
            bo: [false; 17],
        },
        mul: MulWitness {
            r: [0; 16],
            h: [0; 16],
            c: [0; 33],
            h_sum: 0,
            h_zero: false,
        },
    };
    let (Some(op), Some(a), Some(b)) = (op, left, right) else {
        return out;
    };
    out.uu = true;
    match op {
        ArithmeticOp::Add => {
            for i in 0..16 {
                let total = u16::from(a[i]) + u16::from(b[i]) + u16::from(out.add.c[i]);
                out.add.r[i] = (total % 256) as u8;
                out.add.c[i + 1] = total >= 256;
            }
            out.ovf = out.add.c[16];
        }
        ArithmeticOp::Sub | ArithmeticOp::Lt => {
            for i in 0..16 {
                let subtrahend = u16::from(b[i]) + u16::from(out.sub.bo[i]);
                out.sub.bo[i + 1] = u16::from(a[i]) < subtrahend;
                // Rearranged SUB-i, always in 0..=255 after the borrow choice.
                out.sub.r[i] =
                    (u16::from(a[i]) + 256 * u16::from(out.sub.bo[i + 1]) - subtrahend) as u8;
            }
            out.ovf = out.sel_sub && out.sub.bo[16];
            out.lt_out = out.sel_lt && out.sub.bo[16];
        }
        ArithmeticOp::Mul => {
            // 32 diagonals, including S_31=0: no truncated u128 product.
            // Each sum <= 16*255*255=1_040_400; u32 also holds its carry.
            for k in 0..32 {
                let mut total = u32::from(out.mul.c[k]);
                for i in 0..16 {
                    if k >= i && k - i < 16 {
                        total += u32::from(a[i]) * u32::from(b[k - i]);
                    }
                }
                let digit = (total % 256) as u8;
                if k < 16 {
                    out.mul.r[k] = digit;
                } else {
                    out.mul.h[k - 16] = digit;
                }
                out.mul.c[k + 1] = (total / 256) as u16;
            }
            out.mul.h_sum = out.mul.h.iter().map(|&x| u16::from(x)).sum();
            out.mul.h_zero = out.mul.h_sum == 0;
            out.ovf = !out.mul.h_zero;
        }
    }
    out
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FuelWitness {
    pub z: [bool; 16],
    /// zp[0]=en_evt; supplied FuelProbe prefix columns are zp[1..17].
    pub zp: [bool; 17],
    /// bw[0]=en_chg; supplied FuelDec columns are bw[1..17].
    pub bw: [bool; 17],
    /// Host results of the normative formula and FD-2, not extra columns.
    pub charged: bool,
    pub post: [u8; 16],
}

/// Compute FP-1/2 and FD-1/2 using §5's rule-agnostic en_chg formula.
/// Caller derives en_evt=isPend*nz_n and term=tRet0+tErr from valid pre tags;
/// tRet0 and tErr are disjoint. There is no charge-by-rule inventory (E1/E-FUEL).
/// Free publications precede exhaustion and preserve fuel, including fuel zero.
pub(super) fn materialize_fuel(en_evt: bool, term: bool, pre: &[u8; 16]) -> FuelWitness {
    let mut out = FuelWitness {
        z: [false; 16],
        zp: [false; 17],
        bw: [false; 17],
        charged: false,
        post: *pre,
    };
    out.zp[0] = en_evt;
    for i in 0..16 {
        out.z[i] = en_evt && pre[i] == 0;
        out.zp[i + 1] = out.zp[i] && out.z[i];
    }
    out.charged = en_evt && !term && !out.zp[16];
    out.bw[0] = out.charged;
    for i in 0..16 {
        out.bw[i + 1] = out.bw[i] && out.z[i];
        out.post[i] =
            (u16::from(pre[i]) + 256 * u16::from(out.bw[i + 1]) - u16::from(out.bw[i])) as u8;
    }
    out
}

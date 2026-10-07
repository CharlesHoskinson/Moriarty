//! The built-in calls and the arguments each one takes.
//!
//! Every argument is required and no others are allowed. Relations between
//! arguments, such as distinct transfer endpoints, are checked separately.

use mori_lexer::TokenKind;

/// What kind of value an argument must be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Scalar,
    /// A scalar or a quantity.
    Numeric,
    Quantity,
    QuantityList,
    String,
    Asset,
    Domain,
    Account,
    Obligation,
    Pool,
    Instrument,
    Observation,
    Policy,
    ShareClass,
}

impl Role {
    /// The declaration keyword an entity argument must have been declared with.
    pub fn entity_kind(self) -> Option<TokenKind> {
        Some(match self {
            Self::Asset => TokenKind::KwAsset,
            Self::Domain => TokenKind::KwDomain,
            Self::Account => TokenKind::KwAccount,
            Self::Obligation => TokenKind::KwObligation,
            Self::Pool => TokenKind::KwPool,
            Self::Instrument => TokenKind::KwInstrument,
            Self::Observation => TokenKind::KwObservation,
            Self::Policy => TokenKind::KwPolicy,
            Self::ShareClass => TokenKind::KwShareClass,
            Self::Scalar | Self::Numeric | Self::Quantity | Self::QuantityList | Self::String => {
                return None;
            }
        })
    }

    /// How the role reads in a sentence, such as "an account".
    pub fn describe(self) -> &'static str {
        match self {
            Self::Scalar => "a whole number",
            Self::Numeric => "a number or an amount",
            Self::Quantity => "an amount",
            Self::QuantityList => "a list of amounts",
            Self::String => "a string",
            Self::Asset => "an asset",
            Self::Domain => "a domain",
            Self::Account => "an account",
            Self::Obligation => "an obligation",
            Self::Pool => "a pool",
            Self::Instrument => "an instrument",
            Self::Observation => "an observation",
            Self::Policy => "a policy",
            Self::ShareClass => "a share class",
        }
    }

    /// How the role reads in a signature, such as `account`.
    pub fn signature(self) -> &'static str {
        match self {
            Self::Scalar => "number",
            Self::Numeric => "number or amount",
            Self::Quantity => "amount",
            Self::QuantityList => "[amount]",
            Self::String => "string",
            Self::Asset => "asset",
            Self::Domain => "domain",
            Self::Account => "account",
            Self::Obligation => "obligation",
            Self::Pool => "pool",
            Self::Instrument => "instrument",
            Self::Observation => "observation",
            Self::Policy => "policy",
            Self::ShareClass => "share_class",
        }
    }
}

pub struct Call {
    pub name: &'static str,
    pub args: &'static [(&'static str, Role)],
}

impl Call {
    /// The call with its arguments, such as `rounds(domain: domain, ...)`.
    pub fn signature(&self) -> String {
        let args: Vec<String> = self
            .args
            .iter()
            .map(|(name, role)| format!("{name}: {}", role.signature()))
            .collect();
        format!("{}({})", self.name, args.join(", "))
    }

    pub fn role(&self, arg: &str) -> Option<Role> {
        self.args
            .iter()
            .find(|(name, _)| *name == arg)
            .map(|&(_, role)| role)
    }
}

pub fn find(name: &str) -> Option<&'static Call> {
    CALLS.iter().find(|call| call.name == name)
}

pub const CALLS: &[Call] = &[
    Call {
        name: "atoms",
        args: &[("asset", Role::Asset), ("value", Role::Scalar)],
    },
    Call {
        name: "min",
        args: &[("a", Role::Numeric), ("b", Role::Numeric)],
    },
    Call {
        name: "max",
        args: &[("a", Role::Numeric), ("b", Role::Numeric)],
    },
    Call {
        name: "rounds",
        args: &[
            ("domain", Role::Domain),
            ("from", Role::Scalar),
            ("to", Role::Scalar),
        ],
    },
    Call {
        name: "transfer",
        args: &[
            ("from", Role::Account),
            ("to", Role::Account),
            ("fee_to", Role::Account),
            ("value", Role::Quantity),
            ("fee", Role::Quantity),
        ],
    },
    Call {
        name: "repay",
        args: &[
            ("obligation", Role::Obligation),
            ("payer", Role::Account),
            ("amount", Role::Quantity),
        ],
    },
    Call {
        name: "amm.swap_exact_input",
        args: &[
            ("pool", Role::Pool),
            ("owner", Role::Account),
            ("input", Role::Quantity),
            ("output_asset", Role::Asset),
            ("net_floor", Role::Quantity),
            ("fee_cap", Role::Quantity),
        ],
    },
    Call {
        name: "amm.redeem",
        args: &[
            ("pool", Role::Pool),
            ("owner", Role::Account),
            ("share_atoms", Role::Scalar),
        ],
    },
    Call {
        name: "amm.mint",
        args: &[
            ("pool", Role::Pool),
            ("owner", Role::Account),
            ("amounts", Role::QuantityList),
            ("minimum_share_atoms", Role::Scalar),
        ],
    },
    Call {
        name: "lending.originate",
        args: &[
            ("obligation", Role::Obligation),
            ("debtor", Role::Account),
            ("creditor", Role::Account),
            ("principal", Role::Quantity),
            ("collateral", Role::Quantity),
        ],
    },
    Call {
        name: "lending.liquidate",
        args: &[("obligation", Role::Obligation)],
    },
    Call {
        name: "lending.roll_forward",
        args: &[("obligation", Role::Obligation), ("to_round", Role::Scalar)],
    },
    Call {
        name: "stablecoin.mint",
        args: &[
            ("instrument", Role::Instrument),
            ("owner", Role::Account),
            ("supply", Role::Quantity),
            ("backing", Role::Quantity),
        ],
    },
    Call {
        name: "stablecoin.redeem",
        args: &[
            ("instrument", Role::Instrument),
            ("owner", Role::Account),
            ("burn", Role::Quantity),
            ("minimum_backing", Role::Quantity),
        ],
    },
    Call {
        name: "stablecoin.emergency_settle",
        args: &[
            ("instrument", Role::Instrument),
            ("owner", Role::Account),
            ("claim", Role::Quantity),
        ],
    },
    Call {
        name: "option.fix",
        args: &[
            ("instrument", Role::Instrument),
            ("observation", Role::Observation),
        ],
    },
    Call {
        name: "option.exercise",
        args: &[("instrument", Role::Instrument), ("holder", Role::Account)],
    },
    Call {
        name: "option.settle",
        args: &[
            ("instrument", Role::Instrument),
            ("holder", Role::Account),
            ("payoff", Role::Quantity),
        ],
    },
    Call {
        name: "oracle.select",
        args: &[("observation", Role::Observation)],
    },
    Call {
        name: "governance.queue",
        args: &[("policy", Role::Policy), ("next_epoch", Role::Scalar)],
    },
    Call {
        name: "governance.execute",
        args: &[("policy", Role::Policy)],
    },
    Call {
        name: "governance.veto",
        args: &[("policy", Role::Policy)],
    },
    Call {
        name: "bridge.escrow",
        args: &[
            ("owner", Role::Account),
            ("amount", Role::Quantity),
            ("destination", Role::Domain),
            ("claim_id", Role::String),
        ],
    },
    Call {
        name: "bridge.claim",
        args: &[
            ("owner", Role::Account),
            ("amount", Role::Quantity),
            ("source", Role::Domain),
            ("claim_id", Role::String),
        ],
    },
    Call {
        name: "bridge.recover",
        args: &[
            ("owner", Role::Account),
            ("amount", Role::Quantity),
            ("claim_id", Role::String),
        ],
    },
    Call {
        name: "staking.deposit",
        args: &[
            ("owner", Role::Account),
            ("shares", Role::ShareClass),
            ("backing", Role::Quantity),
        ],
    },
    Call {
        name: "staking.reward",
        args: &[("shares", Role::ShareClass), ("amount", Role::Quantity)],
    },
    Call {
        name: "staking.slash",
        args: &[("shares", Role::ShareClass), ("amount", Role::Quantity)],
    },
    Call {
        name: "staking.unbond",
        args: &[
            ("owner", Role::Account),
            ("shares", Role::ShareClass),
            ("share_atoms", Role::Scalar),
        ],
    },
    Call {
        name: "staking.withdraw",
        args: &[
            ("owner", Role::Account),
            ("shares", Role::ShareClass),
            ("share_atoms", Role::Scalar),
            ("minimum_backing", Role::Quantity),
        ],
    },
];

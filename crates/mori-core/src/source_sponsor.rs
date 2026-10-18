//! Exact-source local sponsorship, with trusted unsigned local accounting.
//! Primitive work is retained on failure; only uncommitted economic holds roll back.
//! Parsing, hashing, validation, cloning and normalization are outside the cost profile.
use crate::intent::{canonical, hash_bytes};
use crate::policy::FeeCapability;
use crate::{
    CostReport, Error, Execution, LANGUAGE, LOGICAL_COST_PROFILE, SEMANTICS, Scenario, UInt, parse,
    simulate_metered, validate,
};
use serde_json::json;

/// Binding domain for the local exact-source consumer.
pub const SOURCE_SPONSOR_PROFILE: &str = "moriarty-local-source-sponsor/1";
/// Constant profile identities and the exact raw UTF-8 source commitment.
/// Diagnostic clones carry no executable authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceBinding {
    pub profile: String,
    pub language: String,
    pub semantics: String,
    pub logical_cost_profile: String,
    pub source_hash: String,
    pub program_binding: String,
}
/// Read-only scope diagnostics; every field must match the owner's scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceSponsorScope {
    pub grant: String,
    pub network: String,
    pub anchor: String,
    pub action: String,
    pub beneficiary: String,
    pub binding: SourceBinding,
}
/// Trusted, unsigned bootstrap limits. Each bootstrap creates an independent allowance.
/// The recovery slice excludes ordinary work; it supplies no recovery executor.
pub struct LocalSourceSponsorConfig {
    pub grant: String,
    pub network: String,
    pub anchor: String,
    pub action: String,
    pub beneficiary: String,
    pub dust_cap: UInt,
    pub logical_total: UInt,
    pub logical_recovery_reserved: UInt,
    pub logical_per_call: UInt,
}
/// Exclusive local owner of fee reservations and cumulative primitive work.
/// This owner cannot be cloned or restored from serialized state:
///
/// ```compile_fail
/// use mori_core::LocalSourceSponsor;
/// fn cloneable<T: Clone>() {}
/// cloneable::<LocalSourceSponsor>();
/// ```
///
/// ```compile_fail
/// use mori_core::LocalSourceSponsor;
/// fn serializable<T: serde::Serialize>() {}
/// serializable::<LocalSourceSponsor>();
/// ```
///
/// ```compile_fail
/// use mori_core::LocalSourceSponsor;
/// fn restorable<T: for<'de> serde::Deserialize<'de>>() {}
/// restorable::<LocalSourceSponsor>();
/// ```
///
/// Fee authority stays private, including mutable access:
///
/// ```compile_fail
/// use mori_core::LocalSourceSponsor;
/// fn extract(owner: LocalSourceSponsor) { let _ = owner.fee; }
/// ```
///
/// ```compile_fail
/// use mori_core::LocalSourceSponsor;
/// fn reset(owner: &mut LocalSourceSponsor) { owner.reset(); }
/// ```
#[derive(Debug)]
pub struct LocalSourceSponsor {
    scope: SourceSponsorScope,
    fee: FeeCapability,
    total: UInt,
    recovery_reserved: UInt,
    per_call: UInt,
    used: UInt,
}
/// Preparation accepts source and scenario, never caller IR, costs or limits.
pub struct SourceSponsoredRequest<'a> {
    pub candidate: &'a str,
    pub scope: &'a SourceSponsorScope,
    pub source: &'a str,
    pub scenario: &'a Scenario,
    pub dust_reservation: UInt,
}
/// Complete local execution plus measured primitive work and reservation diagnostics.
/// Success leaves a pending economic hold; it does not establish paid DUST.
#[derive(Debug, PartialEq, Eq)]
pub struct SourceSponsoredExecution {
    pub execution: Execution,
    pub cost: CostReport,
    pub candidate: String,
    pub scope: SourceSponsorScope,
}
/// Original refusal with measured work when the metered consumer was entered.
/// There is no partial execution, effect sequence or poststate.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("{error}")]
pub struct SourceSponsoredError {
    pub error: Error,
    pub cost: Option<CostReport>,
}
impl LocalSourceSponsor {
    /// Validate exact source and create fresh local authority under caller-trusted limits.
    pub fn from_trusted_local_source(
        source: &str,
        config: LocalSourceSponsorConfig,
    ) -> Result<Self, Error> {
        if [
            &config.grant,
            &config.network,
            &config.anchor,
            &config.action,
            &config.beneficiary,
        ]
        .iter()
        .any(|value| value.is_empty())
        {
            return Err(Error::new(
                "sponsor",
                "source sponsor scope must be nonempty",
            ));
        }
        let program = parse(source)?;
        validate(&program)?;
        if !program
            .actions
            .iter()
            .any(|action| action.name == config.action)
        {
            return Err(Error::new(
                "sponsor",
                "configured action is absent from source",
            ));
        }
        if config.logical_recovery_reserved.0 > config.logical_total.0 {
            return Err(Error::new(
                "sponsor",
                "recovery reserve exceeds logical total",
            ));
        }
        let source_hash = hash_bytes(source.as_bytes());
        let program_binding = hash_bytes(&canonical(&json!({
            "profile": SOURCE_SPONSOR_PROFILE, "language": LANGUAGE,
            "semantics": SEMANTICS, "logical_cost_profile": LOGICAL_COST_PROFILE,
            "source_hash": source_hash,
        }))?);
        let scope = SourceSponsorScope {
            grant: config.grant,
            network: config.network,
            anchor: config.anchor,
            action: config.action,
            beneficiary: config.beneficiary,
            binding: SourceBinding {
                profile: SOURCE_SPONSOR_PROFILE.into(),
                language: LANGUAGE.into(),
                semantics: SEMANTICS.into(),
                logical_cost_profile: LOGICAL_COST_PROFILE.into(),
                source_hash,
                program_binding,
            },
        };
        let fee = FeeCapability::new(
            &scope.grant,
            &scope.network,
            &scope.anchor,
            &scope.binding.program_binding,
            &scope.action,
            &scope.beneficiary,
            config.dust_cap,
        );
        Ok(Self {
            scope,
            fee,
            total: config.logical_total,
            recovery_reserved: config.logical_recovery_reserved,
            per_call: config.logical_per_call,
            used: UInt(0),
        })
    }
    pub fn scope(&self) -> &SourceSponsorScope {
        &self.scope
    }
    pub fn logical_used(&self) -> UInt {
        self.used
    }
    /// Remaining ordinary work, excluding the protected recovery slice.
    pub fn ordinary_remaining(&self) -> Result<UInt, Error> {
        self.total
            .0
            .checked_sub(self.recovery_reserved.0)
            .and_then(|n| n.checked_sub(self.used.0))
            .map(UInt)
            .ok_or_else(|| Error::new("sponsor", "invalid logical accounting"))
    }
    pub fn dust_available(&self) -> Result<UInt, Error> {
        self.fee.available()
    }
    pub fn dust_spent(&self) -> UInt {
        self.fee.spent()
    }
    /// Reserve economics before interpretation, then retain actual work on either outcome.
    // The refusal carries its charged cost report by design.
    #[allow(clippy::result_large_err)]
    pub fn prepare(
        &mut self,
        request: SourceSponsoredRequest<'_>,
    ) -> Result<SourceSponsoredExecution, SourceSponsoredError> {
        let unmetered = |error| SourceSponsoredError { error, cost: None };
        if request.scope != &self.scope {
            return Err(unmetered(Error::new(
                "sponsor",
                "source sponsor scope mismatch",
            )));
        }
        if hash_bytes(request.source.as_bytes()) != self.scope.binding.source_hash {
            return Err(unmetered(Error::new(
                "sponsor",
                "source sponsor source mismatch",
            )));
        }
        if request.candidate.is_empty() {
            return Err(unmetered(Error::new(
                "sponsor",
                "candidate identity must be nonempty",
            )));
        }
        if request.dust_reservation.0 == 0 {
            return Err(unmetered(Error::new(
                "sponsor",
                "DUST reservation must be positive",
            )));
        }
        // Stage only economic authority. Logical usage never rolls back with this clone.
        let mut staged = self.fee.clone();
        staged
            .reserve(
                request.candidate,
                &self.scope.network,
                &self.scope.anchor,
                &self.scope.binding.program_binding,
                &self.scope.action,
                &self.scope.beneficiary,
                request.dust_reservation,
            )
            .map_err(unmetered)?;
        let remaining = self.ordinary_remaining().map_err(unmetered)?;
        let cap = UInt(self.per_call.0.min(remaining.0));
        let program = parse(request.source).map_err(unmetered)?;
        let result = simulate_metered(&program, &self.scope.action, request.scenario, cap);
        let cost = match &result {
            Ok(value) => &value.cost,
            Err(value) => &value.cost,
        };
        // Compute before assigning: defensive arithmetic refusal drops the stage and
        // preserves prior usage. The existing meter guarantees used <= cap.
        let limit = self
            .total
            .0
            .checked_sub(self.recovery_reserved.0)
            .ok_or_else(|| unmetered(Error::new("sponsor", "invalid logical accounting")))?;
        let next_used = self
            .used
            .0
            .checked_add(cost.used.0)
            .filter(|next| *next <= limit)
            .ok_or_else(|| unmetered(Error::new("sponsor", "invalid logical accounting")))?;
        self.used = UInt(next_used);
        match result {
            Ok(value) => {
                self.fee = staged;
                Ok(SourceSponsoredExecution {
                    execution: value.execution,
                    cost: value.cost,
                    candidate: request.candidate.into(),
                    scope: self.scope.clone(),
                })
            }
            Err(value) => Err(SourceSponsoredError {
                error: value.error,
                cost: Some(value.cost),
            }),
        }
    }
    /// Unknown finality retains the full hold; timeout does not release it.
    pub fn mark_unknown(&mut self, candidate: &str) -> Result<(), Error> {
        self.fee.mark_unknown(candidate)
    }
    /// Release only a never-submitted hold; consumed candidate identities stay consumed.
    pub fn release_unused(&mut self, candidate: &str) -> Result<(), Error> {
        self.fee.release_unused(candidate)
    }
    /// Reconcile numeric spend from an observation authenticated by the trusted caller.
    /// This method does not query a network or authenticate that observation.
    pub fn settle_from_trusted_local_observation(
        &mut self,
        candidate: &str,
        actual: UInt,
    ) -> Result<(), Error> {
        self.fee.settle(candidate, actual)
    }
}

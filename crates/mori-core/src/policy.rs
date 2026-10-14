//! Deterministic cost accounting and scoped local authority.
//! Authentication of grants and settlement observations belongs to the caller.
//! Sponsored preparation calls the source evaluator. It commits reservation
//! accounting only after that evaluation succeeds. A caller premise is not a
//! receipt, a funding proof, or a ledger observation.
use crate::{Error, UInt};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Cost {
    pub logical: UInt,
    pub dust: UInt,
    pub service: UInt,
    pub external: UInt,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CostPlane {
    Logical,
    Dust,
    Service,
    External,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Meter {
    caps: Cost,
    used: Cost,
}
impl Meter {
    pub fn new(caps: Cost) -> Self {
        Self {
            caps,
            used: Cost::default(),
        }
    }
    pub fn used(&self) -> Cost {
        self.used
    }
    pub fn charge(&mut self, plane: CostPlane, amount: UInt) -> Result<(), Error> {
        let (used, cap) = match plane {
            CostPlane::Logical => (&mut self.used.logical, self.caps.logical),
            CostPlane::Dust => (&mut self.used.dust, self.caps.dust),
            CostPlane::Service => (&mut self.used.service, self.caps.service),
            CostPlane::External => (&mut self.used.external, self.caps.external),
        };
        let next = used
            .0
            .checked_add(amount.0)
            .ok_or_else(|| Error::new("meter", "cost counter overflow"))?;
        if next > cap.0 {
            return Err(Error::new("meter", "cost plane cap exceeded"));
        }
        *used = UInt(next);
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum Reservation {
    Reserved(UInt),
    Unknown(UInt),
    Settled(UInt),
    Released,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeeCapability {
    id: String,
    network: String,
    anchor: String,
    program: String,
    action: String,
    beneficiary: String,
    cap: UInt,
    spent: UInt,
    reservations: BTreeMap<String, Reservation>,
}
impl FeeCapability {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        network: &str,
        anchor: &str,
        program: &str,
        action: &str,
        beneficiary: &str,
        cap: UInt,
    ) -> Self {
        Self {
            id: id.into(),
            network: network.into(),
            anchor: anchor.into(),
            program: program.into(),
            action: action.into(),
            beneficiary: beneficiary.into(),
            cap,
            spent: UInt(0),
            reservations: BTreeMap::new(),
        }
    }
    pub fn spent(&self) -> UInt {
        self.spent
    }
    pub fn available(&self) -> Result<UInt, Error> {
        let pending = self
            .reservations
            .values()
            .try_fold(0_u128, |total, r| match r {
                Reservation::Reserved(a) | Reservation::Unknown(a) => total.checked_add(a.0),
                _ => Some(total),
            })
            .ok_or_else(|| Error::new("sponsor", "reserved capacity overflow"))?;
        self.cap
            .0
            .checked_sub(self.spent.0)
            .and_then(|n| n.checked_sub(pending))
            .map(UInt)
            .ok_or_else(|| Error::new("sponsor", "invalid capacity accounting"))
    }
    #[allow(clippy::too_many_arguments)]
    pub fn reserve(
        &mut self,
        id: &str,
        network: &str,
        anchor: &str,
        program: &str,
        action: &str,
        beneficiary: &str,
        amount: UInt,
    ) -> Result<(), Error> {
        if [
            &self.id,
            &self.network,
            &self.anchor,
            &self.program,
            &self.action,
            &self.beneficiary,
        ]
        .iter()
        .any(|s| s.is_empty())
            || id.is_empty()
            || (network, anchor, program, action, beneficiary)
                != (
                    self.network.as_str(),
                    self.anchor.as_str(),
                    self.program.as_str(),
                    self.action.as_str(),
                    self.beneficiary.as_str(),
                )
        {
            return Err(Error::new("sponsor", "fee capability scope mismatch"));
        }
        if self.reservations.contains_key(id) {
            return Err(Error::new(
                "sponsor",
                "reservation identity already consumed",
            ));
        }
        if amount.0 > self.available()?.0 {
            return Err(Error::new("sponsor", "insufficient sponsor capacity"));
        }
        self.reservations
            .insert(id.into(), Reservation::Reserved(amount));
        Ok(())
    }
    pub fn mark_unknown(&mut self, id: &str) -> Result<(), Error> {
        match self.reservations.get_mut(id) {
            Some(r @ Reservation::Reserved(_)) => {
                let Reservation::Reserved(a) = *r else {
                    unreachable!()
                };
                *r = Reservation::Unknown(a);
                Ok(())
            }
            _ => Err(Error::new(
                "sponsor",
                "reservation is not pending submission",
            )),
        }
    }
    /// Release a reservation that has never been submitted. Timeout is insufficient.
    pub fn release_unused(&mut self, id: &str) -> Result<(), Error> {
        match self.reservations.get_mut(id) {
            Some(r @ Reservation::Reserved(_)) => {
                *r = Reservation::Released;
                Ok(())
            }
            _ => Err(Error::new(
                "sponsor",
                "cannot release an unknown or finalized spend",
            )),
        }
    }
    /// The caller must authenticate a final settlement observation before reconciling.
    pub fn settle(&mut self, id: &str, actual: UInt) -> Result<(), Error> {
        let bound = match self.reservations.get(id) {
            Some(Reservation::Reserved(a) | Reservation::Unknown(a)) => *a,
            _ => return Err(Error::new("sponsor", "reservation is absent or finalized")),
        };
        if actual.0 > bound.0 {
            return Err(Error::new(
                "sponsor",
                "settled DUST exceeds reserved authority",
            ));
        }
        let spent = self
            .spent
            .0
            .checked_add(actual.0)
            .ok_or_else(|| Error::new("sponsor", "settled spend overflow"))?;
        if spent > self.cap.0 {
            return Err(Error::new("sponsor", "sponsor cap exceeded"));
        }
        self.spent = UInt(spent);
        self.reservations
            .insert(id.into(), Reservation::Settled(actual));
        Ok(())
    }
    pub fn reservation(&self, id: &str) -> Option<SponsorReservation> {
        self.reservations.get(id).map(|held| match held {
            Reservation::Reserved(amount) => SponsorReservation::Reserved(*amount),
            Reservation::Unknown(amount) => SponsorReservation::Unknown(*amount),
            Reservation::Settled(amount) => SponsorReservation::Settled(*amount),
            Reservation::Released => SponsorReservation::Released,
        })
    }
    /// Record a timeout. This does not prove that a spend occurred or did not occur.
    pub fn note_timeout(&mut self, id: &str) -> Result<(), Error> {
        self.mark_unknown(id)
    }
    /// Settle only when the caller sets `authenticated`.
    /// This function does not authenticate the observation.
    pub fn settle_authenticated(
        &mut self,
        id: &str,
        actual: UInt,
        authenticated: bool,
    ) -> Result<(), Error> {
        if !authenticated {
            return Err(Error::new(
                "sponsor",
                "authenticated settlement observation required",
            ));
        }
        self.settle(id, actual)
    }
    /// Return an unknown hold only when the caller sets `authenticated`.
    /// This function does not authenticate the unused observation.
    pub fn release_authenticated_unused(
        &mut self,
        id: &str,
        authenticated: bool,
    ) -> Result<(), Error> {
        if !authenticated {
            return Err(Error::new(
                "sponsor",
                "authenticated unused observation required",
            ));
        }
        match self.reservations.get_mut(id) {
            Some(held @ Reservation::Unknown(_)) => {
                *held = Reservation::Released;
                Ok(())
            }
            _ => Err(Error::new(
                "sponsor",
                "authenticated unused release requires an unknown reservation",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SponsorReservation {
    Reserved(UInt),
    Unknown(UInt),
    Settled(UInt),
    Released,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkBudget {
    meter: Meter,
    recovery_reserved: Cost,
    recovery_used: Cost,
}
impl WorkBudget {
    pub fn new(caps: Cost) -> Self {
        Self {
            meter: Meter::new(caps),
            recovery_reserved: Cost::default(),
            recovery_used: Cost::default(),
        }
    }
    pub fn ordinary_used(&self) -> Cost {
        self.meter.used()
    }
    pub fn recovery_reserved(&self) -> Cost {
        self.recovery_reserved
    }
    pub fn recovery_used(&self) -> Cost {
        self.recovery_used
    }
    pub fn reserve_recovery(&mut self, plane: CostPlane, amount: UInt) -> Result<(), Error> {
        if amount.0 == 0 {
            return Err(Error::new("meter", "recovery reservation must be positive"));
        }
        let reserved = plane_amount(&self.recovery_reserved, plane).0;
        let ordinary = plane_amount(&self.meter.used(), plane).0;
        let cap = plane_amount(&self.meter.caps, plane).0;
        let next = reserved
            .checked_add(amount.0)
            .ok_or_else(|| Error::new("meter", "cost counter overflow"))?;
        let occupied = next
            .checked_add(ordinary)
            .ok_or_else(|| Error::new("meter", "cost counter overflow"))?;
        if occupied > cap {
            return Err(Error::new(
                "meter",
                "recovery reservation exceeds remaining capacity",
            ));
        }
        set_plane(&mut self.recovery_reserved, plane, UInt(next));
        Ok(())
    }
    pub fn charge_ordinary(&mut self, plane: CostPlane, amount: UInt) -> Result<(), Error> {
        if amount.0 == 0 {
            return Ok(());
        }
        let used = plane_amount(&self.meter.used(), plane).0;
        let reserved = plane_amount(&self.recovery_reserved, plane).0;
        let cap = plane_amount(&self.meter.caps, plane).0;
        let next = used
            .checked_add(amount.0)
            .ok_or_else(|| Error::new("meter", "cost counter overflow"))?;
        let occupied = next
            .checked_add(reserved)
            .ok_or_else(|| Error::new("meter", "cost counter overflow"))?;
        if next > cap {
            return Err(Error::new("meter", "cost plane cap exceeded"));
        }
        if occupied > cap {
            return Err(Error::new(
                "meter",
                "ordinary work would consume reserved recovery",
            ));
        }
        self.meter.charge(plane, amount)
    }
    pub fn charge_recovery(&mut self, plane: CostPlane, amount: UInt) -> Result<(), Error> {
        if amount.0 == 0 {
            return Ok(());
        }
        let used = plane_amount(&self.recovery_used, plane).0;
        let reserved = plane_amount(&self.recovery_reserved, plane).0;
        let next = used
            .checked_add(amount.0)
            .ok_or_else(|| Error::new("meter", "cost counter overflow"))?;
        if next > reserved {
            return Err(Error::new(
                "meter",
                "recovery charge exceeds reserved slice",
            ));
        }
        set_plane(&mut self.recovery_used, plane, UInt(next));
        Ok(())
    }
}

fn plane_amount(cost: &Cost, plane: CostPlane) -> UInt {
    match plane {
        CostPlane::Logical => cost.logical,
        CostPlane::Dust => cost.dust,
        CostPlane::Service => cost.service,
        CostPlane::External => cost.external,
    }
}
fn set_plane(cost: &mut Cost, plane: CostPlane, amount: UInt) {
    match plane {
        CostPlane::Logical => cost.logical = amount,
        CostPlane::Dust => cost.dust = amount,
        CostPlane::Service => cost.service = amount,
        CostPlane::External => cost.external = amount,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SponsorScope {
    pub grant: String,
    pub network: String,
    pub anchor: String,
    pub program: String,
    pub action: String,
    pub beneficiary: String,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalObservation {
    Hold,
    Timeout,
    Settle { actual: UInt, authenticated: bool },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SponsoredExecution {
    pub execution: crate::Execution,
}
#[derive(Debug, Clone)]
pub struct SponsoredRequest<'a> {
    pub candidate: &'a str,
    pub scope: &'a SponsorScope,
    pub dust: UInt,
    pub ordinary: Cost,
    pub program: &'a crate::Program,
    pub action: &'a str,
    pub scenario: &'a crate::Scenario,
    pub observation: LocalObservation,
}

/// Prepare one local sponsored execution.
/// Scope, capacity, recovery, and observation refusals leave both inputs unchanged.
/// Evaluator failure also leaves both inputs unchanged.
pub fn prepare_sponsored_execution(
    grant: &mut FeeCapability,
    budget: &mut WorkBudget,
    request: SponsoredRequest<'_>,
) -> Result<SponsoredExecution, Error> {
    let grant_before = grant.clone();
    let budget_before = budget.clone();
    let prepared = commit_sponsored_execution(grant, budget, request);
    if prepared.is_err() {
        *grant = grant_before;
        *budget = budget_before;
    }
    prepared
}

fn commit_sponsored_execution(
    grant: &mut FeeCapability,
    budget: &mut WorkBudget,
    request: SponsoredRequest<'_>,
) -> Result<SponsoredExecution, Error> {
    sponsor_scope_matches(
        grant,
        request.scope,
        request.program,
        request.action,
        request.candidate,
    )?;
    if request.dust.0 == 0 {
        return Err(Error::new("sponsor", "dust reservation must be positive"));
    }
    if let LocalObservation::Settle {
        actual,
        authenticated,
    } = request.observation
    {
        if !authenticated {
            return Err(Error::new(
                "sponsor",
                "authenticated settlement observation required",
            ));
        }
        if actual.0 > request.dust.0 {
            return Err(Error::new(
                "sponsor",
                "settled DUST exceeds reserved authority",
            ));
        }
    }
    let execution = crate::simulate(request.program, request.action, request.scenario)?;
    grant.reserve(
        request.candidate,
        &request.scope.network,
        &request.scope.anchor,
        &request.scope.program,
        &request.scope.action,
        &request.scope.beneficiary,
        request.dust,
    )?;
    charge_ordinary_cost(budget, request.ordinary)?;
    match request.observation {
        LocalObservation::Hold => {}
        LocalObservation::Timeout => grant.note_timeout(request.candidate)?,
        LocalObservation::Settle {
            actual,
            authenticated,
        } => grant.settle_authenticated(request.candidate, actual, authenticated)?,
    }
    Ok(SponsoredExecution { execution })
}

fn sponsor_scope_matches(
    grant: &FeeCapability,
    scope: &SponsorScope,
    program: &crate::Program,
    action: &str,
    candidate: &str,
) -> Result<(), Error> {
    let aligned = !scope.grant.is_empty()
        && !scope.network.is_empty()
        && !scope.anchor.is_empty()
        && !scope.program.is_empty()
        && !scope.action.is_empty()
        && !scope.beneficiary.is_empty()
        && !candidate.is_empty()
        && scope.grant == grant.id
        && scope.network == grant.network
        && scope.anchor == grant.anchor
        && scope.program == grant.program
        && scope.action == grant.action
        && scope.beneficiary == grant.beneficiary
        && program.name == grant.program
        && action == grant.action;
    if aligned {
        Ok(())
    } else {
        Err(Error::new("sponsor", "fee capability scope mismatch"))
    }
}

fn charge_ordinary_cost(budget: &mut WorkBudget, cost: Cost) -> Result<(), Error> {
    budget.charge_ordinary(CostPlane::Logical, cost.logical)?;
    budget.charge_ordinary(CostPlane::Dust, cost.dust)?;
    budget.charge_ordinary(CostPlane::Service, cost.service)?;
    budget.charge_ordinary(CostPlane::External, cost.external)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Clear,
    Prove,
    Relay,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RoleGrant {
    pub members: BTreeSet<String>,
    pub quorum: u16,
    pub calls: BTreeSet<(String, String)>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FederationPolicy {
    pub network: String,
    pub anchor: String,
    pub epoch: UInt,
    pub grants: BTreeMap<Role, RoleGrant>,
    pub revoked: BTreeSet<String>,
}
impl FederationPolicy {
    #[allow(clippy::too_many_arguments)]
    pub fn authorize(
        &self,
        role: Role,
        members: &[String],
        epoch: UInt,
        network: &str,
        anchor: &str,
        program: &str,
        action: &str,
    ) -> Result<(), Error> {
        if epoch != self.epoch
            || network != self.network
            || anchor != self.anchor
            || network.is_empty()
            || anchor.is_empty()
        {
            return Err(Error::new("policy", "policy epoch or domain mismatch"));
        }
        let grant = self
            .grants
            .get(&role)
            .ok_or_else(|| Error::new("policy", "role has no grant"))?;
        let unique: BTreeSet<_> = members.iter().collect();
        if grant.quorum == 0
            || usize::from(grant.quorum) > grant.members.len()
            || unique.len() != members.len()
            || unique.len() < usize::from(grant.quorum)
            || unique
                .iter()
                .any(|m| !grant.members.contains(*m) || self.revoked.contains(*m))
            || !grant.calls.contains(&(program.into(), action.into()))
        {
            return Err(Error::new("policy", "role, quorum or call scope denied"));
        }
        Ok(())
    }
}

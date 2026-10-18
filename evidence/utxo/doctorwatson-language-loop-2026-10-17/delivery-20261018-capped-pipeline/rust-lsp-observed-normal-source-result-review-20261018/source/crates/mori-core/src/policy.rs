//! Deterministic cost accounting and scoped local authority.
//! Authentication of grants and settlement observations belongs to the caller.
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

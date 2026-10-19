use super::owner_ext_type_builder::ExtTypeBuildError;
use super::{
    CapacityKind, HostViolation, HostViolationKind, LocatorC, NeededNatC, SliceC, Tooling,
};

fn core_cap_err(
    kind: CapacityKind,
    current_len: usize,
    limit: usize,
    locator: Option<LocatorC>,
) -> ExtTypeBuildError {
    let needed = match (current_len as u128).checked_add(1) {
        Some(n) => NeededNatC::Exact(n),
        None => NeededNatC::AboveU128,
    };
    ExtTypeBuildError::CoreTooling(Tooling::CapacityNotAdmitted {
        kind,
        needed,
        limit: limit as u128,
        locator,
    })
}

fn host_slot_err(kind: HostViolationKind, detail: &'static str) -> ExtTypeBuildError {
    ExtTypeBuildError::Host(HostViolation {
        kind,
        clause: None,
        rule: None,
        locator: Some(detail.to_string()),
    })
}

pub(super) fn reserve_slot<T>(
    slots: &mut Vec<Option<T>>,
    limit: usize,
    kind: CapacityKind,
    locator: Option<LocatorC>,
) -> Result<usize, ExtTypeBuildError> {
    let index = slots.len();
    if index >= limit {
        return Err(core_cap_err(kind, index, limit, locator));
    }
    slots.push(None);
    Ok(index)
}

pub(super) fn reserve_pair<A, B>(
    left: &mut Vec<Option<A>>,
    right: &mut Vec<Option<B>>,
    limit: usize,
    kind: CapacityKind,
    locator: Option<LocatorC>,
) -> Result<usize, ExtTypeBuildError> {
    let index = left.len();
    if index != right.len() {
        return Err(host_slot_err(
            HostViolationKind::WrongVariant,
            "unaligned reserved slot pair",
        ));
    }
    if index >= limit {
        return Err(core_cap_err(kind, index, limit, locator));
    }
    left.push(None);
    right.push(None);
    Ok(index)
}

pub(super) fn fill_reserved_once<T>(
    slots: &mut Vec<Option<T>>,
    index: usize,
    value: T,
) -> Result<(), ExtTypeBuildError> {
    match slots.get_mut(index) {
        None => Err(host_slot_err(
            HostViolationKind::ReadOutOfBounds,
            "reserved slot index out of bounds",
        )),
        Some(Some(_)) => Err(host_slot_err(
            HostViolationKind::WrongVariant,
            "reserved slot already filled",
        )),
        Some(slot @ None) => {
            *slot = Some(value);
            Ok(())
        }
    }
}

pub(super) fn append_ids_bounded(
    target: &mut Vec<usize>,
    ids: &[usize],
    limit: usize,
    kind: CapacityKind,
    locator: Option<LocatorC>,
) -> Result<SliceC, ExtTypeBuildError> {
    let start = target.len();
    for &id in ids {
        if target.len() >= limit {
            return Err(core_cap_err(kind, target.len(), limit, locator));
        }
        target.push(id);
    }
    Ok(SliceC::normalized(start, ids.len()))
}

//! Strict bounded decoding of local stage files and the R7 canonical boundary.
//!
//! Size limits apply to bytes actually read, before parsing. Parsing keeps the
//! legacy duplicate-key, depth and node limits; only the byte ceiling differs.
//! A resumed boundary is checked as written and is never re-sorted.
use super::{Continuation, NamedValue, StageEvent, StagedProgram};
use crate::{AssetId, Error, MAX_TRACE_CELLS, Program, Snapshot, Statement};
use serde::de::DeserializeOwned;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub use super::{MAX_CONTINUATION_BYTES, MAX_EVENT_BYTES};

pub const CONTINUATION_TOO_LARGE: &str = "Continuation JSON exceeds 4 MiB";
pub const EVENT_TOO_LARGE: &str = "Stage event JSON exceeds 64 KiB";

/// Reads at most `max + 1` bytes, so the limit holds even if the file grows
/// after it is opened. Oversized or non-UTF-8 input fails with `json`.
pub fn read_bounded(path: &Path, max: usize, too_large: &'static str) -> Result<String, Error> {
    use std::io::Read;
    let file = std::fs::File::open(path)
        .map_err(|e| Error::new("io", format!("Cannot open stage input: {e}")))?;
    let limit = u64::try_from(max).unwrap_or(u64::MAX).saturating_add(1);
    let mut bytes = Vec::new();
    file.take(limit)
        .read_to_end(&mut bytes)
        .map_err(|e| Error::new("io", format!("Cannot read stage input: {e}")))?;
    if bytes.len() > max {
        return Err(Error::new("json", too_large));
    }
    String::from_utf8(bytes).map_err(|_| Error::new("json", "Stage input is not UTF-8"))
}

fn strict<T: DeserializeOwned>(text: &str, max: usize, too_large: &'static str) -> Result<T, Error> {
    let value = crate::json::parse_limited(text, max, too_large)?;
    serde_json::from_value(value).map_err(|e| Error::new("json", e.to_string()))
}

/// Exactly one `Continuation`; unknown fields, a missing `cap` key and
/// non-canonical UInt strings are rejected by the type's derives.
pub fn continuation(text: &str) -> Result<Continuation, Error> {
    strict(text, MAX_CONTINUATION_BYTES, CONTINUATION_TOO_LARGE)
}

pub fn event(text: &str) -> Result<StageEvent, Error> {
    strict(text, MAX_EVENT_BYTES, EVENT_TOO_LARGE)
}

type Identity<'a> = (&'a AssetId, &'a str, &'a str);

/// Source-fixed liability identities over every action of the program.
/// For `/2` this is exactly the former private `registry`. For `/3` it is
/// that set union `Program.duties`, each duty asset name resolved to its
/// declared exact `AssetId`. A duty-caused conflict uses
/// `sequence_duty_declaration`; a statement-only conflict keeps `liability`.
/// This must agree with `stage::sequence`-side registries if one is added
/// later and with `runtime` liability identities for `/2`.
pub(crate) fn registry_for(p: &Program) -> Result<BTreeMap<&str, Identity<'_>>, Error> {
    let mut identities = BTreeMap::new();
    for stmt in p.actions.iter().flat_map(|action| &action.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
        {
            let id = asset_identity(p, asset)?;
            let identity = (id, debtor.as_str(), creditor.as_str());
            if identities
                .insert(name.as_str(), identity)
                .is_some_and(|existing| existing != identity)
            {
                return Err(Error::new(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
        }
    }
    if p.language == super::LANGUAGE_3 && p.semantics == super::SEMANTICS_3 {
        for decl in &p.duties {
            let id = asset_identity(p, &decl.asset).map_err(|_| {
                Error::new(
                    "sequence_duty_declaration",
                    "External duty declaration is not admissible in this program.",
                )
            })?;
            let identity = (id, decl.debtor.as_str(), decl.creditor.as_str());
            if identities
                .insert(decl.name.as_str(), identity)
                .is_some_and(|existing| existing != identity)
            {
                return Err(Error::new(
                    "sequence_duty_declaration",
                    "External duty declaration is not admissible in this program.",
                ));
            }
        }
    } else if !p.duties.is_empty() {
        return Err(Error::new(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    Ok(identities)
}

fn asset_identity<'a>(p: &'a Program, name: &str) -> Result<&'a AssetId, Error> {
    p.assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| Error::new("asset", format!("Unknown asset {name:?}.")))
}

/// R7: the legacy `scenario_snapshot` predicates applied to a resumed boundary
/// without normalisation. Codes and messages follow `scenario_snapshot`;
/// ordering failures are new `balance`/`liability` messages. The registry
/// step is profile-dispatched only through `registry_for`: `/2` uses the
/// statement registry, `/3` uses REG3. Every other predicate is unchanged.
///
/// Balances keep the baseline predicate: every account the whole action
/// transfers from or to must be present, and other declared-asset accounts
/// (such as an untouched observer) are preserved, not refused.
pub fn canonical_boundary(
    sp: &StagedProgram,
    action: usize,
    captures: &[NamedValue],
    b: &Snapshot,
) -> Result<(), Error> {
    let p = sp.program();
    let (Some(a), Some(plan)) = (p.actions.get(action), sp.plan().actions.get(action)) else {
        return Err(Error::new("stage_site", "Unknown staged action."));
    };
    let _ = captures;
    if b.balances.len() > 4096 || b.liabilities.len() > 1024 {
        return Err(Error::new(
            "bound",
            "Scenario exceeds 4,096 balances or 1,024 liabilities.",
        ));
    }
    if b.state.len() != p.states.len() {
        return Err(Error::new(
            "state",
            "Boundary must provide every declared state value and no other.",
        ));
    }
    for d in &p.states {
        let value = b
            .state
            .get(&d.name)
            .ok_or_else(|| Error::new("state", format!("Missing state {:?}.", d.name)))?;
        if value.ty() != d.ty {
            return Err(Error::new(
                "state",
                format!("Wrong type for state {:?}.", d.name),
            ));
        }
    }
    let ids: BTreeSet<&AssetId> = p.assets.iter().map(|a| &a.identity).collect();
    let mut previous: Option<(&AssetId, &str)> = None;
    for balance in &b.balances {
        if !ids.contains(&balance.asset) {
            return Err(Error::new(
                "asset",
                format!(
                    "Undeclared exact asset identity for account {:?}.",
                    balance.account
                ),
            ));
        }
        if balance.account.is_empty() || balance.account.len() > 1024 {
            return Err(Error::new(
                "balance",
                "Duplicate or invalid balance account/asset.",
            ));
        }
        let key = (&balance.asset, balance.account.as_str());
        if previous.is_some_and(|prior| prior >= key) {
            return Err(Error::new("balance", "Balances are not canonical"));
        }
        previous = Some(key);
    }
    for stmt in &a.statements {
        if let Statement::Transfer {
            asset, from, to, ..
        }
        | Statement::Fee {
            asset, from, to, ..
        } = stmt
        {
            let id = asset_identity(p, asset)?;
            for account in [from, to] {
                if b.balances
                    .binary_search_by(|x| (&x.asset, x.account.as_str()).cmp(&(id, account.as_str())))
                    .is_err()
                {
                    return Err(Error::new(
                        "balance",
                        format!("Missing explicit balance for {account:?} and asset {asset:?}."),
                    ));
                }
            }
        }
    }
    let registry = registry_for(p)?;
    let mut previous: Option<&str> = None;
    for l in &b.liabilities {
        if !ids.contains(&l.asset) {
            return Err(Error::new(
                "asset",
                "Liability uses undeclared exact asset identity.",
            ));
        }
        if l.name.is_empty()
            || l.name.len() > 1024
            || l.debtor.is_empty()
            || l.creditor.is_empty()
            || l.debtor.len() > 1024
            || l.creditor.len() > 1024
        {
            return Err(Error::new(
                "liability",
                "Invalid or duplicate liability identity.",
            ));
        }
        if previous.is_some_and(|prior| prior >= l.name.as_str()) {
            return Err(Error::new("liability", "Liabilities are not canonical"));
        }
        previous = Some(l.name.as_str());
        if registry.get(l.name.as_str()) != Some(&(&l.asset, l.debtor.as_str(), l.creditor.as_str())) {
            return Err(Error::new(
                "liability",
                format!("Existing liability identity differs for {:?}.", l.name),
            ));
        }
    }
    let added = a
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, Statement::Liability { .. }))
        .count();
    let records = p.states.len() + b.balances.len() + b.liabilities.len() + added + 1;
    if records.saturating_mul(usize::from(plan.statement_count) + 2) > MAX_TRACE_CELLS {
        return Err(Error::new(
            "bound",
            "Scenario and action exceed 65,536 trace snapshot records.",
        ));
    }
    Ok(())
}

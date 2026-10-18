//! Explicit local transfer presentation consumer of mori-source6.
use std::{io::Write, process::ExitCode};

use mori_core::UInt;
use mori_diagnostics::NamedSource;
use mori_source6::{Allowance, Balance, LocalTransferScenario, Replay, expand_transfer};
use serde::Deserialize;

use crate::{cli::ActionInput, commands::core_io, shell::Shell};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TransferScenario {
    profile: String,
    kind: String,
    domain: String,
    asset: String,
    head: String,
    predecessor: String,
    round: UInt,
    balances: [BalanceCell; 3],
    allowance: AllowanceCell,
    replay: ReplayClaim,
    work_remaining: UInt,
    work_spent: UInt,
    post_head: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BalanceCell {
    account: String,
    amount: UInt,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AllowanceCell {
    owner: String,
    remaining: UInt,
    spent: UInt,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ReplayClaim {
    Unused,
    Consumed,
}

impl TransferScenario {
    fn local(&self) -> LocalTransferScenario<'_> {
        LocalTransferScenario {
            domain: &self.domain,
            asset: &self.asset,
            head: &self.head,
            predecessor: &self.predecessor,
            round: self.round.0,
            balances: self.balances.each_ref().map(|cell| Balance {
                account: &cell.account,
                amount: cell.amount.0,
            }),
            allowance: Allowance {
                owner: &self.allowance.owner,
                remaining: self.allowance.remaining.0,
                spent: self.allowance.spent.0,
            },
            replay: match self.replay {
                ReplayClaim::Unused => Replay::Unused,
                ReplayClaim::Consumed => Replay::Consumed,
            },
            work_remaining: self.work_remaining.0,
            work_spent: self.work_spent.0,
            post_head: &self.post_head,
        }
    }
}

pub(super) fn run(input: ActionInput, shell: &mut Shell) -> crate::commands::Result<ExitCode> {
    let source = core_io::read_source(&input.file)?;
    let scenario: TransferScenario = core_io::read_json(&input.scenario)?;
    if scenario.profile != "moriarty-local-scenario/1" || scenario.kind != "local-stipulation" {
        return Err(core_io::operation(
            "Source/6 transfer expansion requires moriarty-local-scenario/1 local-stipulation input.",
        ));
    }
    let expansion = match expand_transfer(&source, &input.action, &scenario.local()) {
        Ok(expansion) => expansion,
        Err(diagnostics) => {
            for diagnostic in diagnostics {
                shell.report(&diagnostic.with_source_code(NamedSource::new(
                    input.file.display().to_string(),
                    source.clone(),
                )));
            }
            return Ok(ExitCode::FAILURE);
        }
    };
    // The library includes its final newline. Write the actual generated text,
    // without adding a JSON/string wrapper or constructing a stage result.
    std::io::stdout()
        .lock()
        .write_all(expansion.source6.as_bytes())
        .map_err(core_io::operation)?;
    shell.status(
        "Expanded",
        format!("Source/6 transfer proposal ({})", expansion.qualification),
    );
    Ok(ExitCode::SUCCESS)
}

> For the complete documentation index, see [llms.txt](/llms.txt)

# Midnight agent skills

AI coding assistants like Claude, GitHub Copilot, and Cursor don't have Compact in their training data. When you ask them to write a Compact contract, they generate plausible-looking code that fails to compile.

`midnight_agent_skills` is a set of four modular agent skills that fix this. Install them once, and your AI assistant has accurate knowledge of Compact syntax, Midnight SDK, network configuration, and common gotchas sourced from the developer community.

## Install[​](#install "Direct link to Install")

Install all skills at once:

```
npx skills add https://github.com/mzf11125/midnight_agent_skills
```

Or install individual skills:

```
npx skills add https://github.com/mzf11125/midnight_agent_skills --skill midnight-compact

npx skills add https://github.com/mzf11125/midnight_agent_skills --skill midnight-api
```

## Available skills[​](#available-skills "Direct link to Available skills")

The following four skills can be installed individually or all at once. Each skill targets a specific area of Midnight development.

### midnight-concepts[​](#midnight-concepts "Direct link to midnight-concepts")

Foundational knowledge about Midnight's zero-knowledge architecture: the Kachina protocol, DUST/NIGHT tokenomics, the dual-state model (public ledger vs private state), and ZK proof fundamentals.

### midnight-compact[​](#midnight-compact "Direct link to midnight-compact")

Complete guide to the Compact language (v0.22+):

* Circuit syntax and the constraint model
* Ledger operations: `Counter`, `Map`, `Set`, `MerkleTree`
* The `disclose()` mechanism and witness protection
* Common syntax gotchas (`.read()` not `.value()`, enum access with `.` not `::`, witness functions have no body)
* On-chain design patterns from production experience

### midnight-api[​](#midnight-api "Direct link to midnight-api")

SDK integration for building DApps:

* Wallet connection (Lace, 1AM) via `midnight-wallet-kit`
* Contract deployment and interaction
* SDK compatibility matrix and known silent failures
* Preprod network configuration and troubleshooting

### midnight-network[​](#midnight-network "Direct link to midnight-network")

Network infrastructure:

* Proof server setup via Docker
* Indexer configuration
* Node deployment

## What skills cover[​](#what-skills-cover "Direct link to What skills cover")

The skills are built from official Midnight documentation plus community articles from the Midnight Aliit Fellowship. They include:

### Mental model corrections for developers coming from EVM[​](#mental-model-corrections-for-developers-coming-from-evm "Direct link to Mental model corrections for developers coming from EVM")

* Circuits declare constraints, they don't execute. `assert` is a constraint declaration, not a runtime guard.
* `disclose()` is a compile-time annotation, not encryption. The compiler tracks witness data through arithmetic and rejects undeclared disclosures.
* Block limits are hard limits, not gas costs. `BlockLimitExceeded` means the transaction cannot execute at all.

### On-chain design patterns[​](#on-chain-design-patterns "Direct link to On-chain design patterns")

* Flat maps over struct maps (struct reads pull every field into the circuit)
* Off-chain computation with Merkle root verification
* Minimal on-chain state

### Common syntax gotchas[​](#common-syntax-gotchas "Direct link to Common syntax gotchas")

The following examples show frequently misused syntax and their correct equivalents.

```
// WRONG

const val = counter.value();

if (state == GameState::playing) { ... }

witness get_key(): Bytes<32> { return local_key(); }



// CORRECT

const val = counter.read();

if (state == GameState.playing) { ... }

witness local_key(): Bytes<32>;
```

## Source[​](#source "Direct link to Source")

* Repository: <https://github.com/mzf11125/midnight_agent_skills>
* Install via npm: `npx skills add https://github.com/mzf11125/midnight_agent_skills`

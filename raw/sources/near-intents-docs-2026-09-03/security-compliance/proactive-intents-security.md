> ## Documentation Index
> Fetch the complete documentation index at: https://docs.near-intents.org/llms.txt
> Use this file to discover all available pages before exploring further.

# SHIELD: Proactive Intents Security

> How SHIELD proactively raises security posture across guarded NEAR Intents surfaces

SHIELD is the security policy layer behind NEAR Intents and the 1Click stack. It lets integrated services evaluate requests against incident state and shared security posture before continuing execution.

Instead of relying on a single global stop switch, SHIELD is built for scoped response. Today that means request-time decisions such as allow or delay on integrated flows, with broader orchestration expanding over time.

***

## Implementation status

Use this page as both a product direction overview and a status snapshot.

<CardGroup cols={1}>
  <Card title="Live today" icon="circle-check" horizontal>
    Incident submission and resolution, scoped permissions, security modes, and quote-time evaluation decisions for integrated Shield consumers.
  </Card>

  <Card title="Rolling out" icon="hourglass-half" horizontal>
    Broader runtime adoption of Shield decisions across additional request paths and operational surfaces.
  </Card>

  <Card title="Vision" icon="stars" horizontal>
    Multi-surface posture orchestration informed by richer operational, transaction, and chain-health signals.
  </Card>
</CardGroup>

***

## Architecture

At its core, SHIELD is a dedicated incident and policy service. Authorized producers submit incidents, and integrated consumers query SHIELD to determine how a request should proceed.

This architecture supports graceful degradation by applying scoped policy decisions at request time instead of immediately shutting down the full platform.

<CardGroup cols={1}>
  <Card title="Incident ingress" icon="tower-broadcast" horizontal>
    One write surface for incident creation and resolution with authentication, scoped authorization checks, and auditable records.
  </Card>

  <Card title="Security mode" icon="gauge-high" horizontal>
    Shared posture represented as coarse modes (`normal`, `paranoid`, `under_attack`) that integrated consumers can apply consistently.
  </Card>

  <Card title="Policy decisions" icon="sliders" horizontal>
    Evaluate endpoints return scoped policy decisions based on incidents, posture, and configured thresholds.
  </Card>
</CardGroup>

### Guarded surfaces

SHIELD can coordinate security posture across multiple parts of the platform:

<CardGroup cols={2}>
  <Card title="1Click swaps" icon="arrow-right-arrow-left">
    Live today for integrated quote-time checks, including scoped delay.
  </Card>

  <Card title="Bridge" icon="bridge">
    Rolling out as Shield decisions are adopted across additional execution paths and bridge-touching flows.
  </Card>

  <Card title="Solver pool" icon="wave-square">
    Vision: solver and liquidity controls can consume the same incident and posture model as integration expands.
  </Card>

  <Card title="Public status" icon="signal-stream">
    Live today through incident public descriptions that support partner-facing degraded-state messaging.
  </Card>
</CardGroup>

***

## Security Modes

SHIELD defines three operational modes. They are intentionally coarse so they are easy to reason about under pressure.

<CardGroup cols={1}>
  <Card title="Normal" icon="circle-check" horizontal>
    No friction. This is the default operating state.
  </Card>

  <Card title="Paranoid" icon="triangle-exclamation" horizontal>
    Buy time. Add delay windows and increase operational scrutiny while the situation is assessed.
  </Card>

  <Card title="Under attack" icon="ban" horizontal>
    Pause affected functionality and monitor in depth while responders contain the incident.
  </Card>
</CardGroup>

<Warning>
  The modes are designed to be simple on purpose. In an incident, operators benefit from clear posture changes more than they benefit from a large number of fine-grained states.
</Warning>

***

## Partner Permissions

The SHIELD signal bus is open to multiple producers, including partner integrations, on-call operators, internal anomaly detection, and internal tooling. Producers do not inherit capabilities automatically. Each producer receives an explicit permission set.

Typical capabilities include:

<CardGroup cols={2}>
  <Card title="Pause a destination" icon="circle-pause">
    Pause a single chain, token, or destination without escalating the whole platform.
  </Card>

  <Card title="Flag observation" icon="flag">
    Submit an observation for review without forcing a posture change.
  </Card>

  <Card title="Raise mode" icon="arrow-up-right-dots">
    Move a scoped surface or the full platform into `paranoid` or `under_attack`.
  </Card>

  <Card title="Read-only" icon="eye">
    Inspect status and history without changing platform posture.
  </Card>
</CardGroup>

These permissions are scoped. The model supports granular scope by chain, bridge, token, address, and security mode. Scope enforcement is rolling out incrementally. As newer request-evaluation paths adopt narrower chain-specific permissions, behavior may vary by integration path.

### Example permission models

<CardGroup cols={2}>
  <Card title="Internal incident responder" icon="user-shield">
    An internal on-call operator may be granted `raise mode` at platform scope so they can move the system into `paranoid` or `under_attack` while a real incident unfolds.
  </Card>

  <Card title="Consumer integration" icon="building-shield">
    A partner integration may be granted per-chain scope, plus read-only access elsewhere, so it can pause a chain it cares about without escalating the global posture.
  </Card>
</CardGroup>

<Warning>
  Capabilities are auditable in both directions. Partners can inspect their own action history, while internal operators can review the full action set by partner, scope, or effect.
</Warning>

***

## Anomaly Engine

SHIELD integrates a first-party anomaly engine as an additional decision input for quote evaluation.

The engine is rule-based and explainable rather than model-driven. It maps observed conditions to concrete decision outcomes.

<CardGroup cols={2}>
  <Card title="Current behavior" icon="database">
    In current integration, anomaly scoring is applied on quote evaluation under configured thresholds and feature flags.
  </Card>

  <Card title="Severity mapping" icon="sitemap">
    Elevated outcomes can increase friction (for example delay), while failures are handled defensively.
  </Card>
</CardGroup>

As integration expands, anomaly inputs can inform more surfaces beyond the current quote-centric path.

***

## Transaction and Account Monitoring

Proactive intent security is not only about chain-level conditions. It is also about identifying when a specific transaction, account, destination, or fund flow looks unusual in the context of the broader system.

SHIELD's policy model is designed to support transaction and account-level targeting across connected chains, including:

<CardGroup cols={2}>
  <Card title="Transaction-level anomalies" icon="receipt">
    Today: integrated quote requests can be delayed based on incident and anomaly outcomes.
  </Card>

  <Card title="Account and destination risk" icon="user-lock">
    Model support exists for scoped controls by recipient, sender, deposit address, and counterparties.
  </Card>

  <Card title="Suspicious flow of funds" icon="arrows-split-up-and-left">
    Rolling out: broader use of policy decisions across more end-to-end flow checkpoints.
  </Card>

  <Card title="Scoped response" icon="crosshairs">
    Vision: apply targeted friction to the transaction, account, route, or destination that needs attention without broad platform degradation.
  </Card>
</CardGroup>

This remains core to the SHIELD vision: proactive security should react not only to unhealthy chains, but also to suspicious activity tied to individual requests and participant accounts.

<Info>
  In practice, that means NEAR Intents can raise friction for a specific flow of funds while the rest of the system continues operating normally.
</Info>

***

## Chain Health

Different chains fail in different ways. A proof-of-work chain may lose hashrate. A proof-of-stake chain may stall on finality. An L2 may fall behind its sequencer.

SHIELD is being designed around a normalized health model. The target state is chain-family adapters that emit a shared output at the policy boundary.

<CardGroup cols={2}>
  <Card title="One health vocabulary" icon="heart-pulse">
    Vision: each adapter answers shared practical questions such as block production, finality health, throughput envelope, and anomaly conditions.
  </Card>

  <Card title="Native adapters" icon="plug">
    Vision: each chain family stays native to its own environment while translating to shared policy semantics.
  </Card>
</CardGroup>

Common adapter families include:

<CardGroup cols={2}>
  <Card title="Bitcoin / PoW family" icon="bitcoin-sign">
    Metrics such as hashrate and block cadence.
  </Card>

  <Card title="Ethereum / EVM family" icon="ethereum">
    Metrics such as finality, reorgs, and mempool behavior.
  </Card>

  <Card title="NEAR / PoS family" icon="circle-nodes">
    Metrics such as block production and validator health.
  </Card>

  <Card title="Other supported chains" icon="link">
    A family-specific adapter that translates chain-native signals into the shared health model.
  </Card>
</CardGroup>

<Info>
  Chain-health integration follows the same incident and policy model. As adapters are rolled out, they can feed incidents through shared authorization and audit paths.
</Info>

***

## Incident API

SHIELD exposes two conceptual surfaces:

<CardGroup cols={2}>
  <Card title="Status feed" icon="rss" horizontal>
    A read path for consumers who need a summary of current incidents and degraded conditions.
  </Card>

  <Card title="Incident submission" icon="paper-plane" horizontal>
    A write path for authorized producers. Requests are authenticated and scope-checked before incident state is changed.
  </Card>
</CardGroup>

The public surface is intentionally narrow. Detailed schemas are shared through partner-facing reference materials, while this page explains the purpose of the system and the security model behind it.

<Card title="Shield Incident API" icon="paper-plane" href="/security-compliance/shield-incident-api" horizontal>
  Partner-facing reference for pulling active incidents and submitting new ones — endpoints, auth, and scope.
</Card>

***

## Summary

SHIELD gives NEAR Intents a proactive way to raise security posture before an issue becomes system-wide damage. Today it combines shipped controls with an explicit roadmap:

<CardGroup cols={2}>
  <Card title="Implemented controls" icon="tower-cell">
    Incident management, scoped permissions, coarse security modes, and evaluate endpoints used by integrated consumers.
  </Card>

  <Card title="Scoped permissions" icon="key">
    Explicit producer capabilities instead of inherited authority.
  </Card>

  <Card title="Coarse security modes" icon="traffic-light">
    Simple postures that operators can apply quickly during an incident.
  </Card>

  <Card title="Roadmap-aligned vision" icon="shield">
    Progressive expansion from quote-centric safeguards toward broader, surface-specific response across the platform.
  </Card>
</CardGroup>

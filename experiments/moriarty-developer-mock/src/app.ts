import {
  advance,
  defaultConfig,
  evaluate,
  newSession,
  verifyRealProof,
  type Config,
  type Example,
  type Fault,
  type Session,
} from "./model.js";

type JsonRecord = Record<string, unknown>;
type FixtureEntry = {
  source: string;
  sha256: string;
  sourceCommit: string;
  provenance: string;
  fixture: { identifier?: string; terms?: JsonRecord; results?: JsonRecord[] };
};
type FixtureCorpus = Partial<Record<"loan" | "calendarBefore" | "calendarAfter" | "option", FixtureEntry>>;
type ProposedClaim = {
  kind: "IntentEffects" | "ContractInvariant" | "TransitionValidity" | "HistoryCompliance";
  required: true;
  status: "unavailable";
  reason: string;
};

const examples: { value: Example; label: string; detail: string }[] = [
  { value: "loan", label: "LAM repayment", detail: "Imported schedule and an illustrative first-period calculation" },
  { value: "calendar", label: "Business-day contrast", detail: "CSF and SCF imported fixture outcomes" },
  { value: "option", label: "Delayed option settlement", detail: "Exercise amount and later settlement event" },
  { value: "swap", label: "AMM swap", detail: "Integer quote, slippage and rounding mutation" },
  { value: "mandate", label: "Lending + mandate", detail: "Separate risk and allocator authority" },
  { value: "join", label: "Multi-parent join", detail: "Input identity, output and consumption accounting" },
  { value: "partial", label: "Cancellation + partial fill", detail: "Residual state and tab-local mock submission identity" },
];

const faults: { value: Fault; label: string }[] = [
  { value: "none", label: "No injected fault" },
  { value: "missing-observation", label: "Missing observation" },
  { value: "stale-observation", label: "Stale observation" },
  { value: "wrong-authority", label: "Wrong authority" },
  { value: "missing-proof", label: "Missing proof" },
  { value: "altered-proof", label: "Altered proof" },
  { value: "wrong-domain", label: "Wrong domain" },
  { value: "stale-predecessor", label: "Stale predecessor" },
  { value: "duplicate-consumption", label: "Duplicate consumption" },
  { value: "cancelled", label: "Cancelled intent" },
  { value: "rounding-mutation", label: "Rounding mutation" },
];

const storageKey = "moriarty.developer-mock.config.v1";
let config: Config = defaultConfig("loan");
let session: Session = newSession();
let consumed: string[] = [];
let fixtures: FixtureCorpus = {};
let selectedEvent = 0;
let workflowMessage = "Choose inputs, inspect the preview, then begin the DEMO-only workflow.";
let restored = false;

const proposedClaims: ProposedClaim[] = [
  { kind: "IntentEffects", required: true, status: "unavailable", reason: "No effect-constraint checker is connected." },
  { kind: "ContractInvariant", required: true, status: "unavailable", reason: "No contract property checker is connected." },
  { kind: "TransitionValidity", required: true, status: "unavailable", reason: "No real transition prover or verifier is connected." },
  { kind: "HistoryCompliance", required: true, status: "unavailable", reason: "No recursive proof or live ledger acceptance path is connected." },
];

function el<K extends keyof HTMLElementTagNameMap>(tag: K, className?: string, text?: string): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  if (className) node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

function card(kicker: string, title: string, body: HTMLElement, badge?: HTMLElement): HTMLElement {
  const article = el("section", "card");
  const head = el("div", "card-head");
  const heading = el("div");
  heading.append(el("p", "eyebrow", kicker), el("h2", "", title));
  head.append(heading);
  if (badge) head.append(badge);
  const wrap = el("div", "card-body");
  wrap.append(body);
  article.append(head, wrap);
  return article;
}

function tag(text: string, kind = ""): HTMLElement { return el("span", `tag ${kind}`.trim(), text); }

function selectControl<T extends string>(labelText: string, value: T, options: { value: T; label: string }[], onChange: (value: T) => void): HTMLLabelElement {
  const label = el("label", "");
  const select = el("select");
  select.setAttribute("aria-label", labelText);
  options.forEach((item) => {
    const option = el("option", "", item.label);
    option.value = item.value;
    option.selected = item.value === value;
    select.append(option);
  });
  select.addEventListener("change", () => onChange(select.value as T));
  label.append(document.createTextNode(labelText), select);
  return label;
}

function resetWorkflow(message: string): void {
  session = newSession();
  workflowMessage = message;
}

function saveConfig(): void {
  try { localStorage.setItem(storageKey, JSON.stringify(config)); } catch { /* Storage may be unavailable. */ }
}

function isExample(value: unknown): value is Example { return examples.some((item) => item.value === value); }
function isFault(value: unknown): value is Fault { return faults.some((item) => item.value === value); }

function restoreConfig(): void {
  try {
    const raw = localStorage.getItem(storageKey);
    if (!raw) return;
    const candidate = JSON.parse(raw) as Partial<Config>;
    if (!isExample(candidate.example) || !isFault(candidate.fault)) return;
    const base = defaultConfig(candidate.example);
    for (const key of Object.keys(base) as (keyof Config)[]) {
      const value = candidate[key];
      if (typeof value === "string") (base as Record<string, string>)[key] = value;
    }
    config = base;
    session = newSession();
    restored = true;
    workflowMessage = "Restored local configuration. The staged workflow was reset to stage 0.";
  } catch { /* Ignore malformed or inaccessible storage. */ }
}

function setExample(example: Example): void {
  config = defaultConfig(example);
  selectedEvent = 0;
  restored = false;
  resetWorkflow("Example changed. Prepared DEMO-only state was cleared.");
  saveConfig();
  render();
}

function setConfig(key: keyof Config, value: string): void {
  config = { ...config, [key]: value };
  restored = false;
  resetWorkflow("Inputs changed. Prepared authorization, proof, and submission state were cleared.");
  saveConfig();
  render();
}

function field(key: keyof Config, labelText: string, wide = false): HTMLLabelElement {
  const label = el("label", wide ? "wide" : "");
  const input = el("input");
  input.value = String(config[key]);
  input.name = String(key);
  input.autocomplete = "off";
  input.spellcheck = false;
  input.addEventListener("change", () => setConfig(key, input.value));
  label.append(document.createTextNode(labelText), input);
  return label;
}

function editableFields(): HTMLElement {
  const grid = el("div", "field-grid");
  if (config.example === "loan") {
    grid.append(field("principal", "Principal / notional"), field("rate", "Rate (decimal)"), field("days", "Accrual days"), field("actor", "Authority role", true));
  } else if (config.example === "calendar" || config.example === "option") {
    grid.append(field("actor", "Authority role", true));
  } else if (config.example === "swap" || config.example === "join" || config.example === "partial") {
    grid.append(field("reserveA", "Reserve A"), field("reserveB", "Reserve B"), field("amountIn", "Exact amount in"), field("minOut", "Minimum output"), field("actor", "Authority role", true));
    if (config.example === "partial") grid.append(el("p", "caption wide", "Fixed total allowance: 10,000 A. Each successor fill requires fresh state and authorization."));
  } else {
    grid.append(field("amountIn", "Requested allocation"), field("actor", "Authority role"), el("p", "caption wide", "Fixed destination cap: 20,000 units."));
  }
  return grid;
}

function fixtureForExample(): FixtureEntry[] {
  if (config.example === "loan") return fixtures.loan ? [fixtures.loan] : [];
  if (config.example === "calendar") return [fixtures.calendarBefore, fixtures.calendarAfter].filter((item): item is FixtureEntry => Boolean(item));
  if (config.example === "option") return fixtures.option ? [fixtures.option] : [];
  return [];
}

function renderTerms(entries: FixtureEntry[]): HTMLElement {
  const box = el("div");
  if (!entries.length) {
    box.append(el("p", "empty", "No imported fixture applies to this hand-worked example. The fields above feed only the local illustrative model."));
    return box;
  }
  entries.forEach((entry, index) => {
    if (entries.length > 1) box.append(tag(index === 0 ? "CSF fixture" : "SCF fixture", "imported"));
    const list = el("dl", "kv");
    Object.entries(entry.fixture.terms ?? {}).forEach(([key, value]) => {
      list.append(el("dt", "", key), el("dd", "", typeof value === "string" ? value : JSON.stringify(value)));
    });
    box.append(list);
  });
  return box;
}

function renderProvenance(entries: FixtureEntry[]): HTMLElement {
  const box = el("div", "source-meta");
  if (!entries.length) {
    box.append(tag("Calculated illustration"), el("p", "empty", "No third-party result corpus is used for this scenario."));
    return box;
  }
  entries.forEach((entry) => {
    box.append(tag("Imported fixture — not calculated", "imported"));
    box.append(el("p", "caption", entry.provenance));
    box.append(el("code", "", entry.source));
    box.append(el("code", "", `SHA-256 ${entry.sha256}`));
    box.append(el("code", "", `Source commit ${entry.sourceCommit}`));
  });
  return box;
}

function resultRows(entries: FixtureEntry[]): { origin: string; value: JsonRecord }[] {
  return entries.flatMap((entry) => (entry.fixture.results ?? []).map((value) => ({ origin: entry.fixture.identifier ?? "fixture", value })));
}

function renderEvents(entries: FixtureEntry[]): HTMLElement {
  const box = el("div");
  const rows = resultRows(entries);
  if (!rows.length) {
    box.append(el("p", "empty", "This example has no imported event trace. Its behavior below is a calculated illustration from the local model."));
    return box;
  }
  const fields = Array.from(new Set(rows.flatMap((row) => Object.keys(row.value))));
  const wrap = el("div", "table-wrap");
  const table = el("table");
  table.setAttribute("aria-label", "Imported fixture event results");
  const thead = el("thead");
  const header = el("tr");
  header.append(el("th", "", "Fixture"));
  fields.forEach((name) => header.append(el("th", "", name)));
  thead.append(header);
  const tbody = el("tbody");
  rows.forEach((row, index) => {
    const tr = el("tr", index === selectedEvent ? "selected" : "");
    tr.tabIndex = 0;
    tr.setAttribute("aria-selected", String(index === selectedEvent));
    tr.append(el("td", "", row.origin));
    fields.forEach((name) => tr.append(el("td", "", row.value[name] === undefined ? "—" : String(row.value[name]))));
    const choose = (): void => { selectedEvent = index; render(); };
    tr.addEventListener("click", choose);
    tr.addEventListener("keydown", (event) => { if (event.key === "Enter" || event.key === " ") { event.preventDefault(); choose(); } });
    tbody.append(tr);
  });
  table.append(thead, tbody);
  wrap.append(table);
  const current = rows[Math.min(selectedEvent, rows.length - 1)];
  const detail = el("dl", "kv");
  Object.entries(current.value).forEach(([key, value]) => detail.append(el("dt", "", key), el("dd", "", String(value))));
  box.append(wrap, el("p", "caption", "Selected imported event · every result field is preserved"), detail);
  return box;
}

function renderPreview(): HTMLElement {
  const preview = evaluate(config);
  const box = el("div");
  box.append(tag("Calculated illustration"));
  box.append(el("p", `status ${preview.ok ? "ok" : "fail"}`, preview.headline));
  const metrics = el("div", "metric-grid");
  preview.metrics.forEach(([name, value]) => {
    const metric = el("div", "metric");
    metric.append(el("span", "", name), el("strong", "", value));
    metrics.append(metric);
  });
  box.append(metrics);
  if (preview.diagnostics.length) {
    const title = el("p", "caption", "Diagnostics");
    const list = el("ul", "diagnostics");
    preview.diagnostics.forEach((diagnostic) => {
      const item = el("li", "diagnostic");
      item.append(el("code", "", diagnostic.code), document.createTextNode(diagnostic.message));
      list.append(item);
    });
    box.append(title, list);
  }
  if (preview.effects.length) {
    const title = el("p", "caption", "Proposed effects · not committed");
    const list = el("ul", "effects");
    preview.effects.forEach((effect) => list.append(el("li", "effect", effect)));
    box.append(title, list);
  }
  return box;
}

function properties(): HTMLElement {
  const box = el("ul", "properties");
  ["Principal / dues accounting", "Ordered accrual", "Authority and effect constraints"].forEach((name) => {
    const item = el("li", "property");
    item.append(el("span", "", name), tag("Unavailable", "unavailable"));
    box.append(item);
  });
  box.append(el("p", "caption", "No checker is connected. A successful illustration or imported trace does not prove these properties."));
  return box;
}

function claimsPanel(): HTMLElement {
  const box = el("div");
  const list = el("ul", "properties");
  proposedClaims.forEach((claim) => {
    const item = el("li", "property");
    const text = el("span");
    text.append(el("strong", "", claim.kind), el("small", "caption", `Required · ${claim.reason}`));
    item.append(text, tag("Unavailable", "unavailable"));
    list.append(item);
  });
  box.append(list, el("p", "caption", "Proposed PCTE claim interface only. No checker is implemented. Midnight-native Halo2 recursion is the first planned backend candidate; it is not connected here."));
  return box;
}

const stageLabels = ["Prepare plan", "Simulate authorization", "Generate simulated proof", "Check simulated proof", "Simulate ledger submission"];

function renderWorkflow(): HTMLElement {
  const box = el("div", "workflow");
  box.append(tag("DEMO only"));
  const stages = el("ol", "stage-list");
  stageLabels.forEach((name, index) => {
    const item = el("li", index < session.stage ? "done" : index === session.stage ? "current" : "");
    item.append(el("span", "dot", index < session.stage ? "✓" : String(index + 1)), el("span", "", name));
    stages.append(item);
  });
  const message = el("p", "workflow-message", workflowMessage);
  message.setAttribute("aria-live", "polite");
  const step = el("button", "", session.stage < stageLabels.length ? `DEMO · ${stageLabels[session.stage]}` : "DEMO workflow complete");
  step.id = "advance-workflow";
  step.disabled = session.stage >= stageLabels.length;
  step.addEventListener("click", () => {
    const outcome = advance(session, config, consumed);
    session = outcome.session;
    consumed = outcome.consumed;
    workflowMessage = outcome.message;
    render();
  });
  box.append(stages, message, step);
  if (session.statement) box.append(el("pre", "", session.statement));
  if (session.evidence) {
    box.append(tag("SimulatedEvidence / mock", ""));
    const evidence = el("pre");
    evidence.textContent = JSON.stringify(session.evidence, null, 2);
    box.append(evidence);
  }
  if (session.receipt) box.append(el("p", "caption", `Simulated receipt: ${session.receipt}`));
  return box;
}

function realActions(): HTMLElement {
  const box = el("div", "button-stack");
  ["Sign with wallet", "Verify real proof", "Submit to ledger"].forEach((name) => {
    const button = el("button", "secondary", name);
    button.disabled = true;
    box.append(button);
  });
  const unavailable = verifyRealProof(null);
  box.append(el("p", "blocked", `Real operations unavailable: ${unavailable.reason}`));
  return box;
}

function exportMock(): void {
  const preview = evaluate(config);
  const envelope = {
    kind: "MoriartyDeveloperMockExport",
    mock: true,
    evidenceKind: "SimulatedEvidence",
    config,
    preview,
    session,
    consumed,
    proposedClaims,
    realVerification: verifyRealProof(session.evidence),
  };
  const blob = new Blob([JSON.stringify(envelope, null, 2)], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const link = el("a");
  link.href = url;
  link.download = `moriarty-${config.example}-mock.json`;
  link.click();
  URL.revokeObjectURL(url);
  workflowMessage = "Exported a mock JSON envelope. It retains SimulatedEvidence and mock markers.";
  render();
}

function render(): void {
  const root = document.querySelector<HTMLElement>("#app");
  if (!root) return;
  root.replaceChildren();
  const shell = el("main", "shell");
  const topbar = el("header", "topbar");
  const brand = el("div", "brand");
  const brandText = el("div");
  brandText.append(el("strong", "", "Moriarty"), el("small", "", "developer workspace"));
  brand.append(el("span", "mark", "M"), brandText);
  const languageLink = el("a", "demo-badge", "Open executable R2");
  languageLink.setAttribute("href", "/language");
  const topActions = el("div", "top-actions");
  topActions.append(el("span", "demo-badge", "Local mock · no live systems"), languageLink);
  topbar.append(brand, topActions);

  const hero = el("section", "hero");
  const intro = el("div");
  intro.append(el("p", "eyebrow", "Agreement semantics workspace"), el("h1", "", "Inspect intent before evidence."), el("p", "", "Explore imported reference results beside local calculations, then walk through a clearly simulated authorization and proof lifecycle."));
  const controls = el("div", "hero-controls");
  controls.append(selectControl("Example", config.example, examples, setExample));
  const reset = el("button", "secondary", "Reset defaults");
  reset.addEventListener("click", () => {
    config = defaultConfig(config.example);
    selectedEvent = 0;
    restored = false;
    resetWorkflow("Defaults restored. Prepared DEMO-only state was cleared.");
    try { localStorage.removeItem(storageKey); } catch { /* Storage may be unavailable. */ }
    render();
  });
  const exportButton = el("button", "", "Export mock JSON");
  exportButton.id = "export-json";
  exportButton.addEventListener("click", exportMock);
  const resetLedger = el("button", "secondary", "Reset demo ledger");
  resetLedger.id = "reset-demo-ledger";
  resetLedger.addEventListener("click", () => {
    consumed = [];
    resetWorkflow("Tab-local demo ledger cleared. Prepared DEMO-only state was also cleared.");
    render();
  });
  controls.append(reset, resetLedger, exportButton);
  hero.append(intro, controls);

  const notice = el("div", "notice");
  notice.append(el("strong", "", restored ? "Restored context." : "Truthful boundary."), document.createTextNode(restored ? " Configuration was restored locally; workflow evidence was not restored and remains at stage 0." : " Imported fixtures are reference expectations. Edited previews are illustrations. No DSL parser, property checker, wallet, prover, or ledger is connected."));

  const entries = fixtureForExample();
  const workspace = el("div", "workspace");
  const agreement = el("div", "column agreement");
  agreement.append(
    card("01 · Agreement", examples.find((item) => item.value === config.example)?.label ?? "Agreement", editableFields(), tag("Editable")),
    card("Terms", entries.length ? "Imported source terms" : "Illustrative configuration", renderTerms(entries), entries.length ? tag("Imported", "imported") : tag("Calculated")),
    card("Source integrity", "Fixture provenance", renderProvenance(entries)),
  );

  const behavior = el("div", "column behavior");
  const faultControl = selectControl("Fault injection", config.fault, faults, (fault) => setConfig("fault", fault));
  behavior.append(
    card("02 · Behavior", "Imported event results", renderEvents(entries), entries.length ? tag("Imported · exact", "imported") : tag("No imported trace")),
    card("Calculated from edited fields", "Illustrative preview", renderPreview(), tag("Not fixture output")),
    card("Generated preview", "Illustrative source", (() => { const pre = el("pre"); pre.textContent = evaluate(config).source; return pre; })(), tag("Not parsed")),
    card("Fault laboratory", "Inject one named failure", faultControl, tag(config.fault === "none" ? "Inactive" : "Active", config.fault === "none" ? "" : "unavailable")),
  );

  const transaction = el("div", "column transaction");
  transaction.append(
    card("03 · Transaction", "Staged evidence workflow", renderWorkflow(), tag(`Stage ${session.stage} / 5`)),
    card("Proposed PCTE interface", "Required typed claims", claimsPanel(), tag("All unavailable", "unavailable")),
    card("Analysis", "Named properties", properties(), tag("Checker absent", "unavailable")),
    card("External effects", "Real signing, proof and submission", realActions(), tag("Unavailable", "unavailable")),
  );
  workspace.append(agreement, behavior, transaction);

  const footer = el("footer", "footer");
  footer.append(el("span", "", "Moriarty developer mock · all workflow evidence is simulated"), el("span", "", "Demo ledger is tab-local and resets on reload · configuration alone is stored locally"));
  shell.append(topbar, hero, notice, workspace, footer);
  root.append(shell);
}

async function boot(): Promise<void> {
  restoreConfig();
  try {
    const response = await fetch("/fixtures.json", { headers: { Accept: "application/json" } });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    fixtures = await response.json() as FixtureCorpus;
  } catch {
    workflowMessage = "Local fixture corpus could not be loaded. Calculated illustrations remain available.";
  }
  render();
}

void boot();

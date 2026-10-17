import React from "react";

const NOOP = () => {};
const VIEW_W = 760;
const VIEW_H = 340;
const HUB = { x: VIEW_W / 2, y: 166 };
const NODE_W = 210;
const NODE_H = 108;

function wrapLabel(value, maxChars = 23) {
  const words = String(value || "Future role").split(/\s+/);
  const lines = [];
  let line = "";
  for (const word of words) {
    const next = line ? `${line} ${word}` : word;
    if (next.length > maxChars && line) {
      lines.push(line);
      line = word;
    } else {
      line = next;
    }
  }
  if (line) lines.push(line);
  return lines.slice(0, 2);
}

function positionFor(index, count) {
  const angle = -Math.PI / 2 + (2 * Math.PI * index) / Math.max(count, 1);
  const cx = HUB.x + 250 * Math.cos(angle);
  const cy = 154 + 104 * Math.sin(angle);
  return { cx, cy, x: cx - NODE_W / 2, y: cy - NODE_H / 2 };
}

/**
 * An editable illustration of future role specifications for one proposed Web.
 * Selection and omission are controlled by the parent page.
 */
export default function WebDiagram({
  scenario,
  selectedStrand,
  onSelect = NOOP,
  omittedStrands = [],
  onToggleOmit = NOOP,
}) {
  const roles = Array.isArray(scenario?.strands) ? scenario.strands : [];
  const title = scenario?.title || "Proposed Web scenario";
  const omitted = new Set(Array.isArray(omittedStrands) ? omittedStrands : []);
  const positioned = roles.map((strand, index) => ({
    strand,
    index,
    position: positionFor(index, roles.length),
    isSelected: strand.id === selectedStrand,
    isOmitted: omitted.has(strand.id),
  }));
  const omittedCount = positioned.filter((item) => item.isOmitted).length;

  if (roles.length === 0) {
    return (
      <figure className="web-diagram" aria-label={`${title}: future role illustration`}>
        <p className="web-diagram__empty">No future role specifications are listed for this scenario.</p>
      </figure>
    );
  }

  const perimeter = positioned.map((item, index) => {
    const next = positioned[(index + 1) % positioned.length];
    if (!next || positioned.length < 2) return null;
    const { cx: x1, cy: y1 } = item.position;
    const { cx: x2, cy: y2 } = next.position;
    const incomplete = item.isOmitted || next.isOmitted;
    return (
      <path
        key={`web-edge-${item.strand.id}-${next.strand.id}`}
        className="web-diagram__edge web-diagram__edge--perimeter"
        d={`M ${x1} ${y1} Q ${HUB.x} ${HUB.y} ${x2} ${y2}`}
        fill="none"
        stroke="currentColor"
        strokeWidth={incomplete ? 2 : 3}
        strokeDasharray={incomplete ? "8 7" : undefined}
      />
    );
  });

  return (
    <figure className="web-diagram" aria-label={`${title}: proposed future proof-role illustration`}>
      <div className="web-diagram__visual" aria-hidden="true">
        <svg
          className="web-diagram__svg"
          viewBox={`0 0 ${VIEW_W} ${VIEW_H}`}
          preserveAspectRatio="xMidYMid meet"
          focusable="false"
        >
          <g className="web-diagram__connections">
            {positioned.map(({ strand, position, isOmitted }) => (
              <path
                key={`hub-edge-${strand.id}`}
                className="web-diagram__edge"
                d={`M ${HUB.x} ${HUB.y} L ${position.cx} ${position.cy}`}
                fill="none"
                stroke="currentColor"
                strokeWidth={isOmitted ? 2 : 3}
                strokeDasharray={isOmitted ? "8 7" : undefined}
              />
            ))}
            {perimeter}
          </g>
          <g className="web-diagram__hub">
            <circle cx={HUB.x} cy={HUB.y} r="45" fill="var(--web-hub-fill, #f4efe5)" stroke="currentColor" strokeWidth="2" />
            <text x={HUB.x} y={HUB.y - 4} textAnchor="middle" className="web-diagram__hub-label">FUTURE</text>
            <text x={HUB.x} y={HUB.y + 14} textAnchor="middle" className="web-diagram__hub-label">REQUIRED SET</text>
          </g>
          {positioned.map(({ strand, index, position, isSelected, isOmitted }) => {
            const lines = wrapLabel(strand.label);
            const stateLabel = `${isSelected ? "SELECTED · " : ""}${isOmitted ? "OMITTED" : "REQUIRED"}`;
            return (
              <g
                key={strand.id}
                className={`web-diagram__role ${isSelected ? "is-selected" : ""} ${isOmitted ? "is-omitted" : ""}`}
              >
                <rect
                  className="web-diagram__role-shape"
                  x={position.x}
                  y={position.y}
                  width={NODE_W}
                  height={NODE_H}
                  rx="16"
                  fill="var(--web-node-fill, #fffdf8)"
                  stroke="currentColor"
                  strokeWidth={isSelected ? 5 : 2}
                  strokeDasharray={isOmitted ? "8 6" : undefined}
                />
                <text x={position.cx} y={position.cy - 25} textAnchor="middle" className="web-diagram__role-number">
                  {`FUTURE ROLE ${String(index + 1).padStart(2, "0")}`}
                </text>
                {lines.map((line, lineIndex) => (
                  <text
                    key={`${strand.id}-label-${lineIndex}`}
                    x={position.cx}
                    y={position.cy + lineIndex * 17 - (lines.length > 1 ? 1 : -7)}
                    textAnchor="middle"
                    className="web-diagram__role-label"
                  >
                    {line}
                  </text>
                ))}
                <text x={position.cx} y={position.cy + 40} textAnchor="middle" className="web-diagram__role-state">
                  {stateLabel}
                </text>
              </g>
            );
          })}
        </svg>
      </div>

      <figcaption className="web-diagram__caption">
        <strong>Illustration of one proposed Web’s three required FUTURE proof-role specifications.</strong> These are separate role requirements, not three independent proof producers, existing producers, concrete Strands, proof instances, a verified set, or settlement. A native whole-execution proof may cover several arithmetic conditions. Omitting a required role marks this illustration incomplete.
      </figcaption>

      <p className="web-diagram__state" aria-live="polite">
        {omittedCount === 0
          ? "Illustration state: all listed future roles are included."
          : `Illustration state: ${omittedCount} of ${roles.length} listed future roles omitted; this illustrated set is incomplete.`}
      </p>

      <div className="web-diagram__controls" role="group" aria-label={`Future role controls for ${title}`}>
        {positioned.map(({ strand, index, isSelected, isOmitted }) => (
          <article className={`strand-node ${isSelected ? "is-selected" : ""} ${isOmitted ? "is-omitted" : ""}`} key={strand.id}>
            <p className="strand-node__eyebrow">Future role specification {index + 1}</p>
            <h3 className="strand-node__title">{strand.label}</h3>
            <p className="strand-node__claim"><strong>Claim:</strong> {strand.claim}</p>
            <p className="strand-node__statement"><strong>Statement:</strong> {strand.statement}</p>
            <p className="strand-node__authentication"><strong>Context and authentication:</strong> {strand.authentication}</p>
            <p className="strand-node__limits"><strong>Limits:</strong> {strand.limits}</p>
            <p className="strand-node__status">Status: illustrative future proof role. No concrete proof instance is supplied.</p>
            <div className="strand-node__actions">
              <button
                className="strand-node__button strand-node__button--select"
                type="button"
                aria-pressed={isSelected}
                aria-label={`${isSelected ? "Selected" : "Select"} ${strand.label} in ${title}`}
                style={{ minHeight: "44px", minWidth: "44px" }}
                onClick={() => onSelect(strand.id)}
              >
                {isSelected ? "Selected role" : "Inspect role"}
              </button>
              <button
                className="strand-node__button strand-node__button--omit"
                type="button"
                aria-pressed={isOmitted}
                aria-label={`${isOmitted ? "Restore" : "Omit"} ${strand.label} in this illustration`}
                style={{ minHeight: "44px", minWidth: "44px" }}
                onClick={() => onToggleOmit(strand.id)}
              >
                {isOmitted ? "Restore role" : "Mark omitted"}
              </button>
            </div>
          </article>
        ))}
      </div>
    </figure>
  );
}

import React, {useId, useLayoutEffect, useRef, useState} from 'react';

/** A controlled view of the undirected neighbourhood in supplied relations. */
export default function ConcernGraph({layers, selected, onSelect}) {
  const graphId = useId();
  const titleId = `${graphId}-title`;
  const summaryId = `${graphId}-summary`;
  const captionId = `${graphId}-caption`;
  const networkRef = useRef(null);
  const nodeRefs = useRef(new Map());
  const [paths, setPaths] = useState([]);
  const pairs = relationPairs(layers);
  const pairSignature = pairs.map(pair => pair.join(':')).join('|');
  const active = layers.find(layer => layer.number === selected);
  const neighbours = new Set(pairs.flatMap(([a, b]) => a === selected ? [b] : b === selected ? [a] : []));
  const related = layers.filter(layer => neighbours.has(layer.number));
  const others = layers.filter(layer => layer.number !== active?.number);

  useLayoutEffect(() => {
    const network = networkRef.current;
    if (!network) return undefined;

    // Read actual button bounds after wrapping. Paths stay within the central
    // gutter, so no connection passes through an unrelated concern's card.
    const measure = () => {
      const origin = network.getBoundingClientRect();
      const source = nodeRefs.current.get(selected)?.getBoundingClientRect();
      const connected = relationPairs(layers).flatMap(([a, b]) => a === selected ? [b] : b === selected ? [a] : []);
      const next = source ? connected.map((number, index) => {
        const target = nodeRefs.current.get(number)?.getBoundingClientRect();
        if (!target) return null;
        const centerX = origin.width / 2;
        const targetIsLeft = target.x + target.width / 2 < source.x + source.width / 2;
        const side = targetIsLeft ? -1 : 1;
        const laneX = centerX + side * (5 + index % 3 * 4);
        const startX = centerX + side * 10;
        const startY = source.bottom - origin.y;
        const endX = (targetIsLeft ? target.right : target.left) - origin.x;
        const endY = target.y + target.height / 2 - origin.y;
        const bend = Math.min(18, Math.abs(endX - laneX));
        return {
          number,
          d: `M ${startX} ${startY} C ${startX} ${startY + 22}, ${laneX} ${startY + 22}, ${laneX} ${startY + 38} L ${laneX} ${endY - 14} Q ${laneX} ${endY}, ${laneX + side * bend} ${endY} L ${endX} ${endY}`,
        };
      }).filter(Boolean) : [];
      setPaths(previous => JSON.stringify(previous) === JSON.stringify(next) ? previous : next);
    };
    measure();
    const observer = typeof ResizeObserver === 'undefined' ? null : new ResizeObserver(measure);
    observer?.observe(network);
    nodeRefs.current.forEach(node => observer?.observe(node));
    window.addEventListener('resize', measure);
    return () => {
      observer?.disconnect();
      window.removeEventListener('resize', measure);
    };
  }, [selected, pairSignature, layers]);

  return (
    <section className="concern-graph" aria-labelledby={titleId}>
      <header className="cg-heading">
        <h3 id={titleId}>How the concerns relate</h3>
        <p>Select a concern to trace its immediate relationships. The numbers identify concerns; they are not execution steps.</p>
      </header>

      <div className="cg-network" ref={networkRef} role="group" aria-label="Choose a concern" aria-describedby={captionId}>
        <svg className="cg-lines" aria-hidden="true" focusable="false">
          {paths.map(path => <path key={path.number} d={path.d} data-related-edge={path.number}/>)}
        </svg>
        <p className="cg-relation-label"><span aria-hidden="true"/>Related concerns<span aria-hidden="true"/></p>
        {layers.map(layer => {
          const isActive = layer.number === active?.number;
          const isRelated = neighbours.has(layer.number);
          const slot = others.findIndex(other => other.number === layer.number);
          return (
            <button
              key={layer.number}
              ref={element => element ? nodeRefs.current.set(layer.number, element) : nodeRefs.current.delete(layer.number)}
              type="button"
              className={`cg-node${isActive ? ' cg-node-selected' : ''}${isRelated ? ' cg-node-related' : ''}`}
              style={{gridColumn: isActive ? '1 / -1' : slot % 2 + 1, gridRow: isActive ? 1 : 3 + Math.floor(slot / 2)}}
              data-layer={layer.number}
              aria-pressed={isActive}
              aria-controls={summaryId}
              onClick={() => onSelect?.(layer.number)}
            >
              <span className="cg-number" aria-hidden="true">{layer.number}</span>
              <strong className="cg-name">{layer.name}</strong>
              {layer.subtitle && <span className="cg-subtitle">{layer.subtitle}</span>}
              <span className="cg-node-status">{isActive ? 'Selected concern' : isRelated ? 'Related concern' : 'Other concern'}</span>
            </button>
          );
        })}
      </div>

      <div className="cg-summary" id={summaryId} aria-live="polite" aria-atomic="true">
        {active ? <>
          <p><strong>{active.number} · {active.name}</strong></p>
          <p>{related.length > 0 ? `Related concerns: ${related.map(layer => `${layer.number} · ${layer.name}`).join('; ')}.` : 'No direct relationships are supplied for this concern.'}</p>
        </> : <p>Select a concern to show its related concerns.</p>}
      </div>
      <p className="cg-caption" id={captionId}>Lines show only relationships touching the selected concern. Each line means “related concerns”, with no direction. A supplied link from either concern is shown once; this is not a complete map of every relationship.</p>
    </section>
  );
}

/** Reciprocal declarations represent one pair. Unknown/self links add no edge. */
export function relationPairs(layers) {
  const available = new Set(layers.map(layer => layer.number));
  const pairs = new Map();
  layers.forEach(layer => (layer.dependencies ?? []).forEach(number => {
    if (!available.has(number) || number === layer.number) return;
    const pair = [layer.number, number].sort((a, b) => a - b);
    pairs.set(pair.join(':'), pair);
  }));
  return [...pairs.values()];
}

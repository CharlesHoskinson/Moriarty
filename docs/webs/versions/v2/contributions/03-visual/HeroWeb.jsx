import React, { useId } from 'react';

/** Original GPT-authored decorative engraving. It depicts no protocol topology. */
export default function HeroWeb({ className = '' }) {
  const id = `web-${useId().replace(/:/g, '')}`;
  const rings = [72, 108, 144, 180, 216];
  const rays = Array.from({ length: 24 }, (_, i) => i * 15);
  const knots = [
    [116, 174], [151, 322], [260, 112], [396, 172],
    [417, 309], [292, 417], [242, 266], [330, 264],
  ];
  return (
    <svg className={`hero-web ${className}`} viewBox="0 0 540 540" fill="none" aria-hidden="true" focusable="false">
      <defs>
        <pattern id={`${id}-hatch`} width="8" height="8" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">
          <path d="M0 0V8" stroke="currentColor" strokeWidth="0.5" />
        </pattern>
        <radialGradient id={`${id}-paper`}>
          <stop offset="0" stopColor="#fffdf7" stopOpacity="0.9" />
          <stop offset="1" stopColor="#fffdf7" stopOpacity="0" />
        </radialGradient>
      </defs>
      <circle cx="270" cy="270" r="260" fill={`url(#${id}-paper)`} />
      <g className="hero-web__construction" stroke="currentColor" strokeWidth="0.65">
        <path d="M270 20V520M20 270H520M91 91L449 449M91 449L449 91" strokeDasharray="2 7" />
        <circle cx="270" cy="270" r="246" />
        <circle cx="270" cy="270" r="239" />
        {rays.map(angle => <path key={angle} d="M270 18V30" transform={`rotate(${angle} 270 270)`} />)}
        <path d="M21 250V290M13 262V278M519 250V290M527 262V278M250 21H290M262 13H278M250 519H290M262 527H278" />
      </g>
      <g className="hero-web__filaments" stroke="currentColor" strokeWidth="0.9">
        {rings.map((r, i) => <ellipse key={r} cx="270" cy="270" rx={r} ry={r * 0.48} transform={`rotate(${i * 31 - 61} 270 270)`} />)}
        <path d="M69 184C180 140 163 337 292 417C375 470 433 367 417 309C396 232 338 299 242 266C151 235 185 120 260 112C351 101 429 214 396 172" />
        <path d="M151 322C179 423 354 423 396 172C418 44 206 69 116 174C31 274 131 374 242 266C314 196 361 190 417 309" />
        <path d="M116 174C233 126 341 146 396 172M151 322C229 365 350 372 417 309M260 112C231 196 210 304 292 417" />
      </g>
      <g className="hero-web__weave" strokeWidth="2.1" strokeLinecap="round">
        <path d="M116 174C204 131 281 170 330 264C354 310 386 327 417 309" stroke="var(--copper, #925e3c)" />
        <path d="M151 322C191 321 208 283 242 266C303 235 337 203 396 172" stroke="var(--forest, #294b3e)" />
        <path d="M260 112C249 171 265 222 330 264C380 296 364 377 292 417" stroke="var(--forest, #294b3e)" />
      </g>
      <g className="hero-web__knots">
        {knots.map(([x, y], i) => (
          <g key={`${x}-${y}`}>
            <circle cx={x} cy={y} r={i > 5 ? 12 : 9} fill="var(--paper, #f5f1e8)" stroke="currentColor" strokeWidth="1" />
            <circle cx={x} cy={y} r={i > 5 ? 4 : 2.8} fill={i % 3 === 0 ? 'var(--copper, #925e3c)' : 'var(--forest, #294b3e)'} />
          </g>
        ))}
      </g>
      <g className="hero-web__marginalia" stroke="currentColor" strokeWidth="0.75">
        <path d="M53 133H95L110 165M424 309H472V350M292 434V475H329" />
        <path d="M49 129V137M45 133H53M468 350H476M329 471V479" />
        <circle cx="270" cy="270" r="31" fill={`url(#${id}-hatch)`} stroke="none" />
      </g>
    </svg>
  );
}

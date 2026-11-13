// DOM label layer: short labels anchored to 3D points, projected every frame, hidden behind the camera.
import * as THREE from 'three/webgpu';

export const MAX_SCENE_LABELS = 6;

// Text helpers (labels and narration accept this small HTML subset).
export const sup = (base, exp) => `${base}<sup>${exp}</sup>`;
export const pow2 = (n) => sup('2', n);
export const ident = (name) => `<code>${name}</code>`;

const KINDS = new Set(['program', 'input', 'outcome', 'step', 'state', 'error', 'neutral', 'zoneoff', 'zoneon']);

export class LabelLayer {
  constructor(container) {
    this.root = container;
    this.els = new Map();
    this.queue = [];
    this.fixed = [];   // rects {x0,y0,x1,y1} of fixed shell elements that labels keep clear of (location indicator, later)
    this._v = new THREE.Vector3();
  }

  begin() { this.queue.length = 0; }

  // pos: world-space Vector3. opts: kind (colour meaning), carrier (not counted in the cap of six).
  show(id, html, pos, { kind = 'neutral', carrier = false, compact = false } = {}) {
    this.queue.push({ id, html, pos: pos.clone(), kind: KINDS.has(kind) ? kind : 'neutral', carrier, compact });
  }

  // Projects the queued labels with `camera` for a stage of w by h CSS pixels, keeps every label inside the
  // stage and moves a label up when it would cover one that is already placed (carrier labels are placed first).
  flush(camera, w, h) {
    camera.updateMatrixWorld();
    const seen = new Set();
    const items = [];
    let scene = 0;
    for (const q of this.queue) {
      if (!q.carrier && ++scene > MAX_SCENE_LABELS) continue;
      let el = this.els.get(q.id);
      if (!el) {
        el = document.createElement('div');
        el.className = 'pl-label';
        el.innerHTML = '<span class="pl-chip"></span><span class="pl-lt"></span>';
        this.root.appendChild(el);
        this.els.set(q.id, el);
      }
      const txt = el.lastChild;
      if (txt._html !== q.html) { txt.innerHTML = q.html; txt._html = q.html; }
      const cls = `pl-label pl-k-${q.kind}${q.compact ? ' pl-compact' : ''}`;
      if (el._cls !== cls) { el.className = cls; el._cls = cls; }
      const cam = this._v.copy(q.pos).applyMatrix4(camera.matrixWorldInverse);
      const behind = cam.z >= -camera.near;
      let x = 0, y = 0, visible = !behind;
      if (visible) {
        const p = this._v.copy(q.pos).project(camera);
        x = Math.round((p.x * 0.5 + 0.5) * w);
        y = Math.round((-p.y * 0.5 + 0.5) * h);
        visible = x > -40 && x < w + 40 && y > -20 && y < h + 20;
      }
      el.style.display = visible ? '' : 'none';
      if (visible) items.push({ el, x, y, carrier: q.carrier, fixedSpot: q.id.startsWith('c:') });
      seen.add(q.id);
    }
    for (const [id, el] of this.els) if (!seen.has(id)) el.style.display = 'none';
    // Placement. Integer pixels: a fractional offset rasterises differently between runs.
    items.sort((a, b) => (b.carrier ? 1 : 0) - (a.carrier ? 1 : 0));
    const placed = this.fixed.slice();
    const GAP = 3;
    for (const it of items) {
      const bw = it.el.offsetWidth, bh = it.el.offsetHeight;
      let x0 = it.x - Math.round(bw / 2);
      let y0 = it.y - bh - 6;                       // margin-top of the label is -6
      x0 = Math.max(4, Math.min(w - bw - 4, x0));
      y0 = Math.max(4, Math.min(h - bh - 4, y0));
      if (!it.fixedSpot) {
        // Free spot nearest the anchor: same place, then up and down in label-height steps, each with sideways
        // shifts; with no free spot, the candidate that covers the least area.
        const inside = (cx, cy) => cx >= 4 && cx + bw <= w - 4 && cy >= 4 && cy + bh <= h - 4;
        const cover = (cx, cy) => placed.reduce((sum, r) => sum + Math.max(0, Math.min(cx + bw, r.x1 + GAP) - Math.max(cx, r.x0 - GAP)) * Math.max(0, Math.min(cy + bh, r.y1 + GAP) - Math.max(cy, r.y0 - GAP)), 0);
        let best = null;
        search: for (let n = 0; n <= 30; n++) {
          for (const sy of n === 0 ? [0] : [-1, 1]) {
            for (const sx of [0, 0.55, -0.55, 1.1, -1.1]) {
              const cx = Math.max(4, Math.min(w - bw - 4, x0 + Math.round(sx * bw))), cy = y0 + sy * n * ((bh + GAP) / 3);
              if (!inside(cx, cy)) continue;
              const c = cover(cx, cy);
              if (c === 0) { best = { cx, cy, c }; break search; }
              if (!best || c < best.c) best = { cx, cy, c };
            }
          }
        }
        if (best && best.c > 0 && w < 600 && !it.carrier) { it.el.style.display = 'none'; continue; }   // phone: no free spot, no label
        if (best) { x0 = best.cx; y0 = best.cy; }
      }
      placed.push({ x0, y0, x1: x0 + bw, y1: y0 + bh });
      it.el.style.transform = `translate(${x0}px, ${y0 + 6}px)`;
    }
  }
}

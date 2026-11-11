// DOM label layer: short labels anchored to 3D points, projected every frame, hidden behind the camera.
import * as THREE from 'three/webgpu';

export const MAX_SCENE_LABELS = 6;

// Text helpers (labels and narration accept this small HTML subset).
export const sup = (base, exp) => `${base}<sup>${exp}</sup>`;
export const pow2 = (n) => sup('2', n);
export const ident = (name) => `<code>${name}</code>`;

const KINDS = new Set(['program', 'input', 'outcome', 'step', 'state', 'error', 'neutral']);

export class LabelLayer {
  constructor(container) {
    this.root = container;
    this.els = new Map();
    this.queue = [];
    this._v = new THREE.Vector3();
  }

  begin() { this.queue.length = 0; }

  // pos: world-space Vector3. opts: kind (colour meaning), carrier (not counted in the cap of six).
  show(id, html, pos, { kind = 'neutral', carrier = false } = {}) {
    this.queue.push({ id, html, pos: pos.clone(), kind: KINDS.has(kind) ? kind : 'neutral', carrier });
  }

  // Projects the queued labels with `camera` for a stage of w by h CSS pixels.
  flush(camera, w, h) {
    camera.updateMatrixWorld();
    const seen = new Set();
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
      const cls = `pl-label pl-k-${q.kind}`;
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
      if (visible) el.style.transform = `translate(${x}px, ${y}px) translate(-50%, -100%)`;
      seen.add(q.id);
    }
    for (const [id, el] of this.els) if (!seen.has(id)) el.style.display = 'none';
  }
}

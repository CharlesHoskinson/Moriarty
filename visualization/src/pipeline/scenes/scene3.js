import * as THREE from 'three/webgpu';
import { materials, makeDigestToken, lerp, phase } from '../theme.js';

// Placeholder scene 3. The interface is documented in scene1.js and ../SCENE-API.md.
let ctx = null;
let box = null;
let token = null;

export default {
  id: "leaf",
  title: "The leaf",
  beats: [
    {
      id: 'arrive',
      title: 'What arrives',
      duration: 12,
      narration: "One leaf proves a slice of 16 steps. It is one circuit with one key. CORE sends requests over a single lookup bus, and the ALU, MEM, HASH and ROM chips answer them.",
      labels: [{ id: 'title', text: "Leaf circuit", anchor: 'box', kind: "state", dy: 2.2 }],
    },
    {
      id: 'inspect',
      title: 'What the station does',
      duration: 14,
      narration: "The leaf publishes seven lanes: binding, prog_d, pre_d, post_d, rules_d, h_pre and steps. The lanes bind the slice to the program and to the states before and after it.",
      labels: [{ id: 'id', text: '<code>' + "pre_d" + '</code>', anchor: 'token', kind: "state", dy: 0.9 }],
    },
  ],

  build(c) {
    ctx = c;
    box = new THREE.Mesh(new THREE.BoxGeometry(6, 3, 3), materials.family("state"));
    box.position.y = 1.5;
    ctx.scene.add(box);
    token = makeDigestToken({ family: "state", content: "leaf", size: 2 });
    token.position.set(0, 4.5, 0);
    ctx.scene.add(token);
    ctx.labels.anchor('box', box);
    ctx.labels.anchor('token', token);
    const s = ctx.camera.shots;
    ctx.camera.track([
      { t: 0, pose: s.establish([0, 1.5, 0], 34), dur: 1.6 },
      { t: 12, pose: s.inspect([0, 3, 0], 12, 'front'), dur: 2 },
    ]);
  },

  seek(t) {
    const b = ctx.beatAt(t);
    box.position.x = b.index === 0 ? lerp(-10, 0, phase(b.t, 0.5, 2.5)) : 0;
    token.visible = b.index === 1;
    token.scale.setScalar(b.index === 1 ? 0.2 + 0.8 * phase(b.t, 0.5, 1.5) : 1);
  },

  dispose() {
    ctx = null;
    box = null;
    token = null;
  },
};

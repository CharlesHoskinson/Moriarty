import * as THREE from 'three/webgpu';
import { materials, makeDigestToken, lerp, phase } from '../theme.js';

// Placeholder scene 2. The interface is documented in scene1.js and ../SCENE-API.md.
let ctx = null;
let box = null;
let token = null;

export default {
  id: "run",
  title: "The run",
  beats: [
    {
      id: 'arrive',
      title: 'What arrives',
      duration: 12,
      narration: "mori simulate runs a CEK-style machine. Its state has six parts: control, environment, frame stack, task stack, memory and fuel counters. Each step fires exactly one of 120 enabled rules.",
      labels: [{ id: 'title', text: "Machine step", anchor: 'box', kind: "step", dy: 2.2 }],
    },
    {
      id: 'inspect',
      title: 'What the station does',
      duration: 14,
      narration: "Each step prints one frame on the witness tape: the rule id, the state before and the state after. The run ends as Halted with a value, Rejected with a code or Exhausted with a kind. A 64-bit counter counts the steps.",
      labels: [{ id: 'id', text: '<code>' + "rule id" + '</code>', anchor: 'token', kind: "step", dy: 0.9 }],
    },
  ],

  build(c) {
    ctx = c;
    box = new THREE.Mesh(new THREE.BoxGeometry(6, 3, 3), materials.family("step"));
    box.position.y = 1.5;
    ctx.scene.add(box);
    token = makeDigestToken({ family: "step", content: "run", size: 2 });
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

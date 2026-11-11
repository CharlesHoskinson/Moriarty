import * as THREE from 'three/webgpu';
import { materials, makeDigestToken, lerp, phase } from '../theme.js';

// Placeholder scene 5. The interface is documented in scene1.js and ../SCENE-API.md.
let ctx = null;
let box = null;
let token = null;

export default {
  id: "decider",
  title: "The decider",
  beats: [
    {
      id: 'arrive',
      title: 'What arrives',
      duration: 12,
      narration: "The decider seals the root proof. The genesis leaf ran INIT, so h_lo is 0. The decider opens the header of the final post_d and requires a terminal outcome. A running state cannot seal.",
      labels: [{ id: 'title', text: "Decider seal", anchor: 'box', kind: "outcome", dy: 2.2 }],
    },
    {
      id: 'inspect',
      title: 'What the station does',
      duration: 14,
      narration: "It computes outcome_d and claim_d, verifies the root proof under pinned keys and pairs the accumulator once. Exactly four values leave: prog_d, input_d, outcome_d and n_steps.",
      labels: [{ id: 'id', text: '<code>' + "outcome_d" + '</code>', anchor: 'token', kind: "outcome", dy: 0.9 }],
    },
  ],

  build(c) {
    ctx = c;
    box = new THREE.Mesh(new THREE.BoxGeometry(6, 3, 3), materials.family("outcome"));
    box.position.y = 1.5;
    ctx.scene.add(box);
    token = makeDigestToken({ family: "outcome", content: "decider", size: 2 });
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

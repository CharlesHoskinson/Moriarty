import * as THREE from 'three/webgpu';
import { materials, makeDigestToken, lerp, phase } from '../theme.js';

// Placeholder scene 7. The interface is documented in scene1.js and ../SCENE-API.md.
let ctx = null;
let box = null;
let token = null;

export default {
  id: "contract",
  title: "The contract",
  beats: [
    {
      id: 'arrive',
      title: 'What arrives',
      duration: 12,
      narration: "acceptRun verifies the proof against its verifier key and recomputes claim_d. It refuses a claim_d that is already in runs, also when the same run is proved again under another leaf partition.",
      labels: [{ id: 'title', text: "SherlockRun", anchor: 'box', kind: "state", dy: 2.2 }],
    },
    {
      id: 'inspect',
      title: 'What the station does',
      duration: 14,
      narration: "It stores outcome_d and n_steps under claim_d. A later run can consume this claim at its seal, and a proof from another system enters through the registry.",
      labels: [{ id: 'id', text: '<code>' + "claim_d" + '</code>', anchor: 'token', kind: "state", dy: 0.9 }],
    },
  ],

  build(c) {
    ctx = c;
    box = new THREE.Mesh(new THREE.BoxGeometry(6, 3, 3), materials.family("state"));
    box.position.y = 1.5;
    ctx.scene.add(box);
    token = makeDigestToken({ family: "state", content: "contract", size: 2 });
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

import * as THREE from 'three/webgpu';
import { materials, makeDigestToken, lerp, phase } from '../theme.js';

// Placeholder scene 6. The interface is documented in scene1.js and ../SCENE-API.md.
let ctx = null;
let box = null;
let token = null;

export default {
  id: "transaction",
  title: "The transaction",
  beats: [
    {
      id: 'arrive',
      title: 'What arrives',
      duration: 12,
      narration: "The prover wraps the decider proof and the four public values in a Midnight transaction that calls SherlockRun.acceptRun. The call transcript binds the four values, so changing any of them invalidates the proof.",
      labels: [{ id: 'title', text: "Envelope", anchor: 'box', kind: "neutral", dy: 2.2 }],
    },
    {
      id: 'inspect',
      title: 'What the station does',
      duration: 14,
      narration: "Fees are paid in DUST. The proof is about 6.4 KB. The envelope leaves the prover's side of the bench and crosses to the chain.",
      labels: [{ id: 'id', text: '<code>' + "acceptRun" + '</code>', anchor: 'token', kind: "neutral", dy: 0.9 }],
    },
  ],

  build(c) {
    ctx = c;
    box = new THREE.Mesh(new THREE.BoxGeometry(6, 3, 3), materials.family("neutral"));
    box.position.y = 1.5;
    ctx.scene.add(box);
    token = makeDigestToken({ family: "neutral", content: "transaction", size: 2 });
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

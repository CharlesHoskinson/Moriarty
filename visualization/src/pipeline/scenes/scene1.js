// SCENE INTERFACE (full description in ../SCENE-API.md)
//
//   export default {
//     id: 'program',                 // short stable id
//     title: 'The program',          // shown in the scene list and the panel
//     beats: [{ id, title?, duration, narration, labels }],
//         // title: beat heading in the panel (default: the id with spaces)
//         // duration: seconds (reading time: words / 2.5, at least 12)
//         // narration: full prose, shown in the panel. Paragraphs split on a blank line.
//         //            Allowed markup: <sup>, <code>, <em>.
//         // labels: [{ id?, text, anchor: 'name' | at: [x,y,z], kind, dy?, from?, to? }]
//         //            anchor names come from ctx.labels.anchor(name, object3d); `at` is scene-local.
//         //            from/to are seconds into the beat; the default is the whole beat.
//     build(ctx),                    // once: create objects under ctx.scene, set ctx.camera.track(...)
//     seek(t),                       // t = seconds from the start of the scene. Set EVERYTHING from t alone.
//     dispose(),                     // free geometries, materials, textures
//   }
//
// ctx: scene (Group at this station, x from -16 to 16, z from -6 to 6, bench top at y = 0),
// camera { shots, track(keys) }, labels { show(id, html, anchor, opts), anchor(name, obj) },
// carrier { show(name, amount, content?) }, theme, plates { in, out }, beatAt(t), showAll(flag),
// reducedMotion, duration.
//
// This placeholder: a box slides in during beat 0; a digest token appears in beat 1. The carrier
// receives prog_d and input_d during beat 1, as the real scene does.
import * as THREE from 'three/webgpu';
import { materials, makeDigestToken, lerp, phase } from '../theme.js';

let ctx = null;
let box = null;
let token = null;

export default {
  id: 'program',
  title: 'The program',
  beats: [
    {
      id: 'source',
      title: 'Source becomes a program image',
      duration: 12,
      narration: 'Moriarty source compiles to the Sherlock machine language. The result is a program image: code tables held in ROM. The image is data, so every program is stored and presented to the circuits in the same way.',
      labels: [{ id: 'title', text: 'Program image', anchor: 'box', kind: 'program', dy: 2.2 }],
    },
    {
      id: 'digests',
      title: 'Two digests identify the run',
      duration: 14,
      narration: 'The Poseidon digest of the program image is prog_d. The input value is digested under sherlock-value/1 to input_d. These two tokens identify the run and stay on the carrier through every station.',
      labels: [{ id: 'id', text: '<code>prog_d</code>', anchor: 'token', kind: 'program', dy: 0.9 }],
    },
  ],

  build(c) {
    ctx = c;
    box = new THREE.Mesh(new THREE.BoxGeometry(6, 3, 3), materials.family('program'));
    box.position.y = 1.5;
    ctx.scene.add(box);
    token = makeDigestToken({ family: 'program', content: 'program', size: 2 });
    token.position.set(0, 4.5, 0);
    ctx.scene.add(token);
    ctx.labels.anchor('box', box);
    ctx.labels.anchor('token', token);
    const s = ctx.camera.shots;
    ctx.camera.track([
      { t: 0, pose: s.establish([0, 1.5, 0], 34), cut: true },
      { t: 12, pose: s.inspect([0, 3, 0], 12, 'front'), dur: 2 },
    ]);
  },

  seek(t) {
    const b = ctx.beatAt(t);
    box.position.x = b.index === 0 ? lerp(-10, 0, phase(b.t, 0.5, 2.5)) : 0;
    token.visible = b.index === 1;
    token.scale.setScalar(b.index === 1 ? 0.2 + 0.8 * phase(b.t, 0.5, 1.5) : 1);
    const a = b.index === 1 ? phase(b.t, 2, 3) : 0;
    ctx.carrier.show('prog_d', a, 'program');
    ctx.carrier.show('input_d', a, 'input');
  },

  dispose() {
    ctx = null;
    box = null;
    token = null;
  },
};

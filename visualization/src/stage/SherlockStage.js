import * as THREE from 'three/webgpu';
import { PALETTE, createStudioEnvironmentMap, lerp } from '../designers/theme.js';
import { createPackageSubsystem } from '../designers/designer1_package.js';
import { createWellSubsystem } from '../designers/designer2_well.js';
import { createFramesSubsystem } from '../designers/designer3_frames.js';
import { createArenaSubsystem } from '../designers/designer4_arena.js';
import { createCircuitSubsystem } from '../designers/designer5_circuit.js';
import { createLoomSubsystem } from '../designers/designer6_loom.js';
import { createCinemaSubsystem, CAMERA_PRESETS } from '../designers/designer7_cinema.js';
import { createQuinelingSubsystem } from '../designers/designer8_quineling.js';

export const DESIGNER_LAYERS = [
  {
    id: 'package',
    number: 'D1',
    name: 'Apple Monolithic Package',
    role: 'Warm Alabaster Porcelain Ceramic, Brushed Silver-Platinum Titanium & 24 Gold Leads',
    cameraPreset: 'overview',
  },
  {
    id: 'well',
    number: 'D2',
    name: '6-Ring Well, Control Stator & Rules',
    role: '6 Split-Arc Semantic Rings (12 Slots), Draining Fuel Ring, 10-Sector Control Stator & 39-Rule Floor',
    cameraPreset: 'well',
  },
  {
    id: 'frames',
    number: 'D3',
    name: 'Open-Bore Annular Continuation Rings',
    role: 'Open-Bore (r = 0.36..0.74) Machined Stainless & Sapphire Continuation Rings + Unwind Orb',
    cameraPreset: 'frames',
  },
  {
    id: 'arena',
    number: 'D4',
    name: 'West-Flank Call Chiplets',
    role: 'Companion Call Chiplets (fact(4)..fact(0)) on Studio Ground Plane',
    cameraPreset: 'arena',
  },
  {
    id: 'circuit',
    number: 'D5',
    name: 'Flush M-Series L4 & Aux Pads',
    role: 'Flush Lithography ROM, SRAM State Tracks & 16-Byte Aux/2 Matrix',
    cameraPreset: 'circuit',
  },
  {
    id: 'loom',
    number: 'D6',
    name: 'Minimalist IVC Proof Ribbon',
    role: 'Recessed 32-Segment ReferenceSlice Progress Track',
    cameraPreset: 'loom',
  },
  {
    id: 'cinema',
    number: 'D7',
    name: '3D Atrium Cinema & Reticle',
    role: 'Inside-Chip 3D Atrium Lens, Parabolic Swoop & VisionOS Reticle',
    cameraPreset: 'interior',
  },
  {
    id: 'quineling',
    number: 'D8',
    name: 'QDL Quineling Core Organism',
    role: '3D Procedural Quineling Species & Live CEK Morphogenesis inside the Hollow Processor Core',
    cameraPreset: 'interior',
  },
];

const SUBSYSTEM_KEYS = ['package', 'well', 'frames', 'arena', 'circuit', 'loom', 'quineling', 'cinema'];
const GATE_RULE_LINES = new Set([114, 123, 126, 161, 165, 173, 214, 220]);

export { CAMERA_PRESETS };

export class SherlockStage {
  constructor(canvas, { onInspect = null } = {}) {
    this.canvas = canvas;
    this.onInspect = onInspect;
    this.ambient = 0;
    this.lastTime = performance.now();
    this._disposed = false;
    this._cutawayMix = 0;

    // Pre-allocated scratch structures for zero-allocation per-frame rendering
    this._scratchColor = new THREE.Color();
    this._fullState = { ambient: 0, dt: 0.016, activePreset: 'overview', cutawayMix: 0 };

    this.scene = new THREE.Scene();
    this.scene.background = new THREE.Color(0xf4f0e8);
    this.scene.fog = new THREE.FogExp2(0xf4f0e8, 0.016);

    this.camera = new THREE.PerspectiveCamera(32, 1, 0.025, 100);
    this.camera.position.set(3.85, 4.85, 5.35);
    this.camera.lookAt(-0.38, -0.16, 0.05);

    this.isMobile =
      typeof navigator !== 'undefined' &&
      (/Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent || '') ||
        (typeof window !== 'undefined' &&
          typeof window.matchMedia === 'function' &&
          window.matchMedia('(max-width: 900px), (hover: none) and (pointer: coarse)').matches));

    this.renderer = new THREE.WebGPURenderer({
      canvas: this.canvas,
      antialias: !this.isMobile,
      powerPreference: 'high-performance',
    });
    const maxDpr = this.isMobile ? 1.25 : 2;
    const dpr = typeof window !== 'undefined' ? Math.min(window.devicePixelRatio || 1, maxDpr) : 1;
    this.renderer.setPixelRatio(dpr);
    this.renderer.outputColorSpace = THREE.SRGBColorSpace;
    this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
    this.renderer.toneMappingExposure = 1.06;
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFShadowMap;

    this.root = new THREE.Group();
    this.scene.add(this.root);

    this._setupLights();
    this._setupDesigners();
    this.resize();

    this.ready = false;
    this._initPromise = this._initRenderer();

    this._onResize = () => this.resize();
    if (typeof window !== 'undefined' && window.addEventListener) {
      window.addEventListener('resize', this._onResize);
    }
  }

  async _initRenderer() {
    try {
      await this.renderer.init();
      if (this._disposed) {
        this.renderer.dispose();
        return;
      }
      this.ready = true;
      this.resize();
    } catch (err) {
      this.ready = false;
    }
  }

  _setupLights() {
    // 0. Procedural Apple Studio Softbox Environment Map for 360° stainless & gold IBL reflections
    this.scene.environment = createStudioEnvironmentMap();
    this.scene.environmentIntensity = 0.68;

    // 1. Studio Hemisphere Ambient (balanced for IBL)
    const hemi = new THREE.HemisphereLight(0xfffbf2, 0xc9c2b4, 0.52);
    this.scene.add(hemi);

    // 2. Warm Daylight Key Light with tightened shadow frustum covering chip & West Chiplets
    const shadowRes = this.isMobile ? 1024 : 2048;
    const keyLight = new THREE.DirectionalLight(0xfff7ea, 2.05);
    keyLight.position.set(6.0, 10.5, 5.4);
    keyLight.castShadow = true;
    keyLight.shadow.mapSize.set(shadowRes, shadowRes);
    keyLight.shadow.camera.near = 4.5;
    keyLight.shadow.camera.far = 19.0;
    keyLight.shadow.camera.left = -3.6;
    keyLight.shadow.camera.right = 2.6;
    keyLight.shadow.camera.top = 2.8;
    keyLight.shadow.camera.bottom = -2.8;
    keyLight.shadow.bias = -0.00012;
    keyLight.shadow.normalBias = 0.004;
    keyLight.shadow.autoUpdate = false;
    keyLight.shadow.needsUpdate = true;
    this.keyLight = keyLight;
    this._lastShadowStepKey = '';
    this._shadowFramesRemaining = 2;
    this.scene.add(keyLight);

    // 3. Cool Sky Specular Fill Light for silky metallic reflections across machined stainless & gold rings
    const fillLight = new THREE.DirectionalLight(0xd5e3f8, 0.72);
    fillLight.position.set(-6.5, 5.8, -4.6);
    this.scene.add(fillLight);

    // 4. Warm Champagne Grazing Rim Light
    const rimLight = new THREE.DirectionalLight(0xffeed8, 0.62);
    rimLight.position.set(-2.8, 3.8, 6.2);
    this.scene.add(rimLight);

    // 5. Interior Hollow Chamber Architectural & Semantic Lights
    // Upward-casting vault light at y = -0.65 so the underside of the 6 split-arc rings,
    // 10-sector Control stator, and stainless ceiling soffit gleam when looking up from inside
    this.vaultUpperLight = new THREE.PointLight(0xf8fafc, 1.55, 5.2, 1.6);
    this.vaultUpperLight.position.set(0, -0.65, 0.28);
    this.root.add(this.vaultUpperLight);

    // Semantic mid-chamber glow illuminating the open-bore continuation rings
    this.wellPointLight = new THREE.PointLight(0x2dd4bf, 1.65, 6.0, 1.6);
    this.wellPointLight.position.set(0, -1.55, 0.10);
    this.root.add(this.wellPointLight);

    // Lower chamber floor bounce illuminating the deep -3.38 39-rule decoder floor & lower stainless ribs
    this.vaultFloorLight = new THREE.PointLight(0xe2e8f0, 1.45, 5.5, 1.6);
    this.vaultFloorLight.position.set(0, -2.85, 0.25);
    this.root.add(this.vaultFloorLight);

    this.coreSpecLight = new THREE.PointLight(0xffffff, 0.95, 3.2, 2.0);
    this.coreSpecLight.position.set(0, 0.65, 0.20);
    this.root.add(this.coreSpecLight);
  }

  _setupDesigners() {
    this.subsystems = {
      package: createPackageSubsystem(),
      well: createWellSubsystem(),
      frames: createFramesSubsystem(),
      arena: createArenaSubsystem(),
      circuit: createCircuitSubsystem(),
      loom: createLoomSubsystem(),
      quineling: createQuinelingSubsystem(),
      cinema: createCinemaSubsystem({
        camera: this.camera,
        domElement: this.canvas,
        scene: this.scene,
        onInspect: (meta) => {
          if (this.onInspect) this.onInspect(meta);
        },
      }),
    };

    for (let i = 0; i < SUBSYSTEM_KEYS.length; i++) {
      const key = SUBSYSTEM_KEYS[i];
      const sub = this.subsystems[key];
      if (sub && sub.group) {
        sub.group.name = `designer-${key}`;
        this.root.add(sub.group);
      }
    }

    this.layerVisibility = {
      package: true,
      well: true,
      frames: true,
      arena: true,
      circuit: true,
      loom: true,
      quineling: true,
      cinema: true,
    };
  }

  setLayerVisible(id, visible) {
    if (!(id in this.layerVisibility)) return;
    this.layerVisibility[id] = Boolean(visible);
    if (this.subsystems[id]?.group) {
      this.subsystems[id].group.visible = this.layerVisibility[id];
    }
    this._shadowFramesRemaining = Math.max(this._shadowFramesRemaining || 0, 2);
  }

  toggleLayer(id) {
    this.setLayerVisible(id, !this.layerVisibility[id]);
    return this.layerVisibility[id];
  }

  soloLayer(id) {
    const othersVisible = Object.entries(this.layerVisibility)
      .filter(([k]) => k !== id)
      .some(([, v]) => v);

    if (othersVisible) {
      for (const k of Object.keys(this.layerVisibility)) {
        // Keep package or well subtle context if soloing inner layers, or strictly solo:
        this.setLayerVisible(k, k === id || k === 'cinema');
      }
    } else {
      for (const k of Object.keys(this.layerVisibility)) {
        this.setLayerVisible(k, true);
      }
    }
    return { ...this.layerVisibility };
  }

  setCameraPreset(presetName) {
    if (this.subsystems.cinema?.setCameraPreset) {
      this.subsystems.cinema.setCameraPreset(presetName);
    }
  }

  getCameraPreset() {
    return this.subsystems.cinema?.getCameraPreset?.() || 'overview';
  }

  resize() {
    const parent = this.canvas?.parentElement || this.canvas;
    const fallbackW = typeof window !== 'undefined' ? window.innerWidth : 1280;
    const fallbackH = typeof window !== 'undefined' ? window.innerHeight : 800;
    const width = Math.max(320, parent?.clientWidth || fallbackW || 1280);
    const height = Math.max(320, parent?.clientHeight || fallbackH || 800);
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
    this.renderer.setSize(width, height, false);
    this._shadowFramesRemaining = Math.max(this._shadowFramesRemaining || 0, 2);
  }

  render(stepState) {
    if (this._disposed) return;
    const now = performance.now();
    const rawDt =
      stepState && Number.isFinite(stepState.dt) && stepState.dt > 0
        ? stepState.dt
        : (now - this.lastTime) / 1000;
    const dt = Math.min(0.05, Math.max(0.001, rawDt));
    this.lastTime = now;
    this.ambient += dt;

    const activePreset = this.getCameraPreset();
    const targetCutaway = activePreset === 'frames' ? 1.0 : 0.0;
    const prevCutaway = this._cutawayMix;
    this._cutawayMix = lerp(this._cutawayMix, targetCutaway, 1.0 - Math.exp(-12.0 * dt));
    if (Math.abs(this._cutawayMix - targetCutaway) < 0.005) {
      this._cutawayMix = targetCutaway;
    }

    const fullState = this._fullState;
    if (stepState && typeof stepState === 'object') {
      fullState.semanticTransition = stepState.semanticTransition ?? null;
      fullState.ruleLine = stepState.ruleLine ?? null;
      Object.assign(fullState, stepState);
    }
    fullState.ambient = this.ambient;
    fullState.dt = dt;
    fullState.activePreset = activePreset;
    fullState.cutawayMix = this._cutawayMix;
    fullState.playing = Boolean(stepState?.playing);

    // Dynamic point light color based on active semantic transition & ruleLine (zero heap allocation via _scratchColor)
    const phase = fullState.semanticTransition || fullState.phase || 'DESCEND';
    const ruleLine =
      fullState.ruleLine ??
      fullState.step?.rule?.line ??
      fullState.step?.ruleLine ??
      fullState.step?.after?.ruleLine ??
      0;

    if (phase === 'ERROR') {
      this._scratchColor.setHex(0xe11d48);
      this.wellPointLight.intensity = 1.65;
    } else if (phase === 'GATE' || GATE_RULE_LINES.has(Number(ruleLine))) {
      this._scratchColor.setHex(0x818cf8);
      this.wellPointLight.intensity = 1.50;
    } else if (
      phase === 'ENV' ||
      phase === 'RETURN' ||
      phase === 'RESOLVE' ||
      phase === 'EMBLEM'
    ) {
      this._scratchColor.setHex(0x2dd4bf);
      this.wellPointLight.intensity = 1.50;
    } else {
      this._scratchColor.setHex(0xfbbf24);
      this.wellPointLight.intensity = 1.45;
    }
    this.wellPointLight.color.copy(this._scratchColor);

    // Boost architectural interior vault lights when viewing inside the hollow chip or cross-section
    if (this.vaultUpperLight && this.vaultFloorLight) {
      const inHollow =
        activePreset === 'interior' ||
        activePreset === 'interior_up' ||
        activePreset === 'interior_floor' ||
        activePreset === 'frames';
      const vaultBoost = inHollow ? 1.95 : 1.0;
      this.vaultUpperLight.intensity = 1.25 * vaultBoost;
      this.vaultFloorLight.intensity = 1.55 * vaultBoost;
    }

    for (let i = 0; i < SUBSYSTEM_KEYS.length; i++) {
      const key = SUBSYSTEM_KEYS[i];
      if (this.layerVisibility[key] === false && key !== 'cinema') continue;
      const sub = this.subsystems[key];
      if (sub && typeof sub.update === 'function') {
        sub.update(fullState);
      }
    }

    if (this.ready === true) {
      const callLen = fullState.recursion?.calls?.length ?? 0;
      const frameLen = fullState.step?.after?.frames?.length ?? 0;
      const shadowKey = this.isMobile
        ? `${fullState.presetId || ''}|${Math.round(this._cutawayMix * 10)}`
        : `${fullState.presetId || ''}|${callLen}|${frameLen}|${Math.round(this._cutawayMix * 25)}`;
      if (shadowKey !== this._lastShadowStepKey || Math.abs(this._cutawayMix - prevCutaway) > 0.002) {
        this._lastShadowStepKey = shadowKey;
        this._shadowFramesRemaining = Math.max(this._shadowFramesRemaining || 0, 1);
      }
      if (this.keyLight?.shadow) {
        this.keyLight.shadow.autoUpdate = false;
        this.renderer.shadowMap.autoUpdate = false;
        if (this._shadowFramesRemaining > 0) {
          this.keyLight.shadow.needsUpdate = true;
          this._shadowFramesRemaining -= 1;
        } else {
          this.keyLight.shadow.needsUpdate = false;
        }
      }
      this.renderer.render(this.scene, this.camera);
    }
  }

  dispose() {
    this._disposed = true;
    if (typeof window !== 'undefined' && window.removeEventListener) {
      window.removeEventListener('resize', this._onResize);
    }
    for (let i = 0; i < SUBSYSTEM_KEYS.length; i++) {
      const sub = this.subsystems[SUBSYSTEM_KEYS[i]];
      if (sub && typeof sub.dispose === 'function') sub.dispose();
    }
    if (this.keyLight?.shadow?.map) {
      this.keyLight.shadow.map.dispose();
    }
    if (this.keyLight && typeof this.keyLight.dispose === 'function') {
      this.keyLight.dispose();
    }
    if (this.ready && this.renderer) {
      this.renderer.dispose();
    }
  }
}

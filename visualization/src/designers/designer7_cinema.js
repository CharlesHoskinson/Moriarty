import * as THREE from 'three/webgpu';
import {
  PALETTE,
  CONTROL_KIND_META,
  clamp,
  lerp,
} from './theme.js';

/**
 * Studio Camera Lens Presets for the Sherlock Universal Interpreter Workbench.
 * Includes exterior horological views, oblique wafer cross-sections, and
 * direct interior 3D Atrium immersion inside the hollow chip chamber.
 */
export const CAMERA_PRESETS = {
  overview: {
    name: 'overview',
    label: 'Overview',
    subtitle: '3/4 diagonal studio perspective of the 3-tier Sherlock chip, companion Call Chiplets, and Split-Arc Rings',
    pos: [3.85, 4.85, 5.35],
    lookAt: [-0.38, -0.16, 0.05],
    fov: 32,
  },
  interior: {
    name: 'interior',
    label: 'Inside Hollow Chip',
    subtitle: 'Miniature first-person observer inside the sealed 3D vault — framing the living 3D Quineling Core, 6 Split-Arc Rings overhead, and 39 3D Floor Modules below',
    pos: [0.0, -1.85, 1.12],
    lookAt: [0.0, -2.18, -0.28],
    fov: 72,
  },
  interior_up: {
    name: 'interior_up',
    label: 'Inside Chip · Look Up at Rings',
    subtitle: 'Looking straight up through the open bore at all 6 Split-Arc Rings and 10-Sector Control Stator',
    pos: [0.0, -1.32, 0.18],
    lookAt: [0.0, -0.10, -0.02],
    fov: 70,
  },
  interior_floor: {
    name: 'interior_floor',
    label: 'Inside Chip · 39 Floor Modules',
    subtitle: 'Hovering above the 39 physical 3D silicon rule chiplets on the chamber floor (A080..A225)',
    pos: [0.0, -2.15, 0.88],
    lookAt: [0.0, -3.35, -0.18],
    fov: 70,
  },
  well: {
    name: 'well',
    label: 'Top-Down Rings',
    subtitle: 'Axial horological view of all 12 Split-Arc Half-Rings (R1.A..R6.B), 10-Sector Control Stator, and open bore',
    pos: [0.0, 3.65, 0.01],
    lookAt: [0.0, -0.12, 0.0],
    fov: 28,
  },
  frames: {
    name: 'frames',
    label: 'Cross-Section',
    subtitle: '3/4 architectural cutaway section through the machined stainless collar, 6 split-arc rings, 3D Quineling Core, and 39 3D floor modules',
    pos: [1.85, -0.75, 5.15],
    lookAt: [-0.05, -1.95, 0.0],
    fov: 42,
  },
  arena: {
    name: 'arena',
    label: 'Active Calls & Value Stage',
    subtitle: 'Studio view of the West-Flank Companion Call Chiplets and Horological Well',
    pos: [-1.85, 3.25, 4.15],
    lookAt: [-0.95, -0.22, 0.0],
    fov: 32,
  },
  circuit: {
    name: 'circuit',
    label: 'L4 Die Circuit',
    subtitle: 'High plan view of the die shield, perimeter micro-LED grooves, and 6 concentric rings',
    pos: [0.0, 5.15, 0.70],
    lookAt: [0, 0.02, 0],
    fov: 32,
  },
  aux2: {
    name: 'aux2',
    label: 'Aux/2 ALU',
    subtitle: 'Macro inspection of the East flank 16-byte UInt128 ALU & FuelWitness array',
    pos: [2.55, 2.15, 2.35],
    lookAt: [1.62, 0.05, 0.22],
    fov: 32,
  },
  loom: {
    name: 'loom',
    label: 'IVC Proof Ribbon',
    subtitle: 'Studio view of the North ReferenceSlice execution progress track',
    pos: [-1.60, 2.35, 3.10],
    lookAt: [0, 0.08, -1.25],
    fov: 32,
  },
  cinema: {
    name: 'cinema',
    label: 'Cinema Orbit',
    subtitle: 'Gentle, slow orbital rotation around the Sherlock chip',
    pos: [3.65, 4.45, 4.95],
    lookAt: [-0.25, -0.16, 0.0],
    fov: 32,
  },
};

function isInteriorPresetName(name) {
  return name === 'interior' || name === 'interior_up' || name === 'interior_floor';
}

// Pre-allocated module-scope scratch math objects (zero per-frame or per-event allocations)
const _scratchOffset = new THREE.Vector3();
const _scratchSpherical = new THREE.Spherical();
const _panRight = new THREE.Vector3();
const _panUp = new THREE.Vector3();
const _panFwd = new THREE.Vector3();
const _camOffset = new THREE.Vector3();
const _interiorLookDir = new THREE.Vector3();
const _interiorLookTarget = new THREE.Vector3();
const _interiorHorizFwd = new THREE.Vector3();
const _interiorHorizRight = new THREE.Vector3();

function resolvePhaseMode(phase, controlKind) {
  const p = phase || CONTROL_KIND_META[controlKind]?.phase || 'STILL';
  if (p === 'INGEST' || p === 'DESCEND') return 1;
  if (p === 'RESOLVE' || p === 'RETURN') return 2;
  return 0;
}

// ---------------------------------------------------------------------------
// Minimalist 8-Corner Hairline Reticle (Zero Per-Frame Allocations)
// ---------------------------------------------------------------------------

function createInspectorReticle() {
  const reticleGroup = new THREE.Group();
  reticleGroup.name = 'CinemaInspectorReticle';
  reticleGroup.visible = false;
  reticleGroup.userData.isInspectorReticle = true;

  // 8 corners * 3 orthogonal hairline segments * 2 vertices = 48 vertices
  const cornerPositions = new Float32Array(48 * 3);
  const cornerGeo = new THREE.BufferGeometry();
  cornerGeo.setAttribute('position', new THREE.BufferAttribute(cornerPositions, 3));

  const cornerMat = new THREE.LineBasicMaterial({
    color: PALETTE.goldBright,
    transparent: true,
    opacity: 0.88,
    depthTest: false,
  });
  const cornerLines = new THREE.LineSegments(cornerGeo, cornerMat);
  cornerLines.renderOrder = 999;
  cornerLines.userData.isInspectorReticle = true;
  cornerLines.raycast = () => {};
  reticleGroup.add(cornerLines);

  const cornerSigns = [
    [1, 1, 1],
    [-1, 1, 1],
    [1, -1, 1],
    [-1, -1, 1],
    [1, 1, -1],
    [-1, 1, -1],
    [1, -1, -1],
    [-1, -1, -1],
  ];

  function updateGeometryFromBox(min, max, ambient = 0, pinned = false) {
    const dx = Math.max(0.035, max.x - min.x);
    const dy = Math.max(0.025, max.y - min.y);
    const dz = Math.max(0.035, max.z - min.z);

    const pulse = (pinned ? 0.0025 : 0.005) * Math.sin(ambient * 4.0);
    const x0 = min.x - pulse;
    const y0 = min.y - pulse;
    const z0 = min.z - pulse;
    const x1 = max.x + pulse;
    const y1 = max.y + pulse;
    const z1 = max.z + pulse;

    const bx = clamp(dx * 0.22, 0.018, 0.11);
    const by = clamp(dy * 0.22, 0.014, 0.09);
    const bz = clamp(dz * 0.22, 0.018, 0.11);

    let idx = 0;
    for (let i = 0; i < 8; i++) {
      const [sx, sy, sz] = cornerSigns[i];
      const cx = sx > 0 ? x0 : x1;
      const cy = sy > 0 ? y0 : y1;
      const cz = sz > 0 ? z0 : z1;

      // X hairline
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;
      cornerPositions[idx++] = cx + sx * bx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;

      // Y hairline
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy + sy * by;
      cornerPositions[idx++] = cz;

      // Z hairline
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz;
      cornerPositions[idx++] = cx;
      cornerPositions[idx++] = cy;
      cornerPositions[idx++] = cz + sz * bz;
    }

    cornerGeo.attributes.position.needsUpdate = true;
    cornerMat.color.setHex(pinned ? PALETTE.heatBright : PALETTE.goldBright);
    cornerMat.opacity = pinned ? 0.96 : 0.84;
  }

  function dispose() {
    cornerGeo.dispose();
    cornerMat.dispose();
  }

  return {
    group: reticleGroup,
    updateGeometryFromBox,
    dispose,
  };
}

// ---------------------------------------------------------------------------
// Designer 7 Factory: Pure WebGPU Camera Choreography, Throttled Raycaster & Reticle
// ---------------------------------------------------------------------------

export function createCinemaSubsystem({
  camera,
  domElement,
  scene,
  onInspect = null,
} = {}) {
  const group = new THREE.Group();
  group.name = 'Designer7_Cinema';

  let inspectCallback = typeof onInspect === 'function' ? onInspect : null;

  // Minimalist 8-corner hairline reticle
  const reticle = createInspectorReticle();
  group.add(reticle.group);

  // -------------------------------------------------------------------------
  // 1. Smooth Damped Camera Choreography & First-Person Interior Micro-Gimbal
  // -------------------------------------------------------------------------
  const MIN_ORBIT_DISTANCE = 0.45;
  const MAX_ORBIT_DISTANCE = 13.5;
  const MIN_PHI = 0.002;

  // Interior First-Person / Micro-Gimbal bounds (strictly inside the open central atrium `r <= 0.94`)
  const INTERIOR_MAX_RADIAL = 0.94;
  const INTERIOR_MIN_Y = -2.96;
  const INTERIOR_MAX_Y = -0.52;
  const INTERIOR_MIN_PITCH = -Math.PI * 0.465; // -83.7° (look straight down at 3D floor chiplets)
  const INTERIOR_MAX_PITCH = Math.PI * 0.472;  // +85.0° (look straight up through open bore at 6 rings)

  let currentPresetName = 'overview';
  let desiredFov = CAMERA_PRESETS.overview.fov || 32;
  let transitionArcBoost = 0.0;

  const currentTarget = new THREE.Vector3(...CAMERA_PRESETS.overview.lookAt);
  const desiredTarget = new THREE.Vector3(...CAMERA_PRESETS.overview.lookAt);

  _scratchOffset.set(...CAMERA_PRESETS.overview.pos).sub(currentTarget);
  _scratchSpherical.setFromVector3(_scratchOffset);

  let currentRadius = _scratchSpherical.radius;
  let currentPhi = _scratchSpherical.phi;
  let currentTheta = _scratchSpherical.theta;

  let desiredRadius = _scratchSpherical.radius;
  let desiredPhi = _scratchSpherical.phi;
  let desiredTheta = _scratchSpherical.theta;

  // First-person micro-gimbal state for `interior` mode (`∂eye/∂yaw = ∂eye/∂pitch = 0`)
  const interiorEye = new THREE.Vector3(...CAMERA_PRESETS.interior.pos);
  const desiredInteriorEye = new THREE.Vector3(...CAMERA_PRESETS.interior.pos);
  let interiorYaw = 0.0;
  let desiredInteriorYaw = 0.0;
  let interiorPitch = 0.08;
  let desiredInteriorPitch = 0.08;

  function clampInteriorEye(vec) {
    const r = Math.hypot(vec.x, vec.z);
    if (r > INTERIOR_MAX_RADIAL) {
      const s = INTERIOR_MAX_RADIAL / r;
      vec.x *= s;
      vec.z *= s;
    }
    vec.y = clamp(vec.y, INTERIOR_MIN_Y, INTERIOR_MAX_Y);
  }

  function seedInteriorFromPreset(preset) {
    desiredInteriorEye.set(preset.pos[0], preset.pos[1], preset.pos[2]);
    clampInteriorEye(desiredInteriorEye);

    _scratchOffset
      .set(preset.lookAt[0], preset.lookAt[1], preset.lookAt[2])
      .sub(desiredInteriorEye)
      .normalize();

    const seedPitch = clamp(
      Math.asin(clamp(_scratchOffset.y, -0.995, 0.995)),
      INTERIOR_MIN_PITCH,
      INTERIOR_MAX_PITCH
    );
    const seedYaw = Math.atan2(_scratchOffset.x, -_scratchOffset.z);

    // Unwrap yaw relative to current interiorYaw for smooth shortest-turn interpolation
    let deltaYaw = seedYaw - interiorYaw;
    while (deltaYaw > Math.PI) deltaYaw -= Math.PI * 2;
    while (deltaYaw < -Math.PI) deltaYaw += Math.PI * 2;
    desiredInteriorYaw = interiorYaw + deltaYaw;
    desiredInteriorPitch = seedPitch;
  }

  seedInteriorFromPreset(CAMERA_PRESETS.interior);
  interiorEye.copy(desiredInteriorEye);
  interiorYaw = desiredInteriorYaw;
  interiorPitch = desiredInteriorPitch;

  if (camera) {
    camera.fov = desiredFov;
    if (camera.near > 0.03) {
      camera.near = 0.03;
    }
    if (typeof camera.updateProjectionMatrix === 'function') {
      camera.updateProjectionMatrix();
    }
    camera.position.set(...CAMERA_PRESETS.overview.pos);
    camera.lookAt(currentTarget);
  }

  function getMaxPhiForState(radius, target) {
    if (currentPresetName === 'frames' || target.y < 0.0) {
      return Math.PI * 0.96;
    }
    const inCentralWell = Math.hypot(target.x, target.z) < 0.96 && target.y < 0.05;
    if (inCentralWell && radius < 3.2) {
      return Math.PI * 0.96;
    }
    return Math.PI * 0.48;
  }

  function setCameraPreset(presetName) {
    const key = CAMERA_PRESETS[presetName] ? presetName : 'overview';
    const prevPresetName = currentPresetName;
    const wasInterior = isInteriorPresetName(prevPresetName);
    const nowInterior = isInteriorPresetName(key);
    currentPresetName = key;
    const preset = CAMERA_PRESETS[key];
    desiredFov = preset.fov || 32;

    if (nowInterior) {
      // If entering interior mode from an exterior view, start the smooth dive from a clean upper entry point
      // so the camera glides down through the open bore without clipping the walls
      if (!wasInterior && camera) {
        interiorEye.set(preset.pos[0] * 0.5, -0.65, preset.pos[2] * 0.5);
        clampInteriorEye(interiorEye);
      }
      seedInteriorFromPreset(preset);
      transitionArcBoost = 0.0;
      if (camera) {
        camera.near = 0.008;
        if (typeof camera.updateProjectionMatrix === 'function') {
          camera.updateProjectionMatrix();
        }
      }
      return preset;
    }

    if (wasInterior && camera) {
      camera.near = 0.025;
      if (typeof camera.updateProjectionMatrix === 'function') {
        camera.updateProjectionMatrix();
      }
    }

    desiredTarget.set(preset.lookAt[0], preset.lookAt[1], preset.lookAt[2]);
    _scratchOffset
      .set(preset.pos[0], preset.pos[1], preset.pos[2])
      .sub(desiredTarget);
    _scratchSpherical.setFromVector3(_scratchOffset);

    desiredRadius = clamp(_scratchSpherical.radius, MIN_ORBIT_DISTANCE, MAX_ORBIT_DISTANCE);
    const maxPhi = getMaxPhiForState(desiredRadius, desiredTarget);
    desiredPhi = clamp(_scratchSpherical.phi, MIN_PHI, maxPhi);

    // Unwrap desiredTheta so shortest angular path is taken relative to currentTheta
    let deltaTheta = _scratchSpherical.theta - currentTheta;
    while (deltaTheta > Math.PI) deltaTheta -= Math.PI * 2;
    while (deltaTheta < -Math.PI) deltaTheta += Math.PI * 2;
    desiredTheta = currentTheta + deltaTheta;

    // Trigger a smooth parabolic arc-lift when transitioning between exterior presets
    if (prevPresetName !== key) {
      transitionArcBoost = 1.0;
    }

    return preset;
  }

  function getCameraPreset() {
    return currentPresetName;
  }

  // -------------------------------------------------------------------------
  // 2. High-Performance Throttled Raycaster (<= 16 Hz / 60ms)
  //    Supports per-instance inspection of the 39 3D Floor Rule Chiplets!
  // -------------------------------------------------------------------------
  const raycaster = new THREE.Raycaster();
  raycaster.params.Line = { threshold: 0.015 };
  const pointerNDC = new THREE.Vector2();
  let pointerInside = false;

  let isDragging = false;
  let isPanning = false;
  let dragMovedDistance = 0;
  let lastPointerX = 0;
  let lastPointerY = 0;

  let lastRaycastTime = 0;
  let lastRaycastClientX = -9999;
  let lastRaycastClientY = -9999;
  const RAYCAST_THROTTLE_MS = 60;

  let hoveredObject = null;
  let pinnedObject = null;

  // Smoothed world-space bounding box for the inspector reticle
  const rawBox = new THREE.Box3();
  const smoothMin = new THREE.Vector3();
  const smoothMax = new THREE.Vector3();
  let reticleInitialized = false;

  // Lightweight proxy object for per-instance raycast hits on the 39 3D Floor Rule Chiplets
  const floorRuleProxyCache = new Map();
  function getFloorRuleProxy(instancedMesh, cellRecord) {
    let proxy = floorRuleProxyCache.get(cellRecord.index);
    if (!proxy) {
      const rule = cellRecord.rule;
      proxy = {
        name: `FloorRule_${rule.id}`,
        visible: true,
        parent: instancedMesh.parent,
        customBox: new THREE.Box3(
          new THREE.Vector3(cellRecord.x - 0.20, -3.38, cellRecord.z - 0.17),
          new THREE.Vector3(cellRecord.x + 0.20, -3.28, cellRecord.z + 0.17)
        ),
        userData: {
          inspectable: true,
          type: 'k-decoder-rule-chiplet',
          id: `floor-rule-${rule.id}`,
          title: `${rule.id} · L${rule.line} ${rule.shortName}`,
          subtitle: `${rule.fullName} · ${rule.categoryLabel}`,
          details: [
            rule.description,
            `Fuel Charge: ${rule.charge}F · Matrix Slot [${cellRecord.row + 1}, ${cellRecord.col + 1}]`,
          ],
        },
      };
      floorRuleProxyCache.set(cellRecord.index, proxy);
    } else {
      proxy.parent = instancedMesh.parent;
    }
    return proxy;
  }

  function findInspectableTarget(intersections) {
    for (let i = 0; i < intersections.length; i++) {
      const hit = intersections[i];
      const hitObj = hit.object;
      if (!hitObj || hitObj.userData?.isInspectorReticle) continue;

      let visCheck = hitObj;
      let isVisible = true;
      while (visCheck) {
        if (visCheck.visible === false) {
          isVisible = false;
          break;
        }
        visCheck = visCheck.parent;
      }
      if (!isVisible) continue;

      // Check for per-instance 39-rule 3D floor chiplet hit first!
      const ruleRecords = hitObj.userData?.ruleRecords;
      if (Array.isArray(ruleRecords) && hit.instanceId !== undefined && ruleRecords[hit.instanceId]) {
        return getFloorRuleProxy(hitObj, ruleRecords[hit.instanceId]);
      }

      let cur = hitObj;
      while (cur) {
        if (cur.userData?.inspectable) {
          return cur;
        }
        cur = cur.parent;
      }
      if (hitObj.userData?.inspectableParent) {
        return hitObj.userData.inspectableParent;
      }
    }
    return null;
  }

  function extractInspectMeta(obj, pinned = false) {
    if (!obj || !obj.userData) return null;
    const ud = obj.userData;
    return {
      type: ud.type || 'component',
      id: ud.id || obj.name || 'subsystem',
      title: ud.title || obj.name || 'Sherlock Subsystem',
      subtitle: ud.subtitle || '',
      details: ud.details || {},
      pinned,
    };
  }

  function emitInspect(obj, pinned = false) {
    if (!inspectCallback) return;
    if (!obj) {
      inspectCallback(null);
      return;
    }
    inspectCallback(extractInspectMeta(obj, pinned));
  }

  function performRaycast() {
    if (!camera || !scene || !pointerInside || isDragging) return null;
    raycaster.setFromCamera(pointerNDC, camera);
    const hits = raycaster.intersectObjects(scene.children, true);
    const target = findInspectableTarget(hits);

    if (target !== hoveredObject) {
      hoveredObject = target;
      if (!pinnedObject) {
        emitInspect(hoveredObject, false);
      }
    }
    if (domElement) {
      domElement.style.cursor = isDragging
        ? 'grabbing'
        : hoveredObject
          ? 'pointer'
          : 'grab';
    }
    return target;
  }

  function updatePointerNDC(event) {
    if (!domElement) return;
    const rect = domElement.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) return;
    pointerNDC.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
    pointerNDC.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
  }

  function onPointerDown(event) {
    if (!domElement) return;
    isDragging = true;
    isPanning = event.button === 2 || event.button === 1 || event.shiftKey;
    dragMovedDistance = 0;
    lastPointerX = event.clientX;
    lastPointerY = event.clientY;
    pointerInside = true;
    updatePointerNDC(event);
    if (domElement.setPointerCapture && event.pointerId !== undefined) {
      try {
        domElement.setPointerCapture(event.pointerId);
      } catch (_) {
        // Ignore pointer capture errors on synthetic events
      }
    }
    domElement.style.cursor = 'grabbing';
  }

  function onPointerMove(event) {
    pointerInside = true;

    if (isDragging) {
      const dx = event.clientX - lastPointerX;
      const dy = event.clientY - lastPointerY;
      lastPointerX = event.clientX;
      lastPointerY = event.clientY;
      dragMovedDistance += Math.hypot(dx, dy);

      // First-Person / Micro-Gimbal Mode when inside the hollow chip:
      // Normal drag rotates Yaw (360°) and Pitch (-83°..+85°) IN PLACE with ZERO camera translation,
      // so rotating NEVER leaves the chip! Shift-drag / right-drag strafes inside the bounded atrium.
      if (isInteriorPresetName(currentPresetName)) {
        if (isPanning) {
          const moveScale = 0.0022;
          _interiorHorizFwd.set(Math.sin(interiorYaw), 0, -Math.cos(interiorYaw));
          _interiorHorizRight.set(Math.cos(interiorYaw), 0, Math.sin(interiorYaw));
          desiredInteriorEye.addScaledVector(_interiorHorizRight, -dx * moveScale);
          desiredInteriorEye.addScaledVector(_interiorHorizFwd, dy * moveScale);
          clampInteriorEye(desiredInteriorEye);
        } else {
          desiredInteriorYaw += dx * 0.0052;
          desiredInteriorPitch = clamp(
            desiredInteriorPitch - dy * 0.0048,
            INTERIOR_MIN_PITCH,
            INTERIOR_MAX_PITCH
          );
        }
        return;
      }

      if (isPanning && camera) {
        const panScale = Math.max(0.9, desiredRadius) * 0.0014;
        camera.matrixWorld.extractBasis(_panRight, _panUp, _panFwd);
        desiredTarget.addScaledVector(_panRight, -dx * panScale);
        desiredTarget.addScaledVector(_panUp, dy * panScale);
        desiredTarget.y = clamp(desiredTarget.y, -3.35, 1.6);
      } else {
        const maxPhi = getMaxPhiForState(desiredRadius, desiredTarget);
        desiredTheta -= dx * 0.0068;
        desiredPhi = clamp(desiredPhi - dy * 0.0058, MIN_PHI, maxPhi);
      }
      return;
    }

    // Only raycast when the pointer has actually moved and at most once every 60ms (16 Hz)
    if (
      event.clientX === lastRaycastClientX &&
      event.clientY === lastRaycastClientY
    ) {
      return;
    }

    const now = performance.now();
    if (now - lastRaycastTime < RAYCAST_THROTTLE_MS) {
      return;
    }

    lastRaycastTime = now;
    lastRaycastClientX = event.clientX;
    lastRaycastClientY = event.clientY;
    updatePointerNDC(event);
    performRaycast();
  }

  function onPointerUp(event) {
    if (domElement && domElement.releasePointerCapture && event.pointerId !== undefined) {
      try {
        domElement.releasePointerCapture(event.pointerId);
      } catch (_) {
        // Ignore release errors
      }
    }
    const wasClick = isDragging && dragMovedDistance < 5.0 && event.button === 0;
    isDragging = false;
    isPanning = false;

    if (wasClick) {
      updatePointerNDC(event);
      lastRaycastTime = performance.now();
      lastRaycastClientX = event.clientX;
      lastRaycastClientY = event.clientY;
      const hitTarget = performRaycast();
      if (hitTarget) {
        if (pinnedObject === hitTarget) {
          pinnedObject = null;
          emitInspect(hoveredObject, false);
        } else {
          pinnedObject = hitTarget;
          emitInspect(pinnedObject, true);
        }
      } else if (pinnedObject) {
        pinnedObject = null;
        emitInspect(null, false);
      }
    } else if (domElement) {
      domElement.style.cursor = hoveredObject ? 'pointer' : 'grab';
    }
  }

  function onPointerLeave() {
    pointerInside = false;
    if (!isDragging) {
      hoveredObject = null;
      if (!pinnedObject) {
        emitInspect(null, false);
      }
      if (domElement) {
        domElement.style.cursor = 'grab';
      }
    }
  }

  function onWheel(event) {
    event.preventDefault();

    // Inside the hollow chip: scroll wheel operates the vertical elevator (`y ∈ [-2.96, -0.52]`),
    // gliding smoothly between the 39 3D floor modules at the bottom and the 6 split-arc rings at the top
    // without ever pushing the camera through the chamber walls!
    if (isInteriorPresetName(currentPresetName)) {
      desiredInteriorEye.y = clamp(
        desiredInteriorEye.y - event.deltaY * 0.0022,
        INTERIOR_MIN_Y,
        INTERIOR_MAX_Y
      );
      clampInteriorEye(desiredInteriorEye);
      return;
    }

    const zoomFactor = Math.exp(event.deltaY * 0.0011);
    const prevRadius = desiredRadius;
    desiredRadius = clamp(desiredRadius * zoomFactor, MIN_ORBIT_DISTANCE, MAX_ORBIT_DISTANCE);

    // When zooming inside the central aperture (desiredRadius < 2.1), gently guide lookAt
    // down into the hollow 3D chamber so the camera dives smoothly inside the chip
    if (Math.hypot(desiredTarget.x, desiredTarget.z) < 0.65) {
      if (desiredRadius < 2.1 && event.deltaY < 0) {
        const diveT = clamp((2.1 - desiredRadius) / (2.1 - MIN_ORBIT_DISTANCE), 0, 1);
        const chamberY = lerp(-0.18, -2.15, diveT);
        desiredTarget.y = Math.min(desiredTarget.y, chamberY);
      } else if (prevRadius < 2.1 && event.deltaY > 0 && currentPresetName === 'overview') {
        const surfaceT = clamp((desiredRadius - MIN_ORBIT_DISTANCE) / (3.2 - MIN_ORBIT_DISTANCE), 0, 1);
        desiredTarget.y = lerp(desiredTarget.y, -0.12, surfaceT * 0.35);
      }
    }
  }

  function onContextMenu(event) {
    event.preventDefault();
  }

  if (domElement && domElement.addEventListener) {
    domElement.style.cursor = 'grab';
    domElement.addEventListener('pointerdown', onPointerDown);
    domElement.addEventListener('pointermove', onPointerMove);
    domElement.addEventListener('pointerup', onPointerUp);
    domElement.addEventListener('pointerleave', onPointerLeave);
    domElement.addEventListener('wheel', onWheel, { passive: false });
    domElement.addEventListener('contextmenu', onContextMenu);
  }

  function isHierarchyVisible(obj) {
    let cur = obj;
    while (cur) {
      if (cur.visible === false) return false;
      cur = cur.parent;
    }
    return Boolean(obj && obj.parent);
  }

  // -------------------------------------------------------------------------
  // 3. Per-Frame Update Loop: Damped Camera Choreography & Reticle Tracking
  // -------------------------------------------------------------------------
  let lastStepIndex = -1;
  let lastBoxTarget = null;
  let lastBoxStepIndex = -1;

  function update(stepState = {}) {
    const ambient = stepState.ambient ?? 0;
    const dt = clamp(stepState.dt ?? 0.016, 0.001, 0.1);
    const progress = clamp(stepState.progress ?? 0, 0, 1);
    const step = stepState.step || {};
    const phaseMode = resolvePhaseMode(stepState.phase, step.controlKind);

    if (camera) {
      const damp = 1.0 - Math.exp(-12.0 * dt);

      if (typeof camera.fov === 'number' && Math.abs(camera.fov - desiredFov) > 0.05) {
        camera.fov = lerp(camera.fov, desiredFov, damp);
        if (typeof camera.updateProjectionMatrix === 'function') {
          camera.updateProjectionMatrix();
        }
      }

      if (isInteriorPresetName(currentPresetName)) {
        // First-Person / Micro-Gimbal Interior Camera Update:
        // Strictly clamps `interiorEye` inside the sealed vault and rotates the look vector in place!
        clampInteriorEye(desiredInteriorEye);
        interiorEye.lerp(desiredInteriorEye, damp);
        clampInteriorEye(interiorEye);

        interiorYaw = lerp(interiorYaw, desiredInteriorYaw, damp);
        interiorPitch = lerp(interiorPitch, desiredInteriorPitch, damp);

        const cosPitch = Math.cos(interiorPitch);
        _interiorLookDir.set(
          Math.sin(interiorYaw) * cosPitch,
          Math.sin(interiorPitch),
          -Math.cos(interiorYaw) * cosPitch
        );

        camera.position.copy(interiorEye);
        _interiorLookTarget.copy(interiorEye).add(_interiorLookDir);
        camera.lookAt(_interiorLookTarget);

        // Keep exterior spherical state synced to current camera so switching back to exterior is seamless
        currentTarget.copy(_interiorLookTarget);
        desiredTarget.copy(_interiorLookTarget);
      } else {
        if (currentPresetName === 'cinema' && !isDragging) {
          desiredTheta += dt * 0.18;
          desiredPhi = 0.82 + Math.sin(ambient * 0.24) * 0.06;
          desiredRadius = 7.45 + Math.cos(ambient * 0.20 + progress * Math.PI) * 0.38;

          const phaseTargetY =
            phaseMode === 1 ? -0.26 : phaseMode === 2 ? -0.10 : -0.16;
          desiredTarget.y = lerp(desiredTarget.y, phaseTargetY, dt * 1.6);
        }

        currentRadius = lerp(currentRadius, desiredRadius, damp);
        currentPhi = lerp(currentPhi, desiredPhi, damp);
        currentTheta = lerp(currentTheta, desiredTheta, damp);
        currentTarget.lerp(desiredTarget, damp);

        transitionArcBoost = Math.max(0, transitionArcBoost - dt * 2.4);
        const arcLiftY = Math.sin(transitionArcBoost * Math.PI) * 0.28;

        _camOffset.setFromSphericalCoords(
          currentRadius,
          currentPhi,
          currentTheta
        );
        camera.position.copy(currentTarget).add(_camOffset);
        camera.position.y += arcLiftY;

        // Only clamp to the inner well walls when user manually zooms inside the non-cutaway North/East/West walls
        if (
          currentPresetName !== 'frames' &&
          currentTarget.y < -0.60 &&
          camera.position.y < -0.22 &&
          camera.position.z < 0.75
        ) {
          const horizR = Math.hypot(camera.position.x, camera.position.z);
          if (horizR > 0.94) {
            const clampScale = 0.94 / horizR;
            camera.position.x *= clampScale;
            camera.position.z *= clampScale;
          }
          camera.position.y = Math.max(camera.position.y, -3.22);
        }

        camera.lookAt(currentTarget);
      }
    }

    // Drop pinned or hovered object if it was popped/hidden
    if (pinnedObject && !isHierarchyVisible(pinnedObject)) {
      pinnedObject = null;
      emitInspect(hoveredObject && isHierarchyVisible(hoveredObject) ? hoveredObject : null, false);
    }
    if (hoveredObject && !isHierarchyVisible(hoveredObject)) {
      hoveredObject = null;
    }

    // Update Minimalist 8-Corner Hairline Reticle
    const activeInspectTarget = pinnedObject || hoveredObject;
    if (activeInspectTarget && isHierarchyVisible(activeInspectTarget)) {
      const currentStepIdx = stepState.stepIndex ?? 0;
      if (
        activeInspectTarget !== lastBoxTarget ||
        currentStepIdx !== lastBoxStepIndex ||
        rawBox.isEmpty()
      ) {
        if (activeInspectTarget.customBox) {
          rawBox.copy(activeInspectTarget.customBox);
        } else {
          rawBox.setFromObject(activeInspectTarget);
          if (!rawBox.isEmpty()) {
            rawBox.expandByScalar(0.018);
          }
        }
        lastBoxTarget = activeInspectTarget;
        lastBoxStepIndex = currentStepIdx;
      }

      if (!rawBox.isEmpty()) {
        if (!reticleInitialized) {
          smoothMin.copy(rawBox.min);
          smoothMax.copy(rawBox.max);
          reticleInitialized = true;
        } else {
          const boxDamp = 1.0 - Math.exp(-20.0 * dt);
          smoothMin.lerp(rawBox.min, boxDamp);
          smoothMax.lerp(rawBox.max, boxDamp);
        }
        reticle.group.visible = true;
        reticle.updateGeometryFromBox(smoothMin, smoothMax, ambient, Boolean(pinnedObject));

        if (stepState.stepIndex !== undefined && stepState.stepIndex !== lastStepIndex) {
          lastStepIndex = stepState.stepIndex;
          emitInspect(activeInspectTarget, Boolean(pinnedObject));
        }
      } else {
        reticle.group.visible = false;
      }
    } else {
      reticle.group.visible = false;
      reticleInitialized = false;
      lastBoxTarget = null;
    }
  }

  function setOnInspect(fn) {
    inspectCallback = typeof fn === 'function' ? fn : null;
  }

  function dispose() {
    if (domElement && domElement.removeEventListener) {
      domElement.removeEventListener('pointerdown', onPointerDown);
      domElement.removeEventListener('pointermove', onPointerMove);
      domElement.removeEventListener('pointerup', onPointerUp);
      domElement.removeEventListener('pointerleave', onPointerLeave);
      domElement.removeEventListener('wheel', onWheel);
      domElement.removeEventListener('contextmenu', onContextMenu);
    }
    reticle.dispose();
  }

  return {
    group,
    update,
    setCameraPreset,
    getCameraPreset,
    setOnInspect,
    dispose,
  };
}

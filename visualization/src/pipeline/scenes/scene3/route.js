// Route helpers, copied from the earlier leaf chip view and extended to 3D points.
export function polyLen(pts) {
  let L = 0;
  for (let i = 1; i < pts.length; i++) {
    L += Math.hypot(pts[i][0] - pts[i - 1][0], pts[i][1] - pts[i - 1][1], pts[i][2] - pts[i - 1][2]);
  }
  return L;
}

// Point at fraction f of the polyline, written into `out` (no allocation).
export function pointAt(pts, f, out) {
  const L = polyLen(pts) * Math.max(0, Math.min(1, f));
  let acc = 0;
  for (let i = 1; i < pts.length; i++) {
    const a = pts[i - 1];
    const b = pts[i];
    const seg = Math.hypot(b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    if (acc + seg >= L) {
      const u = seg ? (L - acc) / seg : 0;
      out[0] = a[0] + (b[0] - a[0]) * u;
      out[1] = a[1] + (b[1] - a[1]) * u;
      out[2] = a[2] + (b[2] - a[2]) * u;
      return out;
    }
    acc += seg;
  }
  const e = pts[pts.length - 1];
  out[0] = e[0]; out[1] = e[1]; out[2] = e[2];
  return out;
}

// Bus routes from the CORE port: lane (x -3.3 lane 0, x -2.7 lane 1), then the provider branch.
const PORT = [-4.7, 0.3, -0.6];
export const PROVIDER_Z = { alu: -1.7, mem: 0.2, hash: 2.1, page: 4.2 };
export function busRoute(lane, provider) {
  const lx = lane === 0 ? -3.3 : -2.7;
  const z = PROVIDER_Z[provider];
  return [PORT, [lx, 0.3, -0.6], [lx, 0.3, z], [-0.6, 0.3, z]];
}

// Yaw (rotation about y) that points a local +x axis along the route at fraction f.
const A = [0, 0, 0];
const B = [0, 0, 0];
export function routeYaw(pts, f) {
  if (f > 0.03) { pointAt(pts, f - 0.03, A); pointAt(pts, f, B); } else { pointAt(pts, f, A); pointAt(pts, f + 0.03, B); }
  const dx = B[0] - A[0];
  const dz = B[2] - A[2];
  return Math.abs(dx) + Math.abs(dz) < 1e-6 ? 0 : Math.atan2(-dz, dx);
}

import Lean

/-!
# `#audit_ns`

Copied from the software-factory token-bucket example (`TokenBucket/Audit.lean`).
It lists every declaration of a namespace (or of a module under it) that the
compiled oracle runs differently from what the kernel checks:
`implemented_by`, `extern`, `csimp`, `unsafe`, `partial`, `partial_fixpoint`,
`opaque` and `axiom`. Each oracle pins it at `[]` with `#guard_msgs`.
-/

open Lean Elab Command

elab "#audit_ns " ns:ident : command => do
  let env ← getEnv
  let mut bad : Array String := #[]
  for (n, ci) in env.constants.toList do
    let inModule := match env.getModuleIdxFor? n with
      | some i => ns.getId.isPrefixOf (env.header.moduleNames[i.toNat]!)
      | none => false
    if (ns.getId.isPrefixOf n || inModule) && !(n matches .str _ "_unsafe_rec") then
      if (Compiler.getImplementedBy? env n).isSome then bad := bad.push s!"implemented_by {n}"
      if isExtern env n then bad := bad.push s!"extern {n}"
      if ci.isUnsafe then bad := bad.push s!"unsafe {n}"
      if (Lean.Elab.PartialFixpoint.eqnInfoExt.find? env n).isSome then
        bad := bad.push s!"partial_fixpoint {n}"
      if ci.isPartial then bad := bad.push s!"partial {n}"
      if (Compiler.CSimp.ext.getState env).map.contains n then bad := bad.push s!"csimp {n}"
      match ci with
      | .axiomInfo _ => bad := bad.push s!"axiom {n}"
      | .opaqueInfo _ => bad := bad.push s!"opaque {n}"
      | _ => pure ()
  logInfo m!"{bad.qsort (· < ·)}"

module literalEnvironmentContinuity {
  import literalParametricImages.* from "./literalParametricImages"
  // Normal semantic projection only. Premises: owner-admitted finite acyclic
  // EnvRoot/EnvCells and distinct prepared Names. Not a new admission predicate.
  // Value references are observed without decoding Data/Constant payloads.
  type EBinding = EUnvisited | EAbsent | EFound(PRow)
  type EPhysical = {active: int, present: int, index: int, rows: List[PRow]}
  pure def ePhysical(imageArg: PImage): EPhysical = {
    active: imageArg.root.nth(2), present: imageArg.root.nth(6),
    index: imageArg.root.nth(7), rows: imageArg.tables.nth(2),
  }
  // First encountered name wins, matching state_decode::environment. The bound
  // visits at most activeEnvCells cells plus the final absent root. This pure
  // fold is not a B helper microstep, source work unit or runtime guard.
  // EUnvisited remains distinct if an out-of-domain chain does not terminate.
  pure def eLookupAdmitted(imageArg: PImage, nameIdArg: int): EBinding = {
    val projected = ePhysical(imageArg)
    val scan = range(0, projected.active + 1).foldl({
      present: projected.present, index: projected.index, binding: EUnvisited,
    }, (cursorEnv, _) => match cursorEnv.binding {
      | EUnvisited => if (cursorEnv.present == 0)
          {...cursorEnv, binding: EAbsent}
        else {
          val cell = projected.rows.nth(cursorEnv.index)
          if (cell.nth(0) == nameIdArg)
            {...cursorEnv, binding: EFound(List(cell.nth(1), cell.nth(2)))}
          else {present: cell.nth(3), index: cell.nth(4), binding: EUnvisited}
        }
      | _ => cursorEnv
    })
    scan.binding
  }
  pure def eContinuity(beforeArg: PImage, middleArg: PImage,
      doneArg: PImage, nameIdsArg: PRow): bool =
    ePhysical(beforeArg) == ePhysical(middleArg)
      and ePhysical(middleArg) == ePhysical(doneArg)
      and nameIdsArg.indices().forall(namePosition => {
        val nameId = nameIdsArg.nth(namePosition)
        eLookupAdmitted(beforeArg, nameId) == eLookupAdmitted(middleArg, nameId)
          and eLookupAdmitted(middleArg, nameId) == eLookupAdmitted(doneArg, nameId)
      })
}

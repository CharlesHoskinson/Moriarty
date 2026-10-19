module literalCompleteInputs {
  import literalCompleteImages.* from "./literalCompleteImages"
  // SAME-AUTHOR source-shaped positive inputs, not observed images or an oracle.
  // Fixed A types/constructors are from literal_ui_valid.rs. No source hashes or
  // cryptographic context are computed here; context limbs are opaque test atoms.
  pure val contextInput: Row = List(17, 29)
  pure val programHeader: Row = List(0,0,0,1,0,1,0,0,0,0,0,4,4,5,1,0)
  pure def programFor(constantRow: Row): Image = {
    root: programHeader, tables: List(
      List(),
      List(),
      List(List(0, 0, 0, 0)),
      List(),
      List(List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
      List(),
      List(),
      List(),
      List(),
      List(),
      List(List(0, 0, 0), List(1, 0, 0), List(2, 0, 2), List(3, 2, 2)),
      List(List(2, 0, 0, 0), List(2, 1, 0, 2), List(3, 0, 2, 1), List(3, 1, 3, 2)),
      List(List(0), List(2), List(0), List(3), List(3)),
      List(constantRow),
      List()
    ),
  }
  pure val zeroTables: List[List[Row]] = List(
      List(List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
      List(List(0, 0), List(0, 0)),
      List(List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
      List(List(0, 0), List(0, 0)),
      List(List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)),
      List(List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), List(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0))
    )
  pure def initFor(bytesLE: Row): Image = {
    root: List(0,0,0,0,0,1,0,0,2).concat(bytesLE).concat(bytesLE).concat(List(0,0,0,0)),
    tables: zeroTables,
  }
  pure val oneFuel: Row = List(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
  pure val borrowFuel: Row = List(0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
  pure val uintProgram: Image = programFor(List(0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0))
  pure val boolProgram: Image = programFor(List(1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0))
  pure val uintInit: Image = initFor(oneFuel)
  pure val boolInit: Image = initFor(borrowFuel)
  action init = initWith(uintProgram, uintInit, contextInput)
}

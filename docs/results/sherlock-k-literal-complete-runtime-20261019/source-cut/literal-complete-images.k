requires "domains.md"

// Isolated logical L4 carrier model, not an import or replacement of main.k.
// Positive domain: prepared UInt literal entry, complete Init, explicit context.
// No hash, authentication, field serialization, admission or projectA relation.
module MORIARTY-LITERAL-COMPLETE-IMAGES-SYNTAX
  imports DOMAINS-SYNTAX
  syntax LCRow ::= lcNoCells() | lcCell(Int, LCRow)
  syntax LCRows ::= lcNoRows() | lcRow(LCRow, LCRows)
  syntax LCTables ::= lcNoTables() | lcTable(LCRows, LCTables)
  syntax LCImage ::= lcImage(LCRow, LCTables)
  syntax LCContext ::= lcContext(Int, Int)
  syntax LCCaps ::= lcCaps(LCRow, LCRow, Int)
  syntax LCMachine ::= lcMachine(LCContext, LCCaps, LCImage, LCImage)
  syntax LCStates ::= lcNoStates() | lcState(LCMachine, LCStates)
  syntax LCEvents ::= lcNoEvents()
                    | lcEvent(Int, Int, Int, LCMachine, LCMachine, LCEvents)
  syntax LCTrace ::= lcTrace(LCMachine, LCStates, LCMachine, LCEvents)
  syntax LCChange ::= lcChange(Int, Int, Int, LCMachine, LCMachine)
  syntax LCResult ::= lcResult(LCTrace, LCRow)
  syntax LCRun ::= lcRun(LCContext, LCCaps, LCImage, LCImage, Int)
  syntax KItem ::= LCRun | LCResult
endmodule

module MORIARTY-LITERAL-COMPLETE-IMAGES
  imports MORIARTY-LITERAL-COMPLETE-IMAGES-SYNTAX
  imports DOMAINS

  configuration <k> $PGM:LCRun </k>

  // The observation budget bounds this model's walk, not semantic fuel/work.
  rule <k> lcRun(C, B, P, S, N)
        => lcFinish(lcWalk(lcMachine(C, B, P, S), N)) </k>
    requires N >=Int 2 andBool lcGoodInput(C, B, P, S)

  syntax Int ::= lcLen(LCRow) [function]
               | lcGet(LCRow, Int) [function]
               | lcLE(LCRow) [function]
  rule lcLen(lcNoCells()) => 0
  rule lcLen(lcCell(_, R)) => 1 +Int lcLen(R)
  rule lcGet(lcCell(X, _), 0) => X
  rule lcGet(lcCell(_, R), N) => lcGet(R, N -Int 1) requires N >Int 0
  rule lcLE(lcNoCells()) => 0
  rule lcLE(lcCell(X, R)) => X +Int (256 *Int lcLE(R))

  syntax LCRow ::= lcTake(LCRow, Int) [function]
                 | lcDrop(LCRow, Int) [function]
                 | lcSet(LCRow, Int, Int) [function]
                 | lcCat(LCRow, LCRow) [function]
                 | lcZeros(Int) [function]
                 | lcBorrow(LCRow, Int) [function]
  rule lcTake(_, 0) => lcNoCells()
  rule lcTake(lcCell(X, R), N) => lcCell(X, lcTake(R, N -Int 1)) requires N >Int 0
  rule lcDrop(R, 0) => R
  rule lcDrop(lcCell(_, R), N) => lcDrop(R, N -Int 1) requires N >Int 0
  rule lcSet(lcCell(_, R), 0, X) => lcCell(X, R)
  rule lcSet(lcCell(H, R), N, X) => lcCell(H, lcSet(R, N -Int 1, X)) requires N >Int 0
  rule lcCat(lcNoCells(), R) => R
  rule lcCat(lcCell(X, R), S) => lcCell(X, lcCat(R, S))
  rule lcZeros(0) => lcNoCells()
  rule lcZeros(N) => lcCell(0, lcZeros(N -Int 1)) requires N >Int 0
  // Called only on positive, exactly sixteen-byte fuel. No wraparound clause.
  rule lcBorrow(R, 0) => R
  rule lcBorrow(lcCell(0, R), 1) => lcCell(255, lcBorrow(R, 1))
  rule lcBorrow(lcCell(X, R), 1) => lcCell(X -Int 1, R) requires X >Int 0

  syntax LCRow ::= lcRowsAt(LCRows, Int) [function]
  syntax LCRows ::= lcTableAt(LCTables, Int) [function]
                  | lcSetRow(LCRows, Int, LCRow) [function]
                  | lcZeroRows(LCRows) [function]
  syntax LCTables ::= lcSetTable(LCTables, Int, LCRows) [function]
  rule lcRowsAt(lcRow(R, _), 0) => R
  rule lcRowsAt(lcRow(_, R), N) => lcRowsAt(R, N -Int 1) requires N >Int 0
  rule lcTableAt(lcTable(T, _), 0) => T
  rule lcTableAt(lcTable(_, T), N) => lcTableAt(T, N -Int 1) requires N >Int 0
  rule lcSetRow(lcRow(_, R), 0, X) => lcRow(X, R)
  rule lcSetRow(lcRow(H, R), N, X) => lcRow(H, lcSetRow(R, N -Int 1, X)) requires N >Int 0
  rule lcSetTable(lcTable(_, T), 0, X) => lcTable(X, T)
  rule lcSetTable(lcTable(H, T), N, X) => lcTable(H, lcSetTable(T, N -Int 1, X)) requires N >Int 0
  rule lcZeroRows(lcNoRows()) => lcNoRows()
  rule lcZeroRows(lcRow(R, RS)) => lcRow(lcZeros(lcLen(R)), lcZeroRows(RS))

  syntax Bool ::= lcUInt(Int) [function]
                | lcCells(LCRow, Int) [function]
                | lcRowShape(LCRows, Int, Int) [function]
                | lcTablesShape(LCTables, LCRow, LCRow) [function]
                | lcAllZero(LCRow) [function]
                | lcRowsZero(LCRows) [function]
                | lcTablesZero(LCTables) [function]
                | lcCounts(LCRow, LCRow) [function]
  rule lcUInt(X) => X >=Int 0 andBool X <Int 340282366920938463463374607431768211456
  rule lcCells(lcNoCells(), _) => true
  rule lcCells(lcCell(X, R), B) => X >=Int 0 andBool X <Int B andBool lcCells(R, B)
  rule lcRowShape(lcNoRows(), 0, _) => true
  rule lcRowShape(lcRow(R, RS), N, W)
    => lcLen(R) ==Int W andBool lcCells(R, 340282366920938463463374607431768211456)
       andBool lcRowShape(RS, N -Int 1, W) requires N >Int 0
  rule lcRowShape(_, _, _) => false [owise]
  rule lcTablesShape(lcNoTables(), lcNoCells(), lcNoCells()) => true
  rule lcTablesShape(lcTable(T, TS), lcCell(N, NS), lcCell(W, WS))
    => N >=Int 0 andBool lcRowShape(T, N, W) andBool lcTablesShape(TS, NS, WS)
  rule lcTablesShape(_, _, _) => false [owise]
  rule lcAllZero(lcNoCells()) => true
  rule lcAllZero(lcCell(X, R)) => X ==Int 0 andBool lcAllZero(R)
  rule lcRowsZero(lcNoRows()) => true
  rule lcRowsZero(lcRow(R, RS)) => lcAllZero(R) andBool lcRowsZero(RS)
  rule lcTablesZero(lcNoTables()) => true
  rule lcTablesZero(lcTable(T, TS)) => lcRowsZero(T) andBool lcTablesZero(TS)
  rule lcCounts(lcNoCells(), lcNoCells()) => true
  rule lcCounts(lcCell(X, R), lcCell(B, BS))
    => X >=Int 0 andBool X <=Int B andBool lcCounts(R, BS)
  rule lcCounts(_, _) => false [owise]

  // Exact logical column widths from literal_host::literal_schema, not bytes/fields.
  syntax LCRow ::= lcProgramWidths() [function] | lcStateWidths() [function]
  rule lcProgramWidths() => lcCell(2, lcCell(1, lcCell(4, lcCell(1,
    lcCell(11, lcCell(1, lcCell(6, lcCell(2, lcCell(3, lcCell(2,
    lcCell(3, lcCell(4, lcCell(1, lcCell(22, lcCell(2, lcNoCells())))))))))))))))
  rule lcStateWidths() => lcCell(22, lcCell(2, lcCell(5,
    lcCell(2, lcCell(17, lcCell(36, lcNoCells()))))))

  syntax Bool ::= lcGoodInput(LCContext, LCCaps, LCImage, LCImage) [function]
                | lcInitRoot(LCRow, Int) [function]
                | lcScalarLiteral(LCRow, LCTables) [function]
                | lcUIntConstant(LCRow) [function]
  rule lcGoodInput(lcContext(A, B), lcCaps(PC, SC, SL), lcImage(P, PT), lcImage(S, ST))
    => lcUInt(A) andBool lcUInt(B) andBool lcUInt(SL)
       andBool lcLen(PC) ==Int 15 andBool lcLen(SC) ==Int 6
       andBool lcLen(P) ==Int 16 andBool lcLen(S) ==Int 45
       andBool lcCells(P, 340282366920938463463374607431768211456)
       andBool lcCells(S, 340282366920938463463374607431768211456)
       andBool lcTablesShape(PT, PC, lcProgramWidths())
       andBool lcTablesShape(ST, SC, lcStateWidths())
       andBool lcCounts(lcDrop(P, 1), PC)
       andBool lcGet(P, 0) ==Int 0
       andBool lcGet(P, 5) >Int 0 andBool lcGet(P, 14) >Int 0
       andBool lcGet(PC, 10) >=Int 4 andBool lcGet(PC, 11) >=Int 4
       andBool lcGet(PC, 12) >=Int 5 andBool lcGet(SC, 5) >Int 0
       andBool lcInitRoot(S, SL) andBool lcTablesZero(ST)
       andBool lcScalarLiteral(P, PT)
  // Init tables, including Eval(0) and every unused row, are entirely zero.
  rule lcInitRoot(S, SL)
    => lcAllZero(lcTake(S, 5)) andBool lcGet(S, 5) ==Int 1
       andBool lcAllZero(lcTake(lcDrop(S, 6), 2)) andBool lcGet(S, 8) <=Int SL
       andBool lcTake(lcDrop(S, 9), 16) ==K lcTake(lcDrop(S, 25), 16)
       andBool lcCells(lcTake(lcDrop(S, 9), 16), 256)
       andBool lcLE(lcTake(lcDrop(S, 9), 16)) >Int 0
       andBool lcAllZero(lcDrop(S, 41))
  rule lcScalarLiteral(P, PT)
    => lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 0) ==Int 0
       andBool lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 1) ==Int 0
       andBool lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 2) ==Int 0
       andBool lcAllZero(lcDrop(lcRowsAt(lcTableAt(PT, 4), 0), 4))
       andBool lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 3) <Int lcGet(P, 14)
       andBool lcUIntConstant(lcRowsAt(lcTableAt(PT, 13),
                         lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 3)))
  rule lcUIntConstant(R)
    => lcAllZero(lcTake(R, 3))
       andBool lcCells(lcTake(lcDrop(R, 3), 16), 256)
       andBool lcAllZero(lcDrop(R, 19))

  syntax LCImage ::= lcA102(LCImage, LCImage) [function]
                   | lcA80(LCImage) [function]
  syntax LCRow ::= lcReturnRow(Int, Int) [function]
  // Row tag=Return(1), exprId=0, then the actual selected ValueRef and zero tail.
  rule lcReturnRow(S, I) => lcCell(1, lcCell(0, lcCell(S, lcCell(I, lcZeros(32)))))
  // Entry row 0 is selected because lcGoodInput requires all-zero Init
  // control tables (Eval exprId=0); no later Eval is reachable in this entry.
  rule lcA102(lcImage(_, PT), lcImage(R, ST))
    => lcImage(lcCat(lcTake(R, 9), lcCat(lcBorrow(lcTake(lcDrop(R, 9), 16), 1), lcDrop(R, 25))),
       lcSetTable(ST, 5, lcSetRow(lcTableAt(ST, 5), 0,
         lcReturnRow(lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 2),
                     lcGet(lcRowsAt(lcTableAt(PT, 4), 0), 3)))))
  rule lcA80(lcImage(R, ST))
    => lcImage(lcSet(lcSet(lcSet(lcSet(lcSet(R, 5, 0), 41, 1),
          42, lcGet(lcRowsAt(lcTableAt(ST, 5), 0), 2)),
          43, lcGet(lcRowsAt(lcTableAt(ST, 5), 0), 3)), 44, 0),
       lcSetTable(ST, 5, lcZeroRows(lcTableAt(ST, 5))))

  syntax LCChange ::= lcAdvance(LCMachine) [function]
  rule lcAdvance(lcMachine(C, B, P, lcImage(R, ST)))
    => lcChange(102, 3, 1, lcMachine(C, B, P, lcImage(R, ST)),
                            lcMachine(C, B, P, lcA102(P, lcImage(R, ST))))
    requires lcGet(R, 5) ==Int 1 andBool lcGet(R, 41) ==Int 0
     andBool lcGet(lcRowsAt(lcTableAt(ST, 5), 0), 0) ==Int 0
     andBool lcLE(lcTake(lcDrop(R, 9), 16)) >Int 0
  rule lcAdvance(lcMachine(C, B, P, lcImage(R, ST)))
    => lcChange(80, 0, 0, lcMachine(C, B, P, lcImage(R, ST)),
                           lcMachine(C, B, P, lcA80(lcImage(R, ST))))
    requires lcGet(R, 5) ==Int 1 andBool lcGet(R, 41) ==Int 0
     andBool lcGet(R, 4) ==Int 0
     andBool lcGet(lcRowsAt(lcTableAt(ST, 5), 0), 0) ==Int 1

  syntax LCTrace ::= lcWalk(LCMachine, Int) [function]
                   | lcContinue(LCChange, Int) [function]
                   | lcPrepend(LCChange, LCTrace) [function]
  rule lcWalk(M, 0) => lcTrace(M, lcNoStates(), M, lcNoEvents())
  rule lcWalk(lcMachine(C, B, P, lcImage(R, ST)), N)
    => lcTrace(lcMachine(C, B, P, lcImage(R, ST)), lcNoStates(),
               lcMachine(C, B, P, lcImage(R, ST)), lcNoEvents())
    requires N >Int 0 andBool lcGet(R, 5) ==Int 0 andBool lcGet(R, 41) ==Int 1
  rule lcWalk(lcMachine(C, B, P, lcImage(R, ST)), N)
    => lcContinue(lcAdvance(lcMachine(C, B, P, lcImage(R, ST))), N -Int 1)
    requires N >Int 0 andBool lcGet(R, 5) ==Int 1 andBool lcGet(R, 41) ==Int 0
  rule lcContinue(lcChange(L, T, Q, A, B), N)
    => lcPrepend(lcChange(L, T, Q, A, B), lcWalk(B, N))
  rule lcPrepend(lcChange(L, T, Q, A, B), lcTrace(B, SS, Z, ES))
    => lcTrace(A, lcState(B, SS), Z, lcEvent(L, T, Q, A, B, ES))

  syntax LCRow ::= lcFlatRows(LCRows) [function]
                 | lcFlatTables(LCTables) [function]
                 | lcFlat(LCImage) [function]
  rule lcFlatRows(lcNoRows()) => lcNoCells()
  rule lcFlatRows(lcRow(R, RS)) => lcCat(R, lcFlatRows(RS))
  rule lcFlatTables(lcNoTables()) => lcNoCells()
  rule lcFlatTables(lcTable(T, TS)) => lcCat(lcFlatRows(T), lcFlatTables(TS))
  rule lcFlat(lcImage(R, TS)) => lcCat(R, lcFlatTables(TS))

  syntax LCResult ::= lcFinish(LCTrace) [function]
  // Both envelopes come from actual steps; nonlinear variables bind the real join.
  // Statement order is complete_bridge::LiteralCompleteLayout::statement.
  rule lcFinish(lcTrace(lcMachine(lcContext(A, B), C, P, I),
         lcState(lcMachine(lcContext(A, B), C, P, M),
           lcState(lcMachine(lcContext(A, B), C, P, D), lcNoStates())),
         lcMachine(lcContext(A, B), C, P, D),
         lcEvent(102, T1, Q1, lcMachine(lcContext(A, B), C, P, I),
                             lcMachine(lcContext(A, B), C, P, M),
           lcEvent(80, T2, Q2, lcMachine(lcContext(A, B), C, P, M),
                              lcMachine(lcContext(A, B), C, P, D), lcNoEvents()))))
    => lcResult(lcTrace(lcMachine(lcContext(A, B), C, P, I),
         lcState(lcMachine(lcContext(A, B), C, P, M),
           lcState(lcMachine(lcContext(A, B), C, P, D), lcNoStates())),
         lcMachine(lcContext(A, B), C, P, D),
         lcEvent(102, T1, Q1, lcMachine(lcContext(A, B), C, P, I),
                             lcMachine(lcContext(A, B), C, P, M),
           lcEvent(80, T2, Q2, lcMachine(lcContext(A, B), C, P, M),
                              lcMachine(lcContext(A, B), C, P, D), lcNoEvents()))),
         lcCell(A, lcCell(B, lcCat(lcFlat(P), lcCat(lcFlat(I),
           lcCat(lcFlat(M), lcCat(lcFlat(D), lcCell(T1, lcCell(Q1,
             lcCell(T2, lcCell(Q2, lcNoCells())))))))))))
endmodule

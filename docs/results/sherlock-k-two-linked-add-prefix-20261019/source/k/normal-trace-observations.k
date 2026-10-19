requires "nominal-product.k"

// Test-only complete-state observer. No expected data enters ngStep.
module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS
  imports SHERLOCK-NOMINAL-PRODUCT
  syntax NGStates ::= ngNoStates() | ngStateRow(NGState, NGStates)
  syntax NGTape ::= ngTape(NGState, NGStates, NGState)
                 | ngObserve(NGState, Int) [function]
                 | ngObserveLoop(NGState, NGState, NGStates, Int) [function]
                 | ngObserveNext(NGState, NGState, NGStates, Int) [function]
  syntax NGStates ::= ngAppendState(NGStates, NGState) [function]
  rule ngAppendState(ngNoStates(), S) => ngStateRow(S, ngNoStates())
  rule ngAppendState(ngStateRow(S, Ss), T)
    => ngStateRow(S, ngAppendState(Ss, T))
  // Internal history is newest-first; emit the same chronological tape once.
  syntax NGStates ::= ngReverseStates(NGStates, NGStates) [function]
  rule ngReverseStates(ngNoStates(), Acc) => Acc
  rule ngReverseStates(ngStateRow(S, Ss), Acc)
    => ngReverseStates(Ss, ngStateRow(S, Acc))
  rule ngObserve(S, N) => ngObserveLoop(S, S, ngNoStates(), N)
  rule ngObserveLoop(Initial, Current, States, 0)
    => ngTape(Initial, ngReverseStates(States, ngNoStates()), Current)
  rule ngObserveLoop(Initial, ngState(P, C, Rho, Fs, Es), States, N)
    => ngTape(Initial, ngReverseStates(States, ngNoStates()), ngState(P, C, Rho, Fs, Es))
    requires N >Int 0 andBool ngTerminal(C)
  rule ngObserveLoop(Initial, ngState(P, C, Rho, Fs, Es), States, N)
    => ngObserveNext(Initial, ngStep(ngState(P, C, Rho, Fs, Es)), States, N -Int 1)
    requires N >Int 0 andBool notBool ngTerminal(C)
  rule ngObserveNext(Initial, Next, States, N)
    => ngObserveLoop(Initial, Next, ngStateRow(Next, States), N)
endmodule

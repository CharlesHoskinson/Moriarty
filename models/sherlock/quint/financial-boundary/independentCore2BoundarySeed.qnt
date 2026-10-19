// Independent adopted-contract input only; no expected model result.
// Do not derive this seed from an actual Core2 publication.
module independentCore2BoundarySeed {
  import financialBoundary.* from "./financialBoundary"
  pure val suppliedInvocation: Invocation = {
    guardCosts: Map(1 -> 2),
    identity: {
      actionName: "carry",
      planHash: "a0ad53aae0b47e2bff7266e0f35dee8f3e9c559816549cf998c111fde50da619",
      programHash: "c698ee1e1d3742579d2b328171b7257748533cff936e9fb74fb909c835ccbc04",
      semantics: "moriarty-core/2"
    },
    input: ResumeInput({
        cliAction: NoText,
        cliCap: NoUInt,
        continuation: {
          actionName: "carry",
          boundary: {
            balances: List({
                account: "holder-a",
                amount: 23,
                asset: {
                  issuer: "core2-b06-carry-probe",
                  network: "local",
                  reference: "UNIT",
                  scale: 2
                }
              }, {
                account: "holder-b",
                amount: 0,
                asset: {
                  issuer: "core2-b06-carry-probe",
                  network: "local",
                  reference: "UNIT",
                  scale: 2
                }
              }),
            liabilities: List({
                amount: 5,
                asset: {
                  issuer: "core2-b06-carry-probe",
                  network: "local",
                  reference: "UNIT",
                  scale: 2
                },
                creditor: "holder-b",
                debtor: "holder-a",
                name: "duty-open"
              }, {
                amount: 0,
                asset: {
                  issuer: "core2-b06-carry-probe",
                  network: "local",
                  reference: "UNIT",
                  scale: 2
                },
                creditor: "holder-b",
                debtor: "holder-a",
                name: "duty-zero"
              }),
            state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(37))
          },
          captures: List({
              name: "retained",
              value: UIntValue(37)
            }),
          format: "moriarty-continuation/1",
          logical: {
            cap: SomeUInt(64),
            committed: 3,
            profile: "moriarty-reference-logical-cost/2"
          },
          planHash: "a0ad53aae0b47e2bff7266e0f35dee8f3e9c559816549cf998c111fde50da619",
          programHash: "c698ee1e1d3742579d2b328171b7257748533cff936e9fb74fb909c835ccbc04",
          site: 1
        },
        event: {
          actionName: "carry",
          format: "moriarty-stage-event/1",
          label: "gate",
          values: List({
              name: "ready",
              value: BoolValue(false)
            })
        }
      }),
    plan: {
      actionName: "carry",
      committedBefore: List(0, 3),
      planTotal: 7,
      segmentCosts: List(3, 4),
      sites: List({
          binders: List({
              name: "ready",
              ty: BoolType
            }),
          captures: List({
              name: "retained",
              ordinal: 0,
              origin: LetOrigin,
              ty: UIntType
            }),
          label: "gate",
          ordinal: 1,
          site: 1
        }),
      statementCount: 3
    },
    runtime: {
      attempted: 2,
      runtimeResult: NotAdvanced
    },
    startBoundary: {
      balances: List({
          account: "holder-a",
          amount: 23,
          asset: {
            issuer: "core2-b06-carry-probe",
            network: "local",
            reference: "UNIT",
            scale: 2
          }
        }, {
          account: "holder-b",
          amount: 0,
          asset: {
            issuer: "core2-b06-carry-probe",
            network: "local",
            reference: "UNIT",
            scale: 2
          }
        }),
      liabilities: List({
          amount: 5,
          asset: {
            issuer: "core2-b06-carry-probe",
            network: "local",
            reference: "UNIT",
            scale: 2
          },
          creditor: "holder-b",
          debtor: "holder-a",
          name: "duty-open"
        }, {
          amount: 0,
          asset: {
            issuer: "core2-b06-carry-probe",
            network: "local",
            reference: "UNIT",
            scale: 2
          },
          creditor: "holder-b",
          debtor: "holder-a",
          name: "duty-zero"
        }),
      state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(37))
    }
  }
}

// Independent conditional B18 invocation input only, not expected results.
// Requires Root-adopted finite scope/port contract before use.
module independentCore2SequenceSeed {
  import financialBoundary.* from "./financialBoundary"
  pure val suppliedInvocations: List[Invocation] = List({
      identity: {
        programHash: "811705bdd52c12fb56e69110550485bc7d3983b20b4863a094da2d15bdcd4db0",
        planHash: "c85f009c3be2463302b4bd03c9d9a4ddd18033408db85983ca79356d96194363",
        actionName: "advance",
        semantics: "moriarty-core/2"
      },
      plan: {
        actionName: "advance",
        statementCount: 6,
        sites: List({
            site: 1,
            ordinal: 1,
            label: "gate",
            binders: List({
                name: "ready",
                ty: BoolType
              }),
            captures: List({
                name: "delta",
                ty: UIntType,
                origin: InputOrigin,
                ordinal: 0
              }, {
                name: "retained",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 0
              })
          }, {
            site: 2,
            ordinal: 5,
            label: "finish",
            binders: List({
                name: "accepted",
                ty: BoolType
              }),
            captures: List({
                name: "delta",
                ty: UIntType,
                origin: InputOrigin,
                ordinal: 0
              }, {
                name: "retained",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 0
              }, {
                name: "ready",
                ty: BoolType,
                origin: EventOrigin,
                ordinal: 1
              }, {
                name: "seen",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 4
              })
          }),
        segmentCosts: List(3, 11, 2),
        committedBefore: List(0, 3, 14),
        planTotal: 16
      },
      guardCosts: Map(1 -> 2, 2 -> 2),
      input: StartInput({
          actionName: "advance",
          scenario: {
            state: Map("marker" -> UIntValue(41), "flag" -> BoolValue(false)),
            inputs: Map("delta" -> UIntValue(6)),
            balances: List({
                account: "holder-a",
                asset: {
                  network: "local",
                  issuer: "core2-b06-sequence-probe",
                  reference: "UNIT",
                  scale: 2
                },
                amount: 31
              }, {
                account: "holder-b",
                asset: {
                  network: "local",
                  issuer: "core2-b06-sequence-probe",
                  reference: "UNIT",
                  scale: 2
                },
                amount: 2
              }),
            liabilities: List({
                name: "duty-open",
                asset: {
                  network: "local",
                  issuer: "core2-b06-sequence-probe",
                  reference: "UNIT",
                  scale: 2
                },
                debtor: "holder-a",
                creditor: "holder-b",
                amount: 9
              }, {
                name: "duty-zero",
                asset: {
                  network: "local",
                  issuer: "core2-b06-sequence-probe",
                  reference: "UNIT",
                  scale: 2
                },
                debtor: "holder-a",
                creditor: "holder-b",
                amount: 0
              })
          },
          cap: SomeUInt(96)
        }),
      startBoundary: {
        state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
        balances: List({
            account: "holder-a",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            amount: 31
          }, {
            account: "holder-b",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            amount: 2
          }),
        liabilities: List({
            name: "duty-open",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            debtor: "holder-a",
            creditor: "holder-b",
            amount: 9
          }, {
            name: "duty-zero",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            debtor: "holder-a",
            creditor: "holder-b",
            amount: 0
          })
      },
      runtime: {
        runtimeResult: Ran({
            execution: {
              semantics: "moriarty-core/2",
              actionName: "advance",
              pre: {
                state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
                balances: List({
                    account: "holder-a",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 31
                  }, {
                    account: "holder-b",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 2
                  }),
                liabilities: List({
                    name: "duty-open",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 9
                  }, {
                    name: "duty-zero",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 0
                  })
              },
              post: {
                state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
                balances: List({
                    account: "holder-a",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 31
                  }, {
                    account: "holder-b",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 2
                  }),
                liabilities: List({
                    name: "duty-open",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 9
                  }, {
                    name: "duty-zero",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 0
                  })
              },
              steps: List({
                  index: 0,
                  statement: LetStatement({
                      name: "retained",
                      value: 0
                    }),
                  value: SomeScalar(UIntValue(41)),
                  snapshot: {
                    state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
                    balances: List({
                        account: "holder-a",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 31
                      }, {
                        account: "holder-b",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 2
                      }),
                    liabilities: List({
                        name: "duty-open",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 9
                      }, {
                        name: "duty-zero",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 0
                      })
                  },
                  effects: List()
                }),
              effects: List()
            },
            exitSite: 1,
            capturesOut: List({
                name: "delta",
                value: UIntValue(6)
              }, {
                name: "retained",
                value: UIntValue(41)
              })
          }),
        attempted: 3
      }
    }, {
      identity: {
        programHash: "811705bdd52c12fb56e69110550485bc7d3983b20b4863a094da2d15bdcd4db0",
        planHash: "c85f009c3be2463302b4bd03c9d9a4ddd18033408db85983ca79356d96194363",
        actionName: "advance",
        semantics: "moriarty-core/2"
      },
      plan: {
        actionName: "advance",
        statementCount: 6,
        sites: List({
            site: 1,
            ordinal: 1,
            label: "gate",
            binders: List({
                name: "ready",
                ty: BoolType
              }),
            captures: List({
                name: "delta",
                ty: UIntType,
                origin: InputOrigin,
                ordinal: 0
              }, {
                name: "retained",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 0
              })
          }, {
            site: 2,
            ordinal: 5,
            label: "finish",
            binders: List({
                name: "accepted",
                ty: BoolType
              }),
            captures: List({
                name: "delta",
                ty: UIntType,
                origin: InputOrigin,
                ordinal: 0
              }, {
                name: "retained",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 0
              }, {
                name: "ready",
                ty: BoolType,
                origin: EventOrigin,
                ordinal: 1
              }, {
                name: "seen",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 4
              })
          }),
        segmentCosts: List(3, 11, 2),
        committedBefore: List(0, 3, 14),
        planTotal: 16
      },
      guardCosts: Map(1 -> 2, 2 -> 2),
      input: ResumeInput({
          continuation: {
            format: "moriarty-continuation/1",
            programHash: "811705bdd52c12fb56e69110550485bc7d3983b20b4863a094da2d15bdcd4db0",
            planHash: "c85f009c3be2463302b4bd03c9d9a4ddd18033408db85983ca79356d96194363",
            actionName: "advance",
            site: 1,
            captures: List({
                name: "delta",
                value: UIntValue(6)
              }, {
                name: "retained",
                value: UIntValue(41)
              }),
            logical: {
              profile: "moriarty-reference-logical-cost/2",
              cap: SomeUInt(96),
              committed: 3
            },
            boundary: {
              state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
              balances: List({
                  account: "holder-a",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  amount: 31
                }, {
                  account: "holder-b",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  amount: 2
                }),
              liabilities: List({
                  name: "duty-open",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  debtor: "holder-a",
                  creditor: "holder-b",
                  amount: 9
                }, {
                  name: "duty-zero",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  debtor: "holder-a",
                  creditor: "holder-b",
                  amount: 0
                })
            }
          },
          event: {
            format: "moriarty-stage-event/1",
            actionName: "advance",
            label: "gate",
            values: List({
                name: "ready",
                value: BoolValue(false)
              })
          },
          cliAction: NoText,
          cliCap: NoUInt
        }),
      startBoundary: {
        state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
        balances: List({
            account: "holder-a",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            amount: 31
          }, {
            account: "holder-b",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            amount: 2
          }),
        liabilities: List({
            name: "duty-open",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            debtor: "holder-a",
            creditor: "holder-b",
            amount: 9
          }, {
            name: "duty-zero",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            debtor: "holder-a",
            creditor: "holder-b",
            amount: 0
          })
      },
      runtime: {
        runtimeResult: NotAdvanced,
        attempted: 2
      }
    }, {
      identity: {
        programHash: "811705bdd52c12fb56e69110550485bc7d3983b20b4863a094da2d15bdcd4db0",
        planHash: "c85f009c3be2463302b4bd03c9d9a4ddd18033408db85983ca79356d96194363",
        actionName: "advance",
        semantics: "moriarty-core/2"
      },
      plan: {
        actionName: "advance",
        statementCount: 6,
        sites: List({
            site: 1,
            ordinal: 1,
            label: "gate",
            binders: List({
                name: "ready",
                ty: BoolType
              }),
            captures: List({
                name: "delta",
                ty: UIntType,
                origin: InputOrigin,
                ordinal: 0
              }, {
                name: "retained",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 0
              })
          }, {
            site: 2,
            ordinal: 5,
            label: "finish",
            binders: List({
                name: "accepted",
                ty: BoolType
              }),
            captures: List({
                name: "delta",
                ty: UIntType,
                origin: InputOrigin,
                ordinal: 0
              }, {
                name: "retained",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 0
              }, {
                name: "ready",
                ty: BoolType,
                origin: EventOrigin,
                ordinal: 1
              }, {
                name: "seen",
                ty: UIntType,
                origin: LetOrigin,
                ordinal: 4
              })
          }),
        segmentCosts: List(3, 11, 2),
        committedBefore: List(0, 3, 14),
        planTotal: 16
      },
      guardCosts: Map(1 -> 2, 2 -> 2),
      input: ResumeInput({
          continuation: {
            format: "moriarty-continuation/1",
            programHash: "811705bdd52c12fb56e69110550485bc7d3983b20b4863a094da2d15bdcd4db0",
            planHash: "c85f009c3be2463302b4bd03c9d9a4ddd18033408db85983ca79356d96194363",
            actionName: "advance",
            site: 1,
            captures: List({
                name: "delta",
                value: UIntValue(6)
              }, {
                name: "retained",
                value: UIntValue(41)
              }),
            logical: {
              profile: "moriarty-reference-logical-cost/2",
              cap: SomeUInt(96),
              committed: 3
            },
            boundary: {
              state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
              balances: List({
                  account: "holder-a",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  amount: 31
                }, {
                  account: "holder-b",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  amount: 2
                }),
              liabilities: List({
                  name: "duty-open",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  debtor: "holder-a",
                  creditor: "holder-b",
                  amount: 9
                }, {
                  name: "duty-zero",
                  asset: {
                    network: "local",
                    issuer: "core2-b06-sequence-probe",
                    reference: "UNIT",
                    scale: 2
                  },
                  debtor: "holder-a",
                  creditor: "holder-b",
                  amount: 0
                })
            }
          },
          event: {
            format: "moriarty-stage-event/1",
            actionName: "advance",
            label: "gate",
            values: List({
                name: "ready",
                value: BoolValue(true)
              })
          },
          cliAction: NoText,
          cliCap: NoUInt
        }),
      startBoundary: {
        state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
        balances: List({
            account: "holder-a",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            amount: 31
          }, {
            account: "holder-b",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            amount: 2
          }),
        liabilities: List({
            name: "duty-open",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            debtor: "holder-a",
            creditor: "holder-b",
            amount: 9
          }, {
            name: "duty-zero",
            asset: {
              network: "local",
              issuer: "core2-b06-sequence-probe",
              reference: "UNIT",
              scale: 2
            },
            debtor: "holder-a",
            creditor: "holder-b",
            amount: 0
          })
      },
      runtime: {
        runtimeResult: Ran({
            execution: {
              semantics: "moriarty-core/2",
              actionName: "advance",
              pre: {
                state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
                balances: List({
                    account: "holder-a",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 31
                  }, {
                    account: "holder-b",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 2
                  }),
                liabilities: List({
                    name: "duty-open",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 9
                  }, {
                    name: "duty-zero",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 0
                  })
              },
              post: {
                state: Map("flag" -> BoolValue(true), "marker" -> UIntValue(47)),
                balances: List({
                    account: "holder-a",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 31
                  }, {
                    account: "holder-b",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    amount: 2
                  }),
                liabilities: List({
                    name: "duty-open",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 9
                  }, {
                    name: "duty-zero",
                    asset: {
                      network: "local",
                      issuer: "core2-b06-sequence-probe",
                      reference: "UNIT",
                      scale: 2
                    },
                    debtor: "holder-a",
                    creditor: "holder-b",
                    amount: 0
                  })
              },
              steps: List({
                  index: 1,
                  statement: AwaitStatement({
                      label: "gate",
                      binders: List({
                          name: "ready",
                          ty: BoolType
                        }),
                      guard: 1
                    }),
                  value: SomeScalar(BoolValue(true)),
                  snapshot: {
                    state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(41)),
                    balances: List({
                        account: "holder-a",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 31
                      }, {
                        account: "holder-b",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 2
                      }),
                    liabilities: List({
                        name: "duty-open",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 9
                      }, {
                        name: "duty-zero",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 0
                      })
                  },
                  effects: List()
                }, {
                  index: 2,
                  statement: NextStatement({
                      state: "marker",
                      value: 4
                    }),
                  value: SomeScalar(UIntValue(47)),
                  snapshot: {
                    state: Map("flag" -> BoolValue(false), "marker" -> UIntValue(47)),
                    balances: List({
                        account: "holder-a",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 31
                      }, {
                        account: "holder-b",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 2
                      }),
                    liabilities: List({
                        name: "duty-open",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 9
                      }, {
                        name: "duty-zero",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 0
                      })
                  },
                  effects: List(StateEffect({
                        name: "marker",
                        before: UIntValue(41),
                        after: UIntValue(47)
                      }))
                }, {
                  index: 3,
                  statement: NextStatement({
                      state: "flag",
                      value: 5
                    }),
                  value: SomeScalar(BoolValue(true)),
                  snapshot: {
                    state: Map("flag" -> BoolValue(true), "marker" -> UIntValue(47)),
                    balances: List({
                        account: "holder-a",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 31
                      }, {
                        account: "holder-b",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 2
                      }),
                    liabilities: List({
                        name: "duty-open",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 9
                      }, {
                        name: "duty-zero",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 0
                      })
                  },
                  effects: List(StateEffect({
                        name: "flag",
                        before: BoolValue(false),
                        after: BoolValue(true)
                      }))
                }, {
                  index: 4,
                  statement: LetStatement({
                      name: "seen",
                      value: 6
                    }),
                  value: SomeScalar(UIntValue(47)),
                  snapshot: {
                    state: Map("flag" -> BoolValue(true), "marker" -> UIntValue(47)),
                    balances: List({
                        account: "holder-a",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 31
                      }, {
                        account: "holder-b",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        amount: 2
                      }),
                    liabilities: List({
                        name: "duty-open",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 9
                      }, {
                        name: "duty-zero",
                        asset: {
                          network: "local",
                          issuer: "core2-b06-sequence-probe",
                          reference: "UNIT",
                          scale: 2
                        },
                        debtor: "holder-a",
                        creditor: "holder-b",
                        amount: 0
                      })
                  },
                  effects: List()
                }),
              effects: List(StateEffect({
                    name: "marker",
                    before: UIntValue(41),
                    after: UIntValue(47)
                  }), StateEffect({
                    name: "flag",
                    before: BoolValue(false),
                    after: BoolValue(true)
                  }))
            },
            exitSite: 2,
            capturesOut: List({
                name: "delta",
                value: UIntValue(6)
              }, {
                name: "retained",
                value: UIntValue(41)
              }, {
                name: "ready",
                value: BoolValue(true)
              }, {
                name: "seen",
                value: UIntValue(47)
              })
          }),
        attempted: 11
      }
    })
}

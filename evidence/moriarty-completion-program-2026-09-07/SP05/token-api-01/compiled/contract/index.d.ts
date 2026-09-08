import type * as __compactRuntime from '@midnight-ntwrk/compact-runtime';

export type Witnesses<PS> = {
}

export type ImpureCircuits<PS> = {
  mintUnshieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                           domainSep_0: Uint8Array,
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  mintUnshieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                               domainSep_0: Uint8Array,
                               address_0: { bytes: Uint8Array },
                               amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  mintUnshieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                           domainSep_0: Uint8Array,
                           address_0: { bytes: Uint8Array },
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  sendUnshieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array,
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, []>;
  sendUnshieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                               color_0: Uint8Array,
                               amount_0: bigint,
                               address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, []>;
  sendUnshieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array,
                           amount_0: bigint,
                           address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, []>;
  receiveUnshieldedTest(context: __compactRuntime.CircuitContext<PS>,
                        color_0: Uint8Array,
                        amount_0: bigint): __compactRuntime.CircuitResults<PS, []>;
  getUnshieldedBalanceTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array): __compactRuntime.CircuitResults<PS, bigint>;
  getUnshieldedBalanceGtTest(context: __compactRuntime.CircuitContext<PS>,
                             color_0: Uint8Array,
                             amount_0: bigint): __compactRuntime.CircuitResults<PS, boolean>;
  getUnshieldedBalanceLtTest(context: __compactRuntime.CircuitContext<PS>,
                             color_0: Uint8Array,
                             amount_0: bigint): __compactRuntime.CircuitResults<PS, boolean>;
  mintShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                         domainSep_0: Uint8Array,
                         value_0: bigint,
                         nonce_0: Uint8Array): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                     color: Uint8Array,
                                                                                     value: bigint
                                                                                   }>;
  mintShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                             domainSep_0: Uint8Array,
                             value_0: bigint,
                             nonce_0: Uint8Array,
                             address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                                      color: Uint8Array,
                                                                                                      value: bigint
                                                                                                    }>;
  mintShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                         domainSep_0: Uint8Array,
                         value_0: bigint,
                         nonce_0: Uint8Array,
                         publicKey_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }>;
  receiveShieldedTest(context: __compactRuntime.CircuitContext<PS>,
                      coin_0: { nonce: Uint8Array,
                                color: Uint8Array,
                                value: bigint
                              }): __compactRuntime.CircuitResults<PS, []>;
  sendShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                         input_0: { nonce: Uint8Array,
                                    color: Uint8Array,
                                    value: bigint,
                                    mt_index: bigint
                                  },
                         value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                           value: { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }
                                                                                         },
                                                                                 sent: { nonce: Uint8Array,
                                                                                         color: Uint8Array,
                                                                                         value: bigint
                                                                                       }
                                                                               }>;
  sendShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                             input_0: { nonce: Uint8Array,
                                        color: Uint8Array,
                                        value: bigint,
                                        mt_index: bigint
                                      },
                             address_0: { bytes: Uint8Array },
                             value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                               value: { nonce: Uint8Array,
                                                                                                        color: Uint8Array,
                                                                                                        value: bigint
                                                                                                      }
                                                                                             },
                                                                                     sent: { nonce: Uint8Array,
                                                                                             color: Uint8Array,
                                                                                             value: bigint
                                                                                           }
                                                                                   }>;
  sendShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                         input_0: { nonce: Uint8Array,
                                    color: Uint8Array,
                                    value: bigint,
                                    mt_index: bigint
                                  },
                         publicKey_0: { bytes: Uint8Array },
                         value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                           value: { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }
                                                                                         },
                                                                                 sent: { nonce: Uint8Array,
                                                                                         color: Uint8Array,
                                                                                         value: bigint
                                                                                       }
                                                                               }>;
  sendImmediateShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                                  input_0: { nonce: Uint8Array,
                                             color: Uint8Array,
                                             value: bigint
                                           },
                                  value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                    value: { nonce: Uint8Array,
                                                                                                             color: Uint8Array,
                                                                                                             value: bigint
                                                                                                           }
                                                                                                  },
                                                                                          sent: { nonce: Uint8Array,
                                                                                                  color: Uint8Array,
                                                                                                  value: bigint
                                                                                                }
                                                                                        }>;
  sendImmediateShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                                      input_0: { nonce: Uint8Array,
                                                 color: Uint8Array,
                                                 value: bigint
                                               },
                                      address_0: { bytes: Uint8Array },
                                      value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                        value: { nonce: Uint8Array,
                                                                                                                 color: Uint8Array,
                                                                                                                 value: bigint
                                                                                                               }
                                                                                                      },
                                                                                              sent: { nonce: Uint8Array,
                                                                                                      color: Uint8Array,
                                                                                                      value: bigint
                                                                                                    }
                                                                                            }>;
  sendImmediateShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                                  input_0: { nonce: Uint8Array,
                                             color: Uint8Array,
                                             value: bigint
                                           },
                                  publicKey_0: { bytes: Uint8Array },
                                  value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                    value: { nonce: Uint8Array,
                                                                                                             color: Uint8Array,
                                                                                                             value: bigint
                                                                                                           }
                                                                                                  },
                                                                                          sent: { nonce: Uint8Array,
                                                                                                  color: Uint8Array,
                                                                                                  value: bigint
                                                                                                }
                                                                                        }>;
}

export type ProvableCircuits<PS> = {
  mintUnshieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                           domainSep_0: Uint8Array,
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  mintUnshieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                               domainSep_0: Uint8Array,
                               address_0: { bytes: Uint8Array },
                               amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  mintUnshieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                           domainSep_0: Uint8Array,
                           address_0: { bytes: Uint8Array },
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  sendUnshieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array,
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, []>;
  sendUnshieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                               color_0: Uint8Array,
                               amount_0: bigint,
                               address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, []>;
  sendUnshieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array,
                           amount_0: bigint,
                           address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, []>;
  receiveUnshieldedTest(context: __compactRuntime.CircuitContext<PS>,
                        color_0: Uint8Array,
                        amount_0: bigint): __compactRuntime.CircuitResults<PS, []>;
  getUnshieldedBalanceTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array): __compactRuntime.CircuitResults<PS, bigint>;
  getUnshieldedBalanceGtTest(context: __compactRuntime.CircuitContext<PS>,
                             color_0: Uint8Array,
                             amount_0: bigint): __compactRuntime.CircuitResults<PS, boolean>;
  getUnshieldedBalanceLtTest(context: __compactRuntime.CircuitContext<PS>,
                             color_0: Uint8Array,
                             amount_0: bigint): __compactRuntime.CircuitResults<PS, boolean>;
  mintShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                         domainSep_0: Uint8Array,
                         value_0: bigint,
                         nonce_0: Uint8Array): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                     color: Uint8Array,
                                                                                     value: bigint
                                                                                   }>;
  mintShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                             domainSep_0: Uint8Array,
                             value_0: bigint,
                             nonce_0: Uint8Array,
                             address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                                      color: Uint8Array,
                                                                                                      value: bigint
                                                                                                    }>;
  mintShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                         domainSep_0: Uint8Array,
                         value_0: bigint,
                         nonce_0: Uint8Array,
                         publicKey_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }>;
  receiveShieldedTest(context: __compactRuntime.CircuitContext<PS>,
                      coin_0: { nonce: Uint8Array,
                                color: Uint8Array,
                                value: bigint
                              }): __compactRuntime.CircuitResults<PS, []>;
  sendShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                         input_0: { nonce: Uint8Array,
                                    color: Uint8Array,
                                    value: bigint,
                                    mt_index: bigint
                                  },
                         value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                           value: { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }
                                                                                         },
                                                                                 sent: { nonce: Uint8Array,
                                                                                         color: Uint8Array,
                                                                                         value: bigint
                                                                                       }
                                                                               }>;
  sendShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                             input_0: { nonce: Uint8Array,
                                        color: Uint8Array,
                                        value: bigint,
                                        mt_index: bigint
                                      },
                             address_0: { bytes: Uint8Array },
                             value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                               value: { nonce: Uint8Array,
                                                                                                        color: Uint8Array,
                                                                                                        value: bigint
                                                                                                      }
                                                                                             },
                                                                                     sent: { nonce: Uint8Array,
                                                                                             color: Uint8Array,
                                                                                             value: bigint
                                                                                           }
                                                                                   }>;
  sendShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                         input_0: { nonce: Uint8Array,
                                    color: Uint8Array,
                                    value: bigint,
                                    mt_index: bigint
                                  },
                         publicKey_0: { bytes: Uint8Array },
                         value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                           value: { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }
                                                                                         },
                                                                                 sent: { nonce: Uint8Array,
                                                                                         color: Uint8Array,
                                                                                         value: bigint
                                                                                       }
                                                                               }>;
  sendImmediateShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                                  input_0: { nonce: Uint8Array,
                                             color: Uint8Array,
                                             value: bigint
                                           },
                                  value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                    value: { nonce: Uint8Array,
                                                                                                             color: Uint8Array,
                                                                                                             value: bigint
                                                                                                           }
                                                                                                  },
                                                                                          sent: { nonce: Uint8Array,
                                                                                                  color: Uint8Array,
                                                                                                  value: bigint
                                                                                                }
                                                                                        }>;
  sendImmediateShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                                      input_0: { nonce: Uint8Array,
                                                 color: Uint8Array,
                                                 value: bigint
                                               },
                                      address_0: { bytes: Uint8Array },
                                      value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                        value: { nonce: Uint8Array,
                                                                                                                 color: Uint8Array,
                                                                                                                 value: bigint
                                                                                                               }
                                                                                                      },
                                                                                              sent: { nonce: Uint8Array,
                                                                                                      color: Uint8Array,
                                                                                                      value: bigint
                                                                                                    }
                                                                                            }>;
  sendImmediateShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                                  input_0: { nonce: Uint8Array,
                                             color: Uint8Array,
                                             value: bigint
                                           },
                                  publicKey_0: { bytes: Uint8Array },
                                  value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                    value: { nonce: Uint8Array,
                                                                                                             color: Uint8Array,
                                                                                                             value: bigint
                                                                                                           }
                                                                                                  },
                                                                                          sent: { nonce: Uint8Array,
                                                                                                  color: Uint8Array,
                                                                                                  value: bigint
                                                                                                }
                                                                                        }>;
}

export type PureCircuits = {
}

export type Circuits<PS> = {
  mintUnshieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                           domainSep_0: Uint8Array,
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  mintUnshieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                               domainSep_0: Uint8Array,
                               address_0: { bytes: Uint8Array },
                               amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  mintUnshieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                           domainSep_0: Uint8Array,
                           address_0: { bytes: Uint8Array },
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, Uint8Array>;
  sendUnshieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array,
                           amount_0: bigint): __compactRuntime.CircuitResults<PS, []>;
  sendUnshieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                               color_0: Uint8Array,
                               amount_0: bigint,
                               address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, []>;
  sendUnshieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array,
                           amount_0: bigint,
                           address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, []>;
  receiveUnshieldedTest(context: __compactRuntime.CircuitContext<PS>,
                        color_0: Uint8Array,
                        amount_0: bigint): __compactRuntime.CircuitResults<PS, []>;
  getUnshieldedBalanceTest(context: __compactRuntime.CircuitContext<PS>,
                           color_0: Uint8Array): __compactRuntime.CircuitResults<PS, bigint>;
  getUnshieldedBalanceGtTest(context: __compactRuntime.CircuitContext<PS>,
                             color_0: Uint8Array,
                             amount_0: bigint): __compactRuntime.CircuitResults<PS, boolean>;
  getUnshieldedBalanceLtTest(context: __compactRuntime.CircuitContext<PS>,
                             color_0: Uint8Array,
                             amount_0: bigint): __compactRuntime.CircuitResults<PS, boolean>;
  mintShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                         domainSep_0: Uint8Array,
                         value_0: bigint,
                         nonce_0: Uint8Array): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                     color: Uint8Array,
                                                                                     value: bigint
                                                                                   }>;
  mintShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                             domainSep_0: Uint8Array,
                             value_0: bigint,
                             nonce_0: Uint8Array,
                             address_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                                      color: Uint8Array,
                                                                                                      value: bigint
                                                                                                    }>;
  mintShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                         domainSep_0: Uint8Array,
                         value_0: bigint,
                         nonce_0: Uint8Array,
                         publicKey_0: { bytes: Uint8Array }): __compactRuntime.CircuitResults<PS, { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }>;
  receiveShieldedTest(context: __compactRuntime.CircuitContext<PS>,
                      coin_0: { nonce: Uint8Array,
                                color: Uint8Array,
                                value: bigint
                              }): __compactRuntime.CircuitResults<PS, []>;
  sendShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                         input_0: { nonce: Uint8Array,
                                    color: Uint8Array,
                                    value: bigint,
                                    mt_index: bigint
                                  },
                         value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                           value: { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }
                                                                                         },
                                                                                 sent: { nonce: Uint8Array,
                                                                                         color: Uint8Array,
                                                                                         value: bigint
                                                                                       }
                                                                               }>;
  sendShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                             input_0: { nonce: Uint8Array,
                                        color: Uint8Array,
                                        value: bigint,
                                        mt_index: bigint
                                      },
                             address_0: { bytes: Uint8Array },
                             value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                               value: { nonce: Uint8Array,
                                                                                                        color: Uint8Array,
                                                                                                        value: bigint
                                                                                                      }
                                                                                             },
                                                                                     sent: { nonce: Uint8Array,
                                                                                             color: Uint8Array,
                                                                                             value: bigint
                                                                                           }
                                                                                   }>;
  sendShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                         input_0: { nonce: Uint8Array,
                                    color: Uint8Array,
                                    value: bigint,
                                    mt_index: bigint
                                  },
                         publicKey_0: { bytes: Uint8Array },
                         value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                           value: { nonce: Uint8Array,
                                                                                                    color: Uint8Array,
                                                                                                    value: bigint
                                                                                                  }
                                                                                         },
                                                                                 sent: { nonce: Uint8Array,
                                                                                         color: Uint8Array,
                                                                                         value: bigint
                                                                                       }
                                                                               }>;
  sendImmediateShieldedToSelfTest(context: __compactRuntime.CircuitContext<PS>,
                                  input_0: { nonce: Uint8Array,
                                             color: Uint8Array,
                                             value: bigint
                                           },
                                  value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                    value: { nonce: Uint8Array,
                                                                                                             color: Uint8Array,
                                                                                                             value: bigint
                                                                                                           }
                                                                                                  },
                                                                                          sent: { nonce: Uint8Array,
                                                                                                  color: Uint8Array,
                                                                                                  value: bigint
                                                                                                }
                                                                                        }>;
  sendImmediateShieldedToContractTest(context: __compactRuntime.CircuitContext<PS>,
                                      input_0: { nonce: Uint8Array,
                                                 color: Uint8Array,
                                                 value: bigint
                                               },
                                      address_0: { bytes: Uint8Array },
                                      value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                        value: { nonce: Uint8Array,
                                                                                                                 color: Uint8Array,
                                                                                                                 value: bigint
                                                                                                               }
                                                                                                      },
                                                                                              sent: { nonce: Uint8Array,
                                                                                                      color: Uint8Array,
                                                                                                      value: bigint
                                                                                                    }
                                                                                            }>;
  sendImmediateShieldedToUserTest(context: __compactRuntime.CircuitContext<PS>,
                                  input_0: { nonce: Uint8Array,
                                             color: Uint8Array,
                                             value: bigint
                                           },
                                  publicKey_0: { bytes: Uint8Array },
                                  value_0: bigint): __compactRuntime.CircuitResults<PS, { change: { is_some: boolean,
                                                                                                    value: { nonce: Uint8Array,
                                                                                                             color: Uint8Array,
                                                                                                             value: bigint
                                                                                                           }
                                                                                                  },
                                                                                          sent: { nonce: Uint8Array,
                                                                                                  color: Uint8Array,
                                                                                                  value: bigint
                                                                                                }
                                                                                        }>;
}

export type Ledger = {
}

export type ContractReferenceLocations = any;

export declare const contractReferenceLocations : ContractReferenceLocations;

export declare class Contract<PS = any, W extends Witnesses<PS> = Witnesses<PS>> {
  witnesses: W;
  circuits: Circuits<PS>;
  impureCircuits: ImpureCircuits<PS>;
  provableCircuits: ProvableCircuits<PS>;
  constructor(witnesses: W);
  initialState(context: __compactRuntime.ConstructorContext<PS>): __compactRuntime.ConstructorResult<PS>;
}

export declare function ledger(state: __compactRuntime.StateValue | __compactRuntime.ChargedState): Ledger;
export declare const pureCircuits: PureCircuits;

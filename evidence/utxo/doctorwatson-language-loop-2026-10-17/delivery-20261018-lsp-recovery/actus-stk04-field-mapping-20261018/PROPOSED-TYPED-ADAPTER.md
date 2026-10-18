This is a proposed adapter shape for review, not an adopted interpretation or executable implementation.

An exact adapter must distinguish three domains: (1) contract terms, (2) mathematical states at a named time and event side, and (3) common result/export fields that may include schema padding. It must never obtain the expected ACTUS state from a Moriarty initializer or a captured execution.

Required source-side tagged type:

```
ReferenceField =
  Defined { contractType, symbol, value: ExactDecimal,
            dimension: Money(currency) | AnnualRate,
            instant: ISO8601WithoutTimezone, eventSide: Initial | Pre | Post,
            signConvention, normativeRuleRef }
  | NotApplicable { contractType, symbol, applicabilityRuleRef }
  | Undefined { contractType, symbol, reason, normativeRuleRef }
  | CommonSchemaPadding { symbol, encodedValue, exportSchemaRuleRef,
                          underlying: NotApplicable | Undefined }
```

The target descriptor must separately name state key, source/semantics commit, `UInt` representation, currency/scale for money, rate precision/denominator for rates, sign/range policy, exactness or rounding rule, event instant and event side. A generic result-column name is not a sufficient descriptor.

Exact conversion of a Defined amount can be proposed as `atomic = value * 10^scale`, allowed only if the product is an exact nonnegative integer <= u128::MAX and currency, sign and event instant are equal. A Defined annual rate requires its own explicit denominator/precision and annual time basis; asset scale cannot supply it. These are dimensional conversion requirements, not selected parameter values.

Undefined and NotApplicable must produce an explicit Unmapped result unless an independently adopted export-schema rule specifies their exact scalar representation. If a common ACTUS result schema encodes nonapplicable fields as zero, an adapter must label the comparison as schema-padding equivalence; it must not claim an economic principal/rate/accrual state equal to zero. No such rule is present in this inspected packet.

For these three STK04 fields the proposed adapter therefore returns Unmapped with the field-specific blockers in FIELD-MAPPINGS.json. In particular NT=1000 remains a known contract term and is not replaced by the local state0; missing IPNR/IPAC are not defaulted from absence. STK inherits Prf and Sd only, not the complete PAM initialization table. The Core1 source can carry arbitrary named UInt cells, but the source language supplies no unit, annual-rate, applicability or cross-profile meaning for these three names.

Required reviewable follow-up input is an authoritative pinned common-result-field schema/applicability/default rule (including field dictionary state definitions), the agreed event-row time/side, and exact numeric/sign/units mapping. A substantive review would then decide whether these are economic states or typed nonapplicable export placeholders. No outcome of that review is presumed here.

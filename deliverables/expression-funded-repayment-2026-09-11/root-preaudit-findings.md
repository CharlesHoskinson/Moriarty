Root preaudit on initial adapter/CLI candidate:

19 independent runtime probes pass. Actual normal CLI funded command succeeds.
Typecheck fails with TS2345 at adapter line42 (unknown vs ValueType) and TS2322 at line262 (RejectedRepayment vs index-signature rejection). See root-typecheck.txt.

Usability correction requested before final delivery: make the runnable example compute nominalAmount as well as transferAmount from source inputs. Current example locks nominalAmount to literal30 and checks payment==30. The adapter supports Quantity addition and dynamic amount construction; independent-probes.mjs demonstrates this. Prefer source arguments first/second typed Quantity<Units<denomination,1>,0>, let nominal=first+second, reject negative, cash=amount<Cash>(magnitude(nominal)); emit that same nominal to Repay. Show at least two payments through the normal CLI without changing source, with balances/residual debt from full projection. Retain due100/pay30 as default.

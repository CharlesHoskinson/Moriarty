# Persona 03 — Asset issuer

I selected **Capped issuance**, opened **Supply arithmetic**, then followed the case’s missing-work disclosure, developer handoff and Evidence disclosures in a fresh local browser.

The example shows 300 application-asset units issued under a 1,000-unit cap, split 180 and 120, leaving 700 units of unissued capacity. It explicitly says this capacity is not debt. Any redemption duty is separate and must be defined separately.

The page does not identify an actual policy that mints the application asset. The inspector says an issuer policy, exact asset and predecessor need an authenticated path; the missing-work disclosure calls for actual mint-policy enforcement. Evidence names a C2 claim policy that issues proof-claim outputs, while the case says those do not mint the application asset. I recommend renaming that disclosure “Where proof claims are issued and checked” and stating directly that the app-asset mint policy is unspecified here.

The developer handoff gives a conceptual sequence and says it is not a shipping SDK or wallet integration. It names mori-k2 commands documented in a source README but not executed here; proposed Web and transaction command families are not runnable. No issuer-specific mint interface is shown.

I inspected the supplied desktop-top, mobile-top and explorer screenshots, plus my own [capped issuance capture](capped-issuance.png). Action details and answers are in [FEEDBACK.json](FEEDBACK.json) and [navigation.json](navigation.json). The frozen artifact SHA matched the manifest. This is model persona review, not real-user research or security review.

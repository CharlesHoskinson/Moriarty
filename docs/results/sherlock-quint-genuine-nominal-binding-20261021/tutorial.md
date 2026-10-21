# Tutorial: follow a MapQueue result

Start with the existing [nominal model family](../../../models/sherlock/quint/public-nominal-source-cases-20261020/q11NominalIdentity.qnt), its unchanged input module and independently frozen expected returns. MapQueue's logical result was specified before the author transformed a captured payload. The read budget256 comes from that source-first freeze and the earlier model interface; it was not adjusted to an observed root index.

Open [the original MapQueue observation](data/map-queue.observation.json). Its preparation supplies six admission keys including program_view, and its execution supplies all23 image fields and a typed Halted reference. The new typed DATA carries the full program view and image; status/representation remain separate typed strings. No runtime TypeId/CtorId or root index is replaced with a fixture identity.

Read [the inverse receipt](data/map-queue.INVERSE-RESULT.json), then the new `q11ActualMapQueueFinalRootTest` in the test module. It compares the supplied structural root read to the pre-existing independent typed return, requires empty pending at budget256, keeps the entire supplied view/image and checks zero-budget residual tokens/pending. MirrorArbor exercises the same interface with a contrasting nested typed tree and ordered same-typed children.

Finally inspect [the full actual two-test stdout](actual/03-twoExactActualNominalRootTests/stdout.bin): exactly the two named passes, one sample each. This is a bounded final-value result. The retained physical image and counters were not independently predicted, and no complete ordered Core event trace is present.

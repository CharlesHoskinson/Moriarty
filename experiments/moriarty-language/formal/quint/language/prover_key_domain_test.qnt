module prover_key_domain_test {
  import prover_key_domain.* from "./prover_key_domain"
  // Pure real-count boundaries: no byte vectors or file acquisition.
  pure val U64_MAX = 18446744073709551615
  pure val MIB = 1048576
  pure def realProfileValid(profile: Profile, signed: bool, leaf: int, public: int,
      pk: int, input: int, vk: int): bool = and {
    Set(leaf, public, pk, input, vk).forall(n => n > 0 and n <= U64_MAX),
    if (profile == V2) and { leaf <= 16*MIB, public <= 16*MIB, pk <= 64*MIB,
      input <= MIB, vk <= 65536 } else if (signed) public <= 16*MIB else true,
  }
  pure def countsAccept(profile: Profile, keyset: bool, leafCap: int, publicCap: int,
      pkCap: int, public: int, pk: int, max: int): bool = and {
    public >= 0 and public <= publicCap,
    pk >= 0,
    checkedAdd(public, pk, max),
    if (profile == V2 and keyset) pk <= pkCap else and {
      pk <= min(pkCap, leafCap), public + pk <= publicCap },
  }
  run coldWitnessTest = init.then(begin(request)).then(acquirePublic).then(coldSnapshot)
    .expect(coldComplete and op.publicBytes == 5 and op.pkBytes == 0 and safe)
  run provingCheckWitnessTest = init.then(begin({...request, reader: Proving})).then(acquirePublic)
    .then(checkPkDeclaration).expect(pkDeclarationChecked and not(op.accessed.contains(SelectedPk)))
  run legacyRefusalTest = init.then(refuseAdmission({...request, profile: V1, reader: Export}))
    .expect(refused and op.refusal == "pk-declaration")
  run coldHugeOptionalTest = init.then(begin({...request, fixture: {...fixture, declaredPk: 31,
      actualPk: 0, declaredHash: 999, provingMarker: 31}})).then(acquirePublic).then(coldSnapshot)
    .expect(coldComplete and coldNoOptionalAccess and op.publicBytes == 5)
  run overflowPureTest = init.expect(not(checkedAdd(16, 16, 31)) and checkedAdd(15, 16, 31))
  run provingSnapshotWitnessTest = init.then(begin({...request, reader: Proving})).then(acquirePublic)
    .then(checkPkDeclaration).then(acquirePk).then(provingSnapshot)
    .expect(provingComplete and safe and op.publicBytes == 6 and op.pkBytes == 12 and op.physicalBytes == 18)
  run publishWitnessTest = init.then(begin({...request, reader: Export})).then(startWrite).then(writeLeaves)
    .then(writeProvingMarker).then(publishPublicMarker)
    .expect(published and safe and op.publicBytes == 6 and op.pkBytes == 12 and op.physicalBytes == 18)
  run writerLeafFailureTest = init.then(begin({...request, reader: Export})).then(startWrite).then(refuseLeafWrite)
    .expect(failedLeafWrite and markerLast)
  run writerProvingMarkerFailureTest = init.then(begin({...request, reader: Export})).then(startWrite)
    .then(writeLeaves).then(refuseProvingMarkerWrite).expect(failedProvingMarker and markerLast)
  run writerPublicMarkerFailureTest = init.then(begin({...request, reader: Export})).then(startWrite)
    .then(writeLeaves).then(writeProvingMarker).then(refusePublicMarkerWrite)
    .expect(failedPublicMarker and markerLast and op.publicBytes == 5 and op.pkBytes == 12)
  run publicBudgetBeforeAccessTest = init.then(refuseAdmission({...request, reader: Proving,
      limits: {...limits, public: 5}})).expect(op.refusal == "public" and op.accessed == Set())
  run pkBoundBeforeAccessTest = init.then(begin({...request, reader: Proving,
      fixture: {...fixture, declaredPk: 17}})).then(acquirePublic).then(refusePkDeclaration)
    .expect(op.refusal == "pk-declaration" and not(op.accessed.contains(SelectedPk)))
  run markerAgreementBeforePkTest = init.then(begin({...request, reader: Proving,
      fixture: {...fixture, provingIdentity: 10}})).then(acquirePublic).then(refuseMarkerAgreement)
    .expect(op.refusal == "marker-agreement" and not(op.accessed.contains(SelectedPk)))
  run pkShortTest = init.then(begin({...request, reader: Proving, fixture: {...fixture, actualPk: 11}}))
    .then(acquirePublic).then(checkPkDeclaration).then(acquirePk).then(refusePkLength)
    .expect(op.refusal == "pk-length" and safe)
  run pkLongTest = init.then(begin({...request, reader: Proving, fixture: {...fixture, actualPk: 13}}))
    .then(acquirePublic).then(checkPkDeclaration).then(acquirePk).then(refusePkLength)
    .expect(op.refusal == "pk-length" and safe)
  run pkChangedHashTest = init.then(begin({...request, reader: Proving, fixture: {...fixture, actualHash: 8}}))
    .then(acquirePublic).then(checkPkDeclaration).then(acquirePk).then(refusePkHash)
    .expect(op.refusal == "pk-hash" and safe)
  run genericPkFilenameNoExceptionTest = init.then(refuseAdmission({...request, purpose: GenericPackage,
      reader: Export, fixture: {...fixture, keyName: Lookalike}})).expect(op.refusal == "public" and safe)
  run extraPkKeysetRefusedTest = init.then(refuseAdmission({...request, reader: Export,
      fixture: {...fixture, pkCount: 2}})).expect(op.refusal == "pk-declaration")
  run renamedPkRefusedTest = init.then(refuseAdmission({...request, reader: Export,
      fixture: {...fixture, keyName: Lookalike}})).expect(op.refusal == "pk-declaration")
  run realVersionCountBoundaryTest = init.expect(and {
    countsAccept(V2, true, 16*MIB, 16*MIB, 32*MIB, 16*MIB, 32*MIB, U64_MAX),
    not(countsAccept(V1, true, 16*MIB, 16*MIB, 32*MIB, 16*MIB, 32*MIB, U64_MAX)),
    not(countsAccept(V2, true, 16*MIB, 16*MIB, 32*MIB, 16*MIB+1, 32*MIB, U64_MAX)),
    not(countsAccept(V2, true, 16*MIB, 16*MIB, 32*MIB, 16*MIB, 32*MIB+1, U64_MAX)),
    countsAccept(V2, true, 16*MIB, 16*MIB, 64*MIB, 16*MIB, 64*MIB, U64_MAX),
    not(countsAccept(V2, true, 16*MIB, 16*MIB, 64*MIB, 16*MIB, 64*MIB+1, U64_MAX)),
    checkedAdd(16*MIB, 64*MIB, U64_MAX), 16*MIB + 64*MIB == 80*MIB,
    checkedAdd(U64_MAX, 0, U64_MAX), not(checkedAdd(U64_MAX, 1, U64_MAX)),
  })
  run realProfileBoundaryTest = init.expect(and {
    realProfileValid(V2, false, 16*MIB, 16*MIB, 64*MIB, MIB, 65536),
    not(realProfileValid(V2, false, 16*MIB+1, 16*MIB, 64*MIB, MIB, 65536)),
    not(realProfileValid(V2, false, 16*MIB, 16*MIB+1, 64*MIB, MIB, 65536)),
    not(realProfileValid(V2, false, 16*MIB, 16*MIB, 64*MIB+1, MIB, 65536)),
    not(realProfileValid(V2, false, 16*MIB, 16*MIB, 64*MIB, MIB+1, 65536)),
    not(realProfileValid(V2, false, 16*MIB, 16*MIB, 64*MIB, MIB, 65537)),
    realProfileValid(V1, false, 16*MIB+1, 16*MIB+1, 64*MIB+1, MIB+1, 65537),
    not(realProfileValid(V1, true, 16*MIB, 16*MIB+1, 64*MIB, MIB, 65536)),
    not(realProfileValid(V2, false, 0, 16*MIB, 64*MIB, MIB, 65536)),
    not(realProfileValid(V2, false, -1, 16*MIB, 64*MIB, MIB, 65536)),
    not(realProfileValid(V1, false, U64_MAX+1, 16*MIB, 64*MIB, MIB, 65536)),
  })
  run exactScaledCapsTest = init.then(begin({...request, reader: Export,
      limits: {...limits, public: 6, pk: 12}})).then(startWrite).then(writeLeaves)
    .then(writeProvingMarker).then(publishPublicMarker)
    .expect(op.publicBytes == 6 and op.pkBytes == 12 and op.physicalBytes == 18 and safe)
  run genericSmallPackageTest = init.then(begin({...request, purpose: GenericPackage, reader: Export,
      fixture: {...fixture, declaredPk: 1, actualPk: 1}})).then(startWrite).then(writeLeaves)
    .then(writeProvingMarker).then(publishPublicMarker)
    .expect(op.publicBytes == 7 and op.pkBytes == 0 and safe)
  run boundedPublicMarkerTest = init.then(refuseAdmission({...request,
      fixture: {...fixture, publicMarker: 5}})).expect(op.refusal == "public" and op.accessed == Set())
  run publicProvingMarkerBothChargedTest = init.then(refuseAdmission({...request, reader: Export,
      limits: {...limits, public: 5}})).expect(op.refusal == "public")
  run legacyUnsignedLargeCapsTest = init.expect(profileValid({...request, profile: V1,
    limits: {...limits, leaf: 30, public: 30, pk: 30, input: 30, vk: 30}}))
  run legacySignedEnvelopeTest = init.expect(not(profileValid({...request, profile: V1, signed: true,
    limits: {...limits, leaf: 30, public: 30, pk: 30}})))
}

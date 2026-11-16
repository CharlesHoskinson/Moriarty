// Unit tests of denseStark.qnt: the honest walk, flush and cold read are accepted and sound.
module denseStark_test {
  import denseStark(MUTANT = "none").* from "./denseStark"
  pure val st = { recs: List({kind:1,v:0},{kind:2,v:1},{kind:1,v:1},{kind:1,v:1},{kind:2,v:0}), extraSlot: 4, extraD: 0 }
  run hashDefinedTest = assert(H(1,0,1,1) != 0)
  run segDigestDefinedTest = assert(segD(List(PAD, PAD)) != 0)
  run rootDefinedTest = assert(rootOf(st) != 0)
  run honestReadRootIsPreRootTest = assert(honestRead(st, 1).root == rootOf(st))
  run honestWalkReachesRootTest = assert(walk(recD(st.recs[1]), 0, honestRead(st,1).bits, honestRead(st,1).sibs) == rootOf(st))
  pure val sl = slotsOf(st)
  pure val so = range(0, TOP).foldl(List(), (acc, l) => acc.append(sibAt(levelsFrom(sl, SEGB, TOP), l, 2)))
  pure val lf0 = { imported: tailRecs(st), apps: List({kind:1,v:1}), f: 1, postPad: PAD, x0: EMPTY, sibOld: so, sibNew: so }
  run honestFlushAcceptedTest = assert(leafAccepts("none", st, lf0))
  run honestFlushSoundTest = assert(leafSound(st, lf0))
  run honestColdReadAcceptedTest = assert(readAccepts("none", st, canonPost(st, lf0), honestRead(st, 3)))
}

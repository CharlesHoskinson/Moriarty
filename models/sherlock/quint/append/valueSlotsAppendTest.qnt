// Literal normal expectations authored before transition source. All UNRUN.
module valueSlotsAppendTest {
  import valueSlotsAppend.* from "./valueSlotsAppend"
  pure val a = Constant(1)
  pure val x = Data(0)
  pure val g = Data(1)
  pure val c = Constant(0)
  pure val empty: Slice = {start: 0, length: 0}
  pure val e0: Arena = {capacity: 9, active: 4,
    slots: List(c,x,a,g,c,c,c,c,c), holders: List(empty,empty)}
  pure val e1: Arena = {capacity: 9, active: 5,
    slots: List(c,x,a,g,a,c,c,c,c), holders: List({start:4,length:1},empty)}
  pure val e2: Arena = {capacity: 9, active: 6,
    slots: List(c,x,a,g,a,x,c,c,c), holders: List({start:4,length:1},{start:5,length:1})}
  pure val e3: Arena = {capacity: 9, active: 8,
    slots: List(c,x,a,g,a,x,a,g,c), holders: List({start:6,length:2},{start:5,length:1})}
  pure val e4: Arena = {capacity: 9, active: 9,
    slots: List(c,x,a,g,a,x,a,g,c), holders: List({start:6,length:3},{start:5,length:1})}

  run emptyCopyTest = initArena(e0).then(appendTo(0,a))
    .expect(s.arena == e1 and s.event == Applied({holder:0,value:a,mode:COPY}) and modelInv)

  run exactTailBoundaryTest = initArena(e3).then(appendTo(0,c))
    .expect(s.arena == e4 and s.event == Applied({holder:0,value:c,mode:EXT}) and modelInv)

  run nonTailCopyTest = initArena(e2).then(appendTo(0,g))
    .expect(s.arena == e3 and s.event == Applied({holder:0,value:g,mode:COPY}) and modelInv)

  run ownerEx4Test = initArena(e0)
    .then(appendTo(0,a)).expect(s.arena == e1 and modelInv)
    .then(appendTo(1,x)).expect(s.arena == e2 and modelInv)
    .then(appendTo(0,g)).expect(s.arena == e3 and modelInv)
    .then(appendTo(0,c)).expect(s.arena == e4 and modelInv
      and materialize(s.arena,s.arena.holders.nth(0)) == List(a,g,c)
      and materialize(s.arena,{start:4,length:1}) == List(a)
      and materialize(s.arena,s.arena.holders.nth(1)) == List(x))

  pure val shared0: Arena = {capacity:9,active:2,
    slots:List(a,x,c,c,c,c,c,c,c),holders:List({start:0,length:2},{start:0,length:2})}
  pure val shared1: Arena = {capacity:9,active:3,
    slots:List(a,x,g,c,c,c,c,c,c),holders:List({start:0,length:3},{start:0,length:2})}
  pure val shared2: Arena = {capacity:9,active:6,
    slots:List(a,x,g,a,x,c,c,c,c),holders:List({start:0,length:3},{start:3,length:3})}
  run sharedSliceForcesCopyTest = initArena(shared0)
    .then(appendTo(0,g)).expect(s.arena == shared1 and modelInv
      and s.event == Applied({holder:0,value:g,mode:EXT})
      and materialize(s.arena,s.arena.holders.nth(1)) == List(a,x))
    .then(appendTo(1,c)).expect(s.arena == shared2 and modelInv
      and s.event == Applied({holder:1,value:c,mode:COPY})
      and materialize(s.arena,s.arena.holders.nth(0)) == List(a,x,g)
      and materialize(s.arena,s.arena.holders.nth(1)) == List(a,x,c))

  pure val limit8: Arena = {capacity:8,active:8,
    slots:List(c,x,a,g,a,x,a,g),holders:List({start:6,length:2},{start:5,length:1})}
  run tailCapacityRefusalTest = initArena(limit8).then(appendTo(0,c))
    .expect(s.arena == limit8 and s.event == ToolingCapacity({holder:0,value:c,
      required:9,limit:8,firstUnavailable:8}) and modelInv)

  pure val limit7: Arena = {capacity:7,active:6,
    slots:List(c,x,a,g,a,x,c),holders:List({start:4,length:1},{start:5,length:1})}
  run copyCapacityRefusalTest = initArena(limit7).then(appendTo(0,g))
    .expect(s.arena == limit7 and s.event == ToolingCapacity({holder:0,value:g,
      required:8,limit:7,firstUnavailable:7}) and modelInv)

  pure val fullEmpty: Arena = {capacity:9,active:9,
    slots:List(c,x,a,g,a,x,a,g,c),holders:List(empty,{start:5,length:1})}
  run emptyCapacityRefusalTest = initArena(fullEmpty).then(appendTo(0,a))
    .expect(s.arena == fullEmpty and s.event == ToolingCapacity({holder:0,value:a,
      required:10,limit:9,firstUnavailable:9}) and modelInv)
}

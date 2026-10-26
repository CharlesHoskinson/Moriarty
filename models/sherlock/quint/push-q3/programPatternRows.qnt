// 388b92d9b: c6_program_context_canonical.rs::step/var_step uses org.hold
// for row width even when sub == 0 and the new tag is only in nxt.hold.
// c6_phys_program.rs encodes PNil and Wildcard in one lane (tags 1 and 5).
// The useCurrentTag=true variant matches d570c75cc's committed repair in
// both the program-context and duplicated validate-field walk.
// Segment projection only: no full PROG circuit/native execution is claimed.
module programPatternRows {
  type Parser = {hold: int, sub: int, remaining: int, doneAt: int}
  type Scan = {tags: List[int], lanes: List[int], previousTag: int, cursor: int,
    source: Parser, repaired: Parser}
  var scan: Scan
  pure val tags = 0.to(6)
  pure val widths = List(2, 1, 3, 2, 3, 1, 5)
  // The owner codec's actual row shapes, with each usize payload set to zero.
  pure val encodings = List(List(0, 0), List(1), List(2, 0, 0), List(3, 0),
    List(4, 0, 0), List(5), List(6, 0, 0, 0, 0))
  pure def width(held: int): int = if (tags.contains(held)) widths.nth(held) else 2
  pure def encoding(rows: List[int]): List[int] = rows.foldl(List(), (allLanes, tag) => allLanes.concat(encodings.nth(tag)))
  pure def start(rows: List[int], previousTag: int): Scan = {
    val parser = {hold: previousTag, sub: 0, remaining: rows.length(), doneAt: -1}
    {tags: rows, lanes: encoding(rows), previousTag: previousTag, cursor: 0, source: parser, repaired: parser}
  }
  pure def consume(parser: Parser, lane: int, cursor: int, useCurrentTag: bool): Parser =
    if (parser.remaining == 0) parser else {
      val held = if (useCurrentTag and parser.sub == 0) lane else parser.hold
      val last = parser.sub == width(held) - 1
      val remaining = if (last) parser.remaining - 1 else parser.remaining
      {hold: if (parser.sub == 0) lane else parser.hold,
        sub: if (last) 0 else parser.sub + 1, remaining: remaining,
        doneAt: if (remaining == 0) cursor + 1 else -1}
    }
  pure def prefix(rows: List[int], count: int): int =
    0.to(count - 1).fold(0, (total, ix) => total + encodings.nth(rows.nth(ix)).length())
  pure def completed(s: Scan): int = s.tags.indices().filter(ix => prefix(s.tags, ix + 1) <= s.cursor).size()
  pure def agrees(s: Scan, parser: Parser): bool = {
    val done = completed(s)
    parser.remaining == s.tags.length() - done and parser.sub == s.cursor - prefix(s.tags, done)
      and parser.doneAt == (if (done == s.tags.length()) s.lanes.length() else -1)
  }
  action begin(rows: List[int], previousTag: int): bool = scan' = start(rows, previousTag)
  action init = {
    nondet first = tags.oneOf()
    nondet second = tags.oneOf()
    nondet rowCount = Set(1, 2).oneOf()
    nondet previous = 0.to(8).oneOf()
    begin(if (rowCount == 1) List(first) else List(first, second), previous)
  }
  action step = all {
    scan.cursor < scan.lanes.length(),
    scan' = {...scan, cursor: scan.cursor + 1,
      source: consume(scan.source, scan.lanes.nth(scan.cursor), scan.cursor, false),
      repaired: consume(scan.repaired, scan.lanes.nth(scan.cursor), scan.cursor, true)},
  }
  val sourceMatchesCodec = agrees(scan, scan.source)
  val modelInv = agrees(scan, scan.repaired) and and {
    scan.cursor >= 0, scan.cursor <= scan.lanes.length(),
    scan.source.remaining >= 0, scan.source.remaining <= scan.tags.length(),
    scan.source.sub >= 0, scan.source.sub <= scan.cursor,
  }
  val finished = scan.cursor == scan.lanes.length()
  val sourceLate = finished and scan.source.remaining > 0
  val sourceEarly = scan.source.doneAt >= 0 and scan.source.doneAt < scan.lanes.length()
  val repairedUnitRow = finished and scan.tags.indices().exists(ix => scan.tags.nth(ix) == 1 or scan.tags.nth(ix) == 5)
    and agrees(scan, scan.repaired)
}

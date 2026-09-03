#!/usr/bin/env node
/** Reproduce DeFiFormal pair failures with exact eligible-pair denominators. */

import fs from "node:fs";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const MORIARTY_ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const DEFIFORMAL_ROOT = process.env.DEFIFORMAL_ROOT ?? "/home/charl/defiformal";
process.env.DEFIFORMAL_ROOT = DEFIFORMAL_ROOT;
const tables = await import(
  pathToFileURL(path.join(DEFIFORMAL_ROOT, "formal", "v2", "tables.mjs")).href
);
const { ROOT, PARSED_NEW, MECH, CONSUME, bansCond, ungrounded, armedListed } = tables;

const mechanismSet = new Set(MECH);
const selectMechanisms = elements => new Set(elements.filter(element => mechanismSet.has(element)));
const satisfiesLaws = elements => PARSED_NEW.every(law =>
  !law.subjects.some(subject => elements.has(subject))
  || law.terms.every(term => term.external || term.alts.some(alt => elements.has(alt)))
);
const isWarranted = elements => [...elements].every(
  element => !CONSUME[element] || CONSUME[element].some(consumer => elements.has(consumer))
);

const protocols = [];
for (const filename of fs.readdirSync(path.join(ROOT, "corpus50", "lanes")).sort()) {
  const lane = JSON.parse(
    fs.readFileSync(path.join(ROOT, "corpus50", "lanes", filename), "utf8")
  );
  for (const category of lane.categories) {
    for (const protocol of category.protocols) {
      protocols.push({
        name: protocol.name,
        category: category.category,
        elements: selectMechanisms(protocol.elements),
      });
    }
  }
}
const eligible = protocols.filter(
  protocol => satisfiesLaws(protocol.elements) && isWarranted(protocol.elements)
);
const eligibleCategoryCounts = Object.fromEntries(
  [...new Set(eligible.map(protocol => protocol.category))].sort().map(category => [
    category,
    eligible.filter(protocol => protocol.category === category).length,
  ])
);

let withinPairs = 0;
let crossPairs = 0;
let withinFailures = 0;
let crossFailures = 0;
for (let left = 0; left < eligible.length; left += 1) {
  for (let right = left + 1; right < eligible.length; right += 1) {
    const first = eligible[left];
    const second = eligible[right];
    const union = new Set([...first.elements, ...second.elements]);
    const failed = !satisfiesLaws(union)
      || !isWarranted(union)
      || ungrounded(union)
      || bansCond(union).length > 0
      || armedListed(union).length > 0;
    if (first.category === second.category) {
      withinPairs += 1;
      withinFailures += Number(failed);
    } else {
      crossPairs += 1;
      crossFailures += Number(failed);
    }
  }
}

const rate = (numerator, denominator) => Number((numerator / denominator).toFixed(6));
const withinRate = withinFailures / withinPairs;
const crossRate = crossFailures / crossPairs;
const result = {
  schema_version: 1,
  source_repository: "https://github.com/CharlesHoskinson/defiformal.git",
  source_commit: execFileSync("git", ["-C", DEFIFORMAL_ROOT, "rev-parse", "HEAD"], {
    encoding: "utf8",
  }).trim(),
  source_dirty: execFileSync("git", ["-C", DEFIFORMAL_ROOT, "status", "--porcelain"], {
    encoding: "utf8",
  }).trim().length > 0,
  eligible_protocols: eligible.length,
  eligible_category_counts: eligibleCategoryCounts,
  pairs: {
    total: withinPairs + crossPairs,
    within_category: withinPairs,
    cross_category: crossPairs,
  },
  failures: {
    total: withinFailures + crossFailures,
    within_category: withinFailures,
    cross_category: crossFailures,
  },
  failure_rates: {
    within_category: rate(withinFailures, withinPairs),
    cross_category: rate(crossFailures, crossPairs),
    cross_to_within_ratio: Number((crossRate / withinRate).toFixed(6)),
  },
  qualification: (
    "The denominator contains only the 61 protocols that satisfy DeFiFormal's laws and " +
    "warrants. Failure-rate association supports grouping but does not identify the best labels."
  ),
};

fs.writeFileSync(
  path.join(MORIARTY_ROOT, "evidence", "defiformal-composition-rates-2026-09-02.json"),
  `${JSON.stringify(result, null, 2)}\n`,
  "utf8",
);
process.stdout.write(`${JSON.stringify(result)}\n`);

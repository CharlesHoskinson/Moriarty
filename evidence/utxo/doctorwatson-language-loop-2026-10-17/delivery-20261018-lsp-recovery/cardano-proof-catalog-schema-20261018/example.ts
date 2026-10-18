/** Unapplied integration example: caller supplies pinned public JSON text; no I/O. */
import {loadForDisplay, selectForDisplay, toWebsDiagramScenario, displayCard} from './catalogue';
import type {SourceLocation} from './catalogue';

export const exampleIds = ['MCP-PLN-01@2', 'MCP-PLN-02@2'] as const;

export function createExample(catalogueJsonText: string, sourceLocation: SourceLocation) {
  const catalogue = loadForDisplay(catalogueJsonText, sourceLocation);
  const entries = selectForDisplay(catalogue, exampleIds);
  return {
    schema: 'moriarty-cardano-catalogue-example-proposal/1',
    status: 'unapplied; expected-display-data, not runtime-observation',
    entryIds: entries.map(entry => entry.identity.versionedId),
    scenario: toWebsDiagramScenario(entries),
    display: entries.map(displayCard),
  };
}

/* Proposed Webs use after normal integration review:
 * const example = createExample(pinnedCatalogueText, pinnedSourceLocation);
 * <WebDiagram scenario={example.scenario} selectedStrand={selected}
 *   onSelect={setSelected} omittedStrands={[]} />
 * Render example.display in a separate metadata inspector using normal text nodes.
 * sourceStatus remains visible; no proof-satisfied/settled badge is derived.
 * Full App integration needs its own supported scenario contract. This proposal
 * supplies only the fields actually read by WebDiagram.
 */

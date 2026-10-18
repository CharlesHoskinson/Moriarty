/** Unapplied, ordinary display proposal. No verifier, policy, proof or ledger API. */
export interface SourceLocation {
  readonly commit: string | null;
  readonly path: string;
  readonly sha256: string;
  readonly jsonPointer?: string;
}

/** Values and constraints are source declarations. This module never enforces them. */
export interface SourceField {
  readonly name: string;
  readonly type: string;
  readonly source: string;
  readonly context_item?: string;
  readonly digest_of?: string;
  readonly index_of?: string;
  readonly role?: string;
  readonly tx_relation?: string;
  readonly min?: string;
  readonly max?: string;
  readonly value?: string;
  readonly note?: string;
  readonly [sourceAttribute: string]: unknown;
}
export interface SourceStatementSchema {
  readonly encoding: string;
  readonly fields: readonly SourceField[];
  readonly binding_constraints: readonly {
    readonly id: string;
    readonly fields: readonly string[];
    readonly constraint: string;
  }[];
  readonly note?: string;
  readonly [sourceAttribute: string]: unknown;
}
export interface RetainedRow {
  readonly id: string;
  readonly version: string;
  readonly versionedId: string;
  readonly family: string;
  readonly title: string;
  readonly property: string;
  readonly claimKind: string;
  readonly temporalScope: string;
  readonly sources: readonly string[];
  readonly publicInputs: string;
  readonly privateWitness: string;
  readonly anchorAndAssumptions: string;
  readonly freshnessValidity: string;
  readonly implementationEvidence: string;
  readonly status: string;
  readonly category: string;
  readonly firstBlocker: string;
  readonly notEstablished: string;
  readonly supersedes?: string;
  readonly statementSchema?: SourceStatementSchema;
  readonly semanticsId?: string;
  readonly anchorType?: string;
  readonly resultSchema?: string;
  readonly auditRef?: string;
  readonly [sourceAttribute: string]: unknown;
}
export interface RetainedCatalogue {
  readonly catalogueId: string;
  readonly catalogueVersion: string;
  readonly profiles: Readonly<Record<string, unknown>>;
  readonly rows: readonly RetainedRow[];
}
export interface CatalogEntry {
  readonly identity: {
    readonly catalogueId: string;
    readonly catalogueVersion: string;
    readonly id: string;
    readonly version: string;
    readonly versionedId: string;
    readonly family: string;
    readonly sourceHead: boolean;
    readonly supersedes: string | null;
    readonly predicateDigest: null;
    readonly predicateDigestStatus: string;
  };
  readonly title: string;
  readonly claimKind: string;
  readonly property: string;
  readonly source: SourceLocation;
  readonly inputs: {
    readonly publicDescription: string;
    readonly privateWitnessDescription: string;
    readonly typedInputSchemaStatus: string;
    readonly typedInputFields: null;
  };
  readonly outputs: {
    readonly description: string | null;
    readonly schemaStatus: string;
    readonly typedFields: null;
  };
  readonly publicStatement: {
    readonly status: string;
    readonly description: string;
    readonly schema: SourceStatementSchema | null;
  };
  readonly profile: {
    readonly selected: null;
    readonly status: string;
    readonly sourceSemanticsId: string | null;
    readonly catalogueProfileNames: readonly string[];
  };
  readonly target: {
    readonly selected: null;
    readonly status: string;
    readonly sourceAnchorType: string | null;
    readonly sourceTemporalScope: string;
    readonly sourceFreshnessValidity: string;
  };
  readonly evidence: {
    readonly sourceStatus: string;
    readonly sourceCategory: string;
    readonly implementationDescription: string;
    readonly sourceRefs: readonly string[];
    readonly sourceAuditRef: string | null;
    readonly acceptance: string;
  };
  readonly assumptions: {
    readonly sourceDescription: string;
    readonly assessment: string;
  };
  readonly setupIdentity: {
    readonly setupClass: null;
    readonly verificationKeyDigest: null;
    readonly admissionRecord: null;
    readonly status: string;
    readonly note: string;
  };
  readonly limits: {
    readonly notEstablished: string;
    readonly firstBlocker: string;
    readonly ownerDependencies: readonly string[];
  };
  readonly sourceRow: RetainedRow;
}
export interface CatalogView {
  readonly schema: 'moriarty-cardano-catalogue-display-proposal/1';
  readonly status: 'unapplied-source-proposal';
  readonly catalogueSource: SourceLocation;
  readonly catalogueId: string;
  readonly catalogueVersion: string;
  readonly coverage: {
    readonly versionedRows: number;
    readonly sourceHeads: number;
    readonly sourceTypedRows: number;
    readonly scope: string;
  };
  readonly ownerDependencies: readonly string[];
  readonly entries: readonly CatalogEntry[];
}
const ownerDependencies = [
  'Cryptographic/security interpretation: designated Opus owner',
  'Trusted-setup and key/relation identity judgments: designated Opus owner',
  'Statement binding, acceptance and validation rules: designated semantic/security owner',
  'Target qualification, ledger facts and deployment observations: designated target owner',
] as const;

/** Source projection only: preserves all versions and never computes predicate digests. */
export function projectCatalogue(source: RetainedCatalogue, location: SourceLocation): CatalogView {
  const superseded = new Set(source.rows.flatMap(row => row.supersedes ? [row.supersedes] : []));
  const entries: CatalogEntry[] = source.rows.map((row, index) => ({
    identity: {
      catalogueId: source.catalogueId, catalogueVersion: source.catalogueVersion,
      id: row.id, version: row.version, versionedId: row.versionedId, family: row.family,
      sourceHead: !superseded.has(row.versionedId), supersedes: row.supersedes ?? null,
      predicateDigest: null,
      predicateDigestStatus: 'not-imported; do not substitute source file SHA-256',
    },
    title: row.title, claimKind: row.claimKind, property: row.property,
    source: {...location, jsonPointer: `/rows/${index}`},
    inputs: {
      publicDescription: row.publicInputs, privateWitnessDescription: row.privateWitness,
      typedInputSchemaStatus: 'not-separately-defined-in-source; public statement fields shown separately',
      typedInputFields: null,
    },
    outputs: {
      description: row.resultSchema ?? null,
      schemaStatus: row.resultSchema === undefined ? 'not-declared-in-source' : 'source-result-description',
      typedFields: null,
    },
    publicStatement: {
      status: row.statementSchema === undefined ? 'source-prose-only' : 'source-declared-typed-fields',
      description: row.publicInputs, schema: row.statementSchema ?? null,
    },
    profile: {
      selected: null, status: 'not-selected-by-display; owner mapping not supplied to this adapter',
      sourceSemanticsId: row.semanticsId ?? null, catalogueProfileNames: Object.keys(source.profiles),
    },
    target: {
      selected: null, status: 'not-selected-by-display; owner mapping not supplied to this adapter',
      sourceAnchorType: row.anchorType ?? null,
      sourceTemporalScope: row.temporalScope, sourceFreshnessValidity: row.freshnessValidity,
    },
    evidence: {
      sourceStatus: row.status, sourceCategory: row.category,
      implementationDescription: row.implementationEvidence,
      sourceRefs: row.sources, sourceAuditRef: row.auditRef ?? null,
      acceptance: 'owner-dependent; no judgment supplied',
    },
    assumptions: {sourceDescription: row.anchorAndAssumptions, assessment: 'owner-unassessed'},
    setupIdentity: {
      setupClass: null, verificationKeyDigest: null, admissionRecord: null,
      status: 'not-supplied-to-display; no assessment',
      note: 'Catalogue SemanticEntry excludes setup/VK/admission identity; no profile or setup-class selection is inferred.',
    },
    limits: {notEstablished: row.notEstablished, firstBlocker: row.firstBlocker, ownerDependencies},
    sourceRow: row,
  }));
  return {
    schema: 'moriarty-cardano-catalogue-display-proposal/1', status: 'unapplied-source-proposal',
    catalogueSource: location, catalogueId: source.catalogueId, catalogueVersion: source.catalogueVersion,
    coverage: {
      versionedRows: entries.length, sourceHeads: entries.filter(entry => entry.identity.sourceHead).length,
      sourceTypedRows: entries.filter(entry => entry.publicStatement.schema !== null).length,
      scope: 'All retained rows from the supplied catalogue. Counts describe source membership, not accepted property coverage. CAP candidates and other inventories remain independent.',
    },
    ownerDependencies, entries,
  };
}

/**
 * Concrete loader proposal for the pinned retained JSON only.
 * JSON parsing/type assertion is not shape validation or evidence authentication.
 * No schema validator, cryptographic check or acceptance rule is installed here.
 */
export function loadForDisplay(jsonText: string, location: SourceLocation): CatalogView {
  return projectCatalogue(JSON.parse(jsonText) as RetainedCatalogue, location);
}


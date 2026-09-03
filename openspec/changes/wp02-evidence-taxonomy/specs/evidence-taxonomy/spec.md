# Evidence and taxonomy specification

## ADDED Requirements

### Requirement: pinned populations

The dataset SHALL distinguish all 72 decomposed protocols from the 60
constructed protocols. Every row SHALL identify the deployed product unit.

#### Scenario: one organization has several products

- WHEN Circle, Maple, Binance, or another organization has multiple products
- THEN each deployed product receives a separate row and stable identifier.

### Requirement: reproducible metrics

The package SHALL calculate Jaccard, nearest-neighbor, clustering, ARI, NMI,
coverage, and composition metrics from pinned inputs.
It SHALL pin algorithms, library versions, input order, ties, precision, and
reference labels before calculation.

#### Scenario: a metric has implementation sensitivity

- WHEN two accepted algorithms produce different rounded values
- THEN the report preserves raw values and records the method as a contradiction.

### Requirement: complete residue classification

The package SHALL classify all 689 uncovered obligations. Each row SHALL name
its family, facet, mechanism need, kernel disposition, and confidence.

#### Scenario: an obligation does not fit

- WHEN no current class applies
- THEN the row uses an explicit orphan status
- AND the analyst does not force a nearest category.

### Requirement: genuine rater evidence

Agreement claims SHALL use at least two independent human raters. The package
SHALL preserve blinded inputs, raw decisions, disagreements, and adjudication.

#### Scenario: only model-generated ratings exist

- WHEN no independent human ratings exist
- THEN the agreement result is labeled simulated and excluded from decision gates.

### Requirement: rater data protection

The rater protocol SHALL define consent, pseudonyms, blinding, retention,
deletion, withdrawal, access control, and incident handling before collection.

#### Scenario: public evidence is exported

- WHEN raw rating evidence enters the repository
- THEN it uses pseudonyms and excludes the private identity mapping.

### Requirement: taxonomy boundary

Taxonomy evidence SHALL NOT enlarge Moriarty Core. A kernel primitive requires
separate semantic, demand, and proof evidence.

#### Scenario: a common facet appears

- WHEN a facet is frequent across protocols
- THEN frequency alone does not authorize a new Core construct.

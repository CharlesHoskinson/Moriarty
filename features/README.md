# features

Design requirements for language features that another system depends on. Each feature directory states **what Moriarty must provide and why**, so that its milestone placement and design review can be argued against a concrete consumer.

A feature here is a **requirement plus concept notes**. It is not a design that has been accepted. Like `concepts/`, nothing here closes a U predicate or overrides the controlling documents ([ROADMAP.md](../ROADMAP.md), [docs/MORIARTY-CONSOLIDATED-DESIGN.md](../docs/MORIARTY-CONSOLIDATED-DESIGN.md), [docs/MORIARTY-PRODUCT-CONTRACT.md](../docs/MORIARTY-PRODUCT-CONTRACT.md)). The two directories differ in scope:
- `concepts/` holds whole-language designs still under argument, such as MIL/2.
- `features/` holds individual capabilities a consumer requires. A feature normally lands by being designed into the current concept and then adopted through it.

Each feature directory contains:
- `README.md`: the requirement in one paragraph, its consumer, status and owner decisions.
- `CONCEPT.md`: concept notes giving the model, semantics, laws, interactions with existing decisions, and open questions.
- `REQUIREMENTS.md`: numbered EARS requirements and the obligations a design must discharge.

Claims use the MIL/2 tags **[checked]**, **[obligation]**, **[deferred]**, plus **[prop]** for proposals made here that nobody has checked yet.

## Contents

- [multi-principal-composition](multi-principal-composition/README.md): composing the authenticated intents of several principals (user, issuer, regulator, venue, jurisdiction, agent) over one governed object. Required by exocovenants (Fenrir).

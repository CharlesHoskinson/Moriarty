# APSS Permission research bank

Draft research, 2026-09-19. Twelve distinct primary sources; no vault writes, code implementation, provider dispatch or financial transaction. The architecture assumption is permissionless Moriarty deployment with machine-verifiable asset-owner authorization.

The documentation follows the four forms described by [Diátaxis](https://www.diataxis.fr/):

- [Explanation](explanation.md): concepts, tradeoffs, assumptions and the proposed Moriarty boundary.
- [Reference](reference.md): annotated sources, terminology, proposed requirements and falsifiable cases.
- [How-to](how-to.md): review a concrete permission interface.
- [Tutorial](tutorial.md): a guided conceptual payment-grant exercise, explicitly not runnable Moriarty.

Evidence files are separate: `sources.json`, `claims.json`, individual acquisition receipts, raw HTML/PDF, scoped text and `visual-coverage.json`. `acquire.py` fetches the primary sources in bounded rounds. Full PDF bytes are retained; only listed pages were visually read. See `ACQUISITION.md` for limits and failed preparation steps.

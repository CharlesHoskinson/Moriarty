
// Render hyperedges as shaded regions
const hyperedges = [{"id": "deliverables_modern_defi_taxonomy_2026_09_08_cases_deliverables_modern_defi_taxonomy_2026_09_08_validation_async_claim_lifecycle", "label": "Documented asynchronous external-share lifecycle; exact fund deployment binding unverified", "nodes": ["deliverables_modern_defi_taxonomy_2026_09_08_case_12", "deliverables_modern_defi_taxonomy_2026_09_08_erc_7540", "deliverables_modern_defi_taxonomy_2026_09_08_erc_7575", "deliverables_modern_defi_taxonomy_2026_09_08_cases_deliverables_modern_defi_taxonomy_2026_09_08_validation_centrifuge_fund_subscription_request", "deliverables_modern_defi_taxonomy_2026_09_08_cases_deliverables_modern_defi_taxonomy_2026_09_08_validation_centrifuge_claimable_fund_shares"], "relation": "participate_in", "confidence": "EXTRACTED", "confidence_score": 1.0, "source_file": "deliverables/modern-defi-taxonomy-2026-09-08/VALIDATION.md", "source_location": "C16-C18"}, {"id": "deliverables_modern_defi_taxonomy_2026_09_08_cases_deliverables_modern_defi_taxonomy_2026_09_08_validation_morpho_allocation", "label": "Documented vault allocation into lending markets", "nodes": ["deliverables_modern_defi_taxonomy_2026_09_08_case_10", "deliverables_modern_defi_taxonomy_2026_09_08_case_09", "deliverables_modern_defi_taxonomy_2026_09_08_erc_4626"], "relation": "participate_in", "confidence": "EXTRACTED", "confidence_score": 1.0, "source_file": "deliverables/modern-defi-taxonomy-2026-09-08/VALIDATION.md", "source_location": "C47"}];
// afterDrawing passes ctx already transformed to network coordinate space.
// Draw node positions raw — no manual pan/zoom/DPR math needed.

// Andrew's monotone chain. Returns the hull in counter-clockwise order, which
// is what the perimeter must be traced in. Collinear and duplicate points
// collapse to the extremes, so degenerate member sets render as a segment
// rather than a zero-area crossed path.
function convexHull(pts) {
    const p = pts.slice().sort((a, b) => (a.x - b.x) || (a.y - b.y));
    if (p.length < 3) return p;
    const cross = (o, a, b) => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
    const build = seq => {
        const out = [];
        for (const q of seq) {
            while (out.length >= 2 && cross(out[out.length - 2], out[out.length - 1], q) <= 0) out.pop();
            out.push(q);
        }
        out.pop();
        return out;
    };
    const hull = build(p).concat(build(p.slice().reverse()));
    return hull.length >= 3 ? hull : p;
}
network.on('afterDrawing', function(ctx) {
    hyperedges.forEach(h => {
        const positions = h.nodes
            .map(nid => network.getPositions([nid])[nid])
            .filter(p => p !== undefined);
        if (positions.length < 2) return;
        ctx.save();
        ctx.globalAlpha = 0.12;
        ctx.fillStyle = '#6366f1';
        ctx.strokeStyle = '#6366f1';
        ctx.lineWidth = 2;
        ctx.beginPath();
        // Centroid and expanded hull in network coordinates.
        // The perimeter must follow hull order, not h.nodes order: tracing the
        // raw member order self-intersects whenever the layout does not happen
        // to place members in angular order, filling as crossed wedges.
        const cx = positions.reduce((s, p) => s + p.x, 0) / positions.length;
        const cy = positions.reduce((s, p) => s + p.y, 0) / positions.length;
        const hull = convexHull(positions);
        const expanded = hull.map(p => ({
            x: cx + (p.x - cx) * 1.15,
            y: cy + (p.y - cy) * 1.15
        }));
        ctx.moveTo(expanded[0].x, expanded[0].y);
        expanded.slice(1).forEach(p => ctx.lineTo(p.x, p.y));
        ctx.closePath();
        ctx.fill();
        ctx.globalAlpha = 0.4;
        ctx.stroke();
        // Label
        ctx.globalAlpha = 0.8;
        ctx.fillStyle = '#4f46e5';
        ctx.font = 'bold 11px sans-serif';
        ctx.textAlign = 'center';
        ctx.fillText(h.label, cx, cy - 5);
        ctx.restore();
    });
});

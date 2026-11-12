// Placement of the storyboard labels. Text comes from text.js; each entry here follows the label order
// of its beat: [where, kind, dy, from, to]. `where` is an anchor name or a scene-local point.
const X = (u) => 5 + u;
const Z = (v) => 3 + v;

export const PLACE = [
  [[[-14, 2.7, -0.9], 'neutral', 0.2, 0, 8.8], ['vk', 'state', 0.9, 9], ['seal', 'state', 0.5, 9], ['envStrip', 'neutral', 0.6, 9], ['cab', 'neutral', 0.3, 0, 8.8]],
  [[[-6.6, 0.95, -1.4], 'neutral', 0.3, 2.0, 2.6], [[-7.25, 0.95, -2.3], 'neutral', 0.3, 2.0, 2.9], [[-4.6, 1, -2.2], 'state', 0.3], ['claim', 'state', 0.5, 5.3], ['collarEnv', 'state', 0.6]],
  [['cab', 'neutral', 0.3], ['claim', 'state', 1.2], [[3, 1.75, -3.5], 'neutral', 0.6]],
  [['claim', 'state', 0.7], ['cO', 'outcome', 0.5, 4.6], ['collarCp', 'state', 0.5, 4.8], ['cab', 'neutral', 0.3]],
  [['seal2', 'state', 0.7], ['claim', 'state', 0.9], [[3, 0.7, -3.2], 'neutral', 0.1], ['hatch2', 'error', 0.5, 3.6]],
  [['tape', 'neutral', 0.9], ['proofB', 'state', 0.6, 1.0], ['claim2', 'state', 0.6, 3.4], ['hatch2', 'error', 0.5, 3.6]],
  [['nodeA', 'state', 0.9], ['pn1', 'neutral', 0.5], ['claimT', 'state', 0.6]],
  [['nodeA', 'state', 1.0], ['setRoot', 'state', 0.5], ['pn1', 'neutral', 0.5], ['claimT', 'state', 0.6]],
  [['modePlate', 'neutral', 0.4], ['hdr3', 'state', 0.5], [[X(1.0), 0.4, Z(1.0) - 0.4], 'state', 0.2, 0, 2.9], [[X(1.0), 0.4, Z(1.0) + 0.9], 'state', 0.2, 3.1]],
  [['claimT', 'state', 0.5], ['outP', 'outcome', 0.5], [[X(-3.7), 0.35, Z(1.0) + 0.75], 'outcome', 0.1], ['inC', 'input', 0.5], [[X(0), 0.3, Z(1.0) + 0.9], 'state', 0.2, 3.1], ['modePlate', 'neutral', 0.4]],
  [['cons', 'state', 2.6], ['claimC', 'state', 0.9], ['covT', 'state', 1.8], ['link1', 'neutral', 0.4], ['cab', 'neutral', 0.3]],
  [['fs', 'state', 0.8], ['fcard', 'state', 0.4], ['acard', 'state', 0.4], [[X(6.25), 0.2, Z(-2.3)], 'neutral', 0.2, 3.6], ['regT', 'state', 0.7]],
  [[[X(3.8), 0.4, Z(0.4) - 0.5], 'neutral', 0.1], [[X(4.8), 0.4, Z(0.4) - 1.1], 'neutral', 0.1], [[X(5.8), 0.4, Z(0.4) - 0.5], 'neutral', 0.1], [[X(6.8), 0.4, Z(0.4) - 1.1], 'neutral', 0.1], [[X(5.3), 0.2, Z(1.2) + 0.9], 'state', 0.1], ['regT', 'state', 0.7, 2.5]],
  [[[X(2.5), 0.4, Z(0.4) - 0.5], 'neutral', 0.1], [[X(3.5), 0.4, Z(0.4) - 1.1], 'neutral', 0.1], [[X(4.5), 0.4, Z(0.4) - 0.5], 'neutral', 0.1], [[X(5.5), 0.4, Z(0.4) - 1.1], 'state', 0.1], [[X(6.5), 0.4, Z(0.4) - 0.5], 'neutral', 0.1], [[X(7.5), 0.4, Z(0.4) - 1.1], 'neutral', 0.1, 2.5]],
  [[[X(-2.2), 0.2, Z(-1.3)], 'neutral', 0.1], ['fmt', 'state', 0.4], [[X(-1.0), 0.4, Z(-0.5)], 'neutral', 0.1], [[X(1.25), 0.4, Z(-0.5)], 'neutral', 0.1], [[X(2.4), 0.4, Z(-1.1)], 'neutral', 0.1]],
  [['trCard', 'state', 0.4], [[X(-1.0), 0.4, Z(-0.5)], 'neutral', 0.1], [[X(3.5), 0.4, Z(-0.5)], 'input', 0.1, 2.4], [[X(5.5), 0.4, Z(-0.5)], 'state', 0.1, 5.0]],
  [[[X(3.2), 0.2, Z(1.95)], 'neutral', 0.1], [[X(-3), 0.4, Z(0.6) - 0.3], 'neutral', 0.1], [[X(0), 0.4, Z(0.6) - 0.3], 'neutral', 0.1], [[X(3.5), 0.4, Z(0.2) - 0.3], 'state', 0.1, 3.9], [[X(2.0) + 1.8, 0.35, Z(-1.6) - 0.2], 'state', 0.1]],
  [['nodeB', 'state', 0.7], [[X(-2.6), 0.55, Z(-0.8) - 0.9], 'state', 0.1, 3.1], [[X(-3.0), 0.35, Z(1.4) + 0.9], 'state', 0.1], [[X(-1.8), 0.35, Z(1.4) - 1.0], 'neutral', 0.1], [[X(-0.6), 0.35, Z(1.4) - 0.3], 'neutral', 0.1], [[X(0.6), 0.35, Z(1.4) - 1.0], 'state', 0.1]],
  [[[X(-3.5), 0.2, Z(-2.4)], 'neutral', 0.1], ['card1', 'state', 0.4], ['ringSlab', 'state', 0.8], ['block', 'neutral', 0.9]],
  [[[8.0, 0.4, 2.6], 'neutral', 0.1, 3.7], [[14.0, 0.4, 3.4], 'input', 0.1, 3.7], [[14.0, 0.4, 2.7], 'state', 0.1, 3.7], [[11.95, 0.3, 1.9], 'neutral', 0.1], [[8.0, 0.4, 3.6], 'neutral', 0.1, 4.2], [[14.0, 0.4, 2.0], 'neutral', 0.1, 4.2]],
  [['recClaim', 'state', 0.9], ['recOut', 'outcome', 2.0], ['recCollar', 'state', 1.4], null, null, ['rec', 'state', 2.9]],
];

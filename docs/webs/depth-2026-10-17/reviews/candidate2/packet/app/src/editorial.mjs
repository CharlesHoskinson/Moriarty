/** Display edits authorized by the user's editorial review. Frozen research
 * contributions and technical evidence remain unchanged. */
export function applyEditorialRevision(data, revision) {
  const output = structuredClone(data);
  output.scenarios = output.scenarios.map(scenario => ({...scenario, title: revision.scenarioLabels[scenario.id] || scenario.title}));
  const main = revision.main;
  output.v2.main = {
    ...output.v2.main,
    ...main,
    worksheet: {...output.v2.main.worksheet, ...main.worksheet},
  };
  const architecture = revision.sevenLayers;
  output.sevenLayers = {
    ...output.sevenLayers,
    ...architecture,
    layers: output.sevenLayers.layers.map(layer => ({
      ...layer, ...architecture.layers.find(edit => edit.number === layer.number),
    })),
  };
  output.groth16 = {
    ...output.groth16,
    ...revision.groth16,
    counter: {...output.groth16.counter, ...revision.groth16.counter},
  };
  return output;
}

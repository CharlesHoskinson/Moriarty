/**
 * Publication gate for compileCustody. Pure decisions only: no filesystem
 * access, no tool spawn, no admission. A false gate publishes no blueprint.
 */
export const SUCCESS_ARTIFACTS = Object.freeze(['plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json']);

export function shouldApply({ checkOk, compiled }) {
  return checkOk === true && compiled === true;
}

export function shouldPublish({ checkOk, compiled, applied }) {
  return shouldApply({ checkOk, compiled }) && applied?.ok === true;
}

/**
 * Whole-body builder for one head transaction.
 * Cardano Plutus V3 Transaction, as imported by this project, has fee and
 * inputs. It has no collateral field. Collateral is a ledger-body field.
 * A passing shape check does not encode a transaction. buildCustodyBody does.
 */

import { encodeUnsignedBody } from './native-body.mjs';

export const BODY_CONTROL = 'whole-body-builder';

function refKey(ref) {
  if (!ref || typeof ref.transactionId !== 'string' || !Number.isInteger(ref.outputIndex)) return null;
  return `${ref.transactionId}:${ref.outputIndex}`;
}

function refuse(code) {
  return {
    ok: false,
    code,
    ledgerExecution: false,
    nativePins: false,
    nativeBuilder: 'encodeUnsignedBody',
    builderInvoked: false,
    scriptContextCollateral: false,
    class: BODY_CONTROL,
  };
}

export function expectCustodyBody(body) {
  if (!body || typeof body !== 'object') return refuse('BODY_INPUTS_REQUIRED');
  if (body.ledgerExecution === true) return refuse('LEDGER_EXECUTION_REFUSED');
  const head = refKey(body.headRef);
  const fee = refKey(body.feeRef);
  const collateral = refKey(body.collateralRef);
  if (!head || !fee || !collateral) return refuse('BODY_INPUTS_REQUIRED');
  if (fee === collateral) return refuse('FEE_COLLATERAL_ALIAS');
  if (fee === head || collateral === head) return refuse('HEAD_IS_NOT_FEE_OR_COLLATERAL');
  return {
    ok: true,
    code: 'BODY_SHAPE_OK',
    ledgerExecution: false,
    nativePins: false,
    nativeBuilder: 'encodeUnsignedBody',
    builderInvoked: false,
    scriptContextCollateral: false,
    class: BODY_CONTROL,
  };
}

export function buildCustodyBody(spec, cliPath) {
  const encoded = encodeUnsignedBody(spec, cliPath);
  return { ...encoded, nativeBuilder: 'encodeUnsignedBody', builderInvoked: true, scriptContextCollateral: false };
}

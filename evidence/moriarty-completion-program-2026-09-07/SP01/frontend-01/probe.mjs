import {readFileSync} from 'node:fs';
import {compile} from '/home/charl/Moriarty/.worktrees/sp01-frontend-grok/experiments/moriarty-language/src/frontend.ts';
const bounds=readFileSync('/home/charl/Moriarty/.worktrees/sp01-frontend-grok/experiments/moriarty-language/spec/bounds.json');
const status=`agreement Generic profile "moriarty-bounded-atomic/1" {
 lifetime 2; horizon 100;
 status episode closed_when closed == uint(1);
 state closed: UInt128 = uint(0);
 observation now: UInt128;
 status agreement no_remaining_notional;
 action run(actor: Text) { guard uint(1), "bad"; }
}`;
const ordinal=n=>`agreement Generic profile "moriarty-bounded-atomic/1" {
 lifetime 2; horizon 100; unit U;
 state closed: UInt128 = uint(0); observation now: UInt128;
 status episode closed_when closed == uint(1); status agreement no_remaining_notional;
 effect Transfer { asset: Text; from: Text; to: Text; amount: Amount; }
 policy p targets effect(run,${n},amount) { unit U; derivation ""; rounding none; remainder ""; comparison ""; proof "p"; }
 action run(actor: Text) { emit Transfer { asset: text("U"), from: arg.actor, to: text("recipient"), amount: amount(1,U) }; }
}`;
const cases={prematureStatusAndBadGuard:status,prematureStatusOnly:status.replace('guard uint(1)','guard true'),declaredStatusBadGuard:status.replace('status episode closed_when closed == uint(1);\n state closed: UInt128 = uint(0);','state closed: UInt128 = uint(0);\n status episode closed_when closed == uint(1);'),ordinal0:ordinal('0'),ordinal1:ordinal('1'),ordinalMaximum:ordinal('340282366920938463463374607431768211455'),ordinalOverflow:ordinal('340282366920938463463374607431768211456')};
for(const [name,source] of Object.entries(cases)){try{compile(source,bounds);console.log(JSON.stringify({name,result:'accepted'}));}catch(e){console.log(JSON.stringify({name,result:'rejected',diagnostic:e.diagnostic??{code:e.code,message:e.message}}));}}

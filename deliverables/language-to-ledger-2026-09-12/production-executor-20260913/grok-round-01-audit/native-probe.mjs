import {readFileSync} from 'node:fs';
import {validatePreviewLaunchPlan} from "file:///home/charl/Moriarty-wt-moriarty-release-afk-20260913-implement-production-executor/experiments/moriarty-midnight-financial/ledger/preview-launch.mjs";
const plans=JSON.parse(readFileSync("/tmp/moriarty-production-grok-r1-independent-probes/native-plans.json",'utf8'));
for(const [name,p] of Object.entries(plans)){try{validatePreviewLaunchPlan(p);console.log(JSON.stringify({name,accepted:true}));}catch(e){console.log(JSON.stringify({name,accepted:false,error:e.message}));}}

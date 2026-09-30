import {spawn} from 'node:child_process';import fs from 'node:fs';
const p=spawn('./node_modules/.bin/mori',['mcp']);let buf='';const res=[];
p.stdout.on('data',d=>{buf+=d});
const src=fs.readFileSync('merchant/merchant.mori','utf8'),sc=fs.readFileSync('merchant/scenarios/low_allowance.json','utf8');
const send=o=>p.stdin.write(JSON.stringify(o)+'\n');
send({jsonrpc:'2.0',id:1,method:'initialize',params:{protocolVersion:'2025-03-26',capabilities:{},clientInfo:{name:'s1',version:'0'}}});
setTimeout(()=>{send({jsonrpc:'2.0',method:'notifications/initialized'});send({jsonrpc:'2.0',id:2,method:'tools/list'});
send({jsonrpc:'2.0',id:3,method:'tools/call',params:{name:'preview',arguments:{source:src,action:'pay_invoice',scenario:sc}}});},500);
setTimeout(()=>{fs.writeFileSync('outputs/22-mcp-session.txt',buf);p.kill();console.log(buf.slice(0,1500))},1500);

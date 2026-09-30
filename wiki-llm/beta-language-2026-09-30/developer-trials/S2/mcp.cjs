const {spawn}=require("child_process");const fs=require("fs");
const p=spawn("../node_modules/.bin/mori",["mcp"]);let buf="";const out=[];
p.stdout.on("data",d=>{buf+=d;});
const send=o=>p.stdin.write(JSON.stringify(o)+"\n");
send({jsonrpc:"2.0",id:1,method:"initialize",params:{protocolVersion:"2025-06-18",capabilities:{},clientInfo:{name:"s2",version:"0"}}});
setTimeout(()=>{send({jsonrpc:"2.0",method:"notifications/initialized"});send({jsonrpc:"2.0",id:2,method:"tools/list"});
send({jsonrpc:"2.0",id:3,method:"tools/call",params:{name:"preview",arguments:{source:fs.readFileSync("loan.mori","utf8"),action:"pay_full",scenario:fs.readFileSync("scenarios/open.json","utf8")}}});},300);
setTimeout(()=>{fs.writeFileSync("../outputs/mcp_session.txt",buf);p.kill();console.log(buf.length);},1200);

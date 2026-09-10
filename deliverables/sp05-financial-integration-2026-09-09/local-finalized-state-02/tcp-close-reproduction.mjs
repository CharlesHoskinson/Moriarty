import net from 'node:net';
import {writeFileSync} from 'node:fs';
const server=net.createServer(socket=>socket.once('data',()=>socket.destroy()));
await new Promise(r=>server.listen(0,'127.0.0.1',r));
try{
 try{await fetch(`http://127.0.0.1:${server.address().port}/`,{method:'POST',body:'{}',signal:AbortSignal.timeout(3000)});throw Error('UNEXPECTED_SUCCESS');}
 catch(e){if(e.message==='UNEXPECTED_SUCCESS')throw e;const evidence={node:process.version,errorClass:e.name,message:e.message,causeCode:e.cause?.code,scope:'Ephemeral loopback TCP accepts request then closes without HTTP response; not proof of original probe01 cause'};writeFileSync(new URL('./tcp-close-reproduction.json',import.meta.url),JSON.stringify(evidence,null,2)+'\n',{flag:'wx'});console.log(JSON.stringify(evidence));}
}finally{await new Promise(r=>server.close(r));}

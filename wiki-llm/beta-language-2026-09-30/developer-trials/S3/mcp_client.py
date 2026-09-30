import subprocess,json
p=subprocess.Popen(['./node_modules/.bin/mori','mcp'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
def send(m): p.stdin.write(json.dumps(m)+"\n"); p.stdin.flush()
def recv(): return json.loads(p.stdout.readline())
out=[]
send({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"s3","version":"0"}}}); out.append(recv())
send({"jsonrpc":"2.0","method":"notifications/initialized"})
send({"jsonrpc":"2.0","id":2,"method":"tools/list"}); out.append(recv())
src=open('src/swap_fee.mori').read(); sc=open('scenarios/swap_fee_ok.json').read()
send({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"preview","arguments":{"source":src,"action":"settle_swap_fee","scenario":sc}}}); out.append(recv())
send({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"check","arguments":{"source":open('src/pool_oracle.mori').read()}}}); out.append(recv())
p.stdin.close(); p.wait(5)
json.dump(out,open('outputs/mcp_session.json','w'),indent=1)
for o in out: print(json.dumps(o)[:400])

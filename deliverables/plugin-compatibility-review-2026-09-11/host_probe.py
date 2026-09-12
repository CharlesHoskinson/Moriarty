"""Record actual Codex hook/tool events without changing trust or hook config."""
import json
from pathlib import Path
import queue
import subprocess
import sys
import threading
import time

OUT = Path(__file__).resolve().parent
FIXTURE = Path('/tmp/moriarty-plugin-compatibility-host-20260911')
name = sys.argv[1]
records = []
commands = []
messages = queue.Queue()
deadline = time.monotonic() + 180
stderr = (OUT / (name + '.stderr')).open('x')
process = subprocess.Popen(['codex', 'app-server', '--stdio'], stdin=subprocess.PIPE,
                           stdout=subprocess.PIPE, stderr=stderr, text=True, bufsize=1)

def pump():
    for line in process.stdout:
        messages.put(line)
    messages.put(None)

threading.Thread(target=pump, daemon=True).start()

def read():
    line = messages.get(timeout=max(0.1, deadline - time.monotonic()))
    if line is None:
        raise RuntimeError('app-server EOF')
    data = json.loads(line)
    method = data.get('method', '')
    if method.startswith('hook/'):
        records.append(data)
    if method == 'item/completed':
        item = data.get('params', {}).get('item', {})
        if item.get('type') == 'commandExecution':
            commands.append({key: item[key] for key in ('type', 'id', 'status', 'command', 'aggregatedOutput', 'exitCode') if key in item})
    return data

def request(identifier, method, params):
    process.stdin.write(json.dumps({'jsonrpc': '2.0', 'id': identifier, 'method': method, 'params': params}) + '\n')
    process.stdin.flush()
    while True:
        data = read()
        if data.get('id') == identifier:
            if 'error' in data:
                raise RuntimeError(str(data['error']))
            return data['result']

result = {}
try:
    request(1, 'initialize', {'clientInfo': {'name': 'moriarty_compatibility_probe', 'version': '1'}, 'capabilities': {'experimentalApi': True}})
    instruction = ('This is a harmless hook smoke test in a disposable fixture repository. '
                   'Use the shell tool to execute exactly python3 marker.py once. It is expected to be denied by the hook. '
                   'Do not bypass or retry the denial. Then execute exactly pwd once via the shell tool. '
                   'Then execute exactly printf compatibility-denied-marker once; it only prints an action ID and must be permitted. '
                   'Then reply OK and stop. No other tools, reads, edits, delegation, or commands.')
    thread = request(2, 'thread/start', {'cwd': str(FIXTURE), 'ephemeral': True,
                                       'approvalPolicy': 'never', 'model': 'gpt-6-astra',
                                       'baseInstructions': instruction})
    tid = thread['thread']['id']
    request(3, 'turn/start', {'threadId': tid, 'input': [{'type': 'text', 'text': instruction, 'text_elements': []}], 'effort': 'low'})
    while True:
        data = read()
        if data.get('method') == 'turn/completed':
            result = {'threadId': tid, 'turnStatus': data['params']['turn']['status'],
                      'markerExists': (FIXTURE / 'unexpected-marker').exists(), 'commands': commands}
            break
finally:
    (OUT / (name + '-events.json')).write_text(json.dumps(records, indent=2) + '\n')
    (OUT / (name + '-result.json')).write_text(json.dumps(result, indent=2) + '\n')
    process.terminate()
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        process.kill()
        process.wait(timeout=5)
    stderr.close()
print(json.dumps(result))

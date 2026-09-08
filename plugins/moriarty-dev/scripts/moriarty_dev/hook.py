#!/usr/bin/env python3
"""Bounded, read-only Codex hook adapter; guarded CLI remains the launch gate."""
from __future__ import annotations
import json
import os
from pathlib import Path
import shlex
import signal
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from moriarty_dev.policy import assess
from moriarty_dev.records import load_snapshot, read_actions
from moriarty_dev.store import get_db_path, get_undelivered_txs, make_history_reader
from moriarty_dev.notifications import notification_line


def _last_result(repo):
    try:
        data=json.loads((repo/'openspec/moriarty-completion-program.json').read_text())
        stages=data.get('reportReconciliation',{}).get('stageAdmission',{}).get('stages',[])
        done=[r.get('id') for r in stages if r.get('status')=='complete']
        return 'Stages complete: '+(', '.join(done) if done else 'none')
    except Exception:
        return 'No readable program stage record'


def _output(event, **fields):
    return {'hookSpecificOutput':{'hookEventName':event,**fields}}


def _diagnostic(event):
    return {**_output(event), 'systemMessage':'Moriarty hook input or records unavailable; use guarded CLI status/run. Host coverage remains unverified.'}


def _pending(repo):
    rows=get_undelivered_txs(get_db_path(repo),str(repo),timeout=0.05,limit=5)
    if not rows: return ''
    text='Pending public observations: post dedicated notification message, then use deliver.\n'
    text+='\n'.join(notification_line(tx) for tx in rows[:4])
    if len(rows)>4: text+='\nMore pending IDs: use report --json.'
    return text


def _bounded_json(output):
    # Truncate values before serializing, never cut encoded JSON mid-token.
    inner=output.get('hookSpecificOutput',{})
    for owner,key,limit in ((inner,'additionalContext',1300),(inner,'permissionDecisionReason',350),(output,'systemMessage',180)):
        if isinstance(owner.get(key),str): owner[key]=owner[key].encode()[:limit].decode('utf-8','ignore')
    text=json.dumps(output,ensure_ascii=False,separators=(',',':'))
    if len(text.encode())+1>2048:
        if inner.get('permissionDecision')=='deny':
            output=_output('PreToolUse',permissionDecision='deny',permissionDecisionReason='Moriarty denied this action; use CLI status for details.')
        else: output={'systemMessage':'Moriarty hook output exceeded bound; use CLI status.'}
        text=json.dumps(output,separators=(',',':'))
    return text


def main():
    event=sys.argv[1] if len(sys.argv)>1 else ''
    timer_supported=hasattr(signal,'setitimer')
    previous_handler=None
    class HookDeadline(BaseException):
        pass
    def deadline(_signum,_frame):
        raise HookDeadline()
    try:
        if timer_supported:
            previous_handler=signal.signal(signal.SIGALRM,deadline)
            signal.setitimer(signal.ITIMER_REAL,0.75)
        raw=sys.stdin.read(8193) if not sys.stdin.isatty() else ''
        try: payload=json.loads(raw) if raw.strip() else {}
        except ValueError: payload={}
        if not isinstance(payload,dict) or len(raw)>8192: payload={}
        event=event or payload.get('hook_event_name',payload.get('hookEventName',''))
        cwd=payload.get('cwd')
        repo=Path(cwd if isinstance(cwd,str) else os.getcwd()).resolve()
        for ancestor in (repo,*repo.parents):
            if (ancestor/'.moriarty-dev/actions.json').is_file():
                repo=ancestor;break
            if (ancestor/'.git').exists(): break
        output=handle_event(event,payload,repo)
    except (HookDeadline, Exception): output=_diagnostic(event)
    finally:
        if timer_supported:
            signal.setitimer(signal.ITIMER_REAL,0)
            if previous_handler is not None: signal.signal(signal.SIGALRM,previous_handler)
    print(_bounded_json(output))


def handle_event(event_name,payload,repo):
    if not (repo/'.moriarty-dev/actions.json').is_file():
        return _output(event_name,permissionDecision='allow') if event_name=='PreToolUse' else _output(event_name)
    if event_name=='SessionStart': return handle_session_start(repo)
    if event_name=='PreToolUse': return handle_pre_tool_use(payload,repo)
    if event_name=='PostToolUse':
        try: return _output(event_name,additionalContext=_pending(repo))
        except Exception: return _diagnostic(event_name)
    # Stop never blocks, continues, or invokes another action. Pending messages
    # are supplied at SessionStart/PostToolUse and through status/report.
    return _output(event_name)


def handle_session_start(repo):
    try:
        actions=read_actions(str(repo))
        if not actions: return _output('SessionStart')
        act=next((a for a in actions if a.get('kind')!='report'),actions[0])
        snap=load_snapshot(str(repo),act['id'],history_reader=make_history_reader(get_db_path(repo),timeout=0.05))
        lines=[f"Capability: {act.get('requirement')} {act.get('capability')}",f'Last result: {_last_result(repo)}',f"Next: {snap.get('nextActionId') or act['id']}"]
        failures=snap.get('sameDefectFailures')
        if isinstance(failures,int) and failures>=2: lines.append('Blocked: another broad correction; reproduce the unresolved defect.')
        elif failures is None: lines.append('History unresolved; inspect CLI status before dispatch.')
        pending=_pending(repo)
        if pending: lines.append(pending)
        return _output('SessionStart',additionalContext='\n'.join(lines))
    except Exception: return _diagnostic('SessionStart')


def _registered_argv(repo,action):
    ref=action.get('commandRef','')
    if '#' not in ref: return None
    filename,key=ref.split('#',1)
    path=(repo/filename).resolve()
    if not path.is_relative_to(repo.resolve()): return None
    try:
        data=json.loads(path.read_text())
        argv=data.get('commands',{}).get(key,{}).get('argv')
        return argv if isinstance(argv,list) and all(isinstance(s,str) for s in argv) else None
    except (OSError,ValueError,AttributeError): return None


def handle_pre_tool_use(payload,repo):
    event='PreToolUse'
    # Code strings, patch bodies, and unknown tool arguments are not shell
    # commands. Do not block a source edit merely because it names an action.
    name=payload.get('tool_name',payload.get('toolName',''))
    if name not in ('Bash','bash','shell','exec_command','execute_command'):
        return _output(event,permissionDecision='allow')
    try:
        inp=payload.get('tool_input',payload.get('toolInput',{}))
        if not isinstance(inp,dict): return _diagnostic(event)
        value=inp.get('command') or inp.get('cmd') or inp.get('argv') or ''
        tokens=shlex.split(value) if isinstance(value,str) else value
        if not isinstance(tokens,list) or not all(isinstance(t,str) for t in tokens): return _diagnostic(event)
        actions=read_actions(str(repo))
        for act in actions:
            named=act['id'] in tokens or '--action='+act['id'] in tokens
            argv=_registered_argv(repo,act)
            raw=bool(argv) and tokens==argv
            if not named and not raw: continue
            if raw:
                return _output(event,permissionDecision='deny',permissionDecisionReason=f"Registered dispatch requires guarded CLI run --action {act['id']}.")
            snap=load_snapshot(str(repo),act['id'],history_reader=make_history_reader(get_db_path(repo),timeout=0.05))
            dec=assess(snap,act)
            if not dec['allow']:
                return _output(event,permissionDecision='deny',permissionDecisionReason=f"Repeated unresolved failure or unavailable admission ({dec.get('reasonCode')}). Run {dec.get('nextActionId') or 'CLI status'}.")
    except Exception: return _diagnostic(event)
    return _output(event,permissionDecision='allow')

if __name__=='__main__': main()

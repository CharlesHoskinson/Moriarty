import subprocess,sys,time,pathlib; subprocess.Popen([sys.executable,'-c',"import pathlib,time; pathlib.Path('/tmp/moriarty-runner-consumer-7ki3sa5l').mkdir(parents=True,exist_ok=True); pathlib.Path('/tmp/moriarty-runner-consumer-7ki3sa5l/ready').write_text('ready'); time.sleep(3); pathlib.Path('/tmp/moriarty-runner-consumer-7ki3sa5l/escaped').write_text('survived')",'moriarty-probe-eeb8b387c2054e2482d91a01b0e093c3'],start_new_session=True,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL); p=pathlib.Path('/tmp/moriarty-runner-consumer-7ki3sa5l/ready'); deadline=time.monotonic()+2
while not p.exists() and time.monotonic()<deadline: time.sleep(.01)
assert p.exists()
print('DETACHED_READY')

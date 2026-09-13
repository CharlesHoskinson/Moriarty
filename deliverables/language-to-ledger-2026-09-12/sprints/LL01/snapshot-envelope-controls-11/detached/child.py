import subprocess,sys,time,pathlib; subprocess.Popen([sys.executable,'-c',"import pathlib,time; pathlib.Path('/tmp/moriarty-runner-consumer-vpafbd4p').mkdir(parents=True,exist_ok=True); pathlib.Path('/tmp/moriarty-runner-consumer-vpafbd4p/ready').write_text('ready'); time.sleep(3); pathlib.Path('/tmp/moriarty-runner-consumer-vpafbd4p/escaped').write_text('survived')",'moriarty-probe-be6f9ebd41ac4069a6b59a667ee12faa'],start_new_session=True,stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL); p=pathlib.Path('/tmp/moriarty-runner-consumer-vpafbd4p/ready'); deadline=time.monotonic()+2
while not p.exists() and time.monotonic()<deadline: time.sleep(.01)
assert p.exists()
print('DETACHED_READY')

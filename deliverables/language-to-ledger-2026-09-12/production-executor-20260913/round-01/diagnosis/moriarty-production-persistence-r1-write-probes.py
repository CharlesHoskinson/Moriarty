exec(open('/tmp/moriarty-production-persistence-r1-probes.py').read().split("execute_case('baseline')")[0])
orig_write=os.write
for target in ('terminal-observation.json','explicit-stop.json'):
 marker=OUT/(target+'.write-hits')
 def fail_write(fd,raw):
  fdpath=os.readlink('/proc/self/fd/'+str(fd))
  if pathlib.Path(fdpath).name==target:
   with marker.open('a') as f:f.write(json.dumps({'pid':os.getpid(),'fdPath':fdpath,'fault':'write_error'})+'\n')
   raise OSError('probe actual os.write failed')
  return orig_write(fd,raw)
 with patch.object(os,'write',fail_write):execute_case(target+'_write_error')
 rows[-1]['faultHits']=marker.read_text().splitlines()
(OUT/'write-results.json').write_text(json.dumps(rows,indent=2)+'\n')

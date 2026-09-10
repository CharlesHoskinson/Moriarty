# Unchanged head-race fixture after sampler repair

The fixture SHA256 remained `a5b79e786e260d0c015fd0daa1097d68af13ee53e7189bf42d764a7f2e39be81`. Against the repaired production sampler, it completed with `PUBLIC_STATE_VERIFIED` in 1822.34764 ms at finalized state block 20324. The ten-second-bounded subprocess exited 0, stderr was empty, the private-path trap log was empty, and the loopback server closed.

`head-sampling-green.json` retains 33 HTTP requests and four indexed-tip reads. The fixture's original result projection does not include the helper's snapshotSamples field, so no value for that counter is asserted. No fixture or production source edits were made for the green run. Red evidence remains unchanged. This is synthetic transport and retained public inputs, not a live-chain recovery result.

#!/bin/bash
# Lead runs of the r2 lane model (the lane sandbox refused quint).
cd $(dirname $0)
{
echo "## quint test"; timeout 1800 quint test --main decode_test --match '_test$' decode_test.qnt 2>&1 | tail -40
for inv in SOUND_AND_COMPLETE; do for s in 1 2; do echo "## run $inv seed $s"; timeout 3600 quint run --invariant $inv --max-samples 5000 --max-steps 10 --seed $s decode.qnt 2>&1 | tail -6; done; done
for m in no_sponge3_param idx_plus1 no_twice no_u16 no_mapped_replay t_in_wrong no_claim_pub no_chunk_range; do echo "## run SOUND_mut_$m seed 1"; timeout 1800 quint run --invariant SOUND_mut_$m --max-samples 5000 --max-steps 10 --seed 1 decode.qnt 2>&1 | tail -4; done
} > lead-runs.log 2>&1

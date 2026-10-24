//! Bounded storage inverse. Decoded records are caller data, never owner captures.
use super::c6_phys_host::{C6HostConfig, C6HostLimits, decode_host};
use super::c6_range_stream::{
    StreamBinding, StreamBounds, StreamFailure, StreamProgress, completed_call, state_start,
};
use super::c6_stream_event::decode_event;
use super::complete_observation::{CompleteStepRecord, ObservedStep};
use sha2::{Digest, Sha256};
use std::io::{self, Read};
pub struct DecodedRangeRecord {
    pub range: u8,
    pub sequence: u64,
    pub observed: ObservedStep,
}
pub struct C6RangeReader<R: Read> {
    reader: R,
    bounds: StreamBounds,
    config: C6HostConfig,
    limits: C6HostLimits,
    previous: Vec<u128>,
    bytes: u64,
    records: u64,
    completed: u64,
    range: u8,
    terminal: bool,
    finished: bool,
    failed: bool,
    hash: Sha256,
}
impl<R: Read> C6RangeReader<R> {
    fn bytes<const N: usize>(&mut self) -> Result<[u8; N], StreamFailure> {
        if self
            .bytes
            .checked_add(N as u64)
            .ok_or(StreamFailure::Overflow)?
            > self.bounds.max_total_bytes
        {
            return Err(StreamFailure::Bound);
        }
        let mut b = [0; N];
        self.reader.read_exact(&mut b).map_err(StreamFailure::Io)?;
        self.hash.update(b);
        self.bytes += N as u64;
        Ok(b)
    }
    fn u64(&mut self) -> Result<u64, StreamFailure> {
        Ok(u64::from_le_bytes(self.bytes()?))
    }
    fn lanes(&mut self, n: u64, start: u64) -> Result<Vec<u128>, StreamFailure> {
        let size = n.checked_mul(16).ok_or(StreamFailure::Overflow)?;
        if self
            .bytes
            .checked_add(size)
            .ok_or(StreamFailure::Overflow)?
            .checked_sub(start)
            .ok_or(StreamFailure::Overflow)?
            > self.bounds.max_record_bytes
            || self
                .bytes
                .checked_add(size)
                .ok_or(StreamFailure::Overflow)?
                > self.bounds.max_total_bytes
        {
            return Err(StreamFailure::Bound);
        }
        let n: usize = n.try_into().map_err(|_| StreamFailure::Overflow)?;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(u128::from_le_bytes(self.bytes()?))
        }
        Ok(v)
    }
    pub fn begin(
        reader: R,
        expected: StreamBinding,
        config: C6HostConfig,
        limits: C6HostLimits,
        bounds: StreamBounds,
    ) -> Result<Self, StreamFailure> {
        let mut r = Self {
            reader,
            bounds,
            config,
            limits,
            previous: Vec::new(),
            bytes: 0,
            records: 0,
            completed: 0,
            range: 0,
            terminal: false,
            finished: false,
            failed: false,
            hash: Sha256::new(),
        };
        if r.bytes::<8>()? != *b"C6RNG001" {
            return Err(StreamFailure::Immutable);
        }
        let observed = StreamBinding {
            source_sha256: r.bytes()?,
            input_sha256: r.bytes()?,
            expected_result_sha256: r.bytes()?,
        };
        if observed != expected {
            return Err(StreamFailure::Immutable);
        }
        let n = r.u64()?;
        if n as u128 > r.limits.max_total_lanes as u128 {
            return Err(StreamFailure::Bound);
        }
        r.previous = r.lanes(n, 0)?;
        decode_host(&r.previous, &r.config, &r.limits).map_err(StreamFailure::Codec)?;
        state_start(&r.previous)?;
        Ok(r)
    }
    pub fn progress(&self) -> StreamProgress {
        StreamProgress {
            records: self.records,
            bytes: self.bytes,
            completed_intents: self.completed,
            range: self.range,
            terminal: self.terminal,
            poisoned: self.failed,
            complete: self.finished,
            sha256: self.hash.clone().finalize().into(),
        }
    }
    pub fn next(&mut self) -> Result<Option<DecodedRangeRecord>, StreamFailure> {
        if self.failed {
            return Err(StreamFailure::Poisoned);
        }
        let result = self.next_inner();
        if result.is_err() {
            self.failed = true
        }
        result
    }
    fn next_inner(&mut self) -> Result<Option<DecodedRangeRecord>, StreamFailure> {
        if self.finished {
            return Ok(None);
        }
        let start = self.bytes;
        let tag = self.bytes::<1>()?[0];
        if tag == 4 {
            let records = self.u64()?;
            let intents = self.u64()?;
            if !self.terminal
                || self.range != 3
                || self.completed != 16
                || intents != 16
                || records != self.records
            {
                return Err(StreamFailure::Terminal);
            }
            let mut extra = [0];
            match self.reader.read(&mut extra) {
                Ok(0) => {}
                Ok(_) => return Err(StreamFailure::Immutable),
                Err(e) => return Err(StreamFailure::Io(e)),
            }
            self.finished = true;
            return Ok(None);
        }
        if self.terminal || self.records >= self.bounds.max_records || !(tag == 1 || tag == 2) {
            return Err(StreamFailure::Terminal);
        }
        let range = self.bytes::<1>()?[0];
        let sequence = self.u64()?;
        if range != self.range || sequence != self.records {
            return Err(StreamFailure::Continuity);
        }
        let en = self.u64()?;
        let event = self.lanes(en, start)?;
        if (tag == 1 && en == 0) || (tag == 2 && en != 0) {
            return Err(StreamFailure::Terminal);
        }
        let n = self.u64()?;
        if n as u128 > self.limits.max_state_lanes as u128 {
            return Err(StreamFailure::Bound);
        }
        let state = self.lanes(n, start)?;
        let at = state_start(&self.previous)?;
        let before = decode_host(&self.previous, &self.config, &self.limits)
            .map_err(StreamFailure::Codec)?
            .into_host();
        let total = at.checked_add(state.len()).ok_or(StreamFailure::Overflow)?;
        if total > self.limits.max_total_lanes {
            return Err(StreamFailure::Bound);
        }
        let mut host = Vec::with_capacity(total);
        host.extend_from_slice(&self.previous[..at]);
        host[12] = n.into();
        host.extend(state);
        let after = decode_host(&host, &self.config, &self.limits)
            .map_err(StreamFailure::Codec)?
            .into_host();
        let observed = if tag == 1 {
            let event = decode_event(&event, &before.image.run_id).map_err(StreamFailure::Event)?;
            ObservedStep::Event(CompleteStepRecord {
                before,
                event,
                after,
            })
        } else {
            if before != after {
                return Err(StreamFailure::Terminal);
            }
            ObservedStep::Terminal { before, after }
        };
        let completed = match completed_call(&observed)? {
            Some(n) if self.completed.checked_add(1) == Some(n) && n <= 16 => n,
            Some(_) => return Err(StreamFailure::Progress),
            None => self.completed,
        };
        self.records = self.records.checked_add(1).ok_or(StreamFailure::Overflow)?;
        if completed != self.completed && matches!(completed, 4 | 8 | 12) {
            if self.bytes::<1>()?[0] != 3
                || self.bytes::<1>()?[0] != self.range + 1
                || self.u64()? != self.records
                || self.u64()? != completed
            {
                return Err(StreamFailure::Progress);
            }
            self.range += 1;
        }
        if self.bytes - start > self.bounds.max_record_bytes {
            return Err(StreamFailure::Bound);
        }
        self.previous = host;
        self.completed = completed;
        self.terminal = tag == 2;
        Ok(Some(DecodedRangeRecord {
            range,
            sequence,
            observed,
        }))
    }
}

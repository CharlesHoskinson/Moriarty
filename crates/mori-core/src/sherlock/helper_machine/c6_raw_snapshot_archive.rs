//! Raw `HostSnapshot` value archive: a complete, non-validating binary codec
//! and its diagnostic inverse. Every available field is written, including
//! values the owner or `encode_host` would refuse (contradictory counters,
//! reserved markers, out-of-range indices); nothing is checked, normalized or
//! dropped. The inverse yields archived data only. It is not an `Admitted`,
//! `MachineImage`, owner observation or provenance claim.
//!
//! Format `C6RAWHS1`: the 8-byte magic, then the value. Integers are
//! little-endian (`usize` as u64, `u128` as 16 bytes); `bool` and `Option` are
//! a 0|1 tag; `String` and `Vec` are a u64 count then UTF-8 bytes or items;
//! structs are their fields in the order listed below; enums are a tag byte then
//! fields. `TypeExprSrc` is prefix order (`Apply` = tag, name, arg count, args),
//! encoded and decoded without recursion. The reader refuses truncation,
//! trailing bytes, unknown tags, invalid UTF-8, and any count beyond the
//! remaining input or the explicit budgets, before allocating. Every enum tag,
//! unit or data, is an explicit literal in the tables below rather than a
//! declaration discriminant, and struct and variant fields follow table order,
//! so editing a table changes the format; `golden_bytes_pin_the_format` pins
//! hand-written bytes. The magic carries no schema fingerprint.
//!
//! Resource scope: the budgets on `RawSnapshotWriteBudgets` and
//! `RawSnapshotReadBudgets` bound only this codec's own logical capacity
//! requests. They exclude allocator rounding, metadata and realloc transients,
//! the native stack (codec recursion is fixed by the type definitions, not the
//! input), and allocations outside the codec. In particular `TypeExprSrc::drop`,
//! run on refusal paths for partial trees and whenever the caller drops a
//! decoded value, grows an unbudgeted worklist of up to about twice the dropped
//! tree's nodes (at most `max_type_nodes` for a decoded tree) times
//! `size_of::<TypeExprSrc>()` at a time, and the `Clone`, `PartialEq` and `Debug`
//! impls in `complete_observation` use unbudgeted worklists proportional to tree
//! size. Those are iterative per their source, but an allocation failure in them
//! aborts instead of refusing. No other consumer is known to be iterative, so
//! `max_type_depth` and `max_type_nodes` must also suit every consumer of the
//! decoded value. Events, owner errors, config and progress are out of scope.

use std::mem::size_of;

use super::complete_observation::{
    AdmissionSnapshot, ConstFieldsSnapshot, ConstantSnapshot, DataFieldsSnapshot, DataSnapshot,
    ExpressionSnapshot, HostSnapshot, ImageSnapshot, LayoutSnapshot, ProgramSnapshot,
    WidthsSnapshot,
};
use super::shared::adt::{CtorIdC, TypeIdC, TypeRowC};
use super::shared::adt_storage::{CtorFieldRowExt, CtorRowExt};
use super::shared::host_implied_prefix::SegBase;
use super::shared::owner_ext_type_builder::{CtorDeclSrc, CtorFieldSrc, TypeDeclSrc, TypeExprSrc};
use super::shared::{
    ActiveLengths, AllocCounters, AppendPhase, ArmRowC, BinaryOp, BindPhase, BindStatus,
    BoolSelector, BoundBinding, BoundsC, BuildPhase, CaseMatching, ControlC, DigestC, EnvCellC,
    ExhaustReason, FinanceInput, FrameRowC, FunctionRowC, GrammarDomain, HelperCommon,
    HelperControlC, HelperResultC, HelperTaskC, IdentityC, LayoutCapacitiesC, MatchPhase, NodeKind,
    OccurrenceRowC, OutcomeC, ParameterRowC, PatternRowC, PatternTag, PlanAfterWalk, PlanNodeC,
    PlanPhase, PrimOutcome, RefC, RunIdC, RunIntC, ScalarMode, ScalarPhase, ScalarStaged, SiteTag,
    SliceC, TaskVisitC, TriStatus, UnaryOp,
};

pub const RAW_HOST_SNAPSHOT_MAGIC: [u8; 8] = *b"C6RAWHS1";

// `usize` is written as u64; this keeps that cast lossless.
const _: () = assert!(usize::BITS <= 64);

/// Encoder refusal. The `appended()` bytes written before it are a genuine but
/// incomplete prefix; only `Ok` means a complete archive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodeError {
    /// The next primitive of `refused` bytes would pass `limit`; it was not written.
    ByteLimit {
        limit: usize,
        appended: usize,
        refused: usize,
    },
    /// The next `TypeExprSrc` node would pass `max_type_nodes`.
    TypeNodeBudget { appended: usize },
    /// The next `TypeExprSrc` node would be deeper than `max_type_depth`.
    TypeDepthBudget { appended: usize },
    /// Growing the type-walk stack would pass `max_walk_bytes`.
    WalkBudget { appended: usize },
    /// An exact reservation failed or was granted a capacity other than the
    /// request, or the output had no reserved room left.
    Reserve { appended: usize },
}

impl EncodeError {
    pub fn appended(&self) -> usize {
        match *self {
            Self::ByteLimit { appended, .. }
            | Self::TypeNodeBudget { appended }
            | Self::TypeDepthBudget { appended }
            | Self::WalkBudget { appended }
            | Self::Reserve { appended } => appended,
        }
    }
}

/// Explicit finite writer budgets. The writer runs two identical passes
/// (measure, then write) and each pass is bounded by all of them independently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawSnapshotWriteBudgets {
    /// Appended bytes `L`, magic included; bytes already in `out` are not
    /// charged. `out` gets one `try_reserve_exact(L)`: when growth is needed its
    /// capacity must become exactly `out.len() + L` (which may exceed this limit,
    /// and the reallocation moves the existing bytes), and the write pass never
    /// grows it again. After a `Reserve` refusal from that step the capacity may
    /// have changed; the contents have not.
    pub max_output_bytes: usize,
    /// Cumulative `TypeExprSrc` nodes visited per pass.
    pub max_type_nodes: usize,
    /// Maximum `TypeExprSrc` nesting; a root node has depth 1. The walk stack
    /// never holds more than this many frames.
    pub max_type_depth: usize,
    /// Cumulative walk-stack bytes requested per pass: each growth is charged
    /// at its full new capacity before it is requested, never refunded.
    pub max_walk_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodeError {
    pub offset: usize,
    pub kind: DecodeErrorKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeErrorKind {
    InputTooLarge { len: usize, max: usize },
    BadMagic,
    UnexpectedEnd { needed: usize, available: usize },
    InvalidTag { tag: u8 },
    InvalidUtf8 { valid_up_to: usize },
    UsizeOverflow { value: u64 },
    LengthExceedsInput { len: u64 },
    ElementBudget { len: u64 },
    AllocBudget { len: u64 },
    TypeNodeBudget,
    TypeDepthBudget,
    Reserve,
    TrailingBytes { count: usize },
}

/// Explicit finite reader budgets, each checked before the allocation it bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawSnapshotReadBudgets {
    /// Whole archive length, checked before any read.
    pub max_input_bytes: usize,
    /// Cumulative `Vec` items plus `String` bytes.
    pub max_elements: usize,
    /// Cumulative bytes of every capacity the reader requests, never refunded:
    /// each exact `Vec`/`String` reservation (count times item size) and each
    /// open-`Apply` stack growth at its full new capacity. A grant other than
    /// the request refuses as `Reserve`. Drop-time `TypeExprSrc` worklists, on
    /// refusal or when the caller drops the result, are outside this budget;
    /// see the module docs.
    pub max_alloc_bytes: usize,
    /// Cumulative `TypeExprSrc` nodes across the whole snapshot.
    pub max_type_nodes: usize,
    /// Maximum `TypeExprSrc` nesting; a root node has depth 1.
    pub max_type_depth: usize,
}

/// Archived raw value only. It was never revalidated and is not evidence that
/// any owner observed it.
#[derive(Debug)]
pub struct DecodedRawHostSnapshot {
    snapshot: HostSnapshot,
}

impl DecodedRawHostSnapshot {
    pub fn snapshot(&self) -> &HostSnapshot {
        &self.snapshot
    }
    pub fn into_unattested_snapshot(self) -> HostSnapshot {
        self.snapshot
    }
}

/// Appends the archive of `snapshot` to `out` within `budgets` and returns the
/// appended length. Bytes already in `out` are untouched. A first pass writes
/// nothing and measures what the second will append (the whole archive, or the
/// prefix before the first refusal); `out` gets one exact reservation of that
/// length, and the second pass refuses rather than let `out` grow again.
pub fn encode_raw_host_snapshot(
    snapshot: &HostSnapshot,
    budgets: RawSnapshotWriteBudgets,
    out: &mut Vec<u8>,
) -> Result<usize, EncodeError> {
    encode_exact(budgets, out, &|wr| {
        wr.bytes(&RAW_HOST_SNAPSHOT_MAGIC)?;
        snapshot.put(wr)
    })
}

/// Decodes exactly one archive occupying all of `bytes`.
pub fn decode_raw_host_snapshot(
    bytes: &[u8],
    budgets: RawSnapshotReadBudgets,
) -> Result<DecodedRawHostSnapshot, DecodeError> {
    if bytes.len() > budgets.max_input_bytes {
        return Err(DecodeError {
            offset: 0,
            kind: DecodeErrorKind::InputTooLarge {
                len: bytes.len(),
                max: budgets.max_input_bytes,
            },
        });
    }
    let mut rd = R::new(bytes, budgets);
    if rd.arr::<8>()? != RAW_HOST_SNAPSHOT_MAGIC {
        return Err(DecodeError {
            offset: 0,
            kind: DecodeErrorKind::BadMagic,
        });
    }
    let snapshot = <HostSnapshot as Raw>::get(&mut rd)?;
    rd.finish()?;
    Ok(DecodedRawHostSnapshot { snapshot })
}

type Enc = Result<(), EncodeError>;
type Dec<T> = Result<T, DecodeError>;

fn encode_exact(
    budgets: RawSnapshotWriteBudgets,
    out: &mut Vec<u8>,
    put: &dyn Fn(&mut W<'_>) -> Enc,
) -> Result<usize, EncodeError> {
    let len = match encode_pass(budgets, None, put) {
        Ok(n) => n,
        Err(e) => e.appended(),
    };
    let refuse = EncodeError::Reserve { appended: 0 };
    let Some(need) = out.len().checked_add(len) else {
        return Err(refuse);
    };
    let had = out.capacity();
    if out.try_reserve_exact(len).is_err() || (had < need && out.capacity() != need) {
        return Err(refuse);
    }
    encode_pass(budgets, Some(out), put)
}

fn encode_pass(
    budgets: RawSnapshotWriteBudgets,
    out: Option<&mut Vec<u8>>,
    put: &dyn Fn(&mut W<'_>) -> Enc,
) -> Result<usize, EncodeError> {
    let mut wr = W {
        out,
        appended: 0,
        budgets,
        nodes: 0,
        walk: 0,
    };
    put(&mut wr)?;
    Ok(wr.appended)
}

type WalkFrame<'a> = std::slice::Iter<'a, TypeExprSrc>;

/// `out` is `None` in the measuring pass.
struct W<'a> {
    out: Option<&'a mut Vec<u8>>,
    appended: usize,
    budgets: RawSnapshotWriteBudgets,
    nodes: usize,
    walk: usize,
}

impl W<'_> {
    fn bytes(&mut self, b: &[u8]) -> Enc {
        let (appended, limit) = (self.appended, self.budgets.max_output_bytes);
        if b.len() > limit - appended {
            return Err(EncodeError::ByteLimit {
                limit,
                appended,
                refused: b.len(),
            });
        }
        if let Some(out) = self.out.as_deref_mut() {
            if out.capacity() - out.len() < b.len() {
                return Err(EncodeError::Reserve { appended });
            }
            out.extend_from_slice(b);
        }
        self.appended += b.len();
        Ok(())
    }
    fn type_node(&mut self, depth: usize) -> Enc {
        let appended = self.appended;
        if self.nodes >= self.budgets.max_type_nodes {
            return Err(EncodeError::TypeNodeBudget { appended });
        }
        if depth > self.budgets.max_type_depth {
            return Err(EncodeError::TypeDepthBudget { appended });
        }
        self.nodes += 1;
        Ok(())
    }
    /// Geometric growth capped at `max_type_depth` frames; the full new
    /// capacity is charged before it is requested.
    fn grow(&mut self, stack: &mut Vec<WalkFrame<'_>>) -> Enc {
        let appended = self.appended;
        let cap = stack.capacity();
        let new_cap = cap
            .saturating_mul(2)
            .max(4)
            .min(self.budgets.max_type_depth);
        match new_cap.checked_mul(size_of::<WalkFrame<'static>>()) {
            Some(bytes) if new_cap > cap && bytes <= self.budgets.max_walk_bytes - self.walk => {
                self.walk += bytes
            }
            _ => return Err(EncodeError::WalkBudget { appended }),
        }
        stack
            .try_reserve_exact(new_cap - stack.len())
            .map_err(|_| EncodeError::Reserve { appended })?;
        if stack.capacity() != new_cap {
            return Err(EncodeError::Reserve { appended });
        }
        Ok(())
    }
    fn u8(&mut self, v: u8) -> Enc {
        self.bytes(&[v])
    }
}

struct R<'a> {
    buf: &'a [u8],
    pos: usize,
    budgets: RawSnapshotReadBudgets,
    elements: usize,
    alloc: usize,
    nodes: usize,
}

impl<'a> R<'a> {
    fn new(buf: &'a [u8], budgets: RawSnapshotReadBudgets) -> Self {
        Self {
            buf,
            pos: 0,
            budgets,
            elements: 0,
            alloc: 0,
            nodes: 0,
        }
    }
    fn err(&self, kind: DecodeErrorKind) -> DecodeError {
        DecodeError {
            offset: self.pos,
            kind,
        }
    }
    fn bad_tag(&self, tag: u8) -> DecodeError {
        DecodeError {
            offset: self.pos.saturating_sub(1),
            kind: DecodeErrorKind::InvalidTag { tag },
        }
    }
    fn take(&mut self, n: usize) -> Dec<&'a [u8]> {
        let available = self.buf.len() - self.pos;
        if n > available {
            return Err(self.err(DecodeErrorKind::UnexpectedEnd {
                needed: n,
                available,
            }));
        }
        let (buf, start) = (self.buf, self.pos);
        self.pos += n;
        Ok(&buf[start..start + n])
    }
    fn u8(&mut self) -> Dec<u8> {
        Ok(self.take(1)?[0])
    }
    fn arr<const N: usize>(&mut self) -> Dec<[u8; N]> {
        let mut a = [0u8; N];
        a.copy_from_slice(self.take(N)?);
        Ok(a)
    }
    /// A u64 count of items of `size` bytes in memory, checked against the
    /// remaining input (every item encodes to at least one byte) and budgets.
    fn count(&mut self, size: usize) -> Dec<usize> {
        let at = self.pos;
        let len = u64::from_le_bytes(self.arr()?);
        let refuse = |kind| Err(DecodeError { offset: at, kind });
        if len > usize::MAX as u64 {
            return refuse(DecodeErrorKind::UsizeOverflow { value: len });
        }
        let n = len as usize;
        if n > self.buf.len() - self.pos {
            return refuse(DecodeErrorKind::LengthExceedsInput { len });
        }
        if n > self.budgets.max_elements - self.elements {
            return refuse(DecodeErrorKind::ElementBudget { len });
        }
        match n.checked_mul(size) {
            Some(bytes) if bytes <= self.budgets.max_alloc_bytes - self.alloc => {
                self.elements += n;
                self.alloc += bytes;
                Ok(n)
            }
            _ => refuse(DecodeErrorKind::AllocBudget { len }),
        }
    }
    fn charge(&mut self, len: usize, size: usize) -> Dec<()> {
        match len.checked_mul(size) {
            Some(bytes) if bytes <= self.budgets.max_alloc_bytes - self.alloc => {
                self.alloc += bytes;
                Ok(())
            }
            _ => Err(self.err(DecodeErrorKind::AllocBudget { len: len as u64 })),
        }
    }
    /// Exact reservation of a capacity already charged; any other grant refuses.
    fn reserve<T>(&self, v: &mut Vec<T>, cap: usize) -> Dec<()> {
        v.try_reserve_exact(cap - v.len())
            .map_err(|_| self.err(DecodeErrorKind::Reserve))?;
        if size_of::<T>() != 0 && v.capacity() != cap {
            return Err(self.err(DecodeErrorKind::Reserve));
        }
        Ok(())
    }
    /// Geometric type-stack growth capped at `max_type_depth` frames; the full
    /// new capacity is charged before it is requested.
    fn grow<T>(&mut self, stack: &mut Vec<T>) -> Dec<()> {
        let cap = stack.capacity();
        let new_cap = cap
            .saturating_mul(2)
            .max(4)
            .min(self.budgets.max_type_depth);
        if new_cap <= cap {
            return Err(self.err(DecodeErrorKind::TypeDepthBudget));
        }
        self.charge(new_cap, size_of::<T>())?;
        self.reserve(stack, new_cap)
    }
    fn type_node(&mut self, depth: usize) -> Dec<()> {
        if self.nodes >= self.budgets.max_type_nodes {
            return Err(self.err(DecodeErrorKind::TypeNodeBudget));
        }
        if depth > self.budgets.max_type_depth {
            return Err(self.err(DecodeErrorKind::TypeDepthBudget));
        }
        self.nodes += 1;
        Ok(())
    }
    fn finish(&self) -> Dec<()> {
        match self.buf.len() - self.pos {
            0 => Ok(()),
            count => Err(self.err(DecodeErrorKind::TrailingBytes { count })),
        }
    }
}

/// Field-exact archive form; the compiler rejects any omitted field or variant.
trait Raw: Sized {
    fn put(&self, wr: &mut W<'_>) -> Enc;
    fn get(rd: &mut R<'_>) -> Dec<Self>;
}

impl Raw for u8 {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        wr.u8(*self)
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        rd.u8()
    }
}
impl Raw for u32 {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        wr.bytes(&self.to_le_bytes())
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        Ok(u32::from_le_bytes(rd.arr()?))
    }
}
impl Raw for u128 {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        wr.bytes(&self.to_le_bytes())
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        Ok(u128::from_le_bytes(rd.arr()?))
    }
}
impl Raw for usize {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        wr.bytes(&(*self as u64).to_le_bytes())
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        let at = rd.pos;
        let value = u64::from_le_bytes(rd.arr()?);
        if value > usize::MAX as u64 {
            return Err(DecodeError {
                offset: at,
                kind: DecodeErrorKind::UsizeOverflow { value },
            });
        }
        Ok(value as usize)
    }
}
impl Raw for bool {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        wr.u8(u8::from(*self))
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        match rd.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            tag => Err(rd.bad_tag(tag)),
        }
    }
}
impl Raw for String {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        self.len().put(wr)?;
        wr.bytes(self.as_bytes())
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        let n = rd.count(1)?;
        let at = rd.pos;
        let src = rd.take(n)?;
        let mut v = Vec::new();
        rd.reserve(&mut v, n)?;
        v.extend_from_slice(src);
        String::from_utf8(v).map_err(|e| DecodeError {
            offset: at,
            kind: DecodeErrorKind::InvalidUtf8 {
                valid_up_to: e.utf8_error().valid_up_to(),
            },
        })
    }
}
impl<T: Raw> Raw for Vec<T> {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        self.len().put(wr)?;
        for x in self {
            x.put(wr)?;
        }
        Ok(())
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        let n = rd.count(size_of::<T>())?;
        let mut v = Vec::new();
        rd.reserve(&mut v, n)?;
        for _ in 0..n {
            v.push(T::get(rd)?);
        }
        Ok(v)
    }
}
impl<T: Raw> Raw for Option<T> {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        match self {
            None => wr.u8(0),
            Some(x) => {
                wr.u8(1)?;
                x.put(wr)
            }
        }
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        match rd.u8()? {
            0 => Ok(None),
            1 => Ok(Some(T::get(rd)?)),
            tag => Err(rd.bad_tag(tag)),
        }
    }
}
impl<A: Raw, B: Raw> Raw for (A, B) {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        self.0.put(wr)?;
        self.1.put(wr)
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        let a = A::get(rd)?;
        Ok((a, B::get(rd)?))
    }
}
impl Raw for TypeIdC {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        self.0.put(wr)
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        Ok(Self(Raw::get(rd)?))
    }
}
impl Raw for CtorIdC {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        self.0.put(wr)
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        Ok(Self(Raw::get(rd)?))
    }
}
impl Raw for RefC {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        match self {
            RefC::Const(i) => {
                wr.u8(0)?;
                i.put(wr)
            }
            RefC::Data(i) => {
                wr.u8(1)?;
                i.put(wr)
            }
        }
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        match rd.u8()? {
            0 => Ok(RefC::Const(Raw::get(rd)?)),
            1 => Ok(RefC::Data(Raw::get(rd)?)),
            tag => Err(rd.bad_tag(tag)),
        }
    }
}
impl Raw for RunIntC {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        match self {
            RunIntC::Negative => wr.u8(0),
            RunIntC::Nat(n) => {
                wr.u8(1)?;
                n.put(wr)
            }
            RunIntC::NatAboveU128 => wr.u8(2),
        }
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        match rd.u8()? {
            0 => Ok(RunIntC::Negative),
            1 => Ok(RunIntC::Nat(Raw::get(rd)?)),
            2 => Ok(RunIntC::NatAboveU128),
            tag => Err(rd.bad_tag(tag)),
        }
    }
}
// Private fields; the three public getters and `from_parts` cover all of them.
impl Raw for SegBase {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        self.active().put(wr)?;
        self.alloc().put(wr)?;
        self.consumed_work().put(wr)
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        Ok(SegBase::from_parts(
            Raw::get(rd)?,
            Raw::get(rd)?,
            Raw::get(rd)?,
        ))
    }
}

type OpenApply = (String, usize, Vec<TypeExprSrc>);

impl Raw for TypeExprSrc {
    fn put(&self, wr: &mut W<'_>) -> Enc {
        // One frame per open `Apply`, so the stack is depth-sized, not breadth-sized.
        let mut stack: Vec<WalkFrame<'_>> = Vec::new();
        let mut next = Some(self);
        loop {
            let node = match next.take() {
                Some(node) => node,
                None => match stack.last_mut().map(Iterator::next) {
                    None => return Ok(()),
                    Some(Some(node)) => node,
                    Some(None) => {
                        stack.pop();
                        continue;
                    }
                },
            };
            wr.type_node(stack.len() + 1)?;
            match node {
                TypeExprSrc::UInt => wr.u8(0)?,
                TypeExprSrc::Bool => wr.u8(1)?,
                TypeExprSrc::List => wr.u8(2)?,
                TypeExprSrc::Tree => wr.u8(3)?,
                TypeExprSrc::Named { name } => {
                    wr.u8(4)?;
                    name.put(wr)?;
                }
                TypeExprSrc::Apply { name, args } => {
                    wr.u8(5)?;
                    name.put(wr)?;
                    args.len().put(wr)?;
                    if !args.is_empty() {
                        if stack.len() == stack.capacity() {
                            wr.grow(&mut stack)?;
                        }
                        stack.push(args.iter());
                    }
                }
                TypeExprSrc::Param { name } => {
                    wr.u8(6)?;
                    name.put(wr)?;
                }
            }
        }
    }
    fn get(rd: &mut R<'_>) -> Dec<Self> {
        let mut open: Vec<OpenApply> = Vec::new();
        loop {
            rd.type_node(open.len() + 1)?;
            let mut done = match rd.u8()? {
                0 => TypeExprSrc::UInt,
                1 => TypeExprSrc::Bool,
                2 => TypeExprSrc::List,
                3 => TypeExprSrc::Tree,
                4 => TypeExprSrc::Named {
                    name: Raw::get(rd)?,
                },
                5 => {
                    let name: String = Raw::get(rd)?;
                    let n = rd.count(size_of::<TypeExprSrc>())?;
                    if n > 0 {
                        let mut args: Vec<TypeExprSrc> = Vec::new();
                        rd.reserve(&mut args, n)?;
                        if open.len() == open.capacity() {
                            rd.grow(&mut open)?;
                        }
                        open.push((name, n, args));
                        continue;
                    }
                    TypeExprSrc::Apply {
                        name,
                        args: Vec::new(),
                    }
                }
                6 => TypeExprSrc::Param {
                    name: Raw::get(rd)?,
                },
                tag => return Err(rd.bad_tag(tag)),
            };
            loop {
                match open.last_mut() {
                    None => return Ok(done),
                    Some(top) if top.2.len() + 1 < top.1 => {
                        top.2.push(done);
                        break;
                    }
                    Some(_) => {}
                }
                if let Some((name, _, mut args)) = open.pop() {
                    args.push(done);
                    done = TypeExprSrc::Apply { name, args };
                }
            }
        }
    }
}

// Explicit tags, not discriminants: reordering a declaration cannot remap an
// archive, and `put` stays exhaustive over the declared variants.
macro_rules! raw_unit {
    ($($t:ident [$($tag:literal $v:ident)*])*) => {$(
        impl Raw for $t {
            fn put(&self, wr: &mut W<'_>) -> Enc {
                match self { $(Self::$v => wr.u8($tag),)* }
            }
            fn get(rd: &mut R<'_>) -> Dec<Self> {
                match rd.u8()? { $($tag => Ok(Self::$v),)* tag => Err(rd.bad_tag(tag)) }
            }
        }
    )*};
}

macro_rules! raw_enum {
    ($($t:ident { $($tag:literal $v:ident { $($f:ident),* })* })*) => {$(
        impl Raw for $t {
            fn put(&self, wr: &mut W<'_>) -> Enc {
                match self { $(Self::$v { $($f),* } => { wr.u8($tag)?; $($f.put(wr)?;)* })* }
                Ok(())
            }
            fn get(rd: &mut R<'_>) -> Dec<Self> {
                match rd.u8()? {
                    $($tag => Ok(Self::$v { $($f: Raw::get(rd)?),* }),)*
                    tag => Err(rd.bad_tag(tag)),
                }
            }
        }
    )*};
}

macro_rules! raw_struct {
    ($($t:ident { $($f:ident),* })*) => {$(
        impl Raw for $t {
            fn put(&self, wr: &mut W<'_>) -> Enc {
                let Self { $($f),* } = self;
                $($f.put(wr)?;)*
                Ok(())
            }
            fn get(rd: &mut R<'_>) -> Dec<Self> { Ok(Self { $($f: Raw::get(rd)?),* }) }
        }
    )*};
}

raw_unit! {
    GrammarDomain [0 DedicatedSorts 1 GenericNominalExtensionReserved]
    FinanceInput [0 Absent 1 TypedExtensionReserved]
    BinaryOp [0 Add 1 Sub 2 Mul 3 Eq 4 Lt 5 Cons 6 Branch]
    UnaryOp [0 Leaf 1 Not]
    PatternTag [0 PVar 1 PNil 2 PCons 3 PLeaf 4 PBranch 5 WildcardExtensionReserved
        6 ConstructorExtensionReserved]
    ScalarMode [0 Add 1 Sub 2 Mul 3 Lt 4 Cons 5 Branch 6 Leaf 7 Not 8 TestBool 9 TestType]
    ScalarPhase [0 First 1 Second 2 Done]
    NodeKind [0 Cons 1 Leaf 2 Branch]
    BuildPhase [0 Copy 1 Done]
    PlanPhase [0 Walk 1 Test 2 Bind 3 Ready 4 NoMatchReady 5 Done]
    PlanAfterWalk [0 Test 1 Bind]
    BoolSelector [0 True 1 False 2 NotBool]
    TriStatus [0 Pending 1 Positive 2 Negative]
    BindPhase [0 Pairing 1 UnwindType 2 UnwindDup 3 UnwindInsert 4 Propagate 5 Finished]
    BindStatus [0 Ok 1 Bad]
    AppendPhase [0 Copy 1 Tail 2 Done]
    MatchPhase [0 Arm 1 Tag 2 Distinct 3 Field 4 Bind 5 Ready 6 NoMatchReady 7 Done]
    SiteTag [0 EvalLiteral 1 EvalVar 2 EvalLet 3 EvalBinary 4 EvalUnary 5 EvalIf 6 EvalMatch
        7 EvalCall 8 Push 9 RetLet 10 RetLeft 11 RetRight 12 RetUnary 13 RetIf 14 RetMatch
        15 RetArgs 16 RetCall 17 RetScope 18 Invoke 19 Enter 20 Bound 21 Cases 22 CaseResult
        23 Prim 24 EvalConstruct 25 RetCtorFields 26 Materialize]
    ExhaustReason [0 Fuel 1 DataNodes 2 DataFields 3 EnvironmentCells 4 HelperTasks 5 HelperWork
        6 SerializationBytes]
    TaskVisitC [0 Unvisited 1 Fields]
}

raw_enum! {
    DigestC { 0 Unselected {} 1 Selected { format, bytes } }
    ExpressionSnapshot {
        0 Literal { constant } 1 Var { binder } 2 Let { binder, value, body }
        3 Binary { op, left, right } 4 Unary { op, operand }
        5 If { cond, then_expr, else_expr } 6 Call { function_name, args }
        7 Match { scrutinee, arms, plan } 8 Construct { type_id, ctor_id, fields, constant }
    }
    ConstantSnapshot {
        0 U128 { marker_free, n } 1 Bool { marker_free, b } 2 Nil { marker_free }
        3 Cons { marker_free, fields } 4 Leaf { marker_free, fields }
        5 Branch { marker_free, fields } 6 Constructor { marker_free, type_id, ctor_id, fields }
        7 U128OutOfRange { marker_free }
    }
    DataSnapshot {
        0 U128 { n } 1 Bool { b } 2 Nil {} 3 Cons { fields } 4 Leaf { fields }
        5 Branch { fields } 6 Constructor { type_id, ctor_id, fields }
    }
    PatternRowC {
        0 PVar { x } 1 PNil {} 2 PCons { x, y } 3 PLeaf { x } 4 PBranch { x, y }
        5 Wildcard {} 6 Constructor { type_id, ctor_id, children }
    }
    PlanNodeC {
        0 Switch { source_match, path, type_id, cases, default }
        1 Bind { source_match, path, binder, next } 2 Select { source_match, arm }
        3 NoMatch { source_match }
    }
    BoundBinding { 0 Bad {} 1 Ok { env } }
    CaseMatching { 0 NoMatch {} 1 Matched { env } }
    PrimOutcome { 0 Value { value } 1 Error { code } }
    OutcomeC { 0 Running {} 1 Halted { value } 2 Rejected { code } 3 Exhausted { reason } }
    ScalarStaged {
        0 None {} 1 U128Value { n } 2 BoolValue { b } 3 PendingNode { tag }
        4 PrimError { code } 5 Selector { sel } 6 TypeTest { ok }
    }
    ControlC {
        0 Eval { expr } 1 Return { value } 2 ScheduleNext { frame, next, env }
        3 Invoke { function_name, args } 4 Enter { function, args }
        5 Bound { binding, return_type, body } 6 Cases { value, arms, saved_env }
        7 CaseResult { matching, value, body, rest, saved_env } 8 PrimResult { result }
        9 Error { code } 10 Materialize { type_id, ctor_id, fields }
    }
    FrameRowC {
        0 LetF { binder, body, saved_env, origin } 1 LeftF { op, right, saved_env, origin }
        2 RightF { op, left, origin } 3 UnaryF { op, origin }
        4 IfF { then_expr, else_expr, saved_env, origin } 5 MatchF { arms, saved_env, origin }
        6 ArgsF { function_name, completed, rest, saved_env, origin }
        7 ReturnF { return_type, saved_env, origin } 8 ScopeF { saved_env, origin }
        9 ConstructorFieldsF { type_id, ctor_id, index, field_type, checked, completed, rest,
            saved_env, origin }
    }
    HelperControlC {
        0 Scalar { common, mode, operand_a, operand_b, expected, phase, reg_a, staged }
        1 EnvGet { common, binder, cursor, found, status }
        2 EnvPut { common, binder, value, base, new_root, status }
        3 Find { common, function_name, row_cursor, found, status }
        4 Bind { common, function, params, args, p_cursor, a_cursor, partial, dup_cursor, phase,
            status }
        5 Append { common, src, value, src_cursor, dest_start, staged, phase }
        6 Validate { common, root, status }
        7 Equal { common, left, right, status }
        8 Match { common, scrutinee, arm, saved_env, partial, pat_tag, x, y, field_idx, staged,
            phase, status }
        9 Plan { common, source_match, scrutinee, saved_env, partial, node, phase, after_walk, cur,
            path, path_cursor, arm, status }
        10 Build { common, type_id, ctor_id, src, src_cursor, field_start, staged, node, phase }
    }
    HelperTaskC {
        0 ValidateTask { r, visit, cursor } 1 EqualTask { left, right, visit, cursor }
        2 PairTask { binder, sort, arg }
    }
    HelperResultC {
        0 Validated { ok } 1 EnvFound { value } 2 EnvMissing {} 3 EnvUpdated { cell }
        4 FindFound { function } 5 FindMissing {} 6 BindOk { env } 7 BindBad {}
        8 AppendDone { seq } 9 ScalarValue { value } 10 ScalarError { code }
        11 ScalarSelector { sel } 12 ScalarTypeTest { ok } 13 EqualDone { value }
        14 MatchMatched { env } 15 MatchNoMatch {} 16 PlanMatched { arm, env }
        17 PlanNoMatch {} 18 BuildDone { value }
    }
}

raw_struct! {
    HostSnapshot { admission, image }
    AdmissionSnapshot { identity, bounds, view, layout }
    IdentityC { profile_id, grammar_domain, finance, layout_version, layout_digest, bounds_digest,
        field_id, program_membership_digest }
    BoundsC { code_rows, function_rows, code_child_slots, parameter_slots, pattern_rows,
        match_plan_rows, match_arm_rows, type_rows, constructor_rows, constructor_field_slots,
        max_arity, constant_nodes, constant_field_slots, data_nodes, data_field_slots,
        environment_cells, frames, helper_tasks, serialization_bytes, max_reference_work,
        max_helper_work, step_cap }
    ProgramSnapshot { entry, expressions, occurrences, occurrence_paths, children, functions,
        parameters, arms, patterns, pattern_children, types, type_args, constructors, ctor_fields,
        decl_param_counts, decl_ctor_counts, decl_templates, plan_nodes, plan_cases, field_paths,
        constants, constant_fields, names }
    LayoutSnapshot { layout_version, field_id, widths, scalar_layout_id, bounds, capacities,
        boundary_capacities_present }
    WidthsSnapshot { index_widths, counter_widths, record_widths }
    LayoutCapacitiesC { name_rows, name_bytes, occurrence_path_slots, value_slots }
    OccurrenceRowC { owner_is_function, function, path }
    FunctionRowC { function_name, params, return_type, body }
    ParameterRowC { binder, sort }
    ArmRowC { pattern, body }
    TypeRowC { name_id, decl, args, ctor_start, ctor_len }
    CtorRowExt { owning_type, name_id, source_order, field_start, field_len }
    CtorFieldRowExt { name_id, type_id }
    TypeDeclSrc { name, parameters, ctors }
    CtorDeclSrc { name, fields }
    CtorFieldSrc { name, ty }
    ConstFieldsSnapshot { start, len }
    DataFieldsSnapshot { start, len }
    SliceC { start, len }
    ImageSnapshot { run_id, control, control_origin, env_root, stack_limit, outcome, initial_fuel,
        remaining_work, consumed_work, helper_work, helper_visits, reference_cursor, alloc, active,
        helper, result, seg_base, data, data_fields, env, frames, tasks, value_slots }
    RunIdC { identity, fuel_input, stack_limit_input }
    AllocCounters { data_nodes, data_fields, environment_cells }
    ActiveLengths { data, data_fields, env, value_slots }
    HelperCommon { site, origin_reference_cursor, parent_resume }
    EnvCellC { binder, value, previous }
}

#[cfg(test)]
mod tests {
    use super::DecodeErrorKind as DK;
    use super::*;

    const M: usize = usize::MAX;
    const Q: u128 = u128::MAX;

    fn big() -> RawSnapshotReadBudgets {
        RawSnapshotReadBudgets {
            max_input_bytes: 1 << 26,
            max_elements: 1 << 24,
            max_alloc_bytes: 1 << 30,
            max_type_nodes: 1 << 20,
            max_type_depth: 1 << 20,
        }
    }
    fn wbig() -> RawSnapshotWriteBudgets {
        RawSnapshotWriteBudgets {
            max_output_bytes: usize::MAX,
            max_type_nodes: 1 << 20,
            max_type_depth: 1 << 20,
            max_walk_bytes: 1 << 30,
        }
    }
    fn enc_with<T: Raw>(
        v: &T,
        budgets: RawSnapshotWriteBudgets,
        out: &mut Vec<u8>,
    ) -> Result<usize, EncodeError> {
        encode_exact(budgets, out, &|wr| v.put(wr))
    }
    fn enc<T: Raw>(v: &T) -> Vec<u8> {
        let mut out = Vec::new();
        enc_with(v, wbig(), &mut out).unwrap();
        out
    }
    fn dec<T: Raw>(b: &[u8], budgets: RawSnapshotReadBudgets) -> Result<T, DecodeError> {
        let mut rd = R::new(b, budgets);
        let v = T::get(&mut rd)?;
        rd.finish()?;
        Ok(v)
    }
    fn kind<T: Raw>(b: &[u8], budgets: RawSnapshotReadBudgets) -> Option<DecodeErrorKind> {
        dec::<T>(b, budgets).err().map(|e| e.kind)
    }
    fn rt<T: Raw + PartialEq + std::fmt::Debug>(v: &T) {
        let b = enc(v);
        assert_eq!(dec::<T>(&b, big()).as_ref(), Ok(v));
        assert_eq!(enc(&dec::<T>(&b, big()).unwrap()), b);
    }
    fn units<T: Raw>(count: u16) {
        for t in 0..=u8::MAX {
            match dec::<T>(&[t], big()) {
                Ok(v) => {
                    assert!(u16::from(t) < count);
                    assert_eq!(enc(&v), [t]);
                }
                Err(e) => {
                    assert!(u16::from(t) >= count);
                    assert_eq!(e.kind, DK::InvalidTag { tag: t });
                }
            }
        }
    }
    fn bytes(s: &HostSnapshot) -> Vec<u8> {
        let mut out = Vec::new();
        encode_raw_host_snapshot(s, wbig(), &mut out).unwrap();
        out
    }
    fn odd() -> String {
        [
            'λ',
            '→',
            '😀',
            char::from(0),
            char::from(10),
            '"',
            char::MAX,
        ]
        .iter()
        .collect()
    }
    fn sl(start: usize, len: usize) -> SliceC {
        SliceC { start, len }
    }
    fn ident(tag: &str) -> IdentityC {
        IdentityC {
            profile_id: u8::MAX,
            grammar_domain: GrammarDomain::GenericNominalExtensionReserved,
            finance: FinanceInput::TypedExtensionReserved,
            layout_version: (tag.into(), odd()),
            layout_digest: DigestC::Unselected,
            bounds_digest: DigestC::Selected {
                format: (odd(), String::new()),
                bytes: vec![0, 255, 7],
            },
            field_id: None,
            program_membership_digest: DigestC::Selected {
                format: ("x".into(), tag.into()),
                bytes: vec![],
            },
        }
    }

    /// Invalid for the owner and `encode_host`: contradictory counters and
    /// bounds, reserved markers, unsorted widths, out-of-range indices.
    fn sample() -> HostSnapshot {
        use ConstantSnapshot as K;
        use DataSnapshot as D;
        use ExpressionSnapshot as E;
        use FrameRowC as F;
        use PatternRowC as P;
        use PlanNodeC as PN;
        let cf = ConstFieldsSnapshot { start: M, len: 1 };
        let df = DataFieldsSnapshot { start: 0, len: M };
        let ty = TypeExprSrc::Apply {
            name: "Pair".into(),
            args: vec![
                TypeExprSrc::UInt,
                TypeExprSrc::Bool,
                TypeExprSrc::List,
                TypeExprSrc::Tree,
                TypeExprSrc::Named { name: odd() },
                TypeExprSrc::Param { name: "α".into() },
                TypeExprSrc::Apply {
                    name: String::new(),
                    args: vec![],
                },
            ],
        };
        let view = ProgramSnapshot {
            entry: M,
            expressions: vec![
                E::Literal { constant: M },
                E::Var { binder: 0 },
                E::Let {
                    binder: 1,
                    value: 2,
                    body: M,
                },
                E::Binary {
                    op: BinaryOp::Branch,
                    left: M,
                    right: 0,
                },
                E::Unary {
                    op: UnaryOp::Not,
                    operand: 9,
                },
                E::If {
                    cond: 1,
                    then_expr: 2,
                    else_expr: M,
                },
                E::Call {
                    function_name: M,
                    args: sl(M, 0),
                },
                E::Match {
                    scrutinee: 0,
                    arms: sl(1, M),
                    plan: Some(M),
                },
                E::Construct {
                    type_id: TypeIdC(M),
                    ctor_id: CtorIdC(M),
                    fields: sl(0, 0),
                    constant: None,
                },
            ],
            occurrences: vec![OccurrenceRowC {
                owner_is_function: true,
                function: M,
                path: sl(3, 4),
            }],
            occurrence_paths: vec![M, 0],
            children: vec![1],
            functions: vec![FunctionRowC {
                function_name: 0,
                params: sl(0, 1),
                return_type: TypeIdC(77),
                body: M,
            }],
            parameters: vec![ParameterRowC {
                binder: M,
                sort: TypeIdC::Tree,
            }],
            arms: vec![ArmRowC {
                pattern: M,
                body: 0,
            }],
            patterns: vec![
                P::PVar { x: 0 },
                P::PNil,
                P::PCons { x: 1, y: M },
                P::PLeaf { x: 2 },
                P::PBranch { x: 3, y: 4 },
                P::Wildcard,
                P::Constructor {
                    type_id: TypeIdC(5),
                    ctor_id: CtorIdC(6),
                    children: sl(7, 8),
                },
            ],
            pattern_children: vec![],
            types: vec![TypeRowC {
                name_id: M,
                decl: Some(0),
                args: sl(0, 2),
                ctor_start: CtorIdC(M),
                ctor_len: 3,
            }],
            type_args: vec![TypeIdC::UInt, TypeIdC(M)],
            constructors: vec![CtorRowExt {
                owning_type: TypeIdC(9),
                name_id: 1,
                source_order: M,
                field_start: 2,
                field_len: M,
            }],
            ctor_fields: vec![CtorFieldRowExt {
                name_id: 0,
                type_id: TypeIdC(M),
            }],
            decl_param_counts: vec![M],
            decl_ctor_counts: vec![0, 2],
            decl_templates: vec![TypeDeclSrc {
                name: "Pair".into(),
                parameters: vec!["α".into(), String::new()],
                ctors: vec![
                    CtorDeclSrc {
                        name: odd(),
                        fields: vec![CtorFieldSrc {
                            name: "fst".into(),
                            ty,
                        }],
                    },
                    CtorDeclSrc {
                        name: "Empty".into(),
                        fields: vec![],
                    },
                ],
            }],
            plan_nodes: vec![
                PN::Switch {
                    source_match: M,
                    path: sl(0, 1),
                    type_id: TypeIdC(2),
                    cases: sl(3, 4),
                    default: 5,
                },
                PN::Bind {
                    source_match: 0,
                    path: sl(1, 1),
                    binder: 2,
                    next: M,
                },
                PN::Select {
                    source_match: 1,
                    arm: M,
                },
                PN::NoMatch { source_match: M },
            ],
            plan_cases: vec![None, Some(M), Some(0)],
            field_paths: vec![4],
            constants: vec![
                K::U128 {
                    marker_free: true,
                    n: Q,
                },
                K::Bool {
                    marker_free: false,
                    b: true,
                },
                K::Nil { marker_free: true },
                K::Cons {
                    marker_free: false,
                    fields: cf,
                },
                K::Leaf {
                    marker_free: true,
                    fields: cf,
                },
                K::Branch {
                    marker_free: false,
                    fields: cf,
                },
                K::Constructor {
                    marker_free: true,
                    type_id: TypeIdC(M),
                    ctor_id: CtorIdC(0),
                    fields: cf,
                },
                K::U128OutOfRange { marker_free: false },
            ],
            constant_fields: vec![M],
            names: vec!["main".into(), odd(), String::new()],
        };
        let layout = LayoutSnapshot {
            layout_version: ("L".into(), odd()),
            field_id: Some(("f".into(), String::new())),
            widths: Some(WidthsSnapshot {
                index_widths: vec![("z".into(), u32::MAX), ("a".into(), 0)],
                counter_widths: vec![("WorkCounter".into(), 0)],
                record_widths: vec![],
            }),
            scalar_layout_id: None,
            bounds: BoundsC {
                data_nodes: 1,
                step_cap: Q,
                ..Default::default()
            },
            capacities: LayoutCapacitiesC {
                name_bytes: M,
                ..Default::default()
            },
            boundary_capacities_present: true,
        };
        let frame = F::ConstructorFieldsF {
            type_id: TypeIdC(M),
            ctor_id: CtorIdC(1),
            index: M,
            field_type: TypeIdC::List,
            checked: true,
            completed: sl(M, M),
            rest: sl(0, 0),
            saved_env: Some(M),
            origin: M,
        };
        let common = HelperCommon {
            site: SiteTag::Materialize,
            origin_reference_cursor: Q,
            parent_resume: Some(M),
        };
        let image = ImageSnapshot {
            run_id: RunIdC {
                identity: ident("run"),
                fuel_input: RunIntC::Nat(Q),
                stack_limit_input: RunIntC::NatAboveU128,
            },
            control: ControlC::ScheduleNext {
                frame,
                next: M,
                env: None,
            },
            control_origin: M,
            env_root: Some(M),
            stack_limit: 0,
            outcome: OutcomeC::Exhausted {
                reason: ExhaustReason::SerializationBytes,
            },
            initial_fuel: 1,
            remaining_work: Q,
            consumed_work: Q,
            helper_work: Q - 1,
            helper_visits: 0,
            reference_cursor: Q,
            alloc: AllocCounters {
                data_nodes: M,
                data_fields: 0,
                environment_cells: 3,
            },
            active: ActiveLengths {
                data: 0,
                data_fields: M,
                env: 1,
                value_slots: 99,
            },
            helper: Some(HelperControlC::Plan {
                common,
                source_match: M,
                scrutinee: RefC::Const(M),
                saved_env: None,
                partial: Some(0),
                node: M,
                phase: PlanPhase::NoMatchReady,
                after_walk: PlanAfterWalk::Bind,
                cur: RefC::Data(M),
                path: sl(1, 2),
                path_cursor: M,
                arm: Some(M),
                status: TriStatus::Negative,
            }),
            result: Some(HelperResultC::PlanMatched {
                arm: M,
                env: Some(0),
            }),
            seg_base: SegBase::from_parts(
                ActiveLengths {
                    data: M,
                    ..Default::default()
                },
                AllocCounters {
                    environment_cells: M,
                    ..Default::default()
                },
                Q,
            ),
            data: vec![
                D::U128 { n: Q },
                D::Bool { b: false },
                D::Nil,
                D::Cons { fields: df },
                D::Leaf { fields: df },
                D::Branch { fields: df },
                D::Constructor {
                    type_id: TypeIdC(M),
                    ctor_id: CtorIdC(M),
                    fields: df,
                },
            ],
            data_fields: vec![RefC::Data(M), RefC::Const(0)],
            env: vec![EnvCellC {
                binder: M,
                value: RefC::Data(5),
                previous: Some(M),
            }],
            frames: vec![
                F::LetF {
                    binder: M,
                    body: 0,
                    saved_env: None,
                    origin: M,
                },
                F::LeftF {
                    op: BinaryOp::Lt,
                    right: 1,
                    saved_env: Some(M),
                    origin: 0,
                },
                F::RightF {
                    op: BinaryOp::Cons,
                    left: RefC::Data(M),
                    origin: 2,
                },
                F::UnaryF {
                    op: UnaryOp::Leaf,
                    origin: M,
                },
                F::IfF {
                    then_expr: 1,
                    else_expr: M,
                    saved_env: None,
                    origin: 3,
                },
                F::MatchF {
                    arms: sl(M, 1),
                    saved_env: Some(0),
                    origin: 4,
                },
                F::ArgsF {
                    function_name: M,
                    completed: sl(0, M),
                    rest: sl(1, 1),
                    saved_env: None,
                    origin: 5,
                },
                F::ReturnF {
                    return_type: TypeIdC(M),
                    saved_env: Some(1),
                    origin: 6,
                },
                F::ScopeF {
                    saved_env: None,
                    origin: M,
                },
                frame,
            ],
            tasks: vec![
                HelperTaskC::ValidateTask {
                    r: RefC::Const(1),
                    visit: TaskVisitC::Fields,
                    cursor: M,
                },
                HelperTaskC::EqualTask {
                    left: RefC::Data(M),
                    right: RefC::Const(0),
                    visit: TaskVisitC::Unvisited,
                    cursor: 0,
                },
                HelperTaskC::PairTask {
                    binder: M,
                    sort: TypeIdC(M),
                    arg: RefC::Data(0),
                },
            ],
            value_slots: vec![RefC::Const(M)],
        };
        HostSnapshot {
            admission: AdmissionSnapshot {
                identity: ident("adm"),
                bounds: BoundsC {
                    code_rows: M,
                    max_reference_work: Q,
                    ..Default::default()
                },
                view,
                layout,
            },
            image,
        }
    }

    #[test]
    fn invalid_snapshot_roundtrips_by_field_value() {
        let s = sample();
        let mut out = vec![0xEE];
        let n = encode_raw_host_snapshot(&s, wbig(), &mut out).unwrap();
        assert_eq!((out[0], out.len()), (0xEE, n + 1));
        let d = decode_raw_host_snapshot(&out[1..], big()).unwrap();
        let v = d.snapshot();
        assert_eq!(v, &s);
        assert_eq!(
            (
                v.image.initial_fuel,
                v.image.remaining_work,
                v.image.consumed_work
            ),
            (1, Q, Q)
        );
        assert_eq!(
            (
                v.admission.view.entry,
                v.image.stack_limit,
                v.image.frames.len()
            ),
            (M, 0, 10)
        );
        assert_eq!(v.admission.view.names[1], odd());
        assert_eq!(
            v.admission.identity.layout_version.1.chars().last(),
            Some(char::MAX)
        );
        assert_eq!(
            (
                v.image.seg_base.consumed_work(),
                v.image.seg_base.active().data
            ),
            (Q, M)
        );
        assert_eq!(v.image.seg_base.alloc().environment_cells, M);
        assert!(v.admission.layout.boundary_capacities_present);
        assert_eq!(
            v.admission.layout.widths.as_ref().unwrap().index_widths[0],
            ("z".to_string(), u32::MAX)
        );
        assert_eq!(v.image.run_id.fuel_input, RunIntC::Nat(Q));
        assert_eq!(
            v.admission.view.constants[7],
            ConstantSnapshot::U128OutOfRange { marker_free: false }
        );
        assert_eq!(bytes(v), &out[1..]);
    }

    #[test]
    fn every_truncation_and_trailing_byte_refuses() {
        let b = bytes(&sample());
        for cut in 0..b.len() {
            let k = decode_raw_host_snapshot(&b[..cut], big()).unwrap_err().kind;
            assert!(
                matches!(k, DK::UnexpectedEnd { .. } | DK::LengthExceedsInput { .. }),
                "{cut}: {k:?}"
            );
        }
        let mut t = b.clone();
        t.push(0);
        assert_eq!(
            decode_raw_host_snapshot(&t, big()).unwrap_err().kind,
            DK::TrailingBytes { count: 1 }
        );
    }

    #[test]
    fn invalid_tags_utf8_and_magic_refuse() {
        fn tag<T: Raw>(t: u8) {
            assert_eq!(
                dec::<T>(&[t], big()).err(),
                Some(DecodeError {
                    offset: 0,
                    kind: DK::InvalidTag { tag: t }
                })
            );
        }
        tag::<bool>(2);
        tag::<Option<usize>>(2);
        tag::<RefC>(2);
        tag::<RunIntC>(3);
        tag::<DigestC>(2);
        tag::<ExpressionSnapshot>(9);
        tag::<ConstantSnapshot>(8);
        tag::<DataSnapshot>(7);
        tag::<PatternRowC>(7);
        tag::<PlanNodeC>(4);
        tag::<BoundBinding>(2);
        tag::<CaseMatching>(2);
        tag::<PrimOutcome>(2);
        tag::<OutcomeC>(4);
        tag::<ScalarStaged>(7);
        tag::<ControlC>(11);
        tag::<FrameRowC>(10);
        tag::<HelperControlC>(11);
        tag::<HelperTaskC>(3);
        tag::<HelperResultC>(19);
        tag::<TypeExprSrc>(7);
        let mut s = 2u64.to_le_bytes().to_vec();
        s.extend([0xCE, 0xFF]);
        assert_eq!(
            dec::<String>(&s, big()),
            Err(DecodeError {
                offset: 8,
                kind: DK::InvalidUtf8 { valid_up_to: 0 }
            })
        );
        assert_eq!(
            kind::<u8>(&[1, 2], big()),
            Some(DK::TrailingBytes { count: 1 })
        );
        let mut b = bytes(&sample());
        b[0] ^= 1;
        assert_eq!(
            decode_raw_host_snapshot(&b, big()).err(),
            Some(DecodeError {
                offset: 0,
                kind: DK::BadMagic
            })
        );
    }

    #[test]
    fn unit_enum_tags_are_exact() {
        units::<bool>(2);
        units::<GrammarDomain>(2);
        units::<FinanceInput>(2);
        units::<BinaryOp>(7);
        units::<UnaryOp>(2);
        units::<PatternTag>(7);
        units::<ScalarMode>(10);
        units::<ScalarPhase>(3);
        units::<NodeKind>(3);
        units::<BuildPhase>(2);
        units::<PlanPhase>(6);
        units::<PlanAfterWalk>(2);
        units::<BoolSelector>(3);
        units::<TriStatus>(3);
        units::<BindPhase>(6);
        units::<BindStatus>(2);
        units::<AppendPhase>(3);
        units::<MatchPhase>(8);
        units::<SiteTag>(27);
        units::<ExhaustReason>(7);
        units::<TaskVisitC>(2);
    }

    /// Hand-written format pin: `parts` are typed out here, never derived from
    /// the codec, its tag tables or any declaration order.
    fn g<T: Raw + PartialEq + std::fmt::Debug>(v: T, parts: &[&[u8]]) {
        let want = parts.concat();
        assert_eq!(enc(&v), want, "{v:?}");
        assert_eq!(dec::<T>(&want, big()), Ok(v));
    }

    #[test]
    fn golden_bytes_pin_the_format() {
        assert_eq!(&bytes(&sample())[..8], b"C6RAWHS1");
        g(GrammarDomain::GenericNominalExtensionReserved, &[&[1]]);
        g(FinanceInput::TypedExtensionReserved, &[&[1]]);
        g(BinaryOp::Branch, &[&[6]]);
        g(BinaryOp::Eq, &[&[3]]);
        g(UnaryOp::Not, &[&[1]]);
        g(PatternTag::ConstructorExtensionReserved, &[&[6]]);
        g(PatternTag::PLeaf, &[&[3]]);
        g(ScalarMode::TestType, &[&[9]]);
        g(ScalarMode::Lt, &[&[3]]);
        g(ScalarPhase::Done, &[&[2]]);
        g(NodeKind::Branch, &[&[2]]);
        g(BuildPhase::Done, &[&[1]]);
        g(PlanPhase::Done, &[&[5]]);
        g(PlanPhase::Ready, &[&[3]]);
        g(PlanAfterWalk::Bind, &[&[1]]);
        g(BoolSelector::NotBool, &[&[2]]);
        g(TriStatus::Negative, &[&[2]]);
        g(BindPhase::Finished, &[&[5]]);
        g(BindPhase::UnwindInsert, &[&[3]]);
        g(BindStatus::Bad, &[&[1]]);
        g(AppendPhase::Done, &[&[2]]);
        g(MatchPhase::Done, &[&[7]]);
        g(MatchPhase::Field, &[&[3]]);
        g(SiteTag::Materialize, &[&[26]]);
        g(SiteTag::Prim, &[&[23]]);
        g(SiteTag::Push, &[&[8]]);
        g(ExhaustReason::SerializationBytes, &[&[6]]);
        g(ExhaustReason::EnvironmentCells, &[&[3]]);
        g(TaskVisitC::Fields, &[&[1]]);
        g(true, &[&[1]]);
        g(0x0102_0304u32, &[&[4, 3, 2, 1]]);
        g(Some(2usize), &[&[1, 2], &[0; 7]]);
        g("λ".to_string(), &[&[2], &[0; 7], &[0xCE, 0xBB]]);
        g(RefC::Data(1), &[&[1, 1], &[0; 7]]);
        g(RefC::Const(0x0100), &[&[0, 0, 1], &[0; 6]]);
        g(RunIntC::Nat(1), &[&[1, 1], &[0; 15]]);
        g(RunIntC::NatAboveU128, &[&[2]]);
        g(
            OutcomeC::Exhausted {
                reason: ExhaustReason::HelperWork,
            },
            &[&[3, 5]],
        );
        g(ControlC::Error { code: 7 }, &[&[9, 7]]);
        g(
            HelperResultC::BuildDone {
                value: RefC::Const(2),
            },
            &[&[18, 0, 2], &[0; 7]],
        );
        g(
            DigestC::Selected {
                format: ("a".into(), String::new()),
                bytes: vec![9],
            },
            &[&[1, 1], &[0; 7], b"a", &[0; 8], &[1], &[0; 7], &[9]],
        );
        g(SliceC { start: 1, len: 2 }, &[&[1], &[0; 7], &[2], &[0; 7]]);
        g(
            EnvCellC {
                binder: 1,
                value: RefC::Data(2),
                previous: None,
            },
            &[&[1], &[0; 7], &[1, 2], &[0; 7], &[0]],
        );
        g(
            HelperCommon {
                site: SiteTag::Prim,
                origin_reference_cursor: 0x0102,
                parent_resume: None,
            },
            &[&[23, 2, 1], &[0; 14], &[0]],
        );
        g(
            CtorFieldSrc {
                name: "f".into(),
                ty: TypeExprSrc::Bool,
            },
            &[&[1], &[0; 7], b"f", &[1]],
        );
        g(
            TypeExprSrc::Named {
                name: String::new(),
            },
            &[&[4], &[0; 8]],
        );
        g(
            TypeExprSrc::Apply {
                name: "P".into(),
                args: vec![TypeExprSrc::UInt, TypeExprSrc::Param { name: "a".into() }],
            },
            &[
                &[5, 1],
                &[0; 7],
                b"P",
                &[2],
                &[0; 7],
                &[0, 6, 1],
                &[0; 7],
                b"a",
            ],
        );
    }

    #[test]
    fn every_data_variant_roundtrips() {
        use ControlC as C;
        use HelperControlC as H;
        use HelperResultC as HR;
        let c = HelperCommon {
            site: SiteTag::Push,
            origin_reference_cursor: Q,
            parent_resume: None,
        };
        let (a, d, s) = (RefC::Const(M), RefC::Data(0), sl(M, 0));
        for v in [
            H::Scalar {
                common: c,
                mode: ScalarMode::TestType,
                operand_a: a,
                operand_b: Some(d),
                expected: Some(TypeIdC(M)),
                phase: ScalarPhase::Second,
                reg_a: Q,
                staged: ScalarStaged::PrimError { code: 255 },
            },
            H::EnvGet {
                common: c,
                binder: M,
                cursor: Some(0),
                found: Some(a),
                status: TriStatus::Pending,
            },
            H::EnvPut {
                common: c,
                binder: 0,
                value: d,
                base: None,
                new_root: Some(M),
                status: TriStatus::Positive,
            },
            H::Find {
                common: c,
                function_name: M,
                row_cursor: 0,
                found: None,
                status: TriStatus::Negative,
            },
            H::Bind {
                common: c,
                function: M,
                params: s,
                args: s,
                p_cursor: 1,
                a_cursor: M,
                partial: Some(2),
                dup_cursor: None,
                phase: BindPhase::UnwindDup,
                status: BindStatus::Bad,
            },
            H::Append {
                common: c,
                src: s,
                value: a,
                src_cursor: 3,
                dest_start: M,
                staged: Some(d),
                phase: AppendPhase::Tail,
            },
            H::Validate {
                common: c,
                root: a,
                status: TriStatus::Pending,
            },
            H::Equal {
                common: c,
                left: a,
                right: d,
                status: TriStatus::Positive,
            },
            H::Match {
                common: c,
                scrutinee: d,
                arm: M,
                saved_env: Some(M),
                partial: None,
                pat_tag: PatternTag::ConstructorExtensionReserved,
                x: M,
                y: 0,
                field_idx: u8::MAX,
                staged: None,
                phase: MatchPhase::Distinct,
                status: TriStatus::Negative,
            },
            H::Plan {
                common: c,
                source_match: 0,
                scrutinee: a,
                saved_env: None,
                partial: None,
                node: 1,
                phase: PlanPhase::Walk,
                after_walk: PlanAfterWalk::Test,
                cur: d,
                path: s,
                path_cursor: 0,
                arm: None,
                status: TriStatus::Pending,
            },
            H::Build {
                common: c,
                type_id: TypeIdC(M),
                ctor_id: CtorIdC(M),
                src: s,
                src_cursor: M,
                field_start: 0,
                staged: Some(a),
                node: None,
                phase: BuildPhase::Copy,
            },
        ] {
            rt(&v);
        }
        for v in [
            ScalarStaged::None,
            ScalarStaged::U128Value { n: Q },
            ScalarStaged::BoolValue { b: true },
            ScalarStaged::PendingNode {
                tag: NodeKind::Branch,
            },
            ScalarStaged::PrimError { code: 0 },
            ScalarStaged::Selector {
                sel: BoolSelector::NotBool,
            },
            ScalarStaged::TypeTest { ok: false },
        ] {
            rt(&v);
        }
        for v in [
            C::Eval { expr: M },
            C::Return { value: a },
            C::ScheduleNext {
                frame: FrameRowC::ScopeF {
                    saved_env: None,
                    origin: 0,
                },
                next: 0,
                env: Some(M),
            },
            C::Invoke {
                function_name: M,
                args: s,
            },
            C::Enter {
                function: None,
                args: s,
            },
            C::Bound {
                binding: BoundBinding::Bad,
                return_type: TypeIdC(M),
                body: 0,
            },
            C::Bound {
                binding: BoundBinding::Ok { env: Some(M) },
                return_type: TypeIdC(0),
                body: M,
            },
            C::Cases {
                value: d,
                arms: s,
                saved_env: None,
            },
            C::CaseResult {
                matching: CaseMatching::NoMatch,
                value: a,
                body: M,
                rest: s,
                saved_env: Some(0),
            },
            C::CaseResult {
                matching: CaseMatching::Matched { env: None },
                value: d,
                body: 0,
                rest: s,
                saved_env: None,
            },
            C::PrimResult {
                result: PrimOutcome::Value { value: d },
            },
            C::PrimResult {
                result: PrimOutcome::Error { code: 255 },
            },
            C::Error { code: 0 },
            C::Materialize {
                type_id: TypeIdC(0),
                ctor_id: CtorIdC(M),
                fields: s,
            },
        ] {
            rt(&v);
        }
        for v in [
            HR::Validated { ok: true },
            HR::EnvFound { value: a },
            HR::EnvMissing,
            HR::EnvUpdated { cell: M },
            HR::FindFound { function: M },
            HR::FindMissing,
            HR::BindOk { env: None },
            HR::BindBad,
            HR::AppendDone { seq: s },
            HR::ScalarValue { value: d },
            HR::ScalarError { code: 255 },
            HR::ScalarSelector {
                sel: BoolSelector::True,
            },
            HR::ScalarTypeTest { ok: false },
            HR::EqualDone { value: a },
            HR::MatchMatched { env: Some(M) },
            HR::MatchNoMatch,
            HR::PlanMatched { arm: M, env: None },
            HR::PlanNoMatch,
            HR::BuildDone { value: d },
        ] {
            rt(&v);
        }
        for v in [
            OutcomeC::Running,
            OutcomeC::Halted { value: a },
            OutcomeC::Rejected { code: 255 },
            OutcomeC::Exhausted {
                reason: ExhaustReason::Fuel,
            },
        ] {
            rt(&v);
        }
        for v in [
            RunIntC::Negative,
            RunIntC::Nat(0),
            RunIntC::Nat(Q),
            RunIntC::NatAboveU128,
        ] {
            rt(&v);
        }
    }

    #[test]
    fn budgets_refuse_before_allocation() {
        let b = bytes(&sample());
        let k = |budgets| decode_raw_host_snapshot(&b, budgets).err().map(|e| e.kind);
        assert_eq!(
            k(RawSnapshotReadBudgets {
                max_input_bytes: b.len() - 1,
                ..big()
            }),
            Some(DK::InputTooLarge {
                len: b.len(),
                max: b.len() - 1
            })
        );
        assert!(matches!(
            k(RawSnapshotReadBudgets {
                max_elements: 3,
                ..big()
            }),
            Some(DK::ElementBudget { .. })
        ));
        assert!(matches!(
            k(RawSnapshotReadBudgets {
                max_alloc_bytes: 16,
                ..big()
            }),
            Some(DK::AllocBudget { .. })
        ));
        assert_eq!(
            k(RawSnapshotReadBudgets {
                max_type_nodes: 1,
                ..big()
            }),
            Some(DK::TypeNodeBudget)
        );
        assert_eq!(
            k(RawSnapshotReadBudgets {
                max_type_depth: 1,
                ..big()
            }),
            Some(DK::TypeDepthBudget)
        );
        assert_eq!(k(big()), None);
        assert_eq!(
            kind::<Vec<HelperControlC>>(&u64::MAX.to_le_bytes(), big()),
            Some(DK::LengthExceedsInput { len: u64::MAX })
        );
        assert_eq!(
            kind::<String>(&5u64.to_le_bytes(), big()),
            Some(DK::LengthExceedsInput { len: 5 })
        );
    }

    #[test]
    fn deep_type_expr_is_iterative_and_bounded() {
        const N: usize = 100_000;
        let mut t = TypeExprSrc::Param { name: "t".into() };
        for _ in 0..N {
            t = TypeExprSrc::Apply {
                name: "A".into(),
                args: vec![t],
            };
        }
        let b = enc(&t);
        let back = dec::<TypeExprSrc>(&b, big()).ok().expect("deep decode");
        assert_eq!(enc(&back), b);
        let (mut node, mut depth) = (&back, 0);
        while let TypeExprSrc::Apply { name, args } = node {
            assert_eq!((name.as_str(), args.len()), ("A", 1));
            node = &args[0];
            depth += 1;
        }
        assert!(matches!(node, TypeExprSrc::Param { name } if name == "t"));
        assert_eq!(depth, N);
        assert_eq!(
            kind::<TypeExprSrc>(
                &b,
                RawSnapshotReadBudgets {
                    max_type_depth: N,
                    ..big()
                }
            ),
            Some(DK::TypeDepthBudget)
        );
        assert_eq!(
            kind::<TypeExprSrc>(
                &b,
                RawSnapshotReadBudgets {
                    max_type_nodes: N,
                    ..big()
                }
            ),
            Some(DK::TypeNodeBudget)
        );
    }

    #[test]
    fn encode_byte_limit_keeps_genuine_prefix_in_exact_capacity() {
        let full = bytes(&sample());
        let wb = |max_output_bytes| RawSnapshotWriteBudgets {
            max_output_bytes,
            ..wbig()
        };
        let mut out = vec![0xEE];
        let e = encode_raw_host_snapshot(&sample(), wb(full.len() - 1), &mut out).unwrap_err();
        assert!(matches!(e, EncodeError::ByteLimit { limit, appended, .. }
            if limit == full.len() - 1 && appended + 1 == out.len()));
        assert!(out[0] == 0xEE && full.starts_with(&out[1..]) && out.len() <= full.len());
        assert_eq!(out.capacity(), out.len());
        let mut exact = Vec::new();
        assert_eq!(
            encode_raw_host_snapshot(&sample(), wb(full.len()), &mut exact),
            Ok(full.len())
        );
        assert_eq!((exact.capacity(), &exact), (full.len(), &full));
    }

    fn chain(k: usize) -> TypeExprSrc {
        let mut t = TypeExprSrc::Param { name: "t".into() };
        for _ in 0..k {
            t = TypeExprSrc::Apply {
                name: "A".into(),
                args: vec![t],
            };
        }
        t
    }

    #[test]
    fn encoder_type_and_walk_budgets_refuse_with_genuine_prefix() {
        // Each `A` node encodes to 18 bytes and the `t` leaf to 10.
        let frame = size_of::<WalkFrame<'static>>();
        let wb = |max_type_nodes, max_type_depth, max_walk_bytes| RawSnapshotWriteBudgets {
            max_output_bytes: usize::MAX,
            max_type_nodes,
            max_type_depth,
            max_walk_bytes,
        };
        let (t4, t2) = (chain(4), chain(2));
        for (t, budgets, want) in [
            (&t4, wb(9, 9, 4 * frame), Ok(82)),
            (
                &t4,
                wb(9, 4, 4 * frame),
                Err(EncodeError::TypeDepthBudget { appended: 72 }),
            ),
            (
                &t4,
                wb(4, 9, 4 * frame),
                Err(EncodeError::TypeNodeBudget { appended: 72 }),
            ),
            (
                &t4,
                wb(9, 9, 4 * frame - 1),
                Err(EncodeError::WalkBudget { appended: 18 }),
            ),
            (
                &t4,
                wb(9, 0, 0),
                Err(EncodeError::TypeDepthBudget { appended: 0 }),
            ),
            (&t2, wb(9, 3, 3 * frame), Ok(46)),
            (
                &t2,
                wb(9, 9, 3 * frame),
                Err(EncodeError::WalkBudget { appended: 18 }),
            ),
        ] {
            let full = enc(t);
            let mut out = Vec::new();
            assert_eq!(enc_with(t, budgets, &mut out), want);
            let n = want.unwrap_or_else(|e| e.appended());
            assert_eq!((out.len(), out.capacity()), (n, n));
            assert!(full.starts_with(&out));
        }
    }

    #[test]
    fn decoder_charges_full_open_stack_growth() {
        let b = enc(&chain(2));
        let (t, o) = (size_of::<TypeExprSrc>(), size_of::<OpenApply>());
        let rb = |max_alloc_bytes, max_type_depth| RawSnapshotReadBudgets {
            max_alloc_bytes,
            max_type_depth,
            ..big()
        };
        // Names "A", "A", "t", two one-item `args`, one stack growth to `frames`.
        for (depth, frames) in [(1 << 20, 4), (3, 3)] {
            let need = 3 + 2 * t + frames * o;
            assert!(dec::<TypeExprSrc>(&b, rb(need, depth)).is_ok());
            assert!(matches!(
                kind::<TypeExprSrc>(&b, rb(need - 1, depth)),
                Some(DK::AllocBudget { .. })
            ));
        }
        assert!(matches!(
            kind::<TypeExprSrc>(&b, rb(3 + 2 * t + 3 * o, 1 << 20)),
            Some(DK::AllocBudget { .. })
        ));
    }
}

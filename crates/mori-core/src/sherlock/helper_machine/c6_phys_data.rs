//! Data/data-field/environment/value-slot table section of sherlock-c6-phys/1
//! (C6P1), Chunk 3A.
//!
//! Codec over four owned `ImageSnapshot` tables. It admits nothing, constructs
//! no `Admitted`/`MachineImage`/`ImageSnapshot`, steps nothing and checks no
//! cross-table membership (Chunk 4). Root counters (`alloc`, `active`,
//! `seg_base`) stay in the Chunk 1 section; the counts here are physical row
//! counts and never stand in for them. Helper/result/task tables are Chunk 3B,
//! assembly Chunk 3C. No native, constraint or proof claim.
//!
//! Layout: magic, version, `DATA_SECTION_TAG`, four counts (data, data_fields,
//! env, value_slots), then those tables in that order, fixed-width rows:
//! - data (5): U128 [0,n,0,0,0]; Bool [1,b,0,0,0]; Nil [2,0,0,0,0];
//!   Cons/Leaf/Branch [3|4|5,0,0,start,len]; Constructor [6,type,ctor,start,len].
//!   Field slices are raw: an empty slice keeps its start.
//! - ref (2): Const [0,i], Data [1,i], as Chunk 1.
//! - env (5): binder, value ref, previous (None [0,0], Some [1,i]), as Chunk 1.
//!
//! `decode_data_tables` first-error order:
//! 1. fewer than 7 lanes: `Truncated`; 2. `BadMagic`; 3. `BadVersion`;
//! 4. tag lane: `BadTag`; 5. counts, structurally, in lane order;
//! 6. counts against caps in table order; 7. exact length: `Truncated`, then
//!    `TrailingLanes` (no allocation before); 8. tables in order, each row
//!    structurally (ascending row-relative lanes) then against caps.
//! Indices are strictly below their cap; counts at most their cap; a field
//! slice checks end overflow, then `max_arity`, then end <= data field slots.

use super::c6_phys::{
    C6_PHYS_MAGIC, C6_PHYS_VERSION, C6CodecBudgets, C6PhysCaps, C6PhysError, C6Result,
};
use super::complete_observation::{DataFieldsSnapshot, DataSnapshot, HostSnapshot, ImageSnapshot};
use super::shared::adt::{CtorIdC, TypeIdC};
use super::shared::{
    BoundsC, EnvCellC, EnvRootC, ExtCapacitiesC, LayoutCapacitiesC, LayoutSchemaC, RefC,
};

pub const DATA_SECTION_TAG: u128 = 0x4441_5441;
pub const DATA_HEADER_LANES: usize = 7;
pub const DATA_ROW_LANES: usize = 5;
pub const REF_LANES: usize = 2;
pub const ENV_ROW_LANES: usize = 5;
const SECTION: &str = "state_data";
const DATA_S: &str = "state.data";
const WIDTHS: [usize; 4] = [DATA_ROW_LANES, REF_LANES, ENV_ROW_LANES, REF_LANES];

/// Chunk 3 representation caps: the Chunk 1/2 caps plus the two admitted
/// limits `C6PhysCaps` does not carry. Exact admitted values; never raised.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6StateCaps {
    pub phys: C6PhysCaps,
    /// `BoundsC::data_field_slots`.
    pub max_data_field_slots: usize,
    /// `BoundsC::helper_tasks` (Chunk 3B task table).
    pub max_helper_tasks: usize,
}

impl C6StateCaps {
    pub fn from_parts(
        bounds: &BoundsC,
        capacities: &LayoutCapacitiesC,
        ext: &ExtCapacitiesC,
        budgets: C6CodecBudgets,
    ) -> Self {
        Self {
            phys: C6PhysCaps::from_parts(bounds, capacities, ext, budgets),
            max_data_field_slots: bounds.data_field_slots,
            max_helper_tasks: bounds.helper_tasks,
        }
    }

    pub fn from_layout(layout: &LayoutSchemaC, budgets: C6CodecBudgets) -> Self {
        let ext = ExtCapacitiesC::effective_for_extended_domain(layout.ext_capacities);
        Self::from_parts(&layout.bounds, &layout.capacities, &ext, budgets)
    }

    /// Convenience caps for tests and tooling. Not admitted limits.
    pub const fn test_caps() -> Self {
        Self {
            phys: C6PhysCaps::test_caps(),
            max_data_field_slots: 1024,
            max_helper_tasks: 64,
        }
    }
}

// ---------- lane primitives (Chunk 1 conventions; shared with Chunk 3B) ----------

pub(crate) const fn u(v: usize) -> u128 {
    v as u128
}

pub(crate) fn rd(section: &'static str, value: u128) -> C6Result<usize> {
    usize::try_from(value).map_err(|_| C6PhysError::UsizeOverflow { section, value })
}
pub(crate) fn rd_bool(section: &'static str, value: u128) -> C6Result<bool> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(C6PhysError::BadBool { section, value }),
    }
}
/// Lanes `from..to` must be zero; `lane` is an index into `lanes`.
pub(crate) fn zero(section: &'static str, lanes: &[u128], from: usize, to: usize) -> C6Result<()> {
    for (lane, &value) in lanes.iter().enumerate().take(to).skip(from) {
        if value != 0 {
            return Err(C6PhysError::NonZeroPadding {
                section,
                lane,
                value,
            });
        }
    }
    Ok(())
}
pub(crate) fn put_ref(out: &mut [u128], at: usize, value: RefC) {
    match value {
        RefC::Const(index) => out[at + 1] = u(index),
        RefC::Data(index) => {
            out[at] = 1;
            out[at + 1] = u(index);
        }
    }
}
pub(crate) fn get_ref(section: &'static str, lanes: &[u128], at: usize) -> C6Result<RefC> {
    match lanes[at] {
        0 => Ok(RefC::Const(rd(section, lanes[at + 1])?)),
        1 => Ok(RefC::Data(rd(section, lanes[at + 1])?)),
        tag => Err(C6PhysError::BadTag { section, tag }),
    }
}
pub(crate) fn put_env(out: &mut [u128], at: usize, env: EnvRootC) {
    if let Some(index) = env {
        out[at] = 1;
        out[at + 1] = u(index);
    }
}
pub(crate) fn get_env(section: &'static str, lanes: &[u128], at: usize) -> C6Result<EnvRootC> {
    match lanes[at] {
        0 => {
            zero(section, lanes, at + 1, at + 2)?;
            Ok(None)
        }
        1 => Ok(Some(rd(section, lanes[at + 1])?)),
        tag => Err(C6PhysError::BadTag { section, tag }),
    }
}

pub(crate) fn cap_count(section: &'static str, needed: usize, limit: usize) -> C6Result<()> {
    if needed > limit {
        Err(C6PhysError::CapacityExceeded {
            section,
            needed,
            limit,
        })
    } else {
        Ok(())
    }
}
pub(crate) fn cap_index(section: &'static str, index: usize, limit: usize) -> C6Result<()> {
    let needed = index.checked_add(1).ok_or(C6PhysError::UsizeOverflow {
        section,
        value: u(index),
    })?;
    cap_count(section, needed, limit)
}
/// Const indexes the program constant table; Data indexes the data arena.
pub(crate) fn cap_ref(section: &'static str, value: RefC, c: &C6StateCaps) -> C6Result<()> {
    match value {
        RefC::Const(index) => cap_index(section, index, c.phys.max_constants),
        RefC::Data(index) => cap_index(section, index, c.phys.max_data_nodes),
    }
}
pub(crate) fn cap_env(section: &'static str, env: EnvRootC, c: &C6StateCaps) -> C6Result<()> {
    match env {
        Some(index) => cap_index(section, index, c.phys.max_env_cells),
        None => Ok(()),
    }
}
fn cap_fields(f: DataFieldsSnapshot, c: &C6StateCaps) -> C6Result<()> {
    const S: &str = "state.data.fields";
    let end = f
        .start
        .checked_add(f.len)
        .ok_or(C6PhysError::UsizeOverflow {
            section: S,
            value: u(f.start),
        })?;
    cap_count(S, f.len, c.phys.max_arity)?;
    cap_count(S, end, c.max_data_field_slots)
}

// ---------- fixed-width rows ----------

pub fn encode_data_row(row: &DataSnapshot) -> [u128; DATA_ROW_LANES] {
    use DataSnapshot as D;
    let mut o = [0u128; DATA_ROW_LANES];
    let mut fields = |o: &mut [u128; DATA_ROW_LANES], tag: u128, f: DataFieldsSnapshot| {
        o[0] = tag;
        o[3] = u(f.start);
        o[4] = u(f.len);
    };
    match *row {
        D::U128 { n } => o[1] = n,
        D::Bool { b } => {
            o[0] = 1;
            o[1] = u128::from(b);
        }
        D::Nil => o[0] = 2,
        D::Cons { fields: f } => fields(&mut o, 3, f),
        D::Leaf { fields: f } => fields(&mut o, 4, f),
        D::Branch { fields: f } => fields(&mut o, 5, f),
        D::Constructor {
            type_id,
            ctor_id,
            fields: f,
        } => {
            fields(&mut o, 6, f);
            o[1] = u(type_id.0);
            o[2] = u(ctor_id.0);
        }
    }
    o
}

fn get_fields(l: &[u128; DATA_ROW_LANES]) -> C6Result<DataFieldsSnapshot> {
    Ok(DataFieldsSnapshot {
        start: rd(DATA_S, l[3])?,
        len: rd(DATA_S, l[4])?,
    })
}

/// Structural data-row decode: tag, representability and padding, ascending lanes.
pub fn decode_data_row(l: &[u128; DATA_ROW_LANES]) -> C6Result<DataSnapshot> {
    use DataSnapshot as D;
    let (row, used) = match l[0] {
        0 => (D::U128 { n: l[1] }, 2),
        1 => (
            D::Bool {
                b: rd_bool(DATA_S, l[1])?,
            },
            2,
        ),
        2 => (D::Nil, 1),
        3..=5 => {
            zero(DATA_S, l, 1, 3)?;
            let fields = get_fields(l)?;
            (
                match l[0] {
                    3 => D::Cons { fields },
                    4 => D::Leaf { fields },
                    _ => D::Branch { fields },
                },
                DATA_ROW_LANES,
            )
        }
        6 => (
            D::Constructor {
                type_id: TypeIdC(rd(DATA_S, l[1])?),
                ctor_id: CtorIdC(rd(DATA_S, l[2])?),
                fields: get_fields(l)?,
            },
            DATA_ROW_LANES,
        ),
        tag => {
            return Err(C6PhysError::BadTag {
                section: DATA_S,
                tag,
            });
        }
    };
    zero(DATA_S, l, used, DATA_ROW_LANES)?;
    Ok(row)
}

fn data_row_caps(row: &DataSnapshot, c: &C6StateCaps) -> C6Result<()> {
    use DataSnapshot as D;
    match *row {
        D::U128 { .. } | D::Bool { .. } | D::Nil => Ok(()),
        D::Cons { fields } | D::Leaf { fields } | D::Branch { fields } => cap_fields(fields, c),
        D::Constructor {
            type_id,
            ctor_id,
            fields,
        } => {
            cap_index("state.data.type_id", type_id.0, c.phys.max_types)?;
            cap_index("state.data.ctor_id", ctor_id.0, c.phys.max_constructors)?;
            cap_fields(fields, c)
        }
    }
}

pub fn encode_ref_row(value: RefC) -> [u128; REF_LANES] {
    let mut o = [0u128; REF_LANES];
    put_ref(&mut o, 0, value);
    o
}
pub fn decode_ref_row(section: &'static str, l: &[u128; REF_LANES]) -> C6Result<RefC> {
    get_ref(section, l, 0)
}

pub fn encode_env_row(cell: &EnvCellC) -> [u128; ENV_ROW_LANES] {
    let mut o = [0u128; ENV_ROW_LANES];
    o[0] = u(cell.binder);
    put_ref(&mut o, 1, cell.value);
    put_env(&mut o, 3, cell.previous);
    o
}
pub fn decode_env_row(l: &[u128; ENV_ROW_LANES]) -> C6Result<EnvCellC> {
    const S: &str = "state.env";
    Ok(EnvCellC {
        binder: rd(S, l[0])?,
        value: get_ref(S, l, 1)?,
        previous: get_env(S, l, 3)?,
    })
}
fn env_row_caps(cell: &EnvCellC, c: &C6StateCaps) -> C6Result<()> {
    cap_index("state.env.binder", cell.binder, c.phys.max_names)?;
    cap_ref("state.env.value", cell.value, c)?;
    cap_env("state.env.previous", cell.previous, c)
}

// ---------- owned table carrier and section ----------

/// The four owned image tables, exactly as captured: order, multiplicity,
/// raw slice starts and RefC tags are retained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6DataTables {
    pub data: Vec<DataSnapshot>,
    pub data_fields: Vec<RefC>,
    pub env: Vec<EnvCellC>,
    pub value_slots: Vec<RefC>,
}

impl C6DataTables {
    /// Owned extraction of the four table fields; no counter is read.
    pub fn from_image(image: &ImageSnapshot) -> Self {
        Self {
            data: image.data.clone(),
            data_fields: image.data_fields.clone(),
            env: image.env.clone(),
            value_slots: image.value_slots.clone(),
        }
    }
    /// Diagnostic: overwrite exactly these four fields of a caller-owned image
    /// (normally a clone). Not import, admission or a MachineImage.
    pub fn replace_tables_of(self, image: &mut ImageSnapshot) {
        image.data = self.data;
        image.data_fields = self.data_fields;
        image.env = self.env;
        image.value_slots = self.value_slots;
    }
    fn counts(&self) -> [usize; 4] {
        [
            self.data.len(),
            self.data_fields.len(),
            self.env.len(),
            self.value_slots.len(),
        ]
    }
}

fn count_caps(n: [usize; 4], c: &C6StateCaps) -> C6Result<()> {
    cap_count("state.data", n[0], c.phys.max_data_nodes)?;
    cap_count("state.data_fields", n[1], c.max_data_field_slots)?;
    cap_count("state.env", n[2], c.phys.max_env_cells)?;
    cap_count("state.value_slots", n[3], c.phys.max_value_slots)
}

/// Table start offsets plus the exact section length at index 4.
fn table_offsets(n: [usize; 4]) -> C6Result<[usize; 5]> {
    let mut o = [DATA_HEADER_LANES; 5];
    for k in 0..4 {
        o[k + 1] = n[k]
            .checked_mul(WIDTHS[k])
            .and_then(|w| o[k].checked_add(w))
            .ok_or(C6PhysError::UsizeOverflow {
                section: SECTION,
                value: u(n[k]),
            })?;
    }
    Ok(o)
}

/// Canonical encoder. Counts and every row are capped before allocation.
pub fn encode_data_tables(t: &C6DataTables, caps: &C6StateCaps) -> C6Result<Vec<u128>> {
    let counts = t.counts();
    count_caps(counts, caps)?;
    for row in &t.data {
        data_row_caps(row, caps)?;
    }
    for r in &t.data_fields {
        cap_ref("state.data_fields.ref", *r, caps)?;
    }
    for cell in &t.env {
        env_row_caps(cell, caps)?;
    }
    for r in &t.value_slots {
        cap_ref("state.value_slots.ref", *r, caps)?;
    }
    let mut out = Vec::with_capacity(table_offsets(counts)?[4]);
    out.extend([C6_PHYS_MAGIC, C6_PHYS_VERSION, DATA_SECTION_TAG]);
    out.extend(counts.map(u));
    for row in &t.data {
        out.extend(encode_data_row(row));
    }
    for r in &t.data_fields {
        out.extend(encode_ref_row(*r));
    }
    for cell in &t.env {
        out.extend(encode_env_row(cell));
    }
    for r in &t.value_slots {
        out.extend(encode_ref_row(*r));
    }
    Ok(out)
}

fn decode_rows<const N: usize, T>(
    lanes: &[u128],
    from: usize,
    to: usize,
    mut row: impl FnMut(&[u128; N]) -> C6Result<T>,
) -> C6Result<Vec<T>> {
    let truncated = || C6PhysError::Truncated {
        section: SECTION,
        needed: to,
        actual: lanes.len(),
    };
    let part = lanes.get(from..to).ok_or_else(truncated)?;
    let mut out = Vec::with_capacity(part.len() / N);
    for chunk in part.chunks_exact(N) {
        let record: &[u128; N] = chunk.try_into().map_err(|_| truncated())?;
        out.push(row(record)?);
    }
    Ok(out)
}

fn checked_ref(
    l: &[u128; REF_LANES],
    section: &'static str,
    cap_section: &'static str,
    caps: &C6StateCaps,
) -> C6Result<RefC> {
    let value = decode_ref_row(section, l)?;
    cap_ref(cap_section, value, caps)?;
    Ok(value)
}

/// Decoder; see the module documentation for the exact first-error order.
/// Returns owned tables only, never an image, admission or machine state.
pub fn decode_data_tables(lanes: &[u128], caps: &C6StateCaps) -> C6Result<C6DataTables> {
    if lanes.len() < DATA_HEADER_LANES {
        return Err(C6PhysError::Truncated {
            section: SECTION,
            needed: DATA_HEADER_LANES,
            actual: lanes.len(),
        });
    }
    if lanes[0] != C6_PHYS_MAGIC {
        return Err(C6PhysError::BadMagic { actual: lanes[0] });
    }
    if lanes[1] != C6_PHYS_VERSION {
        return Err(C6PhysError::BadVersion { actual: lanes[1] });
    }
    if lanes[2] != DATA_SECTION_TAG {
        return Err(C6PhysError::BadTag {
            section: SECTION,
            tag: lanes[2],
        });
    }
    let counts = [
        rd(SECTION, lanes[3])?,
        rd(SECTION, lanes[4])?,
        rd(SECTION, lanes[5])?,
        rd(SECTION, lanes[6])?,
    ];
    count_caps(counts, caps)?;
    let o = table_offsets(counts)?;
    let needed = o[4];
    if lanes.len() < needed {
        return Err(C6PhysError::Truncated {
            section: SECTION,
            needed,
            actual: lanes.len(),
        });
    }
    if lanes.len() > needed {
        return Err(C6PhysError::TrailingLanes {
            section: SECTION,
            extra: lanes.len() - needed,
        });
    }
    Ok(C6DataTables {
        data: decode_rows::<DATA_ROW_LANES, _>(lanes, o[0], o[1], |l| {
            let row = decode_data_row(l)?;
            data_row_caps(&row, caps)?;
            Ok(row)
        })?,
        data_fields: decode_rows::<REF_LANES, _>(lanes, o[1], o[2], |l| {
            checked_ref(l, "state.data_fields", "state.data_fields.ref", caps)
        })?,
        env: decode_rows::<ENV_ROW_LANES, _>(lanes, o[2], o[3], |l| {
            let cell = decode_env_row(l)?;
            env_row_caps(&cell, caps)?;
            Ok(cell)
        })?,
        value_slots: decode_rows::<REF_LANES, _>(lanes, o[3], o[4], |l| {
            checked_ref(l, "state.value_slots", "state.value_slots.ref", caps)
        })?,
    })
}

/// Normal owned-snapshot caller: encode the four tables of a captured host.
/// Constructs no Admitted/MachineImage; claims no admission or execution.
pub fn data_lanes_from_host(host: &HostSnapshot, caps: &C6StateCaps) -> C6Result<Vec<u128>> {
    encode_data_tables(&C6DataTables::from_image(&host.image), caps)
}

pub fn roundtrip_data_tables_owned(t: &C6DataTables, caps: &C6StateCaps) -> C6Result<C6DataTables> {
    decode_data_tables(&encode_data_tables(t, caps)?, caps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sherlock::helper_machine::c6_phys::fixtures;
    use crate::sherlock::helper_machine::c6_phys_program::PROGRAM_SECTION_TAG;
    use crate::sherlock::helper_machine::complete_observation::{
        ObservedStep, capture, record_step,
    };
    use crate::sherlock::helper_machine::shared::HelperResultC;

    fn caps() -> C6StateCaps {
        C6StateCaps::test_caps()
    }
    fn f(start: usize, len: usize) -> DataFieldsSnapshot {
        DataFieldsSnapshot { start, len }
    }

    fn all_variants() -> C6DataTables {
        C6DataTables {
            data: vec![
                DataSnapshot::U128 { n: u128::MAX },
                DataSnapshot::U128 { n: 0 },
                DataSnapshot::Bool { b: true },
                DataSnapshot::Bool { b: false },
                DataSnapshot::Nil,
                DataSnapshot::Cons { fields: f(0, 2) },
                DataSnapshot::Leaf { fields: f(2, 1) },
                DataSnapshot::Branch { fields: f(3, 2) },
                DataSnapshot::Constructor {
                    type_id: TypeIdC(5),
                    ctor_id: CtorIdC(6),
                    fields: f(5, 3),
                },
                DataSnapshot::Constructor {
                    type_id: TypeIdC(4),
                    ctor_id: CtorIdC(4),
                    fields: f(7, 0),
                },
            ],
            data_fields: vec![
                RefC::Const(3),
                RefC::Data(3),
                RefC::Data(3),
                RefC::Const(0),
                RefC::Data(0),
                RefC::Const(3),
                RefC::Data(9),
                RefC::Const(1),
            ],
            env: vec![
                EnvCellC {
                    binder: 2,
                    value: RefC::Data(8),
                    previous: None,
                },
                EnvCellC {
                    binder: 2,
                    value: RefC::Const(8),
                    previous: Some(0),
                },
                EnvCellC {
                    binder: 0,
                    value: RefC::Data(0),
                    previous: Some(1),
                },
            ],
            value_slots: vec![RefC::Data(9), RefC::Data(9), RefC::Const(9), RefC::Const(0)],
        }
    }

    /// Changing any single lane must refuse or decode to a different value.
    fn every_lane_significant<const N: usize, T: PartialEq>(
        lanes: [u128; N],
        value: &T,
        decode: fn(&[u128; N]) -> C6Result<T>,
    ) {
        for at in 0..N {
            let mut changed = lanes;
            changed[at] ^= 0x55;
            assert!(
                decode(&changed).map_or(true, |v| &v != value),
                "lane {at} was ignored"
            );
        }
    }

    #[test]
    fn every_table_variant_and_raw_field_roundtrips_exactly() {
        // SYNTHETIC codec vectors over every variant; not admitted images.
        let t = all_variants();
        let lanes = encode_data_tables(&t, &caps()).unwrap();
        assert_eq!(
            lanes[..DATA_HEADER_LANES],
            [
                C6_PHYS_MAGIC,
                C6_PHYS_VERSION,
                DATA_SECTION_TAG,
                10,
                8,
                3,
                4
            ]
        );
        assert_eq!(
            lanes.len(),
            7 + 10 * DATA_ROW_LANES + 8 * REF_LANES + 3 * ENV_ROW_LANES + 4 * REF_LANES
        );
        assert_eq!(lanes[7..12], [0, u128::MAX, 0, 0, 0]);
        assert_eq!(lanes[32..37], [3, 0, 0, 0, 2]);
        assert_eq!(lanes[47..52], [6, 5, 6, 5, 3]);
        assert_eq!(
            lanes[52..57],
            [6, 4, 4, 7, 0],
            "nonzero start of an empty slice is kept"
        );
        assert_eq!(
            lanes[57..61],
            [0, 3, 1, 3],
            "same-index Const and Data stay distinct"
        );
        assert_eq!(
            lanes[73..83],
            [2, 1, 8, 0, 0, 2, 0, 8, 1, 0],
            "None vs Some(0) previous"
        );
        let decoded = decode_data_tables(&lanes, &caps()).unwrap();
        assert_eq!(decoded, t);
        assert_eq!(
            encode_data_tables(&decoded, &caps()).unwrap(),
            lanes,
            "accepted lanes re-encode identically"
        );
        assert_eq!(roundtrip_data_tables_owned(&t, &caps()).unwrap(), t);
        for row in &t.data {
            every_lane_significant(encode_data_row(row), row, decode_data_row);
        }
        for cell in &t.env {
            every_lane_significant(encode_env_row(cell), cell, decode_env_row);
        }
        for r in t.data_fields.iter().chain(&t.value_slots) {
            every_lane_significant(encode_ref_row(*r), r, |l| decode_ref_row("state.ref", l));
        }
        let differs = |m: C6DataTables| assert_ne!(encode_data_tables(&m, &caps()).unwrap(), lanes);
        let mut m = t.clone();
        m.value_slots.swap(1, 2);
        differs(m);
        let mut m = t.clone();
        m.data_fields[0] = RefC::Data(3);
        differs(m);
        let mut m = t.clone();
        m.env[0].previous = Some(0);
        differs(m);
        let mut m = t.clone();
        m.data[9] = DataSnapshot::Constructor {
            type_id: TypeIdC(4),
            ctor_id: CtorIdC(4),
            fields: f(0, 0),
        };
        differs(m);
        let mut m = t;
        m.data[0] = DataSnapshot::U128 { n: u128::MAX - 1 };
        differs(m);
    }

    #[test]
    fn refusals_follow_the_declared_first_error_order() {
        let t = all_variants();
        let lanes = encode_data_tables(&t, &caps()).unwrap();
        let dec = |l: &[u128]| decode_data_tables(l, &caps());
        let set = |pairs: &[(usize, u128)]| {
            let mut m = lanes.clone();
            for &(k, v) in pairs {
                m[k] = v;
            }
            m
        };
        fn cap<T>(section: &'static str, needed: usize, limit: usize) -> C6Result<T> {
            Err(C6PhysError::CapacityExceeded {
                section,
                needed,
                limit,
            })
        }
        assert_eq!(
            dec(&lanes[..6]),
            Err(C6PhysError::Truncated {
                section: SECTION,
                needed: 7,
                actual: 6
            })
        );
        assert_eq!(
            dec(&set(&[(0, 0), (2, 0)])),
            Err(C6PhysError::BadMagic { actual: 0 })
        );
        assert_eq!(
            dec(&set(&[(1, 2)])),
            Err(C6PhysError::BadVersion { actual: 2 })
        );
        assert_eq!(
            dec(&set(&[(2, PROGRAM_SECTION_TAG)])),
            Err(C6PhysError::BadTag {
                section: SECTION,
                tag: PROGRAM_SECTION_TAG
            })
        );
        // Structural counts precede count caps; count caps precede length.
        assert_eq!(
            dec(&set(&[(3, 999), (4, u128::MAX)])),
            Err(C6PhysError::UsizeOverflow {
                section: SECTION,
                value: u128::MAX
            })
        );
        assert_eq!(dec(&set(&[(3, 257)])), cap("state.data", 257, 256));
        assert_eq!(
            dec(&set(&[(3, 11)])),
            Err(C6PhysError::Truncated {
                section: SECTION,
                needed: 101,
                actual: 96
            })
        );
        let mut trailing = lanes.clone();
        trailing.push(0);
        assert_eq!(
            dec(&trailing),
            Err(C6PhysError::TrailingLanes {
                section: SECTION,
                extra: 1
            })
        );
        // Data rows at 7 + 5k: structural, row-relative lanes.
        assert_eq!(
            dec(&set(&[(7, 7)])),
            Err(C6PhysError::BadTag {
                section: DATA_S,
                tag: 7
            })
        );
        assert_eq!(
            dec(&set(&[(18, 2)])),
            Err(C6PhysError::BadBool {
                section: DATA_S,
                value: 2
            })
        );
        assert_eq!(
            dec(&set(&[(28, 1)])),
            Err(C6PhysError::NonZeroPadding {
                section: DATA_S,
                lane: 1,
                value: 1
            })
        );
        assert_eq!(
            dec(&set(&[(34, 4)])),
            Err(C6PhysError::NonZeroPadding {
                section: DATA_S,
                lane: 2,
                value: 4
            })
        );
        // Within a row, structure precedes caps; earlier tables precede later ones.
        assert_eq!(
            dec(&set(&[(48, 64), (49, u128::MAX)])),
            Err(C6PhysError::UsizeOverflow {
                section: DATA_S,
                value: u128::MAX
            })
        );
        assert_eq!(
            dec(&set(&[(48, 64), (88, 7)])),
            cap("state.data.type_id", 65, 64)
        );
        assert_eq!(dec(&set(&[(49, 256)])), cap("state.data.ctor_id", 257, 256));
        assert_eq!(dec(&set(&[(51, 9)])), cap("state.data.fields", 9, 8));
        assert_eq!(
            dec(&set(&[(50, 1022)])),
            cap("state.data.fields", 1025, 1024)
        );
        assert_eq!(
            dec(&set(&[(50, usize::MAX as u128)])),
            Err(C6PhysError::UsizeOverflow {
                section: "state.data.fields",
                value: usize::MAX as u128
            })
        );
        // Ref, env and value-slot rows.
        assert_eq!(
            dec(&set(&[(57, 2)])),
            Err(C6PhysError::BadTag {
                section: "state.data_fields",
                tag: 2
            })
        );
        assert_eq!(
            dec(&set(&[(77, 1)])),
            Err(C6PhysError::NonZeroPadding {
                section: "state.env",
                lane: 4,
                value: 1
            })
        );
        assert_eq!(
            dec(&set(&[(76, 2)])),
            Err(C6PhysError::BadTag {
                section: "state.env",
                tag: 2
            })
        );
        assert_eq!(dec(&set(&[(73, 256)])), cap("state.env.binder", 257, 256));
        assert_eq!(dec(&set(&[(82, 256)])), cap("state.env.previous", 257, 256));
        assert_eq!(
            dec(&set(&[(88, 7)])),
            Err(C6PhysError::BadTag {
                section: "state.value_slots",
                tag: 7
            })
        );
        // Const and Data references use their own tables.
        let one = |r| C6DataTables {
            data: vec![],
            data_fields: vec![r],
            env: vec![],
            value_slots: vec![],
        };
        let mut c = caps();
        c.phys.max_data_nodes = 4;
        c.phys.max_constants = 9;
        assert!(encode_data_tables(&one(RefC::Const(8)), &c).is_ok());
        assert_eq!(
            encode_data_tables(&one(RefC::Data(4)), &c),
            cap("state.data_fields.ref", 5, 4)
        );
        let mut l = encode_data_tables(&one(RefC::Data(3)), &c).unwrap();
        l[8] = 4;
        assert_eq!(
            decode_data_tables(&l, &c),
            cap("state.data_fields.ref", 5, 4)
        );
        // The encoder applies the same caps; the data-field limit is its own.
        let mut tight = caps();
        tight.max_data_field_slots = 7;
        assert_eq!(
            encode_data_tables(&t, &tight),
            cap("state.data_fields", 8, 7)
        );
        assert_eq!(
            decode_data_tables(&lanes, &tight),
            cap("state.data_fields", 8, 7)
        );
        let mut wide = t;
        wide.data[8] = DataSnapshot::Constructor {
            type_id: TypeIdC(64),
            ctor_id: CtorIdC(6),
            fields: f(5, 3),
        };
        assert_eq!(
            encode_data_tables(&wide, &caps()),
            cap("state.data.type_id", 65, 64)
        );
        // Exact admitted mapping: no borrowed or defaulted limit.
        let bounds = BoundsC {
            data_nodes: 5,
            data_field_slots: 11,
            helper_tasks: 13,
            ..BoundsC::default()
        };
        let budgets = C6CodecBudgets {
            type_expr_nodes: 1,
            template_bytes: 2,
        };
        let (lc, ext) = (LayoutCapacitiesC::default(), ExtCapacitiesC::all_zero());
        assert_eq!(
            C6StateCaps::from_parts(&bounds, &lc, &ext, budgets),
            C6StateCaps {
                phys: C6PhysCaps::from_parts(&bounds, &lc, &ext, budgets),
                max_data_field_slots: 11,
                max_helper_tasks: 13
            }
        );
    }

    #[test]
    fn captured_nominal_host_tables_roundtrip_and_preserve_host() {
        // OWNER-REACHABLE: real generic nominal admission, real fixed-owner steps.
        let (admitted, mut image) = fixtures::admitted_run(64);
        let mut terminal = false;
        for _ in 0..4096 {
            match record_step(&admitted, &mut image) {
                Ok(ObservedStep::Event(_)) => {}
                Ok(ObservedStep::Terminal { .. }) => {
                    terminal = true;
                    break;
                }
                Err(_) => panic!("owner step"),
            }
        }
        assert!(terminal, "the nominal run reaches its terminal boundary");
        let host = fixtures::must(capture(&admitted, &image), "capture");
        let img = &host.image;
        assert!(
            img.data
                .iter()
                .any(|d| matches!(d, DataSnapshot::Constructor { .. }))
        );
        assert!(img.data_fields.iter().any(|r| matches!(r, RefC::Const(_))));
        assert!(img.data_fields.iter().any(|r| matches!(r, RefC::Data(_))));
        assert!(!img.env.is_empty() && !img.value_slots.is_empty());
        // Admitted bounds/capacities of this host; the data section reads no ext limit.
        let caps = C6StateCaps::from_parts(
            &host.admission.bounds,
            &host.admission.layout.capacities,
            &ExtCapacitiesC::all_zero(),
            C6CodecBudgets {
                type_expr_nodes: 0,
                template_bytes: 0,
            },
        );
        let lanes = data_lanes_from_host(&host, &caps).unwrap();
        let owned = C6DataTables::from_image(img);
        assert_eq!(lanes, encode_data_tables(&owned, &caps).unwrap());
        let decoded = decode_data_tables(&lanes, &caps).unwrap();
        assert_eq!(decoded, owned);
        assert_eq!(encode_data_tables(&decoded, &caps).unwrap(), lanes);
        // Replace ONLY the four tables of a clone: the whole owned host is equal.
        let mut rebuilt = host.clone();
        C6DataTables {
            data: vec![],
            data_fields: vec![],
            env: vec![],
            value_slots: vec![],
        }
        .replace_tables_of(&mut rebuilt.image);
        assert_ne!(rebuilt, host);
        decoded.replace_tables_of(&mut rebuilt.image);
        assert_eq!(rebuilt, host);
        // Root counters and other state are not in this section.
        let mut other = host.clone();
        other.image.alloc.data_nodes += 1;
        other.image.active.data_fields += 1;
        other.image.reference_cursor += 7;
        other.image.frames.clear();
        other.image.result = Some(HelperResultC::EnvMissing);
        assert_eq!(data_lanes_from_host(&other, &caps).unwrap(), lanes);
        // A table field change is visible.
        let mut changed = host.clone();
        changed.image.data_fields[0] = match changed.image.data_fields[0] {
            RefC::Const(i) => RefC::Data(i),
            RefC::Data(i) => RefC::Const(i),
        };
        assert_ne!(data_lanes_from_host(&changed, &caps).unwrap(), lanes);
        let n = img.data_fields.len();
        let mut tight = caps;
        tight.max_data_field_slots = n - 1;
        assert_eq!(
            data_lanes_from_host(&host, &tight),
            Err(C6PhysError::CapacityExceeded {
                section: "state.data_fields",
                needed: n,
                limit: n - 1
            })
        );
    }
}

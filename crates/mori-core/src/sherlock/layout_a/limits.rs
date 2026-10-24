use super::rows::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Locator {
    pub expression: Option<usize>,
    pub function: Option<usize>,
    pub field: &'static str,
    pub item: usize,
}
impl Locator {
    pub fn field(field: &'static str) -> Self {
        Self {
            expression: None,
            function: None,
            field,
            item: 0,
        }
    }
}

/// The preparation phase produces only these two of L4's seven refusal classes.
/// No decoder/observer is stubbed here; other phase failures remain future work.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrepareFailure {
    Capacity {
        table: TableId,
        needed: u128,
        limit: u128,
        locator: Locator,
    },
    HostUnrepresentable {
        locator: Locator,
    },
}
pub type Result<T> = std::result::Result<T, PrepareFailure>;

pub fn host(locator: Locator) -> PrepareFailure {
    PrepareFailure::HostUnrepresentable { locator }
}
pub fn push<T>(values: &mut Vec<T>, value: T, loc: Locator) -> Result<()> {
    values.try_reserve(1).map_err(|_| host(loc))?;
    values.push(value);
    Ok(())
}
pub fn copy_slice<T: Clone>(values: &[T], loc: Locator) -> Result<Vec<T>> {
    let mut out = Vec::new();
    out.try_reserve(values.len()).map_err(|_| host(loc))?;
    out.extend_from_slice(values);
    Ok(out)
}
pub struct Limits {
    pub capacities: Capacities,
    pub host: [usize; 15],
    pub reserved: [usize; 15],
}
impl Limits {
    pub fn new(capacities: Capacities) -> Result<Self> {
        let mut host_sizes = [0; 15];
        for (i, n) in capacities.tables.iter().enumerate() {
            host_sizes[i] = usize::try_from(*n).map_err(|_| {
                host(Locator {
                    item: i,
                    ..Locator::field("capacity")
                })
            })?;
        }
        // D need not fit a host vector here: no Data table is allocated.
        Ok(Self {
            capacities,
            host: host_sizes,
            reserved: [0; 15],
        })
    }
    pub fn reserve(&mut self, table: TableId, count: usize, loc: Locator) -> Result<()> {
        let i = table as usize;
        let needed = (self.reserved[i] as u128)
            .checked_add(count as u128)
            .ok_or_else(|| host(loc))?;
        if needed > self.capacities.tables[i] {
            return Err(PrepareFailure::Capacity {
                table,
                needed,
                limit: self.capacities.tables[i],
                locator: loc,
            });
        }
        self.reserved[i] = usize::try_from(needed).map_err(|_| host(loc))?;
        Ok(())
    }
    pub fn pad<T: Default + Clone>(&self, table: TableId, values: &mut Table<T>) -> Result<()> {
        values.active = values.rows.len();
        let n = self.host[table as usize];
        let extra = n
            .checked_sub(values.active)
            .ok_or_else(|| host(Locator::field("padding")))?;
        values.rows.try_reserve_exact(extra).map_err(|_| {
            host(Locator {
                item: table as usize,
                ..Locator::field("padding")
            })
        })?;
        values.rows.resize(n, T::default());
        Ok(())
    }
}

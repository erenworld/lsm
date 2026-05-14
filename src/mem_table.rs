// #![allow(unused_variables)]
// #![allow(dead_code)]

use std::ops::Bound;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use anyhow::Result;
use bytes::Bytes;
use crossbeam_skiplist::SkipMap;
use ouroboros::self_referencing;

use crate::iterators::StorageIterator;
use crate::key::KeySlice;
use crate::table::SsTableBuilder;
use crate::wal::wal;
pub struct MemTable {
    // insert:  O(log n)
    // search:  O(log n)
    map: Arc<SkipMap<Bytes, Bytes>>, 
    wal: Option<Wal>,
    id: usize,
    approximate_size: Arc<AtomicUsize>,
}

// Scan [a, z)
pub(crate) fn map_bound(bound: Bound<&[u8]>) -> Bound<Bytes> {
    match bound {
        Bound::Included(x) => Bound::Included(Bytes::copy_from_slice(x)),
        Bound::Excluded(x) => Bound::Excluded(Bytes::copy_from_slice(x)),
        Bound::Unbounded => Bound::Unbounded,
    }
}

impl MemTable {
    // Create a new mem-table
    pub fn create(_id: usize) -> Self {
        unimplemented!()
    }

    // Create a new mem-table from WAL
    pub fn create_with_wal(_id: usize, _path: impl AsRef<Path>) -> Result<Self> {
        unimplemented!()
    }
}

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
    // Create a new mem-table.
    pub fn create(_id: usize) -> Self {
        unimplemented!()
    }

    // Create a new mem-table from WAL.
    pub fn create_with_wal(_id: usize, _path: impl AsRef<Path>) -> Result<Self> {
        unimplemented!()
    }

    // Recover a mem-table from WAL.
    pub fn recover_from_wal(_id: usize, _path: impl AsRef<Path>) -> Result<Self> {
        unimplemented!()
    }

    pub fn for_testing_put_slice(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.put(key, value)
    }

    pub fn for_testing_get_slice(&self, key: &[u8]) -> Option<Bytes> {
        self.get(key)
    }

    // you do not need to consider the bound exclude/include logic. Simply provide `DEFAULT_TS` as the
    // timestamp for the key-ts pair.
    pub fn for_testing_scan_slice(
        &self,
        lower: Bound<&[u8]>,
        upper: Bound<&[u8]>
    ) -> MemTableIterator {
        self.scan(lower, upper)
    }

    // Get a value by key.
    pub fn get(&self, _key: &[u8]) -> Option<Bytes> {
        unimplemented!()
    }

    // Put a key-value pairs into the mem-table.
    //
    // simply put the key-value pair into the skipmap.
    // also flush the data to WAL.
    // modify the function to use the batch API.
    pub fn put(&self, _key: &[u8], _value: &[u8]) -> Result<()> {
        unimplemented!()
    }
}

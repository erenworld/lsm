# LSM-Tree storage engine 

## Overview
An LSM storage engine generally contains three parts:

1. Write-ahead log to persist temporary data for recovery.
2. SSTs on the disk to maintain an LSM-tree structure.
3. Mem-tables in memory for batching small writes.

The storage engine generally provides the following interfaces:
- `Put(key, value)`: store a key-value pair in the LSM tree.
- `Delete(key)`: remove a key and its corresponding value.
- `Get(key)`: get the value corresponding to a key.
- `Scan(range)`: get a range of key-values pairs.

To ensure persistence,
- `Sync()`: ensure all the operations before `sync` are persisted to the disk.

Some engines choose to combine `Put` and `Delete` into a single operation called `WriteBatch`, which accepts a batch of key-values pairs.

> For this project, we assume the LSM tree is using a `leveled compaction algorithm`, which is commonly used in real-world systems.@

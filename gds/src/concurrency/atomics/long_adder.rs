// LongAdder - Striped accumulator for high-contention scenarios
//
// Coming soon! This will use a striped cell pattern like Java's LongAdder
// to reduce contention in highly concurrent scenarios.
//
// The pattern: Instead of one atomic value, use an array of 64 cells.
// Each thread hashes to a cell and updates it. sum() adds all cells.
// This trades memory for reduced CAS contention.
//
// For now, use AtomicI64 directly for low-contention scenarios.

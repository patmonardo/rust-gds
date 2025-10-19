// Atomic aggregators for lock-free parallel algorithm operations.
//
// This module provides lock-free atomic types for concurrent aggregation operations
// commonly needed in parallel graph algorithms. All types use Compare-And-Swap (CAS)
// operations to ensure thread-safety without locks.

mod atomic_double;
mod atomic_max;
mod atomic_min;
// mod double_adder;  // Coming soon - striped accumulator pattern
// mod long_adder;    // Coming soon - striped accumulator pattern

pub use atomic_double::AtomicDouble;
pub use atomic_max::AtomicMax;
pub use atomic_min::AtomicMin;
// pub use double_adder::DoubleAdder;
// pub use long_adder::LongAdder;

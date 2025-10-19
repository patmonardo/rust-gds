// Thread pool size configuration
//
// This module provides types for configuring thread pool dimensions,
// mirroring the Java GDS PoolSizes interfaces.
//
// ## Rust vs Java: The EJB Lesson
//
// Java needed `@ServiceProvider` annotations, priority systems, and service
// loading frameworks because Enterprise Java Beans made interfaces painful.
//
// Rust? **Traits + functions = done.** No service framework needed!

mod open_gds_pool_sizes;
mod pool_sizes;
mod pool_sizes_provider;

// Core trait and implementations
pub use pool_sizes::{
    custom, default, fixed, from_cpu_cores, single_threaded, CustomPoolSizes, DefaultPoolSizes,
    FixedPoolSizes, PoolSizes,
};

// Enterprise integration (simplified!)
pub use open_gds_pool_sizes::OpenGdsPoolSizes;
pub use pool_sizes_provider::{LicenseState, PoolSizesProvider, PoolSizesService};

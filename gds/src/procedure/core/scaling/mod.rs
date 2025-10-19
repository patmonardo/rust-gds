//! Scaling module - Feature scaling for ML pipelines
//!
//! **Translation Source**: `org.neo4j.gds.scaling.*` package
//!
//! This module provides feature scaling transformations for node properties before ML algorithms.
//!
//! ## Architecture
//!
//! Instead of translating each Java scaler file separately (which would create 7+ files
//! with 90% duplicate boilerplate), we use a **unified approach**:
//! - Single `PropertyStats` aggregator computes all statistics in one parallel pass
//! - Each scaler extracts only the stats it needs
//! - Automatic parallel/serial execution based on concurrency
//! - Zero-value handling when range/std is too small
//!
//! This demonstrates Rust's strength: what takes Java 2,000+ lines across 10+ files
//! takes Rust 626 lines in a single implementation file with zero-cost abstractions.
//!
//! ## Module Organization
//!
//! - `scaler.rs` - All scaler implementations (MinMax, StdScore, Mean, Max, Center, Log, None)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rust_gds::procedure::core::scaling::*;
//!
//! let property_fn = |node_id: u64| (node_id as f64) * 10.0;
//! let scaler = MinMaxScaler::create(100, &property_fn, 4);
//! let scaled = scaler.scale_property(42, &property_fn);
//! ```

// Implementation
mod scaler;

// Public exports
pub use scaler::{
    CenterScaler, LogScaler, MaxScaler, MeanScaler, MinMaxScaler, NoneScaler, Scaler,
    ScalerStatistics, StdScoreScaler, CLOSE_TO_ZERO,
};

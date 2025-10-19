//! Feature toggles for runtime configuration.
//!
//! **The Hacker's Tool**: Quick runtime changes without recompilation or ceremony.
//! Perfect for experimentation, A/B testing, performance tuning.
//!
//! **Design Philosophy**:
//! - Config system = Type-safe, validated, compile-time configuration
//! - Feature toggles = Runtime switches for quick iteration
//! - Both have their place: Config for production, Toggles for development/tuning
//!
//! **Rust Translation**:
//! Uses environment variables and atomic operations for thread-safe runtime control.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::RwLock;

/// Feature toggle enum - defines all available toggles.
///
/// Direct translation of `org.neo4j.gds.utils.GdsFeatureToggles`.
///
/// **Usage**:
/// ```ignore
/// if FeatureToggles::USE_BIT_ID_MAP.is_enabled() {
///     // Use optimized bit ID map
/// }
///
/// FeatureToggles::USE_PACKED_ADJACENCY_LIST.toggle(true);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureToggle {
    UseBitIdMap,
    UseUncompressedAdjacencyList,
    UsePackedAdjacencyList,
    UseMixedAdjacencyList,
    UseReorderedAdjacencyList,
    EnableArrowDatabaseImport,
    FailOnProgressTrackerErrors,
    EnableAdjacencyCompressionMemoryTracking,
}

impl FeatureToggle {
    /// Get the default value for this toggle.
    pub fn default_value(&self) -> bool {
        match self {
            Self::UseBitIdMap => true,
            Self::UseUncompressedAdjacencyList => false,
            Self::UsePackedAdjacencyList => false,
            Self::UseMixedAdjacencyList => false,
            Self::UseReorderedAdjacencyList => false,
            Self::EnableArrowDatabaseImport => true,
            Self::FailOnProgressTrackerErrors => false,
            Self::EnableAdjacencyCompressionMemoryTracking => false,
        }
    }

    /// Get the environment variable name for this toggle.
    fn env_name(&self) -> String {
        let name = match self {
            Self::UseBitIdMap => "useBitIdMap",
            Self::UseUncompressedAdjacencyList => "useUncompressedAdjacencyList",
            Self::UsePackedAdjacencyList => "usePackedAdjacencyList",
            Self::UseMixedAdjacencyList => "useMixedAdjacencyList",
            Self::UseReorderedAdjacencyList => "useReorderedAdjacencyList",
            Self::EnableArrowDatabaseImport => "enableArrowDatabaseImport",
            Self::FailOnProgressTrackerErrors => "failOnProgressTrackerErrors",
            Self::EnableAdjacencyCompressionMemoryTracking => {
                "enableAdjacencyCompressionMemoryTracking"
            }
        };
        format!("org.neo4j.gds.utils.GdsFeatureToggles.{}", name)
    }

    /// Check if this toggle is enabled.
    pub fn is_enabled(&self) -> bool {
        FEATURE_TOGGLES.is_enabled(*self)
    }

    /// Check if this toggle is disabled.
    pub fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }

    /// Set the toggle value, returning the previous value.
    pub fn toggle(&self, value: bool) -> bool {
        FEATURE_TOGGLES.toggle(*self, value)
    }

    /// Reset to default value.
    pub fn reset(&self) {
        let default = self.default_value();
        self.toggle(default);
    }

    /// Enable the toggle and run code, restoring previous value after.
    ///
    /// **Test Pattern**: Useful for testing with specific feature configurations.
    pub fn enable_and_run<F, R>(&self, code: F) -> R
    where
        F: FnOnce() -> R,
    {
        let before = self.toggle(true);
        let result = code();
        self.toggle(before);
        result
    }

    /// Disable the toggle and run code, restoring previous value after.
    pub fn disable_and_run<F, R>(&self, code: F) -> R
    where
        F: FnOnce() -> R,
    {
        let before = self.toggle(false);
        let result = code();
        self.toggle(before);
        result
    }
}

/// Adjacency packing strategy enum.
///
/// Determines how adjacency data is packed for memory efficiency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdjacencyPackingStrategy {
    BlockAlignedTail,
    VarLongTail,
    PackedTail,
    InlinedHeadPackedTail,
}

impl Default for AdjacencyPackingStrategy {
    fn default() -> Self {
        Self::InlinedHeadPackedTail
    }
}

/// Global feature toggles state.
struct FeatureTogglesState {
    toggles: [AtomicBool; 8],
    pages_per_thread: AtomicI32,
    adjacency_packing_strategy: RwLock<AdjacencyPackingStrategy>,
}

impl FeatureTogglesState {
    fn new() -> Self {
        // Initialize from environment variables
        let use_bit_id_map = Self::read_env_bool(
            FeatureToggle::UseBitIdMap.env_name(),
            FeatureToggle::UseBitIdMap.default_value(),
        );
        let use_uncompressed = Self::read_env_bool(
            FeatureToggle::UseUncompressedAdjacencyList.env_name(),
            FeatureToggle::UseUncompressedAdjacencyList.default_value(),
        );
        let use_packed = Self::read_env_bool(
            FeatureToggle::UsePackedAdjacencyList.env_name(),
            FeatureToggle::UsePackedAdjacencyList.default_value(),
        );
        let use_mixed = Self::read_env_bool(
            FeatureToggle::UseMixedAdjacencyList.env_name(),
            FeatureToggle::UseMixedAdjacencyList.default_value(),
        );
        let use_reordered = Self::read_env_bool(
            FeatureToggle::UseReorderedAdjacencyList.env_name(),
            FeatureToggle::UseReorderedAdjacencyList.default_value(),
        );
        let enable_arrow = Self::read_env_bool(
            FeatureToggle::EnableArrowDatabaseImport.env_name(),
            FeatureToggle::EnableArrowDatabaseImport.default_value(),
        );
        let fail_on_errors = Self::read_env_bool(
            FeatureToggle::FailOnProgressTrackerErrors.env_name(),
            FeatureToggle::FailOnProgressTrackerErrors.default_value(),
        );
        let enable_compression_tracking = Self::read_env_bool(
            FeatureToggle::EnableAdjacencyCompressionMemoryTracking.env_name(),
            FeatureToggle::EnableAdjacencyCompressionMemoryTracking.default_value(),
        );

        let pages_per_thread = Self::read_env_int(
            "org.neo4j.gds.utils.GdsFeatureToggles.pagesPerThread".to_string(),
            4,
        );

        Self {
            toggles: [
                AtomicBool::new(use_bit_id_map),
                AtomicBool::new(use_uncompressed),
                AtomicBool::new(use_packed),
                AtomicBool::new(use_mixed),
                AtomicBool::new(use_reordered),
                AtomicBool::new(enable_arrow),
                AtomicBool::new(fail_on_errors),
                AtomicBool::new(enable_compression_tracking),
            ],
            pages_per_thread: AtomicI32::new(pages_per_thread),
            adjacency_packing_strategy: RwLock::new(AdjacencyPackingStrategy::default()),
        }
    }

    fn is_enabled(&self, toggle: FeatureToggle) -> bool {
        let index = toggle as usize;
        self.toggles[index].load(Ordering::Relaxed)
    }

    fn toggle(&self, toggle: FeatureToggle, value: bool) -> bool {
        let index = toggle as usize;
        self.toggles[index].swap(value, Ordering::Relaxed)
    }

    fn read_env_bool(name: String, default: bool) -> bool {
        match std::env::var(&name) {
            Ok(val) => {
                let val_lower = val.to_lowercase();
                if default {
                    val_lower != "false"
                } else {
                    val_lower == "true"
                }
            }
            Err(_) => default,
        }
    }

    fn read_env_int(name: String, default: i32) -> i32 {
        match std::env::var(&name) {
            Ok(val) => val.parse().unwrap_or(default),
            Err(_) => default,
        }
    }
}

// Global singleton
lazy_static::lazy_static! {
    static ref FEATURE_TOGGLES: FeatureTogglesState = FeatureTogglesState::new();
}

/// Global configuration values (not toggles, but related).
pub struct FeatureConfig;

impl FeatureConfig {
    /// How many pages per loading thread.
    ///
    /// **Tuning**: Pages are locked while written to, so:
    /// - More pages = less contention
    /// - Fewer pages = higher throughput
    pub const PAGES_PER_THREAD_DEFAULT: usize = 4;

    /// Get the current pages per thread setting.
    pub fn pages_per_thread() -> i32 {
        FEATURE_TOGGLES.pages_per_thread.load(Ordering::Relaxed)
    }

    /// Set pages per thread (runtime override).
    pub fn set_pages_per_thread(value: i32) {
        FEATURE_TOGGLES
            .pages_per_thread
            .store(value, Ordering::Relaxed);
    }

    /// Get the adjacency packing strategy.
    pub fn adjacency_packing_strategy() -> AdjacencyPackingStrategy {
        *FEATURE_TOGGLES.adjacency_packing_strategy.read().unwrap()
    }

    /// Set the adjacency packing strategy (runtime override).
    pub fn set_adjacency_packing_strategy(strategy: AdjacencyPackingStrategy) {
        *FEATURE_TOGGLES.adjacency_packing_strategy.write().unwrap() = strategy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        // UseBitIdMap defaults to true
        assert!(FeatureToggle::UseBitIdMap.default_value());

        // UsePackedAdjacencyList defaults to false
        assert!(!FeatureToggle::UsePackedAdjacencyList.default_value());
    }

    #[test]
    fn test_toggle() {
        let toggle = FeatureToggle::UseMixedAdjacencyList;
        let original = toggle.is_enabled();

        // Toggle to true
        toggle.toggle(true);
        assert!(toggle.is_enabled());
        assert!(!toggle.is_disabled());

        // Toggle to false
        toggle.toggle(false);
        assert!(toggle.is_disabled());

        // Restore original
        toggle.toggle(original);
    }

    #[test]
    fn test_reset() {
        let toggle = FeatureToggle::UseReorderedAdjacencyList;

        // Change from default
        toggle.toggle(!toggle.default_value());
        assert_eq!(toggle.is_enabled(), !toggle.default_value());

        // Reset to default
        toggle.reset();
        assert_eq!(toggle.is_enabled(), toggle.default_value());
    }

    #[test]
    fn test_enable_and_run() {
        let toggle = FeatureToggle::FailOnProgressTrackerErrors;
        let original = toggle.is_enabled();

        // Run with toggle enabled
        let result = toggle.enable_and_run(|| {
            assert!(toggle.is_enabled());
            42
        });

        assert_eq!(result, 42);
        assert_eq!(toggle.is_enabled(), original); // Restored
    }

    #[test]
    fn test_disable_and_run() {
        let toggle = FeatureToggle::EnableArrowDatabaseImport;
        let original = toggle.is_enabled();

        // Run with toggle disabled
        let result = toggle.disable_and_run(|| {
            assert!(toggle.is_disabled());
            "test"
        });

        assert_eq!(result, "test");
        assert_eq!(toggle.is_enabled(), original); // Restored
    }

    #[test]
    fn test_pages_per_thread() {
        let original = FeatureConfig::pages_per_thread();

        FeatureConfig::set_pages_per_thread(8);
        assert_eq!(FeatureConfig::pages_per_thread(), 8);

        // Restore
        FeatureConfig::set_pages_per_thread(original);
    }

    #[test]
    fn test_adjacency_packing_strategy() {
        let original = FeatureConfig::adjacency_packing_strategy();

        FeatureConfig::set_adjacency_packing_strategy(AdjacencyPackingStrategy::VarLongTail);
        assert_eq!(
            FeatureConfig::adjacency_packing_strategy(),
            AdjacencyPackingStrategy::VarLongTail
        );

        // Restore
        FeatureConfig::set_adjacency_packing_strategy(original);
    }

    #[test]
    fn test_adjacency_strategy_default() {
        assert_eq!(
            AdjacencyPackingStrategy::default(),
            AdjacencyPackingStrategy::InlinedHeadPackedTail
        );
    }
}

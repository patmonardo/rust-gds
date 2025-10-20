use std::marker::PhantomData;

/// Interface for building results from algorithm execution.
/// This is a core pattern in the Applications system for transforming
/// algorithm results into different output formats.
pub trait ResultBuilder<CONFIG, RESULT, OUTPUT, META> {
    /// Builds the final result from the algorithm execution.
    /// 
    /// # Arguments
    /// * `config` - The algorithm configuration
    /// * `result` - The algorithm result
    /// * `meta` - Metadata about the execution (e.g., properties written)
    /// 
    /// # Returns
    /// The final output result
    fn build(&self, config: &CONFIG, result: RESULT, meta: META) -> OUTPUT;
}

/// Generic result builder that can be used for simple cases.
pub struct GenericResultBuilder<F, CONFIG, RESULT, OUTPUT, META>
where
    F: Fn(&CONFIG, RESULT, META) -> OUTPUT,
{
    build_fn: F,
    _phantom: PhantomData<(CONFIG, RESULT, OUTPUT, META)>,
}

impl<F, CONFIG, RESULT, OUTPUT, META> GenericResultBuilder<F, CONFIG, RESULT, OUTPUT, META>
where
    F: Fn(&CONFIG, RESULT, META) -> OUTPUT,
{
    pub fn new(build_fn: F) -> Self {
        Self { 
            build_fn,
            _phantom: PhantomData,
        }
    }
}

impl<F, CONFIG, RESULT, OUTPUT, META> ResultBuilder<CONFIG, RESULT, OUTPUT, META> 
    for GenericResultBuilder<F, CONFIG, RESULT, OUTPUT, META>
where
    F: Fn(&CONFIG, RESULT, META) -> OUTPUT,
{
    fn build(&self, config: &CONFIG, result: RESULT, meta: META) -> OUTPUT {
        (self.build_fn)(config, result, meta)
    }
}

/// Specialized result builders for different execution modes.

/// Result builder for streaming results.
pub trait StreamResultBuilder<RESULT, OUTPUT> {
    fn build_stream(&self, result: RESULT) -> OUTPUT;
}

/// Result builder for statistics results.
pub trait StatsResultBuilder<RESULT, OUTPUT> {
    fn build_stats(&self, result: RESULT) -> OUTPUT;
}

/// Result builder for mutation results.
pub trait MutateResultBuilder<CONFIG, RESULT, OUTPUT, META> {
    fn build_mutate(&self, config: &CONFIG, result: RESULT, meta: META) -> OUTPUT;
}

/// Result builder for write results.
pub trait WriteResultBuilder<CONFIG, RESULT, OUTPUT, META> {
    fn build_write(&self, config: &CONFIG, result: RESULT, meta: META) -> OUTPUT;
}

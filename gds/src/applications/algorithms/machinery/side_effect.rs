use crate::core::loading::GraphResources;
use crate::applications::algorithms::machinery::{MutateStep, WriteStep};
use crate::core::utils::progress::JobId;

/// Side Effect - represents side effects that can be applied after algorithm execution
pub trait SideEffect<RESULT_FROM_ALGORITHM, METADATA> {
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
    ) -> Option<METADATA>;
}

/// Side Effect Executor - reusable boilerplate for executing side effects
pub struct SideEffectExecutor;

impl SideEffectExecutor {
    pub fn new() -> Self {
        Self
    }

    /// Reusable boilerplate for executing side effects
    pub fn execute_side_effect<RESULT_FROM_ALGORITHM, METADATA>(
        &self,
        result: Option<RESULT_FROM_ALGORITHM>,
        side_effect: impl Fn(RESULT_FROM_ALGORITHM) -> METADATA,
    ) -> Option<METADATA> {
        if result.is_none() {
            return None;
        }

        let result_from_algorithm = result.unwrap();
        let metadata = side_effect(result_from_algorithm);
        Some(metadata)
    }
}

impl Default for SideEffectExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutate Side Effect - wraps a MutateStep as a SideEffect
pub struct MutateSideEffect<RESULT_FROM_ALGORITHM, MUTATE_METADATA> {
    side_effect_executor: SideEffectExecutor,
    mutate_step: Box<dyn MutateStep<RESULT_FROM_ALGORITHM, MUTATE_METADATA>>,
}

impl<RESULT_FROM_ALGORITHM, MUTATE_METADATA> MutateSideEffect<RESULT_FROM_ALGORITHM, MUTATE_METADATA> {
    pub fn new(mutate_step: Box<dyn MutateStep<RESULT_FROM_ALGORITHM, MUTATE_METADATA>>) -> Self {
        Self {
            side_effect_executor: SideEffectExecutor::new(),
            mutate_step,
        }
    }
}

impl<RESULT_FROM_ALGORITHM, MUTATE_METADATA> SideEffect<RESULT_FROM_ALGORITHM, MUTATE_METADATA> 
    for MutateSideEffect<RESULT_FROM_ALGORITHM, MUTATE_METADATA> 
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
    ) -> Option<MUTATE_METADATA> {
        self.side_effect_executor.execute_side_effect(result, |r| {
            self.mutate_step.execute(
                graph_resources.graph.clone(),
                graph_resources.graph_store.clone(),
                r,
            )
        })
    }
}

/// Write Side Effect - wraps a WriteStep as a SideEffect
pub struct WriteSideEffect<RESULT_FROM_ALGORITHM, WRITE_METADATA> {
    side_effect_executor: SideEffectExecutor,
    job_id: crate::core::utils::progress::JobId,
    write_step: Box<dyn WriteStep<RESULT_FROM_ALGORITHM, WRITE_METADATA>>,
}

impl<RESULT_FROM_ALGORITHM, WRITE_METADATA> WriteSideEffect<RESULT_FROM_ALGORITHM, WRITE_METADATA> {
    pub fn new(
        job_id: crate::core::utils::progress::JobId,
        write_step: Box<dyn WriteStep<RESULT_FROM_ALGORITHM, WRITE_METADATA>>,
    ) -> Self {
        Self {
            side_effect_executor: SideEffectExecutor::new(),
            job_id,
            write_step,
        }
    }
}

impl<RESULT_FROM_ALGORITHM, WRITE_METADATA> SideEffect<RESULT_FROM_ALGORITHM, WRITE_METADATA> 
    for WriteSideEffect<RESULT_FROM_ALGORITHM, WRITE_METADATA> 
{
    fn process(
        &self,
        graph_resources: &GraphResources,
        result: Option<RESULT_FROM_ALGORITHM>,
    ) -> Option<WRITE_METADATA> {
        self.side_effect_executor.execute_side_effect(result, |r| {
            self.write_step.execute(
                graph_resources.graph.clone(),
                graph_resources.graph_store.clone(),
                graph_resources.result_store.as_ref().map(|rs| rs.as_ref()),
                r,
                self.job_id.clone(),
            )
        })
    }
}

use crate::api::Graph;
use crate::api::GraphStore;
use crate::api::ResultStore;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::core::utils::progress::JobId;
use std::marker::PhantomData;

/// Interface for write steps that write algorithm results to the database.
/// This is a core pattern in the Applications system for handling
/// algorithm results that need to be persisted to the database.
pub trait WriteStep<RESULT, META> {
    /// Executes the write step, persisting the algorithm result to the database.
    /// 
    /// # Arguments
    /// * `graph` - The graph that was processed
    /// * `graph_store` - The graph store containing the graph
    /// * `result_store` - The result store for caching results
    /// * `result` - The algorithm result to persist
    /// * `job_id` - The job ID for tracking progress
    /// 
    /// # Returns
    /// Metadata about what was written (e.g., number of properties written)
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        result: RESULT,
        job_id: JobId,
    ) -> META;
}

/// Generic write step that can be used for simple cases.
pub struct GenericWriteStep<F, RESULT, META>
where
    F: Fn(&Graph, &GraphStore, &mut ResultStore, RESULT, JobId) -> META,
{
    execute_fn: F,
    _phantom: PhantomData<(RESULT, META)>,
}

impl<F, RESULT, META> GenericWriteStep<F, RESULT, META>
where
    F: Fn(&Graph, &GraphStore, &mut ResultStore, RESULT, JobId) -> META,
{
    pub fn new(execute_fn: F) -> Self {
        Self { 
            execute_fn,
            _phantom: PhantomData,
        }
    }
}

impl<F, RESULT, META> WriteStep<RESULT, META> for GenericWriteStep<F, RESULT, META>
where
    F: Fn(&Graph, &GraphStore, &mut ResultStore, RESULT, JobId) -> META,
{
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        result: RESULT,
        job_id: JobId,
    ) -> META {
        (self.execute_fn)(graph, graph_store, result_store, result, job_id)
    }
}

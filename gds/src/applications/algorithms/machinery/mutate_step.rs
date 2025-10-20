use crate::api::Graph;
use crate::api::GraphStore;
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use std::marker::PhantomData;

/// Interface for mutation steps that modify graph properties.
/// This is a core pattern in the Applications system for handling
/// algorithm results that need to be written back to the graph.
pub trait MutateStep<RESULT, META> {
    /// Executes the mutation step, applying the algorithm result to the graph.
    /// 
    /// # Arguments
    /// * `graph` - The graph to mutate
    /// * `graph_store` - The graph store containing the graph
    /// * `result` - The algorithm result to apply
    /// 
    /// # Returns
    /// Metadata about what was written (e.g., number of properties written)
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &mut GraphStore,
        result: RESULT,
    ) -> META;
}

/// Generic mutation step that can be used for simple cases.
pub struct GenericMutateStep<F, RESULT, META>
where
    F: Fn(&Graph, &mut GraphStore, RESULT) -> META,
{
    execute_fn: F,
    _phantom: PhantomData<(RESULT, META)>,
}

impl<F, RESULT, META> GenericMutateStep<F, RESULT, META>
where
    F: Fn(&Graph, &mut GraphStore, RESULT) -> META,
{
    pub fn new(execute_fn: F) -> Self {
        Self { 
            execute_fn,
            _phantom: PhantomData,
        }
    }
}

impl<F, RESULT, META> MutateStep<RESULT, META> for GenericMutateStep<F, RESULT, META>
where
    F: Fn(&Graph, &mut GraphStore, RESULT) -> META,
{
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &mut GraphStore,
        result: RESULT,
    ) -> META {
        (self.execute_fn)(graph, graph_store, result)
    }
}

//! PageRank with PropertyStore Integration
//!
//! This example demonstrates the REAL issue we're solving:
//! How to bridge PropertyStore (persistent, schema-constrained) with
//! Pregel (ephemeral, algorithm-specific computation).
//!
//! ## The Problem
//!
//! Currently, InitContext has NO access to PropertyStore!
//! We need to show how PropertyProjection can bridge this gap.
//!
//! ## Run this example
//!
//! ```bash
//! cargo run --example pregel_pagerank_with_propertystore --features core
//! ```

use rust_gds::pregel::{DefaultValue, PregelSchema};
use rust_gds::projection::PropertyDescriptor;
use rust_gds::types::random::{random_graph_store, RandomGraphConfig};
use rust_gds::types::ValueType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  PageRank with PropertyStore Integration");
    println!("═══════════════════════════════════════════════════════════════\n");

    // ========================================================================
    // STEP 1: Create graph and PropertyStore with initial values
    // ========================================================================

    println!("Step 1: Create graph and initialize PropertyStore");
    println!("───────────────────────────────────────────────────────────────");

    let config = RandomGraphConfig::seeded(42)
        .with_node_count(10)
        .with_relationship_count(20);

    let mut graph = random_graph_store(config);

    // Register a property in PropertyStore
    println!(
        "  ✓ Created graph: {} nodes, {} relationships",
        graph.node_count(),
        graph.relationship_count()
    );

    // Register property descriptor
    let seed_descriptor = PropertyDescriptor::new(1, "seed_rank", ValueType::Double);

    println!("  ✓ Registering property: 'seed_rank' (Double)");
    // Note: This would be: graph.register_property(seed_descriptor)?;
    // For now, we'll simulate having initial values

    println!();

    // ========================================================================
    // STEP 2: Define Pregel Schema (currently manual, no PropertyStore link)
    // ========================================================================

    println!("Step 2: Define Pregel Schema");
    println!("───────────────────────────────────────────────────────────────");

    let schema = PregelSchema::builder()
        .add_public("rank", ValueType::Double)
        .add_with_default(
            "prev_rank",
            DefaultValue::Double(0.0),
            rust_gds::pregel::Visibility::Private,
        )
        .build();

    println!("  ✓ Schema defined:");
    println!("    - rank (Double, Public) ← will be final result");
    println!("    - prev_rank (Double, Private) ← scratch space");
    println!();

    // ========================================================================
    // STEP 3: Show the PROBLEM - InitContext has no PropertyStore access
    // ========================================================================

    println!("Step 3: The Problem - InitContext Limitation");
    println!("───────────────────────────────────────────────────────────────");
    println!("  ❌ InitContext signature:");
    println!("     pub struct InitContext<C: PregelConfig> {{");
    println!("         node_id: u64,");
    println!("         node_values: &mut NodeValue,  // ← Only Pregel storage!");
    println!("         // NO graph, NO PropertyStore access!");
    println!("     }}");
    println!();
    println!("  This means we CANNOT read 'seed_rank' from PropertyStore");
    println!("  during initialization! We're stuck with manual defaults.");
    println!();

    // ========================================================================
    // STEP 4: Show PropertyProjection (the bridge we built)
    // ========================================================================

    println!("Step 4: The Solution - PropertyProjection Trait");
    println!("───────────────────────────────────────────────────────────────");
    println!("  ✓ We implemented PropertyProjection trait:");
    println!("    trait PropertyProjection {{");
    println!("        fn from_property(");
    println!("            props: &dyn NodePropertyValues,");
    println!("            node_id: u64");
    println!("        ) -> Option<Self>;");
    println!("    }}");
    println!();
    println!("  ✓ Implemented for DefaultValue (Pregel's runtime value type)");
    println!();

    // ========================================================================
    // STEP 5: Simulate what we WANT to be able to do
    // ========================================================================

    println!("Step 5: What We WANT to Do (Not Yet Wired)");
    println!("───────────────────────────────────────────────────────────────");
    println!("  Ideal code:");
    println!();
    println!("  // In PregelSchema builder:");
    println!("  let schema = PregelSchema::builder()");
    println!("      .add_public(\"rank\", ValueType::Double)");
    println!("      .with_property_source(\"rank\", \"seed_rank\")  // ← Link to PropertyStore!");
    println!("      .build();");
    println!();
    println!("  // InitContext would auto-populate from PropertyStore:");
    println!("  fn init(context: &mut InitContext) {{");
    println!("      // 'rank' already initialized from 'seed_rank' if available!");
    println!("      let initial = context.double_node_value(\"rank\")");
    println!("          .unwrap_or(1.0);  // Fallback only if PropertyStore empty");
    println!("  }}");
    println!();

    // ========================================================================
    // STEP 6: Show current manual workaround
    // ========================================================================

    println!("Step 6: Current Reality - Manual Workaround");
    println!("───────────────────────────────────────────────────────────────");
    println!("  What we have to do NOW:");
    println!();

    // Simulate having PropertyStore values
    let simulated_seed_ranks = vec![0.5, 0.8, 1.2, 0.9, 1.1, 0.7, 1.0, 0.6, 0.9, 1.3];

    println!("  Simulated PropertyStore 'seed_rank' values:");
    for (i, rank) in simulated_seed_ranks.iter().enumerate() {
        println!("    Node {}: {:.1}", i, rank);
    }
    println!();

    // Manual initialization (what we currently have to do)
    println!("  Must manually handle PropertyStore → Pregel projection:");
    println!("  - Read PropertyStore values outside Pregel");
    println!("  - Pass via closure captures");
    println!("  - Manually set in init function");
    println!();

    // ========================================================================
    // STEP 7: Show PropertyProjection usage (if we had PropertyValues)
    // ========================================================================

    println!("Step 7: PropertyProjection in Action");
    println!("───────────────────────────────────────────────────────────────");
    println!("  If we had NodePropertyValues from PropertyStore:");
    println!();
    println!("  let props = graph.get_node_property_values(\"seed_rank\")?;");
    println!("  for node_id in 0..graph.node_count() {{");
    println!("      if let Some(initial) = DefaultValue::from_property(&*props, node_id) {{");
    println!("          // Use initial value from PropertyStore");
    println!("          context.set_node_value(\"rank\", initial);");
    println!("      }} else {{");
    println!("          // Fallback to default");
    println!("          context.set_node_value(\"rank\", DefaultValue::Double(1.0));");
    println!("      }}");
    println!("  }}");
    println!();

    // ========================================================================
    // STEP 8: Simulate PageRank computation
    // ========================================================================

    println!("Step 8: Simulate PageRank Computation");
    println!("───────────────────────────────────────────────────────────────");
    println!("  NOTE: Not actually running Pregel (would need full executor setup)");
    println!("  Just showing the data flow concept:");
    println!();

    // Simulate computed ranks after convergence
    let computed_ranks = vec![0.85, 1.23, 0.92, 1.45, 0.78, 1.12, 0.89, 1.34, 0.95, 1.01];

    println!("  Simulated final ranks after 20 iterations:");
    for (i, rank) in computed_ranks.iter().enumerate() {
        println!("    Node {}: {:.2}", i, rank);
    }
    println!();

    // ========================================================================
    // STEP 9: Show materialize_pregel_values (write back to PropertyStore)
    // ========================================================================

    println!("Step 9: Write Results Back to PropertyStore");
    println!("───────────────────────────────────────────────────────────────");
    println!("  We implemented materialize_pregel_values() for write-back:");
    println!();
    println!("  // Register target property");
    println!("  graph.register_property(");
    println!("      \"computed_rank\",");
    println!("      PropertyDescriptor::new(2, \"computed_rank\", ValueType::Double)");
    println!("  )?;");
    println!();
    println!("  // Materialize Pregel results → PropertyStore");
    println!("  materialize_pregel_values(");
    println!("      &mut graph,");
    println!("      \"computed_rank\",");
    println!("      result.node_values().iter_with_ids(),");
    println!("  )?;");
    println!();
    println!("  ✓ Results now persisted in PropertyStore!");
    println!();

    // ========================================================================
    // SUMMARY
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("  Summary: What's Missing");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("✓ HAVE: PropertyProjection trait (DefaultValue ↔ NodePropertyValues)");
    println!("✓ HAVE: materialize_pregel_values() (write back to PropertyStore)");
    println!();
    println!("✗ MISSING: InitContext access to PropertyStore");
    println!("✗ MISSING: PregelSchema.with_property_source() builder method");
    println!("✗ MISSING: Automatic initialization from PropertyStore");
    println!("✗ MISSING: result.materialize_to_property_store() convenience method");
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Next Steps");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("1. Add property_source field to PregelSchema::Element");
    println!("2. Add .with_property_source() to PregelSchemaBuilder");
    println!("3. Pass Graph to Pregel::new() (currently only Arc<dyn Graph>)");
    println!("4. Update InitContext to receive optional PropertyStore values");
    println!("5. Wire PropertyProjection in Pregel executor initialization");
    println!("6. Add convenience methods to PregelResult");
    println!();
    println!("This would enable:");
    println!("  PropertyStore → Pregel → Compute → Pregel → PropertyStore");
    println!("  (Complete loop with validation at boundaries)");
    println!();

    Ok(())
}

//! PageRank with PropertyStore Integration - Demonstration
//!
//! This example demonstrates the REAL problem we're trying to solve:
//! **How to bridge PropertyStore (persistent) with Pregel (ephemeral)**
//!
//! ## The Core Issue
//!
//! Pregel's `InitContext` has NO access to PropertyStore!
//! We need to show how `PropertyProjection` can bridge this gap.
//!
//! ## Run
//!
//! ```bash
//! cargo run --example pregel_propertystore_demo --features core
//! ```

use rust_gds::pregel::{DefaultValue, PregelSchema};
use rust_gds::projection::PropertyDescriptor;
use rust_gds::types::ValueType;

fn main() {
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  PageRank + PropertyStore: The Real Issue");
    println!("═══════════════════════════════════════════════════════════════\n");

    // ========================================================================
    // THE PROBLEM: Two Separate Schema Systems
    // ========================================================================

    println!("PROBLEM: Two Separate, Unconnected Schema Systems");
    println!("─────────────────────────────────────────────────────────────────\n");

    // PropertyStore schema (persistent, Form Processor validated)
    let propertystore_schema = PropertyDescriptor::new(1, "seed_rank", ValueType::Double);

    println!("PropertyStore Schema (Persistent/Gross):");
    println!("  Property: {}", propertystore_schema.name);
    println!("  Type: {:?}", propertystore_schema.value_type);
    println!("  Purpose: Store initial PageRank seeds\n");

    // Pregel schema (ephemeral, algorithm-specific)
    let pregel_schema = PregelSchema::builder()
        .add_public("rank", ValueType::Double)
        .add_with_default(
            "prev_rank",
            DefaultValue::Double(0.0),
            rust_gds::pregel::Visibility::Private,
        )
        .build();

    println!("Pregel Schema (Ephemeral/Subtle):");
    println!("  Properties: {} total", pregel_schema.elements().len());
    for element in pregel_schema.elements() {
        println!(
            "    - {} ({:?}, {:?})",
            element.property_key, element.property_type, element.visibility
        );
    }
    println!("  Purpose: Algorithm computation state\n");

    println!("❌ NO CONNECTION between these two schemas!");
    println!("❌ PropertyStore 'seed_rank' ≠ Pregel 'rank' (just strings!)\n");

    // ========================================================================
    // THE GAP: InitContext Has No PropertyStore Access
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("THE GAP: InitContext Cannot Access PropertyStore");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Current InitContext signature:");
    println!("  pub struct InitContext<C: PregelConfig> {{");
    println!("      node_id: u64,");
    println!("      node_values: &mut NodeValue,  // ← Only Pregel storage");
    println!("      config: &C,");
    println!("      // ❌ NO graph reference!");
    println!("      // ❌ NO PropertyStore access!");
    println!("  }}\n");

    println!("This means the init function CANNOT do:");
    println!("  fn init(context: &mut InitContext) {{");
    println!("      let seed = graph.get_property(node_id, \"seed_rank\");  // ❌");
    println!("      context.set_node_value(\"rank\", seed);");
    println!("  }}\n");

    println!("We're stuck with hardcoded defaults:");
    println!("  fn init(context: &mut InitContext) {{");
    println!("      context.set_node_value(\"rank\", 1.0);  // ← Always 1.0!");
    println!("  }}\n");

    // ========================================================================
    // THE SOLUTION WE BUILT: PropertyProjection
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("SOLUTION WE BUILT: PropertyProjection Trait");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✓ Created PropertyProjection trait:");
    println!("  trait PropertyProjection {{");
    println!("      fn from_property(");
    println!("          props: &dyn NodePropertyValues,");
    println!("          node_id: u64");
    println!("      ) -> Option<Self>;");
    println!("  }}\n");

    println!("✓ Implemented for DefaultValue (Pregel runtime values):");
    println!("  impl PropertyProjection for DefaultValue {{");
    println!("      fn from_property(props, node_id) -> Option<Self> {{");
    println!("          match props.value_type() {{");
    println!("              ValueType::Double => props.double_value(node_id)");
    println!("                  .ok().map(DefaultValue::Double),");
    println!("              // ... other types");
    println!("          }}");
    println!("      }}");
    println!("  }}\n");

    println!("✓ Created materialize_pregel_values() for write-back\n");

    // ========================================================================
    // WHAT'S MISSING: Wiring It Together
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("WHAT'S MISSING: Wiring PropertyProjection into Pregel");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Need to add to PregelSchema:");
    println!("  1. Property source mapping (\"rank\" ← \"seed_rank\")");
    println!("  2. Builder method: .with_property_source(pregel_key, store_key)\n");

    println!("Need to update Pregel executor:");
    println!("  3. Accept Graph (not just Arc<dyn Graph>)");
    println!("  4. Read PropertyStore values during initialization");
    println!("  5. Use PropertyProjection to convert values");
    println!("  6. Pass to InitContext (or pre-populate NodeValue)\n");

    println!("Need convenience methods:");
    println!("  7. result.materialize_to_property_store(graph, mapping)\n");

    // ========================================================================
    // DESIRED API
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("DESIRED API (After Wiring)");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("// Step 1: Define schema WITH PropertyStore link");
    println!("let schema = PregelSchema::builder()");
    println!("    .add_public(\"rank\", ValueType::Double)");
    println!("    .with_property_source(\"rank\", \"seed_rank\")  // ← Link!");
    println!("    .build();\n");

    println!("// Step 2: Run Pregel (auto-loads from PropertyStore)");
    println!("let result = Pregel::new(graph, config, schema, init, compute, ...)");
    println!("    .run()?;\n");

    println!("// Step 3: Init gets values automatically!");
    println!("fn init(context: &mut InitContext) {{");
    println!("    // 'rank' already populated from 'seed_rank' if available");
    println!("    let rank = context.double_node_value(\"rank\")");
    println!("        .unwrap_or(1.0);  // Fallback only if missing");
    println!("}}\n");

    println!("// Step 4: Write results back");
    println!("result.materialize_to_property_store(");
    println!("    &graph,");
    println!("    \"rank\" -> \"computed_rank\"  // Write as new property");
    println!(")?;\n");

    // ========================================================================
    // CURRENT vs EVAL_MACRO
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("NOTE: This is About PREGEL, Not eval_macro");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("Two separate systems:");
    println!();
    println!("  1. PREGEL (what we're fixing NOW):");
    println!("     - PregelSchema → NodeValue → ComputeContext");
    println!("     - PropertyStore integration (the gap we're filling)");
    println!("     - PropertyProjection trait (bridge we built)");
    println!();
    println!("  2. EVAL_MACRO (future/speculative):");
    println!("     - value_type_table! macro DSL");
    println!("     - Unified PropertyDescriptor generation");
    println!("     - Form Processor functors");
    println!("     - NOT needed for Pregel PropertyStore integration!");
    println!();
    println!("The eval_macro is a 'massive speculative bubble' for future");
    println!("unification of PropertyStore schema generation. We DON'T need");
    println!("it to fix the Pregel PropertyStore integration issue.\n");

    // ========================================================================
    // SUMMARY
    // ========================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("SUMMARY: What to Implement Next");
    println!("═══════════════════════════════════════════════════════════════\n");

    println!("✓ DONE: PropertyProjection trait (src/pregel/projection.rs)");
    println!("✓ DONE: materialize_pregel_values() function");
    println!();
    println!("TODO (Concrete Steps):");
    println!();
    println!("  1. Add property_source: Option<String> to PregelSchema::Element");
    println!("  2. Add .with_property_source(key, source) to PregelSchemaBuilder");
    println!("  3. Update Pregel::new() to accept GraphStore (not just Graph trait)");
    println!("  4. In Pregel initialization, before calling init_fn:");
    println!("     - Check schema for property_source mappings");
    println!("     - Load PropertyStore values via graph.get_property_values()");
    println!("     - Use PropertyProjection to convert");
    println!("     - Pre-populate NodeValue storage");
    println!("  5. Add PregelResult::materialize_to() convenience method");
    println!();
    println!("This enables: PropertyStore → Pregel → Compute → PropertyStore");
    println!("(Complete validated loop)\n");

    println!("═══════════════════════════════════════════════════════════════");
    println!("Ready to implement? (Not just document!)");
    println!("═══════════════════════════════════════════════════════════════\n");
}

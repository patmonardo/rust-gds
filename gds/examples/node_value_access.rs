//! Node Value Access Example
//!
//! Demonstrates how algorithms should access node properties through
//! the GdsValue "Type Machine" instead of direct typed accessors.
//!
//! Run with: cargo run --example node_value_access

use gds::types::graph::id_map::{NodeIterator, SimpleIdMap};

fn main() {
    println!("=== Node Value Access Pattern ===\n");

    example_1_current_pattern();
    println!();

    example_2_ideal_pattern();
    println!();

    example_3_algorithm_usage();
}

/// CURRENT PATTERN: Direct typed accessors (bypasses type machine)
fn example_1_current_pattern() {
    println!("1. CURRENT PATTERN (No Type Machine)");
    println!("-------------------------------------");

    // Simulated graph with node properties
    println!("Graph has 3 nodes with 'age' property:");
    println!("  Node 0: age = 25");
    println!("  Node 1: age = 30");
    println!("  Node 2: age = 35");
    println!();

    println!("Algorithm accesses values:");
    println!("  ❌ props.long_value(node_id)  // Direct typed access");
    println!("  ❌ No type validation");
    println!("  ❌ No unified type system");
    println!("  ❌ GdsValue bypassed completely!");
}

/// IDEAL PATTERN: GdsValue type machine
fn example_2_ideal_pattern() {
    println!("2. IDEAL PATTERN (Through Type Machine)");
    println!("----------------------------------------");

    println!("Graph has 3 nodes with 'score' property:");
    println!("  Node 0: score = 0.85 (Double)");
    println!("  Node 1: score = 100 (Long, needs conversion)");
    println!("  Node 2: score = [0.8, 0.9] (Array)");
    println!();

    println!("Algorithm accesses values through GdsValue:");
    println!("  ✅ gds_value = props.gds_value(node_id)");
    println!("  ✅ match gds_value {{ ... }}  // Type-safe extraction");
    println!("  ✅ Runtime type validation");
    println!("  ✅ Explicit conversions");
    println!("  ✅ All data flows through type machine!");
}

/// Example: Simple algorithm showing iteration + property access
fn example_3_algorithm_usage() {
    println!("3. ALGORITHM PATTERN");
    println!("--------------------");

    // Create simple ID map
    let id_map = SimpleIdMap::from_original_ids([0, 1, 2]);

    println!("Iterating nodes in pure ID space:");
    for node_id in id_map.iter() {
        println!("  Processing node_id: {}", node_id);
    }
    println!();

    println!("CURRENT: Algorithm gets typed values directly");
    println!("  for node_id in graph.iter() {{");
    println!("    let age = age_props.long_value(node_id)?;  // ❌ No type machine");
    println!("    process(age);");
    println!("  }}");
    println!();

    println!("IDEAL: Algorithm goes through GdsValue");
    println!("  for node_id in graph.iter() {{");
    println!("    let value = age_props.gds_value(node_id)?;  // ✅ Type machine!");
    println!("    match value {{");
    println!("      GdsValue::Long(age) => process_long(age),");
    println!("      GdsValue::Double(age) => process_double(age),");
    println!("      _ => handle_unexpected(),");
    println!("    }}");
    println!("  }}");
}

/// Pseudo-code showing how PageRank would use GdsValue
#[allow(dead_code)]
fn pagerank_example_pseudo() {
    println!("PSEUDO-CODE: PageRank with GdsValue Type Machine");
    println!();
    println!("fn compute_pagerank(graph: &DefaultGraph) -> HashMap<u64, f64> {{");
    println!("    let mut ranks = HashMap::new();");
    println!("    ");
    println!("    // 1. Pure ID iteration");
    println!("    for node_id in graph.iter() {{");
    println!("        ");
    println!("        // 2. Get columnar storage");
    println!("        if let Some(props) = graph.node_properties(\"initial_rank\") {{");
    println!("            ");
    println!("            // 3. ACCESS THROUGH TYPE MACHINE!");
    println!("            match props.gds_value(node_id) {{");
    println!("                Ok(GdsValue::Double(rank)) => {{");
    println!("                    ranks.insert(node_id, rank);  // ✅ Type-safe!");
    println!("                }}");
    println!("                Ok(GdsValue::Long(rank)) => {{");
    println!("                    ranks.insert(node_id, rank as f64);  // ✅ Explicit conversion");
    println!("                }}");
    println!("                Ok(other) => {{");
    println!("                    eprintln!(\"Unexpected type: {{}}\", other.value_type());");
    println!("                    ranks.insert(node_id, 1.0);");
    println!("                }}");
    println!("                Err(e) => {{");
    println!("                    eprintln!(\"Error: {{}}\", e);");
    println!("                    ranks.insert(node_id, 1.0);");
    println!("                }}");
    println!("            }}");
    println!("        }} else {{");
    println!("            ranks.insert(node_id, 1.0);  // Default");
    println!("        }}");
    println!("    }}");
    println!("    ");
    println!("    ranks");
    println!("}}");
}

// TODO: Once gds_value() method is added to NodePropertyValues,
// this example can be updated to actually demonstrate the type machine!

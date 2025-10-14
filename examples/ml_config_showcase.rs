//! Demonstrates ML-related configuration patterns in rust-gds.
//!
//! Shows ModelConfig, PregelConfig, and MorphConfig usage.

use rust_gds::config::*;
use rust_gds::core::utils::partition::Partitioning;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ML Configuration Showcase ===\n");

    // 1. ModelConfig - for ML model metadata
    println!("1. ModelConfig:");
    let model = ModelConfig::builder()
        .model_name("pagerank_model".to_string())
        .model_user("alice".to_string())
        .build()?;
    println!("  Model: {}", model.model_name);
    println!("  User: {}\n", model.username());

    // With username override
    let model_override = ModelConfig::builder()
        .model_name("community_detection".to_string())
        .model_user("bob".to_string())
        .username_override(Some("admin".to_string()))
        .build()?;
    println!("  Model: {}", model_override.model_name);
    println!("  Effective user: {}\n", model_override.username());

    // 2. PregelConfig - for Pregel computation framework
    println!("2. PregelConfig:");
    let pregel = PregelConfig::builder()
        .max_iterations(50)
        .tolerance(Some(0.0001))
        .is_asynchronous(true)
        .partitioning(Partitioning::Degree)
        .build()?;
    println!("  Max iterations: {}", pregel.max_iterations());
    println!("  Tolerance: {:?}", pregel.tolerance());
    println!("  Async: {}", pregel.is_asynchronous());
    println!("  Partitioning: {:?}\n", pregel.partitioning());

    // 3. MorphConfig - nested container demonstration
    println!("3. MorphConfig (nested containers):");
    let morph = MorphConfig::builder()
        .with_shape(|builder| {
            builder
                .width(100)
                .height(50)
                .label("neural_net".to_string())
        })?
        .with_context(|builder| {
            builder
                .locale("en-US".to_string())
                .timezone("UTC".to_string())
        })?
        .morph_key("embedding_layer".to_string())
        .build()?;
    println!("  Shape: {}x{}", morph.shape.width, morph.shape.height);
    println!("  Label: {}", morph.shape.label);
    println!(
        "  Context: {} / {}",
        morph.context.locale, morph.context.timezone
    );
    println!("  Morph key: {}\n", morph.morph_key);

    // Access via convenience method
    let (shape, context) = morph.morph_tuple();
    println!(
        "  Via tuple accessor: {}x{} in {}",
        shape.width, shape.height, context.locale
    );

    println!("\n✓ All ML configs constructed and validated successfully");
    Ok(())
}

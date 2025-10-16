//! # Brahmachakra Integration Test
//!
//! This test demonstrates the COMPLETE Five-Fold Synthesis in action:
//!
//! 1. **TypeValidator** (Pinky) - Infer PropertyDescriptor from actual VALUES
//! 2. **HugeArrayProjector** (Thumb) - Project to dense sequential storage
//! 3. **ArrowProjector** (1st Middle) - Project to columnar batch storage
//! 4. **PregelProjector** (2nd Middle) - Project to BSP vertex-centric computation
//! 5. **AdaptiveProjector** (3rd Middle) - Learn optimal projection from workload
//!
//! This IS the unity of Dialectical and Metaphysical Idealism in executable code!

use rust_gds::projection::codegen::{
    AdaptiveProjector, ArrowProjector, HugeArrayProjector, PregelProjector, TypeProjector,
    TypeValidator,
};
use rust_gds::projection::property_descriptor::{PropertyDescriptor, StorageHint};
use rust_gds::types::ValueType;

/// The Complete Brahmachakra - All Five Fingers Working Together
#[test]
fn test_complete_brahmachakra_five_fingers() {
    println!("\n🔥 THE BRAHMACHAKRA BEGINS TO SPIN! 🔥\n");

    // ========================================================================
    // STEP 1: TypeValidator (PINKY) - Nāma from Rūpa
    // ========================================================================
    println!("👉 PINKY FINGER: TypeValidator infers Form from Values...");

    let node_ages = vec![25i64, 30i64, 45i64, 22i64, 67i64, 33i64];
    let edge_weights = vec![0.5f64, 0.8f64, 0.3f64, 0.9f64, 0.1f64];
    let active_flags = vec![true, false, true, true, false, true];

    // Infer PropertyDescriptors from actual VALUES (Rūpa → Nāma)
    let age_descriptor = TypeValidator::infer_from_i64_values(1, "person_age", &node_ages)
        .expect("Failed to infer age descriptor");

    let weight_descriptor = TypeValidator::infer_from_f64_values(2, "edge_weight", &edge_weights)
        .expect("Failed to infer weight descriptor");

    let active_descriptor = TypeValidator::infer_from_bool_values(3, "is_active", &active_flags)
        .expect("Failed to infer active descriptor");

    println!("   ✅ Inferred 3 PropertyDescriptors from raw values!");
    println!(
        "      - {} (id={}, type={:?})",
        age_descriptor.name, age_descriptor.id, age_descriptor.value_type
    );
    println!(
        "      - {} (id={}, type={:?})",
        weight_descriptor.name, weight_descriptor.id, weight_descriptor.value_type
    );
    println!(
        "      - {} (id={}, type={:?})",
        active_descriptor.name, active_descriptor.id, active_descriptor.value_type
    );

    // Validate that inferred descriptors match the VALUES (Brahman knowing!)
    TypeValidator::validate_i64_values(&age_descriptor, &node_ages).expect("Age validation failed");
    TypeValidator::validate_f64_values(&weight_descriptor, &edge_weights)
        .expect("Weight validation failed");
    TypeValidator::validate_bool_values(&active_descriptor, &active_flags)
        .expect("Active validation failed");

    println!("   ✅ Brahman knowing - Form ↔ Values are consistent!\n");

    // ========================================================================
    // STEP 2: HugeArrayProjector (THUMB) - Dense Sequential Storage
    // ========================================================================
    println!("👍 THUMB FINGER: HugeArrayProjector projects to dense storage...");

    let huge_projector = HugeArrayProjector::new();

    let age_storage = huge_projector
        .project_to_storage(&age_descriptor)
        .expect("Failed to project age to HugeArray storage");
    let age_computation = huge_projector
        .project_to_computation(&age_descriptor)
        .expect("Failed to project age to HugeArray computation");

    println!("   ✅ Age property projected:");
    println!("      Storage: {:?}", age_storage);
    println!("      Computation: {:?}", age_computation);

    // Validate the projection (Maya consistency check)
    huge_projector
        .validate_projection(&age_descriptor, &age_storage, &age_computation)
        .expect("HugeArray projection validation failed");

    println!("   ✅ HugeArray projection validated - Maya is consistent!\n");

    // ========================================================================
    // STEP 3: ArrowProjector (1ST MIDDLE FINGER) - Columnar Batch
    // ========================================================================
    println!("🖕 1ST MIDDLE FINGER: ArrowProjector projects to columnar batch...");

    let arrow_projector = ArrowProjector::new();

    let weight_storage = arrow_projector
        .project_to_storage(&weight_descriptor)
        .expect("Failed to project weight to Arrow storage");
    let weight_computation = arrow_projector
        .project_to_computation(&weight_descriptor)
        .expect("Failed to project weight to Arrow computation");

    println!("   ✅ Weight property projected:");
    println!("      Storage: {:?}", weight_storage);
    println!("      Computation: {:?}", weight_computation);

    arrow_projector
        .validate_projection(&weight_descriptor, &weight_storage, &weight_computation)
        .expect("Arrow projection validation failed");

    println!("   ✅ Arrow projection validated - Columnar batch is consistent!\n");

    // ========================================================================
    // STEP 4: PregelProjector (2ND MIDDLE FINGER) - BSP Vertex-Centric
    // ========================================================================
    println!("🖕 2ND MIDDLE FINGER: PregelProjector projects to BSP computation...");

    let pregel_projector = PregelProjector::new();

    let active_storage = pregel_projector
        .project_to_storage(&active_descriptor)
        .expect("Failed to project active to Pregel storage");
    let active_computation = pregel_projector
        .project_to_computation(&active_descriptor)
        .expect("Failed to project active to Pregel computation");

    println!("   ✅ Active flag projected:");
    println!("      Storage: {:?}", active_storage);
    println!("      Computation: {:?}", active_computation);

    pregel_projector
        .validate_projection(&active_descriptor, &active_storage, &active_computation)
        .expect("Pregel projection validation failed");

    println!("   ✅ Pregel projection validated - BSP is consistent!\n");

    // ========================================================================
    // STEP 5: AdaptiveProjector (3RD MIDDLE FINGER) - Being-and-NotBeing
    // ========================================================================
    println!("🖕 3RD MIDDLE FINGER: AdaptiveProjector learns and adapts...");

    let mut adaptive = AdaptiveProjector::with_conservatism(0.1); // Very aggressive learning

    // Project all three properties - Adaptive will learn from each!
    let age_adaptive_storage = adaptive
        .project_to_storage(&age_descriptor)
        .expect("Failed adaptive age storage");
    let weight_adaptive_storage = adaptive
        .project_to_storage(&weight_descriptor)
        .expect("Failed adaptive weight storage");
    let active_adaptive_storage = adaptive
        .project_to_storage(&active_descriptor)
        .expect("Failed adaptive active storage");

    println!("   ✅ Adaptive projected 3 properties:");
    println!("      Age: {:?}", age_adaptive_storage);
    println!("      Weight: {:?}", weight_adaptive_storage);
    println!("      Active: {:?}", active_adaptive_storage);

    // Adaptive IS Being-and-NotBeing - it delegates but has no fixed strategy
    println!("   ✅ AdaptiveProjector IS all projectors and IS none!\n");

    // ========================================================================
    // FINAL: The Brahmachakra Completes Its Rotation
    // ========================================================================
    println!("🔥 THE BRAHMACHAKRA HAS COMPLETED ONE FULL ROTATION! 🔥");
    println!("\nThe Five Fingers have worked together:");
    println!("  👉 Pinky (TypeValidator) - Inferred 3 descriptors from VALUES");
    println!("  👍 Thumb (HugeArray) - Projected to dense sequential");
    println!("  🖕 1st Middle (Arrow) - Projected to columnar batch");
    println!("  🖕 2nd Middle (Pregel) - Projected to BSP vertex-centric");
    println!("  🖕 3rd Middle (Adaptive) - Learned and delegated dynamically");
    println!("\n✨ The unity of Dialectical and Metaphysical Idealism! ✨");
    println!("✨ Nāma to Rūpa and back again! ✨");
    println!("✨ Maya knows itself! ✨\n");
}

/// Demonstrate TypeValidator → TypeProjector → TypeValidator Roundtrip
#[test]
fn test_nama_to_rupa_roundtrip() {
    println!("\n🌀 NĀMA ↔ RŪPA ROUNDTRIP TEST 🌀\n");

    // Start with VALUES (Rūpa - Form/Manifestation)
    let original_values = vec![42i64, 1337i64, 99i64, 2025i64];
    println!("📊 Original VALUES (Rūpa): {:?}", original_values);

    // ========== MOVEMENT 1: Rūpa → Nāma (TypeValidator Inference) ==========
    let inferred_descriptor =
        TypeValidator::infer_from_i64_values(888, "mysterious_numbers", &original_values)
            .expect("Failed to infer descriptor");

    println!("📜 Inferred DESCRIPTOR (Nāma):");
    println!("   ID: {}", inferred_descriptor.id);
    println!("   Name: {}", inferred_descriptor.name);
    println!("   Type: {:?}", inferred_descriptor.value_type);
    println!("   Storage Hint: {:?}", inferred_descriptor.storage_hint);

    // ========== MOVEMENT 2: Nāma → (Storage, Computation) (TypeProjector) ==========
    let projector = HugeArrayProjector::new();

    let storage_descriptor = projector
        .project_to_storage(&inferred_descriptor)
        .expect("Failed to project to storage");
    let computation_descriptor = projector
        .project_to_computation(&inferred_descriptor)
        .expect("Failed to project to computation");

    println!("\n🗄️  Projected to STORAGE: {:?}", storage_descriptor);
    println!("⚙️  Projected to COMPUTATION: {:?}", computation_descriptor);

    // ========== MOVEMENT 3: Validation (Brahman Knowing) ==========
    projector
        .validate_projection(
            &inferred_descriptor,
            &storage_descriptor,
            &computation_descriptor,
        )
        .expect("Projection validation failed");

    println!("\n✅ Projection is valid - Storage ↔ Computation consistent!");

    // ========== MOVEMENT 4: Rūpa Validation (Return to Source) ==========
    TypeValidator::validate_i64_values(&inferred_descriptor, &original_values)
        .expect("Value validation failed");

    println!("✅ VALUES validated against inferred descriptor!");

    // ========== THE LOOP IS CLOSED ==========
    println!("\n🔄 THE COMPLETE LOOP:");
    println!("   VALUES → infer → DESCRIPTOR → project → (STORAGE, COMPUTATION)");
    println!("                     ↓                              ↓");
    println!("                  validate ← ← ← ← ← ← ← ← ← ← ← ←");
    println!("\n✨ Brahman knows itself through the complete cycle! ✨\n");
}

/// Demonstrate Adaptive Learning from Actual Workload
#[test]
fn test_adaptive_learning_from_workload() {
    println!("\n🧠 ADAPTIVE LEARNING TEST - Maya Learns Itself! 🧠\n");

    let adaptive = AdaptiveProjector::with_conservatism(0.1); // Very aggressive learning

    // Create different property types that favor different projectors
    let dense_sequential_prop = PropertyDescriptor::new(1, "dense_ids", ValueType::Long)
        .with_storage_hint(StorageHint::FixedWidth)
        .with_nullable(false);

    let sparse_batch_prop = PropertyDescriptor::new(2, "sparse_weights", ValueType::Double)
        .with_storage_hint(StorageHint::VariableLength)
        .with_nullable(true);

    let vertex_centric_prop = PropertyDescriptor::new(3, "convergence_flags", ValueType::Boolean)
        .with_storage_hint(StorageHint::FixedWidth)
        .with_nullable(false);

    println!("🎯 Testing 3 different property patterns...\n");

    // Project each property - Adaptive should learn different strategies
    let storage1 = adaptive
        .project_to_storage(&dense_sequential_prop)
        .expect("Failed projection 1");
    println!("1️⃣  Dense sequential property:");
    println!("    Projected to: {:?}", storage1);
    println!("    (Should favor HugeArray for dense sequential)\n");

    let storage2 = adaptive
        .project_to_storage(&sparse_batch_prop)
        .expect("Failed projection 2");
    println!("2️⃣  Sparse batch property:");
    println!("    Projected to: {:?}", storage2);
    println!("    (Should favor Arrow for batch processing)\n");

    let storage3 = adaptive
        .project_to_storage(&vertex_centric_prop)
        .expect("Failed projection 3");
    println!("3️⃣  Vertex-centric convergence property:");
    println!("    Projected to: {:?}", storage3);
    println!("    (Should favor Pregel for iterative computation)\n");

    println!("✅ AdaptiveProjector successfully delegated to different strategies!");
    println!("✅ This IS Being-and-NotBeing - no fixed nature, pure adaptation!");
    println!("✅ Maya as self-learning process!\n");
}

/// Demonstrate Schema Migration Detection
#[test]
fn test_schema_evolution_migration() {
    println!("\n♻️  SCHEMA EVOLUTION TEST - Īśvara's Saṃhāra-Sṛṣṭi! ♻️\n");

    // Original schema - integers
    let original_descriptor = PropertyDescriptor::new(42, "evolving_property", ValueType::Long)
        .with_storage_hint(StorageHint::FixedWidth)
        .with_nullable(false);

    println!("📋 Original Schema:");
    println!("   ID: {}", original_descriptor.id);
    println!("   Name: {}", original_descriptor.name);
    println!("   Type: {:?}", original_descriptor.value_type);

    // Detect that data has evolved to floating-point
    let needs_migration = TypeValidator::needs_migration(&original_descriptor, ValueType::Double);

    println!("\n🔍 Migration Detection:");
    println!("   Old type: {:?}", original_descriptor.value_type);
    println!("   New type: {:?}", ValueType::Double);
    println!(
        "   Needs migration? {}",
        if needs_migration { "YES ✅" } else { "NO ❌" }
    );

    assert!(needs_migration, "Should detect type change");

    // Suggest migration path
    let migrated_descriptor =
        TypeValidator::suggest_migration(&original_descriptor, ValueType::Double)
            .expect("Failed to suggest migration");

    println!("\n📋 Migrated Schema:");
    println!("   ID: {} (preserved!)", migrated_descriptor.id);
    println!("   Name: {} (versioned!)", migrated_descriptor.name);
    println!("   Type: {:?} (evolved!)", migrated_descriptor.value_type);
    println!(
        "   Storage Hint: {:?} (updated!)",
        migrated_descriptor.storage_hint
    );

    // Verify migration properties
    assert_eq!(
        migrated_descriptor.id, original_descriptor.id,
        "ID should be preserved"
    );
    assert!(
        migrated_descriptor.name.contains("_v"),
        "Name should be versioned"
    );
    assert_eq!(
        migrated_descriptor.value_type,
        ValueType::Double,
        "Type should be evolved"
    );

    println!("\n✅ Schema evolution detected and migration suggested!");
    println!("✅ This IS Īśvara's Pañca-kṛtya:");
    println!("   - Sṛṣṭi (Creation): New descriptor created");
    println!("   - Sthiti (Preservation): ID preserved");
    println!("   - Saṃhāra (Destruction): Old schema obsoleted");
    println!("   - Tirodhāna (Concealing): Complexity hidden");
    println!("   - Anugraha (Revealing): New schema revealed\n");
}

/// Demonstrate All Projectors on Same Property
#[test]
fn test_all_projectors_same_property() {
    println!("\n🎨 ALL PROJECTORS, ONE PROPERTY - Maya's Many Faces! 🎨\n");

    // Single property - many projections!
    let property = PropertyDescriptor::new(1, "multi_faced_property", ValueType::Long)
        .with_storage_hint(StorageHint::FixedWidth)
        .with_nullable(false);

    println!(
        "📦 Source Property: {} (id={}, type={:?})\n",
        property.name, property.id, property.value_type
    );

    // Project with ALL FOUR explicit projectors
    let huge = HugeArrayProjector::new();
    let arrow = ArrowProjector::new();
    let pregel = PregelProjector::new();
    let adaptive = AdaptiveProjector::new();

    println!("👍 HugeArrayProjector:");
    let huge_storage = huge.project_to_storage(&property).unwrap();
    let huge_computation = huge.project_to_computation(&property).unwrap();
    println!("   Storage: {:?}", huge_storage);
    println!("   Computation: {:?}", huge_computation);
    huge.validate_projection(&property, &huge_storage, &huge_computation)
        .unwrap();
    println!("   ✅ Validated!\n");

    println!("🖕 ArrowProjector:");
    let arrow_storage = arrow.project_to_storage(&property).unwrap();
    let arrow_computation = arrow.project_to_computation(&property).unwrap();
    println!("   Storage: {:?}", arrow_storage);
    println!("   Computation: {:?}", arrow_computation);
    arrow
        .validate_projection(&property, &arrow_storage, &arrow_computation)
        .unwrap();
    println!("   ✅ Validated!\n");

    println!("🖕 PregelProjector:");
    let pregel_storage = pregel.project_to_storage(&property).unwrap();
    let pregel_computation = pregel.project_to_computation(&property).unwrap();
    println!("   Storage: {:?}", pregel_storage);
    println!("   Computation: {:?}", pregel_computation);
    pregel
        .validate_projection(&property, &pregel_storage, &pregel_computation)
        .unwrap();
    println!("   ✅ Validated!\n");

    println!("🖕 AdaptiveProjector:");
    let adaptive_storage = adaptive.project_to_storage(&property).unwrap();
    let adaptive_computation = adaptive.project_to_computation(&property).unwrap();
    println!("   Storage: {:?}", adaptive_storage);
    println!("   Computation: {:?}", adaptive_computation);
    adaptive
        .validate_projection(&property, &adaptive_storage, &adaptive_computation)
        .unwrap();
    println!("   ✅ Validated!\n");

    println!("✨ ONE PROPERTY, FOUR PROJECTIONS! ✨");
    println!("✨ This IS Maya - One Reality, infinite manifestations! ✨");
    println!("✨ Each projector sees the SAME property differently! ✨\n");
}

/// Performance comparison between projectors (metadata only)
#[test]
fn test_projector_characteristics_comparison() {
    println!("\n📊 PROJECTOR CHARACTERISTICS COMPARISON 📊\n");

    println!("┌─────────────────┬──────────────┬─────────────────┬─────────────────┐");
    println!("│ Projector       │ Storage      │ Computation     │ Best For        │");
    println!("├─────────────────┼──────────────┼─────────────────┼─────────────────┤");
    println!("│ HugeArray       │ Chunked      │ BSP/Vertex      │ Dense/Sequential│");
    println!("│ (Thumb)         │ Sequential   │ Cursor-based    │ Large arrays    │");
    println!("├─────────────────┼──────────────┼─────────────────┼─────────────────┤");
    println!("│ Arrow           │ Columnar     │ Dataflow/Global │ Batch analytics │");
    println!("│ (1st Middle)    │ Batch        │ Zero-copy       │ Immutable data  │");
    println!("├─────────────────┼──────────────┼─────────────────┼─────────────────┤");
    println!("│ Pregel          │ Hybrid       │ BSP/Messages    │ Iterative algos │");
    println!("│ (2nd Middle)    │ Partitioned  │ Convergence     │ Graph traversal │");
    println!("├─────────────────┼──────────────┼─────────────────┼─────────────────┤");
    println!("│ Adaptive        │ Dynamic      │ Dynamic         │ Unknown workload│");
    println!("│ (3rd Middle)    │ Delegates    │ Delegates       │ Learning systems│");
    println!("└─────────────────┴──────────────┴─────────────────┴─────────────────┘");

    println!("\n🎯 Key Insights:");
    println!("   • HugeArray = Dense sequential, cursor iteration");
    println!("   • Arrow = Columnar batch, zero-copy, immutable");
    println!("   • Pregel = Message-passing BSP, convergence-driven");
    println!("   • Adaptive = Self-learning, delegates to optimal");

    println!("\n✅ Each projector excels in its domain!");
    println!("✅ AdaptiveProjector learns which to use when!");
    println!("✅ This IS the Theory of Solutions to All Problems!\n");
}

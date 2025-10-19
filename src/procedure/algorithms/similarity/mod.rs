//! Similarity algorithms - Faithful 1:1 translation from Java GDS
//!
//! This module contains faithful translations of Java GDS similarity support classes:
//! - `SimilaritySummaryBuilder.java` → `similarity_summary_builder.rs`
//! - `ActualSimilaritySummaryBuilder.java` → `actual_similarity_summary_builder.rs`
//! - `EmptySimilaritySummaryBuilder.java` → `empty_similarity_summary_builder.rs`
//! - `WriteRelationshipResult.java` → `write_relationship_result.rs`
//! - `MutateRelationshipService.java` → `mutate_relationship_service.rs`
//! - `WriteRelationshipService.java` → `write_relationship_service.rs`
//! - `Neo4jDatabaseRelationshipWriter.java` → `neo4j_database_relationship_writer.rs`
//! - `SimilaritySingleTypeRelationshipsHandler.java` → `similarity_single_type_relationships_handler.rs`

pub mod similarity_summary_builder;
pub mod actual_similarity_summary_builder;
pub mod empty_similarity_summary_builder;
pub mod write_relationship_result;
pub mod mutate_relationship_service;
pub mod write_relationship_service;
pub mod neo4j_database_relationship_writer;
pub mod similarity_single_type_relationships_handler;

// Re-export the translated types
pub use similarity_summary_builder::*;
pub use actual_similarity_summary_builder::*;
pub use empty_similarity_summary_builder::*;
pub use write_relationship_result::*;
pub use mutate_relationship_service::*;
pub use write_relationship_service::*;
pub use neo4j_database_relationship_writer::*;
pub use similarity_single_type_relationships_handler::*;
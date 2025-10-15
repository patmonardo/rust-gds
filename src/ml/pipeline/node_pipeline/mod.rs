// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Node property prediction pipeline infrastructure.
//!
//! This module contains the node-specific pipeline types for:
//! - Node classification (predicting categorical node properties)
//! - Node regression (predicting continuous node properties)

// Phase 1.1: Simple Types
pub mod node_feature_step;
pub mod node_property_pipeline_base_train_config;

// Re-exports for convenience
pub use node_feature_step::NodeFeatureStep;
pub use node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;

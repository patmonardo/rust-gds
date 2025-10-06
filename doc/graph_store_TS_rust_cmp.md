# TS → Rust Property API Mapping (auto-generated sketch)

Notes:

- Left = TypeScript source (ts-gds/api/properties)
- Right = canonical Rust module (src/types/properties)
- Java column lists likely class names (pattern-based); verify against Java sources.

| TS (source)                                      | Rust (target)                                                                  | Java (likely)                                                                 | Notes                           |
| ------------------------------------------------ | ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- | ------------------------------- |
| PropertyStore.ts                                 | src/types/properties/property_store.rs                                         | org.neo4j.gds.properties.PropertyStore                                        | top-level generic store helpers |
| Property.ts                                      | src/types/properties/property.rs                                               | org.neo4j.gds.properties.Property                                             | header + values                 |
| PropertyValues.ts                                | src/types/properties/property_values.rs                                        | org.neo4j.gds.properties.PropertyValues                                       | trait / enum of types           |
| graph/index.ts                                   | src/types/properties/graph/mod.rs                                              | org.neo4j.gds.properties.graph.\*                                             | graph domain exports            |
| graph/GraphProperty.ts                           | src/types/properties/graph/graph_property.rs                                   | org.neo4j.gds.properties.graph.GraphProperty                                  | header for graph properties     |
| graph/GraphPropertyStore.ts                      | src/types/properties/graph/graph_property_store.rs                             | org.neo4j.gds.properties.graph.GraphPropertyStore                             | store trait                     |
| graph/GraphPropertyStoreBuilder.ts               | src/types/properties/graph/impls/default_graph_property_store.rs               | org.neo4j.gds.properties.graph.DefaultGraphPropertyStoreBuilder               | builder impl                    |
| graph/abstract/\* (Long/Double/Array)            | src/types/properties/graph/impls/values/\*.rs                                  | org.neo4j.gds.properties.graph.\*GraphPropertyValues                          | abstract trait variants         |
| graph/primitive/DefaultGraphProperty.ts          | src/types/properties/graph/impls/default_graph_property.rs                     | org.neo4j.gds.properties.graph.DefaultGraphProperty                           | concrete graph property         |
| nodes/index.ts                                   | src/types/properties/node/mod.rs                                               | org.neo4j.gds.properties.node.\*                                              | node domain exports             |
| nodes/NodeProperty.ts                            | src/types/properties/node/node_property.rs                                     | org.neo4j.gds.properties.node.NodeProperty                                    | node header                     |
| nodes/NodePropertyStore.ts                       | src/types/properties/node/node_property_store.rs                               | org.neo4j.gds.properties.node.NodePropertyStore                               | store trait                     |
| nodes/NodePropertyStoreBuilder.ts                | src/types/properties/node/impls/default_node_property_store.rs                 | org.neo4j.gds.properties.node.DefaultNodePropertyStoreBuilder                 | builder impl                    |
| nodes/NodePropertyValues.ts                      | src/types/properties/node/node_property_values.rs                              | org.neo4j.gds.properties.node.NodePropertyValues                              | trait for node values           |
| nodes/abstract/\*                                | src/types/properties/node/impls/values/\*.rs                                   | org.neo4j.gds.properties.node.\*NodePropertyValues                            | abstract value types            |
| nodes/primitive/DefaultLongNodePropertyValues.ts | src/types/properties/node/impls/values/long.rs                                 | org.neo4j.gds.properties.node.DefaultLongNodePropertyValues                   | concrete impl                   |
| relationships/index.ts                           | src/types/properties/relationship/mod.rs                                       | org.neo4j.gds.properties.relationship.\*                                      | rel domain exports              |
| relationships/RelationshipProperty.ts            | src/types/properties/relationship/relationship_property.rs                     | org.neo4j.gds.properties.relationship.RelationshipProperty                    | rel header                      |
| relationships/RelationshipPropertyStore.ts       | src/types/properties/relationship/relationship_property_store.rs               | org.neo4j.gds.properties.relationship.RelationshipPropertyStore               | store trait                     |
| relationships/RelationshipPropertyBuilder.ts     | src/types/properties/relationship/impls/default_relationship_property_store.rs | org.neo4j.gds.properties.relationship.DefaultRelationshipPropertyStoreBuilder | builder impl                    |
| relationships/primitive/\*                       | src/types/properties/relationship/impls/\*.rs                                  | org.neo4j.gds.properties.relationship.\*                                      | concrete rel impls              |

## Short checklist to verify mapping

- Ensure each Rust file exports the same trait/type names used by TS.
- Confirm Arc vs Box ownership choices in Rust match expected semantics in TS/JAVA.
- For each TS file that contains tests, locate corresponding Rust unit tests under src/types/.../test.

## Quick helper: generate a heuristic map locally

Run this in the repo root to list TS -> candidate Rust paths:

```bash
rg --files ts-gds/api/properties | sed 's|ts-gds/api/properties|src/types/properties|' | sed 's|\.ts$|.rs|'
```

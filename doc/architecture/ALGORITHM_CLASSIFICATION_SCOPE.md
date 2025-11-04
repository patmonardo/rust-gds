# Algorithm Classification by Scope: Local, Relational, Collective

**Date**: 2025-01-27  
**Classification System**: Three types reflecting Node/Edge/Graph levels

## The Three Classification Types

```
┌──────────────┬───────────────┬──────────────┐
│   Local      │   Relational  │   Collective │
├──────────────┼───────────────┼──────────────┤
│   Node-level │   Edge-level  │  Graph-level │
│   (Nodal)    │   (Pairs)     │  (Global)    │
└──────────────┴───────────────┴──────────────┘
```

### 1. Local (Node-level, Nodal)

**Scope**: Single node and its immediate neighborhood

**Characteristics**:
- Operates on individual nodes
- Uses local neighborhood information
- Node-centric computation
- Examples: Degree, Local Clustering Coefficient, Node centrality scores

**Computation Pattern**:
- Input: Single node + neighborhood
- Output: Value per node
- Scope: Limited to 1-hop or k-hop neighbors

**Examples**:
- **DegreeCentrality**: Degree of each node (node-level property)
- **LocalClusteringCoefficient**: Clustering within node's neighborhood
- **Node embeddings**: Per-node feature vectors
- **Node property prediction**: Predicting node labels/values

### 2. Relational (Edge-level, Pairs)

**Scope**: Relationships between pairs of nodes

**Characteristics**:
- Operates on node pairs (edges/relationships)
- Considers relationships between entities
- Pair-based computation
- Examples: Link prediction, edge similarity, relationship scoring

**Computation Pattern**:
- Input: Node pairs (source, target)
- Output: Value per relationship/pair
- Scope: Between pairs of nodes

**Examples**:
- **Link Prediction**: Predict if relationship exists between two nodes
- **NodeSimilarity**: Similarity between pairs of nodes
- **JaccardSimilarity**: Similarity of neighbor sets (pair-based)
- **Edge weight computation**: Weighting relationships
- **Relationship property prediction**: Predicting relationship types

### 3. Collective (Graph-level, Global)

**Scope**: Entire graph or large subgraphs

**Characteristics**:
- Operates on the whole graph
- Global, graph-wide computation
- Collective/aggregate properties
- Examples: Graph metrics, community structure, global centrality

**Computation Pattern**:
- Input: Entire graph structure
- Output: Graph-level value or per-node values requiring global context
- Scope: All nodes and edges

**Examples**:
- **PageRank**: Global centrality requiring entire graph
- **Community Detection** (Louvain, Leiden): Graph-wide community structure
- **Connected Components** (WCC, SCC): Global connectivity analysis
- **Graph embedding**: Graph-level representation
- **Graph density**: Graph-wide statistics
- **BetweennessCentrality**: Requires all-pairs shortest paths (global context)

---

## Mapping to Graph Structure Levels

### Triadic Graph Structure

The three classification types map perfectly to the **triadic graph store architecture**:

```
Graph Structure Levels:
├── Node Level (Local)      → Nodal properties, node features
├── Relationship Level (Relational) → Edge properties, link features  
└── Graph Level (Collective) → Global metadata, graph features
```

### Property Store Alignment

- **Local (Node-level)**: `NodePropertyStore` - Properties attached to nodes
- **Relational (Edge-level)**: `RelationshipPropertyStore` - Properties attached to relationships
- **Collective (Graph-level)**: `GraphPropertyStore` - Graph-wide metadata

### Type-Level Constraint: Properties Bound to Their Stores

**Key Architectural Insight**: `NodeProperty` can only link to a `NodePropertyStore`.

**How the Constraint Works**:

The Rust type system enforces this at compile time:

```rust
// Each store trait has a specific Property type
pub trait NodePropertyStore: PropertyStore {
    type Property = NodeProperty;  // Implicit constraint
    
    fn get_property_values(&self, key: &str) 
        -> Option<&dyn NodePropertyValues>;  // Returns NodePropertyValues, not GraphPropertyValues
    
    // Builder methods take NodeProperty, not GraphProperty
    fn put(self, key: &str, property: Self::Property) -> Self;
}

pub trait GraphPropertyStore: PropertyStore {
    type Property = GraphProperty;  // Different type!
    
    fn get_property_values(&self, key: &str) 
        -> Option<&dyn GraphPropertyValues>;  // Returns GraphPropertyValues, not NodePropertyValues
    
    // Builder methods take GraphProperty, not NodeProperty
    fn put(self, key: &str, property: Self::Property) -> Self;
}
```

**Why This Is Interesting**:

1. **Compile-Time Safety**: You literally cannot put a `NodeProperty` into a `GraphPropertyStore` - the compiler prevents it
2. **Scope Enforced by Types**: The Local/Relational/Collective classification is enforced at the type level
3. **No Runtime Checks Needed**: The architecture prevents mixing scopes before code runs
4. **Perfect Alignment**: Type constraints enforce the triadic structure (Node/Relationship/Graph)

**The Constraint Chain**:
```
NodeProperty 
  ↓ (contains)
Arc<dyn NodePropertyValues>
  ↓ (must match)
NodePropertyStore::put(NodeProperty)
  ↓ (returns)
Option<&dyn NodePropertyValues>
```

Each level enforces the scope constraint - you cannot mix Node-level properties with Graph-level stores, or vice versa.

**Architectural Principle**:
- **Properties match their store level** by type design
- **Type safety = scope safety** (Local/Relational/Collective)
- **The triadic structure is enforced by the type system**, not by convention

### Different Exponents: The Metaphysics of Scope

**Key Insight**: Local/Relational/Collective correspond to **different exponents** (Potenz levels) in the HyperSchema system.

**The Exponent Connection**:

In the HyperSchema metaphysics:
- **The Exponent determines the level of determinateness** (Potenz in Schelling)
- **Each exponent is a Named Quality**, not just a quantity
- **X^n is not a quantity but a Named Quality** (the polynomial XXX...)

**Mapping to Classification**:

```
Local (Node-level)      → Monadic (n=1) → Potenz of Simple Unity (Being)
Relational (Edge-level) → Dyadic  (n=2) → Potenz of Relationship (ground ↔ condition)
Collective (Graph-level) → Triadic (n=3) → Potenz of Conceiving (X AS Y - Objectivity)
```

**What This Means**:

1. **Different Exponents = Different Levels of Determinateness**:
   - **Local (n=1)**: Simple unity - a single node with its being
   - **Relational (n=2)**: Binary relationship - ground ↔ condition
   - **Collective (n=3)**: Determinate structure - conceiving X AS Y (graph as object)

2. **Each Classification is a Named Quality (Potenz)**:
   - Not just "more nodes" or "bigger scope"
   - Each is a **qualitatively different level of determinateness**
   - Local isn't "smaller" Collective - it's a **different kind of being**

3. **The Exponent Determines What the Schema IS**:
   - Local algorithms operate at **Monadic Being** (simple unity)
   - Relational algorithms operate at **Dyadic Relationship** (ground ↔ condition)
   - Collective algorithms operate at **Triadic Conceiving** (X AS Y - graph structure)

**HyperSchema Notation**:
```
S₁ = Monadic (n=1) → Local (Node-level)
S₂ = Dyadic  (n=2) → Relational (Edge-level)
S₃ = Triadic (n=3) → Collective (Graph-level)
```

**Connection to Schelling's Naturphilosophie**:
- Each Potenz is a **self-positing stage**
- Not applied mathematics, but **Nature's own self-organization**
- The exponent determines the **Named Quality**, not the quantity

**For Graph Data Science**:
- **Local algorithms** work at the **Monadic level** - simple unity of individual nodes
- **Relational algorithms** work at the **Dyadic level** - relationships between pairs
- **Collective algorithms** work at the **Triadic level** - the graph as a determined structure

**Different exponents make sense** because each classification type is not just a different scope, but a **fundamentally different level of determinateness** - a different Named Quality (Potenz) in the structure of graph computation.

### Nodal as Being: Quality-Quantity-Measure

**Key Insight**: **Nodal is the Moment of Being** which is Quality-Quantity-Measure.

> **"There it is Nodal is the Moment of Being which is Quality-Quantity-Measure."**

**Hegel's Doctrine of Being**:
- **Three moments**: Quality → Quantity → Measure
- **Being** = Immediate determinations (not yet reflected)
- **Nodal** = Determinations of Being (simple, immediate)

**The Three Moments**:
1. **Quality**: What something is (its nature, character)
2. **Quantity**: How much (extent, magnitude)
3. **Measure**: Unity of Quality and Quantity (Quality-Quantity synthesis)

**Nodal as Being**:
- **Local (Nodal)** algorithms operate at **Being** level
- **Determinations of Being**: Immediate, not yet reflected
- **Quality-Quantity-Measure**: The three moments of Being unified

**In Graph ML**:
- **Quality**: Node structure, schema, Named Quality
- **Quantity**: Node values, empirical data
- **Measure**: Node property (Quality-Quantity synthesis)

**Monadic (n=1) = Being**:
- Simple unity, immediate determinations
- Quality-Quantity-Measure as three moments
- Not yet reflected (not yet raised to Property)

### Relational as Property: Second Exponent

**Key Insight**: **Relational is Property** - where **Quality dialectically evolves into Property**.

> **"Relational is Property - that is key really, Quality dialectically evolves into Property."

**Property as Second Exponent**:
- **Property** = **Quality raised to a higher power** (n=2, Dyadic)
- **Second Exponent**: Quality^2 = Property
- **Dyadic (n=2)** = Property level

**The Dialectical Evolution**:
```
Quality (Being, n=1) 
  → Dialectically evolves
    → Property (Essence, n=2)
      → Quality raised to second power
        → Determinations of Reflection
```

**Relational as Property**:
- **Relational** algorithms operate at **Property** level
- **Determinations of Reflection**: Ground-condition of facticity
- **Property** = Ground-condition of the facticity of things

**Why Property is Second Exponent**:
- **Quality (n=1)**: Immediate determination (Being)
- **Property (n=2)**: Reflected determination (Essence)
- **Second exponent**: Quality^2 = Property
- **Dyadic (n=2)**: Property is Quality raised to second power

**Hegel's Doctrine of Essence**:
- **Essence** = Reflected determinations (mediated, not immediate)
- **Property** = Ground-condition of facticity
- **Determinations of Reflection**: Not immediate Being, but reflected Essence

**In Graph ML**:
- **Nodal (Being)**: Quality-Quantity-Measure (immediate determinations)
- **Relational (Property)**: Quality^2 = Property (reflected determinations)
- **Property**: Ground-condition for the facticity of relationships

### The Dialectical Movement

**From Being to Essence**:

1. **Being (Nodal, n=1)**:
   - Immediate determinations
   - Quality-Quantity-Measure
   - Not yet reflected
   - Determinations of Being

2. **Dialectical Evolution**:
   - Quality evolves
   - Becomes mediated
   - Raised to second power

3. **Essence (Relational, n=2)**:
   - Reflected determinations
   - Property (Quality^2)
   - Ground-condition of facticity
   - Determinations of Reflection

**The Key Movement**:
- **Quality (Being)** → Dialectically evolves → **Property (Essence)**
- **Property = Quality raised to second power** (n=2, Dyadic)
- **Second exponent**: Where Quality becomes Property

**For Graph ML**:
- **Nodal (Local)**: Determinations of Being (Quality-Quantity-Measure)
- **Relational**: Determinations of Reflection (Property = Quality^2)
- **Property**: Ground-condition for the facticity of relationships

### Property as Ground-Condition of Facticity

**Key Insight**: **Properties are the ground-condition of the facticity of things**.

**Facticity**:
- The "that it is" - the actual existence
- Grounded in properties (not immediate being)
- Requires reflection (not immediate determinations)

**Property as Ground-Condition**:
- **Properties** provide the ground for facticity
- **Relationships** exist because of properties (ground-condition)
- **Relational** algorithms operate at Property level

**Determinations of Reflection**:
- **Not Being**: Not immediate determinations
- **Essence**: Reflected determinations
- **Property**: Ground-condition for facticity
- **Relational**: Where properties ground relationships

**The Hierarchy**:
```
Being (Nodal, n=1)
  → Quality-Quantity-Measure (immediate)
  
Essence (Relational, n=2)
  → Property = Quality^2 (reflected)
  → Ground-condition of facticity
  → Determinations of Reflection
```

### Conclusion: The Dialectical Structure

**Nodal (Local, n=1) = Being**:
- **Moment of Being**: Quality-Quantity-Measure
- **Determinations of Being**: Immediate, not yet reflected
- **Monadic**: Simple unity, immediate determinations

**Relational (Edge-level, n=2) = Property**:
- **Property**: Quality raised to second power
- **Determinations of Reflection**: Ground-condition of facticity
- **Dyadic**: Reflected determinations, mediated

**The Evolution**:
- **Quality (Being)** → Dialectically evolves → **Property (Essence)**
- **Property = Quality^2** (Second exponent)
- **Relational = Property level** (n=2, Dyadic)

**For Graph ML**: **Nodal is the Moment of Being** (Quality-Quantity-Measure) - immediate determinations. **Relational is Property** - where **Quality dialectically evolves into Property**, raised to the **Second Exponent** (n=2, Dyadic). **Property is the ground-condition** of the facticity of things, making **Relational algorithms operate at the Property level** - determinations of Reflection, not immediate Being.

### Collective as Subjective Logic: Subject-Object

**Key Insight**: **Graph level combines the Quality-Property Relations of Objective Logic into Subjective Logic** where we have **Subject-Object** which are **Subject-Predicate**.

> **"and graph level combines the Quality-Property Relations of Objective Logic into Subjective Logic where we have Subject-Object ok which are Subject-Predicate"**

**Objective Logic** (Being + Essence):
- **Being (n=1)**: Quality-Quantity-Measure (Nodal)
- **Essence (n=2)**: Property, Ground-Condition (Relational)
- **Objective**: Determinations of things (not yet for consciousness)

**Subjective Logic** (Concept, n=3):
- **Concept**: The Idea, Subjectivity
- **Subject-Object**: Subject as self-relating, Object as determined
- **Subject-Predicate**: Subject determines itself through predicates
- **Collective (n=3)**: Triadic level (Subjective Logic)

**Graph Level as Subjective Logic**:
- **Collective algorithms**: Operate at Concept level (n=3, Triadic)
- **Combine**: Quality-Property Relations (from Objective Logic)
- **Into**: Subjective Logic (Subject-Object/Predicate)
- **Graph as Concept**: Self-determining structure

**Subject-Object Structure**:
- **Subject**: The self-relating structure (graph as whole)
- **Object**: The determined structure (nodes/edges as objects)
- **Subject-Predicate**: Graph determines itself through predicates (properties, relationships)

**Triadic (n=3) = Concept**:
- **Triadic**: Potenz of Conceiving (X AS Y - Objectivity becoming Subjective)
- **Concept**: The Idea, self-determining
- **Subjective Logic**: Where Being and Essence become Concept

**For Graph ML**:
- **Nodal (Being)**: Quality-Quantity-Measure (Objective Logic, n=1)
- **Relational (Essence)**: Property, Ground-Condition (Objective Logic, n=2)
- **Collective (Concept)**: Subject-Object, Subject-Predicate (Subjective Logic, n=3)

**The Complete Dialectical Progression**:
```
Being (n=1) → Quality-Quantity-Measure (Nodal)
  ↓
Essence (n=2) → Property, Ground-Condition (Relational)
  ↓
Concept (n=3) → Subject-Object, Subject-Predicate (Collective)
```

### Property Enters ML Pipelines: The Consequence Relation

**Key Insight**: **Property can enter into ML Pipelines, not Features** - because **Ground-Condition implies Grounded-Conditioned**, which is **the Consequence Relation**.

> **"so Property can enter into ML Pipelines, not Features. because Ground,Condition implies Grounded-Conditioned . that is the Consequence Relation"**

**Why Properties, Not Features**:

**Features (Being level, n=1)**:
- **Quality-Quantity dyad**: Immediate determinations
- **Being level**: Not yet reflected, not yet conditioned
- **No Ground-Condition**: Cannot enter ML pipelines directly
- **Must be grounded**: In Properties to enter pipelines

**Properties (Essence level, n=2)**:
- **Ground-Condition system**: Reflected determinations
- **Grounded-Conditioned**: Consequence relation structure
- **If Property, then Conditioned**: The implication ML needs
- **Can enter pipelines**: Properties have the structure ML requires

**The Consequence Relation**:
```
Ground → Conditioned
If (Ground) then (Conditioned)
X → Y (Implication)
```

**Property as Ground-Condition**:
- **Property**: Ground-condition of facticity
- **Grounded**: Property grounds the facticity
- **Conditioned**: Facticity is conditioned by Property
- **Consequence**: If Property, then Facticity (X → Y)

**ML Pipeline Structure**:
```
Node Property Steps → Properties (Ground-Condition)
  ↓
Feature Steps → Features (Grounded in Properties)
  ↓
Training → Uses Properties (Ground) → Predictions (Conditioned)
```

**Why This Works**:
- **Properties**: Ground-Condition structure (Essence level, n=2)
- **Consequence Relation**: If (Ground Property) then (Conditioned Prediction)
- **Features inherit**: Ground-Condition from Properties they're derived from
- **Training requires**: Ground-Condition → Consequence structure

**For Graph ML**: **Property can enter into ML Pipelines, not Features** - because **Ground-Condition implies Grounded-Conditioned** through **the Consequence Relation** (X → Y, if-then). Properties operate at Essence level (n=2) with Ground-Condition structure, enabling the consequence relation that ML training requires. Features (Being level, n=1, Quality-Quantity-Measure) lack this structure and must be grounded in Properties to enter pipelines - they inherit the Ground-Condition → Consequence Relation from their source Properties.

---

## Algorithm Examples by Classification

### Local Algorithms (Node-level)

| Algorithm | Type | Output |
|-----------|------|--------|
| DegreeCentrality | Local | Score per node |
| LocalClusteringCoefficient | Local | Coefficient per node |
| Node embeddings (Node2Vec, FastRP) | Local | Vector per node |
| Node property prediction | Local | Prediction per node |

**ML Context**:
- **Node Classification**: Predict node class (local)
- **Node Regression**: Predict node value (local)
- **Features**: Node-level feature vectors

### Relational Algorithms (Edge-level)

| Algorithm | Type | Output |
|-----------|------|--------|
| Link Prediction | Relational | Probability per edge |
| NodeSimilarity | Relational | Similarity per pair |
| JaccardSimilarity | Relational | Similarity per pair |
| Edge weight computation | Relational | Weight per relationship |

**ML Context**:
- **Link Prediction**: Predict relationship existence (relational)
- **Features**: Pair-based features (source node + target node embeddings)

### Collective Algorithms (Graph-level)

| Algorithm | Type | Output |
|-----------|------|--------|
| PageRank | Collective | Score per node (requires global graph) |
| BetweennessCentrality | Collective | Score per node (requires all pairs) |
| Community Detection (Louvain) | Collective | Community per node (global optimization) |
| Connected Components (WCC) | Collective | Component ID per node (global structure) |
| Graph embedding | Collective | Single vector for entire graph |

**ML Context**:
- **Graph Classification**: Predict graph label (collective)
- **Features**: Graph-level aggregated features

---

## Computational Complexity Patterns

### Local
- **Time Complexity**: Often O(degree) or O(k) for k-hop neighborhoods
- **Space Complexity**: O(n) for per-node storage
- **Parallelization**: Highly parallel (each node independent)

### Relational
- **Time Complexity**: O(m) for m edges, or O(n²) for all pairs
- **Space Complexity**: O(m) for per-edge storage
- **Parallelization**: Moderate (can parallelize over edges)

### Collective
- **Time Complexity**: Often O(n²) or higher (global algorithms)
- **Space Complexity**: O(n) or O(m) for graph structure
- **Parallelization**: More challenging (requires coordination)

---

## ML Pipeline Alignment

### Three ML Pipeline Types

```
ML Pipeline Types map to Classification:
├── Node Classification/Regression → Local (node-level)
├── Link Prediction → Relational (edge-level)
└── Graph Classification → Collective (graph-level) [Future]
```

### Feature Extraction by Scope

**Local Features**:
- Node properties
- Node embeddings
- Local centrality scores
- Neighbor aggregations

**Relational Features**:
- Edge properties
- Pair embeddings (concat source + target)
- Pair similarity scores
- Path features between pairs

**Collective Features**:
- Graph-level statistics (density, clustering)
- Aggregated node properties (mean, max, etc.)
- Community structure features
- Global centrality distributions

---

## Architectural Implications

### Data Structures

**Local (Node-level)**:
- `HugeDoubleArray` for node scores
- `NodePropertyStore` for node features
- Per-node arrays

**Relational (Edge-level)**:
- `HugeObjectArray<Edge>` for edges
- `RelationshipPropertyStore` for edge features
- Edge lists, adjacency lists

**Collective (Graph-level)**:
- Graph-wide data structures
- `GraphPropertyStore` for metadata
- Global aggregations

### Algorithm Execution Patterns

**Local**:
- Parallel over nodes
- No coordination needed
- Embarrassingly parallel

**Relational**:
- Parallel over edges or pairs
- Some coordination for pair-wise operations
- Moderate coordination needed

**Collective**:
- Global iterations (Pregel, BSP)
- Requires coordination (master_compute)
- Graph-wide synchronization

---

## Connection to Kantian Philosophy

### Local = Empirical Individual

**Kantian Perspective**: 
- Local algorithms operate on individual entities (nodes)
- Each node's computation is independent (empirical, contingent)
- No global coordination needed

### Relational = Pure Relation

**Kantian Perspective**:
- Relational algorithms operate on pairs (pure relation, Dyadic)
- The relation itself is the object of study
- Connects local entities (nodes) without requiring global structure

### Collective = Transcendental Unity

**Kantian Perspective**:
- Collective algorithms require the whole graph (transcendental unity)
- Global coordination (master_compute) = transcendental apperception
- The graph as a unified whole (not just sum of parts)

---

## Conclusion

**The Three Classification Types**:

1. **Local (Nodal)**: Node-level computation
   - Independent per-node operations
   - Highly parallel
   - Node properties, node embeddings

2. **Relational (Pairs)**: Edge-level computation
   - Pair-wise operations
   - Moderate coordination
   - Link prediction, relationship features

3. **Collective (Global)**: Graph-level computation
   - Requires entire graph
   - Global coordination needed
   - Community detection, global centrality, graph embeddings

**Perfect Alignment**:
- Maps to triadic graph structure (Node/Relationship/Graph)
- Aligns with property store architecture
- Corresponds to ML pipeline types
- Reflects computational complexity patterns
- Connects to Kantian philosophy (Empirical/Relation/Transcendental)

This classification system provides a **unified framework** for understanding algorithm scope, data structures, execution patterns, and ML feature extraction across the entire graph data science platform.


# Graph Projection API - Gamma Translation Plan (Prakasa)

**Document Type**: Translation Plan (TP)  
**Status**: Prakasa Complete (Viyoga Stage - Membership Declared)  
**Date**: October 15, 2025  
**Package**: `org.neo4j.gds` graph-projection-api  
**Method**: **Gamma Translation** - Structure complete, implementation deferred  
**Target State**: **Pre-Prim 0.0.x** (Absolute Viyoga)

---

## Membership Protocol - Location within Encyclopedia

**This document locates itself as follows:**

```text
rust-gds Encyclopedia of Software Translations
│
├─ ADRs/ (Architecture Decision Records)
│  ├─ adr0001 - Property Graph Store Design
│  ├─ adr0002 - Triadic GraphStore Architecture
│  ├─ adr0006 - Projection as GDSL
│  └─ ... (numbered, architectural decisions)
│
├─ Translation Plans/ (TP) ← THIS DOCUMENT RESIDES HERE
│  ├─ LINK_PIPELINE_TRANSLATION_PLAN.md (TP-001)
│  ├─ GRAPH_PROJECTION_API_TRANSLATION_PLAN.md (TP-002) ← YOU ARE HERE
│  └─ ... (future translation plans)
│
├─ Translation Completions/ (TC)
│  ├─ LINK_PIPELINE_TRANSLATION_COMPLETE.md (TC-001)
│  └─ ... (post-translation reports)
│
├─ Philosophical Foundations/
│  ├─ BRAHMA_VIDYA_SEMANTIC_VERSIONING.md
│  ├─ PROJECTION_FUNNY_BUSINESS.md
│  └─ ... (conceptual frameworks)
│
└─ Workflow Templates/ (Universal patterns)
   ├─ TRANSLATION_WORKFLOW_TEMPLATE.md
   └─ TRANSLATION_INDEX.md (Navigation hub)
```

**Location Justification** (Fichte's Protocol):

1. **This is a Translation Plan** - NOT an ADR, NOT a completion report
2. **This precedes execution** - Prakasa (illumination) before Kriya (work)
3. **This is Viyoga Stage** - Separation, where we declare membership
4. **This serves as template** - Shows HOW to plan complex translations

**Related Documents**:

- **Depends on**: `adr0006_projection_as_gdsl.md` (architectural foundation)
- **Coordinates with**: `TRANSLATION_WORKFLOW_TEMPLATE.md` (universal method)
- **Will produce**: `GRAPH_PROJECTION_API_TRANSLATION_COMPLETE.md` (TC-002, future)

---

## Executive Summary - The Prakasa

**"The Plans are our Prakasa, our Phase Diagrams, our Workflows"**

This is the **illumination** that sees the whole Graph Projection architecture before we begin the **Kriya** (work of translation). We create this plan **as if we hadn't already translated it** - to have a record of **how we approach** complex architectural translation.

### The Philosophy

```text
PRAKASA (प्रकाश) = Illumination, Seeing the Whole
  - Plan before execute
  - Understand before translate
  - Illuminate before act

KRIYA (क्रिया) = Action, The Work
  - Translation execution
  - Code generation
  - Implementation

KRAMA (क्रम) = Order, Progression
  - Phased approach
  - Dependency-aware sequencing
  - Ordered completion

This document IS Prakasa - the light that illuminates the path!
```

### What is Graph Projection?

**Graph Projection** = The system for defining HOW to load/create graph views from data

**Core Concept**:

- You have **source data** (Neo4j database, CSV files, etc.)
- You define a **projection** (what nodes, relationships, properties to include)
- System creates a **projected graph** (in-memory graph matching your spec)

**Why "Projection"?**

- Just like political projection (attributes your "native factory" to external representation)
- Graph projection attributes **stored data structure** to **graph views**
- The projection is how you **see** the underlying data!

## Package Overview

**Total Files**: 17 Java files (~2,500 lines)  
**Package Structure**:

```text
org.neo4j.gds/
├─ Core Identifiers (2 files, ~115 lines)
│  ├─ ElementIdentifier.java (67 lines) - Base class for NodeLabel/RelationshipType
│  └─ AbstractProjections.java (42 lines) - Base for collections
│
├─ Type Identifiers (2 files, ~100 lines)
│  ├─ NodeLabel.java (48 lines) - Node type identifier
│  └─ RelationshipType.java (52 lines) - Relationship type identifier
│
├─ Element Projections (1 file, ~166 lines)
│  └─ ElementProjection.java (166 lines) - Base projection class
│
├─ Node Projection (2 files, ~357 lines)
│  ├─ NodeProjection.java (143 lines) - Single node projection
│  └─ NodeProjections.java (214 lines) - Collection of node projections
│
├─ Relationship Projection (3 files, ~521 lines)
│  ├─ RelationshipProjection.java (248 lines) - Single relationship projection
│  ├─ RelationshipProjections.java (190 lines) - Collection
│  └─ Orientation.java (83 lines) - NATURAL/REVERSE/UNDIRECTED
│
├─ Property System (2 files, ~426 lines)
│  ├─ PropertyMapping.java (229 lines) - Single property mapping
│  └─ PropertyMappings.java (197 lines) - Collection
│
└─ API Values (5 files, ~881 lines)
   ├─ api/DefaultValue.java (272 lines) - Default value handling
   ├─ api/DefaultValueUtil.java (127 lines) - Default value parsing
   ├─ api/ValueConversion.java (73 lines) - Type conversions
   ├─ api/nodeproperties/ValueType.java (257 lines) - Property value types
   └─ core/Aggregation.java (152 lines) - Aggregation strategies
```

## Architectural Understanding (The Deep Prakasa)

### The Core Pattern

```text
PROJECTION ARCHITECTURE (CAR-CDR Structure)

ElementIdentifier (CAR - The Given)
  ├─ NodeLabel (of "Person")
  └─ RelationshipType (of "KNOWS")

ElementProjection (CDR - The Reconstruction)
  ├─ NodeProjection (label + properties)
  ├─ RelationshipProjection (type + orientation + aggregation + properties)
  └─ PropertyMappings (list of PropertyMapping)

Collections (Unity)
  ├─ NodeProjections (Map<NodeLabel, NodeProjection>)
  ├─ RelationshipProjections (Map<RelationshipType, RelationshipProjection>)
  └─ PropertyMappings (List<PropertyMapping>)
```

### The Key Insight (Political Projection Metaphor)

**What we're building**:

- `ElementIdentifier` = The "native factory" label (NodeLabel, RelationshipType)
- `ElementProjection` = The projection structure (what/how to project)
- `AbstractProjections` = Collection of projections
- `PropertyMapping` = How to map properties from source to target

**The funny business**:

- User says "give me a graph with Person nodes"
- We **project** the native storage (factory) into a graph view
- User works with "Person nodes" not "storage blocks"
- **Attribution of internal structure to external view!**

### The Duality Pattern

```text
IDENTIFIER vs PROJECTION (The Is vs The Ought)

IDENTIFIER (The Is - Das Ist)
  - NodeLabel.of("Person") = The Given
  - RelationshipType.of("KNOWS") = The Appearance
  - Just a name/string
  - Prim (primitive identification)

PROJECTION (The Ought - Das Soll)
  - NodeProjection(label="Person", properties=[...])
  - RelationshipProjection(type="KNOWS", orientation=NATURAL, ...)
  - Full specification with properties, aggregation, defaults
  - Proper (property-rich specification)

Pattern: Proper CONTAINS Prim!
- Every Projection contains an Identifier
- Truth contains Appearance!
```

## Translation Strategy (Gamma Method)

### What is Gamma Translation?

**Gamma Translation** = Recognizing Pre-Prim as architecturally complete

```text
GAMMA TRANSLATION CHECKLIST:
✅ Structure complete (types, traits, modules defined)
✅ API articulated (methods, functions, public interface)
✅ Compiles (zero errors, type-checks)
✅ Tests pass (behavior validated at structural level)
✅ TODOs explicit (Bija seeds planted, implementation points marked)
⏳ Implementation deferred (actual logic waits for Prim 0.1.x)

This is PRE-PRIM 0.0.x (Absolute Viyoga - architectural separation)
```

### Why Gamma for Projection?

**Projection is conceptually complex**:

- Multiple interacting types (Identifier, Projection, Mappings)
- Recursive structures (Projections contain Projections)
- Builder patterns everywhere
- Type conversions and parsing
- Validation logic

**Gamma approach**:

1. **First**: Get the architecture right (structure, traits, types)
2. **Second**: Define the API surface (methods, constructors, accessors)
3. **Third**: Plant implementation TODOs (Bija seeds)
4. **Later**: Implement in Prim 0.1.x (when dependencies ready)

## Phase Breakdown (The Krama - Ordered Progression)

### Phase 1: Core Identifiers (Foundation)

**Files**: 2 files, ~115 lines  
**Time Estimate**: 1 hour  
**Dependencies**: None (foundational)

**1.1 ElementIdentifier.java → element_identifier.rs** (67 lines)

- Abstract base class → Rust trait
- Fields: `name: String`
- Methods: `name()`, `project_all()` (abstract)
- Validation: No wildcards, no empty strings
- Pattern: Base trait for NodeLabel and RelationshipType

**1.2 AbstractProjections.java → abstract_projections.rs** (42 lines)

- Generic base for projection collections
- Type parameters: `<I: ElementIdentifier, P: ElementProjection>`
- Methods: `projections()`, `all_properties()`, `all_projections()`
- Pattern: Trait for NodeProjections and RelationshipProjections

**Gamma Deliverable**:

- ✅ Two traits defined (ElementIdentifier, AbstractProjections)
- ✅ Type bounds clear
- ✅ Tests for trait constraints
- ⏳ Concrete types deferred to Phase 2

### Phase 2: Type Identifiers (Concrete Given)

**Files**: 2 files, ~100 lines  
**Time Estimate**: 1 hour  
**Dependencies**: Phase 1 (ElementIdentifier)

**2.1 NodeLabel.java → node_label.rs** (48 lines)

- Struct: `NodeLabel { name: String }`
- Implements: `ElementIdentifier` trait
- Constants: `ALL_NODES = NodeLabel::of("__ALL__")`
- Factories: `of(name)`, `list_of(names...)`
- Pattern: Simple interned identifier

**2.2 RelationshipType.java → relationship_type.rs** (52 lines)

- Struct: `RelationshipType { name: String }`
- Implements: `ElementIdentifier` trait
- Constants: `ALL_RELATIONSHIPS = RelationshipType::of("__ALL__")`
- Factories: `of(name)`, `to_string(rel)`, `list_of(...)`
- Pattern: Simple interned identifier (already exists in codebase!)

**Gamma Deliverable**:

- ✅ Two concrete identifier types
- ✅ Constants defined
- ✅ Factory methods (PhantomData for validation logic)
- ⏳ Validation logic deferred to Prim 0.1.x

### Phase 3: Value System (Prim and Proper Foundation)

**Files**: 5 files, ~881 lines  
**Time Estimate**: 4-5 hours (largest phase!)  
**Dependencies**: None (foundational value types)

**3.1 api/nodeproperties/ValueType.java → value_type.rs** (257 lines)

- Enum: LONG, DOUBLE, LONG_ARRAY, DOUBLE_ARRAY, FLOAT_ARRAY, UNKNOWN
- Methods: `fallbackValue()`, `cypherName()`, `equals(ValueType)`
- Pattern: Property value type enumeration

**3.2 api/DefaultValue.java → default_value.rs** (272 lines)

- Struct: `DefaultValue { value: Option<Value>, is_user_defined: bool }`
- Constants: DEFAULT, INTEGER_DEFAULT_FALLBACK, LONG_DEFAULT_FALLBACK, etc.
- Factories: `of(value)`, `forInt()`, `forLong()`, `forDouble()`, etc.
- Accessors: `longValue()`, `doubleValue()`, `floatValue()`, `*ArrayValue()`
- Pattern: Type-safe default value container

**3.3 api/DefaultValueUtil.java → default_value_util.rs** (127 lines)

- Util functions for parsing defaults
- Methods: `parseDoubleArrayValue()`, `parseLongArrayValue()`, `parseFloatArrayValue()`
- Pattern: Type conversion utilities

**3.4 api/ValueConversion.java → value_conversion.rs** (73 lines)

- Safe type conversions
- Methods: `exactDoubleToLong()`, `exactLongToDouble()`, `exactLongToFloat()`, `notOverflowingDoubleToFloat()`
- Pattern: Checked conversions with errors

**3.5 core/Aggregation.java → aggregation.rs** (152 lines)

- Enum: DEFAULT, NONE, SINGLE, SUM, MIN, MAX, COUNT
- Methods: `merge(running_total, value)`, `normalizePropertyValue()`, `emptyValue()`
- Pattern: Aggregation strategy enum (already partially exists!)

**Gamma Deliverable**:

- ✅ ValueType enum complete
- ✅ DefaultValue struct with factories
- ✅ Conversion utilities defined
- ✅ Aggregation enum (may already exist in codebase!)
- ⏳ Parsing logic deferred (Bija seeds)

**Why This Phase is Critical**:
This is the **Prim and Proper** foundation!

- ValueType = Types of Prim (primitive values)
- DefaultValue = Prim values (actual primitives)
- Aggregation = Operations on Prim
- These enable property mappings (Proper)!

### Phase 4: Property System (Proper - The Ought)

**Files**: 2 files, ~426 lines  
**Time Estimate**: 3 hours  
**Dependencies**: Phase 3 (DefaultValue, Aggregation)

**4.1 PropertyMapping.java → property_mapping.rs** (229 lines)

- Struct: `PropertyMapping { property_key, neo_property_key, default_value, aggregation }`
- Factories: `of(key)`, `of(key, neo_key, default)`, `of(key, default, agg)`
- Methods: `hasValidName()`, `exists()`, `toObject()`, `setNonDefaultAggregation()`
- Parsing: `fromObject(key, stringOrMap)` - complex parsing logic
- Pattern: Single property mapping with validation

**4.2 PropertyMappings.java → property_mappings.rs** (197 lines)

- Struct: `PropertyMappings { mappings: Vec<PropertyMapping> }`
- Factories: `of(mappings...)`, `fromObject(input)`
- Methods: `propertyKeys()`, `stream()`, `iterator()`, `hasMappings()`, `isEmpty()`, `mergeWith(other)`
- Validation: Check for aggregation mixing
- Pattern: Collection with builder

**Gamma Deliverable**:

- ✅ PropertyMapping struct with all methods
- ✅ PropertyMappings collection
- ✅ Builder pattern defined
- ⏳ Complex parsing deferred (Bija seeds)
- ⏳ Validation logic deferred

**This IS "Proper"**:

- Properties are the "Proper" in "Prim and Proper"
- PropertyMapping = specification of how to handle properties
- This is where Truth (graph properties) emerges from Appearance (stored values)!

### Phase 5: Element Projection Base (The Projection Pattern)

**Files**: 1 file, ~166 lines  
**Time Estimate**: 2 hours  
**Dependencies**: Phase 4 (PropertyMappings)

**5.1 ElementProjection.java → element_projection.rs** (166 lines)

- Trait: `ElementProjection`
- Fields: `properties: PropertyMappings`
- Methods: `withAdditionalPropertyMappings()`, `projectAll()`, `toObject()`
- Pattern: Base trait for NodeProjection and RelationshipProjection
- Inner trait: `InlineProperties<Self>` for builder pattern

**Gamma Deliverable**:

- ✅ ElementProjection trait defined
- ✅ InlineProperties builder trait
- ✅ Method signatures complete
- ⏳ toObject() serialization deferred

**The Pattern Emerges**:

```text
ElementIdentifier (CAR - The Given) → Already translated (Phase 2)
ElementProjection (CDR - The Reconstruction) → This phase!

CAR (Given) + CDR (Reconstruction) = Complete Structure!
```

### Phase 6: Orientation (Relationship Direction)

**Files**: 1 file, ~83 lines  
**Time Estimate**: 30 minutes  
**Dependencies**: None (independent enum)

**6.1 Orientation.java → orientation.rs** (83 lines)

- Enum: NATURAL, REVERSE, UNDIRECTED
- Methods: `inverse()`, `parse(input)`, `to_string()`
- Pattern: Simple enum (may already exist!)

**Gamma Deliverable**:

- ✅ Orientation enum
- ✅ Parse and display methods
- ✅ Tests for inverse relationships

**Quick Win**: This might already exist in codebase! Check before translating.

### Phase 7: Node Projection (Putting It Together - Nodes)

**Files**: 2 files, ~357 lines  
**Time Estimate**: 3 hours  
**Dependencies**: Phases 1-5 (identifiers, properties, base projection)

**7.1 NodeProjection.java → node_projection.rs** (143 lines)

- Struct: `NodeProjection { label: String, properties: PropertyMappings }`
- Implements: `ElementProjection` trait
- Constants: `ALL = NodeProjection::fromString("*")`
- Factories: `of(label)`, `all()`, `fromObject()`, `fromString()`, `fromMap()`
- Methods: `label()`, `properties()`, `projectAll()`, `withAdditionalPropertyMappings()`
- Builder: `NodeProjection::builder()`
- Pattern: Concrete projection for nodes

**7.2 NodeProjections.java → node_projections.rs** (214 lines)

- Struct: `NodeProjections { projections: HashMap<NodeLabel, NodeProjection> }`
- Implements: `AbstractProjections` trait
- Constants: `ALL = NodeProjections::create(...)`
- Factories: `fromObject()`, `fromString()`, `fromMap()`, `fromList()`, `create()`, `single()`, `all()`
- Methods: `projections()`, `addPropertyMappings()`, `labelProjection()`, `isEmpty()`, `toObject()`
- Validation: Property key uniqueness
- Pattern: Collection with complex parsing

**Gamma Deliverable**:

- ✅ NodeProjection struct complete
- ✅ NodeProjections collection complete
- ✅ Builder pattern implemented
- ⏳ Complex parsing logic deferred (fromObject, fromMap, fromList)
- ⏳ Validation logic deferred

**This Completes Nodes**:

- NodeLabel (identifier) + NodeProjection (spec) = Complete node projection!

### Phase 8: Relationship Projection (Putting It Together - Relationships)

**Files**: 2 files, ~438 lines  
**Time Estimate**: 4 hours (most complex!)  
**Dependencies**: Phases 1-6 (identifiers, properties, base, orientation)

**8.1 RelationshipProjection.java → relationship_projection.rs** (248 lines)

- Struct: `RelationshipProjection { type, orientation, aggregation, index_inverse, properties }`
- Implements: `ElementProjection` trait
- Constants: `ALL`, `ALL_UNDIRECTED`
- Factories: `of(type, orientation)`, `fromMap()`, `fromString()`, `builder()`
- Methods: `type()`, `orientation()`, `aggregation()`, `indexInverse()`, `properties()`, `projectAll()`, `withAdditionalPropertyMappings()`
- Validation: `check()`, `checkAggregation()` - aggregation requires properties
- Pattern: Most complex projection (orientation, aggregation, inverse indexing)

**8.2 RelationshipProjections.java → relationship_projections.rs** (190 lines)

- Struct: `RelationshipProjections { projections: HashMap<RelationshipType, RelationshipProjection> }`
- Implements: `AbstractProjections` trait
- Constants: `ALL`, `ALL_UNDIRECTED`
- Factories: `fromObject()`, `fromString()`, `fromMap()`, `fromList()`, `single()`, `create()`
- Methods: `projections()`, `getFilter()`, `addPropertyMappings()`, `modifyProjections()`, `typeFilter()`, `isEmpty()`
- Pattern: Collection with operator-based modification

**Gamma Deliverable**:

- ✅ RelationshipProjection struct complete (most complex!)
- ✅ RelationshipProjections collection complete
- ✅ Orientation integration
- ✅ Aggregation integration
- ⏳ Complex validation deferred
- ⏳ Parsing logic deferred

**This Completes Relationships**:

- RelationshipType (identifier) + RelationshipProjection (spec) = Complete relationship projection!

## Summary Statistics

### Total Scope

- **Files**: 17 Java files → 17 Rust files
- **Lines**: ~2,500 Java LOC → ~3,500 Rust LOC (estimated)
- **Phases**: 8 phases (ordered by dependency)
- **Estimated Time**: ~18-20 hours for Gamma translation

### Phase Breakdown

1. **Phase 1**: Core Identifiers (2 files, 1 hour)
2. **Phase 2**: Type Identifiers (2 files, 1 hour)
3. **Phase 3**: Value System (5 files, 4-5 hours) ⚠️ **Largest**
4. **Phase 4**: Property System (2 files, 3 hours)
5. **Phase 5**: Element Projection (1 file, 2 hours)
6. **Phase 6**: Orientation (1 file, 0.5 hours)
7. **Phase 7**: Node Projection (2 files, 3 hours)
8. **Phase 8**: Relationship Projection (2 files, 4 hours) ⚠️ **Most Complex**

### Complexity Levels

**Simple** (5 files, ~3 hours):

- ElementIdentifier, AbstractProjections
- NodeLabel, RelationshipType (might exist!)
- Orientation (might exist!)

**Medium** (6 files, ~8 hours):

- ValueType, ValueConversion
- DefaultValue, DefaultValueUtil
- PropertyMapping, PropertyMappings

**Complex** (6 files, ~9 hours):

- Aggregation (might exist partially!)
- ElementProjection
- NodeProjection, NodeProjections
- RelationshipProjection, RelationshipProjections

## Implementation Strategy (The Kriya Plan)

### Gamma Translation Principles

**For Each File**:

1. **Structure First**: Define struct/enum/trait with fields
2. **API Surface**: Add all method signatures
3. **Tests**: Write tests for structure and API
4. **TODOs**: Plant Bija seeds for implementation
5. **Compile**: Ensure zero errors
6. **Document**: Add philosophical comments

**What to Implement Now**:

- ✅ Type definitions (structs, enums, traits)
- ✅ Method signatures (full API surface)
- ✅ Simple accessors (getters)
- ✅ Constants and factories (structure)
- ✅ Tests (structural validation)

**What to Defer (Bija Seeds)**:

- ⏳ Complex parsing (fromObject, fromMap, fromList)
- ⏳ Validation logic (detailed checks)
- ⏳ Type conversions (parsing arrays, checking bounds)
- ⏳ Error messages (detailed formatting)
- ⏳ Builder implementations (complex state)

### Module Organization

```rust
// src/projection/api/mod.rs
pub mod element_identifier;
pub mod abstract_projections;
pub mod node_label;
pub mod relationship_type;
pub mod value_type;
pub mod default_value;
pub mod default_value_util;
pub mod value_conversion;
pub mod aggregation;
pub mod property_mapping;
pub mod property_mappings;
pub mod element_projection;
pub mod orientation;
pub mod node_projection;
pub mod node_projections;
pub mod relationship_projection;
pub mod relationship_projections;

// Re-exports
pub use element_identifier::ElementIdentifier;
pub use node_label::NodeLabel;
pub use relationship_type::RelationshipType;
// ... etc
```

### Dependency Graph

```text
Level 0 (No dependencies):
  - ElementIdentifier (trait)
  - AbstractProjections (trait)
  - ValueType (enum)
  - Orientation (enum)

Level 1 (Level 0 deps):
  - NodeLabel (impl ElementIdentifier)
  - RelationshipType (impl ElementIdentifier)
  - ValueConversion (uses ValueType)
  - Aggregation (uses ValueType)

Level 2 (Level 1 deps):
  - DefaultValue (uses ValueType, Aggregation)
  - DefaultValueUtil (uses DefaultValue)

Level 3 (Level 2 deps):
  - PropertyMapping (uses DefaultValue, Aggregation)

Level 4 (Level 3 deps):
  - PropertyMappings (uses PropertyMapping)

Level 5 (Level 4 deps):
  - ElementProjection (uses PropertyMappings)

Level 6 (Level 5 deps):
  - NodeProjection (impl ElementProjection, uses NodeLabel)
  - RelationshipProjection (impl ElementProjection, uses RelationshipType, Orientation, Aggregation)

Level 7 (Level 6 deps):
  - NodeProjections (impl AbstractProjections, uses NodeLabel, NodeProjection)
  - RelationshipProjections (impl AbstractProjections, uses RelationshipType, RelationshipProjection)
```

**Translation Order**: Follow dependency levels (0 → 7)

## What We Learn From This Plan (Meta-Prakasa)

### Why This Document Exists

**"I want a record of how we do this. The Plans are our Prakasa"**

This plan IS our **workflow documentation**. It shows:

1. **How we approach complexity** (phase by dependency level)
2. **How we see the whole** (Prakasa before Kriya)
3. **How we order work** (Krama - dependency-aware progression)
4. **How we use Gamma** (structure before implementation)
5. **How we plant seeds** (Bija - TODOs as future growth)

### The Workflow Pattern

```text
PROJECTION TRANSLATION WORKFLOW (Universal Pattern)

1. PRAKASA (Illumination)
   - Read all source files
   - Count lines, identify dependencies
   - Group into phases
   - Create dependency graph
   - Write THIS PLAN

2. KRIYA (Action)
   - Phase 1: Translate level 0 dependencies
   - Phase 2: Translate level 1 dependencies
   - ...
   - Phase N: Translate top-level types
   - Each file: Structure → API → Tests → TODOs → Compile

3. KRAMA (Progression)
   - Follow dependency order strictly
   - Complete each phase before next
   - Validate at each step
   - Track progress (24/25 files style)

4. VIYOGA (Separation)
   - Structure separated from implementation
   - API separated from logic
   - Tests separated from production code
   - This is Pre-Prim 0.0.x!

5. BIJA (Seeds)
   - Plant TODOs explicitly
   - Document what's deferred
   - Mark future implementation points
   - "Yellows and reds = seeds"

6. SANYOGA (Union) - Future
   - Prim 0.1.x: Implement primitives
   - Proper 1.0.x: Complete properties
   - Prim and Proper 1.x.x: Full system
```

### Transferable Knowledge

**This workflow applies to ANY complex translation**:

- ✅ Understand before translate (Prakasa)
- ✅ Plan before execute (create THIS document)
- ✅ Order by dependencies (Krama)
- ✅ Structure before implementation (Gamma / Viyoga)
- ✅ Plant seeds for future (Bija)
- ✅ Validate at each step (tests)

**The Pattern Recognition**:

- Types package followed this (implicitly)
- LinkPipeline followed this (explicitly)
- Projection will follow this (documented here!)
- **Any future package** can follow this!

## Next Actions

### Immediate (If Starting Now)

1. **Create module structure** (`src/projection/api/`)
2. **Start Phase 1** (ElementIdentifier, AbstractProjections)
3. **Follow Krama** (complete each phase sequentially)
4. **Track in TODO list** (update progress like LinkPipeline)
5. **Celebrate each phase** (recognize completion!)

### Or: Document First, Translate Later

Since some of this MAY already exist in the codebase:

1. **Review existing code** (check what's already translated)
2. **Gap analysis** (what's missing vs this plan)
3. **Merge plan** (integrate existing + planned)
4. **Execute gaps** (translate only what's needed)

## Philosophical Notes (The Deep Prakasa)

### Why Graph Projection is Profound

**Graph Projection** embodies all our philosophical frameworks:

**Prim and Proper**:

- Identifiers (NodeLabel, RelationshipType) = **Prim** (The Given, The Is)
- Projections (Node/RelationshipProjection) = **Proper** (The Truth, The Ought)
- PropertyMappings = Bridge from Prim to Proper!

**Political Projection Metaphor**:

- Native Factory (storage) → Projected onto → Graph Views
- Just like: Unconscious (native factory) → Projected onto → External perception
- Difference: Graph projection is **conscious** (good design!)

**CAR-CDR Structure**:

- ElementIdentifier = CAR (The Given)
- ElementProjection = CDR (The Reconstruction)
- Collections = Unity (projections map)

**Viyoga and Sanyoga**:

- This plan achieves **Viyoga** (architectural separation)
- Gamma translation maintains **Viyoga** (structure without implementation)
- Future Prim 0.1.x begins **Sanyoga** (union of structure and implementation)

### The Meta-Lesson

**"This Projection business is a funny business"** (politically and graphically!)

And now we have **a plan** for translating it - a plan that itself embodies:

- **Prakasa** (this document illuminates the whole)
- **Kriya** (future translation work)
- **Krama** (ordered phase progression)

**The plan IS the philosophy in action!**

---

_"The Plans are our Prakasa, our Phase Diagrams, our Workflows. This document illuminates the path for translating Graph Projection API - whether we've translated it before or not. It's a record of HOW we approach complex architectural translation using Gamma method, Prakasa-Kriya-Krama triad, and Bija philosophy."_

_"Graph Projection: Where 'projecting your native factory onto external views' meets 'projecting stored data onto graph views'. Both are funny businesses - one psychological, one graphical, both architectural!"_

**Graph Projection API - Prakasa Complete! Ready for Kriya!** 🕉️

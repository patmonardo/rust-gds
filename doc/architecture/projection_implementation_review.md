# Projection Implementation Review

## Date: October 1, 2025

## Current Status: PAUSED FOR REVIEW

### What We Accomplished

#### 1. Module Refactoring ✅

- **Renamed**: `src/projection/abstract/` → `src/projection/traits/`
- **Created**: `src/projection/impls/` for concrete implementations
- **Rationale**: Idiomatic Rust - `traits/` for behavior, `impls/` for concrete types
- **Result**: Clean separation, no keyword conflicts, all existing tests pass

#### 2. Core Foundation ✅

**NodeLabel** and **RelationshipType** are SOLID:

- Thread-safe interning with `Arc<String>` + `RwLock`
- Clean API, well-tested (13 tests passing)
- Used extensively throughout the codebase
- These are the **real workhorses**

#### 3. Projection Traits ✅ (Already Complete)

In `src/projection/traits/`:

- `property_mapping.rs` - PropertyMapping struct, Aggregation enum
- `element_projection.rs` - ElementProjection trait, PropertyMappings (HashMap-based)
- `abstract_projections.rs` - AbstractProjections trait, Projections implementation
- **16 tests passing** for all trait-based code

### What Got Messy: PropertyMappings Confusion

#### The Problem

There are **TWO different PropertyMappings** concepts that got conflated:

1. **In Traits** (`traits/element_projection.rs`):

   ```rust
   pub struct PropertyMappings {
       mappings: HashMap<String, PropertyMapping>,  // HashMap for lookup
   }
   ```

   - Used by ElementProjection trait
   - HashMap-based for efficient property lookup
   - Already complete and working

2. **In Impls** (`impls/property_mappings.rs`) - **ATTEMPTED**:
   ```rust
   pub struct PropertyMappings {
       mappings: Vec<PropertyMapping>,  // Vec for ordering + validation
   }
   ```
   - Vec-based with ordering preservation
   - Aggregation mixing validation
   - Builder pattern with default aggregation
   - **STATUS**: Compiles but tests broken (42 test errors)

#### Why This Happened

Looking at TypeScript:

- `projection/abstract/ElementProjection.ts` - has inline PropertyMappings usage
- `projection/primitive/PropertyMappings.ts` - separate concrete collection class

I tried to mirror this but **the distinction isn't needed in Rust** the same way.

### Architectural Insight

**NodeLabel** and **RelationshipType** are fundamental primitives used everywhere.

**Projections** are higher-level configuration objects used in specific contexts:

- Graph loading/projection configuration
- Not as pervasive as NodeLabel/RelationshipType
- More complex with subelements (NodePropertyMapping, etc.)

### What Actually Needs Implementation

From your TS code, the projection hierarchy is:

```
Projections (Abstract Layer - DONE)
├── AbstractProjections trait ✅
├── PropertyMapping struct ✅
└── ElementProjection trait ✅

Projections (Concrete Layer - MINIMAL NEED)
├── NodeProjection
│   ├── Uses PropertyMappings collection
│   └── Has label + properties
├── RelationshipProjection
│   ├── Uses PropertyMappings collection
│   ├── Has type + orientation + aggregation + properties
│   └── More complex than NodeProjection
├── NodeProjections (collection of NodeProjection)
└── RelationshipProjections (collection of RelationshipProjection)
```

But also subelements you mentioned:

- NodePropertyMapping
- RelationshipPropertyMapping
- Various other specialized mappings

### Recommendation

Given that:

1. **NodeLabel and RelationshipType are battle-tested and solid**
2. **Projection traits are complete and working**
3. **Projection impls got complicated fast**
4. **You're not sure how much you need the projection stuff**

**I suggest:**

#### Option 1: Minimal Path ⭐ RECOMMENDED

- **Keep**: NodeLabel, RelationshipType (the workhorses)
- **Keep**: Projection traits (already working, provide interfaces)
- **Delete**: The messy `impls/property_mappings.rs` attempt
- **Document**: What projection impls would look like if needed later
- **Move on**: To more critical parts (IdMap? GraphStore? Polars integration?)

#### Option 2: Clean Slate

- Review the TS projection code more carefully
- Understand the subelement structure (NodePropertyMapping, etc.)
- Design a cleaner Rust API from scratch
- Implement only what you actually need

#### Option 3: Finish PropertyMappings

- Fix the 42 test errors (mechanical work)
- But this might be effort for code you don't need

### Files to Review

**Good (Keep)**:

- `src/projection/node_label.rs` ✅
- `src/projection/relationship_type.rs` ✅
- `src/projection/traits/*.rs` ✅

**Questionable (Review)**:

- `src/projection/impls/property_mappings.rs` ❓ (compiles but tests broken)
- `src/projection/traits/element_projection.rs` ❓ (has its own PropertyMappings)

**Documentation**:

- `doc/projection_module.md` - NodeLabel/RelationshipType
- `doc/projection_abstract_module.md` - Trait system
- `doc/projection_refactoring.md` - Module restructure

### Key Takeaway

**NodeLabel** and **RelationshipType** with their interning system are **genuinely valuable**.

The **projection configuration objects** are more complex and might not be worth the effort right now, especially with subelements like NodePropertyMapping that add another layer.

**Better to pause, review your actual needs, and prioritize what matters most.**

---

## Next Steps (Your Choice)

1. **Clean up** - Remove messy PropertyMappings impl, keep the good stuff
2. **Redirect** - Focus on IdMap, GraphStore, or Polars integration
3. **Study** - Review TS projection architecture more carefully first
4. **Continue** - Fix tests and implement full projection system (significant effort)

What serves your actual use case best?

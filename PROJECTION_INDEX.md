# PROJECTION: Master Index and Navigation Guide

## Quick Navigation

### For the Big Picture

Start here if you're new to Projection:

1. **`doc/PROJECTION_FIVE_FOLD_SYNTHESIS.md`** — Philosophical grounding and stakes
2. **`doc/PROJECTION_ARCHITECTURE_COMPLETE.md`** — Complete architectural guide
3. **`PROJECTION_QUICK_REFERENCE.md`** — Quick lookup guide

### For Understanding Recursion

Start here if you want to understand eval and catalog:

1. **`doc/PROJECTION_RECURSIVE_DESCENT.md`** — How eval and catalog are recursive
2. **`doc/PROJECTION_ARCHITECTURE_DIAGRAM.md`** — Visual system overview
3. **`SESSION_SUMMARY_RECURSIVE_DESCENT.md`** — The breakthrough explained

### For Implementation Details

Start here if you're coding:

1. **`src/projection/codegen/mod.rs`** — Ground concept (read the docs)
2. **`src/projection/codegen/eval.rs`** — First recursive descent
3. **`src/projection/codegen/catalog.rs`** — Second recursive descent
4. **`PROJECTION_QUICK_REFERENCE.md`** — Quick API reference

---

## The Complete Concept in One Page

### What is Projection?

**Projection = Complete Knowledge + Complete Power = One Principle**

```
Projection unifies:
- Omniscience (what we KNOW) = eval
- Omnipotence (what we CREATE) = catalog
```

### The Five-Fold Structure

Every manifestation of Projection has five moments:

1. **TRANSFORM** — Ground principle (Descriptor → Runtime)
2. **DESCRIPTOR** — Identity pole (what IS)
3. **MEMBERSHIP** — Inherence (what belongs to it)
4. **RUNTIME** — Difference pole (what MANIFESTS)
5. **CONSEQUENCE** — Logical entailment (what MUST follow)

### Recursive Descent

The concept applies at multiple levels:

```
Level 0: Projection (Ground)
├─ Level 1: eval (Projection into Computation)
└─ Level 2: catalog (Projection into Storage)
```

Each level inherits the Five-Fold and specializes it to a domain.

### How It Works

```
ComputationDescriptor
    ↓
eval.analyze()  ← Level 1 recursive descent (Omniscience)
    ↓
ComputationSchema (what we KNOW)
    ↓
catalog.create()  ← Level 2 recursive descent (Omnipotence)
    ↓
StorageRuntime (what we CREATE)
```

---

## Documentation by Audience

### For Architects

- **Primary:** `doc/PROJECTION_ARCHITECTURE_COMPLETE.md`
- **Secondary:** `doc/PROJECTION_RECURSIVE_DESCENT.md`
- **Visual:** `doc/PROJECTION_ARCHITECTURE_DIAGRAM.md`

### For Philosophers / Theory

- **Primary:** `doc/PROJECTION_FIVE_FOLD_SYNTHESIS.md`
- **Secondary:** `SESSION_SUMMARY_RECURSIVE_DESCENT.md`

### For Implementers

- **Primary:** `PROJECTION_QUICK_REFERENCE.md`
- **Secondary:** Code documentation in source files
- **Reference:** `doc/PROJECTION_ARCHITECTURE_COMPLETE.md`

### For Project Managers

- **Status:** `SESSION_SUMMARY_PROJECTION_FIVE_FOLD.md`
- **Progress:** `SESSION_SUMMARY_RECURSIVE_DESCENT.md`

---

## Code Organization

### Ground (codegen/)

```
src/projection/codegen/
├── mod.rs                    ← The Five-Fold Concept
├── consequence.rs            ← ConsequenceRule
├── eval.rs                   ← Eval trait
├── catalog.rs                ← Catalog trait
├── descriptors/              ← ComputationDescriptor, PropertyDescriptor, StorageDescriptor
├── runtimes/                 ← Computer, StorageRuntime, ProcedureFacade
└── transforms/               ← Type projection implementations
```

### First Descent: eval (Computation)

```
src/projection/codegen/eval.rs
├── ComputationTransform      ← Ground of this descent
├── ComputationDescriptor     ← Identity
├── ComputationMembership     ← Inherence
├── ComputationSchema         ← Difference (intermediate)
└── ComputationConsequence    ← Entailment
```

### Second Descent: catalog (Storage)

```
src/projection/codegen/catalog.rs
├── StorageTransform          ← Ground of this descent
├── StorageSchema             ← Identity (input)
├── StorageConstraints        ← Inherence
├── StorageRuntime            ← Difference (output)
└── StorageConsequence        ← Entailment
```

---

## Key Files

| File                                      | Purpose                     | Status         |
| ----------------------------------------- | --------------------------- | -------------- |
| `src/projection/codegen/mod.rs`           | Ground Concept (Five-Fold)  | ✅ Documented  |
| `src/projection/codegen/eval.rs`          | First Descent (Computation) | ✅ Documented  |
| `src/projection/codegen/catalog.rs`       | Second Descent (Storage)    | ✅ Documented  |
| `src/projection/codegen/consequence.rs`   | Logical Entailment          | ✅ Implemented |
| `doc/PROJECTION_FIVE_FOLD_SYNTHESIS.md`   | Philosophical Grounding     | ✅ Complete    |
| `doc/PROJECTION_ARCHITECTURE_COMPLETE.md` | Architecture Guide          | ✅ Complete    |
| `doc/PROJECTION_RECURSIVE_DESCENT.md`     | Recursion Explained         | ✅ Complete    |
| `doc/PROJECTION_ARCHITECTURE_DIAGRAM.md`  | Visual Overview             | ✅ Complete    |
| `PROJECTION_QUICK_REFERENCE.md`           | Quick Lookup                | ✅ Complete    |
| `SESSION_SUMMARY_PROJECTION_FIVE_FOLD.md` | First Session Summary       | ✅ Complete    |
| `SESSION_SUMMARY_RECURSIVE_DESCENT.md`    | Second Session Summary      | ✅ Complete    |

---

## Implementation Status

### Phase I: Possess the Concept ✅ COMPLETE

- ✅ Ground (Five-Fold) defined in codegen/
- ✅ eval module (first descent) implemented
- ✅ catalog module (second descent) implemented
- ✅ Recursive descent structure documented
- ✅ Code compiles, tests pass
- ✅ Backward compatible with existing code

### Phase II: Apply the Concept ⏳ IN PROGRESS

- ✅ eval trait defined and implemented
- ✅ catalog trait defined and implemented
- ⏳ Extend eval with full Computation schema extraction
- ⏳ Extend catalog with full Storage runtime creation
- ⏳ Build Pipeline (orchestrate eval ∘ catalog)
- ⏳ Integrate with eval! macro

### Phase III: Realize the System 📋 PLANNED

- 📋 Complete level implementations
- 📋 Macro-driven descriptor registration
- 📋 Runtime registry
- 📋 Further domain descents (Graph, Node, etc.)
- 📋 Compile-time optimization

---

## Getting Started

### If You're New to Projection

1. Read: `doc/PROJECTION_FIVE_FOLD_SYNTHESIS.md` (10 min)
2. Read: `doc/PROJECTION_ARCHITECTURE_COMPLETE.md` (20 min)
3. Scan: `doc/PROJECTION_ARCHITECTURE_DIAGRAM.md` (5 min)
4. Reference: `PROJECTION_QUICK_REFERENCE.md` (as needed)

**Total: ~35 minutes to understand the complete system**

### If You're Implementing

1. Read: `PROJECTION_QUICK_REFERENCE.md`
2. Read: `src/projection/codegen/mod.rs` (code docs)
3. Read: `src/projection/codegen/eval.rs` (code docs)
4. Read: `src/projection/codegen/catalog.rs` (code docs)
5. Implement your domain following the same Five-Fold pattern

### If You're Contributing

1. Read: All of above
2. Read: `doc/PROJECTION_RECURSIVE_DESCENT.md`
3. Ensure your code follows the Five-Fold pattern
4. Document your code explaining which "moment" each part represents
5. Reference the abstract principle, not just implementation details

---

## The Absolute Idea

```
PROJECTION = TRANSFORM<Omniscience, Omnipotence>
           = Complete Knowledge unified with Complete Power
           = The Architecture of Being Itself
```

This is not just code. This is how reality itself organizes.

The Five-Fold Concept is universal. It appears at every level:

- Ground (abstract principle)
- eval (knowledge extraction)
- catalog (power manifestation)
- (future domains via recursive descent)

When fully realized, every domain in rust-gds will be understandable through this one principle.

---

## References and Further Reading

### Philosophical Background

- Hegel's Dialectical Method (the Genetic Method)
- Kant's concept of Membership, Consequence, Inherence
- Vedantic philosophy (Maya, Brahman, Sat-Chit-Ananda)

### Related Code Patterns

- Visitor Pattern (implements domain operations)
- Strategy Pattern (determines behavior from constraints)
- Catalog Pattern (creates objects from specifications)
- Projection Pattern (maps between domains)

### Design Principles

- Constraint-based design (membership = constraints)
- Recursive descent (apply principle to subdomains)
- Minimal abstraction (Five-Fold is minimal, complete)

---

## Questions to Ask Yourself

### About Projection

- "What Five-Fold elements does this code represent?"
- "Is this a ground concept or a recursive descent?"
- "What domain is this specialized to?"
- "Could this principle apply to other domains?"

### About eval and catalog

- "Is eval analyzing or creating?"
- "Is catalog creating or analyzing?"
- "What is the output of eval?"
- "What is the input of catalog?"
- "How do they compose?"

### About New Domains

- "What is the Transform for this domain?"
- "What are the domain-specific Descriptors?"
- "What are the domain-specific Membership constraints?"
- "What Runtime do we want to create?"
- "What Consequences follow from the membership?"

---

## Contact and Support

For questions about:

- **Architecture:** See `doc/PROJECTION_ARCHITECTURE_COMPLETE.md`
- **Theory:** See `doc/PROJECTION_FIVE_FOLD_SYNTHESIS.md`
- **Implementation:** See `PROJECTION_QUICK_REFERENCE.md` and source code
- **Recursion:** See `doc/PROJECTION_RECURSIVE_DESCENT.md`

---

## Version and Updates

**Last Updated:** October 17, 2025

**Current Phase:** II (Apply the Concept) — IN PROGRESS

**Next Review:** When Phase II completion is planned

---

**Welcome to Projection. The Architecture of Being Itself.**

🙏

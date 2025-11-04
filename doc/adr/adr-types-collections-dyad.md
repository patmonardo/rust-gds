# ADR: Types-Values Dyad and Collections Interface

**Status:** DRAFT  
**Date:** 2025-01-27  
**Context:** Clarifying the relationship between the Types/Values dyad and the Collections substrate.

## Problem Statement

Our architecture contains three conceptually distinct but interwoven systems:

1. **Collections** (Level 0): Backend-agnostic data structure abstractions
2. **Types/Values** (Level 1 foundation): The Property Graph type algebra
3. **Projection** (Level 1 machine): The Graph World that subsumes Types/Values

The questions:
- **Does Values belong to Projection or Collections?**
- **What dyad perfectly describes the Collections machine?**

## Current Architecture

### Physical Structure
```
gds/src/
├── collections/    (81 files)  - Level 0
├── types/          (139 files) - Property Graph type system
├── values/         (7 files)   - Property value semantics
└── projection/     (156 files) - Graph World projection machine
```

### Conceptual Relationship

```
Projection(Types, Values) → GraphStore + API
    ↓
Collections(Structure, Operations) → Backend primitives
```

## Analysis

### 1. Collections Dyad

**Collections = Structure ⊗ Operations**

- **Structure**: The backend substrate (Vec, HugeArray, Arrow)
- **Operations**: The trait-based interface (push, get, iterate)
- The dyad is **backend abstraction** through **trait contracts**

Collections is fundamentally about **abstracting over storage** through **uniform interfaces**.

### 2. Types/Values Relationship

**Types = Graph World type algebra**
**Values = Property value semantics**

These are **semantically coupled** but can inhabit different architectural layers:

#### Option A: Types/Values as Projection-only
```
Projection(Types, Values) → GraphStore API
    ↓
Collections → Raw storage
```

- Types/Values are **projection machinery**
- Collections remain pure structure
- Clear separation: Collections doesn't know about Graph semantics

#### Option B: Types → Collection interface, Values → Projection
```
Types → Collection + Projection interface
Values → Projection implementation
```

- Types define **what Collection can store** (type parameters)
- Values define **how Projection interprets** storage (semantics)

#### Option C: Both in Projection
```
Projection(Types + Values) → Complete Graph World
    ↓
Collections → Pure substrate (no Graph semantics)
```

- Collections is **neutral substrate**
- All Graph semantics in Projection
- Types/Values sublated into Projection together

## Philosophical Analysis

### The Sublation (Hegelian Aufhebung)

**Aufhebung** = preserve, cancel, transcend

- Types/Values are **sublated into Projection**
- Their individuality is preserved (separate modules)
- Their separation is canceled (used together in Graph Store)
- The result transcends both (Graph World emerges)

So the dyad **Types:Values** becomes sublated into the **Projection machine**.

### The Collections Dyad

Collections is about:
- **Storage abstraction** (Structure)
- **Uniform access** (Operations)

The dyad is: **Structure ⊗ Operations**

But at the interface with Projection:
- Collections provides **storage**
- Projection provides **semantics**

This suggests: **Collections(Storage, Interface) ⊗ Projection(Types, Values)**

## Recommended Architecture

### The Top-Level Dyad

**Projection:Collections**

- **Projection** = Graph World semantics (Types + Values sublated)
- **Collections** = Storage substrate (Structure + Operations)

### Where Values Belongs

**Values belongs to Projection**, but interfaces through Collections:

```
Values → Property semantics (Graph World interpretation)
Types → Type algebra (Graph World structure)
Collections → Storage substrate (neutral, type-parameterized)
```

**Types** mediate between Projection and Collections:
- Types constrain what Collections stores (via trait bounds)
- Types define what Projection projects (via Graph semantics)
- Types are the **contract** between storage and interpretation

**Values** are purely Projection:
- Values are Graph World interpretations
- Values don't belong in Collections (which is backend-neutral)
- Values need Types + Collections to materialize

## The Rhyme

```
Projection(Types, Values) ⊗ Collections(Structure, Operations)
```

This rhymes because:
- **Both are dyads** (two-part systems)
- **Projection is semantic**, Collections is **structural**
- **Types mediates** between them (the contract)
- **Values lives purely** in the Graph World (Projection)

## Implementation Implications

1. **Collections Package**: Never reference `PropertyValue`, always `T: Clone + Send + Sync`
2. **Types Package**: Defines trait bounds that Collections must satisfy
3. **Values Package**: Implements Property semantics, depends on Types + Collections
4. **Projection Package**: Sublates Types + Values into GraphStore, uses Collections as substrate

## Open Questions

1. **Arrow Schema**: When we add Arrow support, where does schema live?
   - Answer: In Projection (Graph World semantics), not Collections
   
2. **Polars Integration**: Does Polars belong at Collection level or Projection level?
   - Answer: Collection level (backend), but schema interpretation in Projection

3. **Values as Embeddings**: When Values are numerical (embeddings), do they blur the boundary?
   - Answer: No - Values are Graph semantics. Even `Vec<f64>` is "Graph World values", not raw Collections.

## The Organic Unity of Types and Values

**Organic Unity** means that Types and Values are not just related, but **organically unified** - one puts one on top of the other, and the other side reverses the priority.

### The Two Regimes

#### 1. Type-Priority Regime (Schema emphasizes Types)
```
Schema → Property → Store ↔ Value
  ↑         ↑
Types     Configuration
(priority)
```
**Projection/Schema gives Types priority:**
- Schema defines structure (the "next" relation)
- Values fill that structure
- Types constrain Values
- This is the **Projection regime** - Schema-driven

#### 2. Value-Priority Regime (Values drive Types)
```
Value → Property → Store ↔ Schema
  ↑         ↑
Values   Interpretation
(priority)
```
**Embeddings/ML give Values priority:**
- Values are the semantics (embeddings)
- Types emerge from value distributions
- Schema follows Values
- This is the **ML regime** - Value-driven

### The Reversal

**Organic Unity** = each side can put the other on top:
- **Schema prioritizes Types** (structure first)
- **Embeddings prioritize Values** (semantics first)
- **Property enables the reversal** (the middle term can flip)

### The Property as Middle

**Property is the synthesizing third term** that mediates between:
- **Store** (Collections side - Types extreme)
- **Value** (Projection side - Values extreme)

Property is the **middle term** that enables the **priority reversal**.

Schema sets "next" to Property, meaning **Schema is the rule that configures Property** to mediate between Store and Value.

```
Types:Values (dyad with priority reversal)
    ↓
Property (middle/synthesis)
    ↓↙    ↘
Store   Value
(Collections)  (Projection)

But Property can FLIP the priority:
- Type-priority: Schema → Property → Store
- Value-priority: Value → Property → Store
```

## Left Hand Side vs Right Hand Side

**Projection = Left_Hand_Side**
**Collections = Right_Hand_Side**

Relating them uses **Types** (Universal Adapter), not Schemas.

### Schema vs Types

**Schema** = Domain-specific, Property-driven
- The "next" relation
- Property configuration
- Graph semantics
- **Schema drives Property** in the Projection regime

**Types** = Universal interface, structural
- Trait bounds
- Adapter pattern
- Interface contracts
- **Types mediate** between Left and Right

### Arrow as Type System

**Is Arrow more Type-system than Schema-system?**

Yes. Arrow provides:
- **Type-level** schemas (column types, memory layout)
- **Structural** constraints (field names, types)
- **Not** domain semantics (the "next" relation)

Arrow is **Type infrastructure**, not Schema-driven Property logic.

### Polars: Schema or Types?

**Polars is higher level** - analytics/DataFrame oriented.
**Polars is "more like 'us' in some ways"** - Schema-driven analytics.

So we should say **"Polars Schemas"** not "Polars Types" because:
- **Arrow** = Type-system (structural, low-level)
- **Polars** = Schema-system (analytics, higher-level, more like "us")

Polars sits closer to the Schema-driven world (Right Hand Side domain logic).

### The Hierarchy Clarification

**Types / Schema,Property**

The hierarchy shows:
- **Schema** = Upper layer, owns Store (Right Hand Side domain)
- **Property** = Middle term
- Property relates **Types to Values** (not Schema directly)

**Store owns Schema** - Schema is Right Hand Side domain logic, not Left Hand Side.

**Types and Values** are reciprocally, organically related - they show up in both Left and Right worlds.

### The Complete Picture

**Right Hand (Collections):** Schema-driven, Type-bounded, structure-focused
**Left Hand (Projection):** Value-driven, Type-bounded, semantic-focused
**Relation:** Types (Universal Adapter, shows in both)

**Schema drives Store** (Right Side regime)
**Property mediates** Types ↔ Values (the middle)
**Types and Values** appear in both Left and Right (organic unity)

### Projection: Codegen and Eval

**Codegen** → generates Graph Store API
**Eval** → Values process execution framework
**Both Codegen and Eval stay in Projection** (Left Hand Side)

**Arrow imports belong in Collections** (Right Hand Side, backend)

So:
- **Codegen + Eval** = Projection machinery (Left)
- **Arrow import** = Collections backend (Right)
- **Types** mediates between them

## Conclusion

### The Dyads Revisited

- **Types:Values** = The fundamental dyad
- **Collections** = One-sided extreme (Types side → Structure ⊗ Operations)  
  = **Right Hand Side**
- **Projection** = One-sided extreme (Values side → Graph semantics ⊗ E-Valuation)  
  = **Left Hand Side**

Both Projection and Collections are **partial views** of the Types:Values dyad.

### Property as Synthesis

**Property is the middle term** that:
- Mediates between Store (Collections) and Value (Projection)
- Synthesizes the two one-sided views into a complete picture
- Is configured by Schema (the "next" relation)

### The Complete Picture

**Values belongs to Projection** as the E-Valuator (Pure Form).  
**Types belongs to both** as the Universal Adapter (between Left and Right).  
**Property synthesizes** the two extremes through mediation.

**Arrow is Type-system** - the structural interface between Left and Right.
**Schema is Property-driven** - the domain logic of the Left Side.

The rhyme is perfect: **Projection:Collections** = **Left:Right** where both are one-sided extremes of **Types:Values**, mediated by **Property**.

**Schema drives Property (Left regime). Types mediates Left↔Right.**


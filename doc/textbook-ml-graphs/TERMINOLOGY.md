# Terminology Guide: Real:Ideal Type:Value Systems

## Property Architecture Terminology

This textbook uses **Kant-Fichte-Hegel language** for architectural clarity:

### Real:Ideal (Type:Value Systems)

**Real Type:Value System**
- Store-bound, Schema-bound
- PropertyValues (persistent, structured)
- Connected to definite Store
- The concrete, actualized pole

**Ideal Type:Value System**
- Free Values, Schema-less
- FeatureSpace / PrimitiveValues (empirical, computational)
- Not connected to definite Store
- The abstract, potential pole

**Property = Unites Both**
- Property is the **Middle** that unites Real and Ideal Type:Value systems
- Property = Universal mapping (Real → Ideal)
- PropertyDescriptor = Root Descriptor (sublates Storage + Computation)
- PropertyFunctors = Universal mapping system (Real → Ideal)

### Property as Middle of Middles

**Property sublates two Middles:**
1. **Ideal:Real Middle** = Synthesis of Ideal and Real Type:Value systems
2. **Type:Value Middle** = Synthesis of Type and Value
3. **Property** = Middle of Middles (sublates both syntheses)

### Feature vs PropertyValue

**Feature** = One-sided projection of Property
- Property loses connection to Schema → Becomes Feature
- Type → Value (direct, no Store)
- Half of Projection Space (Type→Value side)

**PropertyValue** = Store-bound subset of FeatureValues
- Real Type:Value system (Store-bound, persistent)
- Filtered overlay on PrimitiveValues (NotSupported filter)
- Complete Property (Store-bound + Free Values)

### Projection Mechanism

**Projection** = Built on four-fold PropertyFunctor foundation
- Simultaneously subsumes Real and Ideal Type:Value spaces
- PropertyFunctors serve Projection Evaluator
- Projection Mechanism = Four-fold PropertyFunctor foundation of PropertyDescriptors

## Legacy Terminology

**Note**: Codebase may still reference "Gross/Subtle" terminology in:
- Code comments
- Type names (GrossToSubtle, SubtleToGross, etc.)
- Older documentation

**Current terminology** (this textbook):
- **Real** (replaces "Gross") = Store-bound, Schema-bound, PropertyValues
- **Ideal** (replaces "Subtle") = Free Values, Schema-less, PrimitiveValues/FeatureSpace

Code-level refactoring (trait/type renaming) is a larger architectural change.


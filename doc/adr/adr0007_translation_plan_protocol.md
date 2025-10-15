# ADR0007: Translation Plan Protocol and Encyclopedia Structure

**Status**: Accepted  
**Date**: October 15, 2025  
**Decision Maker**: rust-gds development team  
**Type**: Process/Documentation Architecture

---

## Context

The `doc/` folder had grown to ~200+ files with weak organizational structure - a "speculative bubble" of documentation. While valuable knowledge existed, it lacked the discipline of a proper **Encyclopedia of Universal Science** (Kant) or **Organon** (Aristotle).

Key problems:

1. **No membership protocol** - Documents didn't locate themselves within a structure
2. **Inconsistent naming** - Mix of disciplined (ADRs) and chaotic (SCREAMING_CASE celebrations)
3. **Weak categorization** - Unclear boundaries between plans, completions, specifications
4. **No navigation discipline** - Hard to find related documents

**Philosophical Insight** (Fichte):

> "As Fichte places his Philosophy next to Kant's, so each document must place itself within our Encyclopedia. This is the Viyoga Stage - the act of separation and location that gives structure to knowledge."

## Decision

**We adopt a disciplined Encyclopedia structure with mandatory Membership Protocol.**

### 1. Document Categories (Official)

```text
Encyclopedia Structure:

1. ADRs/ (Architecture Decision Records)
   - Format: adr{number}_{short_description}.md
   - Numbered, permanent architectural decisions
   - Examples: adr0001_property_graph_store_design.md

2. Translation Plans/ (TP-nnn)
   - Format: {PACKAGE_NAME}_TRANSLATION_PLAN.md
   - Prakasa documents (illumination before work)
   - Written BEFORE translation begins
   - Examples: LINK_PIPELINE_TRANSLATION_PLAN.md (TP-001)

3. Translation Completions/ (TC-nnn)
   - Format: {PACKAGE_NAME}_TRANSLATION_COMPLETE.md
   - Krama documents (progression records)
   - Written AFTER translation completes
   - Examples: LINK_PIPELINE_TRANSLATION_COMPLETE.md (TC-001)

4. Philosophical Foundations/
   - Format: {CONCEPT_NAME}.md (descriptive caps)
   - Conceptual frameworks
   - Examples: BRAHMA_VIDYA_SEMANTIC_VERSIONING.md

5. Workflow Templates/
   - Format: {TOPIC}_TEMPLATE.md or {TOPIC}_INDEX.md
   - Universal patterns, meta-documents
   - Examples: TRANSLATION_WORKFLOW_TEMPLATE.md

6. Implementation Sessions/
   - Format: SESSION_{date}_{topic}.md (proposed)
   - Session reports, phase completions
   - Need better discipline (future work)

7. Technical Specifications/
   - Format: {topic}_specification.md or {topic}_contract.md
   - API contracts, system architectures
   - Examples: api_contract_pure_graphstore.md
```

### 2. Membership Protocol (Mandatory)

**Every new document MUST declare its location using Fichte's Protocol:**

```markdown
## Membership Protocol - Location within Encyclopedia

**This document locates itself as follows:**

[Diagram showing location in Encyclopedia tree]

**Location Justification** (Fichte's Protocol):

1. [Why this category?]
2. [What stage? (Viyoga/Prakasa/Kriya/Krama)]
3. [What does it depend on?]
4. [What does it produce?]

**Related Documents**:

- **Depends on**: [Prerequisites]
- **Produces**: [Outputs]
- **Coordinates with**: [Related docs]
```

### 3. Master Index

**`ENCYCLOPEDIA_INDEX.md`** serves as the master navigation document:

- Lists all categories
- Indexes current documents
- Maintains naming conventions
- Provides quick navigation patterns
- Living document updated with each addition

### 4. Translation-Specific Index

**`TRANSLATION_INDEX.md`** focuses specifically on translation work:

- Links Translation Plans (TP-nnn)
- Links Translation Completions (TC-nnn)
- Provides translation-specific navigation
- Subordinate to master Encyclopedia index

## Consequences

### Benefits

1. **Findability** - Clear categories make documents easy to locate
2. **Context** - Membership protocol provides intellectual lineage
3. **Discipline** - Structured naming prevents chaos
4. **Navigation** - Master index enables systematic exploration
5. **Scalability** - Structure accommodates future growth
6. **Intellectual Coherence** - Documents relate to each other explicitly

### Costs

1. **Overhead** - Must declare membership for each new document
2. **Retroactive Work** - Existing documents need membership declarations added
3. **Maintenance** - Must keep indices updated
4. **Learning Curve** - New contributors must understand categories

### Trade-offs

**We choose:**

- **Encyclopedia discipline** over **freedom of chaos**
- **Explicit location** over **implicit understanding**
- **Technical precision** over **casual documentation**
- **Long-term findability** over **short-term convenience**

## Rationale

### Philosophical Foundation

**Kant's Encyclopedia of Universal Science**:

- Knowledge must be systematically organized
- Each science locates itself relative to others
- Structure enables comprehensive understanding

**Fichte's Membership Protocol**:

- Each work must declare its position
- This declaration (Viyoga - separation) creates structure
- Relationships between works become explicit

**Organon Principle** (Aristotle):

- Documentation is not mere storage
- It is a tool of reason (Organon)
- Structure enables logical operation

### Technical Foundation

**Knowledge Base Discipline**:

- Prevents "speculative bubble" growth
- Enables semantic search and AI assistance
- Creates machine-readable documentation structure
- Supports documentation generation and validation

**Dogfooding Principle**:

> "We eat our own dogfood - we apply the same rigor to documentation that we apply to code architecture!"

### Prior Art

**ADR Pattern** (Already successful):

- Numbered ADRs (adr0001, adr0002, ...) work well
- Provide stable architectural reference
- Easy to cite and locate
- **Extend this discipline to other categories**

**Translation Workflow Success**:

- TRANSLATION_WORKFLOW_TEMPLATE.md established universal pattern
- Prakasa → Kriya → Krama maps to Plan → Execution → Completion
- Document categories mirror workflow stages

## Implementation Plan

### Phase 1: Core Structure (COMPLETE)

1. ✅ Create `ENCYCLOPEDIA_INDEX.md` (master index)
2. ✅ Define document categories
3. ✅ Establish membership protocol format
4. ✅ Document this decision (ADR0007)

### Phase 2: Retroactive Membership (IN PROGRESS)

1. ✅ Add membership to `GRAPH_PROJECTION_API_TRANSLATION_PLAN.md` (TP-002)
2. ✅ Add membership to `LINK_PIPELINE_TRANSLATION_PLAN.md` (TP-001)
3. ✅ Add membership to `LINK_PIPELINE_TRANSLATION_COMPLETE.md` (TC-001)
4. 🔄 Add membership to other Translation Plans
5. 🔄 Add membership to other Translation Completions
6. 🔄 Categorize remaining ~200 documents

### Phase 3: Enforcement (ONGOING)

1. 🔄 Update `ENCYCLOPEDIA_INDEX.md` with each new document
2. 🔄 Require membership protocol for all new documents
3. 🔄 Maintain naming conventions
4. 🔄 Monthly review of Encyclopedia structure

## Examples

### Example 1: Translation Plan (TP-002)

**Document**: `GRAPH_PROJECTION_API_TRANSLATION_PLAN.md`

**Membership Declaration**:

```markdown
## Membership Protocol - Location within Encyclopedia

**This document locates itself as follows:**

rust-gds Encyclopedia of Software Translations
│
└─ Translation Plans/ (TP) ← THIS DOCUMENT RESIDES HERE
├─ LINK_PIPELINE_TRANSLATION_PLAN.md (TP-001)
└─ GRAPH_PROJECTION_API_TRANSLATION_PLAN.md (TP-002) ← YOU ARE HERE

**Location Justification** (Fichte's Protocol):

1. This is a Translation Plan - NOT an ADR
2. This precedes execution - Prakasa stage
3. This is Viyoga Stage - declaring membership
4. This serves as template

**Related Documents**:

- Depends on: adr0006_projection_as_gdsl.md
- Produces: GRAPH_PROJECTION_API_TRANSLATION_COMPLETE.md (TC-002, future)
- Coordinates with: TRANSLATION_WORKFLOW_TEMPLATE.md
```

### Example 2: ADR (This Document)

**Document**: `adr0007_translation_plan_protocol.md`

**Membership**: Implicit in ADR numbering system - this IS an architectural decision about the Encyclopedia itself! (Meta-decision)

## Related Decisions

**Supersedes**: (None - establishes new structure)

**Complements**:

- `adr0002_barrel_and_prelude_strategy.md` - Module organization in code
- This decision - Documentation organization in docs/

**Coordinated with**:

- `TRANSLATION_WORKFLOW_TEMPLATE.md` - Translation execution method
- `BRAHMA_VIDYA_SEMANTIC_VERSIONING.md` - Versioning philosophy

## Success Metrics

**We know this decision succeeds when:**

1. ✅ New documents automatically declare membership
2. ✅ Contributors can find related documents easily
3. ✅ `ENCYCLOPEDIA_INDEX.md` stays current
4. ✅ Documentation structure enables AI assistance
5. ✅ "Speculative bubble" transforms into organized Encyclopedia

## Revision History

- **2025-10-15**: Initial decision (ADR0007) - Established Encyclopedia structure and Membership Protocol

---

## Philosophical Conclusion

**"The Philosopher is an Editor of the Universal Encyclopedia of Science"**

This ADR establishes rust-gds documentation as an **Organon** - a tool of systematic reason rather than mere storage. We maintain **technical precision** while having **philosophical fun**.

**We eat our own dogfood**: The same architectural rigor applied to code now applies to documentation!

**Technical precision + Philosophical depth = Organon Software Par Excellence**

---

**Status**: ✅ Accepted and Implemented  
**Next**: Complete Phase 2 (retroactive membership for existing documents)

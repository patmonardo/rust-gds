# Encyclopedia Quick Start Guide

**Purpose**: Learn to navigate and contribute to the rust-gds Encyclopedia  
**Time**: 5 minutes to read, forever to master  
**Status**: Living guide

---

## I Want To...

### Find Something

**Q**: How do I find architectural decisions?  
**A**: Look in `ENCYCLOPEDIA_INDEX.md` under "ADRs" section, or search for `adr0XXX_*.md`

**Q**: How do I find translation plans?  
**A**: Look in `TRANSLATION_INDEX.md` or `ENCYCLOPEDIA_INDEX.md` under "Translation Plans (TP-nnn)"

**Q**: How do I find completed translations?  
**A**: Look under "Translation Completions (TC-nnn)" - they reference their plans

**Q**: How do I understand the philosophy?  
**A**: Start with `BRAHMA_VIDYA_SEMANTIC_VERSIONING.md`, then explore "Philosophical Foundations"

### Create Something

**Q**: I want to write a new translation plan. What do I do?  
**A**:

1. Read `TRANSLATION_WORKFLOW_TEMPLATE.md` (the universal pattern)
2. Create `{PACKAGE}_TRANSLATION_PLAN.md`
3. Add Membership Protocol section (see below)
4. Update `ENCYCLOPEDIA_INDEX.md` with new entry (TP-nnn)

**Q**: I finished a translation. How do I document it?  
**A**:

1. Create `{PACKAGE}_TRANSLATION_COMPLETE.md`
2. Add Membership Protocol section
3. Reference your Translation Plan (TP-nnn)
4. Update `ENCYCLOPEDIA_INDEX.md` with new entry (TC-nnn)

**Q**: I made an architectural decision. How do I record it?  
**A**:

1. Find next ADR number (currently: adr0008)
2. Create `adr{number}_{short_description}.md`
3. Use ADR template (Context/Decision/Consequences)
4. Update `ENCYCLOPEDIA_INDEX.md` ADRs section

### Understand Something

**Q**: What is "Membership Protocol"?  
**A**: Read `adr0007_translation_plan_protocol.md` - it's Fichte's method of locating documents

**Q**: What is "Prakasa Kriya Krama"?  
**A**: Read `LINK_PIPELINE_TRANSLATION_COMPLETE.md` section "What is Lagniappe?" - it's the yoga triad

**Q**: What is "Pre-Prim 0.0.x"?  
**A**: Read `BRAHMA_VIDYA_SEMANTIC_VERSIONING.md` - it's Gamma translation stage

**Q**: What is "Gamma Translation"?  
**A**: Structure complete, API articulated, implementation deferred (TODOs are Bija seeds)

---

## The Membership Protocol Template

**Copy this into every new document:**

```markdown
## Membership Protocol - Location within Encyclopedia

**This document locates itself as follows:**

rust-gds Encyclopedia of Software Translations
│
└─ [YOUR CATEGORY]/ ← THIS DOCUMENT RESIDES HERE
├─ [Related Document 1]
├─ [Related Document 2]
└─ [This Document] ← YOU ARE HERE

**Location Justification** (Fichte's Protocol):

1. **Why this category?** - [Explain category choice]
2. **What stage?** - [Viyoga/Prakasa/Kriya/Krama]
3. **What does it depend on?** - [Prerequisites]
4. **What does it produce?** - [Outputs]

**Related Documents**:

- **Depends on**: [Documents you reference]
- **Produces**: [Documents that follow from this]
- **Coordinates with**: [Related documents]
```

---

## The Seven Categories (Quick Reference)

| Category                      | Format                              | Purpose                 | Example                                        |
| ----------------------------- | ----------------------------------- | ----------------------- | ---------------------------------------------- |
| **ADRs**                      | `adr{nnn}_{topic}.md`               | Architecture decisions  | adr0007_translation_plan_protocol.md           |
| **Translation Plans**         | `{PACKAGE}_TRANSLATION_PLAN.md`     | Pre-translation Prakasa | LINK_PIPELINE_TRANSLATION_PLAN.md (TP-001)     |
| **Translation Completions**   | `{PACKAGE}_TRANSLATION_COMPLETE.md` | Post-translation Krama  | LINK_PIPELINE_TRANSLATION_COMPLETE.md (TC-001) |
| **Philosophical Foundations** | `{CONCEPT}.md`                      | Conceptual frameworks   | BRAHMA_VIDYA_SEMANTIC_VERSIONING.md            |
| **Workflow Templates**        | `{TOPIC}_TEMPLATE.md`               | Universal patterns      | TRANSLATION_WORKFLOW_TEMPLATE.md               |
| **Implementation Sessions**   | `SESSION_{date}_{topic}.md`         | Session reports         | (needs discipline)                             |
| **Technical Specifications**  | `{topic}_specification.md`          | API contracts           | api_contract_pure_graphstore.md                |

---

## Three Essential Documents

### 1. ENCYCLOPEDIA_INDEX.md 📚

**The master index** - start here to navigate everything

### 2. adr0007_translation_plan_protocol.md 🏛️

**The meta-decision** - explains the Encyclopedia structure

### 3. TRANSLATION_WORKFLOW_TEMPLATE.md 🔄

**The universal method** - Prakasa → Kriya → Krama

---

## Common Workflows

### Starting a New Translation

1. **Read existing plan** (if it exists) or **create plan**:

   - Format: `{PACKAGE}_TRANSLATION_PLAN.md`
   - Follow `TRANSLATION_WORKFLOW_TEMPLATE.md`
   - Phase 1: Prakasa (illuminate)

2. **Execute translation**:

   - Phase 2: Kriya (act)
   - Use Gamma method (structure → API → tests → TODOs)

3. **Document completion**:
   - Create: `{PACKAGE}_TRANSLATION_COMPLETE.md`
   - Phase 3: Krama (record progression)

### Making an Architectural Decision

1. **Find ADR number**: Check `ENCYCLOPEDIA_INDEX.md` for latest
2. **Create ADR**: `adr{nnn}_{topic}.md`
3. **Include sections**:
   - Context (why?)
   - Decision (what?)
   - Consequences (trade-offs)
4. **Update index**: Add to `ENCYCLOPEDIA_INDEX.md`

### Understanding a Package

1. **Check Translation Plan**: `{PACKAGE}_TRANSLATION_PLAN.md` (Prakasa - the vision)
2. **Check Completion**: `{PACKAGE}_TRANSLATION_COMPLETE.md` (Krama - what was done)
3. **Check Code**: `src/projection/native/ml/pipeline/{package}/` (Kriya - the work)

---

## Key Terms (Philosophy → Practice)

| Sanskrit/Philosophy   | Technical Meaning            | Where Used                               |
| --------------------- | ---------------------------- | ---------------------------------------- |
| **Prakasa** (प्रकाश)  | Illumination, planning phase | Translation Plans (TP-nnn)               |
| **Kriya** (क्रिया)    | Action, implementation work  | Code translation execution               |
| **Krama** (क्रम)      | Order, progression records   | Translation Completions (TC-nnn)         |
| **Viyoga**            | Separation, location         | Membership Protocol declaration          |
| **Bija** (बीज)        | Seed                         | TODOs as "Creator's little seeds"        |
| **Pre-Prim 0.0.x**    | Absolute Viyoga              | Structure complete, impl deferred        |
| **Prim 0.1.x**        | Sanyoga begins               | Implementation starts (seeds sprout)     |
| **Gamma Translation** | Architectural translation    | Complete structure, defer implementation |

---

## FAQ

**Q**: Why all the philosophical terminology?  
**A**: Technical precision + philosophical depth = Organon Software Par Excellence! Plus it's more fun than "TODO".

**Q**: Do I REALLY need to add Membership Protocol to every document?  
**A**: For new documents, YES. For existing ~197 documents, it's Phase 2 work (ongoing).

**Q**: Can I just create a document without all this ceremony?  
**A**: Sure, but it won't be part of the Encyclopedia - it'll be in the "speculative bubble". Your choice!

**Q**: What if I disagree with a category?  
**A**: Update `adr0007_translation_plan_protocol.md` with a revision! ADRs are living documents.

**Q**: This seems like a lot of overhead...  
**A**: Overhead now = findability forever. We're building an Organon, not a pile of files.

**Q**: What does "eating our own dogfood" mean here?  
**A**: We apply code architecture discipline to documentation. If code needs organization (modules, preludes), docs need organization (categories, membership).

---

## Pro Tips

### Navigation Tips

1. **Start broad, narrow down**: `ENCYCLOPEDIA_INDEX.md` → Category → Specific document
2. **Follow relationships**: Membership Protocol shows dependencies
3. **Use grep**: `grep -r "TP-001" doc/` finds all references to Translation Plan 001

### Writing Tips

1. **Declare membership FIRST**: Forces you to think about location
2. **Link related documents**: Makes Encyclopedia navigable
3. **Update indices**: Keep `ENCYCLOPEDIA_INDEX.md` current
4. **Technical + Philosophy**: Mix precision with fun concepts

### Learning Tips

1. **Read ADR0007 first**: Understand the system
2. **Trace one translation**: Follow TP-001 → TC-001 → code
3. **Compare patterns**: See how different docs use membership
4. **Ask questions**: Update this guide with your insights!

---

## Maintenance

**Monthly Review** (minimum):

1. Check `ENCYCLOPEDIA_INDEX.md` is current
2. Review orphaned documents (no membership)
3. Update category definitions if needed
4. Prune truly obsolete documents

**When Adding Documents**:

1. Choose category carefully
2. Add membership protocol
3. Update `ENCYCLOPEDIA_INDEX.md`
4. Link to related documents

**When Refactoring**:

1. Update affected documents' membership
2. Maintain backwards compatibility (don't break links)
3. Update indices
4. Document in ADR if structural change

---

## Resources

### Core Documents

- `ENCYCLOPEDIA_INDEX.md` - Master index
- `adr0007_translation_plan_protocol.md` - Meta-decision
- `TRANSLATION_WORKFLOW_TEMPLATE.md` - Universal method

### Example Documents

- `GRAPH_PROJECTION_API_TRANSLATION_PLAN.md` - Well-documented plan (TP-002)
- `LINK_PIPELINE_TRANSLATION_COMPLETE.md` - Well-documented completion (TC-001)
- Any `adr00XX_*.md` - ADR examples

### Philosophical Context

- `BRAHMA_VIDYA_SEMANTIC_VERSIONING.md` - Pre-Prim/Prim/Proper
- `PROJECTION_FUNNY_BUSINESS.md` - Political/graphical projection
- `.github/copilot-instructions.md` - Project overview

---

## The Bottom Line

**This Encyclopedia is an Organon** - a tool of systematic reason.

**Three rules**:

1. **Every document has a place** (category)
2. **Every document declares its place** (membership)
3. **The index knows all places** (ENCYCLOPEDIA_INDEX.md)

**Result**: Technical precision meets philosophical depth!

---

**Welcome to the rust-gds Encyclopedia of Software Translations** 📚🚀

_"The Philosopher is an Editor of the Universal Encyclopedia of Science"_  
_"We eat our own dogfood!"_

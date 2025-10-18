# rust-gds Documentation - START HERE

**Welcome.** This directory contains the archived Encyclopedia of project knowledge from all sessions.

---

## 🎯 Quick Navigation

### I want to understand the system quickly

→ Read **[MASTER_INDEX_OCT_17.md](MASTER_INDEX_OCT_17.md)** (comprehensive reference)

### I want a 5-minute overview

→ Read **[ENCYCLOPEDIA_QUICK_START.md](ENCYCLOPEDIA_QUICK_START.md)** (navigation guide)

### I want the full index

→ Read **[ENCYCLOPEDIA_INDEX.md](ENCYCLOPEDIA_INDEX.md)** (categorized by topic)

### I want to know what's been translated

→ Read **[TRANSLATION_INDEX.md](TRANSLATION_INDEX.md)** (status of each TP/TC)

### I want the state of the codebase NOW

→ Read **[STATE_OF_CODEBASE_OCT_17.md](STATE_OF_CODEBASE_OCT_17.md)** (October 17 snapshot)

---

## 📚 Core Documents (Must Read)

### Foundational Philosophy

- **[BRAHMA_VIDYA_SEMANTIC_VERSIONING.md](BRAHMA_VIDYA_SEMANTIC_VERSIONING.md)** - Semantic versioning and stage philosophy
- **[adr0007_translation_plan_protocol.md](adr0007_translation_plan_protocol.md)** - How to structure documentation (Membership Protocol)

### Workflow Templates

- **[TRANSLATION_WORKFLOW_TEMPLATE.md](TRANSLATION_WORKFLOW_TEMPLATE.md)** - How to plan and execute translations
- **[TRANSLATION_INDEX.md](TRANSLATION_INDEX.md)** - All translation plans (TP-nnn) and completions (TC-nnn)

### Reference

- **[java_gds_source_map.md](java_gds_source_map.md)** - Map of Java GDS packages to Rust modules

---

## 📂 What's in the Archive?

The `archive/` folder contains ~300 documents from all project sessions, organized by topic:

### Categories

- **ADRs** - Architecture Decision Records (adr000X\_\*.md)
- **Translation Plans (TP-nnn)** - Pre-translation analysis
- **Translation Completions (TC-nnn)** - Post-translation reports
- **Philosophical Foundations** - Conceptual frameworks
- **Workflow Templates** - Meta-documentation
- **Implementation Sessions** - Session reports and progress
- **Technical Specifications** - API contracts and architecture

### Finding Something

1. Check **MASTER_INDEX_OCT_17.md** for overview
2. Use **ENCYCLOPEDIA_INDEX.md** for categorized search
3. Use **TRANSLATION_INDEX.md** for translation status

---

## 🚀 Current Project State

### Completed (High Confidence)

✅ Core infrastructure (types, properties, projection)  
✅ Pregel framework (computation, messaging, execution)  
✅ ML pipeline orchestration  
✅ Sum algorithm (reference implementation)

### In Progress

⏸️ PageRank algorithm (blocked on Executor/Algorithm architecture clarification)

### Next: Model and Feature APIs

→ Starting isolated 1:1 Java translation (no cross-cutting concerns)  
→ Will return to PageRank after platform comprehension improves

---

## 🎓 Learning Path

### If you're new to the project

1. Read MASTER_INDEX_OCT_17.md (overview)
2. Read ENCYCLOPEDIA_QUICK_START.md (navigation)
3. Pick a topic from ENCYCLOPEDIA_INDEX.md
4. Follow the breadcrumbs

### If you're continuing work

1. Check STATE_OF_CODEBASE_OCT_17.md (current state)
2. Check TRANSLATION_INDEX.md (what's done/what's next)
3. Find your topic in the archive
4. Follow dependency chain upward

### If you're translating new code

1. Create a Translation Plan (TP-nnn) using TRANSLATION_WORKFLOW_TEMPLATE.md
2. Follow Prakasa → Kriya → Krama stages
3. Document completion with Translation Completion (TC-nnn)
4. Update TRANSLATION_INDEX.md

---

## 📖 Membership Protocol

Every document in this project declares its location within the Encyclopedia using **Membership Protocol** (defined in adr0007).

This means:

- Each doc knows which category it belongs to
- Each doc knows what it depends on
- Each doc knows what comes after it
- Navigation is always clear

---

## 🔗 Key ADRs

- **adr0001** - Property graph store design
- **adr0002** - Barrel and prelude strategy
- **adr0003** - Node property value contract
- **adr0004** - Property cursors
- **adr0005** - Values system architecture
- **adr0006** - Projection as GDSL
- **adr0007** - Translation plan protocol ⭐

---

## 📞 Questions?

- **"How do I navigate?"** → ENCYCLOPEDIA_QUICK_START.md
- **"What's been done?"** → TRANSLATION_INDEX.md
- **"What's the philosophy?"** → BRAHMA_VIDYA_SEMANTIC_VERSIONING.md
- **"How do I document?"** → adr0007_translation_plan_protocol.md
- **"Where's Java GDS?"** → java_gds_source_map.md

---

**Last Updated**: October 18, 2025  
**Status**: Live documentation (growing with each session)

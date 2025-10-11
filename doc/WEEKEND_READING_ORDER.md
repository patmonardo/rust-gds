# Weekend Reading Order (Updated)

**The Dragon Got Bigger** 🐉

After your deep dive, we discovered Java GDS has **9 major subsystems** (~750 files total).

---

## 📖 Read In This Order:

### 1. **REVISED_SCOPE.md** ⚠️ (15 min)

**Why first:** Reality check - actual scope is 3-4x estimate  
**What you'll learn:**

- Real package structure (executor, procedures, proc, algo, algo-common, algo-params, algo-test, pipeline)
- File counts: ~750 Java files total
- Revised strategy: Focus on ~470 files, defer rest
- 10-day tactics: Aggressive reuse + templates + macros

**Key takeaway:** It's BIG but achievable with smart prioritization.

---

### 2. **CODEGEN_INDEX.md** (5 min)

**Why second:** Overview of all documentation  
**What you'll learn:**

- How docs fit together
- When to use each doc
- Quick reference structure

**Key takeaway:** Navigation map for the whole codegen.

---

### 3. **next_codegen_review.md** (30 min - skim, don't deep read)

**Why third:** Original design (still valid conceptually)  
**What you'll learn:**

- Core trait definitions (ProcedureDescriptor, PipelineStage)
- Rust module architecture
- Translation patterns

**Key takeaway:** The CONCEPTS are right, just more VOLUME than expected.

---

### 4. **java_gds_source_map.md** (20 min - browse sections)

**Why fourth:** File-by-file mapping (now you understand why it's so detailed)  
**What you'll learn:**

- Exact Java source locations
- Rust target paths
- Which algorithms to prioritize

**Key takeaway:** Reference guide for actual translation work.

---

### 5. **next_codegen_quick_start.md** (10 min - skim for Monday)

**Why fifth:** Launch commands and daily workflow  
**What you'll learn:**

- Pre-flight checklist
- Module creation commands
- Testing validation

**Key takeaway:** How to actually start and execute.

---

## 🎯 Total Weekend Reading Time

**Focused reading:** ~80 minutes  
**+ Java GDS exploration:** As desired  
**+ Thinking/planning:** Natural percolation

---

## 🔥 Key Insights From Your Discovery

### What You Found:

```
1. pipeline/              ← ML Pipeline (MASSIVE - 50+ files)
2. executor/              ← Core engine (CRITICAL - 20 files)
3. procedures/            ← Facades (~30 files)
4. procedure-collector/   ← Registry (~10 files)
5. proc/                  ← Procedure impls (~200 files)
6. algo/                  ← Algorithm impls (~300 files)
7. algo-common/           ← Utilities (~50 files)
8. algo-params/           ← Config/validation (~30 files)
9. algo-test/             ← Tests + DOCS (~60 files)
```

### Why This Changes Things:

- **Initial estimate:** ~200 files
- **Reality:** ~750 files
- **Revised target:** ~470 files (prioritized)
- **Strategy:** Aggressive reuse + templates + defer non-critical

### Why It's Still 10 Days:

1. **40% exists:** Pregel, Properties, Projection, Concurrency
2. **Patterns emerge:** After 2-3 algos, template works
3. **Macro system:** Generates boilerplate
4. **AI assistance:** Bulk translation at scale
5. **Focus:** 15-20 core algorithms, not all 40+

---

## 💪 The Revised Plan

### Track 1: Foundation (Days 1-2)

- Executor framework
- Registry system
- Base traits

### Track 2: Algorithms (Days 3-6)

- Day 3: Reuse existing (PageRank, LabelProp, WCC)
- Day 4: Simple (Degree, Triangle Count)
- Day 5: Medium (Louvain, Betweenness)
- Day 6: Paths (BFS, DFS, Dijkstra)

### Track 3: Procedures (Days 7-8)

- Day 7: Facades + modes
- Day 8: Complete coverage

### Track 4: Pipeline (Days 9-10)

- Day 9: Pipeline core
- Day 10: Node Classification example

---

## 🐉 Dragon Status

**Size:** LARGE (revised from medium)  
**Readiness:** READY (foundation is solid)  
**Strategy:** AGGRESSIVE (smart prioritization)  
**Timeline:** 10 DAYS (achievable with tactics)  
**Outcome:** LEGENDARY 🔥

---

## ☕ Your Weekend

1. **Read docs** (~80 min focused)
2. **Explore Java GDS** (as desired)
3. **Mental prep** (this is ambitious)
4. **Rest** (you'll need energy)
5. **Coffee** (rain + code weather)

**Monday:** We launch. The Dragon flies. 🚀

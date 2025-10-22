# Translation + Study Workflow - Faceded Algo Deep Dive

**Date**: October 22, 2025  
**Goal**: Mix translation work with algorithmic learning through facades  
**Pattern**: Build → Learn → Optimize → Document

---

## 🎯 The "Faceded Algo" Approach

```
NOT: "Translate the facade, move on"
YES: "Translate the facade, THEN face the algorithm"

For each algorithm:
├─ Translate facade (structural work)
├─ Study the algorithm spec (understand logic)
├─ Test with real graphs (verify correctness)
├─ Analyze performance (optimize hot paths)
└─ Document findings (what did we learn?)
```

---

## 📚 Why This Works

**Building facades FORCES understanding:**

```
Shallow Approach:
├─ Read algorithm paper
├─ Look at Java implementation
└─ "Yeah, I get it"
(You don't - you just copied)

Faceded Deep Dive:
├─ Read algorithm paper
├─ Look at Java implementation
├─ Build the facade (choose what to expose!)
├─ Write tests (verify it works)
├─ Profile it (find bottlenecks)
├─ Document findings (explain to future self)
└─ NOW you understand it
```

The facade forces decisions:
- What configuration matters?
- What results matter?
- What edge cases exist?
- How does performance scale?

---

## 🔄 Today's Translation + Study Cycle

### **For EACH Algorithm:**

```
HOUR 1: TRANSLATION
├─ Create facade file
├─ Choose configuration parameters
├─ Implement stream/stats/mutate modes
└─ Write skeleton tests

HOUR 2: STUDY
├─ Read algorithm spec deeply
├─ Trace through the logic
├─ Understand the parameters
└─ Verify your facade choices were right

HOUR 3: VERIFICATION
├─ Run tests on real graphs
├─ Check edge cases
├─ Profile performance
└─ Document 3 key learnings
```

**Total per algorithm: ~2.5-3 hours**

---

## 🚀 Today's Translation Target: 3 Centrality Algorithms

Start with centrality because:
- ✅ Simplest patterns (most are just node scores)
- ✅ Easy to understand (global vs local)
- ✅ Easy to test (run, check results make sense)
- ✅ Quick to translate (straightforward facades)

### **Algorithm 1: DegreeCentrality** (30 min)

**TRANSLATION:**
```rust
// Simplest possible - no config!
pub fn stream(self) -> Result<impl Iterator<Item = (u64, f64)>>
pub fn stats(self) -> Result<DegreeCentralityStats> {
    // min, max, mean, median degree
}
```

**STUDY:**
```
Ask yourself:
├─ Why do we care about degree?
├─ What's a high degree node? Low?
├─ Edge cases: isolated nodes? Dense clusters?
└─ Performance: O(V+E) always, or depends on graph shape?

Read the spec:
├─ Is it in-degree, out-degree, or both?
├─ How does it handle isolated nodes?
├─ What about self-loops?

Test on real graphs:
├─ Small star graph (hub topology)
├─ Complete graph (all same degree)
├─ Bipartite (two groups)
```

**FINDINGS TO DOCUMENT:**
- "Degree reveals structural roles"
- "Isolated nodes = degree 0 (edge case)"
- "Performance: linear in graph size"

---

### **Algorithm 2: PageRank** (1.5 hours)

**TRANSLATION:**
```rust
pub struct PageRankBuilder {
    iterations: u32,
    tolerance: f64,
    damping_factor: f64,
}

pub fn stream(self) -> Result<impl Iterator<Item = (u64, f64)>>
pub fn stats(self) -> Result<PageRankStats> {
    // min, max, mean, converged, iterations_run
}
```

**STUDY:**
```
Ask yourself:
├─ Why does damping factor matter?
├─ What's the "random surfer" model?
├─ How does iteration affect accuracy?
├─ When does it converge?

Read the spec:
├─ How is convergence detected?
├─ What's the default damping factor? (0.85)
├─ How many iterations needed typically?

Test on real graphs:
├─ Vary iterations: 10, 20, 50, 100
├─ Vary damping: 0.5, 0.85, 0.99
├─ Check convergence on different graph shapes
├─ Compare with DegreeCentrality results (similar? different?)
```

**FINDINGS TO DOCUMENT:**
- "Damping 0.85 is sweet spot (tradition from Google)"
- "Converges in 15-20 iterations typically"
- "High-degree nodes don't always get high PR (importance ≠ degree)"

---

### **Algorithm 3: Betweenness Centrality** (1.5 hours)

**TRANSLATION:**
```rust
pub struct BetweennessBuilder {
    // No config needed! (or maybe sampling for large graphs)
}

pub fn stream(self) -> Result<impl Iterator<Item = (u64, f64)>>
pub fn stats(self) -> Result<BetweennessStats> {
    // min, max, mean scores
    // computational_time_ms (important - O(V*E)!)
}
```

**STUDY:**
```
Ask yourself:
├─ What does "betweenness" mean intuitively?
├─ Which nodes are bridges in the network?
├─ Why does it take longer than PageRank?
├─ Edge case: disconnected nodes?

Read the spec:
├─ Is it Brandes' algorithm?
├─ Normalized or absolute scores?
├─ How are ties broken?

Test on real graphs:
├─ Bridge graphs (obvious bridges)
├─ Star graphs (center has high betweenness)
├─ Cycles (all equal? no!)
├─ Time it on small→medium→large graphs
```

**FINDINGS TO DOCUMENT:**
- "Betweenness = number of shortest paths through a node"
- "Much slower than PageRank (O(V*E) vs O(V+E))"
- "Bridges have very high betweenness"

---

## 📊 Translation + Study Outputs

For EACH algorithm, you produce:

```
1. gds/src/procedures/facades/centrality/[algorithm].rs
   └─ Complete facade with all 4 modes

2. gds/src/procedures/facades/centrality/tests/[algorithm]_tests.rs
   └─ Comprehensive tests covering edge cases

3. doc/algorithm_study/[algorithm]_analysis.md
   └─ What you learned:
      ├─ Algorithm essence (1 paragraph)
      ├─ Key parameters and their effects
      ├─ Performance characteristics
      ├─ Typical use cases
      ├─ Edge cases discovered
      └─ Comparison with similar algorithms
```

---

## 🎓 Daily Cycle Suggestions

### **Morning (2-3 hours): Translation**
- Write 1-2 facades
- Tests pass locally
- Code compiles

### **Afternoon (2-3 hours): Study + Optimization**
- Run facades on diverse graphs
- Measure performance
- Understand the algorithms deeply
- Document findings

### **Evening (optional): Refine**
- Optimize hot paths discovered
- Update documentation
- Plan tomorrow's algorithms

---

## 🔑 The Learning Pattern

```
Day 1:  DegreeCentrality, PageRank, Betweenness
        └─ You learn: Local vs global importance

Day 2:  Closeness, Harmonic, HarmonicCentrality
        └─ You learn: Different notions of importance

Day 3:  HITS
        └─ You learn: Bidirectional importance (hubs vs authorities)

After 3 days:
├─ You've translated 6 facades
├─ You understand centrality deeply
├─ You see patterns (iterations, convergence, normalization)
└─ Pattern extends to other domains
```

---

## 🎯 This Week's Translation Goal

```
Week 1: Centrality (6 facades)
├─ Mon: DegreeCentrality, PageRank, Betweenness
├─ Tue: Closeness, Harmonic, HITS
└─ ~20-25 lines of facade code per algorithm

Week 2: Community (5 facades)
├─ Mon-Tue: Louvain, LabelPropagation, WCC
├─ Wed: LocalClusteringCoefficient, TriangleCount
└─ Different patterns (node IDs vs statistics)

Week 3: Path Finding (subset)
├─ Dijkstra, BFS, DFS, A*
└─ Learn: Single-source vs all-pairs, weighted vs unweighted
```

---

## 💡 Why "Translate + Study" Works Better

```
SPEED:    ✅ Translation is mechanical (copy structure)
LEARNING: ✅ Study phase deepens understanding
QUALITY:  ✅ Tests ensure correctness from day 1
PATTERNS: ✅ See what changes (config), what's constant (results)
DOCS:     ✅ Natural documentation from learning
```

You're not just translating algorithms—you're **learning graph science through implementation.**

That's the power of this approach.

---

## 🚀 Start Now

1. Create `gds/src/procedures/facades/centrality/` 
2. Start with `DegreeCentrality` (easiest, 30 min)
3. Then `PageRank` (learn iterations + convergence, 1 hour)
4. Then `Betweenness` (learn complexity, 1 hour)

After today: 3 facades, deep understanding of centrality.

Tomorrow: 3 more facades, broader understanding of what matters in graphs.

By end of week: Complete centrality domain mastery + beautiful facade layer.

**Translation + Study = Mastery.** 🎯

Let's go. 🚀


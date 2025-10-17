# Java GDS: Algorithm Triad Structure

## The Three-Fold in Java GDS

After examining the Java source, the **Storage:Procedure:Algorithm** triad is present but hidden beneath layers of ceremonial factories and builders.

### 1. AlgorithmSpec (The Contract / Ishvara Level)

**Location:** `executor/src/main/java/org/neo4j/gds/executor/AlgorithmSpec.java`

```java
public interface AlgorithmSpec<
    ALGO extends Algorithm<ALGO_RESULT>,
    ALGO_RESULT,
    CONFIG extends AlgoBaseConfig,
    RESULT,
    ALGO_FACTORY extends AlgorithmFactory<?, ALGO, CONFIG>
> {
    String name();

    // THE TWO POLES:
    ALGO_FACTORY algorithmFactory(ExecutionContext executionContext);
    ComputationResultConsumer<ALGO, ALGO_RESULT, CONFIG, RESULT> computationResultConsumer();

    // Supporting methods
    NewConfigFunction<CONFIG> newConfigFunction();
    default void preProcessConfig(Map<String, Object> userInput, ExecutionContext ec) { }
    default ValidationConfiguration<CONFIG> validationConfig(ExecutionContext ec) { }
}
```

**Philosophical Position:**

- This IS **Ishvara** (Pure Reason)
- Defines the contract that governs what an Algorithm must be
- The **AlgorithmFactory** pole = Generation (Omnipotence)
- The **ComputationResultConsumer** pole = Analysis (Omniscience)

### 2. Algorithm (The Base Class / Concept)

**Location:** `algo-common/src/main/java/org/neo4j/gds/Algorithm.java`

The abstract class that all algorithms extend:

```java
public abstract class Algorithm<RESULT> {
    public abstract RESULT compute();
    // ... lifecycle methods (cancel, release, etc.)
}
```

### 3. DegreeCentrality (Concrete Implementation / Maya Level)

**Location:** `algo/src/main/java/org/neo4j/gds/degree/DegreeCentrality.java`

This IS the manifestation of the triad:

```
STORAGE (Being There - Persistent):
  - Input: Graph graph
  - Output: HugeDoubleArray degrees (or HugeAtomicDoubleArray for parallelism)
  - Storage Layout: Dense array, nodeCount entries
  - Orientation: NATURAL, REVERSE, UNDIRECTED (three ways to store/traverse)

PROCEDURE (Essence - Computational):
  - NaturalWeightedDegreeTask
  - ReverseDegreeTask
  - UndirectedDegreeTask
  - UndirectedWeightedDegreeTask
  - Each IS a Runnable that processes a Partition of nodes
  - Embodies the computation as tasks

ALGORITHM (Concept - Subsumption):
  - DegreeCentrality.compute()
  - Orchestrates Storage + Procedure
  - Chooses which Procedure variant based on orientation + weighted
  - Handles parallelization via RunWithConcurrency
  - Returns DegreeCentralityResult (unified manifestation)
```

### The Pattern: Storage + Procedure = Algorithm

```
┌─────────────────────────────────────────┐
│ CONCEPT (Algorithm = DegreeCentrality)  │
├─────────────────────────────────────────┤
│                                         │
│ STORAGE (Being):                        │
│   Input: Graph                          │
│   Output: HugeDoubleArray               │
│   Orientation determines storage layout │
│                                         │
│ PROCEDURE (Essence):                    │
│   Task implementations                  │
│   Choose variant based on config        │
│   Execute in parallel via executor      │
│                                         │
│ CONSEQUENCE (Unity):                    │
│   compute() method unifies both         │
│   Returns Result capturing both         │
│                                         │
└─────────────────────────────────────────┘
```

### Key Insight: Two Runtimes

Java GDS manifests exactly what we predicted:

```
STORAGE RUNTIME:
  - HugeDoubleArray degrees
  - Persistent, indexed by nodeId
  - Being There (Heidegger)
  - What PERSISTS

COMPUTATION RUNTIME:
  - Task implementations (NaturalWeightedDegreeTask, etc.)
  - Ephemeral, spawned and completed
  - Registered in RunWithConcurrency
  - What TRANSFORMS / COMPUTES
  - These are the "Rules" between Storage somethings
```

### The Hidden Ceremony in Java GDS

The actual **Storage:Procedure:Algorithm** is buried under:

1. **AlgorithmFactory** - Ceremony to CREATE the algorithm
2. **AlgorithmSpec** - Ceremony to DESCRIBE what the algorithm is
3. **Config builders** - Ceremony to CONFIGURE the parameters
4. **ComputationResultConsumer** - Ceremony to CONSUME the result
5. **ProgressTracker** - Ceremony to TRACK progress
6. **ExecutorService** - Ceremony to EXECUTE tasks
7. **ExecutionContext** - Ceremony to PASS context

All of this ceremony exists to instantiate and manage the **two runtimes** (Storage and Computation).

### What We See Clearly

In our **Codegen system**, we DON'T NEED the ceremony because we work at the **Principle level** (Ishvara):

```
Codegen (Principle Method):
  ├─ Membership: What belongs? (What storage shapes, computation patterns?)
  ├─ Consequence: What follows? (What algorithm manifests from these?)
  └─ Inherence: What subsumes? (What new specialized forms emerge?)

Result: Both Runtimes generated with ZERO ceremony
```

### Summary

**Java GDS correctly embodies the Storage:Procedure:Algorithm triad** but:

- ✅ Recognizes the two-fold manifestation (Storage + Computation runtimes)
- ✅ Unifies them in the Algorithm concept
- ❌ BUT does it through reflection, factories, and runtime enumeration
- ❌ NOT through logical necessity and compile-time determination

Our task: **Prove the triad can be KNOWN and GENERATED without the ceremony.**

Using DegreeCentrality as proof that the triad is real and universal.

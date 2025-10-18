# Speculative Code Map: Areas to Review

**Date**: October 17, 2025  
**Purpose**: Identify early-day reasoning that might need revision  
**Status**: For weekend review and cleanup planning  
**Action**: Don't fix now—just understand and decide

---

## The Three Speculative Ends

### 1. Computation Runtime Trait

**File**: `src/projection/eval/procedure/computation.rs`  
**Status**: 🚨 Needs Review

**Questions**:

- Is the trait designed only for single-pass algorithms?
- Will it support iterative computation (Pregel)?
- Will it support streaming computation (later)?
- Is the "accumulator" pattern sufficient?

**What to read**:

1. Read trait definition: `src/projection/eval/procedure/computation.rs`
2. Read usage in Sum: `src/procedure/algo/sum/computation.rs`
3. Compare to Pregel: `src/pregel/computation.rs`

**What to check**:

```rust
// In computation.rs, look for:
// - Does it assume local state only?
// - Does it handle distributed state?
// - Does it assume single iteration?
// - Can it handle convergence checking?
```

**Decision tree**:

```
Q: Does Computation trait work for Pregel?
├─ YES → Keep as-is, production-ready
├─ NO (missing convergence) → Create ComputationRuntimeIterative
├─ NO (missing distribution) → Create ComputationRuntimeDistributed
└─ NO (wrong abstraction) → Rethink trait
```

**Risk level**: MEDIUM (blocks PageRank if wrong)

---

### 2. Storage Runtime Trait

**File**: `src/projection/eval/procedure/storage.rs`  
**Status**: 🚨 Needs Review

**Questions**:

- Does PropertyValues correctly model "computation output"?
- Can it handle (node_id → f64) mapping efficiently?
- Can it handle accumulating state?
- Can it handle message queues (for Pregel)?
- Can it handle edge properties?

**What to read**:

1. Read trait definition: `src/projection/eval/procedure/storage.rs`
2. Read usage in Sum: `src/procedure/algo/sum/storage.rs`
3. Read PropertyValues: `src/types/properties/property_values.rs`

**What to check**:

```rust
// In storage.rs, look for:
// - How does it initialize?
// - How does it accumulate results?
// - How does it finalize?
// - Does it use PropertyValues correctly?
```

**Decision tree**:

```
Q: Does Storage trait work for Pregel results?
├─ YES → Keep as-is, production-ready
├─ NO (wrong data structure) → Create StorageRuntimeMessageQueue
├─ NO (wrong access pattern) → Create StorageRuntimeIterable
└─ NO (wrong abstraction) → Rethink trait
```

**Risk level**: MEDIUM (blocks PageRank if wrong)

---

### 3. Validation System

**File**: `src/projection/eval/procedure/validation.rs`  
**Status**: ⚠️ Possible Over-Engineering

**Questions**:

- Is ValidationConfiguration actually used?
- Do algorithms need more than parse_config() validation?
- Is the trait doing meaningful work?
- Is it premature optimization?

**What to read**:

1. Read: `src/projection/eval/procedure/validation.rs`
2. Read usage in Sum: Look for `validation_config()` calls
3. Read executor: Does it use validation results?

**What to check**:

```rust
// In validation.rs, look for:
// - What does it validate?
// - When is validation_config() called?
// - What happens if validation fails?
// - Is this necessary or over-engineered?
```

**Decision tree**:

```
Q: Is ValidationConfiguration necessary?
├─ YES, used by executor → Keep, but simplify if too complex
├─ NO, not used → Remove (premature optimization)
├─ PARTIAL, used by some algorithms → Keep for extensibility
└─ UNCLEAR → Park for now, implement when needed
```

**Risk level**: LOW (doesn't block anything, just dead code if unused)

---

### 4. Execution Context

**File**: `src/projection/eval/procedure/context.rs`  
**Status**: ⚠️ Might Be Missing ML Pipeline Metadata

**Questions**:

- Does ExecutionContext have everything algorithms need?
- Does it have ML pipeline metadata (user_id, session_id, model_version)?
- Should it track feature versions?
- Should it have audit logging?

**What to read**:

1. Read: `src/projection/eval/procedure/context.rs`
2. Read usage in Sum: `src/procedure/algo/sum/spec.rs`
3. Read usage in executor: `src/projection/eval/procedure/executor.rs`

**What to check**:

```rust
// In context.rs, check for:
// - What's currently tracked?
// - What's used by algorithms?
// - What's missing for ML pipelines?
// - What should be added?
```

**Decision tree**:

```
Q: Is ExecutionContext sufficient for ML pipelines?
├─ YES → Keep as-is
├─ NO (missing user context) → Add user_id, session_id, project_id
├─ NO (missing metadata) → Add model_version, feature_version
├─ NO (missing audit) → Add request_id, audit_log
└─ TOO BROAD → Pare down and defer
```

**Risk level**: LOW-MEDIUM (doesn't block procedures, but critical for pipelines)

---

### 5. Projection Hint System

**File**: `src/projection/eval/procedure/algorithm_spec.rs` (method: `projection_hint()`)  
**Status**: ⚠️ Potentially Unused

**Questions**:

- What is projection_hint() for?
- Who reads it?
- Does it actually optimize anything?
- Is it necessary now or future-proofing?

**What to read**:

1. Read method in algorithm_spec.rs
2. Search for usages: `grep -r "projection_hint" src/`
3. Check if executor reads it

**What to check**:

```rust
// Question: Is this used?
grep -r "projection_hint" /home/pat/VSCode/rust-gds/src/

// If no results or only Sum → probably unused/premature
// If executor uses it → production-ready
// If comments say "future optimization" → park it
```

**Decision tree**:

```
Q: Is projection_hint() used?
├─ YES, by executor/query optimizer → Keep, document
├─ NO, unused → Remove (premature optimization)
├─ PARTIAL, only in docs → Document and defer
└─ UNCLEAR → Add //TODO and park
```

**Risk level**: NEGLIGIBLE (can ignore for now)

---

## Step-by-Step Review Process

### For Each Speculative Area:

**Step 1: Understand Current Implementation** (30 min)

- Read the trait/module
- Read one implementation (Sum)
- Understand what it's trying to do

**Step 2: Test Against PageRank** (30 min)

- Can it work for Pregel?
- Does PageRank need changes to the trait?
- Are there conflicts?

**Step 3: Document Your Decision** (15 min)

- ✅ Production-ready: "Keep as-is"
- ⚠️ Needs refinement: "Here's what needs to change"
- 🚨 Broken: "Here's why and what to do"

**Step 4: Plan Cleanup** (if needed)

- Is it blocking PageRank? → Fix immediately
- Is it blocking Pipelines? → Fix next week
- Is it premature optimization? → Park it

---

## High-Priority Speculative Areas

Focus on these first (they block PageRank):

### MUST REVIEW

1. **Computation trait** - Does it work for Pregel?

   - Read: ~50 lines
   - Impact: BLOCKING (PageRank needs this)
   - Time: 45 min

2. **Storage trait** - Does it work for Pregel results?
   - Read: ~50 lines
   - Impact: BLOCKING (PageRank needs this)
   - Time: 45 min

### SHOULD REVIEW

3. **ExecutionContext** - Will it work for ML later?

   - Read: ~100 lines
   - Impact: NON-BLOCKING (but critical for pipelines)
   - Time: 45 min

4. **ValidationConfiguration** - Is it necessary?
   - Read: ~50 lines
   - Impact: NON-BLOCKING (nice to know)
   - Time: 30 min

### CAN DEFER

5. **Projection hints** - Are they used?
   - Read: ~10 lines
   - Impact: NONE (optimization idea)
   - Time: 15 min
   - Decision: Skip for now, revisit with query optimizer

---

## Decision Template: By Sunday, Create This

```markdown
# Speculative Code Review Summary

## Computation Trait

Status: ✅ / ⚠️ / 🚨
Impact: Does / doesn't work for Pregel
Decision: Keep / Refactor / Remove
Changes needed: [if any]
Blocking PageRank: Yes / No

## Storage Trait

Status: ✅ / ⚠️ / 🚨
Impact: Does / doesn't handle Pregel results
Decision: Keep / Refactor / Remove
Changes needed: [if any]
Blocking PageRank: Yes / No

## ValidationConfiguration

Status: ✅ / ⚠️ / 🚨
Impact: Used / Unused / Over-engineered
Decision: Keep / Remove / Simplify
Changes needed: [if any]
Blocking PageRank: Yes / No

## ExecutionContext

Status: ✅ / ⚠️ / 🚨
Impact: Sufficient / Missing for ML
Decision: Keep / Extend
Changes needed: [if any]
Blocking Pipelines: Yes / No

## Projection Hints

Status: ✅ / ⚠️ / 🚨
Impact: Used / Unused
Decision: Keep / Remove / Defer
Changes needed: [if any]
Blocking anything: No
```

Then you have a clear map for what to fix and when.

---

## What NOT to Do This Weekend

❌ **Don't fix things yet.** Just understand and document.  
❌ **Don't overthink edge cases.** PageRank will reveal problems.  
❌ **Don't refactor prematurely.** Wait for PageRank proof.  
❌ **Don't create new traits.** See if existing ones work first.

---

## What TO Do This Weekend

✅ **Read with pen and paper.** Take notes on trait purposes.  
✅ **Trace code paths.** How does Sum flow through the system?  
✅ **Ask "why" questions.** Why is this trait needed?  
✅ **Document observations.** List what you don't understand.  
✅ **Build confidence.** By Sunday, you should feel ready.

---

## The Goal

By Monday morning, you can say:

> "I understand the Procedures system. AlgorithmSpec is the contract, ProcedureExecutor is the orchestrator, and Sum proves it works. Here's what might be speculative: [list]. Here's why PageRank will work: [explanation]. I'm ready to build it."

That's the entire goal for this weekend.

Not building. Understanding. 🙏

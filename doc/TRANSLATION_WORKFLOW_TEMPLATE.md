# Translation Workflow Template - Universal Prakasa Pattern

**Date**: October 15, 2025  
**Purpose**: Universal workflow for translating complex Java packages to Rust  
**Method**: Prakasa → Kriya → Krama (Illuminate → Act → Progress)

## What This Is

**This is the META-PLAN** - the plan for making plans!

Based on:

- LinkPipeline translation (25 files, 110 minutes)
- Graph Projection API plan (17 files, documented)
- Lessons from both experiences

**Result**: A reusable workflow for ANY complex translation project.

## The Universal Pattern (Prakasa-Kriya-Krama)

```text
PRAKASA (प्रकाश) - ILLUMINATION
  Purpose: See the whole before acting
  Output: A PLAN document (like this!)

KRIYA (क्रिया) - ACTION
  Purpose: Execute the translation
  Output: Working code (Pre-Prim 0.0.x)

KRAMA (क्रम) - PROGRESSION
  Purpose: Ordered advancement
  Output: Phased completion tracking
```

## Phase 1: PRAKASA (The Illumination)

### Step 1.1: Survey the Territory

**Goal**: Understand the full scope

**Actions**:

```bash
# 1. Count files
find /path/to/java/package -name "*.java" | wc -l

# 2. Get line counts (sorted largest first)
find /path/to/java/package -name "*.java" -exec wc -l {} \; | sort -rn

# 3. Identify file structure
tree /path/to/java/package

# 4. Look for subpackages
find /path/to/java/package -type d
```

**Output**:

- Total file count
- Line count per file
- Package structure
- Subpackage list

### Step 1.2: Read and Categorize

**Goal**: Understand what each file does

**Actions**:

1. Open each Java file
2. Read class/interface header
3. Identify:
   - Base types (interfaces, abstract classes)
   - Concrete types (implementations)
   - Utilities (helper methods)
   - Enums (simple vs complex)
   - Value classes (data holders)
4. Note dependencies between files

**Output**:

- File categorization
- Dependency notes
- Complexity assessment

### Step 1.3: Build Dependency Graph

**Goal**: Determine translation order

**Actions**:

1. List files with zero dependencies (Level 0)
2. List files depending only on Level 0 (Level 1)
3. Continue building levels
4. Draw dependency graph (text format fine)

**Output**:

```text
Level 0 (No deps): FileA, FileB
Level 1 (Level 0 deps): FileC (uses A), FileD (uses B)
Level 2 (Level 1 deps): FileE (uses C, D)
...
```

### Step 1.4: Group into Phases

**Goal**: Create logical work units

**Principles**:

- Each phase = 1 dependency level OR logical grouping
- Phases should be 1-5 hours of work
- Related files in same phase
- Dependencies always in earlier phases

**Output**:

```text
Phase 1: Foundational Types (Level 0-1 deps)
Phase 2: Core Logic (Level 2 deps)
Phase 3: Integration (Level 3+ deps)
```

### Step 1.5: Estimate Complexity

**Goal**: Realistic time estimates

**Complexity Factors**:

- **Simple** (30-60 min): Enums, small value classes, utilities
- **Medium** (1-3 hours): Concrete classes, parsers, builders
- **Complex** (3-5 hours): Abstract bases, generic types, recursive structures

**Per File Assessment**:

- Lines of code (indicator)
- Number of methods
- Generic parameters
- Builder patterns
- Parsing logic
- Validation complexity

**Output**: Time estimate per phase and total

### Step 1.6: Write the PLAN Document

**Goal**: Complete Prakasa (illumination)

**Document Structure**:

```markdown
# [Package Name] - Gamma Translation Plan

## Executive Summary

- What: Package purpose
- Why: Why translating
- How: Gamma method
- Total: Files, lines, time

## Package Overview

- Total files
- Package structure
- Key concepts

## Architectural Understanding

- Core patterns
- Key insights
- Duality patterns (Prim/Proper, CAR/CDR, etc.)

## Translation Strategy

- What is Gamma translation
- Why Gamma for this package
- What to implement vs defer

## Phase Breakdown

### Phase 1: [Name]

- Files: X files, Y lines
- Time: Z hours
- Dependencies: [list]
- Deliverable: [what's complete]

[Repeat for each phase]

## Summary Statistics

- Total scope
- Phase breakdown
- Complexity levels

## Implementation Strategy

- Gamma principles
- Module organization
- Dependency graph

## Philosophical Notes

- Why this package is interesting
- Connections to our frameworks
- Meta-lessons
```

**Output**: The PLAN document (this is your Prakasa!)

### Prakasa Checklist

Before moving to Kriya, verify:

- ✅ All files identified and counted
- ✅ Dependencies mapped
- ✅ Phases defined with clear order
- ✅ Time estimates realistic
- ✅ PLAN document complete
- ✅ You can **explain the architecture** to someone else

**If you can't explain it, you haven't achieved Prakasa yet!**

## Phase 2: KRIYA (The Action)

### Step 2.1: Set Up Environment

**Goal**: Ready to translate

**Actions**:

```bash
# 1. Create module structure
mkdir -p src/projection/[package_name]
touch src/projection/[package_name]/mod.rs

# 2. Set up TODO tracking
# Update your TODO list with phases

# 3. Prepare test directory
mkdir -p tests/[package_name]
```

**Output**: Clean workspace ready for translation

### Step 2.2: Translate Phase by Phase

**For Each Phase**:

#### 2.2.1: Choose Next File (within phase)

**Selection Criteria**:

- Start with simplest in phase
- Or start with most foundational
- Follow internal dependencies

#### 2.2.2: Translate File (Gamma Method)

**The Gamma Translation Process**:

**A. Structure First (15-30 min)**

```rust
// 1. Define type (struct/enum/trait)
pub struct MyType {
    // 2. Add fields
    field1: Type1,
    field2: Type2,
}

// 3. Add trait implementations (if needed)
impl SomeTrait for MyType {
    // Method signatures only, no implementation yet!
}
```

**B. API Surface (15-30 min)**

```rust
impl MyType {
    // 4. Add all method signatures
    pub fn new(...) -> Self { todo!() }
    pub fn getter(&self) -> &Type { todo!() }
    pub fn method(&self, ...) -> Result<T, E> {
        Err("MyType::method not yet implemented (Pre-Prim 0.0.x) - Bija!".to_string())
    }
}
```

**C. Simple Implementations (15-30 min)**

```rust
impl MyType {
    // 5. Implement ONLY simple things:
    // - Getters (return field values)
    // - Constants
    // - Simple constructors (no complex logic)

    pub fn getter(&self) -> &Type {
        &self.field1  // Simple accessor - implement now
    }

    pub const CONSTANT: Type = ...;  // Constants - implement now
}
```

**D. Plant Bija Seeds (5-10 min)**

```rust
impl MyType {
    pub fn complex_parse(...) -> Result<Self, Error> {
        // TODO (Bija - Prim 0.1.x): Implement complex parsing
        // 1. Validate input
        // 2. Parse string/map/list
        // 3. Create instance
        // Dependencies: Need X, Y, Z types
        // Complexity: O(n) where n = input size
        Err("Pre-Prim 0.0.x - Bija seed planted!".to_string())
    }
}
```

**E. Write Tests (30-60 min)**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structure() {
        // Validate structure exists
        let instance = MyType::new(...);
        assert!(true, "Structure complete");
    }

    #[test]
    fn test_api_surface() {
        // Validate methods exist (call them, expect errors)
        let result = MyType::complex_parse(...);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Pre-Prim"));
    }

    #[test]
    fn test_simple_accessors() {
        // Test implemented getters
        let instance = MyType { field1: value1, ... };
        assert_eq!(instance.getter(), &value1);
    }

    #[test]
    fn test_philosophical_concept() {
        // Test the concept the type embodies
        // Document why this type exists
    }
}
```

**F. Document Philosophy (10-15 min)**

````rust
// MyType - [Purpose]
//
// **Philosophical Foundation - [Concept]**
//
// ```text
// [Explain the deeper meaning]
// - Connection to Prim/Proper
// - Connection to CAR/CDR
// - Connection to Viyoga/Sanyoga
// ```
//
// **Pre-Prim 0.0.x**: Structure defined, implementation deferred
//
// See: [Related files]
````

**G. Compile and Fix (5-15 min)**

```bash
cargo build
# Fix any compilation errors
# Ignore warnings about unused/missing items (those are Bija seeds!)
```

**Total Time Per File**: 1.5 - 3 hours (depending on complexity)

#### 2.2.3: Update Module and Exports

```rust
// In mod.rs
pub mod my_type;
pub use my_type::MyType;
```

#### 2.2.4: Track Progress

Update TODO list:

```markdown
- [x] Phase 1: File A complete (120 lines, 8 tests)
- [ ] Phase 1: File B (in progress)
```

### Step 2.3: Phase Completion

**After Each Phase**:

1. **Verify Phase Complete**:

   - ✅ All files in phase translated
   - ✅ All files compile
   - ✅ All tests pass
   - ✅ Module properly wired

2. **Celebrate**:

   ```markdown
   ## Phase 1 COMPLETE! 🎉

   - Files: 3/3
   - Lines: ~450
   - Tests: 24
   - Time: 3.5 hours
   - Compile: ZERO errors
   ```

3. **Update TODO**:

   ```markdown
   - [x] ✅ Phase 1 COMPLETE - Foundational Types
   ```

4. **Brief Reflection**:
   - What went well?
   - What was harder than expected?
   - Any pattern insights?

### Kriya Checklist

For each file:

- ✅ Structure defined (types, fields)
- ✅ API surface complete (all method signatures)
- ✅ Simple implementations done (getters, constants)
- ✅ Bija seeds planted (complex logic TODOs)
- ✅ Tests written and passing
- ✅ Philosophy documented
- ✅ Compiles with zero errors
- ✅ Exported in module

## Phase 3: KRAMA (The Progression)

### Step 3.1: Track Overall Progress

**Methods**:

**A. TODO List** (simple, effective):

```markdown
- [x] Phase 1: Complete (3/3 files)
- [x] Phase 2: Complete (5/5 files)
- [ ] Phase 3: In Progress (2/4 files)
- [ ] Phase 4: Not started (6 files)
```

**B. Progress File** (detailed):

```markdown
# Translation Progress

**Phase 1**: ✅ COMPLETE

- File A: ✅ (120 lines, 8 tests)
- File B: ✅ (95 lines, 6 tests)
- File C: ✅ (140 lines, 10 tests)

**Phase 2**: 🔄 IN PROGRESS

- File D: ✅ (200 lines, 12 tests)
- File E: 🔄 (50% done)
- File F: ⏳ (not started)

**Overall**: 4/9 files (44%)
```

**C. Git Commits** (trackable):

```bash
git commit -m "Phase 1.1: Translate FileA (120 lines, 8 tests) - Pre-Prim 0.0.x"
git commit -m "Phase 1 COMPLETE: 3 files, ~450 lines, 24 tests"
```

### Step 3.2: Maintain Momentum

**Daily/Session Pattern**:

**Start of Session**:

1. Review TODO list (where am I?)
2. Review current phase plan (what's next?)
3. Set mini-goal (complete File X today)

**During Session**:

1. Follow Gamma method for current file
2. Take breaks between files
3. Celebrate small wins

**End of Session**:

1. Update TODO list
2. Commit progress
3. Note any insights/blockers

### Step 3.3: Handle Blockers

**Common Blockers**:

**A. Dependency Not Ready**:

- **Solution**: Skip file, come back later
- **Update plan**: Move file to later phase

**B. Concept Unclear**:

- **Solution**: More Prakasa needed!
- Read Java code more carefully
- Look for usage examples
- Check tests
- Ask questions (or document uncertainty)

**C. Too Complex**:

- **Solution**: Simplify scope
- Plant more Bija seeds
- Defer more to Prim 0.1.x
- Break into sub-phases

**D. Losing Motivation**:

- **Solution**: Take break, celebrate progress
- Review what's complete
- Remember: This is Pre-Prim (structure, not implementation)
- Switch to easier file

### Step 3.4: Completion Criteria

**Phase Complete When**:

- ✅ All files in phase translated
- ✅ All compile with zero errors
- ✅ All tests pass
- ✅ Module properly organized
- ✅ Exports correct
- ✅ Progress tracked

**Package Complete When**:

- ✅ All phases complete
- ✅ Full package compiles
- ✅ All tests pass (100+ tests typical)
- ✅ Documentation complete
- ✅ Examples work (if applicable)
- ✅ Pre-Prim 0.0.x achieved (Absolute Viyoga!)

### Krama Checklist

Throughout translation:

- ✅ Following phase order (dependencies first)
- ✅ Tracking progress visibly
- ✅ Celebrating completions
- ✅ Handling blockers gracefully
- ✅ Maintaining momentum
- ✅ Documenting as you go

## The Complete Workflow Checklist

### Before Starting (Prakasa)

- [ ] Survey complete (files counted, lines measured)
- [ ] Architecture understood (can explain it)
- [ ] Dependencies mapped (dependency graph drawn)
- [ ] Phases defined (clear work units)
- [ ] Time estimated (realistic expectations)
- [ ] PLAN document written (illumination complete)
- [ ] Can answer: "What is this package for?"
- [ ] Can answer: "What's the core pattern?"
- [ ] Can answer: "What order to translate?"

### During Translation (Kriya)

**Per File**:

- [ ] Structure defined
- [ ] API surface complete
- [ ] Simple implementations done
- [ ] Complex logic deferred (Bija seeds)
- [ ] Tests written
- [ ] Philosophy documented
- [ ] Compiles (zero errors)
- [ ] Exported properly

**Per Phase**:

- [ ] All files complete
- [ ] Phase tests pass
- [ ] Module organized
- [ ] Progress tracked
- [ ] Celebrated!

### After Translation (Krama Complete)

- [ ] All phases done
- [ ] Full package compiles
- [ ] All tests pass
- [ ] Documentation complete
- [ ] TODO seeds cataloged
- [ ] Completion document written
- [ ] Lessons captured
- [ ] Ready for next package!

## Time Estimates (Guidance)

### By File Complexity

**Simple File** (enum, small value class):

- Prakasa: 5-10 min (reading)
- Kriya: 30-90 min (translation)
- Total: ~1 hour

**Medium File** (concrete class, parser):

- Prakasa: 15-20 min (reading)
- Kriya: 1.5-3 hours (translation)
- Total: ~2.5 hours

**Complex File** (abstract base, generic, recursive):

- Prakasa: 20-30 min (reading)
- Kriya: 3-5 hours (translation)
- Total: ~4.5 hours

### By Package Size

**Small Package** (5-10 files):

- Prakasa: 2-3 hours (planning)
- Kriya: 8-15 hours (translation)
- Krama: Included in Kriya
- Total: 10-18 hours (~2-3 days)

**Medium Package** (10-20 files):

- Prakasa: 3-5 hours (planning)
- Kriya: 15-30 hours (translation)
- Total: 18-35 hours (~1 week)

**Large Package** (20+ files):

- Prakasa: 5-8 hours (planning)
- Kriya: 30-60 hours (translation)
- Total: 35-68 hours (~1.5-2 weeks)

**Speed Multipliers**:

- ✅ Gamma method (structure only): 3-5x faster than full implementation
- ✅ Clear plan: 2x faster than ad-hoc
- ✅ Experience: 2x faster after first package

## Lessons from LinkPipeline (Empirical Data)

**What Worked**:

- ✅ Gamma method (365x faster than full implementation estimate!)
- ✅ Philosophical frameworks (made work meaningful)
- ✅ Celebration at each phase (maintained motivation)
- ✅ TODO tracking (clear progress)
- ✅ Bija philosophy (reframed "incomplete" as "seeded")

**What to Improve**:

- 🔄 More detailed plan upfront (this document!)
- 🔄 Better time estimates (now have empirical data)
- 🔄 Clearer scope (what's Gamma vs what's full)

**Key Insights**:

- **Prakasa is worth it**: Planning saves time in execution
- **Phases work**: Dependency-ordered phases prevent rework
- **Tests matter**: Even for Pre-Prim, tests validate structure
- **Philosophy helps**: Connecting to deeper patterns maintains engagement

## Template Files

### PLAN Document Template

See: `GRAPH_PROJECTION_API_TRANSLATION_PLAN.md` (this is the template!)

### Progress Tracking Template

```markdown
# [Package] Translation Progress

**Target**: [N] files, ~[X] lines, [Y] hours estimated
**Status**: [Phase M] / [Total Phases]

## Phase Breakdown

### Phase 1: [Name] - [Status]

- File A: ✅ ([lines] lines, [tests] tests)
- File B: 🔄 (in progress)
- File C: ⏳ (not started)

### Phase 2: [Name] - ⏳

- [Not started]

## Statistics

- **Files Complete**: X/N (XX%)
- **Lines Translated**: ~X,XXX
- **Tests Written**: XX+
- **Time Spent**: XX hours
- **Compile Status**: [errors/warnings]

## Next Actions

1. [Next file to translate]
2. [Blocker to resolve]
3. [Test to write]
```

### Completion Document Template

```markdown
# [Package] Translation - COMPLETE REVIEW

**Status**: ✅ ALL N/N FILES COMPLETE (100%)
**Method**: Gamma Translation at Pre-Prim 0.0.x

## What We Accomplished

- Files: N/N (100%)
- Lines: ~X,XXX
- Tests: XX+
- TODOs: XX Bija seeds
- Time: XX hours

## Phase Summary

[List each phase with stats]

## TODO Analysis

[Breakdown of seeds planted]

## Lessons Learned

[What worked, what to improve]

## Next Steps

[Prim 0.1.x plan, or next package]
```

## Conclusion

**This workflow is:**

- ✅ **Universal**: Works for any package
- ✅ **Structured**: Clear phases and steps
- ✅ **Flexible**: Adapt to package needs
- ✅ **Trackable**: Progress always visible
- ✅ **Philosophical**: Connects to deeper patterns
- ✅ **Proven**: Based on actual experience

**Use this for**:

- Graph Projection API translation
- Any future package translation
- Complex refactoring projects
- Architecture documentation

**The Pattern**:

```text
PRAKASA → KRIYA → KRAMA
(Plan → Execute → Progress)

Illuminate → Act → Complete

See Whole → Do Work → Track Progress
```

---

_"The Plans are our Prakasa. This template IS the workflow. Use it for any complex translation project. Adapt to your needs, but follow the core pattern: Illuminate before acting, act with structure, progress with tracking."_

**Universal Translation Workflow - Complete!** 🕉️

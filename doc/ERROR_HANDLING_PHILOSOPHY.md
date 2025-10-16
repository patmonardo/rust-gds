# Error Handling in Rust-GDS

**Module**: `src/errors.rs`  
**Translation Source**: `org.neo4j.gds.exceptions.*`  
**Pattern**: Type-safe error enums with `thiserror`

---

## Philosophy: Type-Safe Errors Over Exception Hierarchies

### Java Approach (Exception Hierarchy)

```java
// Base exception class
public class GdsException extends RuntimeException { ... }

// Specific exceptions extend base
public class MemoryEstimationNotImplementedException
    extends IllegalArgumentException { ... }
```

**Characteristics**:

- Runtime exceptions (unchecked)
- Inheritance-based hierarchy
- Stack traces by default
- Can forget to catch specific types

### Rust Approach (Error Enums)

```rust
// Specific error type with semantics
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum MemoryEstimationError {
    #[error("Memory estimation is not implemented for this algorithm.")]
    NotImplemented,
}

// General API errors
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Invalid Input: {0}")]
    InvalidInput(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
}
```

**Characteristics**:

- Compile-time checked (must handle `Result<T, E>`)
- Enum-based variants
- Zero-cost error types (just data)
- Impossible to forget error handling

---

## Current Error Types

### 1. MemoryEstimationError

**Translation**: `MemoryEstimationNotImplementedException.java`

```rust
pub enum MemoryEstimationError {
    #[error("Memory estimation is not implemented for this algorithm.")]
    NotImplemented,
}
```

**Usage**:

```rust
trait Algorithm {
    fn memory_estimation(&self) -> Result<MemoryEstimation, MemoryEstimationError> {
        Err(MemoryEstimationError::NotImplemented)
    }
}
```

**Design Decision**:

- In Java, this extends `IllegalArgumentException` (confusing semantics)
- In Rust, dedicated error type preserves semantic meaning
- Can be converted to `ApiError::InvalidInput` when crossing API boundaries

### 2. ApiError

**Purpose**: General API-level errors for external interfaces

```rust
pub enum ApiError {
    NotFound(String),      // Resource not found
    InvalidInput(String),  // Bad user input
    InternalError(String), // Unexpected internal state
}
```

**Usage**:

```rust
fn get_graph(name: &str) -> Result<Graph, ApiError> {
    catalog.find(name)
        .ok_or_else(|| ApiError::NotFound(format!("Graph '{}' not found", name)))
}
```

---

## Design Patterns

### Pattern 1: Specific Error Types

**When**: Domain-specific errors with clear semantics

```rust
#[derive(Debug, Error)]
pub enum GraphLoadError {
    #[error("Invalid graph format: {0}")]
    InvalidFormat(String),

    #[error("Missing required property: {0}")]
    MissingProperty(String),

    #[error("Node {0} not found")]
    NodeNotFound(u64),
}
```

**Benefits**:

- Self-documenting error cases
- Exhaustive pattern matching
- Type-safe error handling

### Pattern 2: Result-Based APIs

**All fallible operations return `Result<T, E>`**:

```rust
// Good
fn run_algorithm(&self) -> Result<AlgorithmResult, AlgorithmError> { ... }

// Bad (Java-style)
fn run_algorithm(&self) -> AlgorithmResult {
    // Might panic! Caller can't prepare.
}
```

### Pattern 3: Error Conversions

**Use `From<SpecificError>` for error propagation**:

```rust
impl From<GraphLoadError> for ApiError {
    fn from(err: GraphLoadError) -> Self {
        ApiError::InternalError(err.to_string())
    }
}

// Enables ? operator
fn api_endpoint() -> Result<Response, ApiError> {
    let graph = load_graph()?; // GraphLoadError auto-converts to ApiError
    Ok(process(graph))
}
```

### Pattern 4: Error Context with `anyhow`

**For application-level error handling** (not in library code):

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    load_config()
        .context("Failed to load configuration")?;

    run_algorithm()
        .context("Algorithm execution failed")?;

    Ok(())
}
```

---

## Comparison: Java Exceptions vs Rust Errors

| Aspect                   | Java Exceptions                               | Rust Errors                   |
| ------------------------ | --------------------------------------------- | ----------------------------- |
| **Checking**             | Runtime (unchecked) or compile-time (checked) | Always compile-time           |
| **Performance**          | Stack unwinding overhead                      | Zero-cost (just data)         |
| **Semantics**            | Inheritance hierarchy                         | Enum variants                 |
| **Pattern matching**     | `instanceof` checks                           | Exhaustive `match`            |
| **Forgetting to handle** | Runtime panic                                 | Compile error                 |
| **Error messages**       | `getMessage()`                                | `Display` trait               |
| **Stack traces**         | Automatic                                     | Optional (`backtrace` crate)  |
| **Recovery**             | try-catch blocks                              | `Result<T, E>` + `?` operator |

---

## Testing Error Types

```rust
#[test]
fn test_error_messages() {
    let err = MemoryEstimationError::NotImplemented;
    assert_eq!(
        err.to_string(),
        "Memory estimation is not implemented for this algorithm."
    );
}

#[test]
fn test_error_handling() {
    fn might_fail() -> Result<i32, MemoryEstimationError> {
        Err(MemoryEstimationError::NotImplemented)
    }

    let result = might_fail();
    assert!(result.is_err());

    match result {
        Err(MemoryEstimationError::NotImplemented) => { /* expected */ }
        _ => panic!("Wrong error type"),
    }
}
```

---

## Future Error Types (from Java GDS)

As we translate more Java exceptions, we'll add:

### Algorithm Errors

- `AlgorithmNotSupportedException` → `AlgorithmError::NotSupported`
- `InvalidConfigurationException` → `ConfigError::Invalid`

### Graph Errors

- `NodeNotFoundException` → `GraphError::NodeNotFound`
- `PropertyNotFoundException` → `GraphError::PropertyNotFound`

### Concurrency Errors

- `TerminatedException` → `ExecutionError::Terminated`
- `TimeoutException` → `ExecutionError::Timeout`

### I/O Errors

- Standard library `std::io::Error` for file operations
- Custom `ImportError` / `ExportError` for graph I/O

---

## Best Practices

### 1. Be Specific

```rust
// Good - specific error type
#[derive(Debug, Error)]
pub enum PageRankError {
    #[error("Invalid damping factor: {0} (must be in [0, 1])")]
    InvalidDampingFactor(f64),

    #[error("Max iterations must be positive, got {0}")]
    InvalidMaxIterations(usize),
}

// Bad - generic string error
pub type PageRankError = String;
```

### 2. Use thiserror for Libraries

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Something went wrong: {0}")]
    Generic(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),  // Auto-implement From<io::Error>
}
```

### 3. Avoid Panicking in Library Code

```rust
// Good - return error
pub fn divide(a: i32, b: i32) -> Result<i32, ArithmeticError> {
    if b == 0 {
        Err(ArithmeticError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// Bad - panic
pub fn divide(a: i32, b: i32) -> i32 {
    a / b  // Panics on division by zero!
}
```

### 4. Document Error Conditions

```rust
/// Runs PageRank algorithm
///
/// # Errors
///
/// Returns `PageRankError::InvalidDampingFactor` if damping factor not in [0, 1].
/// Returns `PageRankError::InvalidMaxIterations` if max iterations is 0.
pub fn run(&self) -> Result<PageRankResult, PageRankError> {
    // ...
}
```

---

## Error Handling Philosophy

> **"Make illegal states unrepresentable, make errors explicit."**

In Rust, errors are **values**, not **exceptions**. This means:

1. **Errors are part of the type signature** - You can't ignore them
2. **Errors are cheap** - Just enums, no stack unwinding
3. **Errors are composable** - `Result<T, E>` works with `?` operator
4. **Errors are testable** - Pattern match on specific variants

This is fundamentally different from Java's exception model, where:

- Unchecked exceptions can be forgotten
- Stack unwinding has performance overhead
- try-catch creates control flow complexity
- Hard to test specific exception types

---

## Translation Strategy

When translating Java exceptions to Rust:

1. **Identify the semantic meaning** - What does this exception represent?
2. **Create specific error type** - Enum variant for each case
3. **Use `thiserror::Error`** - Derive `Error` trait automatically
4. **Document the mapping** - Note Java source in rustdoc
5. **Write tests** - Verify error messages and handling

**Example**:

```rust
/// **Translation**: `MemoryEstimationNotImplementedException.java`
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum MemoryEstimationError {
    #[error("Memory estimation is not implemented for this algorithm.")]
    NotImplemented,
}
```

---

## Current Status

- ✅ `MemoryEstimationError` - Memory estimation not implemented
- ✅ `ApiError` - General API errors (NotFound, InvalidInput, InternalError)
- ✅ Tests for error messages and cloning
- ✅ Documentation of translation strategy

**Next**: Add more domain-specific error types as we translate Java exceptions

---

## References

- **thiserror**: https://docs.rs/thiserror
- **anyhow**: https://docs.rs/anyhow (for applications)
- **Rust Error Handling**: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- **Error Handling in Libraries**: https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err

---

_"Errors are not exceptional. Make them first-class citizens."_

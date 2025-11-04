## 05 — Rust Patterns in Practice

**Hard-won lessons from the codebase**

This chapter documents the Rust patterns you learned during debugging cycles, especially around Arc/Ref/Trait Objects.

### Arc<Trait> vs Arc<dyn Trait>

**Reference vs Trait Objects**:
- `Arc<&Trait>`: Fast, but lifetime constraints
- `Arc<dyn Trait>`: Type-erased, vtable dispatch, but flexible
- When to choose: performance vs flexibility tradeoffs

**From your experience**: You learned this during codegen debugging where trait object lifetimes caused issues.

### Interior Mutability

**RefCell for mutable shared state**:
```rust
// Share mutable state across threads
let shared = Arc<RefCell<State>>();
let thread_handle = {
    let shared = Arc::clone(&shared);
    thread::spawn(move || {
        shared.borrow_mut().update();
    })
};
```

When Pregel algorithms need shared state across supersteps.

### Type Erasure vs Monomorphization

**Tradeoffs in generic code**:
- Generic functions: Monomorphized, fast but code bloat
- Trait objects: Type-erased, slower but code reuse
- Where Rust's compiler hits limits in graph algorithms

### Where Ownership Bites

**Common pitfalls**:
1. **Borrowing across thread boundaries**: Arc required
2. **Mutable references in parallel contexts**: RefCell needed
3. **Lifetime elision failures**: Explicit lifetimes required
4. **Trait object lifetimes**: Dynamic vs static dispatch issues

### Real Examples from Codebase

**GraphStore sharing**:
```rust
// Share GraphStore across algorithm supersteps
let store = Arc::new(graph_store);
for superstep in 0..max_iterations {
    let store = Arc::clone(&store);
    compute_superstep(&store)?;
}
```

**Message passing**:
```rust
// Messages need to cross ownership boundaries
type MessageBox = Arc<Mutex<Vec<Message>>>;
```

### Lessons Learned

You documented these during the "codegen and Arc/Ref cycles":
- Where Rust's ownership model enforces correctness
- Where interior mutability is actually needed
- How trait objects enable flexible algorithm frameworks
- When explicit lifetimes solve lifetime errors

---

**Next Step**: Apply these patterns as you implement more algorithms!


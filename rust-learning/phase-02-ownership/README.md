# Phase 2: Ownership, Borrowing, and Lifetimes 🔐

Welcome to Phase 2! This is where Rust truly differentiates itself from other languages. You'll learn about Rust's unique memory management system that provides memory safety without a garbage collector.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Understand Rust's ownership system
- ✅ Master the concepts of moving and copying
- ✅ Work with references and borrowing
- ✅ Distinguish between mutable and immutable references
- ✅ Understand lifetime annotations
- ✅ Work with the borrow checker confidently

## ⏱️ Estimated Time

5-7 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Ownership Rules

The three fundamental rules:
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

### 2. Memory Management

- Stack vs Heap memory
- How Rust manages memory without a garbage collector
- RAII (Resource Acquisition Is Initialization)
- Drop trait and automatic cleanup

### 3. Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2
// println!("{}", s1); // ❌ Error! s1 no longer valid
```

### 4. Clone and Copy

```rust
// Clone for heap data
let s1 = String::from("hello");
let s2 = s1.clone(); // Deep copy
println!("{} {}", s1, s2); // ✅ Both valid

// Copy for stack data
let x = 5;
let y = x; // Copy, not move
println!("{} {}", x, y); // ✅ Both valid
```

### 5. References and Borrowing

```rust
// Immutable reference (borrow)
let s1 = String::from("hello");
let len = calculate_length(&s1); // Borrow s1
println!("{} has length {}", s1, len); // s1 still valid

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own the data
```

### 6. Mutable References

```rust
let mut s = String::from("hello");
change(&mut s); // Mutable borrow

fn change(s: &mut String) {
    s.push_str(", world");
}
```

**Rules of References:**
- You can have EITHER:
  - One mutable reference, OR
  - Any number of immutable references
- References must always be valid (no dangling references)

### 7. Lifetimes

```rust
// Lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

## 💻 Hands-On Exercises

### Exercise 1: Ownership Basics
**File**: `exercises/ex01_ownership_basics.rs`

Practice moving, copying, and understanding ownership transfer.

### Exercise 2: String Manipulation with Borrowing
**File**: `exercises/ex02_string_borrowing.rs`

Write functions that borrow strings and manipulate them.

### Exercise 3: Mutable References
**File**: `exercises/ex03_mutable_refs.rs`

Practice working with mutable references and the borrowing rules.

### Exercise 4: Lifetime Annotations
**File**: `exercises/ex04_lifetimes.rs`

Work with functions and structs that require lifetime annotations.

### Exercise 5: Reference Counter (Rc)
**File**: `exercises/ex05_rc_example.rs`

Introduction to `Rc<T>` for multiple ownership scenarios.

## 🎯 Mini-Project: Text Parser

Build a text parser that:
- Reads text input
- Tokenizes into words
- Counts word frequency
- Returns references to most common words
- All memory safe with no leaks!

**Key Skills:**
- String ownership and borrowing
- Vector of references
- HashMap with String keys
- Lifetime management

## 📖 Key Concepts Explained

### Why Ownership?

Rust's ownership system prevents:
- **Dangling pointers**: References to freed memory
- **Double free**: Freeing memory twice
- **Memory leaks**: Forgetting to free memory
- **Data races**: Concurrent access issues

All at compile time, with zero runtime cost!

### The Borrow Checker

The borrow checker is your friend (really!):
- Ensures references are always valid
- Prevents data races at compile time
- May seem strict at first, but teaches safe patterns

### Common Patterns

**Pattern 1: Take ownership**
```rust
fn consume(s: String) {
    println!("{}", s);
} // s is dropped here
```

**Pattern 2: Borrow immutably**
```rust
fn read(s: &String) {
    println!("{}", s);
} // Nothing dropped
```

**Pattern 3: Borrow mutably**
```rust
fn modify(s: &mut String) {
    s.push_str(" modified");
}
```

**Pattern 4: Return ownership**
```rust
fn create() -> String {
    String::from("new string")
}
```

## ✅ Knowledge Check

Before moving to Phase 3, ensure you can:

- [ ] Explain the three ownership rules
- [ ] Describe the difference between move and copy
- [ ] Know which types implement Copy
- [ ] Use references without compiler errors
- [ ] Understand mutable vs immutable references
- [ ] Explain why you can't have multiple mutable references
- [ ] Write functions with lifetime annotations
- [ ] Understand when lifetimes are needed
- [ ] Debug borrow checker errors
- [ ] Choose between String and &str appropriately

## 🎓 Common Borrow Checker Errors

### Error 1: Value moved
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // ❌ Error: value borrowed after move
```

**Fix**: Clone or use reference
```rust
let s2 = s1.clone(); // or
let s2 = &s1;
```

### Error 2: Multiple mutable borrows
```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // ❌ Error: cannot borrow as mutable more than once
```

**Fix**: Use borrows in different scopes or use only one

### Error 3: Mutable and immutable borrow together
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &mut s; // ❌ Error: cannot borrow as mutable
```

**Fix**: Finish using immutable references before mutable borrow

## 📚 Additional Resources

- [The Rust Book - Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Visualizing Memory Layout](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

## ➡️ Next Steps

Once comfortable with ownership and borrowing:

**[Phase 3: Structs, Enums, and Pattern Matching](../phase-03-structs-enums/README.md)**

You'll learn to create custom data types and use Rust's powerful pattern matching!

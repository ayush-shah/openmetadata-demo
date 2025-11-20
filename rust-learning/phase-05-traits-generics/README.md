# Phase 5: Traits, Generics, and Advanced Types 🚀

Welcome to Phase 5! Master Rust's trait system and generic programming to write flexible, reusable code. This is where Rust's type system truly shines.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Define and implement traits
- ✅ Use trait bounds effectively
- ✅ Write generic functions and structs
- ✅ Understand associated types
- ✅ Master smart pointers (Box, Rc, Arc, RefCell)
- ✅ Use trait objects for dynamic dispatch

## ⏱️ Estimated Time

6-8 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Traits - Rust's Interfaces

```rust
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("Unknown")
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}
```

### 2. Generic Data Types

```rust
// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Multiple type parameters
struct Pair<T, U> {
    first: T,
    second: U,
}
```

### 3. Trait Bounds

```rust
// Simple bound
fn print_it<T: Display>(item: T) {
    println!("{}", item);
}

// Multiple bounds
fn compare<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("{} is greater", a);
    }
}

// Where clause for readability
fn complex_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Implementation
}
```

### 4. Associated Types

```rust
trait Iterator {
    type Item; // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32; // Concrete type

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 5. Smart Pointers

**Box<T> - Heap allocation:**
```rust
let b = Box::new(5);
// Useful for recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

**Rc<T> - Reference counting:**
```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // Increment count
let c = Rc::clone(&a);
println!("Count: {}", Rc::strong_count(&a)); // 3
```

**Arc<T> - Atomic reference counting:**
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);

thread::spawn(move || {
    println!("{:?}", data_clone);
});
```

**RefCell<T> - Interior mutability:**
```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1; // Runtime borrow checking
```

### 6. Trait Objects

```rust
// Static dispatch (compile time)
fn process<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

// Dynamic dispatch (runtime)
fn process_dyn(item: &dyn Summary) {
    println!("{}", item.summarize());
}

// Collection of trait objects
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Article { /* ... */ }),
    Box::new(Tweet { /* ... */ }),
];
```

## 💻 Hands-On Exercises

### Exercise 1: Generic Collections
**File**: `exercises/ex01_generic_collections.rs`

Build generic collection wrappers:
```rust
struct Repository<T> {
    items: Vec<T>,
}

impl<T> Repository<T> {
    fn new() -> Self { /* ... */ }
    fn add(&mut self, item: T) { /* ... */ }
    fn find<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    { /* ... */ }
}
```

### Exercise 2: Trait Implementation
**File**: `exercises/ex02_traits.rs`

Implement common traits for OpenMetadata entities:
- Display
- Debug
- Clone
- PartialEq
- From/Into

### Exercise 3: Entity Trait
**File**: `exercises/ex03_entity_trait.rs`

Create a common Entity trait:
```rust
trait Entity {
    type Id;

    fn id(&self) -> &Self::Id;
    fn name(&self) -> &str;
    fn fully_qualified_name(&self) -> String;
}

// Implement for Table, Schema, Database, etc.
```

### Exercise 4: Builder Pattern with Generics
**File**: `exercises/ex04_generic_builder.rs`

Generic builder implementation:
```rust
struct Builder<T> {
    inner: T,
}

impl<T> Builder<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }

    fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(T) -> T,
    {
        Self { inner: f(self.inner) }
    }

    fn build(self) -> T {
        self.inner
    }
}
```

### Exercise 5: Smart Pointer Practice
**File**: `exercises/ex05_smart_pointers.rs`

Use smart pointers appropriately:
- Rc for shared ownership
- RefCell for interior mutability
- Arc for thread-safe sharing

## 🎯 Mini-Project: Generic Metadata Store

Build a generic, type-safe metadata store:

```rust
use std::collections::HashMap;
use std::hash::Hash;

trait Identifiable {
    type Id: Eq + Hash + Clone;
    fn id(&self) -> Self::Id;
}

struct Store<T: Identifiable> {
    items: HashMap<T::Id, T>,
}

impl<T: Identifiable> Store<T> {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn insert(&mut self, item: T) {
        let id = item.id();
        self.items.insert(id, item);
    }

    fn get(&self, id: &T::Id) -> Option<&T> {
        self.items.get(id)
    }

    fn remove(&mut self, id: &T::Id) -> Option<T> {
        self.items.remove(id)
    }

    fn list(&self) -> Vec<&T> {
        self.items.values().collect()
    }
}

// Implement for all entity types
impl Identifiable for Table {
    type Id = String;
    fn id(&self) -> String {
        self.fully_qualified_name.clone()
    }
}
```

**Features:**
1. Generic over any identifiable type
2. Type-safe ID handling
3. Common CRUD operations
4. Filter and search capabilities
5. Transaction support (advanced)

## 📖 Advanced Patterns

### Pattern 1: Extension Trait

```rust
trait VecExt<T> {
    fn first_where<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
    fn first_where<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().find(|item| predicate(item))
    }
}
```

### Pattern 2: Newtype Pattern with Traits

```rust
struct TableFQN(String);

impl Display for TableFQN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TableFQN {
    fn from(s: String) -> Self {
        TableFQN(s)
    }
}
```

### Pattern 3: Sealed Traits

```rust
mod sealed {
    pub trait Sealed {}
}

pub trait EntityType: sealed::Sealed {
    fn type_name(&self) -> &str;
}

impl sealed::Sealed for Table {}
impl EntityType for Table {
    fn type_name(&self) -> &str {
        "table"
    }
}
```

### Pattern 4: Trait Object Collections

```rust
trait Metadata {
    fn as_json(&self) -> serde_json::Value;
    fn entity_type(&self) -> &str;
}

struct MetadataCollection {
    items: Vec<Box<dyn Metadata>>,
}

impl MetadataCollection {
    fn add<T: Metadata + 'static>(&mut self, item: T) {
        self.items.push(Box::new(item));
    }

    fn export_all(&self) -> Vec<serde_json::Value> {
        self.items.iter().map(|item| item.as_json()).collect()
    }
}
```

## ✅ Knowledge Check

Before moving to Phase 6, ensure you can:

- [ ] Define and implement traits
- [ ] Use generic type parameters
- [ ] Apply trait bounds correctly
- [ ] Understand associated types vs generic parameters
- [ ] Use Box, Rc, Arc, RefCell appropriately
- [ ] Choose between static and dynamic dispatch
- [ ] Implement common standard library traits
- [ ] Write trait object safe traits
- [ ] Use where clauses for complex bounds
- [ ] Understand lifetime bounds on generics

## 🎓 Design Decisions

### Static vs Dynamic Dispatch

**Static (generics):**
- Faster (no virtual calls)
- Larger binary (monomorphization)
- Known at compile time

```rust
fn process<T: Summary>(item: T) { }
```

**Dynamic (trait objects):**
- Smaller binary
- Slight runtime cost
- Heterogeneous collections

```rust
fn process(item: &dyn Summary) { }
```

### Associated Types vs Generic Parameters

**Associated types:** One implementation per type
```rust
trait Iterator {
    type Item;
}
```

**Generic parameters:** Multiple implementations possible
```rust
trait From<T> {
    fn from(value: T) -> Self;
}
```

## 📚 Additional Resources

- [The Rust Book - Chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust by Example - Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

## ➡️ Next Steps

Ready to organize your code professionally?

**[Phase 6: Testing, Modules, and Project Organization](../phase-06-modules-testing/README.md)**

You'll learn professional code organization and comprehensive testing!

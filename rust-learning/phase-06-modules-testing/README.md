# Phase 6: Testing, Modules, and Project Organization 📦

Welcome to Phase 6! Learn to organize code like a professional and write comprehensive tests. Essential for maintainable projects.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Organize code with modules
- ✅ Use workspaces for multi-crate projects
- ✅ Write unit and integration tests
- ✅ Use documentation tests
- ✅ Benchmark code with criterion
- ✅ Follow Rust project conventions

## ⏱️ Estimated Time

4-6 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Module System

```rust
// lib.rs or main.rs
mod database {
    pub mod table {
        pub struct Table {
            name: String,
        }

        impl Table {
            pub fn new(name: String) -> Self {
                Self { name }
            }
        }
    }

    pub mod schema {
        pub struct Schema {
            name: String,
        }
    }
}

// Using modules
use database::table::Table;
```

**File-based modules:**
```
src/
├── main.rs
├── lib.rs
├── database/
│   ├── mod.rs
│   ├── table.rs
│   ├── schema.rs
│   └── column.rs
└── api/
    ├── mod.rs
    └── client.rs
```

### 2. Visibility and Privacy

```rust
pub mod outer_mod {
    pub fn public_function() {
        println!("Called outer_mod public_function");
        private_function();
    }

    fn private_function() {
        println!("Called outer_mod private_function");
    }

    pub mod nested {
        pub fn function() {
            super::private_function(); // Access parent
        }

        pub(crate) fn crate_visible() {
            // Visible within crate
        }

        pub(super) fn parent_visible() {
            // Visible to parent module
        }
    }
}
```

### 3. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_table_creation() {
        let table = Table::new("users".to_string());
        assert_eq!(table.name(), "users");
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_division_by_zero() {
        divide(10, 0);
    }

    #[test]
    #[ignore] // Skip unless --ignored
    fn expensive_test() {
        // Long running test
    }
}
```

### 4. Integration Tests

```
tests/
├── integration_test.rs
├── common/
│   └── mod.rs
└── database_tests.rs
```

```rust
// tests/integration_test.rs
use my_crate::Table;

#[test]
fn test_table_operations() {
    let mut table = Table::new("users");
    table.add_column("id", "INT");
    assert_eq!(table.column_count(), 1);
}
```

### 5. Documentation Tests

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 6. Workspaces

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "openmetadata-client",
    "openmetadata-types",
    "openmetadata-cli",
]

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 7. Benchmarking with Criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## 💻 Hands-On Exercises

### Exercise 1: Module Organization
**File**: `exercises/ex01_modules/`

Organize an OpenMetadata client into modules:
```
src/
├── lib.rs
├── entities/
│   ├── mod.rs
│   ├── table.rs
│   ├── schema.rs
│   └── database.rs
├── api/
│   ├── mod.rs
│   └── client.rs
└── error.rs
```

### Exercise 2: Comprehensive Testing
**File**: `exercises/ex02_testing/`

Write tests for all functions:
- Unit tests (inline)
- Integration tests (tests/ dir)
- Documentation tests
- Edge cases and error conditions

### Exercise 3: Test Utilities
**File**: `exercises/ex03_test_utilities/`

Create test helpers:
```rust
// tests/common/mod.rs
pub fn setup_test_db() -> TestDatabase {
    // Setup code
}

pub fn mock_table() -> Table {
    Table::new("test_table")
}
```

### Exercise 4: Workspace Project
**File**: `exercises/ex04_workspace/`

Create a workspace with:
- `types` - Shared types
- `client` - API client
- `cli` - Command line tool

### Exercise 5: Benchmarking
**File**: `exercises/ex05_benchmarks/`

Benchmark critical operations:
- Parsing JSON
- Building entities
- Collection operations

## 🎯 Mini-Project: OpenMetadata Client Library

Organize a complete OpenMetadata client:

```
openmetadata-client/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API
│   ├── client.rs           # HTTP client
│   ├── config.rs           # Configuration
│   ├── error.rs            # Error types
│   ├── entities/           # Entity types
│   │   ├── mod.rs
│   │   ├── table.rs
│   │   ├── schema.rs
│   │   ├── database.rs
│   │   └── service.rs
│   ├── api/                # API operations
│   │   ├── mod.rs
│   │   ├── tables.rs
│   │   ├── schemas.rs
│   │   └── lineage.rs
│   └── utils/              # Utilities
│       ├── mod.rs
│       └── pagination.rs
├── tests/                  # Integration tests
│   ├── common/
│   │   └── mod.rs
│   ├── table_tests.rs
│   └── lineage_tests.rs
├── benches/                # Benchmarks
│   └── client_bench.rs
└── examples/               # Example code
    ├── list_tables.rs
    └── create_lineage.rs
```

**lib.rs:**
```rust
//! OpenMetadata Rust Client
//!
//! This crate provides a type-safe Rust client for OpenMetadata API.
//!
//! # Examples
//!
//! ```no_run
//! use openmetadata_client::{Client, Config};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = Config::builder()
//!         .host("http://localhost:8585")
//!         .build();
//!
//!     let client = Client::new(config).await.unwrap();
//!     let tables = client.list_tables().await.unwrap();
//! }
//! ```

pub mod client;
pub mod config;
pub mod error;
pub mod entities;
pub mod api;
mod utils;

// Re-exports
pub use client::Client;
pub use config::Config;
pub use error::{Error, Result};
```

## 📖 Best Practices

### Project Structure

```
my-project/
├── Cargo.toml
├── Cargo.lock           # Committed for binaries, not libraries
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── lib.rs           # Library root
│   ├── main.rs          # Binary entry point
│   └── bin/             # Additional binaries
│       └── tool.rs
├── tests/               # Integration tests
│   └── integration.rs
├── benches/             # Benchmarks
│   └── bench.rs
├── examples/            # Example code
│   └── simple.rs
└── docs/                # Additional documentation
    └── guide.md
```

### Testing Strategy

1. **Unit tests**: Test individual functions
2. **Integration tests**: Test public API
3. **Doc tests**: Ensure examples work
4. **Property tests**: Use proptest for edge cases
5. **Benchmarks**: Track performance

### Module Organization

```rust
// Prefer flat over deep hierarchy
// Good
use crate::entities::{Table, Schema, Database};

// Avoid
use crate::entities::database::schema::table::column::Column;

// Group related items
pub mod entities {
    mod table;
    mod schema;
    mod database;

    pub use table::Table;
    pub use schema::Schema;
    pub use database::Database;
}
```

## ✅ Knowledge Check

Before moving to Phase 7, ensure you can:

- [ ] Organize code into modules
- [ ] Use `pub`, `pub(crate)`, `pub(super)`
- [ ] Create file-based module hierarchies
- [ ] Write unit tests with assertions
- [ ] Write integration tests
- [ ] Use test utilities and fixtures
- [ ] Write documentation tests
- [ ] Set up workspaces
- [ ] Share dependencies across workspace
- [ ] Run benchmarks with criterion
- [ ] Organize a professional Rust project

## 🎓 Testing Tips

### Assert Macros

```rust
assert!(condition);
assert_eq!(left, right);
assert_ne!(left, right);
debug_assert!(condition); // Debug only

// Custom message
assert!(value > 0, "Value must be positive, got {}", value);
```

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;

        #[test]
        fn test_one() { }
    }

    mod integration {
        use super::*;

        #[test]
        fn test_two() { }
    }
}
```

### Running Tests

```bash
cargo test                    # All tests
cargo test test_name         # Specific test
cargo test --lib             # Library tests only
cargo test --test integration # Specific integration test
cargo test -- --nocapture    # Show println output
cargo test -- --ignored      # Run ignored tests
```

## 📚 Additional Resources

- [The Rust Book - Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [The Rust Book - Chapter 11](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust by Example - Testing](https://doc.rust-lang.org/rust-by-example/testing.html)
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/)

## ➡️ Next Steps

Ready for async programming and concurrency?

**[Phase 7: Async/Await and Concurrency](../phase-07-async-concurrency/README.md)**

You'll learn asynchronous programming with Tokio and build concurrent applications!

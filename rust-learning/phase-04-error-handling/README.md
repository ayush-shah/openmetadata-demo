# Phase 4: Error Handling and Collections 🛡️

Welcome to Phase 4! Learn to handle errors gracefully and work with Rust's powerful collection types. Essential skills for building robust applications.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Master Result<T, E> and Option<T>
- ✅ Use the ? operator effectively
- ✅ Create custom error types
- ✅ Work with thiserror and anyhow crates
- ✅ Master Vec, HashMap, and HashSet
- ✅ Use iterators like a pro

## ⏱️ Estimated Time

5-7 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Result and Option

```rust
// Option for nullable values
fn find_user(id: u32) -> Option<User> {
    // Returns Some(user) or None
}

// Result for operations that can fail
fn save_user(user: User) -> Result<(), Error> {
    // Returns Ok(()) or Err(error)
}
```

### 2. The ? Operator

```rust
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // Early return on error
    let processed = content.trim();
    Ok(processed.to_string())
}
```

### 3. Custom Error Types with `thiserror`

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetadataError {
    #[error("Table not found: {0}")]
    TableNotFound(String),

    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Invalid configuration")]
    InvalidConfig,

    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
```

### 4. Flexible Error Handling with `anyhow`

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;

    Ok(config)
}
```

### 5. Vectors

```rust
// Create vector
let mut vec: Vec<i32> = Vec::new();
let vec = vec![1, 2, 3, 4, 5];

// Add elements
vec.push(6);
vec.extend([7, 8, 9]);

// Access elements
let third = vec[2]; // Panics if out of bounds
let third = vec.get(2); // Returns Option<&T>

// Iterate
for item in &vec {
    println!("{}", item);
}
```

### 6. HashMap

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Access
if let Some(score) = scores.get("Blue") {
    println!("Blue team: {}", score);
}

// Update
scores.entry(String::from("Blue"))
    .and_modify(|s| *s += 10)
    .or_insert(50);
```

### 7. Iterators

```rust
let vec = vec![1, 2, 3, 4, 5];

// Iterator adapters (lazy)
let doubled: Vec<_> = vec.iter()
    .map(|x| x * 2)
    .filter(|x| x > &5)
    .collect();

// Common methods
let sum: i32 = vec.iter().sum();
let max = vec.iter().max();
let any_greater_than_3 = vec.iter().any(|&x| x > 3);
```

## 💻 Hands-On Exercises

### Exercise 1: Custom Error Types
**File**: `exercises/ex01_custom_errors.rs`

Create a comprehensive error type for OpenMetadata operations:
- Connection errors
- API errors
- Validation errors
- Not found errors

### Exercise 2: Configuration Parser
**File**: `exercises/ex02_config_parser.rs`

Parse and validate configuration with proper error handling:
```rust
struct Config {
    host: String,
    port: u16,
    auth_token: Option<String>,
}

fn parse_config(path: &str) -> Result<Config, ConfigError> {
    // Implementation with proper error handling
}
```

### Exercise 3: Data Aggregation
**File**: `exercises/ex03_data_aggregation.rs`

Use collections to aggregate metadata:
- Count tables per schema
- Group columns by data type
- Find most common tags

### Exercise 4: Iterator Mastery
**File**: `exercises/ex04_iterators.rs`

Advanced iterator operations:
- Chain multiple transformations
- Custom iterator implementation
- Performance optimization

### Exercise 5: Cache Implementation
**File**: `exercises/ex05_cache.rs`

Build an in-memory cache with HashMap:
```rust
struct MetadataCache {
    tables: HashMap<String, Table>,
    schemas: HashMap<String, Schema>,
}

impl MetadataCache {
    fn get_table(&self, fqn: &str) -> Option<&Table> {
        // Implementation
    }

    fn insert_table(&mut self, fqn: String, table: Table) {
        // Implementation
    }

    fn invalidate(&mut self, fqn: &str) {
        // Implementation
    }
}
```

## 🎯 Mini-Project: Metadata Validator

Build a validation system for OpenMetadata entities:

```rust
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field '{field}' is required")]
    RequiredField { field: String },

    #[error("Field '{field}' has invalid format: {reason}")]
    InvalidFormat { field: String, reason: String },

    #[error("Value '{value}' exceeds maximum {max}")]
    ValueTooLarge { value: String, max: usize },
}

trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

impl Validate for Table {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            return Err(ValidationError::RequiredField {
                field: "name".to_string(),
            });
        }

        if self.columns.is_empty() {
            return Err(ValidationError::RequiredField {
                field: "columns".to_string(),
            });
        }

        // Validate each column
        for column in &self.columns {
            column.validate()?;
        }

        Ok(())
    }
}
```

**Features:**
1. Validate all entity types
2. Comprehensive error messages
3. Aggregate multiple errors
4. Custom validation rules
5. Use Result and Option effectively

## 📖 Error Handling Patterns

### Pattern 1: Early Return with ?

```rust
fn complex_operation() -> Result<Data, Error> {
    let step1 = step_one()?;
    let step2 = step_two(&step1)?;
    let step3 = step_three(&step2)?;
    Ok(step3)
}
```

### Pattern 2: Match for Recovery

```rust
match risky_operation() {
    Ok(data) => process(data),
    Err(e) if e.is_temporary() => retry(),
    Err(e) => return Err(e),
}
```

### Pattern 3: Combining Results

```rust
use itertools::Itertools;

fn validate_all(items: &[Item]) -> Result<(), Vec<Error>> {
    let errors: Vec<_> = items
        .iter()
        .filter_map(|item| item.validate().err())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
```

### Pattern 4: Context Addition

```rust
fn load_table(fqn: &str) -> Result<Table> {
    let data = fetch_from_api(fqn)
        .with_context(|| format!("Failed to fetch table: {}", fqn))?;

    parse_table(&data)
        .with_context(|| format!("Failed to parse table: {}", fqn))?
}
```

## 📊 Collection Patterns

### Pattern 1: HashMap for Lookups

```rust
let table_map: HashMap<String, Table> = tables
    .into_iter()
    .map(|t| (t.fully_qualified_name.clone(), t))
    .collect();
```

### Pattern 2: HashSet for Uniqueness

```rust
let unique_types: HashSet<_> = columns
    .iter()
    .map(|c| &c.data_type)
    .collect();
```

### Pattern 3: Vec for Ordered Data

```rust
let mut tables = vec![/* ... */];
tables.sort_by(|a, b| a.name.cmp(&b.name));
```

### Pattern 4: Iterator Chains

```rust
let pii_tables: Vec<_> = metadata
    .tables
    .iter()
    .filter(|t| t.has_pii_columns())
    .map(|t| &t.name)
    .collect();
```

## ✅ Knowledge Check

Before moving to Phase 5, ensure you can:

- [ ] Use Result and Option appropriately
- [ ] Apply the ? operator correctly
- [ ] Create custom error types with thiserror
- [ ] Use anyhow for application errors
- [ ] Work with Vec, HashMap, and HashSet
- [ ] Chain iterator operations
- [ ] Choose the right collection for the job
- [ ] Handle errors at appropriate levels
- [ ] Provide context for errors
- [ ] Write error-free iterator chains

## 🎓 When to Use What?

### Result vs Option

- **Option**: Something might not exist (nullable)
- **Result**: Operation might fail (error handling)

### anyhow vs thiserror

- **thiserror**: For library error types (precise)
- **anyhow**: For application errors (flexible)

### Vec vs HashMap vs HashSet

- **Vec**: Ordered collection, indexed access
- **HashMap**: Key-value pairs, fast lookup
- **HashSet**: Unique values, membership testing

## 📚 Additional Resources

- [The Rust Book - Chapter 8 & 9](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Error Handling in Rust](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [thiserror docs](https://docs.rs/thiserror/)
- [anyhow docs](https://docs.rs/anyhow/)
- [Iterator documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

## ➡️ Next Steps

Ready for advanced type system features?

**[Phase 5: Traits, Generics, and Advanced Types](../phase-05-traits-generics/README.md)**

You'll learn about Rust's powerful trait system and generic programming!

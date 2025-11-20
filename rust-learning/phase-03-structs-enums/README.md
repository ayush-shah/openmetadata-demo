# Phase 3: Structs, Enums, and Pattern Matching 🎯

Welcome to Phase 3! Here you'll learn to create custom data types that model your domain perfectly. These are the building blocks for representing OpenMetadata entities later.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Define and use structs
- ✅ Implement methods and associated functions
- ✅ Master enums and their variants
- ✅ Use Option<T> and Result<T, E> effectively
- ✅ Pattern match like a pro
- ✅ Model complex data structures

## ⏱️ Estimated Time

4-6 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Structs

**Classic struct:**
```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};
```

**Tuple struct:**
```rust
struct Color(u8, u8, u8);
let black = Color(0, 0, 0);
```

**Unit struct:**
```rust
struct AlwaysEqual;
let subject = AlwaysEqual;
```

### 2. Methods and Associated Functions

```rust
impl User {
    // Associated function (constructor)
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            age: 0,
            active: true,
        }
    }

    // Method (takes &self)
    fn is_active(&self) -> bool {
        self.active
    }

    // Mutable method
    fn deactivate(&mut self) {
        self.active = false;
    }

    // Takes ownership
    fn into_email(self) -> String {
        self.email
    }
}
```

### 3. Enums

```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

### 4. Option<T>

```rust
// Rust's way of handling nullable values
enum Option<T> {
    Some(T),
    None,
}

let some_number = Some(5);
let no_number: Option<i32> = None;

// Pattern matching
match some_number {
    Some(n) => println!("Got: {}", n),
    None => println!("No value"),
}
```

### 5. Result<T, E>

```rust
// For operations that can fail
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

### 6. Pattern Matching

```rust
// match expression
match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    4..=9 => println!("four through nine"),
    _ => println!("anything else"),
}

// if let for single pattern
if let Some(value) = optional {
    println!("Got {}", value);
}

// while let
while let Some(value) = iterator.next() {
    println!("{}", value);
}
```

## 💻 Hands-On Exercises

### Exercise 1: Database Entity Modeling
**File**: `exercises/ex01_database_model.rs`

Model OpenMetadata database entities:
- DatabaseService
- Database
- DatabaseSchema
- Table
- Column

### Exercise 2: Service Type Enum
**File**: `exercises/ex02_service_types.rs`

Create enums for different service types:
- DatabaseServiceType (MySQL, PostgreSQL, Snowflake, etc.)
- StorageServiceType (S3, GCS, Azure)
- MessagingServiceType (Kafka, Pulsar)

### Exercise 3: Configuration Builder
**File**: `exercises/ex03_config_builder.rs`

Build a configuration struct with builder pattern:
```rust
let config = ConfigBuilder::new()
    .host("localhost")
    .port(8585)
    .auth_token("token")
    .build();
```

### Exercise 4: Result and Option Handling
**File**: `exercises/ex04_option_result.rs`

Practice error handling with Result and Option:
- Parse configuration
- Validate inputs
- Handle missing values

### Exercise 5: Pattern Matching Complex Data
**File**: `exercises/ex05_pattern_matching.rs`

Advanced pattern matching scenarios with OpenMetadata entities.

## 🎯 Mini-Project: Metadata Model

Build a simplified OpenMetadata data model:

```rust
// Core entities
struct Table {
    name: String,
    schema: String,
    database: String,
    columns: Vec<Column>,
    tags: Vec<Tag>,
}

struct Column {
    name: String,
    data_type: DataType,
    description: Option<String>,
}

enum DataType {
    Int,
    BigInt,
    String,
    Float,
    Boolean,
    Timestamp,
    Json,
}

struct Tag {
    name: String,
    category: TagCategory,
}

enum TagCategory {
    Tier,
    PII,
    Classification,
    Custom(String),
}

impl Table {
    fn new(name: String, schema: String, database: String) -> Self {
        // Implementation
    }

    fn add_column(&mut self, column: Column) {
        // Implementation
    }

    fn find_column(&self, name: &str) -> Option<&Column> {
        // Implementation
    }

    fn add_tag(&mut self, tag: Tag) {
        // Implementation
    }

    fn has_pii(&self) -> bool {
        // Implementation
    }
}
```

**Requirements:**
1. Define all structs and enums
2. Implement constructors and methods
3. Handle optional fields with Option
4. Use pattern matching for type checking
5. Write tests for all functionality

## 📖 Real-World Patterns

### Pattern 1: Builder Pattern

```rust
struct ConnectionBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
}

impl ConnectionBuilder {
    fn new() -> Self {
        Self {
            host: None,
            port: None,
            username: None,
        }
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn build(self) -> Result<Connection, String> {
        // Validate and build
    }
}
```

### Pattern 2: Type State Pattern

```rust
// Different states as types
struct Disconnected;
struct Connected;

struct Client<State> {
    url: String,
    state: PhantomData<State>,
}

impl Client<Disconnected> {
    fn connect(self) -> Result<Client<Connected>, Error> {
        // ...
    }
}

impl Client<Connected> {
    fn query(&self, sql: &str) -> Result<Data, Error> {
        // Only available when connected!
    }
}
```

### Pattern 3: Newtype Pattern

```rust
struct TableName(String);
struct SchemaName(String);

// Type safety - can't mix them up
fn get_table(table: TableName, schema: SchemaName) -> Table {
    // Implementation
}
```

## ✅ Knowledge Check

Before moving to Phase 4, ensure you can:

- [ ] Define structs with various field types
- [ ] Implement methods on structs
- [ ] Use associated functions (constructors)
- [ ] Create and use enums with different variants
- [ ] Pattern match on enums exhaustively
- [ ] Use Option<T> instead of null
- [ ] Handle Results with match or ?
- [ ] Use if let and while let
- [ ] Implement the builder pattern
- [ ] Model complex domain objects

## 🎓 Design Tips

### When to use Struct vs Enum?

**Use Struct when:**
- You have a fixed set of named fields
- All fields are present
- Example: User, Configuration

**Use Enum when:**
- You have multiple variants/alternatives
- Only one variant is active at a time
- Example: ServiceType, Status, Result

### Public vs Private

```rust
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    internal_id: u64, // Private
}

impl Table {
    pub fn new(name: String) -> Self {
        // Public
    }

    fn internal_validate(&self) -> bool {
        // Private
    }
}
```

## 📚 Additional Resources

- [The Rust Book - Chapter 5 & 6](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust by Example - Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [Rust by Example - Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Pattern Syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

## ➡️ Next Steps

Ready to handle errors and work with collections?

**[Phase 4: Error Handling and Collections](../phase-04-error-handling/README.md)**

You'll learn robust error handling and master Rust's collection types!

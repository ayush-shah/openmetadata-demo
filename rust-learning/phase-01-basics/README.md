# Phase 1: Rust Basics - Setup and Fundamentals 🌱

Welcome to Phase 1! This is where your Rust journey begins. By the end of this phase, you'll understand Rust's basic syntax and be able to write simple programs.

## 📋 Learning Objectives

By completing this phase, you will:
- ✅ Install and configure the Rust toolchain
- ✅ Understand Rust's basic syntax and conventions
- ✅ Work with variables, data types, and functions
- ✅ Master control flow (if, loop, while, for)
- ✅ Use Cargo to build and run projects
- ✅ Write your first Rust programs

## ⏱️ Estimated Time

3-5 days (3-4 hours per day)

## 📚 Topics Covered

### 1. Environment Setup
- Installing rustup, rustc, and cargo
- IDE setup (VS Code with rust-analyzer)
- Understanding the Rust toolchain

### 2. Hello World
- Your first Rust program
- Understanding `fn main()`
- Using `println!` macro

### 3. Variables and Mutability
- Variable declarations with `let`
- Immutability by default
- Mutable variables with `mut`
- Constants with `const`
- Shadowing

### 4. Data Types
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples and arrays
- Type annotations and type inference

### 5. Functions
- Defining functions with `fn`
- Parameters and return values
- Statements vs expressions
- Return values with and without `return` keyword

### 6. Control Flow
- `if` expressions
- `loop`, `while`, and `for` loops
- `break` and `continue`
- Loop labels

### 7. Comments and Documentation
- Line comments `//`
- Block comments `/* */`
- Doc comments `///` and `//!`

## 🛠️ Setup Instructions

### 1. Install Rust

```bash
# Install rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, typically choose option 1 (default installation)

# Add to PATH (usually automatic, but if needed):
source $HOME/.cargo/env
```

### 2. Verify Installation

```bash
rustc --version  # Should show: rustc 1.x.x
cargo --version  # Should show: cargo 1.x.x
rustup --version # Should show: rustup 1.x.x
```

### 3. Update Rust (when needed)

```bash
rustup update
```

### 4. Install VS Code Extensions (Optional but Recommended)

- rust-analyzer: Best Rust language server
- CodeLLDB: Debugging support
- crates: Manage dependencies
- Better TOML: For Cargo.toml files

## 📖 Lessons

### Lesson 1: Hello World

Create your first Rust program:

```bash
cd phase-01-basics
cargo new hello_world
cd hello_world
cargo run
```

**What just happened?**
- `cargo new` created a new Rust project
- `cargo run` compiled and ran your program
- Check out `src/main.rs` to see the code

### Lesson 2: Variables and Mutability

See: `examples/01_variables.rs`

Key concepts:
```rust
// Immutable by default
let x = 5;
// x = 6; // Error! Cannot mutate immutable variable

// Mutable with mut keyword
let mut y = 5;
y = 6; // OK!

// Constants - always immutable, type must be annotated
const MAX_POINTS: u32 = 100_000;

// Shadowing - declare new variable with same name
let z = 5;
let z = z + 1; // New variable, not mutation
let z = z * 2;
```

### Lesson 3: Data Types

See: `examples/02_data_types.rs`

**Scalar Types:**
```rust
// Integers: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
let age: u8 = 42;
let count: i32 = -10;

// Floats: f32, f64
let pi: f64 = 3.14159;

// Boolean
let is_active: bool = true;

// Character (4 bytes, Unicode)
let letter: char = 'A';
let emoji: char = '🦀';
```

**Compound Types:**
```rust
// Tuples - fixed size, mixed types
let person: (&str, u8) = ("Alice", 30);
let (name, age) = person; // Destructuring

// Arrays - fixed size, same type
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let first = numbers[0];
let all_threes = [3; 5]; // [3, 3, 3, 3, 3]
```

### Lesson 4: Functions

See: `examples/03_functions.rs`

```rust
// Basic function
fn greet() {
    println!("Hello!");
}

// With parameters
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// With return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // Expression, no semicolon
}

// Equivalent with return keyword
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
}

// Multiple return values via tuple
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
```

### Lesson 5: Control Flow

See: `examples/04_control_flow.rs`

**If Expressions:**
```rust
let number = 7;

if number < 5 {
    println!("Less than 5");
} else if number < 10 {
    println!("Between 5 and 10");
} else {
    println!("10 or greater");
}

// if is an expression, can assign result
let result = if number > 5 { "big" } else { "small" };
```

**Loops:**
```rust
// Infinite loop
loop {
    println!("Forever!");
    break; // Exit loop
}

// Loop with return value
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};

// While loop
let mut n = 3;
while n != 0 {
    println!("{}!", n);
    n -= 1;
}

// For loop (most common)
for number in 1..=5 {
    println!("{}", number);
}

// Iterate over array
let arr = [10, 20, 30];
for element in arr.iter() {
    println!("{}", element);
}

// Loop with labels
'outer: loop {
    'inner: loop {
        break 'outer; // Break out of outer loop
    }
}
```

## 💻 Hands-On Exercises

Complete these exercises to solidify your understanding. Each exercise has a starter file in the `exercises/` directory.

### Exercise 1: Temperature Converter ❄️🌡️
**File**: `exercises/ex01_temperature.rs`

Write a program that converts temperatures between Celsius and Fahrenheit.

Requirements:
- Function `celsius_to_fahrenheit(c: f64) -> f64`
- Function `fahrenheit_to_celsius(f: f64) -> f64`
- Test with multiple values

Formulas:
- F = C × 9/5 + 32
- C = (F - 32) × 5/9

### Exercise 2: Fibonacci Generator 📈
**File**: `exercises/ex02_fibonacci.rs`

Generate the first n Fibonacci numbers.

Requirements:
- Function `fibonacci(n: u32) -> Vec<u64>`
- Start with 0, 1
- Handle n = 0 (return empty vec)

### Exercise 3: Prime Number Checker 🔢
**File**: `exercises/ex03_prime.rs`

Check if a number is prime.

Requirements:
- Function `is_prime(n: u64) -> bool`
- Handle edge cases (0, 1, 2)
- Optimize for large numbers

### Exercise 4: String Analyzer 📝
**File**: `exercises/ex04_string_analyzer.rs`

Analyze a string and return statistics.

Requirements:
- Count characters, words, lines
- Find longest word
- Return as tuple or struct (we'll use tuple for now)

### Exercise 5: Number Guessing Game 🎮
**File**: `exercises/ex05_guessing_game.rs`

Classic number guessing game.

Requirements:
- Generate random number 1-100
- Read user input
- Give "higher" or "lower" hints
- Count attempts

Hint: You'll need to add `rand` crate to `Cargo.toml`:
```toml
[dependencies]
rand = "0.8"
```

## 🎯 Mini-Project: CLI Calculator

Build a command-line calculator that can:
- Take two numbers as input
- Support operations: +, -, *, /, %
- Handle division by zero
- Continue until user quits

**File**: `projects/calculator/src/main.rs`

Structure:
```rust
fn main() {
    loop {
        // Get operation
        // Get numbers
        // Calculate
        // Display result
        // Ask to continue
    }
}

fn add(a: f64, b: f64) -> f64 { a + b }
fn subtract(a: f64, b: f64) -> f64 { a - b }
fn multiply(a: f64, b: f64) -> f64 { a * b }
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

## ✅ Knowledge Check

Before moving to Phase 2, ensure you can:

- [ ] Explain the difference between `let` and `let mut`
- [ ] Describe what shadowing is and when to use it
- [ ] List all primitive types in Rust
- [ ] Write a function with parameters and return values
- [ ] Use `if`, `loop`, `while`, and `for` correctly
- [ ] Create and run a Cargo project
- [ ] Understand when to use `break` and `continue`
- [ ] Explain why Rust uses semicolons in some places but not others

## 📚 Additional Resources

- [The Rust Book - Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust by Example - Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [Rustlings - Basic Exercises](https://github.com/rust-lang/rustlings)

## 🎓 Quiz

Test your knowledge:

1. What is the default mutability in Rust?
2. What's the difference between `i32` and `u32`?
3. Can you change the type of a variable through shadowing?
4. What's the difference between a statement and an expression?
5. How do you return a value from a loop?

Answers in `QUIZ_ANSWERS.md`

## ➡️ Next Steps

Once you've completed all exercises and the mini-project, you're ready for:

**[Phase 2: Ownership, Borrowing, and Lifetimes](../phase-02-ownership/README.md)**

This is where Rust gets really interesting! You'll learn about Rust's unique memory management system that provides memory safety without garbage collection.

---

**Need Help?**
- Re-read the examples
- Check the solutions (but try first!)
- Consult The Rust Book
- Ask in Rust community forums

Good luck! 🦀

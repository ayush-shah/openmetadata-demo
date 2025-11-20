// Phase 1: Functions
// Demonstrates function definitions, parameters, and return values

fn main() {
    println!("=== Functions in Rust ===\n");

    // Simple function call
    greet();

    // Function with parameters
    greet_person("Alice");
    greet_person("Bob");

    // Functions with return values
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    let difference = subtract(10, 4);
    println!("10 - 4 = {}", difference);

    // Multiple return values via tuple
    let (quotient, remainder) = divide(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);

    // Expression vs statement
    let result = {
        let x = 3;
        x + 1  // No semicolon - this is an expression
    };
    println!("Result from block: {}", result);

    // Calling all operation functions
    println!("\n--- Calculator Functions ---");
    println!("10 + 5 = {}", add(10, 5));
    println!("10 - 5 = {}", subtract(10, 5));
    println!("10 * 5 = {}", multiply(10, 5));
    println!("10 / 5 = {}", divide_float(10.0, 5.0));
    println!("10 % 3 = {}", modulo(10, 3));

    // Early return example
    let is_even = check_even(42);
    println!("\nIs 42 even? {}", is_even);

    // Function that returns nothing (unit type ())
    print_separator();

    // Complex calculation
    let celsius = 100.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C = {}°F", celsius, fahrenheit);
}

// Function without parameters or return value
fn greet() {
    println!("Hello, World!");
}

// Function with parameter
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value (implicit return)
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon - this is returned
}

// Function with explicit return
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;
    // You can use explicit return, but idiomatic Rust prefers implicit
}

// Function with multiple return values (using tuple)
fn divide(a: i32, b: i32) -> (i32, i32) {
    let quotient = a / b;
    let remainder = a % b;
    (quotient, remainder) // Return tuple
}

// More math functions
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide_float(a: f64, b: f64) -> f64 {
    a / b
}

fn modulo(a: i32, b: i32) -> i32 {
    a % b
}

// Function with early return
fn check_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;  // Early return
    }
    false // Implicit return
}

// Function that returns nothing (unit type)
fn print_separator() {
    println!("-------------------");
}

// Practical example: Temperature conversion
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Function with documentation comment
/// Calculates the area of a rectangle.
///
/// # Arguments
///
/// * `width` - The width of the rectangle
/// * `height` - The height of the rectangle
///
/// # Examples
///
/// ```
/// let area = calculate_area(5.0, 10.0);
/// assert_eq!(area, 50.0);
/// ```
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

/*
Key Takeaways:
1. Functions are defined with `fn` keyword
2. Snake_case is the naming convention
3. Parameters must have type annotations
4. Return type specified with -> Type
5. Last expression in function is returned (no semicolon)
6. Can use explicit `return` keyword for early returns
7. Multiple return values via tuples
8. () is the unit type (like void in other languages)
9. Statements end with semicolon, expressions don't
10. Doc comments with /// for documentation
*/

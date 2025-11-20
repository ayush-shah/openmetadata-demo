// Phase 1: Data Types
// Explores Rust's scalar and compound data types

fn main() {
    println!("=== Rust Data Types ===\n");

    // === SCALAR TYPES ===

    // Integers - signed (i) and unsigned (u)
    println!("--- Integers ---");
    let age: u8 = 42;           // 0 to 255
    let temp: i8 = -10;          // -128 to 127
    let population: u32 = 1_000_000;  // Note: underscores for readability
    let distance: i64 = -500_000;

    println!("Age: {}", age);
    println!("Temperature: {}°C", temp);
    println!("Population: {}", population);
    println!("Distance: {}m", distance);

    // Integer types: i8, i16, i32, i64, i128, isize
    //                u8, u16, u32, u64, u128, usize
    // Default: i32

    // Floating-point numbers
    println!("\n--- Floating Point ---");
    let pi: f64 = 3.14159265359;     // 64-bit (default)
    let e: f32 = 2.71828;             // 32-bit
    println!("π = {}", pi);
    println!("e = {}", e);

    // Numeric operations
    println!("\n--- Numeric Operations ---");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    // Boolean
    println!("\n--- Boolean ---");
    let is_rust_awesome: bool = true;
    let is_learning_fun = true; // Type inference
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is learning fun? {}", is_learning_fun);

    // Character - 4 bytes, Unicode Scalar Value
    println!("\n--- Characters ---");
    let letter: char = 'A';
    let emoji: char = '🦀';
    let chinese: char = '中';
    println!("Letter: {}", letter);
    println!("Emoji: {}", emoji);
    println!("Chinese: {}", chinese);

    // === COMPOUND TYPES ===

    // Tuples - fixed length, mixed types
    println!("\n--- Tuples ---");
    let person: (&str, u8, f64) = ("Alice", 30, 5.6);
    println!("Person tuple: {:?}", person);

    // Access by index
    let name = person.0;
    let age = person.1;
    let height = person.2;
    println!("Name: {}, Age: {}, Height: {}m", name, age, height);

    // Destructuring
    let (x, y, z) = person;
    println!("Destructured - x: {}, y: {}, z: {}", x, y, z);

    // Unit tuple (empty tuple) - represents empty value/return
    let unit: () = ();
    println!("Unit type: {:?}", unit);

    // Arrays - fixed length, same type
    println!("\n--- Arrays ---");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let first = numbers[0];
    let second = numbers[1];
    println!("Numbers: {:?}", numbers);
    println!("First: {}, Second: {}", first, second);

    // Array with same value repeated
    let threes = [3; 5]; // [3, 3, 3, 3, 3]
    println!("Threes: {:?}", threes);

    // Array length
    println!("Array length: {}", numbers.len());

    // Type annotations are optional when clear
    let inferred = [1, 2, 3]; // Type: [i32; 3]
    println!("Inferred array: {:?}", inferred);

    // Slices (we'll cover more in later phases)
    let slice = &numbers[1..3]; // [2, 3]
    println!("Slice [1..3]: {:?}", slice);
}

/*
Key Takeaways:
1. Rust is statically typed - all types known at compile time
2. Type inference is powerful but you can always be explicit
3. Integers come in signed (i) and unsigned (u) variants
4. Default integer is i32, default float is f64
5. Tuples can hold different types, arrays must be same type
6. Both tuples and arrays have fixed size at compile time
7. Use _ in numbers for readability: 1_000_000
*/

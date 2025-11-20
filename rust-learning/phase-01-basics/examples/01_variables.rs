// Phase 1: Variables and Mutability
// Demonstrates Rust's variable binding, mutability, and shadowing

fn main() {
    println!("=== Variables and Mutability ===\n");

    // Immutable variables (default)
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // ❌ This would cause a compile error!

    // Mutable variables
    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // ✅ This is allowed because y is mutable
    println!("The value of y is now: {}", y);

    // Constants - always immutable, type annotation required
    const MAX_POINTS: u32 = 100_000;
    const HOURS_IN_DAY: u8 = 24;
    println!("Max points: {}", MAX_POINTS);
    println!("Hours in a day: {}", HOURS_IN_DAY);

    // Shadowing - creating a new variable with the same name
    let z = 5;
    println!("\nShadowing example:");
    println!("z = {}", z);

    let z = z + 1; // Shadow z with new value
    println!("z = {}", z);

    {
        let z = z * 2; // Shadow z in inner scope
        println!("z in inner scope = {}", z);
    }

    println!("z back in outer scope = {}", z);

    // Shadowing allows changing type
    let spaces = "   "; // String
    let spaces = spaces.len(); // Now it's a number!
    println!("\nNumber of spaces: {}", spaces);

    // Type annotations
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Parsed guess: {}", guess);

    // Multiple variables at once
    let (a, b, c) = (1, 2, 3);
    println!("\na = {}, b = {}, c = {}", a, b, c);
}

/*
Key Takeaways:
1. Variables are immutable by default (safe by default)
2. Use `mut` to make them mutable
3. Constants are ALWAYS immutable and require type annotation
4. Shadowing lets you reuse names and change types
5. Type inference is smart, but you can always be explicit
*/

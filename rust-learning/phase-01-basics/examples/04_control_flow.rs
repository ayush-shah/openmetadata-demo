// Phase 1: Control Flow
// Demonstrates if expressions, loops, and iteration

fn main() {
    println!("=== Control Flow in Rust ===\n");

    // === IF EXPRESSIONS ===
    println!("--- If Expressions ---");

    let number = 7;

    if number < 5 {
        println!("{} is less than 5", number);
    } else if number < 10 {
        println!("{} is between 5 and 10", number);
    } else {
        println!("{} is 10 or greater", number);
    }

    // if is an expression, so it returns a value
    let condition = true;
    let value = if condition { 5 } else { 6 };
    println!("The value is: {}", value);

    // === LOOPS ===

    // loop - infinite loop
    println!("\n--- Loop ---");
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            println!("Counter reached 3!");
            break;
        }
        println!("Counter: {}", counter);
    }

    // Returning values from loops
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // Return value when breaking
        }
    };
    println!("Result from loop: {}", result);

    // while loop
    println!("\n--- While Loop ---");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop - most common and safe
    println!("\n--- For Loop ---");

    // Range
    for i in 1..=5 {
        println!("i = {}", i);
    }

    // Array iteration
    let arr = [10, 20, 30, 40, 50];
    println!("\nIterating over array:");
    for element in arr.iter() {
        println!("Element: {}", element);
    }

    // With index
    println!("\nIterating with index:");
    for (index, value) in arr.iter().enumerate() {
        println!("Index {}: Value {}", index, value);
    }

    // Reverse iteration
    println!("\nCounting down:");
    for number in (1..=5).rev() {
        println!("{}", number);
    }
    println!("Go!");

    // === LOOP LABELS ===
    println!("\n--- Loop Labels ---");

    let mut count = 0;
    'outer: loop {
        println!("Outer loop, count = {}", count);
        let mut remaining = 10;

        loop {
            println!("  Inner loop, remaining = {}", remaining);
            if remaining == 9 {
                break; // Breaks inner loop
            }
            if count == 2 {
                break 'outer; // Breaks outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Exited outer loop");

    // === BREAK and CONTINUE ===
    println!("\n--- Break and Continue ---");

    // Skip even numbers
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // Skip rest of iteration
        }
        println!("Odd number: {}", i);
    }

    // Stop at first number divisible by 7
    for i in 1..=100 {
        if i % 7 == 0 {
            println!("First number divisible by 7: {}", i);
            break;
        }
    }

    // === PRACTICAL EXAMPLES ===
    println!("\n--- Practical Examples ---");

    // FizzBuzz
    println!("\nFizzBuzz (1-15):");
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }

    // Sum of numbers
    let sum = sum_range(1, 10);
    println!("\nSum of 1 to 10: {}", sum);

    // Factorial
    let fact = factorial(5);
    println!("5! = {}", fact);

    // Find max in array
    let numbers = [3, 7, 2, 9, 1, 5];
    let max = find_max(&numbers);
    println!("Max in {:?} is {}", numbers, max);
}

// Helper functions

fn sum_range(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    for i in start..=end {
        sum += i;
    }
    sum
}

fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }
    max
}

/*
Key Takeaways:
1. `if` is an expression and can return values
2. `loop` creates infinite loops, use `break` to exit
3. `while` loops continue while condition is true
4. `for` is the most common loop, very safe
5. Ranges: 1..5 (exclusive), 1..=5 (inclusive)
6. `break` exits loops, can return values from loops
7. `continue` skips to next iteration
8. Loop labels with 'label: allow breaking outer loops
9. .iter() for iterating over collections
10. .rev() to reverse iteration
11. .enumerate() to get index and value
*/

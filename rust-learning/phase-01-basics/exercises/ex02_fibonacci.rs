// Exercise 2: Fibonacci Generator
// Generate the first n Fibonacci numbers
//
// Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21, ...
// Each number is the sum of the previous two

fn main() {
    println!("=== Fibonacci Generator ===\n");

    // Test cases
    let test_values = vec![0, 1, 5, 10, 15];

    for n in test_values {
        let fib_sequence = fibonacci(n);
        println!("First {} Fibonacci numbers: {:?}", n, fib_sequence);
    }
}

// TODO: Implement this function
// Returns a vector containing the first n Fibonacci numbers
fn fibonacci(n: u32) -> Vec<u64> {
    // Your code here
    // Hint: Handle n=0 (return empty vec) and n=1 (return vec![0])
    // For n>=2, start with vec![0, 1] and keep adding
    vec![] // Replace this
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_zero() {
        assert_eq!(fibonacci(0), vec![]);
    }

    #[test]
    fn test_fibonacci_one() {
        assert_eq!(fibonacci(1), vec![0]);
    }

    #[test]
    fn test_fibonacci_five() {
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_fibonacci_ten() {
        assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}

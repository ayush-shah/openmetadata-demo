// Exercise 3: Prime Number Checker
// Determine if a number is prime
//
// A prime number is a natural number greater than 1
// that has no positive divisors other than 1 and itself

fn main() {
    println!("=== Prime Number Checker ===\n");

    // Test cases
    let test_numbers = vec![0, 1, 2, 3, 4, 5, 10, 11, 17, 20, 97, 100];

    for num in test_numbers {
        let result = if is_prime(num) { "prime" } else { "not prime" };
        println!("{} is {}", num, result);
    }

    // Find all primes up to 50
    println!("\nAll primes up to 50:");
    let primes: Vec<u64> = (0..=50).filter(|&n| is_prime(n)).collect();
    println!("{:?}", primes);
}

// TODO: Implement this function
fn is_prime(n: u64) -> bool {
    // Your code here
    // Hints:
    // - 0 and 1 are not prime
    // - 2 is the only even prime
    // - For odd numbers, check divisibility up to sqrt(n)
    false // Replace this
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_and_one() {
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_two_and_three() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
    }

    #[test]
    fn test_small_primes() {
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
    }

    #[test]
    fn test_non_primes() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(100), false);
    }

    #[test]
    fn test_large_prime() {
        assert_eq!(is_prime(97), true);
    }
}

// Solution: Prime Number Checker

fn main() {
    println!("=== Prime Number Checker ===\n");

    let test_numbers = vec![0, 1, 2, 3, 4, 5, 10, 11, 17, 20, 97, 100];

    for num in test_numbers {
        let result = if is_prime(num) { "prime" } else { "not prime" };
        println!("{} is {}", num, result);
    }

    println!("\nAll primes up to 50:");
    let primes: Vec<u64> = (0..=50).filter(|&n| is_prime(n)).collect();
    println!("{:?}", primes);
}

fn is_prime(n: u64) -> bool {
    // Handle edge cases
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Check odd divisors up to sqrt(n)
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    true
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

// Solution: Fibonacci Generator

fn main() {
    println!("=== Fibonacci Generator ===\n");

    let test_values = vec![0, 1, 5, 10, 15];

    for n in test_values {
        let fib_sequence = fibonacci(n);
        println!("First {} Fibonacci numbers: {:?}", n, fib_sequence);
    }
}

fn fibonacci(n: u32) -> Vec<u64> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![0];
    }

    let mut sequence = vec![0, 1];

    for i in 2..n {
        let next = sequence[(i - 1) as usize] + sequence[(i - 2) as usize];
        sequence.push(next);
    }

    sequence
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

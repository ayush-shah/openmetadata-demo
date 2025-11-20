// Exercise 1: Temperature Converter
// Convert between Celsius and Fahrenheit
//
// Formulas:
// - Fahrenheit to Celsius: C = (F - 32) × 5/9
// - Celsius to Fahrenheit: F = C × 9/5 + 32

fn main() {
    println!("=== Temperature Converter ===\n");

    // Test cases
    let temps_c = vec![0.0, 100.0, -40.0, 37.0];
    let temps_f = vec![32.0, 212.0, -40.0, 98.6];

    println!("Celsius to Fahrenheit:");
    for c in temps_c {
        let f = celsius_to_fahrenheit(c);
        println!("{}°C = {}°F", c, f);
    }

    println!("\nFahrenheit to Celsius:");
    for f in temps_f {
        let c = fahrenheit_to_celsius(f);
        println!("{}°F = {}°C", f, c);
    }
}

// TODO: Implement this function
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Your code here
    0.0 // Replace this
}

// TODO: Implement this function
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // Your code here
    0.0 // Replace this
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
    }
}

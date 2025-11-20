// Solution: Temperature Converter

fn main() {
    println!("=== Temperature Converter ===\n");

    let temps_c = vec![0.0, 100.0, -40.0, 37.0];
    let temps_f = vec![32.0, 212.0, -40.0, 98.6];

    println!("Celsius to Fahrenheit:");
    for c in temps_c {
        let f = celsius_to_fahrenheit(c);
        println!("{}°C = {:.1}°F", c, f);
    }

    println!("\nFahrenheit to Celsius:");
    for f in temps_f {
        let c = fahrenheit_to_celsius(f);
        println!("{}°F = {:.1}°C", f, c);
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
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

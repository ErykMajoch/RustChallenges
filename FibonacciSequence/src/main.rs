const PHI: f64 = 1.6180339887;
const PHI_CONJ: f64 = -0.6180339887;
const SQRT5: f64 = 2.2360679775;

// This function is fast, but only accurate to the 42nd fibonacci number
fn nth_fibonacci_formula(n: i32) -> Result<u32, String> {
    if n < 0 {
        return Err(String::from("Invalid Input"));
    }
    let result = (PHI.powi(n) - PHI_CONJ.powi(n)) / SQRT5;
    Ok(result.round() as u32)
}

fn nth_fibonacci_recursive(n: i32) -> Result<u64, String> {
    if n < 0 {
        return Err(String::from("Invalid Input"));
    }

    let result = match n {
        0 => 0,
        1 => 1,
        _ => nth_fibonacci_recursive(n - 1).unwrap() + nth_fibonacci_recursive(n - 2).unwrap(),
    };

    Ok(result)
}

fn main() {
    for x in -10..=50 {
        let result = nth_fibonacci_formula(x);
        match result {
            Ok(o) => println!("{} Fibonacci Number: {}", x, o),
            Err(e) => println!("ERROR: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nth_fibonacci_recursive() {
        assert_eq!(nth_fibonacci_recursive(3).unwrap(), 2);
        assert_eq!(nth_fibonacci_recursive(6).unwrap(), 8);
        assert_eq!(nth_fibonacci_recursive(20).unwrap(), 6765);
    }

    #[test]
    fn test_nth_fibonacci_formula() {
        assert_eq!(nth_fibonacci_formula(0).unwrap(), 0);
        assert_eq!(nth_fibonacci_formula(6).unwrap(), 8);
        assert_eq!(nth_fibonacci_formula(37).unwrap(), 24157817);
        assert_eq!(nth_fibonacci_formula(42).unwrap(), 267914296);
    }
}

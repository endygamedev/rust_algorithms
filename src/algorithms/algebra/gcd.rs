//! Euclidean algorithm


/// gcd(x, y) returns greatest common divisor (GCD) between `x` and `y`
pub fn gcd(x: isize, y: isize) -> isize {
    return if y == 0 { x } else { gcd(y, x % y) };
}


#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test01() { assert_eq!(gcd(10, 4), 2); }

    #[test]
    fn test02() { assert_eq!(gcd(12, 5), 1); }

    #[test]
    fn test03() { assert_eq!(gcd(12, 12), 12); }
}
/// Algorithm for finding the greatest common divisor (GCD)
pub fn gcd(x: isize, y: isize) -> isize {
    return if y == 0 { x } else { gcd(y, x % y) };
}

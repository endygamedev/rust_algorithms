#[doc = "Алгоритм для нахождения наибольшего общего делителя (НОД)"]
pub fn gcd(x: isize, y: isize) -> isize {
	return if y == 0 {
		x
	} else {
		gcd(y, x % y)
	}
}

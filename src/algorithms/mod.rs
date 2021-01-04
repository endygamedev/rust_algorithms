pub mod sorts;
pub mod algebra;
pub mod searches;


#[cfg(test)]
mod tests {
	use crate::constructions;
	use crate::algorithms;

	/* Алгебра */

	#[test]
	fn minimal_test() {
		let vec = vec![-10, 10, 20];
		assert_eq!(algorithms::algebra::minimal(vec), -10);
	}

	#[test]
	fn gcd_test() {
		assert_eq!(algorithms::algebra::gcd(10, 4), 2);
		assert_eq!(algorithms::algebra::gcd(12, 5), 1);
		assert_eq!(algorithms::algebra::gcd(12, 12), 12);
	}

	/* Сортировки */

	#[test]
	fn bubble_sort_test() {
		let mut vec: Vec<isize> = constructions::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::bubble_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec)
	}

	#[test]
	fn insertion_sort_test() {
		let mut vec: Vec<isize> = constructions::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::insertion_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec)
	}

	#[test]
	fn selection_sort_test() {
		let mut vec: Vec<isize> = constructions::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::selection_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec)
	}

	#[test]
	fn quick_sort_test() {
		let mut vec: Vec<isize> = constructions::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::quick_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec)
	}

	#[test]
	fn merge_sort_test() {
		let mut vec: Vec<isize> = constructions::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::merge_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec)
	}

	/* Поиски */

	#[test]
	fn binary_search_test() {
		let vec: Vec<isize> = vec![1, 1, 2, 3, 4, 5];
		assert_eq!(algorithms::searches::binary_search(vec, 2), 2);
	}
}

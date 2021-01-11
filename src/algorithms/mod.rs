pub mod sorts;
pub mod algebra;
pub mod searches;
pub mod dynamic_programming;


/// Tests
#[cfg(test)]
mod tests {
	use crate::structures;
	use crate::algorithms;

	/* Algebra */

	#[test]
	fn minimal_test() {
		let vec: Vec<isize> = vec![-10, 10, 20];
		assert_eq!(algorithms::algebra::minimal(vec), -10);
	}

	#[test]
	fn maximum_test () {
		let vec: Vec<isize> = vec![-10, 10, 20];
		assert_eq!(algorithms::algebra::maximum(vec), 20);
	}

	#[test]
	fn gcd_test() {
		assert_eq!(algorithms::algebra::gcd(10, 4), 2);
		assert_eq!(algorithms::algebra::gcd(12, 5), 1);
		assert_eq!(algorithms::algebra::gcd(12, 12), 12);
	}

	/* Sorting */

	#[test]
	fn bubble_sort_test() {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::bubble_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	#[test]
	fn insertion_sort_test() {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::insertion_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	#[test]
	fn selection_sort_test() {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::selection_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	#[test]
	fn quick_sort_test() {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::quick_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	#[test]
	fn merge_sort_test() {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::merge_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	#[test]
	fn shell_sort_test() {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::shell_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	#[test]
	fn cocktail_sort_test () {
		let mut vec: Vec<isize> = structures::random_list(10, -10, 10);
		let sorted_vec: Vec<isize> = algorithms::sorts::cocktail_sort(vec.clone());
		vec.sort();
		assert_eq!(vec, sorted_vec);
	}

	/* Search */

	#[test]
	fn binary_search_test() {
		let vec: Vec<isize> = vec![1, 1, 2, 3, 4, 5];
		assert_eq!(algorithms::searches::binary_search(vec, 2), 2);
	}

	#[test]
	fn linear_search_test() {
		let vec: Vec<isize> = vec![1, 1, 2, 3, 4, 5];
		assert_eq!(algorithms::searches::linear_search((*vec).to_owned(), 2), 2);
		assert_ne!(algorithms::searches::linear_search((*vec).to_owned(), 2), 1);
	}
}

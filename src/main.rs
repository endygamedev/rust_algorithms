mod algorithms;
mod constructions;


fn main() {
	let vec: Vec<isize> = constructions::random_list(10, 1, 9);
	println!("{:?}", vec);
	let sorted: Vec<isize> = algorithms::sorts::quick_sort(vec);
	println!("{:?}", sorted);
	println!("{}", algorithms::searches::binary_search(sorted, 1));
}

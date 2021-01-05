mod algorithms;
mod constructions;


fn main() {
	let vec: Vec<isize> = constructions::random_list(10, 1, 9);
	println!("{:?}", vec);
	let sorted: Vec<isize> = algorithms::sorts::shell_sort(vec);
	println!("{:?}", sorted);
}

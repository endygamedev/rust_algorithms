mod algorithms;
mod structures;


fn main() {
	let vec: Vec<isize> = structures::list_comprehension(20, 10, 1);
	println!("{:?}", vec);
	let sorted: Vec<isize> = algorithms::sorts::cocktail_sort(vec);
	println!("{:?}", sorted);
}

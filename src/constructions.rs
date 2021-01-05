use rand::Rng;
use std::io;
mod structs;
use structs::Range;


/// Create a list of numbers from _start_ to _stop_ with _step_
pub fn list_comprehension(start: isize, stop: isize, step: isize) -> Vec<isize> {
	let range = Range::new(start, stop, step);
	let mut vec: Vec<isize> = Vec::new();
	for i in range { vec.push(i) }
	return vec
}

/// Creates a list of random numbers ranging from _min_ to _max_ with length _len_
pub fn random_list(len: isize, min: isize, max: isize) -> Vec<isize> {
	let mut vec: Vec<isize> = Vec::new();
	let mut rng = rand::thread_rng();
	for _ in 0..len { vec.push(rng.gen_range(min..max)) }
	return vec
}

/// Keyboard input
pub fn input() -> String {
	let mut input = String::new();
	return match io::stdin().read_line(&mut input) {
		Ok(_) => {
			input
		}
		Err(_) => "Input Error...".to_string()
	}
}
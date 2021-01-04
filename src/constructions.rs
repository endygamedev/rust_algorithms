use rand::Rng;
use std::io;
mod structs;
use structs::Range;

#[doc = "Создаёт список из чисел от start до stop с шагом step"]
pub fn list_comprehension(start: isize, stop: isize, step: isize) -> Vec<isize> {
	let range = Range::new(start, stop, step);
	let mut vec: Vec<isize> = Vec::new();
	for i in range { vec.push(i) }
	return vec
}

#[doc = "Создаёт список случаных чисел в диапазоне от min до max длиной len"]
pub fn random_list(len: isize, min: isize, max: isize) -> Vec<isize> {
	let mut vec: Vec<isize> = Vec::new();
	let mut rng = rand::thread_rng();
	for _ in 0..len { vec.push(rng.gen_range(min..max)) }
	return vec
}

#[doc = "Ввод данных с клавиатуры"]
pub fn input() -> String {
	let mut input = String::new();
	return match io::stdin().read_line(&mut input) {
		Ok(_) => {
			input
		}
		Err(_) => "Ошибка ввода!".to_string()
	}
}
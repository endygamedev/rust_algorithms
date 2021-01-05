/// Bubble sort
pub fn bubble_sort(list: Vec<isize>) -> Vec<isize> {
	let mut vec: Vec<isize> = list.clone();
	for _ in 0..vec.len() {
		for j in 0..vec.len()-1 {
			if vec[j+1] < vec[j] {
				vec.swap(j+1, j)
			}
		}
	}
	return vec
}

/// Insertion sort
pub fn insertion_sort(list: Vec<isize>) -> Vec<isize> {
	let mut vec: Vec<isize> = list.clone();
	let mut j: usize;
	for i in 0..vec.len() {
		j = i;
		while j > 0 && vec[j-1] > vec[j] {
			vec.swap(j-1, j);
			j -= 1;
		}
	}
	return vec
}

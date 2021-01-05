/// Returns the maximum value in an array
pub fn maximum(list: Vec<isize>) -> isize {
	let mut res: isize = list[0];
	for element in list {
		if element > res {
			res = element
		}
	}
	return res
}
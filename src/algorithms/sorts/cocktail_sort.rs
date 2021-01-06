/// Cocktail shake sort
pub fn cocktail_sort(list: Vec<isize>) -> Vec<isize> {
	let mut vec: Vec<isize> = list.clone();
	let mut left = 0;
	let mut right = vec.len()-1;
	let mut i;

	while left <= right {
		for i in left..right {
			if vec[i] > vec[i+1] {
				vec.swap(i, i+1);
			}
		}
		right -= 1;

		i = right;
		while i > left {
			if vec[i-1] > vec[i] {
				vec.swap(i-1, i);
			};
			i -= 1;
		}
		left += 1;
	}
	return vec
}
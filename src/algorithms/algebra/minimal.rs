#[doc = "Возвращает индекс минимального элемента"]
pub fn minimal(list: Vec<isize>) -> isize {
	let mut res: isize = list[0];
	for element in list {
		if res > element {
			res = element
		}
	}
	return res
}

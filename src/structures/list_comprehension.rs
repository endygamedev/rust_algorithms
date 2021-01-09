/// Create a list of numbers from _start_ to _stop_ with _step_
/// TODO: Edit case with reverse list
pub fn list_comprehension(start: isize, stop: isize, step: isize) -> Vec<isize> {
	return if start < stop {
		(start..stop).step_by(step as usize).collect::<Vec<isize>>()
	} else {
		(stop..start).rev().step_by(step.abs() as usize).collect::<Vec<isize>>()
	}
}

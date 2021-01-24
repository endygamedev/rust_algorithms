/// Create a list of numbers from _start_ to _stop_ with _step_
/// TODO: Edit case with reverse list
pub fn list_comprehension(start: isize, stop: isize, step: isize) -> Vec<isize> {
    return if start < stop {
        (start..stop).step_by(step as usize).collect::<Vec<isize>>()
    } else {
        (stop..start)
            .rev()
            .step_by(step.abs() as usize)
            .collect::<Vec<isize>>()
    };
}


#[cfg(test)]
mod tests {
    use super::list_comprehension;

    #[test]
    fn test01() {
        assert_eq!(vec![1, 3, 5, 7, 9], list_comprehension(1, 10, 2))
    }

    #[test]
    fn test02() {
        assert_eq!(vec![10, 8, 6, 4, 2, 0], list_comprehension(11, 0, 2))
    }
}
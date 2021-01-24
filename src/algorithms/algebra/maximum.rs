//! Find maximum element in list


/// maximum(list) returns the maximum value in the `list`
pub fn maximum(list: Vec<isize>) -> isize {
    let mut res: isize = list[0];
    for element in list {
        if element > res {
            res = element
        }
    }
    return res;
}


#[cfg(test)]
mod tests {
    use super::maximum;

    #[test]
    fn test01() { assert_eq!(maximum(vec![-10, 10, 20]), 20); }

    #[test]
    fn test02() { assert_eq!(maximum(vec![1, 1, 1]), 1); }

    #[test]
    fn test03() { assert_eq!(maximum(vec![1, 2, 3]), 3); }
}
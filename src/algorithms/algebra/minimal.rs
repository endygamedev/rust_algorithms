//! Find minimal element in list


/// minimal(list) returns the minimal value in the `list`
pub fn minimal(list: Vec<isize>) -> isize {
    let mut res: isize = list[0];
    for element in list {
        if element < res {
            res = element
        }
    }
    return res;
}


#[cfg(test)]
mod tests {
    use super::minimal;

    #[test]
    fn test01() { assert_eq!(minimal(vec![-10, 10, 20]), -10); }

    #[test]
    fn test02() { assert_eq!(minimal(vec![1, 1, 1]), 1); }

    #[test]
    fn test03() { assert_eq!(minimal(vec![1, 2, 3]), 1); }
}
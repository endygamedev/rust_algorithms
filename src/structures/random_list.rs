use rand::Rng;


/// Creates a list of random numbers ranging from _min_ to _max_ with length _len_
pub fn random_list(len: isize, min: isize, max: isize) -> Vec<isize> {
    let mut vec: Vec<isize> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        vec.push(rng.gen_range(min..max))
    }
    return vec;
}

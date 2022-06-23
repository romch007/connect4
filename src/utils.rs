pub fn is_all_same<T: PartialEq>(arr: &Vec<T>) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

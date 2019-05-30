pub fn find_largest_element(elems: &Vec<i64>) -> Option<i64> {

    // this will compile (with warnings), but some tests will fail.
    // replace the code below with your own solution!
    None

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_elements() {
        let empty_vec = Vec::new();

        let result = find_largest_element(&empty_vec);

        assert_eq!(result, None);
    }

    #[test]
    fn test_some_elements() {
        let myvec = vec![1, 5, 3, 8, 0];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(8));
    }
}

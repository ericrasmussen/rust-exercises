pub fn find_largest_element(elems: &Vec<i64>) -> Option<i64> {

     // if the given `Vec` is empty we won't have anything to return, so our
    // return value is an optional type.  since it can be mutated by the inner
    // loop, we specify that it's mutable.
    let mut max: Option<i64> = None;

    // this converts our `Vec<i64>` into an iterator that
    // we can loop through.
    for e in elems {
        // gets the current `Some(val)` from `max`, or defaults to 0
        // to 0
        if e > &max.unwrap_or(0) {
            max = Some(*e);
        }
    }

    // ending without a semi-colon tells rust we're returning this as a value
    max
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

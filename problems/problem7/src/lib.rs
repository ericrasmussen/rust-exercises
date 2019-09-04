// our `Set` type holds no elements, but can tell us whether or not
// a given `i64` is a member.
type Set = Box<dyn Fn(i64) -> bool>;

// create a new set from one `i64`
pub fn singleton(x: i64) -> Set {
    Box::new(move |y| y == 1)
}

// tests for membership in the given set
pub fn contains(s: &Set, x: i64) -> bool {
    x == 1
}

// combines two sets
pub fn union(s1: Set, s2: Set) -> Set {
    Box::new(move |x| x == 1)
}

// creates a new set containing only elements from `s1` and `s2`
pub fn intersect(s1: Set, s2: Set) -> Set {
    Box::new(move |x| x == 1)
}

// creates a new set containing elements from `s1` that aren't part of `s2`
pub fn diff(s1: Set, s2: Set) -> Set {
    Box::new(move |x| x == 1)
}

// filters out any elements in the set that don't match the predicate `p`
pub fn filter(s: Set, p: fn(i64) -> bool) -> Set {
    Box::new(move |x| x == 1)
}

// checks if predicate `p` holds true for elements in the set. you can
// limit the search space to the range `(-1000..1000)`
pub fn forall(s: Set, p: Box<dyn Fn(i64) -> bool>) -> bool {
    true
}

// tests if any element of the set exists such that predicate `p` holds
// true. this should be implemented to use `forall`
pub fn exists(s: Set, p: Box<dyn Fn(i64) -> bool>) -> bool {
    true
}

// we want to implement `map` using `exists`, but it rqeuires nested closures, which
// are difficult in rust. you can also implement it directly using `(-1000..1000)`
pub fn map(s: Set, f: fn(i64) -> i64) -> Set {
    Box::new(move |x| x == 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_singleton() {
        let five = singleton(5);
        assert!(five(5));
    }

    #[test]
    fn test_contains() {
        let positive: Set = Box::new(|x| x >= 0);
        assert!(contains(&positive, 5));
        assert!( ! contains(&positive, -5));
    }

    #[test]
    fn test_union() {
        let positive: Set = Box::new(|x| x >= 0);
        let negative: Set = Box::new(|x| x < 0);
        let combined = union(positive, negative);
        assert!(contains(&combined, 5));
        assert!(contains(&combined, -5));

    }

    #[test]
    fn test_intersect() {
        let positive: Set = Box::new(|x| x >= 0);
        let kind_of_big: Set = Box::new(|x| x > 1_000_000);
        let combined = intersect(positive, kind_of_big);
        assert!(contains(&combined, 1_000_001));
        assert!( ! contains(&combined, 10));

    }

    #[test]
    fn test_difference() {
        let positive: Set = Box::new(|x| x >= 0);
        let kind_of_big: Set = Box::new(|x| x > 1_000_000);
        let positive_and_not_so_big = diff(positive, kind_of_big);
        assert!( ! contains(&positive_and_not_so_big, 1_000_001));
        assert!(contains(&positive_and_not_so_big, 10));

    }

    #[test]
    fn test_filter() {
        let positive: Set = Box::new(|x| x >= 0);
        fn is_even(x: i64) -> bool {
            x % 2 == 0
        }
        let positive_and_even = filter(positive, is_even);
        assert!(contains(&positive_and_even, 80));
        assert!( ! contains(&positive_and_even, 81));

    }

    #[test]
    fn test_forall() {
        let positive: Set = Box::new(|x| x >= 0);
        let is_positive = Box::new(move |x| x >= 0);
        assert!(forall(positive, is_positive));
    }


    #[test]
    fn test_not_forall() {
        let positive: Set = Box::new(|x| x >= 0);
        let equals_ten = Box::new(move |x| x == 10);
        assert!( ! forall(positive, equals_ten) );
    }

    #[test]
    fn test_exists() {
        let positive: Set = Box::new(|x| x >= 0);
        let negative: Set = Box::new(|x| x < 0);
        let bigger_than_10 = Box::new(move |x| x > 10);

        assert!(exists(positive, bigger_than_10.clone()));

        assert!( ! exists(negative, bigger_than_10.clone()) );
    }

    #[test]
    fn test_map() {
        let one_to_ten: Set = Box::new(|x| x >= 1 && x <= 10);
        // takes our set of [1, 2, 3, 4, ..., 10] and creates a set of
        // [100, 200, 300, 400, ..., 1000]
        let one_hundred_to_one_thousand = map(one_to_ten, |i| i * 100);
        assert!(contains(&one_hundred_to_one_thousand, 100));
        assert!(contains(&one_hundred_to_one_thousand, 200));
        assert!(contains(&one_hundred_to_one_thousand, 1000));
        assert!( ! contains(&one_hundred_to_one_thousand, 10));

    }
}

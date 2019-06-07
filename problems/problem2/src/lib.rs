pub fn sum_one_to_n(n: u32) -> u32 {
    // your code for summing all digits from 1 to `n` (inclusive) should go
    // here (you can remove the sample return of `0`)
    0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_0() {
        let result = sum_one_to_n(0);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_1() {
        let result = sum_one_to_n(1);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_sum_100() {
        let result = sum_one_to_n(100);

        assert_eq!(result, 5050);
    }
}

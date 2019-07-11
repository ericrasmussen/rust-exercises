use std::cmp;
use std::vec::Vec;
use std::str;

/// Takes a vector of string slices and formats them
/// line by line in a flower box, e.g.
/// vec!["one", "two", "three"] becomes:
///
/// *********
/// * one   *
/// * two   *
/// * three *
/// *********
///
pub fn make_flower_box(elems: Vec<&str>) -> String {

    let mut flower_box = String::new();

    flower_box

}

/// Creates a string with a border and space padding.
///
/// Examples
/// ```
/// use problem4::format_line;
/// let actual = format_line("hello", 10);
/// let expected = String::from("* hello      *\n");
/// assert_eq!(actual, expected)
/// ```
pub fn format_line(s: &str, max_length: usize) -> String {
    String::from("* *")
}

/// Creates a border that can be used at the top and bottom
/// of our flower box.
///
/// Examples
/// ```
/// use problem4::format_border;
/// let actual = format_border("*", 5);
/// let expected = String::from("*****\n");
/// assert_eq!(actual, expected)
/// ```
pub fn format_border(s: &str, length: usize) -> String {
    String::from("******\n")
}

/// Gets the longest line length from `elems`.
///
/// Examples
/// ```
/// use problem4::get_max_line_length;
/// let elems = vec!["fizz", "buzz", "fizzbuzz"];
/// assert_eq!(get_max_line_length(&elems), 8);
/// ```
pub fn get_max_line_length(elems: &Vec<&str>) -> usize {
    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_box() {
        let expected = String::from("");
        let empty_vec: Vec<&str> = Vec::new();

        assert_eq!(expected, make_flower_box(empty_vec));
    }

    #[test]
    fn test_box() {
        let expected = r####"*********
* one   *
* two   *
* three *
* four  *
* five  *
* six   *
* seven *
*********
"####.to_string();

        let test_vec = vec!["one", "two", "three", "four", "five", "six", "seven"];

        assert_eq!(expected, make_flower_box(test_vec));
    }


}

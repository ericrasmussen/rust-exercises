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

    let max_length = get_max_line_length(&elems);
    if max_length == 0 {
        return String::from("");
    }

    let mut flower_box = String::new();

    // create border adding padding for the sides
    let border = format_border("*", max_length + 4);
    flower_box.push_str(&border);

    for e in elems {
        flower_box.push_str(&format_line(e, max_length));
    }

    flower_box.push_str(&border);

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
    if max_length == 0 {
        return String::from("");
    }

    let mut line: String = String::from("*");

    // Add a padding space
    line.push_str(" ");

    // Add vector element
    line.push_str(&s);

    // Add spaces to equal the longest
    for _ in s.len()..max_length + 1 {
    //for _ in 0..max_length {
        line.push_str(" ");
    }

    // Add ending border plus new line
    line.push_str("*\n");
    line

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
    if length == 0 {
        return String::from("");
    }

    let mut border: String = String::from("");

    // create border using the character that was passed in
    for _ in 0..length {
        border.push_str(s);
    }

    // Add newline
    border.push_str("\n");
    border
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
    if elems.len() == 0 {
        return 0;
    }

    // loop through vector to find the maximum length
    let mut max_length = 0;
    for e in elems {
        if e.len() > max_length {
            max_length = e.len();
        }
    }
    max_length
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

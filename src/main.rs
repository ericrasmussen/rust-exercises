extern crate exercises;

use exercises::exercise1::find_largest_element;


fn main() {

    let example_vec = vec![1, 5, 3];

    // exercise 1: find the largest element in a list
    let largest = find_largest_element(&example_vec);
    let largest_output = format!("Hooray! I found the number {}", largest.unwrap());
    println!("{}", largest_output);
}


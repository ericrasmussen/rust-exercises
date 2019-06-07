use std::io;
use problem2::sum_one_to_n;

fn main() {

    loop {
        println!("Give me a number, any number!");

        // this variable can be used to store the string input from the user
        let mut number = String::new();

        // this should be replaced with code that tries to parse `number`
        // into a value of `u32`, or otherwise go back to the beginning of the
        // loop
        let number: u32 = 0;

        // make sure to modify `sum_one_to_n` in `libr.rs`. the stub
        // implementation included with this problem always returns 0.
        let result = sum_one_to_n(number);

        println!("The sum of 1 to {} is: {}", number, result);

        // this is here to prevent an infinite loop with the sample code above,
        // but can be removed once your program is working
        break;
    }
}

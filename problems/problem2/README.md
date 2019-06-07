# Problem 2

`problem2` has two main components:

## lib.rs

The library part of our program contains a broken function named `sum_one_to_n`,
which should take any `u32` value `n` and return the sum of 1 to n (inclusive).
`sum_one_to_n(0)` should return `0`.

After fixing this function, run your code with: `cargo test --package problem2`


## main.rs

Our main function should prompt a user for a number, call `sum_one_to_n` with
that number, and print the result. If the program cannot parse the input
successfully, it should continue looping until it receives valid input.

You can run your program with: `cargo run --package problem2`

Note that the main loop has a `break` statement to avoid looping infinitely over
the stub code included with the problem. You can leave it there if you want your
program to exit on the first successful case (the first time it's able to parse
a valid `u32` value from the user input). You can also remove it if you want it
to loop infinitely (on most systems you can exit manually with ctrl+c).


## Docs relevant to this problem

* https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
* https://doc.rust-lang.org/std/ops/struct.Range.html


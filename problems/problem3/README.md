# Problem 3

`problem3` is a rock, paper, scissors game. There is an enum `GameElement`
type to represent the different game choices.

## game_element.rs

Commonly in rust libraries we need to be able parse a string into a custom
type we've defined and offer some way for users of our library to represent
our type as a string.

For rock, paper, scissors, we also need a way to compare the items to each other
to decide if the player wins, loses, or ties with the computer.

The `game_element.rs` module has broken implementations for the following:

* `fn cmp(&self, other: &Self) -> Ordering`
* `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`
* `fn from_str(s: &str) -> Result<Self, Self::Err>`

Additional comments and information can be found in the codebase. Or it can
be accessed and browsed via the documentation. See `cargo doc` below.

## cargo doc

Unlike previous problems, the implementations for the above functions are all
very broken, and there are not enough tests to cover all the cases.

We can take advantage of `cargo doc` to generate documentation for the
codebase and read how the implementations are supposed to work:

```
cargo doc --package problem3 --open
```

You can browse to the `game_element` module, view the `GameElement` type,
and read the implementations for `Ord`, `FromStr`, and `Display`.

This is a small example program and you could get by with reading
the comments in the code. However, the generated API docs have an arguably
nicer format, and you can click on traits, parameters, and return types for
your library *and* all of its dependencies.

If you find yourself trying to use a function with the signature
`fn whoknows(a: ArgA, b: ArgB) -> ReturnType`, you can click on `ArgA` to read
its docs. Even if the docs aren't helpful, they will include the source
code. One way or another you'll be able to see how to construct a value of type
`ArgA` so that you can use the `whoknows` function.

If you have the type signatures referencing something that doesn't exist, or
if there's a typo, no worries! Running `cargo doc` requires your program
to compile, which means the compiler knows unambiguously where each type or
trait comes from.


## Running the game

You can try the program out any time with:

```
cargo run --package problem3
```

But remember that it will behave oddly before you fix the code.

The game can be exited by pressing Ctrl+C (or closing the terminal window).


## Docs relevant to this problem

* https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html



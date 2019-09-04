# Problem 7

This problem attempts to define mathematical sets in terms of their characteristic
function, the membership test.

Note: this is based in part on exercises that have been done in scala before, including
https://github.com/ardlema/scala-higher-order-functions

The main goal of the exercise is to look at sets in terms of membership. Most often
if we have a set to work with, we want to know if some item we have belongs to it.
Instead of storing all possible items and then checking, the set itself can be defined as
a boxed function: `type Set = Box<dyn Fn(i64) -> bool>;`

This lets us now define the set of all positive numbers as:
`let positive: Set = Box::new(|x| x >= 0);`

## lib.rs

Now that we have our `Set` type, and we can create new sets, we need to implement
common set operations that will return new functions to test for membership.

Here is an example of the `union` operation combining all positive and negative
integers to make a new set:

```
let positive: Set = Box::new(|x| x >= 0);
let negative: Set = Box::new(|x| x < 0);
let combined = union(positive, negative);
```

Additional examples are available in the tests.

For the definitions of `forall`, `exists`, and `map`, we would need to
know every possible member of the set. To limit the scope for these, they
can be limited to the range `(-1000..1000)`.


## testing

You can run the tests with:

```
cargo test --package problem7
```

# Problem 8

This is an exercise to help practice sending and receiving messages via threads
over a channel.

We will run a simulated internet cafe where we have more visitors than
computers.  Each visitor will take the next available computer and spend some
random amount of time on it before leaving.

We can simulate this by launching a thread every time we allocate a computer
to a visitor. When the visit is over, the thread will send a message summarizing
the visit. We can then free up a computer for use by any remaining visitors.


## visitor.rs

You won't need to modify `visitor::Visitor` for this exercise, but check the
source code to see how groups of visitors are generated. You can inspect
the `visit_start`, `visit`, and `visit_summary` methods that will be used by our
cafe.


## cafe.rs

This is our internet cafe in all of its glory. To make the program work, you
will need to fill in the `allocate_computer` and `run_simulation` methods.


## expected output

The visitors will be generated randomly (with some hard-coded constraints),
and since they will spend different amounts of time on the computer with each
run, you should see different output each time.

For our simulation, every 10 minutes will only take 1 second. Users can take up
to 120 minutes, so you'll see some users won't leave for up to 12 seconds, while
others come and go in-between.

Here's one example run with 20 visitors and 10 computers. The first 10 visitors
get computers immediately. Everyone else needs to jump in as the computers
are freed up.

```
Visitor 1 is online!
Visitor 2 is online!
Visitor 3 is online!
Visitor 4 is online!
Visitor 5 is online!
Visitor 6 is online!
Visitor 7 is online!
Visitor 8 is online!
Visitor 9 is online!
Visitor 10 is online!
Visitor 7 spent 10 minutes online.
Visitor 11 is online!
Visitor 5 spent 18 minutes online.
Visitor 12 is online!
Visitor 8 spent 40 minutes online.
Visitor 13 is online!
Visitor 11 spent 32 minutes online.
Visitor 14 is online!
Visitor 4 spent 50 minutes online.
Visitor 15 is online!
Visitor 6 spent 52 minutes online.
Visitor 16 is online!
Visitor 12 spent 41 minutes online.
Visitor 17 is online!
Visitor 9 spent 88 minutes online.
Visitor 18 is online!
Visitor 10 spent 86 minutes online.
Visitor 19 is online!
Visitor 14 spent 42 minutes online.
Visitor 20 is online!
Visitor 1 spent 94 minutes online.
Visitor 2 spent 99 minutes online.
Visitor 18 spent 14 minutes online.
Visitor 17 spent 44 minutes online.
Visitor 3 spent 103 minutes online.
Visitor 20 spent 25 minutes online.
Visitor 16 spent 69 minutes online.
Visitor 13 spent 80 minutes online.
Visitor 15 spent 109 minutes online.
Visitor 19 spent 79 minutes online.
```

Make sure your output is similar. You don't want to see repeat visitors or
anyone using a computer before it's available.

## testing

There's only one integration test included, and you can run it with:

```
cargo test --package problem8
```

It uses the `test_bin` crate to find the binary cargo will compile for testing,
and then runs it and inspects the output.

We could also remove explicit calls to `println!` and use a generic writer, but
the other problems in this set have plenty of unit test examples. This is our
first example where we capture and inspect output from a program.


## resources

Concurrency in rust: https://doc.rust-lang.org/book/ch16-00-concurrency.html

Inspiration for this problem: http://whipperstacker.com/2015/10/05/3-trivial-concurrency-exercises-for-the-confused-newbie-gopher/


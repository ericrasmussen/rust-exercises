use std::vec::Vec;
use std::thread;
use std::sync::mpsc;
use crate::visitor::Visitor;


/// Our little internet cafe, which is a group of visitors and some number of
/// computers. For this exercise there's no need to allow visitors to join
/// throughout the day (or storm out angrily after a long wait). We'll focus
/// only on serving the group we start with.
#[derive(Debug)]
pub struct Cafe {
    visitors: Vec<Visitor>,
    available_computers: u32
}

impl Cafe {

    /// Creates a brand new internet cafe in no time at all. Amazing!
    pub fn new(visitors: Vec<Visitor>, available_computers: u32) -> Self {

        Cafe {
            visitors: visitors,
            available_computers: available_computers,
        }
    }

    /// This function should create our message channel, decide when there
    /// is a visitor and a free computer, then make use of
    /// `self.allocate_computer` (check its type signature for hints).
    /// It will also need to use `self.handle_msg` in two places (check its
    /// comments for another hint).
    pub fn run_simulation(mut self) {

        println!("There's no simulation here yet... good luck!");
    }

    /// Here we need to go through all the steps of announcing a visitor, giving
    /// them a computer, letting them visit for however long they want, and then
    /// sending a summary of their visit to our channel to indicate when they're
    /// all done. Check `visitor.rs` to see what methods you have available to
    /// you.
    fn allocate_computer(&mut self, v: Visitor, sender: mpsc::Sender<String>) {
        println!("Fix me! I'm not allocating anything right now.");
    }

    /// We have to be prepared to receive messages at different times (while
    /// some visitors are still waiting, and after all computers are allocated
    /// but might still be in use). This helper function will print the summary
    /// and make a computer available again.
    fn handle_msg(&mut self, msg: String) {
        println!("{}", msg);
        self.available_computers = self.available_computers + 1;
    }

}

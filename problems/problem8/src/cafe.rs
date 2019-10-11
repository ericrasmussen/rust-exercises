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

    /// This function should create our message channel and launch one thread
    /// per visit as long as there is an available computer.
    /// It should make use of `self.allocate_computer` (check its type signature
    /// for hints) and `self.handle_msg` (check the comments for another hint).
    pub fn run_simulation(mut self) {

        let (sender, receiver) = mpsc::channel();

        // this solution uses pop to modify the visitors vector in place. we can
        // stop looping when that vector is empty, because at that point we
        // don't need to create any more threads.
        while !self.visitors.is_empty() {

            if self.available_computers > 0 {
                let visitor = self.visitors.pop().unwrap();
                let message_sender = mpsc::Sender::clone(&sender);
                self.allocate_computer(visitor, message_sender);
            }

            // while we're handling all incoming visitors, some visitors will
            // let us know they're all done. it's important to handle these
            // incoming message as part of our loop.
            if let Ok(m) = receiver.try_recv() {
                self.handle_msg(m.to_string());
            }
        }

        // once our loop is finished, we do not need to send more messages.
        // this closes the sending/transmitting half of our channel.
        drop(sender);

        // now that the sender is dropped, we can receive all of the remaining
        // messages
        for msg in receiver {
            &self.handle_msg(msg.to_string());
        }

    }

    /// Here we need to go through all the steps of announcing a visitor, giving
    /// them a computer, letting them visit for however long they want, and then
    /// sending a summary of their visit to our channel to indicate when they're
    /// all done. Check `visitor.rs` to see what methods you have available to
    /// you.
    fn allocate_computer(&mut self, v: Visitor, sender: mpsc::Sender<String>) {
        println!("{}", &v.visit_start());

        let msg = v.visit_summary();

        thread::spawn(move || {
            v.visit();
            sender.send(msg).unwrap();
        });

        self.available_computers -= 1;

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

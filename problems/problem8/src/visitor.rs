use std::thread;
use std::vec::Vec;
use std::time::Duration;


/// A visitor in our internet cafe. We assign them an id and some amount of time
/// to visit.
#[derive(Debug)]
pub struct Visitor {
    id: u32,
    minutes_per_visit: u32
}


impl Visitor {

    /// Creates a new visitor
    pub fn new(id: u32, minutes_per_visit: u32) -> Self {
        Visitor { id: id, minutes_per_visit: minutes_per_visit }
    }

    /// A message announcing the start of this visit
    pub fn visit_start(&self) -> String {
        format!("Visitor {} is online!", &self.id)
    }

    /// A summary of the visit
    pub fn visit_summary(&self) -> String {
        format!("Visitor {} spent {} minutes online.",
                &self.id,
                &self.minutes_per_visit,
        )
    }

    /// This is our simulated visit that will run for some amount of time.
    /// To make the simulation faster, we treat minutes as seconds and then
    /// divide by 10. (e.g. 110 minutes would take 11 seconds for our program).
    pub fn visit(&self) {
        let wait_time: u64 = self.minutes_per_visit as u64 / 10;
        thread::sleep(Duration::from_secs(wait_time));
    }


    /// This is a helper method to generate some number of visitors
    pub fn generate_visitors(num_visitors: u32) -> Vec<Visitor> {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();

        let mut visitors = Vec::new();

        for id in 1..=num_visitors {
            let minutes_per_visit: u32 = rng.gen_range(10, 120);
            let visitor = Self::new(id, minutes_per_visit);
            visitors.push(visitor);
        }

        // Putting the visitors in reverse order makes it nicer to use `.pop()`
        // elsewhere, but your cafe can use any strategy it wants for obtaining
        // the next visitor.
        visitors.reverse();

        visitors
    }
}

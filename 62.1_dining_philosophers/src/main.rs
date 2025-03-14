/*
Five philosophers dine together at the same table.
Each philosopher has their own place at the table.
There is a chopstick between each plate.
The dish served is spaghetti which requires two chopsticks to eat.
Each philosopher can only alternately think and eat.
Moreover, a philosopher can only eat their spaghetti when they have both a left and right chopstick.
Thus two chopsticks will only be available when their two nearest neighbors are thinking, not eating.
After an individual philosopher finishes eating, they will put down both chopsticks.
*/

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// zero-field struct stored in Arc<Mutex>
struct Chopstick;

struct Philosopher {
	name: String,
	left_chopstick: Arc<Mutex<Chopstick>>,
	right_chopstick: Arc<Mutex<Chopstick>>,
	thoughts: mpsc::SyncSender<String>, // could this be Sender instead?
}

impl Philosopher {
	fn think(&self) {
		self.thoughts
			.send(format!("Eureka! {} has a new idea!", &self.name))
			.unwrap();
	}

	fn eat(&self) {
		// Pick up chopsticks...
		println!("{} is eating...", &self.name);
		thread::sleep(Duration::from_millis(10));
	}
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
	// Create chopsticks: start with left, right is borrowed
	let chopsticks: VecDeque<_> = PHILOSOPHERS
		.iter()
		.map(|_| Arc::new(Mutex::new(Chopstick)))
		.collect();

	// Create philosophers

	// Make each of them think and eat 100 times

	// Output their thoughts
}

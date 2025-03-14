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
	id: usize,
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
		println!("{} looking for chopsticks...", &self.name);
		if self.id % 2 == 0 {
			let _left = self.left_chopstick.lock().unwrap();
			let _right = self.right_chopstick.lock().unwrap();
			println!("{} is eating...", &self.name);
		} else {
			let _right = self.right_chopstick.lock().unwrap();
			let _left = self.left_chopstick.lock().unwrap();
			println!("{} is eating...", &self.name);
		}
		thread::sleep(Duration::from_millis(10));
	}
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
	let (tx, rx): (mpsc::SyncSender<String>, mpsc::Receiver<String>) = mpsc::sync_channel(10);
	// Create chopsticks: start with left, right is borrowed
	let initial_chopsticks: VecDeque<_> = PHILOSOPHERS
		.iter()
		.map(|_| Arc::new(Mutex::new(Chopstick)))
		.collect();

	let zipped_philos_chopsticks = PHILOSOPHERS
		.iter()
		.zip(initial_chopsticks.iter().enumerate());
	// Create philosophers, looping over chopsticks
	let philosophers: Vec<_> = zipped_philos_chopsticks
		.map(|(&philo_name, chopstick)| {
			// create philosopher instance
			Philosopher {
				id: chopstick.0,
				name: philo_name.to_string(),
				// assign chopsticks
				left_chopstick: chopstick.1.clone(),
				right_chopstick: initial_chopsticks[(chopstick.0 + 1) % initial_chopsticks.len()]
					.clone(),
				// clone sender for philosophers' thoughts
				thoughts: tx.clone(),
			}
		})
		.collect();

	// run philosopher routine in thread
	let philo_threads: Vec<_> = philosophers
		.into_iter()
		.enumerate()
		.map(|(id, philosopher)| {
			thread::spawn(move || {
				// Make each of them think and eat 100 times
				if id % 2 == 0 {
					// this is not really required:
					// eat() checks for chopstick availability and we do not use sleep
					philosopher.think();
				}
				for _ in 0..100 {
					philosopher.eat();
					philosopher.think();
				}
			})
		})
		.collect();

	// exercise solution cheats, use other way instead
	// COPY-START: loop
	// To avoid a deadlock, we have to break the symmetry
	// somewhere. This will swap the chopsticks without deinitializing
	// either of them.
	// if i == chopsticks.len() - 1 {
	// 	std::mem::swap(&mut left_chopstick, &mut right_chopstick);
	// }
	// COPY-END: loop

	// COPY-START
	drop(tx);
	// Output their thoughts
	for thought in rx {
		println!("{thought}");
	}
	// COPY-END

	// check for errors
	for thread in philo_threads {
		let _ = thread.join().expect("could not join thread");
	}
}

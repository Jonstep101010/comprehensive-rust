use std::sync::Arc;
// use tokio types instead of std
use tokio::sync::{Mutex, mpsc};
use tokio::time;

struct Chopstick;

struct Philosopher {
	id: usize,
	name: String,
	// tokio Mutex
	left_chopstick: Arc<Mutex<Chopstick>>,
	right_chopstick: Arc<Mutex<Chopstick>>,
	// Sender for async
	thoughts: mpsc::Sender<String>,
}

// use await where needed
impl Philosopher {
	async fn think(&self) {
		self.thoughts
			.send(format!("Eureka! {} has a new idea!", &self.name))
			.await
			.unwrap();
	}

	async fn eat(&self) {
		// Keep trying until we have both chopsticks
		println!("{} looking for chopsticks...", &self.name);
		if self.id % 2 == 0 {
			let _left = self.left_chopstick.lock().await;
			let _right = self.right_chopstick.lock().await;
			println!("{} is eating...", &self.name);
		} else {
			let _right = self.right_chopstick.lock().await;
			let _left = self.left_chopstick.lock().await;
			println!("{} is eating...", &self.name);
		}
		time::sleep(time::Duration::from_millis(5)).await;
	}
}

// tokio scheduler doesn't deadlock with 5 philosophers, so have 2.
static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia"];

#[tokio::main]
async fn main() {
	let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);
	// Create chopsticks
	let initial_chopsticks: Vec<_> = PHILOSOPHERS
		.iter()
		.map(|_| Arc::new(Mutex::new(Chopstick)))
		.collect();

	// Create philosophers
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
				left_chopstick: Arc::clone(chopstick.1),
				right_chopstick: Arc::clone(
					&initial_chopsticks[(chopstick.0 + 1) % initial_chopsticks.len()],
				),
				// clone sender for philosophers' thoughts
				thoughts: tx.clone(),
			}
		})
		.collect();

	// to avoid deadlock
	drop(tx);
	// run philosopher routine in thread
	// this could be a simple for loop (not awaiting handles)
	// for phil in philosophers {
	// 	tokio::spawn(async move {
	// 		for _ in 0..100 {
	// 			phil.think().await;
	// 			phil.eat().await;
	// 		}
	// 	});
	// }
	let handles: Vec<_> = philosophers
		.into_iter()
		.map(|phil| {
			tokio::spawn(async move {
				for _ in 0..100 {
					phil.think().await;
					phil.eat().await;
				}
			})
		})
		.collect();

	// Output their thoughts
	while let Some(thought) = rx.recv().await {
		println!("Here is a thought: {thought}");
	}

	for handle in handles {
		let _ = handle.await;
	}
}

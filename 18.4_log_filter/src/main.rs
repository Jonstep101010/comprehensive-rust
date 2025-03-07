/*
Building on the generic logger from this morning, implement a Filter which uses a closure to filter log messages,
 sending those which pass the filtering predicate to an inner logger.
 */

pub trait Logger {
	/// Log a message at the given verbosity level.
	fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
	fn log(&self, verbosity: u8, message: &str) {
		eprintln!("verbosity={verbosity}: {message}");
	}
}

/// only send messages passing the filtering predicate to inner
struct Filter<L, P> {
	inner: L,
	predicate: P,
}

// where
//	L: Logger,
//	P: Fn(u8, &str) -> bool,

impl<L, P> Filter<L, P>
where
	/* required for call to know type (error[E0282]: type annotations needed) */
	L: Logger,
	P: Fn(u8, &str) -> bool,
{
	fn new(inner: L, predicate: P) -> Self {
		Self { inner, predicate }
	}
}
impl<L, P> Logger for Filter<L, P>
where
	L: Logger,
	P: Fn(u8, &str) -> bool,
{
	fn log(&self, verbosity: u8, message: &str) {
		// @note closure requires parenthesis
		if (self.predicate)(verbosity, message) {
			self.inner.log(verbosity, message);
		}
	}
}

// TODO: Define and implement `Filter`.

fn main() {
	let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
	logger.log(5, "FYI");
	logger.log(1, "yikes, something went wrong");
	logger.log(2, "uhoh");
}

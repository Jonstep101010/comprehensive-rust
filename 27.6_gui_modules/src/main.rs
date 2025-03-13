/*
In this exercise, you will reorganize a small GUI Library implementation. This library defines a Widget trait and a few implementations of that trait, as well as a main function.

It is typical to put each type or set of closely-related types into its own module, so each widget type should get its own module.
*/

pub trait Widget {
	/// Natural width of `self`.
	fn width(&self) -> usize;

	/// Draw the widget into a buffer.
	fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

	/// Draw the widget on standard output.
	fn draw(&self) {
		let mut buffer = String::new();
		self.draw_into(&mut buffer);
		println!("{buffer}");
	}
}

mod widgets;

fn main() {
	let mut window = widgets::Window::new("Rust GUI Demo 1.23");
	window.add_widget(Box::new(widgets::Label::new(
		"This is a small text GUI demo.",
	)));
	window.add_widget(Box::new(widgets::Button::new("Click me!")));
	window.draw();
}

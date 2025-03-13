/*
In this exercise, you will reorganize a small GUI Library implementation. This library defines a Widget trait and a few implementations of that trait, as well as a main function.

It is typical to put each type or set of closely-related types into its own module, so each widget type should get its own module.
*/

mod widgets;

use widgets::{Button, Label, Widget, Window};

fn main() {
	let mut window = Window::new("Rust GUI Demo 1.23");
	window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
	window.add_widget(Box::new(Button::new("Click me!")));
	window.draw();
}

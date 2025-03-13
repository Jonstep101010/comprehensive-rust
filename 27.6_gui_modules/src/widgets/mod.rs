use super::Widget;

pub struct Label {
	pub(crate) label: String,
}

impl Label {
	pub(crate) fn new(label: &str) -> Label {
		Label {
			label: label.to_owned(),
		}
	}
}

pub struct Button {
	pub(crate) label: Label,
}

impl Button {
	pub(crate) fn new(label: &str) -> Button {
		Button {
			label: Label::new(label),
		}
	}
}

pub struct Window {
	pub(crate) title: String,
	pub(crate) widgets: Vec<Box<dyn Widget>>,
}

impl Window {
	pub(crate) fn new(title: &str) -> Window {
		Window {
			title: title.to_owned(),
			widgets: Vec::new(),
		}
	}

	pub(crate) fn add_widget(&mut self, widget: Box<dyn Widget>) {
		self.widgets.push(widget);
	}

	pub(crate) fn inner_width(&self) -> usize {
		std::cmp::max(
			self.title.chars().count(),
			self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
		)
	}
}

impl Widget for Window {
	fn width(&self) -> usize {
		// Add 4 paddings for borders
		self.inner_width() + 4
	}

	fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
		let mut inner = String::new();
		for widget in &self.widgets {
			widget.draw_into(&mut inner);
		}

		let inner_width = self.inner_width();

		// TODO: Change draw_into to return Result<(), std::fmt::Error>. Then use the
		// ?-operator here instead of .unwrap().
		writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
		writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap();
		writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();
		for line in inner.lines() {
			writeln!(buffer, "| {:inner_width$} |", line).unwrap();
		}
		writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
	}
}

impl Widget for Button {
	fn width(&self) -> usize {
		self.label.width() + 8 // add a bit of padding
	}

	fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
		let width = self.width();
		let mut label = String::new();
		self.label.draw_into(&mut label);

		writeln!(buffer, "+{:-<width$}+", "").unwrap();
		for line in label.lines() {
			writeln!(buffer, "|{:^width$}|", &line).unwrap();
		}
		writeln!(buffer, "+{:-<width$}+", "").unwrap();
	}
}

impl Widget for Label {
	fn width(&self) -> usize {
		self.label
			.lines()
			.map(|line| line.chars().count())
			.max()
			.unwrap_or(0)
	}

	fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
		writeln!(buffer, "{}", &self.label).unwrap();
	}
}

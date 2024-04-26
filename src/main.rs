use iced::widget::{button, column, text, Column};

#[derive(Clone,Default,Debug)]
struct Counter {
	value: i64,
}

#[derive(Clone,Debug)]
enum Message {
	Increment,
	Decrement,
}

struct ControlFlags {
	decrement: bool,
	high_low: bool,
	increment: bool,
	input_enable: bool,
	output_enable: bool,
	read_write: bool,
	select: u64, // Possibly up to 2 or three lines?

	// SE?
	// R?
	// CN?
}
//clockPhase: ?,

#[derive(Clone,Debug)]
struct Register {
	width: i64,
	value: i64,

}

impl Counter {
	fn update(&mut self, message: Message) {
		match message {
			Message::Increment => {
				self.value += 1;
			}
			Message::Decrement => {
				self.value -= 1;
			}
		}
	}

	fn view(&self) -> Column<Message> {
		let increment = button("+").on_press(Message::Increment);
		let decrement = button("-").on_press(Message::Decrement);

		let counter = text(self.value);

		let interface = column![increment, counter, decrement];

		interface
	}
}

fn main() {
	let _ = iced::run("A cool counter", Counter::update, Counter::view);
}

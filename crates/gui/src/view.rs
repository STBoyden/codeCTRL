use iced::{Command, Element};

pub trait View {
	type Message;

	fn title(&self) -> String { String::new() }
	fn update(&mut self, _message: Self::Message) -> Command<Self::Message> { Command::none() }
	fn view(&self) -> Element<'_, Self::Message>;

	fn send_message(&self, message: Self::Message) -> Command<Self::Message>
	where
		Self::Message: 'static + Send,
	{
		Command::perform(async {}, |()| message)
	}
}

pub trait ViewBorrowed {
	type Message;

	fn title(&self) -> String { String::new() }
	fn update(&mut self, _message: Self::Message) -> Command<Self::Message> { Command::none() }
	fn view<'a>(&self) -> Element<'a, Self::Message>;

	fn send_message(&self, message: Self::Message) -> Command<Self::Message>
	where
		Self::Message: 'static + Send,
	{
		Command::perform(async {}, |()| message)
	}
}

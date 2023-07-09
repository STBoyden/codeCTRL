use crate::{view::View, Message};

use codectrl_protobuf_bindings::data::Log;
use iced::widget::{button, column, row, text};

#[derive(Debug, Clone, Default)]
pub struct LogDetails {
	log: Log,
}

impl LogDetails {
	pub fn new(log: Log) -> Self { Self { log } }
}

impl View for LogDetails {
	type Message = Message;

	fn view(&self) -> iced::Element<'_, Self::Message> {
		column![
			row![button("Close").on_press(Message::LogDetailsSplitClose)],
			text(&self.log.message)
		]
		.into()
	}
}
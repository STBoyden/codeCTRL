use crate::{view::View, Message};

use codectrl_protobuf_bindings::data::Log;
use iced::{
	widget::{button, column, row, text},
	Command,
};
use iced_aw::{split::Axis, Split};

#[derive(Debug, Clone, Default)]
pub struct LogDetails {
	log: Log,
	split_size: Option<u16>,
}

impl LogDetails {
	pub fn new(log: Log) -> Self {
		Self {
			log,
			split_size: None,
		}
	}

	fn trace_view(&self) -> iced::Element<'_, Message> { column![].into() }
	fn code_view(&self) -> iced::Element<'_, Message> { column![].into() }
}

impl View for LogDetails {
	type Message = Message;

	fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
		match message {
			Message::LogDetailsInnerSplitResize(size) => {
				self.split_size = Some(size);
				Command::none()
			},
			_ => Command::none(),
		}
	}

	fn view(&self) -> iced::Element<'_, Self::Message> {
		row![
			button("Close").on_press(Message::LogDetailsSplitClose),
			Split::new(
				self.trace_view(),
				self.code_view(),
				self.split_size,
				Axis::Vertical,
				Message::LogDetailsInnerSplitResize
			)
		]
		.into()
	}
}
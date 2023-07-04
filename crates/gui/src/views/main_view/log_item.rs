use crate::{view::ViewBorrowed, Message};

use chrono::{DateTime, Local};
use codectrl_protobuf_bindings::data::Log;
use iced::{
	alignment::{Alignment, Horizontal},
	widget::{button, radio, row, text},
	Command, Length,
};

#[derive(Clone, Debug)]
pub struct LogItem {
	log: Log,
	pub time: DateTime<Local>,
	selected: bool,
}

impl LogItem {
	pub fn new(log: Log, time: DateTime<Local>) -> Self {
		Self {
			log,
			time,
			selected: false,
		}
	}
}

impl ViewBorrowed for LogItem {
	type Message = Message;

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		use Message::*;

		match message {
			LogIndexChanged(uuid) => {
				if let Some(uuid) = uuid {
					self.selected = self.log.uuid == uuid;
				} else {
					self.selected = false;
				}

				Command::none()
			},
			_ => Command::none(),
		}
	}

	fn view<'a>(&self) -> iced::Element<'a, Message> {
		let message = text(&self.log.message)
			.width(Length::FillPortion(3))
			.horizontal_alignment(Horizontal::Left);
		let time_text = text(&self.time.to_rfc2822())
			.width(Length::Fill)
			.horizontal_alignment(Horizontal::Right);

		let LogItem { log, selected, .. } = self.clone();
		let log_clone = log.clone();

		row![
			radio("", true, Some(selected), move |_| Message::LogClicked(
				log_clone.clone(),
			)),
			button(row![message, time_text].align_items(Alignment::Center))
				.on_press(Message::LogClicked(log))
				.width(Length::Fill)
		]
		.align_items(Alignment::Center)
		.into()
	}
}

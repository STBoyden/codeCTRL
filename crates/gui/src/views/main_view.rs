mod log_item;

use crate::{view::View, Message};
use codectrl_protobuf_bindings::data::Log;
use iced::{
	widget::{column, container, row, scrollable, text},
	Command, Length,
};
use std::fmt;

use self::log_item::LogItem;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum LogAppearanceState {
	#[default]
	NewestFirst,
	OldestFirst,
}

impl LogAppearanceState {
	fn toggle(&mut self) {
		if *self == Self::NewestFirst {
			*self = Self::OldestFirst;
		} else {
			*self = Self::NewestFirst;
		}
	}
}

impl fmt::Display for LogAppearanceState {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let out = match *self {
			Self::NewestFirst => String::from("Newest first"),
			Self::OldestFirst => String::from("Oldest first"),
		};

		write!(f, "{out}")
	}
}

#[derive(Debug, Clone, Default)]
pub struct Main {
	pub scroll_to_selected_log: bool,
	pub log_appearance: LogAppearanceState,
	pub logs: Vec<Log>,
}

impl View for Main {
	type Message = Message;

	fn title(&self) -> String { String::new() }

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		use Message::*;

		match message {
			ScrollToSelectedLogChanged(state) => {
				self.scroll_to_selected_log = state;
				Command::none()
			},
			LogAppearanceStateChanged => {
				self.log_appearance.toggle();
				Command::none()
			},
			ServerAddLog(log) => {
				self.logs.push(log);
				Command::none()
			},
			LogClicked(log) => Command::none(),
			_ => Command::none(),
		}
	}

	fn view(&self) -> iced::Element<'_, Self::Message> {
		let log_items = self
			.logs
			.iter()
			.cloned()
			.map(|log| LogItem::new(log))
			.collect::<Vec<_>>();

		let mut elements = vec![text("Main view").into()];

		for item in log_items {
			elements.push(item.view());
		}

		let logs = scrollable(column(elements)).width(Length::Fill);

		container(logs).into()
	}
}

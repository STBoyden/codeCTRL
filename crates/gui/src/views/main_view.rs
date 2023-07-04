mod log_details_view;
mod log_item;

use crate::{
	view::{View, ViewBorrowed},
	Message,
};
use chrono::Local;

use iced::{
	widget::{column, container, scrollable, text},
	Command, Element, Length,
};
use iced_aw::{split::Axis, Split};
use std::fmt;

use self::{log_details_view::LogDetails, log_item::LogItem};

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
	pub log_appearance: LogAppearanceState,
	logs: Vec<LogItem>,

	log_details_view: Option<LogDetails>,
	log_details_split: u16,
}

impl View for Main {
	type Message = Message;

	fn title(&self) -> String { String::new() }

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		use Message::*;

		match message {
			LogAppearanceStateChanged => {
				self.log_appearance.toggle();

				use LogAppearanceState::*;
				self.logs.sort_by(|a, b| match self.log_appearance {
					NewestFirst => a.time.cmp(&b.time),
					OldestFirst => b.time.cmp(&a.time),
				});

				Command::none()
			},
			ServerAddLog(log) => {
				self.logs.push(LogItem::new(log, Local::now()));
				Command::none()
			},
			LogClicked(log) => {
				let uuid = log.uuid.clone();

				self.log_details_view = Some(LogDetails::new(log));
				self.send_message(UpdateLogItems(Box::new(LogIndexChanged(Some(uuid.into())))))
			},
			LogDetailsSplitResize(size) => {
				self.log_details_split = size;
				Command::none()
			},
			LogDetailsSplitClose => {
				self.log_details_view = None;
				self.send_message(UpdateLogItems(Box::new(LogIndexChanged(None))))
			},
			UpdateLogItems(message) => {
				let updates = self
					.logs
					.iter_mut()
					.map(|log_item| log_item.update(*message.clone()));

				Command::batch(updates)
			},

			LogIndexChanged(_) if self.log_details_view.is_some() =>
				self.log_details_view.as_mut().unwrap().update(message),

			_ => Command::none(),
		}
	}

	fn view(&self) -> iced::Element<'_, Self::Message> {
		let mut elements = vec![];

		for item in self.logs.iter() {
			elements.push(item.view());
		}

		let logs = scrollable(column(elements).spacing(0.5).padding(10.0)).width(Length::Fill);

		let view: Element<'_, _, _> = if self.log_details_view.is_some() {
			Split::new(
				logs,
				self.log_details_view.as_ref().unwrap().view(),
				if self.log_details_split == 0 {
					None
				} else {
					Some(self.log_details_split)
				},
				Axis::Horizontal,
				Message::LogDetailsSplitResize,
			)
			.min_size_first(20)
			.min_size_second(20)
			.into()
		} else {
			logs.into()
		};

		container(column![text("Main view"), view].width(Length::Fill)).into()
	}
}

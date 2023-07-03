use crate::Message;
use codectrl_protobuf_bindings::data::Log;
use iced::widget::{button, column, row, text};

#[derive(Clone, Debug)]
pub struct LogItem {
    inner: Log,
}

impl LogItem {
    pub fn new(log: Log) -> Self { Self { inner: log } }

    pub fn view<'a>(&self) -> iced::Element<'a, Message> {
        let message = text(&self.inner.message);

        button(row![message])
            .on_press(Message::LogClicked(self.inner.clone()))
            .into()
    }
}

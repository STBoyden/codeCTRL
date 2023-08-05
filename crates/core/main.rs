#[cfg(feature = "full")]
use codectrl_gui::{
	iced::{self, Application, Settings},
	App, Flags,
};

#[cfg(feature = "server-only")]
use codectrl_server::run_server;

#[cfg(feature = "server-only")]
#[tokio::main]
async fn main() { _ = run_server(None, None, None, None, true).await; }

#[cfg(feature = "full")]
fn main() -> iced::Result {
	App::run(Settings {
		id: Some(String::from("CodeCTRL")),
		flags: Flags::default(),
		antialiasing: true,
		exit_on_close_request: true,

		..Settings::default()
	})
}
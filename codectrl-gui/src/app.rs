// In this file, we create the main graphical GUI for codeCTRL. A layout was
// proposed in issue #3, where Sebastian proposed a layout similar to this:
// _________________________________________________________________________
// | [ Filter search ] [x] Case insensitive [x] Regex | Some other settings|
// |-----------------------------------------------------------------------|
// |    _______________________________________________________________    |
// |    | x | Message | Host | File name | Line number | Time | ...   |    |
// |    ---------------------------------------------------------------    |
// |    _______________________________________________________________    |
// |    |   | Message | Host | File name | Line number | Time | ...   |    |
// |    ---------------------------------------------------------------    |
// |    _______________________________________________________________    |
// |    |   | Message | Host | File name | Line number | Time | ...   |    |
// |    ---------------------------------------------------------------    |
// |_______________________________________________________________________|
// |  Log details                    |  Code snippet                       |
// |                                 |                                     |
// |                                 |                                     |
// |                                 |                                     |
// |                                 |                                     |
// |                                 |                                     |
// ----------------------------------|--------------------------------------
//
// Further changes can be discussed and implemented at later dates, but this is
// the proposal so far.

use crate::{
    components::{
        about_view, details_view, fonts, main_view, main_view_empty, settings_view,
    },
    data::{AppState, Filter, FontSizes, Receiver},
};
use chrono::{DateTime, Local};
use codectrl_logger::Log;
use egui::{CtxRef, Event, InputState, Key};
use epi::{Frame, Storage};
use rfd::{FileDialog, MessageDialog};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufReader, Error as IOError, ErrorKind, Write},
    path::Path,
    sync::{mpsc::Receiver as Rx, Arc, Mutex},
    thread::{Builder as ThreadBuilder, JoinHandle},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub session_timestamp: String,
    pub received: VecDeque<(Log<String>, DateTime<Local>)>,
    pub message_alerts: BTreeSet<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct App {
    #[serde(skip)]
    pub receiver: Receiver,
    #[serde(skip)]
    update_thread: Option<JoinHandle<()>>,
    data: AppState,
    title: &'static str,
    socket_address: String,
}

impl App {
    pub fn new(receiver: Rx<Log<String>>, socket_address: String) -> Self {
        Self {
            receiver: Some(Arc::new(Mutex::new(receiver))),
            update_thread: None,
            data: AppState::default(),
            title: "codeCTRL",
            socket_address,
        }
    }

    fn handle_key_inputs(&mut self, input_state: &InputState) {
        for event in &input_state.events {
            match event {
                // zoom bindings
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::PageUp
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                {
                    self.data.application_settings.font_sizes.scale(1.0);
                },
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::PageDown
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                    self.data.application_settings.font_sizes.scale(-1.0),
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::Num0
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                {
                    self.data.application_settings.font_sizes = FontSizes::default();
                },
                Event::Zoom(zoom_delta) =>
                    if *zoom_delta > 1.0 {
                        self.data.application_settings.font_sizes.scale(1.0);
                    } else if *zoom_delta < 1.0 {
                        self.data.application_settings.font_sizes.scale(-1.0);
                    },

                // open/load bindings
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::O
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                    self.load_file_dialog(),
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::S
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                    self.save_file_dialog(),

                _ => (),
            }
        }
    }

    fn save_file_dialog(&mut self) {
        self.data.session_timestamp =
            Local::now().format(&self.data.filename_format).to_string();

        let file_path = if let Some(file_path) = FileDialog::new()
            .set_file_name(&format!(
                "{file_name}.cdctrl",
                file_name = self.data.session_timestamp
            ))
            .add_filter("codeCTRL Session", &["cdctrl"])
            .save_file()
        {
            file_path
        } else {
            return;
        };

        let AppState {
            session_timestamp,
            message_alerts,
            ..
        } = self.data.clone();

        let session = Session {
            session_timestamp,
            received: self.data.received.read().unwrap().clone(),
            message_alerts,
        };

        let data = serde_cbor::to_vec(&session).expect("Could not serialise logs");

        let mut file = match File::create(&file_path) {
            Ok(file_path) => file_path,
            Err(error) => {
                MessageDialog::new()
                    .set_title("Could not save file")
                    .set_description(&format!(
                        "Could not save file \"{file_path}\": {error}",
                        file_path = file_path.to_string_lossy(),
                    ))
                    .show();

                return;
            },
        };

        if let Err(error) = file.write_all(data.as_slice()) {
            MessageDialog::new()
                .set_title("Could not write to file")
                .set_description(&format!(
                    "Could not write to file \"{file_path}\": {error}",
                    file_path = file_path.to_string_lossy(),
                ))
                .show();
        }
    }

    fn load_file_dialog(&mut self) {
        let file_path = if let Some(file_path) = FileDialog::new()
            .add_filter("codeCTRL Session", &["cdctrl"])
            .pick_file()
        {
            file_path
        } else {
            return;
        };

        match Self::load_from_file(&file_path, self) {
            Ok(_) => MessageDialog::new()
                .set_title("Successfully loaded file data")
                .set_description("Successfully loaded file data"),
            Err(error) => MessageDialog::new()
                .set_title("Could not parse log data")
                .set_description(&format!("{error}")),
        }
        .show();
    }

    pub fn load_from_file(file_path: &Path, app: &mut App) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;

        let reader = BufReader::new(file);

        let session: Session = match serde_cbor::from_reader(reader) {
            Ok(data) => data,
            Err(error) =>
                return Err(Box::new(IOError::new(
                    ErrorKind::Other,
                    format!(
                        "Could not parse log data from file \"{file_path}\": {error}",
                        file_path = file_path.to_string_lossy()
                    ),
                ))),
        };

        let AppState {
            received,
            session_timestamp,
            message_alerts,
            ..
        } = &mut app.data;

        *received.write().unwrap() = session.received;
        *session_timestamp = session.session_timestamp;
        *message_alerts = session.message_alerts;

        Ok(())
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
        ctx.set_fonts(fonts(self.data.application_settings.font_sizes));

        self.handle_key_inputs(ctx.input());

        if self.data.is_about_open {
            about_view(&mut self.data, ctx);
        }

        if self.data.is_settings_open {
            settings_view(&mut self.data, ctx);
        }

        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Save project").clicked() {
                            self.save_file_dialog();
                        }

                        if ui.button("Open project").clicked() {
                            self.load_file_dialog();
                        }

                        ui.separator();

                        if ui.button("Settings").clicked() {
                            self.data.is_settings_open = !self.data.is_settings_open;
                        }

                        ui.separator();

                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.data.is_about_open = !self.data.is_about_open;
                        }
                    });

                    ui.separator();

                    ui.label("Filter: ");
                    ui.text_edit_singleline(&mut self.data.search_filter);

                    // u1f5d9 = 🗙
                    if ui.button("\u{1f5d9}").clicked() {
                        self.data.search_filter = "".into();
                    }

                    ui.label("Filter by:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.data.filter_by))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::Message,
                                format!("{}", Filter::Message),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::Time,
                                format!("{}", Filter::Time),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::FileName,
                                format!("{}", Filter::FileName),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::Address,
                                format!("{}", Filter::Address),
                            );

                            ui.selectable_value(
                                &mut self.data.filter_by,
                                Filter::LineNumber,
                                format!("{}", Filter::LineNumber),
                            );
                        });

                    ui.checkbox(&mut self.data.is_case_sensitive, "Case sensitive");
                    ui.checkbox(&mut self.data.is_using_regex, "Regex");
                    ui.checkbox(
                        &mut self.data.do_scroll_to_selected_log,
                        "Scroll to selected log",
                    );

                    if ui
                        .button(
                            if self.data.is_newest_first {
                                "\u{2b07} Newest first" // u2b07 = ⬇
                            } else {
                                "\u{2b06} Newest last" // u2b06 = ⬆
                            },
                        )
                        .clicked()
                    {
                        self.data.is_newest_first = !self.data.is_newest_first;
                    }

                    // u1f5d1 = ��
                    if ui.button("\u{1f5d1} Clear logs").clicked() {
                        if let Ok(mut received) = self.data.received.write() {
                            received.clear();
                            self.data.clicked_item = None;
                        }
                    }

                    ui.separator();

                    ui.label(format!("Listening on: {}", self.socket_address));
                });

                ui.add_space(2.0);
            });

        let is_empty = {
            let received = Arc::clone(&self.data.received);

            let x = if let Ok(received) = received.read() {
                received.is_empty()
            } else {
                false
            };

            x
        };

        if is_empty {
            main_view_empty(ctx, &self.socket_address);
        } else {
            main_view(&mut self.data, ctx);
        }

        if self.data.clicked_item.is_some() {
            details_view(&mut self.data, ctx);
        } else {
            self.data.preview_height = 0.0;
        }
    }

    fn setup(&mut self, ctx: &CtxRef, frame: &Frame, storage: Option<&dyn Storage>) {
        if let Some(storage) = storage {
            let data: AppState =
                epi::get_value(storage, epi::APP_KEY).unwrap_or_default();

            if data.preserve_session {
                self.data = data;
            } else {
                self.data = AppState::default();
                self.data.preserve_session = false;
            }
        }

        let rx = Arc::clone(self.receiver.as_ref().unwrap());
        let received = Arc::clone(&self.data.received);

        ctx.set_visuals(self.data.current_theme.clone());

        self.update_thread = Some(unsafe {
            ThreadBuilder::new()
                .name("update_thread".into())
                .spawn_unchecked(move || loop {
                    let recd = match rx.try_lock() {
                        Ok(lock) => lock,
                        Err(error) => {
                            eprintln!("Could not get lock on mutex: {error}");
                            continue;
                        },
                    }
                    .recv();

                    if let Ok(recd) = recd {
                        received.write().unwrap().push_front((recd, Local::now()));
                        frame.request_repaint();
                    }
                })
                .expect("Could not start codeCTRL update thread")
        });
    }

    fn name(&self) -> &str { self.title }

    fn save(&mut self, storage: &mut dyn Storage) {
        epi::set_value(storage, epi::APP_KEY, &self.data);
    }
}
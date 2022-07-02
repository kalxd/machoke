use gtk::{glib, MessageDialog};
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox};

use std::path::PathBuf;
use std::rc::Rc;

mod cover;
mod form;
mod headerbar;
mod song;

pub enum AppAction {
	OpenAudia((PathBuf, id3::Tag)),
	ChangeCover(PathBuf),
	Error(String),
}

pub struct MainWindow {
	window: ApplicationWindow,
	widget: song::SongWidget,

	rx: glib::Receiver<AppAction>,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let (tx, rx) = glib::MainContext::channel::<AppAction>(glib::PRIORITY_DEFAULT);
		let tx = Rc::new(tx);

		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.build();

		let title_bar = headerbar::TitleBar::new();
		window.set_titlebar(Some(&title_bar.bar));

		let main_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin(10)
			.build();

		let form = song::SongWidget::new(tx.clone());
		main_layout.pack_start(&form.layout, false, false, 0);
		window.add(&main_layout);

		title_bar.connect_open_song({
			let tx = tx.clone();
			move |path| {
				match id3::Tag::read_from_path(&path) {
					Ok(tag) => tx.send(AppAction::OpenAudia((path, tag))),
					Err(e) => tx.send(AppAction::Error(e.to_string())),
				}
				.unwrap()
			}
		});

		Self {
			window,
			widget: form,
			rx,
		}
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);

		main_window.window.show_all();
		main_window.widget.hide_something();

		main_window.rx.attach(None, move |msg| {
			match msg {
				AppAction::OpenAudia((filepath, tag)) => {
					main_window.widget.update(filepath, tag);
				}
				AppAction::ChangeCover(path) => {
					main_window.widget.cover.update_cover_from_path(path);
				}
				AppAction::Error(msg) => {
					MessageDialog::builder()
						.text(&msg)
						.message_type(gtk::MessageType::Error)
						.buttons(gtk::ButtonsType::Close)
						.build()
						.run();
				}
			}
			glib::Continue(true)
		});
	}
}

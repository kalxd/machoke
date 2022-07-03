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
	Save,

	Alert(Result<String, String>),
}

pub struct MainWindow {
	window: ApplicationWindow,
	title_bar: headerbar::TitleBar,
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
					Err(e) => tx.send(AppAction::Alert(Err(e.to_string()))),
				}
				.unwrap()
			}
		});

		title_bar.save_btn.connect_clicked({
			let tx = tx.clone();
			move |_| {
				tx.send(AppAction::Save).unwrap();
			}
		});

		Self {
			window,
			title_bar,
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
					main_window.title_bar.save_btn.set_sensitive(true);
					main_window.title_bar.save_as_btn.set_sensitive(true);
				}
				AppAction::ChangeCover(path) => {
					main_window.widget.cover.update_cover_from_path(path);
				}
				AppAction::Save => main_window.widget.save_file(),
				AppAction::Alert(msg) => {
					let mtype = if msg.is_ok() {
						gtk::MessageType::Info
					} else {
						gtk::MessageType::Error
					};

					let msg = match msg {
						Ok(s) => s,
						Err(s) => s,
					};

					let dialog = MessageDialog::builder()
						.text(&msg)
						.message_type(mtype)
						.buttons(gtk::ButtonsType::Close)
						.build();
					dialog.run();
					dialog.emit_close();
				}
			}
			glib::Continue(true)
		});
	}
}

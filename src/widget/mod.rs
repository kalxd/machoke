use gtk::{glib, InfoBar, Label, MessageType};
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox};

use std::rc::Rc;

use crate::emitter::{EmitEvent, Emitter};

mod cover;
mod form;
mod headerbar;
mod song;

pub struct MainWindow {
	window: ApplicationWindow,
	title_bar: headerbar::TitleBar,
	widget: song::SongWidget,
	infobar: InfoBar,
	infolabel: Label,

	rx: glib::Receiver<EmitEvent>,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let (tx, rx) = glib::MainContext::channel::<EmitEvent>(glib::PRIORITY_DEFAULT);
		let tx = Rc::new(Emitter::new(tx));

		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.icon_name("machoke")
			.build();

		let title_bar = headerbar::TitleBar::new();
		window.set_titlebar(Some(&title_bar.bar));

		let main_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin(10)
			.build();

		let infobar = InfoBar::builder()
			.show_close_button(true)
			.visible(false)
			.build();
		let infolabel = Label::new(None);
		infobar.pack_start(&infolabel, false, false, 0);
		main_layout.pack_start(&infobar, false, true, 0);
		infobar.connect_response(|infobar, _| {
			infobar.hide();
		});

		let form = song::SongWidget::new(tx.clone());
		main_layout.pack_start(&form.layout, false, false, 0);
		window.add(&main_layout);

		title_bar.connect_open_song({
			let tx = tx.clone();
			move |path| match id3::Tag::read_from_path(&path) {
				Ok(tag) => tx.send(EmitEvent::OpenTag(tag)),
				Err(e) => tx.error(e),
			}
		});

		title_bar.save_btn.connect_clicked({
			let tx = tx.clone();
			move |_| tx.send(EmitEvent::Save)
		});

		Self {
			window,
			title_bar,
			widget: form,
			infobar,
			infolabel,
			rx,
		}
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);

		main_window.window.show_all();
		main_window.widget.hide_something();
		main_window.infobar.hide();

		main_window.rx.attach(None, move |msg| {
			match msg {
				EmitEvent::OpenTag(tag) => {
					main_window.widget.update(tag);
					main_window.title_bar.save_btn.set_sensitive(true);
				}
				EmitEvent::Alert(result) => {
					let (mtype, msg) = match result {
						Ok(s) => (MessageType::Info, s),
						Err(e) => (MessageType::Error, e),
					};

					main_window.infobar.set_message_type(mtype);
					main_window.infolabel.set_text(&msg);
				}
				_ => {} /*
						AppAction::ChangeCover(path) => {
							main_window.widget.cover.update_cover_from_path(path);
						}
						AppAction::RemoveCover => {
							main_window.widget.cover.remove_cover();
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
						 */
			}
			glib::Continue(true)
		});
	}
}

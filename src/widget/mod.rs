use gtk::{glib, InfoBar, Label, MessageType};
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox};

use std::cell::RefCell;
use std::rc::Rc;

use crate::emitter::{EmitEvent, Emitter};
use crate::value::AppState;

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
	app_state: Rc<RefCell<Option<AppState>>>,

	tx: Rc<Emitter>,
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
			move |path| tx.send(EmitEvent::OpenTag(path))
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
			app_state: Default::default(),
			tx,
			rx,
		}
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);

		main_window.window.show_all();
		main_window.widget.hide_something();
		main_window.infobar.hide();

		let tx = main_window.tx;

		main_window.rx.attach(None, move |msg| {
			match msg {
				EmitEvent::OpenTag(path) => match AppState::try_from(path) {
					Ok(app_data) => {
						main_window.widget.update(&app_data);
						main_window.title_bar.save_btn.set_sensitive(true);
						main_window.app_state.replace(Some(app_data));
					}
					Err(e) => tx.error(e),
				},
				EmitEvent::ChangeCover(path) => {
					main_window.widget.cover.update_cover_from_path(&path);
				}
				EmitEvent::Save => {}
				EmitEvent::Alert(result) => {
					let (mtype, msg) = match result {
						Ok(s) => (MessageType::Info, s),
						Err(e) => (MessageType::Error, e),
					};

					main_window.infobar.set_message_type(mtype);
					main_window.infolabel.set_text(&msg);
				}
			};
			glib::Continue(true)
		});
	}
}

use gtk::gdk::DragAction;
use gtk::{glib, DestDefaults, InfoBar, Label, MessageType, TargetEntry, TargetFlags};
use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox};

use std::cell::RefCell;
use std::rc::Rc;

use crate::emitter::{EmitEvent, Emitter};
use crate::value::{get_drag_drop_filepath, AppState, AppStateBox, SaveData};

mod cover;
mod element;
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
		let (tx, rx) = glib::MainContext::channel::<EmitEvent>(glib::source::Priority::DEFAULT);
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
		infobar
			.content_area()
			.pack_start(&infolabel, false, false, 0);
		main_layout.pack_start(&infobar, false, true, 0);
		infobar.connect_response(|infobar, _| {
			infobar.hide();
		});

		let form = song::SongWidget::new(tx.clone());
		main_layout.pack_start(&form.layout, false, false, 0);
		window.add(&main_layout);

		title_bar.connect_open_song({
			let tx = tx.clone();
			move |path| tx.send(EmitEvent::OpenTag(dbg!(path)))
		});

		title_bar.save_btn.connect_clicked({
			let tx = tx.clone();
			move |_| tx.send(EmitEvent::Save)
		});

		// drop files
		let targets = [TargetEntry::new("text/uri-list", TargetFlags::OTHER_APP, 0)];
		window.drag_dest_set(DestDefaults::ALL, &targets, DragAction::COPY);

		window.connect_drag_data_received({
			let tx = tx.clone();
			move |_, _, _, _, data, _, _| match get_drag_drop_filepath(&data) {
				Some(p) => tx.send(EmitEvent::OpenTag(p)),
				None => tx.send(EmitEvent::Alert((
					MessageType::Warning,
					"不支持该文件类型！".into(),
				))),
			}
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
				EmitEvent::OpenTag(path) => match AppStateBox::try_from(path) {
					Ok(AppStateBox((msg, app_data))) => {
						if let Some(msg) = msg {
							tx.warn(msg);
						}

						main_window.widget.update(&app_data);
						main_window.title_bar.save_btn.set_sensitive(true);
						main_window
							.title_bar
							.bar
							.set_subtitle(app_data.audio_path.to_str());
						main_window.app_state.replace(Some(app_data));
						main_window.infobar.hide();
					}
					Err(e) => tx.error(e),
				},
				EmitEvent::ChangeCover(path) => main_window.widget.change_cover(&path),
				EmitEvent::RemoveCover => main_window.widget.remove_cover(),
				EmitEvent::Save => {
					if let Some(state) = main_window.app_state.borrow_mut().as_mut() {
						let (mime_type, pic_data) = main_window.widget.get_data();
						let form_data = main_window.widget.form.form_data();
						main_window.widget.form.save_to_store(&form_data);

						let cover = mime_type.as_ref().zip(pic_data);

						let save_data = SaveData {
							base: form_data,
							cover,
						};

						let result = state.save(save_data).map(|_| String::from("保存成功！"));
						tx.alert(result);
					}
				}
				EmitEvent::Alert((msg_type, msg)) => {
					main_window.infobar.set_message_type(msg_type);
					main_window.infolabel.set_text(&msg);
					main_window.infobar.show();
				}
			};
			glib::ControlFlow::Continue
		});
	}
}

use futures::channel::mpsc;
use futures::future::ready;
use futures::StreamExt;
use gtk::gdk::DragAction;
use gtk::{glib, DestDefaults, MessageType, TargetEntry, TargetFlags};
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
	alertbar: element::alert::Alert,
	app_state: Rc<RefCell<Option<AppState>>>,
}

impl MainWindow {
	fn new(app: &Application, tx: Emitter) -> Self {
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

		let alertbar = element::alert::Alert::new();
		main_layout.pack_start(&*alertbar, false, true, 0);

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

		// drop files
		let targets = [TargetEntry::new("text/uri-list", TargetFlags::OTHER_APP, 0)];
		window.drag_dest_set(DestDefaults::ALL, &targets, DragAction::COPY);

		window.connect_drag_data_received({
			let tx = tx.clone();
			move |_, _, _, _, data, _, _| match get_drag_drop_filepath(data) {
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
			alertbar,
			app_state: Default::default(),
		}
	}

	fn event_loop(&self, msg: EmitEvent, tx: Emitter) {
		match msg {
			EmitEvent::OpenTag(path) => match AppStateBox::try_from(path) {
				Ok(AppStateBox((msg, app_data))) => {
					if let Some(msg) = msg {
						tx.warn(msg);
					} else {
						self.alertbar.hide();
					}

					self.widget.update(&app_data);
					self.title_bar.save_btn.set_sensitive(true);
					self.title_bar
						.bar
						.set_subtitle(app_data.audio_path.to_str());
					self.app_state.replace(Some(app_data));
				}
				Err(e) => tx.error(e),
			},
			EmitEvent::ChangeCover(path) => self.widget.cover.update_cover_from_path(&path),
			EmitEvent::RemoveCover => self.widget.cover.remove_cover(),
			EmitEvent::ApplyCover((pixbuf, mime_type)) => {
				self.widget.cover.set_cover(pixbuf, mime_type);
			}
			EmitEvent::Save => {
				if let Some(state) = self.app_state.borrow_mut().as_mut() {
					let (pic_data, mime_type) = self.widget.cover.cover();
					let form_data = self.widget.form.form_data();
					self.widget.form.save_to_store(&form_data);

					let cover = mime_type.as_ref().zip(pic_data);

					let save_data = SaveData {
						base: form_data,
						cover,
					};

					let result = state.save(save_data).map(|_| String::from("保存成功！"));
					tx.alert(result);
				}
			}
			EmitEvent::Alert((msg_type, msg)) => self.alertbar.show(msg_type, msg),
		};
	}

	pub fn run(app: &Application) {
		let (tx, rx) = mpsc::channel(10);
		let tx = Emitter::new(tx);

		let main_window = Self::new(app, tx.clone());

		main_window.window.show_all();
		main_window.widget.hide_something();
		main_window.alertbar.hide();

		glib::MainContext::default().spawn_local(async move {
			rx.for_each(|msg| {
				main_window.event_loop(msg, tx.clone());
				ready(())
			})
			.await;
		});
	}
}

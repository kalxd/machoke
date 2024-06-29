use std::ops::Deref;
use std::path::PathBuf;

use gtk::gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use gtk::{glib, FileChooserDialog, FileFilter, ListStore, ResponseType, ScrolledWindow};
use gtk::{prelude::*, Box as GtkBox, Button, Frame, IconView, Image, Orientation};

use crate::emitter::Emitter;
use crate::value::{AppState, CoverMimeType};

const COVER_SIZE: i32 = 128;

fn open_cover_chooser_dialog() -> Option<PathBuf> {
	let filter = FileFilter::new();
	filter.add_mime_type(CoverMimeType::Png.as_ref());
	filter.add_mime_type(CoverMimeType::Jpeg.as_ref());
	let dialog = FileChooserDialog::builder()
		.title("选择新的封面")
		.filter(&filter)
		.build();
	dialog.add_button("确定", ResponseType::Accept);

	let rsp = dialog.run();
	dialog.emit_close();

	match rsp {
		ResponseType::Accept => dialog.filename(),
		_ => None,
	}
}

struct CoverListStore(ListStore);

impl CoverListStore {
	fn new() -> Self {
		let store = ListStore::new(&[
			glib::Type::STRING,
			Pixbuf::static_type(),
			Pixbuf::static_type(),
		]);
		Self(store)
	}

	fn add_history(&self, key: &str, pixbuf: Pixbuf) {
		let is_contains = (0..self.0.iter_n_children(None))
			.map(|i| self.0.iter_nth_child(None, i))
			.map(|miter| miter.and_then(|iter| self.0.value(&iter, 0).get::<'_, String>().ok()))
			.any(|ma| ma.as_deref() == Some(key));

		if is_contains {
			return;
		}

		let scale_pixbuf = pixbuf.scale_simple(64, 64, InterpType::Nearest);
		self.0.insert_with_values(
			None,
			&[(0, &key.to_value()), (1, &scale_pixbuf), (2, &pixbuf)],
		);
	}
}

impl Deref for CoverListStore {
	type Target = ListStore;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

struct CoverList {
	icon_view: IconView,
	frame: Frame,
	store: CoverListStore,
}

impl CoverList {
	fn new() -> Self {
		let frame = Frame::new(Some("封面列表"));
		let scrolled_window = ScrolledWindow::builder().build();

		let store = CoverListStore::new();
		let icon_view = IconView::builder()
			.width_request(200)
			.model(&*store)
			.pixbuf_column(1)
			.selection_mode(gtk::SelectionMode::Single)
			.build();
		scrolled_window.add(&icon_view);
		frame.add(&scrolled_window);

		Self {
			icon_view,
			frame,
			store,
		}
	}

	fn connect_select<F>(&self, f: F)
	where
		F: Fn(Pixbuf) + 'static,
	{
		let store = self.store.clone();
		self.icon_view.connect_selection_changed(move |icon_view| {
			let a = icon_view.selected_items().last().and_then(|path| {
				store
					.iter(path)
					.and_then(|iter| store.value(&iter, 2).get::<'_, Pixbuf>().ok())
			});

			if let Some(pixbuf) = a {
				f(pixbuf);
			}
		});
	}
}

pub struct CoverWidget {
	pub layout: GtkBox,
	info_layout: GtkBox,
	image: Image,
	change_btn: Button,
	remove_btn: Button,
	cover_list: CoverList,

	tx: Emitter,
}

impl CoverWidget {
	pub fn new(tx: Emitter) -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
			.spacing(20)
			.build();

		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		layout.pack_start(&image, false, false, 10);

		let info_layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
			.spacing(20)
			.build();

		let btn_layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
			.valign(gtk::Align::Center)
			.spacing(20)
			.build();

		let change_btn = Button::with_label("更换封面");
		btn_layout.pack_start(&change_btn, false, false, 0);
		let remove_btn = Button::with_label("移除封面");
		btn_layout.pack_start(&remove_btn, false, false, 0);
		info_layout.pack_start(&btn_layout, false, false, 0);
		layout.pack_start(&info_layout, false, false, 0);

		let cover_list = CoverList::new();
		layout.pack_end(&cover_list.frame, true, true, 0);

		cover_list.connect_select(|pixbuf| {
			dbg!(pixbuf);
		});

		Self {
			info_layout,
			layout,
			image,
			change_btn,
			remove_btn,
			cover_list,
			tx,
		}
	}

	pub fn connect_change_cover<F>(&self, f: F)
	where
		F: Fn(PathBuf) + 'static,
	{
		self.change_btn.connect_clicked(move |_| {
			if let Some(path) = open_cover_chooser_dialog() {
				f(path);
			}
		});
	}

	pub fn connect_remove_cover<F>(&self, f: F)
	where
		F: Fn() + 'static,
	{
		self.remove_btn.connect_clicked(move |_| f());
	}

	pub fn hide_something(&self) {
		self.info_layout.hide();
	}

	pub fn update_with_tag(&self, state: &AppState) {
		self.info_layout.show();

		if let Some(picture) = state.front_cover() {
			let loader = PixbufLoader::new();

			let pixbuf = loader
				.write(&picture.data)
				.and_then(|_| loader.close())
				.ok()
				.and_then(|_| loader.pixbuf());

			self.set_pixbuf(pixbuf.as_ref());
		} else {
			self.image.set_pixbuf(None);
		}
	}

	pub fn update_cover_from_path(&self, path: &PathBuf) {
		match Pixbuf::from_file(path) {
			Err(e) => self.tx.error(e),
			Ok(pixbuf) => {
				self.set_pixbuf(Some(&pixbuf));
				self.cover_list
					.store
					.add_history(path.to_str().unwrap(), pixbuf);
			}
		}
	}

	pub fn remove_cover(&self) {
		self.image.set_pixbuf(None);
	}

	pub fn get_pixbuf_bytes(&self) -> Option<Vec<u8>> {
		self.image.pixbuf()?.save_to_bufferv("png", &[]).ok()
	}

	fn set_pixbuf(&self, pixbuf: Option<&Pixbuf>) {
		let pixbuf = pixbuf.and_then(|pixbuf| {
			pixbuf.scale_simple(COVER_SIZE, COVER_SIZE, gtk::gdk_pixbuf::InterpType::Nearest)
		});

		self.image.set_pixbuf(pixbuf.as_ref());
	}
}

use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;

use gtk::gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader};
use gtk::{glib, FileChooserDialog, FileFilter, ListStore, ResponseType, ScrolledWindow};
use gtk::{prelude::*, Box as GtkBox, Button, Frame, IconView, Image, Orientation};
use id3::frame::Picture;

use crate::emitter::{EmitEvent, Emitter};
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

fn picture_to_pixbuf(pic: Option<&Picture>) -> Option<(Pixbuf, CoverMimeType)> {
	let picture = pic?;
	let loader = PixbufLoader::new();
	loader.write(&picture.data).ok()?;
	loader.close().ok()?;
	let pixbuf = loader.pixbuf()?;
	let mime_type = CoverMimeType::from_mime_type(&picture.mime_type);
	Some((pixbuf, mime_type))
}

fn icon_view_selection(icon_view: &IconView, store: &ListStore) -> Option<(Pixbuf, CoverMimeType)> {
	let sel_item = icon_view.selected_items();
	let sel_path = sel_item.last()?;
	let sel_iter = store.iter(sel_path)?;

	let pixbuf: Pixbuf = store.value(&sel_iter, 2).get().ok()?;
	let mime_type: CoverMimeType = store.value(&sel_iter, 3).get().ok()?;
	Some((pixbuf, mime_type))
}

struct CoverListStore(ListStore);

impl CoverListStore {
	fn new() -> Self {
		let store = ListStore::new(&[
			glib::Type::STRING,
			Pixbuf::static_type(),
			Pixbuf::static_type(),
			glib::Type::STRING,
		]);
		Self(store)
	}

	fn add_history(&self, key: &str, pixbuf: Pixbuf, mime_type: &CoverMimeType) {
		let is_contains = (0..self.0.iter_n_children(None))
			.map(|i| self.0.iter_nth_child(None, i))
			.map(|miter| miter.and_then(|iter| self.0.value(&iter, 0).get::<'_, String>().ok()))
			.any(|ma| ma.as_deref() == Some(key));

		if is_contains {
			return;
		}

		let scale_pixbuf = pixbuf.scale_simple(64, 64, InterpType::Nearest);
		let iter = self.0.prepend();
		self.0.set(
			&iter,
			&[
				(0, &key.to_value()),
				(1, &scale_pixbuf),
				(2, &pixbuf),
				(3, mime_type),
			],
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
		let frame = Frame::new(Some("历史封面"));
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
		F: Fn(Pixbuf, CoverMimeType) + 'static,
	{
		let store = self.store.clone();
		self.icon_view.connect_selection_changed(move |icon_view| {
			if let Some((pixbuf, mime_type)) = icon_view_selection(icon_view, &store) {
				f(pixbuf, mime_type);
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
	mime_type: Rc<RefCell<Option<CoverMimeType>>>,

	tx: Emitter,
}

impl CoverWidget {
	pub fn new(tx: Emitter) -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
			.spacing(20)
			.build();

		let left_layout = GtkBox::new(Orientation::Horizontal, 20);

		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		left_layout.pack_start(&image, false, false, 10);

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
		left_layout.pack_start(&info_layout, false, false, 0);

		layout.pack_start(&left_layout, true, true, 0);

		let cover_list = CoverList::new();
		layout.pack_end(&cover_list.frame, true, true, 0);

		cover_list.connect_select({
			let tx = tx.clone();
			move |pixbuf, mime_type| tx.send(EmitEvent::ApplyCover((pixbuf, mime_type)))
		});

		Self {
			info_layout,
			layout,
			image,
			change_btn,
			remove_btn,
			cover_list,
			mime_type: Default::default(),
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

		match picture_to_pixbuf(state.front_cover()) {
			Some((pixbuf, mime_type)) => {
				self.set_pixbuf(Some(&pixbuf));
				self.cover_list.store.add_history(
					state.audio_path.to_str().unwrap(),
					pixbuf,
					&mime_type,
				);
			}
			None => self.remove_cover(),
		}
	}

	pub fn update_cover_from_path(&self, path: &PathBuf) {
		match Pixbuf::from_file(path) {
			Err(e) => self.tx.error(e),
			Ok(pixbuf) => {
				self.set_pixbuf(Some(&pixbuf));
				let mime_type = CoverMimeType::from_path(path);
				self.cover_list
					.store
					.add_history(path.to_str().unwrap(), pixbuf, &mime_type);
				self.mime_type.replace(Some(mime_type));
			}
		}
	}

	pub fn remove_cover(&self) {
		self.image.set_pixbuf(None);
		self.mime_type.replace(None);
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

	pub fn cover(&self) -> (Option<Vec<u8>>, Ref<Option<CoverMimeType>>) {
		let mime_type = self.mime_type.borrow();
		let pic = self.get_pixbuf_bytes();
		(pic, mime_type)
	}

	pub fn set_cover(&self, pixbuf: Pixbuf, mime_type: CoverMimeType) {
		self.mime_type.replace(Some(mime_type));
		self.set_pixbuf(Some(&pixbuf));
	}
}

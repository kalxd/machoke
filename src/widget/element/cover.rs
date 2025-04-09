use gtk::{
	gdk_pixbuf::Pixbuf,
	prelude::{BoxExt, ButtonExt, ContainerExt, DialogExt, FileChooserExt, ImageExt},
	Box as GtkBox, Button, FileChooserDialog, FileFilter, IconView, Image, ResponseType,
	ScrolledWindow,
};
use std::{cell::RefCell, ops::Deref, path::PathBuf, rc::Rc};

use crate::value::{scale_picture, CoverMimeType, ParseBox};

use super::store::HistoryStore;

const COVER_SIZE: i32 = 256;

fn open_cover_chooser_dialog() -> Option<PathBuf> {
	let filter = FileFilter::new();
	filter.add_mime_type(&CoverMimeType::Png.as_mime_type());
	filter.add_mime_type(&CoverMimeType::Jpg.as_mime_type());
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

#[derive(Clone)]
pub struct Cover {
	pub layout: GtkBox,
	image: Image,
	change_btn: Button,

	raw_image: Rc<RefCell<Option<id3::frame::Picture>>>,
}

impl Cover {
	pub fn new() -> Self {
		let raw_image = Rc::new(RefCell::new(None));

		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();

		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		layout.pack_start(&image, true, true, 0);

		let btn_layout = GtkBox::builder().build();
		layout.pack_start(&btn_layout, false, false, 0);

		let change_btn = Button::with_label("更换");
		layout.pack_start(&change_btn, true, true, 0);

		let remove_btn = Button::with_label("移除");
		layout.pack_start(&remove_btn, true, true, 0);
		remove_btn.connect_clicked({
			let raw_image = raw_image.clone();
			let image = image.clone();
			move |_| {
				image.set_pixbuf(None);
				raw_image.take();
			}
		});

		Self {
			layout,
			image,
			change_btn,
			raw_image,
		}
	}

	fn update_state_opt(&self, state: &ParseBox) -> Option<(id3::frame::Picture, Pixbuf)> {
		let pic = state.front_cover()?.clone();
		let (_, pixbuf) = scale_picture(&pic, COVER_SIZE)?;
		Some((pic, pixbuf))
	}

	pub fn set_cover_just(&self, pic: id3::frame::Picture) {
		if let Some((_, pixbuf)) = scale_picture(&pic, COVER_SIZE) {
			self.image.set_pixbuf(Some(&pixbuf));
			self.raw_image.replace(Some(pic));
		}
	}

	pub fn update_state(&self, state: &ParseBox) {
		match self.update_state_opt(state) {
			Some((pic, pixbuf)) => {
				self.image.set_pixbuf(Some(&pixbuf));
				self.raw_image.replace(Some(pic));
			}
			None => {
				self.image.set_pixbuf(None);
				self.raw_image.take();
			}
		}
	}

	pub fn connect_cover_change<F>(&self, f: F)
	where
		F: Fn(PathBuf) + 'static,
	{
		self.change_btn.connect_clicked(move |_| {
			if let Some(path) = open_cover_chooser_dialog() {
				f(path);
			}
		});
	}
}

pub struct HistoryCover {
	pub layout: ScrolledWindow,
	store: HistoryStore,
	icon_view: IconView,
}

impl HistoryCover {
	pub fn new() -> Self {
		let store = HistoryStore::new();

		let layout = ScrolledWindow::builder().build();

		let icon_view = IconView::builder()
			.model(&*store)
			.pixbuf_column(1)
			.selection_mode(gtk::SelectionMode::Single)
			.build();
		layout.add(&icon_view);

		Self {
			layout,
			store,
			icon_view,
		}
	}

	pub fn update_state(&self, state: &ParseBox) {
		if let Some((pic, path)) = state.front_cover().zip(state.audio_src.to_str()) {
			self.store.add_item(path, pic);
		}
	}
}

impl Deref for HistoryCover {
	type Target = IconView;

	fn deref(&self) -> &Self::Target {
		&self.icon_view
	}
}

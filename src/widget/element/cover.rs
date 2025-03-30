use gtk::{
	gdk_pixbuf::{Pixbuf, PixbufLoader},
	prelude::{BoxExt, ButtonExt, DialogExt, FileChooserExt, ImageExt, PixbufLoaderExt},
	Box as GtkBox, Button, FileChooserDialog, FileFilter, Image, ResponseType,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::value::{CoverMimeType, ParseBox};

const COVER_SIZE: i32 = 256;

fn picture_to_pixbuf(pic: &id3::frame::Picture) -> Option<Pixbuf> {
	let loader = PixbufLoader::new();
	loader.write(&pic.data).ok()?;
	loader.close().ok()?;
	loader
		.pixbuf()?
		.scale_simple(COVER_SIZE, COVER_SIZE, gtk::gdk_pixbuf::InterpType::Nearest)
}

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

		let raw_image = Rc::new(RefCell::new(None));

		Self {
			layout,
			image,
			change_btn,
			raw_image,
		}
	}

	fn update_state_opt(&self, state: &ParseBox) -> Option<(id3::frame::Picture, Pixbuf)> {
		let pic = state.front_cover()?.clone();
		let pixbuf = picture_to_pixbuf(&pic)?;
		Some((pic, pixbuf))
	}

	pub fn set_cover_just(&self, pic: id3::frame::Picture) {
		if let Some(pixbuf) = picture_to_pixbuf(&pic) {
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

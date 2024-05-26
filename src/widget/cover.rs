use std::path::PathBuf;

use gtk::gdk_pixbuf::{Pixbuf, PixbufLoader};
use gtk::{prelude::*, Box as GtkBox, Button, Image, Orientation};
use gtk::{FileChooserDialog, FileFilter, ResponseType};

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

pub struct CoverWidget {
	pub layout: GtkBox,
	info_layout: GtkBox,
	image: Image,
	change_btn: Button,
	remove_btn: Button,

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

		Self {
			info_layout,
			layout,
			image,
			change_btn,
			remove_btn,
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

			self.set_pixbuf(pixbuf);
		} else {
			self.image.set_pixbuf(None);
		}
	}

	pub fn update_cover_from_path(&self, path: &PathBuf) {
		match Pixbuf::from_file(path) {
			Err(e) => self.tx.error(e),
			Ok(pixbuf) => {
				self.set_pixbuf(Some(pixbuf));
			}
		}
	}

	pub fn remove_cover(&self) {
		self.image.set_pixbuf(None);
	}

	pub fn get_pixbuf_bytes(&self) -> Option<Vec<u8>> {
		self.image.pixbuf()?.save_to_bufferv("png", &[]).ok()
	}

	fn set_pixbuf(&self, pixbuf: Option<Pixbuf>) {
		let pixbuf = pixbuf.and_then(|pixbuf| {
			pixbuf.scale_simple(COVER_SIZE, COVER_SIZE, gtk::gdk_pixbuf::InterpType::Nearest)
		});

		self.image.set_pixbuf(pixbuf.as_ref());
	}
}

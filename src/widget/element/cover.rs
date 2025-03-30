use gtk::{
	gdk_pixbuf::{Pixbuf, PixbufLoader},
	prelude::{BoxExt, ImageExt, PixbufLoaderExt},
	Box as GtkBox, Button, Image,
};
use std::{cell::RefCell, rc::Rc};

use crate::value::ParseBox;

const COVER_SIZE: i32 = 128;

fn picture_to_pixbuf(pic: &id3::frame::Picture) -> Option<Pixbuf> {
	let loader = PixbufLoader::new();
	loader.write(&pic.data).ok()?;
	loader.close().ok()?;
	loader
		.pixbuf()?
		.scale_simple(COVER_SIZE, COVER_SIZE, gtk::gdk_pixbuf::InterpType::Nearest)
}

pub struct Cover {
	pub layout: GtkBox,
	image: Image,
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
			raw_image,
		}
	}

	fn update_state_opt(&self, state: &ParseBox) -> Option<(id3::frame::Picture, Pixbuf)> {
		let pic = state.front_cover()?.clone();
		let pixbuf = picture_to_pixbuf(&pic)?;
		Some((pic, pixbuf))
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
}

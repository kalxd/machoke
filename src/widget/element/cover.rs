use std::{cell::RefCell, rc::Rc};

use gtk::{
	gdk_pixbuf::Pixbuf,
	prelude::{BoxExt, ImageExt},
	Box as GtkBox, Image,
};

use crate::value::{ParseBox, SlimImage};

const COVER_SIZE: i32 = 128;

pub struct Cover {
	pub layout: GtkBox,
	image: Image,
	raw_image: Rc<RefCell<Option<SlimImage>>>,
}

impl Cover {
	pub fn new() -> Self {
		let layout = GtkBox::builder().build();

		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		layout.pack_start(&image, false, false, 0);

		let raw_image = Rc::new(RefCell::new(None));

		Self {
			layout,
			image,
			raw_image,
		}
	}

	fn update_state_opt(&self, state: &ParseBox) -> Option<(SlimImage, Pixbuf)> {
		let pic = state.front_cover()?;
		let slim_image = SlimImage::from(pic);
		let pixbuf = slim_image.to_pixbuf()?;
		let pixbuf =
			pixbuf.scale_simple(COVER_SIZE, COVER_SIZE, gtk::gdk_pixbuf::InterpType::Nearest)?;

		Some((slim_image, pixbuf))
	}

	pub fn update_state(&self, state: &ParseBox) {
		match self.update_state_opt(state) {
			Some((slim_image, pixbuf)) => {
				self.image.set_pixbuf(Some(&pixbuf));
				self.raw_image.replace(Some(slim_image));
			}
			None => {
				self.image.set_pixbuf(None);
				self.raw_image.take();
			}
		}
	}
}

use gtk::{prelude::BoxExt, Box as GtkBox, Image};

const COVER_SIZE: i32 = 128;

pub struct Cover {
	pub layout: GtkBox,
	image: Image,
}

impl Cover {
	pub fn new() -> Self {
		let layout = GtkBox::builder().build();

		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		layout.pack_start(&image, false, false, 0);

		Self { layout, image }
	}
}

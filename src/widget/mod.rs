use gtk::{traits::BoxExt, Box as GtkBox, Button, Image};

const COVER_SIZE: i32 = 128;

pub struct CoverWidget {
	image: Image,
	pub layout: GtkBox,
}

impl CoverWidget {
	pub fn new() -> Self {
		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Horizontal)
			.build();

		let change_btn = Button::builder().sensitive(false).label("更换封面").build();
		let remove_btn = Button::builder().sensitive(false).label("删除封面").build();
		let btn_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();
		btn_layout.pack_start(&change_btn, false, false, 0);
		btn_layout.pack_end(&remove_btn, false, false, 0);

		layout.pack_start(&image, false, false, 0);
		layout.pack_start(&btn_layout, false, false, 0);

		Self { image, layout }
	}
}

use gtk::{prelude::*, Box as GtkBox, Button, Image, Label, Orientation};

const COVER_SIZE: i32 = 128;

pub struct CoverWidget {
	pub info_layout: GtkBox,
	pub layout: GtkBox,
}

impl CoverWidget {
	pub fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
			.sensitive(false)
			.spacing(20)
			.build();

		let image = Image::builder()
			.width_request(COVER_SIZE)
			.height_request(COVER_SIZE)
			.build();
		layout.pack_start(&image, false, true, 10);

		let info_layout = GtkBox::builder()
			.orientation(Orientation::Vertical)
			.spacing(20)
			.build();

		let label = Label::new(Some("信息"));
		info_layout.pack_start(&label, false, false, 0);

		let btn_layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
			.spacing(20)
			.build();
		let change_btn = Button::with_label("change");
		btn_layout.pack_start(&change_btn, false, false, 0);
		let remove_btn = Button::with_label("remove");
		btn_layout.pack_start(&remove_btn, false, false, 0);
		info_layout.pack_start(&btn_layout, false, false, 0);
		layout.pack_start(&info_layout, false, true, 0);

		Self {
			info_layout,
			layout,
		}
	}

	pub fn hide_something(&self) {
		self.info_layout.hide();
	}
}

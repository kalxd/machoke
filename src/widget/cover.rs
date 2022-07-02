use gtk::gdk_pixbuf::PixbufLoader;
use gtk::{prelude::*, Box as GtkBox, Button, Image, Label, Orientation};
use id3::frame::PictureType;

const COVER_SIZE: i32 = 128;

pub struct CoverWidget {
	pub info_layout: GtkBox,
	pub layout: GtkBox,

	image: Image,
}

impl CoverWidget {
	pub fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(Orientation::Horizontal)
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
			image,
		}
	}

	pub fn hide_something(&self) {
		self.info_layout.hide();
	}

	pub fn update(&self, tag: &id3::Tag) {
		self.info_layout.show();

		let picture = tag
			.pictures()
			.find(|p| p.picture_type == PictureType::CoverFront);

		if let Some(picture) = picture {
			let loader = PixbufLoader::new();

			let pixbuf = loader
				.write(&picture.data)
				.and_then(|_| loader.close())
				.ok()
				.and_then(|_| loader.pixbuf());

			self.image.set_pixbuf(pixbuf.as_ref());
		}
	}
}

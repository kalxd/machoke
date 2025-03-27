use gtk::{
	prelude::{BoxExt, ContainerExt},
	Box as GtkBox, Frame,
};

const LAYOUT_SPACEING: u32 = 10;

use super::element::cover;

pub struct Editor {
	pub layout: GtkBox,
}

impl Editor {
	pub fn new() -> Self {
		let cover = cover::Cover::new();

		let cur_album_frame = Frame::builder().label("封面").build();
		cur_album_frame.set_child(Some(&cover.layout));

		let history_alubm_frame = Frame::builder().label("历史封面").build();

		let album_layout = GtkBox::builder().build();
		album_layout.pack_start(&cur_album_frame, true, true, LAYOUT_SPACEING);
		album_layout.pack_start(&history_alubm_frame, true, true, LAYOUT_SPACEING);

		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();
		layout.pack_start(&album_layout, true, true, 10);

		Self { layout }
	}
}

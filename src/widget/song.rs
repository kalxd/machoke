use gtk::{prelude::*, Box as GtkBox, Frame};

use super::{cover::CoverWidget, form::MetaForm};

pub struct SongWidget {
	cover: CoverWidget,
	pub layout: GtkBox,
}

impl SongWidget {
	pub fn new() -> Self {
		let layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.sensitive(false)
			.spacing(10)
			.build();

		let cover = CoverWidget::new();
		let frame = Frame::builder().label("封面").build();
		frame.add(&cover.layout);
		layout.pack_start(&frame, false, false, 10);

		let w = MetaForm::new();
		let frame = Frame::builder().label("详情").build();
		frame.add(&w.layout);
		layout.pack_start(&frame, false, false, 10);

		Self { cover, layout }
	}

	pub fn hide_something(&self) {
		self.cover.hide_something();
	}
}

use gtk::{prelude::*, Box as GtkBox, Frame};

pub struct MetaForm {
	pub layout: Frame,
}

impl MetaForm {
	pub fn new() -> Self {
		let frame = Frame::new(Some("基本信息"));
		frame.set_sensitive(false);

		Self { layout: frame }
	}
}

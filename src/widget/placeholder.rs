use gtk::{prelude::BoxExt, Box as GtkBox, Label};

pub struct Placeholder {
	pub layout: GtkBox,
}

impl Placeholder {
	pub fn new() -> Self {
		let layout = GtkBox::builder().build();

		let label = Label::builder()
			.label("点击左上角选择文件，或将文件拖入此间")
			.build();

		layout.pack_start(&label, true, true, 0);

		Self { layout }
	}
}

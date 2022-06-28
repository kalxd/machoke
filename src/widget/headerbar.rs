use gtk::{prelude::*, HeaderBar};

pub struct TitleBar {
	pub bar: HeaderBar,
}

impl TitleBar {
	pub fn new() -> Self {
		let bar = HeaderBar::builder()
			.title("音频元信息编辑器")
			.show_close_button(true)
			.build();

		Self { bar }
	}
}

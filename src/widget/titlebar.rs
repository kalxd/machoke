use std::ops::Deref;

use gtk::{prelude::HeaderBarExt, Button, HeaderBar, IconSize, Image};

pub struct TitleBar {
	layout: HeaderBar,
}

impl TitleBar {
	pub fn new() -> Self {
		let header = HeaderBar::builder()
			.title("音频元信息编辑器")
			.show_close_button(true)
			.build();

		let open_btn = Button::builder()
			.image(&Image::from_icon_name(
				Some("document-new"),
				IconSize::Button,
			))
			.tooltip_text("打开新音频")
			.build();
		header.pack_start(&open_btn);

		Self { layout: header }
	}
}

impl Deref for TitleBar {
	type Target = HeaderBar;

	fn deref(&self) -> &Self::Target {
		&self.layout
	}
}

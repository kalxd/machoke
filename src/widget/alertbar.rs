use std::ops::Deref;

use gtk::{
	prelude::{BoxExt, InfoBarExt, LabelExt, WidgetExt},
	InfoBar, Label, MessageType,
};

pub struct AlertBar {
	bar: InfoBar,
	label: Label,
}

impl AlertBar {
	pub fn new() -> Self {
		let bar = InfoBar::builder()
			.show_close_button(true)
			.visible(false)
			.build();

		bar.connect_response(|infobar, _| {
			infobar.hide();
		});

		let label = Label::new(None);
		bar.content_area().pack_start(&label, false, false, 0);

		Self { bar, label }
	}

	pub fn show<T: AsRef<str>>(&self, typ: MessageType, msg: T) {
		self.bar.set_message_type(typ);
		self.label.set_text(msg.as_ref());
		self.bar.show();
	}
}

impl Deref for AlertBar {
	type Target = InfoBar;

	fn deref(&self) -> &Self::Target {
		&self.bar
	}
}

pub struct PathBar {
	bar: InfoBar,
	label: Label,
}

impl PathBar {
	pub fn new() -> Self {
		let bar = InfoBar::builder()
			.show_close_button(false)
			.message_type(MessageType::Other)
			.build();
		let label = Label::new(Some("hehehe"));
		bar.content_area().pack_start(&label, true, true, 0);

		Self { bar, label }
	}

	pub fn set_text(&self, text: &str) {
		self.label.set_text(text);
	}
}

impl Deref for PathBar {
	type Target = InfoBar;

	fn deref(&self) -> &Self::Target {
		&self.bar
	}
}

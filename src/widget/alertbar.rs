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

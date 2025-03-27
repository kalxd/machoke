use std::{ops::Deref, path::PathBuf};

use gtk::{
	prelude::{ButtonExt, DialogExt, FileChooserExt, HeaderBarExt},
	Button, FileChooserDialog, FileFilter, HeaderBar, IconSize, Image, ResponseType,
};

pub struct TitleBar {
	layout: HeaderBar,
	open_btn: Button,
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

		Self {
			layout: header,
			open_btn,
		}
	}

	pub fn connect_open_audio<F>(&self, f: F)
	where
		F: Fn(PathBuf) + 'static,
	{
		self.open_btn.connect_clicked(move |_| {
			let filter = FileFilter::new();
			filter.add_mime_type("audio/*");

			let dialog = FileChooserDialog::builder()
				.title("选择一个小音频")
				.action(gtk::FileChooserAction::Open)
				.select_multiple(false)
				.filter(&filter)
				.build();

			dialog.add_button("确定", ResponseType::Accept);

			if dialog.run() == ResponseType::Accept {
				if let Some(path) = dialog.filename() {
					f(path)
				}
			}

			dialog.emit_close();
		});
	}
}

impl Deref for TitleBar {
	type Target = HeaderBar;

	fn deref(&self) -> &Self::Target {
		&self.layout
	}
}

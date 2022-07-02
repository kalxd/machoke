use std::path::PathBuf;

use gtk::{
	prelude::*, Button, FileChooserDialog, FileFilter, HeaderBar, IconSize, Image, ResponseType,
};

pub struct TitleBar {
	pub bar: HeaderBar,
	open_chooser_btn: Button,
	pub save_btn: Button,
	pub save_as_btn: Button,
}

impl TitleBar {
	pub fn new() -> Self {
		let bar = HeaderBar::builder()
			.title("音频元信息编辑器")
			.show_close_button(true)
			.build();

		let open_chooser_btn = Button::builder()
			.image(&Image::from_icon_name(
				Some("document-new"),
				IconSize::Button,
			))
			.tooltip_text("打开新音频")
			.build();

		bar.pack_start(&open_chooser_btn);

		let save_btn = Button::builder()
			.image(&Image::from_icon_name(
				Some("document-save"),
				IconSize::Button,
			))
			.sensitive(false)
			.tooltip_text("就地保存")
			.build();

		let save_as_btn = Button::builder()
			.image(&Image::from_icon_name(
				Some("document-save-as"),
				IconSize::Button,
			))
			.sensitive(false)
			.tooltip_text("另存为新音频文件")
			.build();
		bar.pack_end(&save_as_btn);
		bar.pack_end(&save_btn);

		let widget = Self {
			bar,
			open_chooser_btn,
			save_btn,
			save_as_btn,
		};
		return widget;
	}

	pub fn connect_open_song<F: Fn(PathBuf) + 'static>(&self, f: F) {
		self.open_chooser_btn.connect_clicked(move |_| {
			let filter = FileFilter::new();
			filter.add_mime_type("audio/*");

			let dialog = FileChooserDialog::builder()
				.title("选择一个小音频")
				.action(gtk::FileChooserAction::Open)
				.select_multiple(false)
				.filter(&filter)
				.build();

			dialog.add_button("确定", ResponseType::Accept);

			if ResponseType::Accept == dialog.run() {
				if let Some(path) = dialog.filename() {
					f(path);
				}
			}

			dialog.emit_close();
		});
	}
}

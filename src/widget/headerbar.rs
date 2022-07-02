use gtk::{glib, prelude::*, Button, HeaderBar, IconSize, Image};

enum TheAction {
	OpenFile,
}

pub struct TitleBar {
	pub bar: HeaderBar,
	open_chooser_btn: Button,
	save_btn: Button,
	save_as_btn: Button,
	tx: glib::Sender<TheAction>,
	rx: glib::Receiver<TheAction>,
}

impl TitleBar {
	pub fn new() -> Self {
		let (tx, rx) = glib::MainContext::channel::<TheAction>(glib::PRIORITY_DEFAULT);

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
			tx,
			rx,
			open_chooser_btn,
			save_btn,
			save_as_btn,
		};
		return widget;
	}

	fn setup_connection(&self) {}
}

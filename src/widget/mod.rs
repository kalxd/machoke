use gtk::{prelude::*, Application, ApplicationWindow};

mod headerbar;

pub struct MainWindow {
	window: ApplicationWindow,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.build();

		let title_bar = headerbar::TitleBar::new();

		window.set_titlebar(Some(&title_bar.bar));

		Self { window }
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.window.show_all();
	}
}

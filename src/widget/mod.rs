use gtk::{
	prelude::{GtkWindowExt, WidgetExt},
	Application, ApplicationWindow,
};

pub mod titlebar;

pub struct MainWindow {
	window: ApplicationWindow,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let titlebar = titlebar::TitleBar::new();

		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.icon_name("mochoke")
			.build();

		window.set_titlebar(Some(&*titlebar));

		Self { window }
	}

	fn show_all(&self) {
		self.window.show_all();
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.show_all();
	}
}

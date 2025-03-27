use gtk::{
	prelude::{ContainerExt, GtkWindowExt, WidgetExt},
	Application, ApplicationWindow,
};

mod placeholder;
mod titlebar;

pub struct MainWindow {
	window: ApplicationWindow,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.icon_name("mochoke")
			.build();

		let titlebar = titlebar::TitleBar::new();
		window.set_titlebar(Some(&*titlebar));
		titlebar.connect_open_audio(|p| {
			dbg!(p);
		});

		let placeholder = placeholder::Placeholder::new();
		window.set_child(Some(&placeholder.layout));

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

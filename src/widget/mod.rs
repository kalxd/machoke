use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox};

mod form;
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

		let main_layout = GtkBox::new(gtk::Orientation::Vertical, 10);

		let form = form::MetaForm::new();
		main_layout.pack_start(&form.layout, false, false, 0);

		window.add(&main_layout);

		Self { window }
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.window.show_all();
	}
}

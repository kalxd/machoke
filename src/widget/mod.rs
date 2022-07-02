use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox};

mod cover;
mod form;
mod headerbar;
mod song;

pub struct MainWindow {
	window: ApplicationWindow,
	widget: song::SongWidget,
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

		let main_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin(10)
			.build();

		let form = song::SongWidget::new();
		main_layout.pack_start(&form.layout, false, false, 0);

		window.add(&main_layout);

		Self {
			window,
			widget: form,
		}
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.window.show_all();
		main_window.widget.hide_something();
	}
}

use gtk::{
	prelude::{ContainerExt, GtkWindowExt, StackExt, WidgetExt},
	Application, ApplicationWindow, Stack,
};

mod placeholder;
mod titlebar;

enum StackName {
	Placeholder,
}

impl StackName {
	const fn as_str(&self) -> &'static str {
		match self {
			Self::Placeholder => "placeholder",
		}
	}
}

pub struct MainWindow {
	window: ApplicationWindow,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let placeholder = placeholder::Placeholder::new();

		let stack = Stack::builder()
			.transition_type(gtk::StackTransitionType::Crossfade)
			.build();
		stack.add_named(&placeholder.layout, StackName::Placeholder.as_str());

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

		window.set_child(Some(&stack));

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

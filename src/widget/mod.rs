use gtk::{
	prelude::{ContainerExt, GtkWindowExt, StackExt, WidgetExt},
	Application, ApplicationWindow, Stack,
};

mod editor;
mod element;
mod placeholder;
mod titlebar;

enum StackName {
	Placeholder,
	Editor,
}

impl StackName {
	const fn as_str(&self) -> &'static str {
		match self {
			Self::Placeholder => "placeholder",
			Self::Editor => "editor",
		}
	}
}

pub struct MainWindow {
	window: ApplicationWindow,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let placeholder = placeholder::Placeholder::new();

		let editor = editor::Editor::new();

		let stack = Stack::builder()
			.transition_type(gtk::StackTransitionType::Crossfade)
			.build();
		stack.add_named(&placeholder.layout, StackName::Placeholder.as_str());
		stack.add_named(&editor.layout, StackName::Editor.as_str());

		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.icon_name("mochoke")
			.build();

		let titlebar = titlebar::TitleBar::new();
		window.set_titlebar(Some(&*titlebar));
		window.set_child(Some(&stack));

		titlebar.connect_open_audio(move |p| {
			stack.set_visible_child_name(StackName::Editor.as_str());
		});

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

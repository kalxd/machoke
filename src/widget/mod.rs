use gtk::{
	prelude::{BoxExt, ContainerExt, GtkWindowExt, StackExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox, Stack,
};

mod alertbar;
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
	alertbar: alertbar::AlertBar,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let alertbar = alertbar::AlertBar::new();

		let placeholder = placeholder::Placeholder::new();

		let editor = editor::Editor::new();

		let stack = Stack::builder()
			.transition_type(gtk::StackTransitionType::Crossfade)
			.build();
		stack.add_named(&placeholder.layout, StackName::Placeholder.as_str());
		stack.add_named(&editor.layout, StackName::Editor.as_str());

		let main_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.build();
		main_layout.pack_start(&*alertbar, false, false, 0);
		main_layout.pack_start(&stack, true, true, 0);

		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.icon_name("mochoke")
			.build();

		let titlebar = titlebar::TitleBar::new();
		window.set_titlebar(Some(&*titlebar));
		window.set_child(Some(&main_layout));

		titlebar.connect_open_audio(move |p| {
			stack.set_visible_child_name(StackName::Editor.as_str());
		});

		Self { window, alertbar }
	}

	fn show_all(&self) {
		self.window.show_all();
		self.alertbar.hide();
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.show_all();
	}
}

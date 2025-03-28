use futures::{future::ready, StreamExt};
use gtk::{
	glib,
	prelude::{BoxExt, ContainerExt, GtkWindowExt, StackExt, WidgetExt},
	Application, ApplicationWindow, Box as GtkBox, Stack,
};

mod alertbar;
mod editor;
mod element;
mod placeholder;
mod titlebar;

use crate::value::{self, EventAction, EventSender, ParseBox};

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
	stack: Stack,

	editor: editor::Editor,
}

impl MainWindow {
	fn new(app: &Application, tx: EventSender) -> Self {
		let window = ApplicationWindow::builder()
			.application(app)
			.default_height(600)
			.default_width(800)
			.icon_name("mochoke")
			.build();

		let main_layout = GtkBox::builder()
			.orientation(gtk::Orientation::Vertical)
			.margin(10)
			.build();
		window.set_child(Some(&main_layout));

		let alertbar = alertbar::AlertBar::new();
		main_layout.pack_start(&*alertbar, false, false, 0);

		let stack = Stack::builder()
			.transition_type(gtk::StackTransitionType::Crossfade)
			.build();
		main_layout.pack_start(&stack, true, true, 0);

		let placeholder = placeholder::Placeholder::new();
		stack.add_named(&placeholder.layout, StackName::Placeholder.as_str());

		let editor = editor::Editor::new();
		stack.add_named(&editor.layout, StackName::Editor.as_str());

		let titlebar = titlebar::TitleBar::new();
		window.set_titlebar(Some(&*titlebar));
		titlebar.connect_open_audio({
			let tx = tx.clone();
			move |p| tx.send(EventAction::OpenAudio(p))
		});

		Self {
			window,
			alertbar,
			stack,
			editor,
		}
	}

	fn show_all(&self) {
		self.window.show_all();
		self.alertbar.hide();
	}

	fn update_state(&self, state: ParseBox) {
		self.editor.update_state(&state);
	}

	pub fn run(app: &Application) {
		let (tx, rx) = value::channel();

		let main_window = Self::new(app, tx);
		main_window.show_all();

		glib::MainContext::default().spawn_local(async move {
			rx.for_each(|msg| {
				match msg {
					EventAction::OpenAudio(p) => match value::ParseBox::parse_from_path(p) {
						Ok((t, msg)) => {
							if let Some(msg) = msg {
								main_window.alertbar.show(msg.0, msg.1);
							}

							main_window.update_state(t);
							main_window
								.stack
								.set_visible_child_name(StackName::Editor.as_str());
						}
						Err(e) => main_window
							.alertbar
							.show(gtk::MessageType::Error, e.to_string()),
					},
					_ => {}
				};

				ready(())
			})
			.await;
		});
	}
}

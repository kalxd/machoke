use std::{cell::RefCell, path::PathBuf, rc::Rc};

use gtk::{
	prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, ButtonBox,
	FileChooserButton, FileFilter, Frame,
};
use tag::read_audio_tag_from_path;
use widget::{CoverWidget, FormWidget};

mod t;
mod tag;
mod widget;

#[derive(Clone, Debug)]
struct AppState {
	audio_path: Rc<RefCell<Option<PathBuf>>>,
}

impl Default for AppState {
	fn default() -> Self {
		Self {
			audio_path: Default::default(),
		}
	}
}

struct MainWindow {
	state: AppState,
	audio_chooser: FileChooserButton,
	cover_widget: CoverWidget,
	form_widget: FormWidget,
	ok_btn: Button,
}

impl MainWindow {
	fn new() -> Self {
		let state = AppState::default();
		let audio_chooser = FileChooserButton::builder()
			.title("歌曲")
			.margin(10)
			.action(gtk::FileChooserAction::Open)
			.filter(&{
				let f = FileFilter::new();
				f.add_mime_type("audio/*");
				f
			})
			.build();
		let cover_widget = widget::CoverWidget::new();
		let form_widget = FormWidget::new();
		let ok_btn = Button::builder().label("好").build();

		Self {
			state,
			audio_chooser,
			cover_widget,
			form_widget,
			ok_btn,
		}
	}

	fn run(app: &Application) {
		let main_window = Self::new();

		let window = ApplicationWindow::builder()
			.application(app)
			.title("我的窗口")
			.default_width(800)
			.default_height(600)
			.build();

		let main_layout = GtkBox::builder()
			.margin(10)
			.orientation(gtk::Orientation::Vertical)
			.spacing(10)
			.build();

		{
			let chooser = main_window.audio_chooser.clone();
			let frame = Frame::builder().label("选择歌曲").build();
			frame.add(&chooser);
			main_layout.pack_start(&frame, false, true, 10);

			let audio_path = main_window.state.audio_path.clone();
			let cover_widget = main_window.cover_widget.clone();
			let form_widget = main_window.form_widget.clone();
			main_window
				.audio_chooser
				.clone()
				.connect_file_set(move |file_btn| {
					let path = file_btn.filename();

					if let Some(path) = &path {
						match read_audio_tag_from_path(path) {
							Ok(tag) => {
								form_widget.set_form_state(&tag);
								cover_widget.update_cover(&tag);
							}
							Err(e) => eprintln!("{:?}", e),
						};
					}
					audio_path.replace(path);
				});
		}

		{
			let cover_frame = Frame::builder().label("封面设置").build();
			cover_frame.add(&main_window.cover_widget.layout);
			main_layout.pack_start(&cover_frame, false, false, 10);
		}

		{
			let frame = Frame::builder().label("属性").build();
			frame.add(&main_window.form_widget.layout);
			main_layout.pack_start(&frame, false, false, 0);
		}

		{
			let button_box = ButtonBox::builder()
				.layout_style(gtk::ButtonBoxStyle::End)
				.build();
			let state = main_window.state.clone();
			main_window.ok_btn.clone().connect_clicked(move |_| {
				dbg!(&state);
			});
			button_box.pack_start(&main_window.ok_btn, false, false, 0);
			main_layout.pack_end(&button_box, false, false, 0);
		}

		window.add(&main_layout);
		window.show_all();
	}
}

fn main() {
	let app = Application::builder().application_id("xgley.com").build();

	app.connect_activate(|app| MainWindow::run(app));

	app.run();
}

use gtk::{
	prelude::*, Application, ApplicationWindow, Box as GtkBox, Button, ButtonBox, DialogFlags,
	FileChooserButton, FileFilter, Frame, MessageDialog,
};
use id3::Tag;
use t::{AppState, AudioBaseInfo};
use widget::{CoverWidget, FormWidget};

mod t;
mod widget;

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

			let cover_widget = main_window.cover_widget.clone();
			let form_widget = main_window.form_widget.clone();
			let tag = main_window.state.tag.clone();
			let audio_path = main_window.state.audio_path.clone();
			main_window.audio_chooser.connect_file_set(move |file_btn| {
				let filepath = file_btn.filename();
				audio_path.replace(filepath.clone());

				let new_tag = file_btn
					.filename()
					.and_then(|path| Tag::read_from_path(path).ok());

				if let Some(tag) = &new_tag {
					let info = AudioBaseInfo::from(tag);
					form_widget.set_form_state(&info);
					cover_widget.update_cover(&info);
				}

				tag.replace(new_tag);
			});
		}

		{
			let cover_frame = Frame::builder().label("封面设置").build();
			cover_frame.add(&main_window.cover_widget.layout);
			main_layout.pack_start(&cover_frame, false, false, 10);

			let cover_path = main_window.state.new_cover_path.clone();
			main_window.cover_widget.connect_cover_changed(move |path| {
				cover_path.replace(Some(path));
			});

			let cover_path = main_window.state.new_cover_path.clone();
			main_window.cover_widget.connect_cover_remove(move || {
				cover_path.replace(None);
			});
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
			let form = main_window.form_widget.clone();
			main_window.ok_btn.clone().connect_clicked(move |_| {
				let form_state = form.get_form_state();

				let msg = match state.save(&form_state) {
					Ok(_) => "保存成功".into(),
					Err(msg) => msg,
				};

				let dialog = MessageDialog::new::<ApplicationWindow>(
					None,
					DialogFlags::MODAL,
					gtk::MessageType::Info,
					gtk::ButtonsType::Ok,
					&msg,
				);
				dialog.run();
				dialog.emit_close();
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

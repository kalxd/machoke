use gtk::{
	prelude::*, Application, ApplicationWindow, Box as GtkBox, FileChooserButton, FileFilter, Frame,
};

mod widget;

fn gui_main(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("我的窗口")
		.default_width(800)
		.default_height(600)
		.build();

	let main_layout = GtkBox::builder()
		.orientation(gtk::Orientation::Vertical)
		.spacing(10)
		.build();

	let file_chooser = FileChooserButton::builder()
		.title("歌曲")
		.margin(10)
		.action(gtk::FileChooserAction::Open)
		.filter(&{
			let f = FileFilter::new();
			f.add_mime_type("audio/*");
			f
		})
		.build();
	{
		file_chooser.connect_file_set(move |file_btn| {
			println!("do this?");
			dbg!(file_btn.file());
		});
	}

	let frame = Frame::builder().label("选择歌曲").build();
	frame.add(&file_chooser);
	main_layout.pack_start(&frame, false, true, 10);

	let cover_widget = widget::CoverWidget::new();
	let cover_frame = Frame::builder().label("封面设置").build();
	cover_frame.add(&cover_widget.layout);

	main_layout.pack_start(&cover_frame, false, false, 10);

	window.add(&main_layout);
	window.show_all();
}

fn main() {
	let app = Application::builder().application_id("xgley.com").build();

	app.connect_activate(|app| gui_main(app));

	app.run();
}

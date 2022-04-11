use gtk::{prelude::*, Application, ApplicationWindow, Frame, Orientation, Paned};

mod widget;

fn gui_main(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("我的窗口")
		.default_width(800)
		.default_height(600)
		.build();

	let main_layout = Paned::new(Orientation::Horizontal);

	let form_layout = widget::SongForm::new();
	main_layout.add1(&form_layout.layout);

	let file_chooser = widget::FileWidget::new();
	let right_layout = Frame::builder().label("文件区").build();
	right_layout.add(&*file_chooser);
	main_layout.add2(&right_layout);

	window.add(&main_layout);
	window.show_all();
}

fn main() {
	let app = Application::builder().application_id("xgley.com").build();

	app.connect_activate(|app| gui_main(app));

	app.run();
}

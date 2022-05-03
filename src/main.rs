use gtk::{prelude::*, Application, ApplicationWindow, Box as GtkBox, Frame};

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
		.build();

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

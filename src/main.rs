use gtk::{prelude::*, Application};

mod emitter;
mod value;
mod widget;

fn main() {
	let app = Application::builder()
		.application_id("com.xgley.machoke")
		.build();

	app.connect_activate(widget::MainWindow::run);

	app.run();
}

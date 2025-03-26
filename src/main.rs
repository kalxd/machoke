use gtk::{
	prelude::{ApplicationExt, ApplicationExtManual},
	Application,
};

mod widget;

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.machoke")
		.build();

	app.connect_activate(widget::MainWindow::run);

	app.run();
}

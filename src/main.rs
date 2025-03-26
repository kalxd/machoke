use gtk::{
	prelude::{ApplicationExt, ApplicationExtManual, WidgetExt},
	Application, ApplicationWindow,
};

fn setup_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.default_height(600)
		.default_width(800)
		.icon_name("mochoke")
		.build();

	window.show();
}

fn main() {
	let app = Application::builder()
		.application_id("person.xgley.machoke")
		.build();

	app.connect_activate(setup_ui);

	app.run();
}

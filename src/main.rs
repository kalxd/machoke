use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn gui_main(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("我的窗口")
        .default_width(800)
        .default_height(600)
        .build();
    window.show_all();
}

fn main() {
    let app = Application::builder().application_id("xgley.com").build();

    app.connect_activate(|app| gui_main(app));

    app.run();
}

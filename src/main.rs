use gtk::{prelude::*, Entry, Label};
use gtk::{Application, ApplicationWindow, Box as GtkBox, SizeGroup};

fn create_form_row(label: &str, size_group: &SizeGroup) -> GtkBox {
    let layout = GtkBox::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .build();

    let label = Label::new(Some(&label));
    size_group.add_widget(&label);
    layout.pack_start(&label, false, false, 1);

    let entry = Entry::new();
    layout.pack_start(&entry, true, true, 0);

    return layout;
}

fn gui_form() -> GtkBox {
    let layout = GtkBox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let size_group = SizeGroup::new(gtk::SizeGroupMode::Both);
    let name_row = create_form_row("name", &size_group);
    let user_row = create_form_row("user", &size_group);

    layout.pack_start(&name_row, false, false, 0);
    layout.pack_start(&user_row, false, false, 0);
    return layout;
}

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

    let form_layout = gui_form();
    main_layout.pack_start(&form_layout, false, false, 0);

    window.add(&main_layout);
    window.show_all();
}

fn main() {
    let app = Application::builder().application_id("xgley.com").build();

    app.connect_activate(|app| gui_main(app));

    app.run();
}

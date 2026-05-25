use gtk::{Application, ApplicationWindow, Box, Button, ListBox, ListBoxRow, Orientation};
use gtk::{Label, prelude::*};

fn create_sidebar_row(name: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_child(Some(&Label::new(Some(name))));
    row
}

pub fn build_ui(app: &Application) {
    let label = Label::new(Some("Hello from GTK + Rust"));
    let button = Button::with_label("Click me");

    let label_clone = label.clone();
    button.connect_clicked(move |_| {
        label_clone.set_label("You clicked the button!");
    });

    let hlayout = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();

    let sidebar = ListBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .width_request(180)
        .vexpand(true)
        .build();
    sidebar.add_css_class("navigation-sidebar");

    let disks_menu_item = create_sidebar_row("Disks");
    let mounts_menu_item = create_sidebar_row("Mounts");

    sidebar.append(&disks_menu_item);
    sidebar.append(&mounts_menu_item);

    let main_layout = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .hexpand(true)
        .vexpand(true)
        .build();

    main_layout.add_css_class("main-area");

    main_layout.append(&label);
    main_layout.append(&button);

    hlayout.append(&sidebar);
    hlayout.append(&main_layout);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Rust Test")
        .default_width(600)
        .default_height(400)
        .child(&hlayout)
        .build();

    window.present();
}

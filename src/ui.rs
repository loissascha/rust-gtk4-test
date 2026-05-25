use gtk::{Application, ApplicationWindow, Box, Button, ListBox, ListBoxRow, Orientation};
use gtk::{Label, prelude::*};

pub fn build_ui(app: &Application) {
    let label = Label::new(Some("Hello from GTK + Rust"));
    let button = Button::with_label("Click me");

    let label_clone = label.clone();
    button.connect_clicked(move |_| {
        label_clone.set_label("You clicked the button!");
    });

    let left_button1 = Button::with_label("Left 1");
    left_button1.set_hexpand(true);
    left_button1.set_halign(gtk::Align::Fill);

    let left_button2 = Button::with_label("Left 2");
    left_button2.set_hexpand(true);
    left_button2.set_halign(gtk::Align::Fill);

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

    let row1 = ListBoxRow::new();
    row1.set_child(Some(&gtk::Label::new(Some("Home"))));

    let row2 = ListBoxRow::new();
    row2.set_child(Some(&gtk::Label::new(Some("Videos"))));

    sidebar.append(&row1);
    sidebar.append(&row2);

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

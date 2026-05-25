use gtk::{Application, ApplicationWindow, Box, Button, ListBox, ListBoxRow, Orientation, Stack};
use gtk::{Label, prelude::*};

fn create_sidebar_row(name: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_child(Some(&Label::new(Some(name))));
    row
}

fn mounts_page() -> Box {
    let main_layout = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .hexpand(true)
        .vexpand(true)
        .build();

    let label = Label::new(Some("Hello from GTK + Rust"));
    let button = Button::with_label("Click me");

    let label_clone = label.clone();
    button.connect_clicked(move |_| {
        label_clone.set_label("You clicked the button!");
    });

    main_layout.append(&label);
    main_layout.append(&button);

    main_layout
}

fn disks_page() -> Box {
    let main_layout = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .hexpand(true)
        .vexpand(true)
        .build();

    let label = Label::new(Some("Hello from Disks"));

    main_layout.append(&label);

    main_layout
}

fn build_sidebar(stack_clone: Stack) -> ListBox {
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

    sidebar.connect_row_selected(move |_list_box, row| {
        let Some(row) = row else {
            return;
        };

        match row.index() {
            0 => stack_clone.set_visible_child_name("disks"),
            1 => stack_clone.set_visible_child_name("mounts"),
            _ => {}
        }
    });
    sidebar.select_row(Some(&disks_menu_item));
    sidebar
}

pub fn build_ui(app: &Application) {
    let stack = Stack::builder().hexpand(true).vexpand(true).build();

    let disks_page = disks_page();
    let mounts_page = mounts_page();

    stack.add_named(&disks_page, Some("disks"));
    stack.add_named(&mounts_page, Some("mounts"));

    let stack_clone = stack.clone();
    let sidebar = build_sidebar(stack_clone);

    let hlayout = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();
    hlayout.append(&sidebar);
    hlayout.append(&stack);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Rust Test")
        .default_width(600)
        .default_height(400)
        .child(&hlayout)
        .build();

    window.present();
}

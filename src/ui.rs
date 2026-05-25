use gtk::{
    AboutDialog, Application, ApplicationWindow, Box, Button, HeaderBar, ListBox, ListBoxRow,
    MenuButton, Orientation, Stack, Window, gio,
};
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

fn build_stack() -> Stack {
    let stack = Stack::builder().hexpand(true).vexpand(true).build();

    let disks_page = disks_page();
    let mounts_page = mounts_page();

    stack.add_named(&disks_page, Some("disks"));
    stack.add_named(&mounts_page, Some("mounts"));
    stack
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

fn build_header_bar() -> HeaderBar {
    let header_bar = HeaderBar::builder().show_title_buttons(true).build();

    let title = Label::new(Some("My Disks App"));
    header_bar.set_title_widget(Some(&title));

    let menu = gio::Menu::new();

    menu.append(Some("Preferences"), Some("app.preferences"));
    menu.append(Some("About"), Some("app.about"));

    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();

    header_bar.pack_end(&menu_button);

    header_bar
}

fn add_app_actions(app: &Application, window: &ApplicationWindow) {
    let window_clone = window.clone();

    let preferences_action = gio::SimpleAction::new("preferences", None);
    preferences_action.connect_activate(move |_, _| {
        let dialog = gtk::Window::builder()
            .title("Preferences")
            .default_width(400)
            .default_height(300)
            .transient_for(&window_clone)
            .modal(true)
            .build();

        dialog.set_child(Some(&gtk::Label::new(Some("Preferences go here"))));
        dialog.present();
    });
    app.add_action(&preferences_action);

    let window_clone = window.clone();

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(move |_, _| {
        let dialog = AboutDialog::builder()
            .program_name("GTK Rust Test")
            .version("0.1.0")
            .comments("A small GTK app written in Rust.")
            .website("https://sascha-loishandl.at")
            .transient_for(&window_clone)
            .modal(true)
            .build();
        dialog.present();
    });
    app.add_action(&about_action);
}

pub fn build_ui(app: &Application) {
    let header_bar = build_header_bar();
    let stack = build_stack();

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
        .default_width(800)
        .default_height(600)
        .child(&hlayout)
        .build();
    window.set_titlebar(Some(&header_bar));

    add_app_actions(app, &window);

    window.present();
}

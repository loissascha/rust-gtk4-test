use gtk::{AboutDialog, Application, ApplicationWindow, HeaderBar, MenuButton, gio};
use gtk::{Label, prelude::*};

pub fn build_ui() -> HeaderBar {
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

pub fn add_app_actions(app: &Application, window: &ApplicationWindow) {
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

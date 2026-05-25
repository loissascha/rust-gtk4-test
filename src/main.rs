use gtk::Application;
use gtk::prelude::*;

mod ui;

fn main() {
    let app = Application::builder()
        .application_id("at.sascha.GtkTest")
        .build();

    app.connect_activate(ui::build_ui);

    app.run();
}

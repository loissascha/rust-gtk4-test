use gtk::Application;
use gtk::prelude::*;

mod fstab;
mod ui;

fn main() {
    match fstab::read_fstab() {
        Ok(content) => {
            println!("{content}");
        }
        Err(err) => {
            panic!("Could not read /etc/fstab: {err}");
        }
    }

    let app = Application::builder()
        .application_id("at.sascha.GtkTest")
        .build();

    app.connect_activate(ui::build_ui);

    app.run();
}

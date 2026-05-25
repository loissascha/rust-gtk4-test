use gtk::Application;
use gtk::prelude::*;

mod fstab;
mod ui;

fn main() {
    let mut ftab = fstab::Fstab::new();
    match ftab.read_fstab() {
        Ok(content) => {
            ftab.set_content(content);
        }
        Err(err) => {
            panic!("Could not read /etc/fstab: {err}");
        }
    }

    println!("found fstab");
    println!("{}", ftab.get_content());

    let app = Application::builder()
        .application_id("at.sascha.GtkTest")
        .build();

    app.connect_activate(ui::build_ui);

    app.run();
}

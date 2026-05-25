use gtk::Application;
use gtk::prelude::*;

mod fstab;
mod ui;

fn main() {
    let ftab = fstab::Fstab::new();

    // println!("---- found fstab ----");
    // println!("{}", ftab.content);
    // println!("---------------------");

    let app = Application::builder()
        .application_id("at.sascha.GtkTest")
        .build();

    app.connect_activate(ui::build_ui);
    app.connect_shutdown(|_| {
        println!("shutdown");
    });

    app.run();
}

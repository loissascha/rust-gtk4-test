use fstab::FsEntry;
use fstab::FsTab;
use gtk::Application;
use gtk::prelude::*;
use std::path::Path;

// mod fstab;
mod ui;

fn main() {
    // let ftab = fstab::Fstab::new();

    // println!("---- found fstab ----");
    // println!("{}", ftab.content);
    // println!("---------------------");
    readfstab();

    let app = Application::builder()
        .application_id("at.sascha.GtkTest")
        .build();

    app.connect_activate(ui::build_ui);
    app.connect_shutdown(|_| {
        println!("shutdown");
    });

    app.run();
}

fn readfstab() {
    let fstab = FsTab::new(Path::new("/etc/fstab"));
    let entries = fstab.get_entries().unwrap();

    for entry in entries {
        println!("Device: {}", entry.fs_spec);
        println!("Mountpoint: {:?}", entry.mountpoint);
        println!("Filesystem: {}", entry.vfs_type);
        println!("Options: {:?}", entry.mount_options);
        println!("Dump: {}", entry.dump);
        println!("Fsck order: {}", entry.fsck_order);
        println!("---");
    }
}

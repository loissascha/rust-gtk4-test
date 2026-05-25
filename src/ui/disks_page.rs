use gtk::{
    AboutDialog, Application, ApplicationWindow, Box, HeaderBar, MenuButton, Orientation, Stack,
    gio,
};
use gtk::{Label, prelude::*};

pub fn build_ui() -> Box {
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

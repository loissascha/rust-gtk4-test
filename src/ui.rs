pub mod disks_page;
pub mod headerbar;
pub mod mounts_page;
pub mod sidebar;

use gtk::{Application, ApplicationWindow, Box, Orientation, Stack, prelude::*};

fn build_stack() -> Stack {
    let stack = Stack::builder().hexpand(true).vexpand(true).build();

    let disks_page = disks_page::build_ui();
    let mounts_page = mounts_page::build_ui();

    stack.add_named(&disks_page, Some("disks"));
    stack.add_named(&mounts_page, Some("mounts"));
    stack
}

pub fn build_ui(app: &Application) {
    let header_bar = headerbar::build_ui();
    let stack = build_stack();

    let stack_clone = stack.clone();
    let sidebar = sidebar::build_sidebar(stack_clone);

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

    headerbar::add_app_actions(app, &window);

    window.present();
}

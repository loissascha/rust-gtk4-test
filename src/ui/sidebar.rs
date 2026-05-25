use gtk::{Label, ListBox, ListBoxRow, Stack, prelude::*};

pub fn build_sidebar(stack_clone: Stack) -> ListBox {
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

fn create_sidebar_row(name: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_child(Some(&Label::new(Some(name))));
    row
}

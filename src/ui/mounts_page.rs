use gtk::{Box, Button, Orientation};
use gtk::{Label, prelude::*};

pub fn build_ui() -> Box {
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

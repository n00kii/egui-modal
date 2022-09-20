# egui-modal, a modal library for [`egui`](https://github.com/emilk/egui)
<!-- [![crates.io]()]() -->
<!-- [![docs]()]() -->
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/n00kii/egui-modal/blob/main/README.md)

![modal](media/modal.png)

## usage:
```rust
    let modal = Modal::new(ctx, "my_modal");

    // What goes inside the modal
    modal.show(|ui| {
        modal.title(ui, "My modal");
        modal.body(ui, "This is a modal");
        modal.buttons(ui, |ui| {
            // After clicking, the modal is automatically closed
            if modal.button("A button").clicked() {
                println!("Hello world!")
            };
        });
    });

    if ui.button("Open the modal").clicked() {
        // Show the modal
        modal.open();
    }
```
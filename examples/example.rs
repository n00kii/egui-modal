use eframe;
use egui::{self, DragValue};
use egui_modal::{Modal, ModalStyle};

struct ExampleApp {
    modal_style: ModalStyle,
    modal_title: String,
    modal_body: String,

    include_title: bool,
    include_body: bool,
    include_buttons: bool,
    close_on_outside_click: bool,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            modal_style: ModalStyle::default(),
            modal_title: "a modal".to_string(),
            modal_body: "here is the modal body".to_string(),

            include_title: true,
            include_body: true,
            include_buttons: true,
            close_on_outside_click: false,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("egui-modal").show(ctx, |ui| {
            // you can put the modal creation and show logic wherever you want
            // (though of course it needs to be created before it can be used)
            let nested_modal = Modal::new(ctx, "nested_modal");
            let modal = Modal::new(ctx, "modal")
                .with_style(&self.modal_style)
                .with_close_on_outside_click(self.close_on_outside_click || !self.include_buttons);

            // the show function defines what is shown in the modal, but the modal
            // won't actually show until you do modal.open(ctx)
            modal.show(|ui| {
                // these helper functions are NOT mandatory to use, they just
                // help implement some styling with margins and separators
                // you can put whatever you like in here
                if self.include_title {
                    modal.title(ui, &mut self.modal_title);
                }
                if self.include_body {
                    modal.body(ui, &mut self.modal_body);
                }
                if self.include_buttons {
                    modal.buttons(ui, |ui| {
                        if modal.button(ui, "close").clicked() {
                            // all buttons created with the helper functions automatically
                            // close the modal on click, but you can close it yourself with
                            // ['modal.close(ctx)']
                            println!("hello world!")
                        }

                        modal.caution_button(ui, "button, but caution");
                        if modal
                            .suggested_button(ui, "open another modal")
                            .clicked()
                        {
                            // always close your previous modal before opening a new one otherwise weird
                            // layering things will happen. again, the helper functions for the buttons automatically
                            // close the modal on click, so we don't have to manually do that here
                            nested_modal.open();
                        }
                    })
                }
            });

            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                if ui.button("open modal").clicked() {
                    modal.open();
                }
                ui.separator();
                // to prevent locking the example window without any way to close the modal :)
                // remember to implement this yourself if you don't use buttons in your modal
                let mut cooc_enabled = self.close_on_outside_click || !self.include_buttons;
                ui.add_enabled_ui(self.include_buttons, |ui| {
                    if ui
                        .checkbox(&mut cooc_enabled, "close if click outside modal")
                        .clicked()
                    {
                        self.close_on_outside_click = !self.close_on_outside_click
                    };
                });
                ui.checkbox(&mut self.include_title, "include title");
                ui.checkbox(&mut self.include_body, "include body");
                ui.checkbox(&mut self.include_buttons, "include buttons");
                ui.separator();
                egui::Grid::new("options_grid")
                    .min_col_width(200.)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("title");
                        ui.text_edit_singleline(&mut self.modal_title);
                        ui.end_row();

                        ui.label("body");
                        ui.text_edit_singleline(&mut self.modal_body);
                        ui.end_row();

                        ui.label("body margin");
                        let body_margin = DragValue::new(&mut self.modal_style.body_margin)
                            .clamp_range(0..=20);
                        ui.add_sized(ui.available_rect_before_wrap().size(), body_margin);
                        ui.end_row();

                        ui.label("overlay color");
                        ui.color_edit_button_srgba(&mut self.modal_style.overlay_color);
                        ui.end_row();

                        ui.label("caution button fill");
                        ui.color_edit_button_srgba(&mut self.modal_style.caution_button_fill);
                        ui.end_row();

                        ui.label("caution button text");
                        ui.color_edit_button_srgba(
                            &mut self.modal_style.caution_button_text_color,
                        );
                        ui.end_row();

                        ui.label("suggested button fill");
                        ui.color_edit_button_srgba(
                            &mut self.modal_style.suggested_button_fill,
                        );
                        ui.end_row();

                        ui.label("suggested button text");
                        ui.color_edit_button_srgba(
                            &mut self.modal_style.suggested_button_text_color,
                        );
                        ui.end_row();
                    });
            });

            // why is this down here?? just wanted to show that you can put
            // the modal's [`.show()`] anywhere but we could have put this above
            // modal if we wanted
            nested_modal.show(|ui| {
                nested_modal.body(ui, "hello there!");
                nested_modal.buttons(ui, |ui| {
                    nested_modal.button(ui, "close");
                })
            });
        });
    }
}
fn main() {
    eframe::run_native(
        "egui-modal example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(ExampleApp::default())),
    )
}

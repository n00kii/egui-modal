use eframe::{
    self,
    egui::{
        style::Margin, Area, Button, Context, Frame, Id, InnerResponse, Label, LayerId, Layout, Order, Response, RichText, Sense, TextEdit, Ui,
        Window,
    },
    emath::{Align, Align2},
    epaint::{Color32, Pos2, Rounding},
};

/// The different styles a modal button can take.
pub enum ModalButtonStyle {
    /// A normal [`egui`] button
    None,
    /// A button highlighted blue
    Suggested,
    /// A button highlighted red
    Caution,
}

#[derive(Clone)]
/// Information about the current state of the modal. (Pretty empty
/// right now but may be expanded upon in the future.)
struct ModalState {
    is_open: bool,
}

#[derive(Clone)]
/// Contains styling parameters for the modal, like body margin 
/// and button colors.
pub struct ModalStyle {
    /// The margin around the modal body. Only applies if using 
    /// [`.body()`]
    pub body_margin: f32,
    /// The color of the overlay that dims the background
    pub overlay_color: Color32,
    
    /// The fill color for the caution button style
    pub caution_button_fill: Color32,
    /// The fill color for the suggested button style
    pub suggested_button_fill: Color32,

    /// The text color for the caution button style
    pub caution_button_text_color: Color32,
    /// The text color for the suggested button style
    pub suggested_button_text_color: Color32,
}

impl ModalState {
    fn load(ctx: &Context, id: Id) -> Self {
        ctx.data().get_persisted(id).unwrap_or_default()
    }
    fn save(self, ctx: &Context, id: Id) {
        ctx.data().insert_persisted(id, self)
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self { is_open: false }
    }
}

impl Default for ModalStyle {
    fn default() -> Self {
        Self {
            body_margin: 5.,
            overlay_color: Color32::from_rgba_unmultiplied(0, 0, 0, 200),

            caution_button_fill: Color32::from_rgb(87, 38, 34),
            suggested_button_fill: Color32::from_rgb(33, 54, 84),

            caution_button_text_color: Color32::from_rgb(242, 148, 148),
            suggested_button_text_color: Color32::from_rgb(141, 182, 242),
        }
    }
}
/// A [`Modal`] is created using [`Modal::new()`]. Make sure to use a `let` binding when 
/// using [`Modal::new()`] to ensure you can call things like [`Modal::open()`] later on.
/// ```
/// let modal = Modal::new("my_modal");
/// modal.show(ctx, |ui| {
///     ui.label("Hello world!")
/// });
/// if ui.button("modal").clicked() {
///     modal.open(ctx);
/// }
/// ```
pub struct Modal {
    close_on_outside_click: bool,
    style: ModalStyle,
    id: Id,
    window_id: Id,
}

fn ui_with_margin<R>(ui: &mut Ui, margin: f32, add_contents: impl FnOnce(&mut Ui) -> R) {
    ui.vertical(|ui| {
        ui.add_space(margin);
        ui.horizontal(|ui| {
            ui.add_space(margin);
            add_contents(ui);
            ui.add_space(margin);
        });
        ui.add_space(margin);
    });
}

impl Modal {
    /// Creates a new [`Modal`]. Can use constructor functions like [`Modal::with_style`]
    /// to modify upon creation.
    pub fn new(id_source: impl std::fmt::Display) -> Self {
        Self {
            id: Id::new(id_source.to_string()),
            style: ModalStyle::default(),
            close_on_outside_click: false,
            window_id: Id::new("window_".to_string() + &id_source.to_string()),
        }
    }

    fn set_open_state(&self, ctx: &Context, is_open: bool) {
        let mut modal_state = ModalState::load(ctx, self.id);
        modal_state.is_open = is_open;
        modal_state.save(ctx, self.id)
    }

    /// Open the modal; make it visible. The modal prevents user input to other parts of the
    /// application.
    pub fn open(&self, ctx: &Context) {
        self.set_open_state(ctx, true)
    }

    /// Close the modal so that it is no longer visible, allowing input to flow back into
    /// the application.
    pub fn close(&self, ctx: &Context) {
        self.set_open_state(ctx, false)
    }

    /// If set to `true`, the modal will close itself if the user clicks outside on the modal window
    /// (onto the overlay).
    pub fn with_close_on_outside_click(mut self, do_close_on_click_ouside: bool) -> Self {
        self.close_on_outside_click = do_close_on_click_ouside;
        self
    }
    
    /// Change the [`ModalStyle`] of the modal upon creation.
    pub fn with_style(mut self, style: &ModalStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Helper function for styling the title of the modal.
    pub fn title(&self, ui: &mut Ui, text: impl Into<RichText>) {
        let text: RichText = text.into();
        ui.vertical_centered(|ui| {
            ui.heading(text);
        });
        ui.separator();
    }

    /// Helper function for styling the body of the modal.
    pub fn body(&self, ui: &mut Ui, text: impl Into<RichText>) {
        let text: RichText = text.into();
        ui_with_margin(ui, self.style.body_margin, |ui| {
            ui.label(text);
        })
    }

    /// Helper function for styling the button container of the modal.
    pub fn buttons<R>(&self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) {
        ui.separator();
        ui.with_layout(Layout::right_to_left(Align::Min), add_contents);
    }

    /// Helper function for creating a normal button for the modal. 
    /// Automatically closes the modal on click.
    pub fn button(&self, ui: &mut Ui, text: impl Into<RichText>) -> Response {
        self.styled_button(ui, text, ModalButtonStyle::None)
    }

    /// Helper function for creating a "cautioned" button for the modal.
    /// Automatically closes the modal on click.
    pub fn caution_button(&self, ui: &mut Ui, text: impl Into<RichText>) -> Response {
        self.styled_button(ui, text, ModalButtonStyle::Caution)
    }

    /// Helper function for creating a "suggested" button for the modal.
    /// Automatically closes the modal on click.
    pub fn suggested_button(&self, ui: &mut Ui, text: impl Into<RichText>) -> Response {
        self.styled_button(ui, text, ModalButtonStyle::Suggested)
    }
    
    fn styled_button(&self, ui: &mut Ui, text: impl Into<RichText>, button_style: ModalButtonStyle) -> Response {
        let button = match button_style {
            ModalButtonStyle::Suggested => {
                let text: RichText = text.into().color(self.style.suggested_button_text_color);
                Button::new(text).fill(self.style.suggested_button_fill)
            }
            ModalButtonStyle::Caution => {
                let text: RichText = text.into().color(self.style.caution_button_text_color);
                Button::new(text).fill(self.style.caution_button_fill)
            }
            ModalButtonStyle::None => Button::new(text.into()),
        };

        let response = ui.add(button);
        if response.clicked() {
            self.close(ui.ctx())
        }
        response
    }

    /// The ui contained in this function will be shown within the modal window. The modal will only actually show 
    /// when [`Modal::open`] is used. 
    pub fn show<R>(&self, ctx: &Context, add_contents: impl FnOnce(&mut Ui) -> R) {
        let mut modal_state = ModalState::load(ctx, self.id);
        if modal_state.is_open {
            let ctx_clone = ctx.clone();
            Area::new(self.id).interactable(true).fixed_pos(Pos2::ZERO).show(ctx, |ui: &mut Ui| {
                let screen_rect = ui.ctx().input().screen_rect;
                let area_response = ui.allocate_response(screen_rect.size(), Sense::click());
                if area_response.clicked() && self.close_on_outside_click {
                    self.close(ctx);
                }
                ui.painter().rect_filled(screen_rect, Rounding::none(), self.style.overlay_color);
            });
            let window = Window::new("")
                .id(self.window_id)
                .open(&mut modal_state.is_open)
                .title_bar(false)
                .anchor(Align2::CENTER_CENTER, [0., 0.])
                .resizable(false);

            let response = window.show(&ctx_clone, add_contents);
            if let Some(inner_response) = response {
                inner_response.response.request_focus();
                ctx_clone.move_to_top(inner_response.response.layer_id);
            }
        }
        // frame.
    }
}

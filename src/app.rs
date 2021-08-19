use crate::expression::Expression;
use crate::types::{MathError, Token};
use eframe::egui::epaint::{color, Shadow};
use eframe::egui::{Align2, Frame, Rgba, Window};
use eframe::{egui, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct CalcApp {
    input: String,
    prev_input: String,
    expression: Expression,
    result: Result<Token, MathError>,
}

impl Default for CalcApp {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            prev_input: "".to_string(),
            expression: Expression {
                infix_token: vec![],
            },
            result: Err(MathError::None),
        }
    }
}

impl epi::App for CalcApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if (ctx.style().visuals.window_shadow
            == Shadow {
                extrusion: 0.0,
                color: color::Color32::TRANSPARENT,
            })
        {
            println!("Test");
        }
        Window::new("Calculator")
            .anchor(Align2::CENTER_CENTER, egui::Vec2::default())
            .frame(Frame::window(&ctx.style()))
            .show(ctx, |ui| {
                ui.text_edit_singleline(&mut self.input);
                if self.input != self.prev_input {
                    if self.input == "!" {
                        frame.quit();
                    }
                    self.prev_input = self.input.clone();
                    self.expression = Expression::default();
                    self.expression.tokenise(&self.input);
                    self.result = self.expression.calculate();
                }
                ui.label(format!("With tokens: {:?}", self.expression.infix_token));
                ui.label(match &self.result {
                    Err(e) => format!("Got Error: {}", e),
                    Ok(t) => format!("Got Result: {:?}", t),
                });
            });

        // frame.set_window_size(ctx.used_size());
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.window_shadow = Shadow {
            extrusion: 0.0,
            color: color::Color32::TRANSPARENT,
        };
        ctx.set_style(style);
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "egui template"
    }

    fn clear_color(&self) -> Rgba {
        Rgba::TRANSPARENT
    }
}

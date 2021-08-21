use crate::expression::Expression;
use crate::types::{MathError, Token};
use eframe::egui::epaint::{color, Shadow};
use eframe::egui::{Align2, Frame, Window};
use eframe::{egui, epi};

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
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
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

        frame.set_window_size(ctx.used_size());
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.window_shadow = Shadow {
            extrusion: 0.0,
            color: color::Color32::TRANSPARENT,
        };
        ctx.set_style(style);
    }

    fn name(&self) -> &str {
        "Calculator"
    }
}

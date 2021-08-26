use calculator::expression::{eval, Expression, Parser, Rule};
use calculator::types::{MathError, Token};
use eframe::egui::epaint::{color, Shadow};
use eframe::egui::{Align2, Frame, Window};
use eframe::{egui, epi};

pub struct CalcApp {
    input: String,
    prev_input: String,
    result: Result<Token, MathError>,
}

impl Default for CalcApp {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            prev_input: "".to_string(),
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
                    self.prev_input = self.input.clone();
                    if self.input == "!" {
                        frame.quit();
                    } else if self.input.is_empty() {
                        self.result = Err(MathError::None);
                    } else {
                        self.result = match Expression::parse(Rule::calculation, &self.input) {
                            Ok(calc) => eval(calc),
                            Err(_) => Err(MathError::SyntaxError),
                        }
                    }
                }
                ui.label(match &self.result {
                    Err(MathError::None) => "Awaiting input...".to_string(),
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

#[cfg(feature = "gui")]
mod app;
#[cfg(feature = "gui")]
use crate::app::CalcApp;
#[cfg(not(feature = "gui"))]
use calculator::expression::{eval, Expression, Parser, Rule};
#[cfg(not(feature = "gui"))]
use calculator::types::MathError;
#[cfg(feature = "gui")]
use eframe::NativeOptions;
#[cfg(not(feature = "gui"))]
use std::io;

#[cfg(feature = "gui")]
fn main() {
    let app = CalcApp::default();
    let native_options = NativeOptions {
        always_on_top: false,
        decorated: true,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: None,
        resizable: true,
        transparent: false,
    };
    eframe::run_native(Box::new(app), native_options);
}
#[cfg(not(feature = "gui"))]
fn main() {
    loop {
        let mut string_expression = String::new();
        io::stdin()
            .read_line(&mut string_expression)
            .expect("Failed to read line");
        let str_expression = string_expression.trim_end_matches(is_newline);
        if str_expression == "stop" || str_expression == "Stop" {
            break;
        }
        match match Expression::parse(Rule::calculation, &str_expression) {
            Ok(calc) => eval(calc),
            Err(_) => Err(MathError::SyntaxError),
        } {
            Err(e) => println!("Got Error: {}", e),
            Ok(t) => println!("Got Result: {:?}", t),
        };
    }
}

#[cfg(not(feature = "gui"))]
fn is_newline(c: char) -> bool {
    c == '\n' || c == '\r'
}

//TODO Change fraction ops to use gcd before adding everything together. + Check why (5^1/3)+2^1/3 is not correct. + Do Power division + add div and mul methods between roots and Frac/Int.

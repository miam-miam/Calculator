use calculator::app::CalcApp;

use eframe::NativeOptions;

fn main() {
    let app = CalcApp::default();
    let native_options = NativeOptions {
        always_on_top: false,
        decorated: false,
        drag_and_drop_support: false,
        icon_data: None,
        initial_window_size: None,
        resizable: false,
        transparent: true,
    };
    eframe::run_native(Box::new(app), native_options);
}

//TODO Change fraction ops to use gcd before adding everything together. + Check why (5^1/3)+2^1/3 is not correct. + Do Power division

use std::cell::RefCell;
use std::rc::Rc;

use crate::MainWindow;
use crate::calculator::{Calculator, CalculatorSnapshot};
use slint::ComponentHandle;

pub fn run(ui: MainWindow) -> Result<(), slint::PlatformError> {
    let calculator = Rc::new(RefCell::new(Calculator::default()));
    update_ui(&ui, calculator.borrow().snapshot());

    let ui_handle = ui.as_weak();
    ui.on_button_pressed({
        let calculator = Rc::clone(&calculator);

        move |label| {
            let snapshot = calculator.borrow_mut().press(label.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                update_ui(&ui, snapshot);
            }
        }
    });

    ui.run()
}

fn update_ui(ui: &MainWindow, snapshot: CalculatorSnapshot) {
    ui.set_display_text(snapshot.display.into());
    ui.set_equation_text(snapshot.equation.into());
    ui.set_active_operator(snapshot.active_operator.into());
    ui.set_clear_label(snapshot.clear_label.into());
    ui.set_display_font_size(snapshot.display_font_size);
}

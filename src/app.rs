use std::cell::RefCell;
use std::rc::Rc;

use crate::MainWindow;
use crate::calculator::Calculator;
use slint::ComponentHandle;

pub fn run(ui: MainWindow) -> Result<(), slint::PlatformError> {
    let calculator = Rc::new(RefCell::new(Calculator::default()));
    ui.set_display_text(calculator.borrow().display().into());

    let ui_handle = ui.as_weak();
    ui.on_button_pressed({
        let calculator = Rc::clone(&calculator);

        move |label| {
            let display = calculator.borrow_mut().press(label.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_display_text(display.into());
            }
        }
    });

    ui.run()
}

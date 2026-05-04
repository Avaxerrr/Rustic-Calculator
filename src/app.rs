use std::cell::RefCell;
use std::rc::Rc;

use crate::MainWindow;
use crate::calculator::{Calculator, CalculatorSnapshot};
use crate::converter_app::{ConverterApp, ConverterSnapshot};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

pub fn run(ui: MainWindow) -> Result<(), slint::PlatformError> {
    let calculator = Rc::new(RefCell::new(Calculator::default()));
    let converter = Rc::new(RefCell::new(ConverterApp::default()));

    update_calculator_ui(&ui, calculator.borrow().snapshot());
    update_converter_ui(&ui, converter.borrow().snapshot());

    let ui_handle = ui.as_weak();
    ui.on_button_pressed({
        let calculator = Rc::clone(&calculator);

        move |label| {
            let snapshot = calculator.borrow_mut().press(label.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                update_calculator_ui(&ui, snapshot);
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_converter_button_pressed({
        let converter = Rc::clone(&converter);

        move |label| {
            let snapshot = converter.borrow_mut().press(label.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                update_converter_ui(&ui, snapshot);
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_category_selected({
        let converter = Rc::clone(&converter);

        move |category| {
            let snapshot = converter.borrow_mut().select_category(category.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_active_view("converter".into());
                ui.set_sidebar_open(false);
                update_converter_ui(&ui, snapshot);
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_standard_selected(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_active_view("calculator".into());
            ui.set_sidebar_open(false);
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_from_unit_selected({
        let converter = Rc::clone(&converter);

        move |unit| {
            let snapshot = converter.borrow_mut().select_from_unit(unit.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                update_converter_ui(&ui, snapshot);
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_to_unit_selected({
        let converter = Rc::clone(&converter);

        move |unit| {
            let snapshot = converter.borrow_mut().select_to_unit(unit.as_str());

            if let Some(ui) = ui_handle.upgrade() {
                update_converter_ui(&ui, snapshot);
            }
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_swap_units_pressed({
        let converter = Rc::clone(&converter);

        move || {
            let snapshot = converter.borrow_mut().swap_units();

            if let Some(ui) = ui_handle.upgrade() {
                update_converter_ui(&ui, snapshot);
            }
        }
    });

    ui.run()
}

fn update_calculator_ui(ui: &MainWindow, snapshot: CalculatorSnapshot) {
    ui.set_display_text(snapshot.display.into());
    ui.set_equation_text(snapshot.equation.into());
    ui.set_active_operator(snapshot.active_operator.into());
    ui.set_clear_label(snapshot.clear_label.into());
    ui.set_display_font_size(snapshot.display_font_size);
}

fn update_converter_ui(ui: &MainWindow, snapshot: ConverterSnapshot) {
    ui.set_category_title(snapshot.category_title.into());
    ui.set_input_value(snapshot.input_value.into());
    ui.set_output_value(snapshot.output_value.into());
    ui.set_from_unit(snapshot.from_unit.into());
    ui.set_to_unit(snapshot.to_unit.into());
    ui.set_from_unit_index(snapshot.from_unit_index);
    ui.set_to_unit_index(snapshot.to_unit_index);
    ui.set_unit_options(to_model(snapshot.unit_options));
    ui.set_relation_text(snapshot.relation.into());
}

fn to_model(values: Vec<String>) -> ModelRc<SharedString> {
    let values = values
        .into_iter()
        .map(SharedString::from)
        .collect::<Vec<_>>();
    ModelRc::new(VecModel::from(values))
}

use crate::features::converters::{self, ConverterCategory};
use crate::number_input::{PasteNumberOptions, parse_pasted_number};

#[derive(Clone, Copy)]
struct ConverterSelection {
    category: ConverterCategory,
    from_index: usize,
    to_index: usize,
}

pub struct ConverterSnapshot {
    pub category_title: String,
    pub input_value: String,
    pub output_value: String,
    pub from_unit: String,
    pub to_unit: String,
    pub from_unit_index: i32,
    pub to_unit_index: i32,
    pub unit_options: Vec<String>,
    pub relation: String,
}

pub struct ConverterApp {
    selection: ConverterSelection,
    input: String,
}

impl Default for ConverterApp {
    fn default() -> Self {
        let category = ConverterCategory::Length;
        let (from_index, to_index) = default_unit_indices(category);

        Self {
            selection: ConverterSelection {
                category,
                from_index,
                to_index,
            },
            input: "1".to_owned(),
        }
    }
}

impl ConverterApp {
    pub fn snapshot(&self) -> ConverterSnapshot {
        let category = self.selection.category;
        let value = parsed_value(&self.input);
        let unit_options = unit_names(category);
        let from_unit = unit_options
            .get(self.selection.from_index)
            .cloned()
            .unwrap_or_default();
        let to_unit = unit_options
            .get(self.selection.to_index)
            .cloned()
            .unwrap_or_default();
        let output = convert_value(category, value, &from_unit, &to_unit);
        let relation = format!(
            "1 {} = {} {}",
            from_unit,
            format_value(convert_value(category, 1.0, &from_unit, &to_unit)),
            to_unit
        );

        ConverterSnapshot {
            category_title: category.title().to_owned(),
            input_value: self.input.clone(),
            output_value: format_value(output),
            from_unit,
            to_unit,
            from_unit_index: self.selection.from_index as i32,
            to_unit_index: self.selection.to_index as i32,
            unit_options,
            relation,
        }
    }

    pub fn select_category(&mut self, title: &str) -> ConverterSnapshot {
        if let Some(category) = category_from_title(title) {
            let (from_index, to_index) = default_unit_indices(category);
            self.selection = ConverterSelection {
                category,
                from_index,
                to_index,
            };
            self.input = default_input(category).to_owned();
        }

        self.snapshot()
    }

    pub fn press(&mut self, label: &str) -> ConverterSnapshot {
        self.input = update_input(&self.input, label);
        self.snapshot()
    }

    pub fn paste_number(&mut self, text: &str) -> ConverterSnapshot {
        let Some(value) = parse_pasted_number(
            text,
            PasteNumberOptions {
                allow_negative: self.selection.category == ConverterCategory::Temperature,
                max_digits: 16,
            },
        ) else {
            return self.snapshot();
        };

        self.input = value;
        self.snapshot()
    }

    pub fn select_from_unit(&mut self, unit_name: &str) -> ConverterSnapshot {
        if let Some(index) = unit_index(self.selection.category, unit_name) {
            self.selection.from_index = index;
        }
        self.snapshot()
    }

    pub fn select_to_unit(&mut self, unit_name: &str) -> ConverterSnapshot {
        if let Some(index) = unit_index(self.selection.category, unit_name) {
            self.selection.to_index = index;
        }
        self.snapshot()
    }

    pub fn swap_units(&mut self) -> ConverterSnapshot {
        let snapshot = self.snapshot();
        self.input = snapshot.output_value.replace(',', "");
        std::mem::swap(&mut self.selection.from_index, &mut self.selection.to_index);
        self.snapshot()
    }
}

fn category_from_title(title: &str) -> Option<ConverterCategory> {
    converters::CATEGORIES
        .iter()
        .copied()
        .find(|category| category.title().eq_ignore_ascii_case(title))
}

fn units(category: ConverterCategory) -> Vec<UnitLabel> {
    match category {
        ConverterCategory::Temperature => converters::TEMPERATURE_UNITS
            .iter()
            .map(|unit| UnitLabel::new(unit))
            .collect(),
        _ => converters::units_for(category)
            .unwrap_or(&[])
            .iter()
            .map(|unit| UnitLabel::new(unit.name))
            .collect(),
    }
}

#[derive(Clone)]
struct UnitLabel {
    name: String,
}

impl UnitLabel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

fn unit_names(category: ConverterCategory) -> Vec<String> {
    units(category).into_iter().map(|unit| unit.name).collect()
}

fn unit_index(category: ConverterCategory, unit_name: &str) -> Option<usize> {
    units(category)
        .into_iter()
        .position(|unit| unit.name.eq_ignore_ascii_case(unit_name))
}

fn default_unit_indices(category: ConverterCategory) -> (usize, usize) {
    match category {
        ConverterCategory::Volume => (0, 4),
        ConverterCategory::Length => (0, 8),
        ConverterCategory::WeightMass => (0, 4),
        ConverterCategory::Temperature => (0, 1),
        ConverterCategory::Energy => (1, 2),
        ConverterCategory::Area => (0, 6),
        ConverterCategory::Speed => (2, 1),
        ConverterCategory::Time => (0, 3),
        ConverterCategory::Power => (0, 1),
        ConverterCategory::Data => (7, 5),
        ConverterCategory::Pressure => (0, 1),
        ConverterCategory::Angle => (1, 0),
    }
}

fn default_input(category: ConverterCategory) -> &'static str {
    match category {
        ConverterCategory::Temperature => "24",
        ConverterCategory::Speed => "55",
        ConverterCategory::Angle => "90",
        ConverterCategory::Data => "8",
        ConverterCategory::WeightMass => "12",
        ConverterCategory::Volume => "3",
        ConverterCategory::Energy => "20",
        _ => "1",
    }
}

fn convert_value(category: ConverterCategory, value: f64, from: &str, to: &str) -> f64 {
    match category {
        ConverterCategory::Temperature => {
            converters::convert_temperature(value, from, to).unwrap_or(0.0)
        }
        _ => converters::convert(category, value, from, to).unwrap_or(0.0),
    }
}

fn update_input(current: &str, label: &str) -> String {
    match label {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            if current == "0" {
                label.to_owned()
            } else if digit_count(current) < 16 {
                format!("{current}{label}")
            } else {
                current.to_owned()
            }
        }
        "." if !current.contains('.') => format!("{current}."),
        "+/-" if current.starts_with('-') => current.trim_start_matches('-').to_owned(),
        "+/-" if current != "0" => format!("-{current}"),
        "Back" => {
            let mut next = current.to_owned();
            next.pop();
            if next.is_empty() || next == "-" {
                "0".to_owned()
            } else {
                next
            }
        }
        "C" | "CE" => "0".to_owned(),
        _ => current.to_owned(),
    }
}

fn parsed_value(input: &str) -> f64 {
    input.parse().unwrap_or(0.0)
}

fn digit_count(input: &str) -> usize {
    input.chars().filter(char::is_ascii_digit).count()
}

fn format_value(value: f64) -> String {
    if !value.is_finite() {
        return "Error".to_owned();
    }

    let mut text = if value.abs() >= 1_000_000_000.0 || (value != 0.0 && value.abs() < 0.000_001) {
        format!("{value:.8e}")
    } else if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.8}")
    };

    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }

    if text.ends_with('.') {
        text.pop();
    }

    add_group_separators(&text)
}

fn add_group_separators(value: &str) -> String {
    if value.contains('e') {
        return value.to_owned();
    }

    let (whole, decimal) = value.split_once('.').map_or((value, ""), |parts| parts);
    let (sign, digits) = whole
        .strip_prefix('-')
        .map_or(("", whole), |digits| ("-", digits));
    let mut grouped = String::new();

    for (index, digit) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }

    let whole: String = grouped.chars().rev().collect();
    if decimal.is_empty() {
        format!("{sign}{whole}")
    } else {
        format!("{sign}{whole}.{decimal}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_length_conversion() {
        let app = ConverterApp::default();
        let snapshot = app.snapshot();

        assert_eq!(snapshot.category_title, "Length");
        assert_eq!(snapshot.from_unit, "Meter");
        assert_eq!(snapshot.to_unit, "Foot");
        assert_eq!(snapshot.output_value, "3.2808399");
    }

    #[test]
    fn updates_input_from_keypad() {
        let mut app = ConverterApp::default();
        app.press("2");
        app.press(".");
        let snapshot = app.press("5");

        assert_eq!(snapshot.input_value, "12.5");
    }

    #[test]
    fn can_select_category_and_pick_units() {
        let mut app = ConverterApp::default();
        let snapshot = app.select_category("Temperature");

        assert_eq!(snapshot.from_unit, "Celsius");
        assert_eq!(snapshot.to_unit, "Fahrenheit");

        let snapshot = app.select_to_unit("Kelvin");
        assert_eq!(snapshot.to_unit, "Kelvin");
    }

    #[test]
    fn pastes_values_into_converter() {
        let mut app = ConverterApp::default();

        let snapshot = app.paste_number(" 12,345.5 ");

        assert_eq!(snapshot.input_value, "12345.5");
    }

    #[test]
    fn only_temperature_accepts_negative_paste() {
        let mut app = ConverterApp::default();
        assert_eq!(app.paste_number("-12").input_value, "1");

        app.select_category("Temperature");
        assert_eq!(app.paste_number("-12").input_value, "-12");
    }
}

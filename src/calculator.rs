#[derive(Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn label(self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "x",
            Operator::Divide => "/",
        }
    }
}

pub struct CalculatorSnapshot {
    pub display: String,
    pub equation: String,
    pub active_operator: String,
    pub clear_label: String,
}

#[derive(Default)]
pub struct Calculator {
    display: String,
    equation: String,
    accumulator: Option<f64>,
    pending_operator: Option<Operator>,
    reset_display_on_next_digit: bool,
    has_error: bool,
}

impl Calculator {
    pub fn display(&self) -> &str {
        if self.display.is_empty() {
            "0"
        } else {
            &self.display
        }
    }

    pub fn snapshot(&self) -> CalculatorSnapshot {
        CalculatorSnapshot {
            display: format_display(self.display()),
            equation: self.equation.clone(),
            active_operator: self
                .pending_operator
                .filter(|_| self.reset_display_on_next_digit)
                .map(|operator| operator.label().to_owned())
                .unwrap_or_default(),
            clear_label: if self.display() == "0"
                && self.accumulator.is_none()
                && self.pending_operator.is_none()
                && !self.has_error
            {
                "AC".to_owned()
            } else {
                "C".to_owned()
            },
        }
    }

    pub fn press(&mut self, label: &str) -> CalculatorSnapshot {
        match label {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.input_digit(label),
            "." => self.input_decimal_point(),
            "+" => self.input_operator(Operator::Add),
            "-" => self.input_operator(Operator::Subtract),
            "x" => self.input_operator(Operator::Multiply),
            "/" => self.input_operator(Operator::Divide),
            "=" => self.evaluate_pending(),
            "+/-" => self.toggle_sign(),
            "%" => self.percent(),
            "Back" => self.backspace(),
            "C" | "AC" | "CE" => self.clear(),
            _ => {}
        }

        self.snapshot()
    }

    fn clear(&mut self) {
        self.display.clear();
        self.equation.clear();
        self.accumulator = None;
        self.pending_operator = None;
        self.reset_display_on_next_digit = false;
        self.has_error = false;
    }

    fn input_digit(&mut self, digit: &str) {
        if self.has_error || self.reset_display_on_next_digit {
            self.display.clear();
            self.has_error = false;
            self.reset_display_on_next_digit = false;
        }

        if self.display == "0" {
            self.display.clear();
        }

        self.display.push_str(digit);
    }

    fn input_decimal_point(&mut self) {
        if self.has_error || self.reset_display_on_next_digit {
            self.display = "0".to_owned();
            self.has_error = false;
            self.reset_display_on_next_digit = false;
        }

        if !self.display.contains('.') {
            if self.display.is_empty() {
                self.display.push('0');
            }

            self.display.push('.');
        }
    }

    fn input_operator(&mut self, operator: Operator) {
        if self.has_error {
            return;
        }

        if self.pending_operator.is_some() && !self.reset_display_on_next_digit {
            self.evaluate_pending();
        } else {
            self.accumulator = Some(self.current_value());
        }

        self.pending_operator = Some(operator);
        self.equation = format!("{} {}", self.display(), operator.label());
        self.reset_display_on_next_digit = true;
    }

    fn evaluate_pending(&mut self) {
        if self.has_error {
            return;
        }

        let Some(operator) = self.pending_operator.take() else {
            self.reset_display_on_next_digit = true;
            return;
        };

        let left = self.accumulator.unwrap_or_else(|| self.current_value());
        let right = self.current_value();
        self.equation = format!(
            "{} {} {} =",
            format_number(left),
            operator.label(),
            format_number(right)
        );

        let Some(result) = apply_operator(left, right, operator) else {
            self.display = "Error".to_owned();
            self.accumulator = None;
            self.has_error = true;
            self.reset_display_on_next_digit = true;
            return;
        };

        self.display = format_number(result);
        self.accumulator = Some(result);
        self.reset_display_on_next_digit = true;
    }

    fn toggle_sign(&mut self) {
        if self.has_error {
            return;
        }

        if self.display().starts_with('-') {
            self.display.remove(0);
        } else if self.display() != "0" {
            self.display.insert(0, '-');
        }
    }

    fn percent(&mut self) {
        if self.has_error {
            return;
        }

        self.display = format_number(self.current_value() / 100.0);
    }

    fn backspace(&mut self) {
        if self.has_error || self.reset_display_on_next_digit {
            self.display.clear();
            self.has_error = false;
            self.reset_display_on_next_digit = false;
            return;
        }

        self.display.pop();

        if self.display.is_empty() || self.display == "-" {
            self.display = "0".to_owned();
        }
    }

    fn current_value(&self) -> f64 {
        self.display().parse().unwrap_or(0.0)
    }
}

fn apply_operator(left: f64, right: f64, operator: Operator) -> Option<f64> {
    match operator {
        Operator::Add => Some(left + right),
        Operator::Subtract => Some(left - right),
        Operator::Multiply => Some(left * right),
        Operator::Divide => {
            if right == 0.0 {
                None
            } else {
                Some(left / right)
            }
        }
    }
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "Error".to_owned();
    }

    if value.fract().abs() < f64::EPSILON {
        return (value as i64).to_string();
    }

    let mut text = format!("{value:.10}");

    while text.contains('.') && text.ends_with('0') {
        text.pop();
    }

    if text.ends_with('.') {
        text.pop();
    }

    if text == "-0" { "0".to_owned() } else { text }
}

fn format_display(value: &str) -> String {
    if value == "Error" {
        return value.to_owned();
    }

    let Some((whole, decimal)) = value.split_once('.') else {
        return add_group_separators(value);
    };

    format!("{}.{}", add_group_separators(whole), decimal)
}

fn add_group_separators(value: &str) -> String {
    let (sign, digits) = value
        .strip_prefix('-')
        .map_or(("", value), |digits| ("-", digits));
    let mut grouped = String::new();

    for (index, digit) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            grouped.push(',');
        }

        grouped.push(digit);
    }

    let grouped: String = grouped.chars().rev().collect();
    format!("{sign}{grouped}")
}

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn adds_two_numbers() {
        let mut calculator = Calculator::default();

        calculator.press("2");
        calculator.press("+");
        calculator.press("3");

        assert_eq!(calculator.press("=").display, "5");
    }

    #[test]
    fn clears_after_error() {
        let mut calculator = Calculator::default();

        calculator.press("8");
        calculator.press("/");
        calculator.press("0");
        assert_eq!(calculator.press("=").display, "Error");

        assert_eq!(calculator.press("1").display, "1");
    }

    #[test]
    fn tracks_active_operator() {
        let mut calculator = Calculator::default();

        calculator.press("9");
        let snapshot = calculator.press("x");

        assert_eq!(snapshot.equation, "9 x");
        assert_eq!(snapshot.active_operator, "x");
    }

    #[test]
    fn formats_grouped_display() {
        let mut calculator = Calculator::default();

        for digit in ["1", "2", "3", "4", "5", "6"] {
            calculator.press(digit);
        }

        assert_eq!(calculator.snapshot().display, "123,456");
    }
}

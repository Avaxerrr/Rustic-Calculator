#[derive(Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Default)]
pub struct Calculator {
    display: String,
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

    pub fn press(&mut self, label: &str) -> String {
        match label {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.input_digit(label),
            "." => self.input_decimal_point(),
            "+" => self.input_operator(Operator::Add),
            "-" => self.input_operator(Operator::Subtract),
            "x" => self.input_operator(Operator::Multiply),
            "/" => self.input_operator(Operator::Divide),
            "=" => self.evaluate_pending(),
            "+/-" => self.toggle_sign(),
            "Back" => self.backspace(),
            "C" => self.clear(),
            _ => {}
        }

        self.display().to_owned()
    }

    fn clear(&mut self) {
        self.display.clear();
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

#[cfg(test)]
mod tests {
    use super::Calculator;

    #[test]
    fn adds_two_numbers() {
        let mut calculator = Calculator::default();

        calculator.press("2");
        calculator.press("+");
        calculator.press("3");

        assert_eq!(calculator.press("="), "5");
    }

    #[test]
    fn clears_after_error() {
        let mut calculator = Calculator::default();

        calculator.press("8");
        calculator.press("/");
        calculator.press("0");
        assert_eq!(calculator.press("="), "Error");

        assert_eq!(calculator.press("1"), "1");
    }
}

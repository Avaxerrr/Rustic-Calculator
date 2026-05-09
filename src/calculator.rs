use crate::number_input::{PasteNumberOptions, parse_pasted_number};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HistoryEntry {
    pub equation: String,
    pub result: String,
}

pub struct CalculatorSnapshot {
    pub display: String,
    pub equation: String,
    pub active_operator: String,
    pub clear_label: String,
    pub display_font_size: i32,
    pub history: Vec<HistoryEntry>,
}

#[derive(Default)]
pub struct Calculator {
    display: String,
    equation: String,
    accumulator: Option<f64>,
    pending_operator: Option<Operator>,
    pending_pasted_expression: Option<String>,
    reset_display_on_next_digit: bool,
    has_error: bool,
    history: Vec<HistoryEntry>,
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
        let display = format_display(self.display());

        CalculatorSnapshot {
            display_font_size: display_font_size(&display),
            display,
            equation: self.equation.clone(),
            active_operator: self
                .pending_operator
                .filter(|_| self.reset_display_on_next_digit)
                .map(|operator| operator.label().to_owned())
                .unwrap_or_default(),
            history: self.history.clone(),
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
        if label != "=" {
            self.pending_pasted_expression = None;
        }

        match label {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.input_digit(label),
            "." => self.input_decimal_point(),
            "+" => self.input_operator(Operator::Add),
            "-" => self.input_operator(Operator::Subtract),
            "x" => self.input_operator(Operator::Multiply),
            "/" => self.input_operator(Operator::Divide),
            "=" => self.evaluate_pending(true),
            "+/-" => self.toggle_sign(),
            "%" => self.percent(),
            "Back" => self.backspace(),
            "C" | "AC" | "CE" => self.clear(),
            _ => {}
        }

        self.snapshot()
    }

    pub fn clear_history(&mut self) -> CalculatorSnapshot {
        self.history.clear();
        self.snapshot()
    }

    pub fn paste_input(&mut self, text: &str) -> CalculatorSnapshot {
        if let Some(value) = parse_pasted_number(text, pasted_number_options()) {
            self.apply_pasted_number(value);
            return self.snapshot();
        }

        let Some(expression) = parse_pasted_expression(text) else {
            return self.snapshot();
        };

        self.clear();
        self.pending_pasted_expression = Some(expression.equation.clone());
        for token in expression.tokens {
            self.apply_paste_token(token);
            if self.has_error {
                break;
            }
        }
        if !self.has_error && self.pending_pasted_expression.is_some() {
            self.equation = expression.equation;
        }
        self.snapshot()
    }

    fn apply_paste_token(&mut self, token: PasteToken) {
        match token {
            PasteToken::Number(value) => self.apply_pasted_number(value),
            PasteToken::Operator(operator) => self.input_operator(operator),
            PasteToken::Percent => self.percent(),
            PasteToken::Equals => self.evaluate_pending(true),
        }
    }

    fn apply_pasted_number(&mut self, value: String) {
        self.display = value;
        self.has_error = false;
        self.reset_display_on_next_digit = false;
    }

    fn clear(&mut self) {
        self.display.clear();
        self.equation.clear();
        self.accumulator = None;
        self.pending_operator = None;
        self.pending_pasted_expression = None;
        self.reset_display_on_next_digit = false;
        self.has_error = false;
    }

    fn input_digit(&mut self, digit: &str) {
        if self.has_error || self.reset_display_on_next_digit {
            self.display.clear();
            self.has_error = false;
            self.reset_display_on_next_digit = false;
        }

        if significant_digit_count(&self.display) >= 18 {
            return;
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
            self.evaluate_pending(false);
        } else {
            self.accumulator = Some(self.current_value());
        }

        self.pending_operator = Some(operator);
        self.equation = format!("{} {}", format_display(self.display()), operator.label());
        self.reset_display_on_next_digit = true;
    }

    fn evaluate_pending(&mut self, record_history: bool) {
        if self.has_error {
            return;
        }

        let Some(operator) = self.pending_operator.take() else {
            self.reset_display_on_next_digit = true;
            return;
        };

        let left = self.accumulator.unwrap_or_else(|| self.current_value());
        let right = self.current_value();
        let equation = format!(
            "{} {} {} =",
            format_display(&format_number(left)),
            operator.label(),
            format_display(&format_number(right))
        );
        self.equation = if record_history {
            self.pending_pasted_expression
                .take()
                .map_or(equation, |expression| format!("{expression} ="))
        } else {
            equation
        };

        let Some(result) = apply_operator(left, right, operator) else {
            self.display = "Error".to_owned();
            self.accumulator = None;
            self.has_error = true;
            self.reset_display_on_next_digit = true;
            if record_history {
                self.history.insert(
                    0,
                    HistoryEntry {
                        equation: self.equation.clone(),
                        result: self.display.clone(),
                    },
                );
            }
            return;
        };

        self.display = format_number(result);
        self.accumulator = Some(result);
        self.reset_display_on_next_digit = true;
        if record_history {
            self.history.insert(
                0,
                HistoryEntry {
                    equation: self.equation.clone(),
                    result: format_display(&self.display),
                },
            );
        }
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

struct PastedExpression {
    tokens: Vec<PasteToken>,
    equation: String,
}

#[derive(Clone, PartialEq, Eq)]
enum PasteToken {
    Number(String),
    Operator(Operator),
    Percent,
    Equals,
}

fn pasted_number_options() -> PasteNumberOptions {
    PasteNumberOptions {
        allow_negative: true,
        max_digits: 18,
    }
}

fn parse_pasted_expression(text: &str) -> Option<PastedExpression> {
    if text.contains(['\r', '\n']) {
        return None;
    }

    let mut tokens = Vec::new();
    let mut index = 0;
    let mut expecting_number = true;

    loop {
        index = skip_whitespace(text, index);
        let Some((character, next_index)) = next_char(text, index) else {
            break;
        };

        if expecting_number {
            let (value, next_index) = parse_expression_number(text, index, tokens.is_empty())?;
            tokens.push(PasteToken::Number(value));
            index = next_index;
            expecting_number = false;
            continue;
        }

        if character == '%' {
            if matches!(tokens.last(), Some(PasteToken::Percent)) {
                return None;
            }
            tokens.push(PasteToken::Percent);
            index = next_index;
            continue;
        }

        if character == '=' {
            index = skip_whitespace(text, next_index);
            if index != text.len() {
                return None;
            }
            tokens.push(PasteToken::Equals);
            break;
        }

        let operator = operator_from_expression_char(character)?;
        tokens.push(PasteToken::Operator(operator));
        index = next_index;
        expecting_number = true;
    }

    if expecting_number || !tokens.iter().any(is_expression_token) {
        return None;
    }

    let equation = pasted_expression_equation(&tokens)?;
    Some(PastedExpression { tokens, equation })
}

fn pasted_expression_equation(tokens: &[PasteToken]) -> Option<String> {
    let mut equation = String::new();

    for token in tokens {
        match token {
            PasteToken::Number(value) => append_expression_number(&mut equation, value),
            PasteToken::Operator(operator) => append_expression_operator(&mut equation, *operator)?,
            PasteToken::Percent => equation.push('%'),
            PasteToken::Equals => {}
        }
    }

    Some(equation.trim().to_owned())
}

fn append_expression_number(equation: &mut String, value: &str) {
    if !equation.is_empty() && !equation.ends_with(' ') {
        equation.push(' ');
    }
    equation.push_str(&format_display(value));
}

fn append_expression_operator(equation: &mut String, operator: Operator) -> Option<()> {
    if equation.trim().is_empty() {
        return None;
    }

    if !equation.ends_with(' ') {
        equation.push(' ');
    }
    equation.push_str(operator.label());
    equation.push(' ');
    Some(())
}

fn parse_expression_number(
    text: &str,
    start_index: usize,
    is_first_token: bool,
) -> Option<(String, usize)> {
    let mut index = start_index;
    let mut has_number_character = false;

    if let Some((sign, next_index)) = next_char(text, index)
        && (sign == '-' || sign == '+')
    {
        if sign == '+' && !is_first_token {
            return None;
        }
        index = next_index;
    }

    while let Some((character, next_index)) = next_char(text, index) {
        if character.is_ascii_digit() || character == ',' || character == '.' {
            has_number_character = true;
            index = next_index;
        } else {
            break;
        }
    }

    if !has_number_character {
        return None;
    }

    let value = parse_pasted_number(&text[start_index..index], pasted_number_options())?;
    Some((value, index))
}

fn is_expression_token(token: &PasteToken) -> bool {
    !matches!(token, PasteToken::Number(_))
}

fn skip_whitespace(text: &str, mut index: usize) -> usize {
    while let Some((character, next_index)) = next_char(text, index) {
        if !character.is_whitespace() {
            break;
        }
        index = next_index;
    }
    index
}

fn next_char(text: &str, index: usize) -> Option<(char, usize)> {
    let character = text.get(index..)?.chars().next()?;
    Some((character, index + character.len_utf8()))
}

fn operator_from_expression_char(character: char) -> Option<Operator> {
    match character {
        '+' => Some(Operator::Add),
        '-' => Some(Operator::Subtract),
        '*' | 'x' | 'X' | '\u{00d7}' => Some(Operator::Multiply),
        '/' | '\u{00f7}' => Some(Operator::Divide),
        _ => None,
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

    let fixed = format_fixed_number(value);

    if format_display(&fixed).chars().count() <= 23 {
        fixed
    } else {
        format_scientific(value)
    }
}

fn format_fixed_number(value: f64) -> String {
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

fn format_scientific(value: f64) -> String {
    let raw = format!("{value:.12e}");
    let Some((mantissa, exponent)) = raw.split_once('e') else {
        return raw;
    };

    let mut mantissa = mantissa.to_owned();

    while mantissa.contains('.') && mantissa.ends_with('0') {
        mantissa.pop();
    }

    if mantissa.ends_with('.') {
        mantissa.pop();
    }

    let exponent = exponent
        .parse::<i32>()
        .map_or_else(|_| exponent.to_owned(), |value| value.to_string());

    format!("{mantissa}e{exponent}")
}

fn format_display(value: &str) -> String {
    if value == "Error" || value.contains('e') {
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

fn significant_digit_count(value: &str) -> usize {
    value.chars().filter(char::is_ascii_digit).count()
}

fn display_font_size(display: &str) -> i32 {
    match display.chars().count() {
        0..=12 => 52,
        13..=16 => 42,
        17..=20 => 28,
        _ => 23,
    }
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

    #[test]
    fn formats_grouped_equation_history() {
        let mut calculator = Calculator::default();

        for digit in "1234567".chars() {
            calculator.press(&digit.to_string());
        }
        assert_eq!(calculator.press("+").equation, "1,234,567 +");

        calculator.press("2");
        assert_eq!(calculator.press("=").equation, "1,234,567 + 2 =");
    }

    #[test]
    fn records_completed_calculations_in_history() {
        let mut calculator = Calculator::default();

        calculator.press("2");
        calculator.press("+");
        calculator.press("3");
        let snapshot = calculator.press("=");

        assert_eq!(snapshot.history.len(), 1);
        assert_eq!(snapshot.history[0].equation, "2 + 3 =");
        assert_eq!(snapshot.history[0].result, "5");
    }

    #[test]
    fn does_not_record_intermediate_operator_chains_in_history() {
        let mut calculator = Calculator::default();

        calculator.press("2");
        calculator.press("+");
        calculator.press("3");
        calculator.press("+");
        assert!(calculator.snapshot().history.is_empty());

        calculator.press("4");
        let snapshot = calculator.press("=");
        assert_eq!(snapshot.history.len(), 1);
        assert_eq!(snapshot.history[0].equation, "5 + 4 =");
    }

    #[test]
    fn clears_history_without_clearing_current_value() {
        let mut calculator = Calculator::default();

        calculator.press("8");
        calculator.press("-");
        calculator.press("3");
        calculator.press("=");
        let snapshot = calculator.clear_history();

        assert_eq!(snapshot.display, "5");
        assert!(snapshot.history.is_empty());
    }

    #[test]
    fn shrinks_long_manual_input() {
        let mut calculator = Calculator::default();

        for digit in "1234567890123".chars() {
            calculator.press(&digit.to_string());
        }

        let snapshot = calculator.snapshot();
        assert_eq!(snapshot.display, "1,234,567,890,123");
        assert_eq!(snapshot.display_font_size, 28);
    }

    #[test]
    fn caps_manual_input_digits() {
        let mut calculator = Calculator::default();

        for digit in "12345678901234567890".chars() {
            calculator.press(&digit.to_string());
        }

        assert_eq!(calculator.display(), "123456789012345678");
    }

    #[test]
    fn pastes_valid_number_and_rejects_invalid_text() {
        let mut calculator = Calculator::default();

        assert_eq!(calculator.paste_input(" 12,345.67 ").display, "12,345.67");
        assert_eq!(calculator.paste_input("15234B").display, "12,345.67");
    }

    #[test]
    fn pasted_number_can_complete_pending_operation() {
        let mut calculator = Calculator::default();

        calculator.press("1");
        calculator.press("2");
        calculator.press("+");
        calculator.paste_input("3");

        assert_eq!(calculator.press("=").display, "15");
    }

    #[test]
    fn pasted_expression_waits_for_equals_when_not_included() {
        let mut calculator = Calculator::default();

        let snapshot = calculator.paste_input("123 + 523");

        assert_eq!(snapshot.display, "523");
        assert_eq!(snapshot.equation, "123 + 523");
        let snapshot = calculator.press("=");

        assert_eq!(snapshot.display, "646");
        assert_eq!(snapshot.history[0].equation, "123 + 523 =");
    }

    #[test]
    fn pasted_expression_with_equals_calculates_and_records_history() {
        let mut calculator = Calculator::default();

        let snapshot = calculator.paste_input("2 + 3 x 4=");

        assert_eq!(snapshot.display, "20");
        assert_eq!(snapshot.equation, "2 + 3 x 4 =");
        assert_eq!(snapshot.history[0].equation, "2 + 3 x 4 =");
        assert_eq!(snapshot.history[0].result, "20");
    }

    #[test]
    fn pasted_expression_accepts_spaces_unicode_operators_and_percent() {
        let mut calculator = Calculator::default();

        assert_eq!(calculator.paste_input(" 50% ").display, "0.5");
        assert_eq!(calculator.paste_input("1,200 \u{00f7} 3=").display, "400");
        assert_eq!(calculator.paste_input("100 + 5%=").display, "100.05");
    }

    #[test]
    fn pasted_multi_operation_history_preserves_full_expression() {
        let mut calculator = Calculator::default();

        let snapshot = calculator.paste_input("12 - 41 + 5 / 51=");

        assert_eq!(snapshot.display, "-0.4705882353");
        assert_eq!(snapshot.equation, "12 - 41 + 5 / 51 =");
        assert_eq!(snapshot.history[0].equation, "12 - 41 + 5 / 51 =");
        assert_eq!(snapshot.history[0].result, "-0.4705882353");
    }

    #[test]
    fn pasted_expression_rejects_malformed_input_as_a_whole() {
        let mut calculator = Calculator::default();

        calculator.paste_input("9");

        for expression in ["1++2", "%5", "5%%", "5%2", "12kg+3", "12 +"] {
            assert_eq!(
                calculator.paste_input(expression).display,
                "9",
                "{expression}"
            );
        }
    }

    #[test]
    fn formats_huge_results_scientifically() {
        let mut calculator = Calculator::default();

        for digit in "999999999999999999".chars() {
            calculator.press(&digit.to_string());
        }
        calculator.press("x");
        calculator.press("9");

        assert!(calculator.press("=").display.contains('e'));
    }
}

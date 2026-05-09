#[derive(Clone, Copy)]
pub struct PasteNumberOptions {
    pub allow_negative: bool,
    pub max_digits: usize,
}

pub fn parse_pasted_number(text: &str, options: PasteNumberOptions) -> Option<String> {
    if text.contains(['\r', '\n']) {
        return None;
    }

    let text = text.trim();
    if text.is_empty() {
        return None;
    }

    let (negative, unsigned) = split_sign(text)?;
    if negative && !options.allow_negative {
        return None;
    }

    let unsigned = validate_unsigned_number(unsigned, options.max_digits)?;
    let normalized = normalize_unsigned(&unsigned);

    if negative && normalized != "0" {
        Some(format!("-{normalized}"))
    } else {
        Some(normalized)
    }
}

fn split_sign(text: &str) -> Option<(bool, &str)> {
    if let Some(rest) = text.strip_prefix('-') {
        Some((true, rest))
    } else if let Some(rest) = text.strip_prefix('+') {
        Some((false, rest))
    } else {
        Some((false, text))
    }
}

fn validate_unsigned_number(text: &str, max_digits: usize) -> Option<String> {
    if text.is_empty() {
        return None;
    }

    let (whole, decimal) = text
        .split_once('.')
        .map_or((text, None), |(whole, decimal)| (whole, Some(decimal)));

    if decimal.is_some_and(|decimal| decimal.is_empty()) {
        return None;
    }

    let whole_digits = validate_whole_part(whole)?;
    let decimal_digits = decimal.map_or(Some(String::new()), validate_decimal_part)?;
    let digit_count = whole_digits.len() + decimal_digits.len();

    if digit_count == 0 || digit_count > max_digits {
        return None;
    }

    if decimal.is_some() {
        Some(format!("{whole_digits}.{decimal_digits}"))
    } else {
        Some(whole_digits)
    }
}

fn validate_whole_part(text: &str) -> Option<String> {
    if text.is_empty() {
        return Some(String::new());
    }

    if text.contains(',') {
        validate_grouped_whole_part(text)
    } else if text.chars().all(|character| character.is_ascii_digit()) {
        Some(text.to_owned())
    } else {
        None
    }
}

fn validate_grouped_whole_part(text: &str) -> Option<String> {
    let mut groups = text.split(',');
    let first = groups.next()?;

    if first.is_empty()
        || first.len() > 3
        || !first.chars().all(|character| character.is_ascii_digit())
    {
        return None;
    }

    let mut normalized = first.to_owned();
    let mut saw_group = false;

    for group in groups {
        saw_group = true;
        if group.len() != 3 || !group.chars().all(|character| character.is_ascii_digit()) {
            return None;
        }
        normalized.push_str(group);
    }

    saw_group.then_some(normalized)
}

fn validate_decimal_part(text: &str) -> Option<String> {
    (!text.is_empty() && text.chars().all(|character| character.is_ascii_digit()))
        .then(|| text.to_owned())
}

fn normalize_unsigned(text: &str) -> String {
    let (whole, decimal) = text.split_once('.').unwrap_or((text, ""));
    let whole = whole.trim_start_matches('0');
    let whole = if whole.is_empty() { "0" } else { whole };

    if decimal.is_empty() {
        whole.to_owned()
    } else {
        format!("{whole}.{decimal}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POSITIVE_ONLY: PasteNumberOptions = PasteNumberOptions {
        allow_negative: false,
        max_digits: 18,
    };
    const SIGNED: PasteNumberOptions = PasteNumberOptions {
        allow_negative: true,
        max_digits: 18,
    };

    #[test]
    fn accepts_plain_and_grouped_numbers() {
        assert_eq!(
            parse_pasted_number("12345", POSITIVE_ONLY),
            Some("12345".to_owned())
        );
        assert_eq!(
            parse_pasted_number(" 12,345.67 ", POSITIVE_ONLY),
            Some("12345.67".to_owned())
        );
        assert_eq!(
            parse_pasted_number(".5", POSITIVE_ONLY),
            Some("0.5".to_owned())
        );
        assert_eq!(
            parse_pasted_number("+.5", POSITIVE_ONLY),
            Some("0.5".to_owned())
        );
    }

    #[test]
    fn handles_negative_numbers_when_allowed() {
        assert_eq!(parse_pasted_number("-.5", SIGNED), Some("-0.5".to_owned()));
        assert_eq!(parse_pasted_number("-40", SIGNED), Some("-40".to_owned()));
        assert_eq!(parse_pasted_number("-40", POSITIVE_ONLY), None);
    }

    #[test]
    fn rejects_malformed_or_ambiguous_pastes() {
        for text in [
            "15234B",
            "12abc34",
            "1.2.3",
            "12,34,567",
            "12kg",
            "1e6",
            "12\n34",
            "12.",
        ] {
            assert_eq!(parse_pasted_number(text, SIGNED), None, "{text}");
        }
    }

    #[test]
    fn enforces_digit_limit() {
        assert_eq!(
            parse_pasted_number("123456789012345678", POSITIVE_ONLY),
            Some("123456789012345678".to_owned())
        );
        assert_eq!(
            parse_pasted_number("1234567890123456789", POSITIVE_ONLY),
            None
        );
    }
}

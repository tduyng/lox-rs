const MAX_DECIMALS: usize = 6;

fn format_number(n: f64) -> String {
    let rounded_number =
        (n * 10_f64.powi(MAX_DECIMALS as i32)).round() / 10_f64.powi(MAX_DECIMALS as i32);

    let mut formatted = format!("{:.*}", MAX_DECIMALS, rounded_number);

    formatted = formatted.trim_end_matches('0').to_string();

    if formatted.ends_with('.') {
        formatted.pop();
    }

    formatted
}

pub fn format_evaluated_number(n: f64) -> String {
    format_number(n)
}

pub fn format_parsed_number(n: f64) -> String {
    let mut formatted = format_number(n);

    if n.fract() == 0.0 && !formatted.contains('.') {
        formatted.push_str(".0");
    }

    formatted
}

pub fn format_tokenized_number(value: &str) -> String {
    let number = value.parse::<f64>().unwrap_or(0.0);
    format_parsed_number(number)
}

pub fn pad_number(number: f64) -> String {
    let mut value = number.to_string();
    if !value.contains('.') {
        value.push_str(".0");
    }
    value
}

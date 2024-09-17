pub fn pad_number(number: f64) -> String {
    let mut value = number.to_string();
    if !value.contains('.') {
        value.push_str(".0");
    }
    value
}

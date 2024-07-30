use email_address::EmailAddress;

pub fn sanitize_cc_list(input: &str) -> Option<&str> {
    let value = input.trim();
    let valid_input = value.split(",").all(EmailAddress::is_valid);
    if valid_input {
        Some(value)
    } else {
        None
    }
}
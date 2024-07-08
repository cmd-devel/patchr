use email_address::EmailAddress;

pub fn sanitize_cc_list(input: &str) -> Option<&str> {
    let value = input.trim();
    let valid_input = value.split(",").any(|elt| {
        EmailAddress::is_valid(elt)
    });
    if valid_input {
        Some(value)
    } else {
        None
    }
}
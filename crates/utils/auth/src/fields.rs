use error_mapper::errors::AerodisSpaceError;
use regex::Regex;

pub fn check_password_field(password: &String) -> Result<(), AerodisSpaceError> {
    let mut errors_msg: Vec<&str> = vec![];
    if password.is_empty() {
        errors_msg.push("Password cannot be empty");
    }
    if password.len() < 12 {
        errors_msg.push("Password must be at least 12 characters long");
    }

    let rgx_capital_letter = Regex::new(r"[A-Z]").unwrap();
    let rgx_lower_letter = Regex::new(r"[a-z]").unwrap();
    let rgx_special = Regex::new(r"\d").unwrap();
    let rgx_number = Regex::new(r"[^\w\s]").unwrap();
    if !rgx_capital_letter.is_match(&password)
        && !rgx_lower_letter.is_match(&password)
        && !rgx_special.is_match(&password)
        && !rgx_number.is_match(&password)
    {
        errors_msg.push(
            "Password must be at least 12 characteres long, 1 Capital Letter, 1 lower case letter, 1 number and a special caractere (#)",
        );
    }
    if (errors_msg.len() > 0) {
        return Err(AerodisSpaceError::InvalidField(errors_msg.join("\n")));
    }

    Ok(())
}

pub fn check_email_field(email: &String) -> Result<(), AerodisSpaceError> {
    let mut errors_msg: Vec<&str> = vec![];
    if email.is_empty() {
        errors_msg.push("Email cannot be empty");
    }
    let rgx_email = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if !rgx_email.is_match(email) {
        errors_msg.push("Invalid email format");
    }

    if !errors_msg.is_empty() {
        return Err(AerodisSpaceError::InvalidField(errors_msg.join("\n")));
    }

    Ok(())
}

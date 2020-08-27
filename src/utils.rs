use regex::Regex;

pub fn check_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}

pub fn check_valid_password(pw: &str) -> Result<(), InvalidPasswordErr> {
    if pw.len() < 8 {
        return Err(InvalidPasswordErr::TooShort);
    } else {
        if pw.len() > 32 {
            return Err(InvalidPasswordErr::TooLong);
        } else {
            return Ok(());
        }
    }
}

pub enum InvalidPasswordErr {
    MissingDigit,
    MissingLowercase,
    MissingUppercase,
    MissingSpecialChar,
    TooLong,
    TooShort,
}

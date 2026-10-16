#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingEquals,
    InvalidNumber,
}

pub fn parse_key_value(input: &str) -> Result<(&str, i32), ParseError> {
    let (key, val_str) = input.split_once('=').ok_or(ParseError::MissingEquals)?;
    let val = val_str
        .trim()
        .parse::<i32>()
        .map_err(|_| ParseError::InvalidNumber)?;
    Ok((key.trim(), val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_key_value("timeout = 30"), Ok(("timeout", 30)));
        assert_eq!(parse_key_value("bad_format"), Err(ParseError::MissingEquals));
        assert_eq!(parse_key_value("port = abc"), Err(ParseError::InvalidNumber));
    }
}

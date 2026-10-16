pub fn luhn_validate(code: &str) -> bool {
    let sanitized: Vec<u32> = match code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10).ok_or(()))
        .collect::<Result<Vec<u32>, ()>>()
    {
        Ok(digits) => digits,
        Err(_) => return false,
    };

    if sanitized.len() <= 1 {
        return false;
    }

    let mut sum = 0;
    for (i, &digit) in sanitized.iter().rev().enumerate() {
        if i % 2 == 1 {
            let doubled = digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += digit;
        }
    }

    sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn() {
        assert!(luhn_validate("046 454 286"));
        assert!(luhn_validate("49927398716"));
        assert!(!luhn_validate("49927398717"));
        assert!(!luhn_validate("0"));
    }
}

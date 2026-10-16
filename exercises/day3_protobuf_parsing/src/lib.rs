#[derive(Debug, PartialEq, Eq)]
pub struct Field<'a> {
    pub tag: u64,
    pub value: &'a str,
}

pub fn parse_field<'a>(buffer: &'a str) -> Option<(Field<'a>, &'a str)> {
    let (tag_str, rest) = buffer.split_once(':')?;
    let tag: u64 = tag_str.parse().ok()?;

    let (len_str, rest) = rest.split_once(':')?;
    let len: usize = len_str.parse().ok()?;

    if rest.len() < len {
        return None;
    }

    let value = &rest[..len];
    let remainder = &rest[len..];

    Some((Field { tag, value }, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protobuf() {
        let buffer = "1:4:PING2:4:PONG";
        let (f1, rest1) = parse_field(buffer).unwrap();
        assert_eq!(f1, Field { tag: 1, value: "PING" });

        let (f2, rest2) = parse_field(rest1).unwrap();
        assert_eq!(f2, Field { tag: 2, value: "PONG" });
        assert_eq!(rest2, "");
    }
}

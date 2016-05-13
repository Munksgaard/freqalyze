#![feature(inclusive_range_syntax)]

fn score_byte(c: u8) -> Option<u64> {
    let c = if b'A' <= c && c <= b'Z' {
        c - b'A' + b'a'
    } else {
        c
    };

    match c {
        b'e' => Some(5),
        b'a' | b't' => Some(4),
        b'h' | b'i' | b'n' | b'o' | b'r' | b's' | b' ' => Some(3),
        b'b' | b'c' | b'd' | b'f' | b'g' | b'k' | b'l' | b'm' | b'p' | b'u'
            | b'v' | b'w' | b'y' => Some(2),
        b'j' | b'q' | b'x' | b'z' => Some(1),
        x if x < b' ' && x != b'\n' || x >= 128 => None,
        _ => Some(0),
    }
}

#[test]
fn test_score_byte_uppercase() {
    for c in b'A'...b'Z' {
        assert_eq!(score_byte(c), score_byte(c - b'A' + b'a'));
    }
}

pub fn score(bytes: &[u8]) -> u64 {
    let mut result = 0;
    for &b in bytes.iter() {
        if let Some(score) = score_byte(b) {
            result += score;
        } else {
            return 0;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let bytes = b"ETAOIN SHRDLU";

        assert_eq!(40, score(bytes));
    }
}

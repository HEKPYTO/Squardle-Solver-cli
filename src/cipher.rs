use base64::{engine::general_purpose, Engine as _};
use std::sync::LazyLock;

const CHAR_MAP: &str = "5pyf0gcrl1a9oe3ui8d2htn67sqjkxbmw4vzPYFGCRLAOEUIDHTNSQJKXBMWVZ";

static DECODE_TABLE: LazyLock<[u8; 128]> = LazyLock::new(|| {
    let chars: Vec<u8> = CHAR_MAP.bytes().collect();
    let len = chars.len();
    let mut table = [0u8; 128];
    for i in 0..128u8 {
        table[i as usize] = i;
    }
    for (idx, &ch) in chars.iter().enumerate() {
        let new_idx = (idx + len - 12) % len;
        table[ch as usize] = chars[new_idx];
    }
    table
});

pub fn decode_words(encoded: &str) -> anyhow::Result<Vec<String>> {
    let mut shifted: String = encoded
        .chars()
        .map(|c| {
            if c.is_ascii() {
                DECODE_TABLE[c as usize] as char
            } else {
                c
            }
        })
        .collect();

    while !shifted.len().is_multiple_of(4) {
        shifted.push('=');
    }

    let decoded_bytes = general_purpose::STANDARD.decode(&shifted)?;
    let decoded_str = String::from_utf8(decoded_bytes)?;

    let words: Vec<String> = decoded_str
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(words)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::LazyLock;

    static ENCODE_TABLE: LazyLock<[u8; 128]> = LazyLock::new(|| {
        let chars: Vec<u8> = CHAR_MAP.bytes().collect();
        let len = chars.len();
        let mut table = [0u8; 128];
        for i in 0..128u8 {
            table[i as usize] = i;
        }
        for (idx, &ch) in chars.iter().enumerate() {
            let new_idx = (idx + 12) % len;
            table[ch as usize] = chars[new_idx];
        }
        table
    });

    pub fn encode_mock(plain: &str) -> String {
        let b64 = general_purpose::STANDARD.encode(plain);
        b64.chars()
            .map(|c| {
                if c.is_ascii() {
                    ENCODE_TABLE[c as usize] as char
                } else {
                    c
                }
            })
            .collect()
    }

    #[test]
    fn test_decode_words() {
        let encoded = encode_mock("hello,world");
        let decoded = decode_words(&encoded).unwrap();
        assert_eq!(decoded, vec!["hello", "world"]);
    }

    #[test]
    fn test_decode_with_padding() {
        let encoded = encode_mock("h");
        let decoded = decode_words(&encoded).unwrap();
        assert_eq!(decoded, vec!["h"]);
    }

    #[test]
    fn test_invalid_base64() {
        let encode_table = &*ENCODE_TABLE;
        let encoded: String = "////"
            .chars()
            .map(|c| {
                if c.is_ascii() {
                    encode_table[c as usize] as char
                } else {
                    c
                }
            })
            .collect();

        assert!(decode_words(&encoded).is_err());
    }

    #[test]
    fn test_decode_empty_string() {
        let encoded = encode_mock("");
        let decoded = decode_words(&encoded).unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_non_map_chars_passthrough() {
        let table = &*DECODE_TABLE;
        assert_eq!(table[b'=' as usize], b'=');
        assert_eq!(table[b'+' as usize], b'+');
        assert_eq!(table[b'/' as usize], b'/');
    }

    #[test]
    fn test_decode_multi_words() {
        let encoded = encode_mock("alpha,bravo,charlie,delta,echo");
        let decoded = decode_words(&encoded).unwrap();
        assert_eq!(
            decoded,
            vec!["alpha", "bravo", "charlie", "delta", "echo"]
        );
    }

    #[test]
    fn test_roundtrip_various_lengths() {
        for len in 1..=20 {
            let word: String = (0..len).map(|i| (b'a' + (i % 26)) as char).collect();
            let encoded = encode_mock(&word);
            let decoded = decode_words(&encoded).unwrap();
            assert_eq!(decoded, vec![word]);
        }
    }
}

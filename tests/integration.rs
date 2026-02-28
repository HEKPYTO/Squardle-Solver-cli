use squaredle_cli::fetcher::extract_grid_and_words_from_js;

use base64::{engine::general_purpose::STANDARD, Engine as _};

const CHAR_MAP: &str = "5pyf0gcrl1a9oe3ui8d2htn67sqjkxbmw4vzPYFGCRLAOEUIDHTNSQJKXBMWVZ";

fn encode_mock(words: Vec<&str>) -> String {
    let plain = words.join(",");
    let b64 = STANDARD.encode(plain);
    let char_map: Vec<char> = CHAR_MAP.chars().collect();
    let map_len = char_map.len() as isize;

    b64.chars()
        .map(|c| {
            if let Some(idx) = char_map.iter().position(|&x| x == c) {
                let new_idx = (idx as isize + 12) % map_len;
                char_map[new_idx as usize]
            } else {
                c
            }
        })
        .collect()
}

#[test]
fn test_end_to_end_parsing() {
    let encoded_words = encode_mock(vec!["hello", "world", "rust"]);
    let encoded_optional = encode_mock(vec!["cli", "tool"]);

    let js = format!(
        r#"
        const gTodayDateStr = '2026/02/25';
        const config = {{
            "2026\/02\/25-xp": {{ "board": ["no"] }},
            "2026\/02\/25": {{
                "board": ["abcd", "efgh", "ijkl", "mnop"],
                "wordScores": "{}",
                "optionalWordScores": "{}"
            }}
        }};
        "#,
        encoded_words, encoded_optional
    );

    let data = extract_grid_and_words_from_js(&js).unwrap();

    assert_eq!(data.date, "2026-02-25");
    assert_eq!(data.grid, "abcd-efgh-ijkl-mnop");

    let expected_words = vec!["hello", "world", "rust", "cli", "tool"];
    assert_eq!(data.words.len(), 5);
    for w in expected_words {
        assert!(data.words.contains(&w.to_string()));
    }
}

#[test]
fn test_single_date_config() {
    let encoded = encode_mock(vec!["alpha", "bravo"]);
    let js = format!(
        r#"
        const gTodayDateStr = '2026/03/01';
        const config = {{
            "2026\/03\/01": {{
                "board": ["xy", "zw"],
                "wordScores": "{}"
            }}
        }};
        "#,
        encoded
    );
    let data = extract_grid_and_words_from_js(&js).unwrap();
    assert_eq!(data.date, "2026-03-01");
    assert_eq!(data.grid, "xy-zw");
    assert_eq!(data.words, vec!["alpha", "bravo"]);
}

#[test]
fn test_words_preserved_order() {
    let encoded = encode_mock(vec!["zulu", "alpha", "mike", "bravo"]);
    let js = format!(
        r#"
        "board": ["ab", "cd"],
        "wordScores": "{}"
        "#,
        encoded
    );
    let data = extract_grid_and_words_from_js(&js).unwrap();
    assert_eq!(data.words, vec!["zulu", "alpha", "mike", "bravo"]);
}

#[test]
fn test_no_optional_words() {
    let encoded = encode_mock(vec!["only", "required"]);
    let js = format!(
        r#"
        const gTodayDateStr = '2026/04/10';
        const config = {{
            "2026\/04\/10": {{
                "board": ["mn", "op"],
                "wordScores": "{}"
            }}
        }};
        "#,
        encoded
    );
    let data = extract_grid_and_words_from_js(&js).unwrap();
    assert_eq!(data.words, vec!["only", "required"]);
}

use anyhow::{anyhow, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;
use serde::Serialize;
use std::sync::LazyLock;

use crate::cipher::decode_words;

static DATE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"gTodayDateStr\s*=\s*'([^']+)'").unwrap());
static NEXT_DATE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#","20\d{2}\\/\d{2}\\/\d{2}"#).unwrap());
static BOARD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?s)"board"\s*:\s*\[(.*?)\]"#).unwrap());
static ROW_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""([^"]+)""#).unwrap());
static WORDS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""wordScores"\s*:\s*"([^"]+)""#).unwrap());
static OPTIONAL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""optionalWordScores"\s*:\s*"([^"]+)""#).unwrap());

#[derive(Debug, Serialize, Clone)]
pub struct PuzzleData {
    pub date: String,
    pub grid: String,
    pub words: Vec<String>,
}

pub fn fetch_today_puzzle() -> Result<PuzzleData> {
    let url = "https://squaredle.app/api/today-puzzle-config.js";
    let client = Client::builder()
        .user_agent("squaredle-cli/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;
    let body = client
        .get(url)
        .send()
        .context("Failed to fetch puzzle from squaredle.app")?
        .text()
        .context("Failed to read response body")?;
    extract_grid_and_words_from_js(&body)
}

pub fn extract_grid_and_words_from_js(js_content: &str) -> Result<PuzzleData> {
    let today_date = DATE_RE
        .captures(js_content)
        .map(|caps| caps[1].replace('/', "\\/"));

    let key_string = if let Some(ref date) = today_date {
        format!("\"{}\":", date)
    } else {
        "\"board\":".to_string()
    };

    let parts: Vec<&str> = js_content.split(&key_string).collect();
    if parts.len() < 2 {
        return Err(anyhow!("Could not locate puzzle key in response"));
    }

    let rest = parts[1];
    let puzzle_block = match NEXT_DATE_RE.find(rest) {
        Some(m) => &rest[..m.start()],
        None => rest,
    };

    let (board_inner_str, words_source) = if let Some(caps) = BOARD_RE.captures(puzzle_block) {
        (caps.get(1).unwrap().as_str(), puzzle_block)
    } else if let Some(caps) = BOARD_RE.captures(js_content) {
        (caps.get(1).unwrap().as_str(), puzzle_block)
    } else {
        return Err(anyhow!("Could not locate board array in response"));
    };

    let rows: Vec<String> = ROW_RE
        .captures_iter(board_inner_str)
        .map(|caps| caps[1].to_string())
        .filter(|row| row != "board")
        .collect();

    if rows.is_empty() {
        return Err(anyhow!("Board array found but no rows extracted"));
    }

    let grid_str = rows.join("-");

    let mut all_words = Vec::new();

    if let Some(caps) = WORDS_RE.captures(words_source) {
        let mut words = decode_words(&caps[1])?;
        all_words.append(&mut words);
    }

    if let Some(caps) = OPTIONAL_RE.captures(words_source) {
        let mut optional_words = decode_words(&caps[1])?;
        all_words.append(&mut optional_words);
    }

    Ok(PuzzleData {
        date: today_date
            .unwrap_or_else(|| "unknown".to_string())
            .replace("\\/", "-"),
        grid: grid_str,
        words: all_words,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_grid_no_words() {
        let js = r#"
            const gTodayDateStr = '2026/02/25';
            const config = {
                "2026\/02\/25": { "board": ["yes", "yes"] }
            }
        "#;
        let result = extract_grid_and_words_from_js(js).unwrap();
        assert_eq!(result.grid, "yes-yes");
        assert!(result.words.is_empty());
    }

    #[test]
    fn test_extract_fallback() {
        let js = r#"
            "board": ["a", "b"]
        "#;
        let result = extract_grid_and_words_from_js(js).unwrap();
        assert_eq!(result.grid, "a-b");
    }

    #[test]
    fn test_missing_puzzle_key() {
        let js = "const something_else = true;";
        let err = extract_grid_and_words_from_js(js).unwrap_err();
        assert_eq!(err.to_string(), "Could not locate puzzle key in response");
    }

    #[test]
    fn test_missing_board_array() {
        let js = r#"
            const gTodayDateStr = '2026/02/25';
            const config = {
                "2026\/02\/25": { "not_board": ["yes", "yes"] }
            }
        "#;
        let err = extract_grid_and_words_from_js(js).unwrap_err();
        assert_eq!(err.to_string(), "Could not locate board array in response");
    }

    #[test]
    fn test_empty_rows_extracted() {
        let js = r#"
            "board": ["board"]
        "#;
        let err = extract_grid_and_words_from_js(js).unwrap_err();
        assert_eq!(err.to_string(), "Board array found but no rows extracted");
    }

    #[test]
    fn test_optional_word_scores() {
        let enc1 = crate::cipher::tests::encode_mock("rust");
        let enc2 = crate::cipher::tests::encode_mock("cli");
        let js = format!(
            r#"
            "board": ["abcd", "efgh"],
            "wordScores": "{}",
            "optionalWordScores": "{}"
        "#,
            enc1, enc2
        );

        let result = extract_grid_and_words_from_js(&js).unwrap();
        assert_eq!(result.grid, "abcd-efgh");
        assert_eq!(result.words.len(), 2);
        assert!(result.words.contains(&"rust".to_string()));
        assert!(result.words.contains(&"cli".to_string()));
    }

    #[test]
    fn test_multi_date_selects_correct() {
        let enc_today = crate::cipher::tests::encode_mock("today_word");
        let enc_other = crate::cipher::tests::encode_mock("other_word");
        let js = format!(
            r#"
            const gTodayDateStr = '2026/02/25';
            const config = {{
                "2026\/02\/24": {{
                    "board": ["xx", "yy"],
                    "wordScores": "{}"
                }},
                "2026\/02\/25": {{
                    "board": ["ab", "cd"],
                    "wordScores": "{}"
                }},
                "2026\/02\/26": {{
                    "board": ["mm", "nn"],
                    "wordScores": "{}"
                }}
            }};
            "#,
            enc_other, enc_today, enc_other
        );
        let data = extract_grid_and_words_from_js(&js).unwrap();
        assert_eq!(data.grid, "ab-cd");
        assert!(data.words.contains(&"today_word".to_string()));
        assert!(!data.words.contains(&"other_word".to_string()));
    }

    #[test]
    fn test_date_formatting_in_output() {
        let js = r#"
            const gTodayDateStr = '2026/02/25';
            const config = {
                "2026\/02\/25": { "board": ["ab", "cd"] }
            }
        "#;
        let data = extract_grid_and_words_from_js(js).unwrap();
        assert_eq!(data.date, "2026-02-25");
    }

    #[test]
    fn test_non_square_grid() {
        let js = r#"
            "board": ["abc", "de", "fghi"]
        "#;
        let data = extract_grid_and_words_from_js(js).unwrap();
        assert_eq!(data.grid, "abc-de-fghi");
    }

    #[test]
    fn test_whitespace_variations() {
        let js = "\"board\":  \n  [  \"abc\"  ,  \n  \"def\"  ]  ";
        let data = extract_grid_and_words_from_js(js).unwrap();
        assert_eq!(data.grid, "abc-def");
    }

    #[test]
    fn test_word_scores_only() {
        let enc = crate::cipher::tests::encode_mock("alpha,bravo");
        let js = format!(
            r#"
            "board": ["ab", "cd"],
            "wordScores": "{}"
        "#,
            enc
        );
        let data = extract_grid_and_words_from_js(&js).unwrap();
        assert_eq!(data.words, vec!["alpha", "bravo"]);
    }

    #[test]
    fn test_optional_scores_only() {
        let enc = crate::cipher::tests::encode_mock("gamma,delta");
        let js = format!(
            r#"
            "board": ["ab", "cd"],
            "optionalWordScores": "{}"
        "#,
            enc
        );
        let data = extract_grid_and_words_from_js(&js).unwrap();
        assert_eq!(data.words, vec!["gamma", "delta"]);
    }
}

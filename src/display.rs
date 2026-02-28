use crate::fetcher::PuzzleData;
use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use std::collections::BTreeMap;

pub fn format_puzzle(data: &PuzzleData) -> String {
    let mut out = String::new();

    out.push_str("\n┌─────────────────────────────┐\n");
    out.push_str(&format!(
        "│     {:<23} │\n",
        "Daily Squaredle".bold().cyan()
    ));
    out.push_str(&format!("│     {:<23} │\n", data.date.yellow()));
    out.push_str("└─────────────────────────────┘\n\n");

    out.push_str(&format!("  {}\n", "Grid:".bold()));
    let mut grid_table = Table::new();
    grid_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);
    for row_str in data.grid.split('-') {
        let row_cells: Vec<Cell> = row_str
            .chars()
            .map(|c| Cell::new(c.to_uppercase().to_string()).fg(comfy_table::Color::Green))
            .collect();
        grid_table.add_row(row_cells);
    }
    for line in grid_table.to_string().lines() {
        out.push_str(&format!("  {}\n", line));
    }
    out.push('\n');

    let mut words_by_len: BTreeMap<usize, Vec<String>> = BTreeMap::new();
    for word in &data.words {
        words_by_len
            .entry(word.len())
            .or_default()
            .push(word.to_uppercase());
    }
    for words in words_by_len.values_mut() {
        words.sort();
    }

    out.push_str(&format!(
        "  {}\n",
        format!("Words ({} total):", data.words.len()).bold()
    ));

    for (len, words) in words_by_len.iter().rev() {
        let mut words_table = Table::new();
        words_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        words_table.set_header(vec![
            Cell::new(format!("{}-letter ({})", len, words.len())).fg(comfy_table::Color::Cyan)
        ]);

        for chunk in words.chunks(4) {
            let chunk_str = chunk.join("  ");
            words_table.add_row(vec![Cell::new(chunk_str)]);
        }
        for line in words_table.to_string().lines() {
            out.push_str(&format!("  {}\n", line));
        }
    }
    out.push('\n');

    out.push_str(&format!("  {}\n", "Summary:".bold()));
    out.push_str("  ─────────────────────────────\n");
    out.push_str(&format!("  Total words: {}\n", data.words.len()));
    let mut summary_parts = Vec::new();
    for (len, words) in words_by_len.iter().rev() {
        summary_parts.push(format!("{}-letter: {}", len, words.len()));
    }

    let mut current_line = String::from("  ");
    for (i, part) in summary_parts.iter().enumerate() {
        if i > 0 {
            let next_len = current_line.len() + 3 + part.len();
            if next_len > 60 {
                out.push_str(&format!("{}\n", current_line));
                current_line = format!("  {}", part);
                continue;
            }
            current_line.push_str(" │ ");
        }
        current_line.push_str(part);
    }
    if current_line.len() > 2 {
        out.push_str(&format!("{}\n", current_line));
    }
    out.push('\n');

    out
}

pub fn print_puzzle(data: &PuzzleData) {
    print!("{}", format_puzzle(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_puzzle() {
        colored::control::set_override(false);
        let data = PuzzleData {
            date: "2026-02-28".to_string(),
            grid: "abcd-efgh".to_string(),
            words: vec!["hello".to_string(), "world".to_string(), "rust".to_string()],
        };
        let formatted = format_puzzle(&data);
        assert!(formatted.contains("Daily Squaredle"));
        assert!(formatted.contains("2026-02-28"));
        assert!(formatted.contains("A ┆ B ┆ C ┆ D") || formatted.contains("A │ B │ C │ D"));
        assert!(formatted.contains("5-letter (2)"));
        assert!(formatted.contains("HELLO  WORLD"));
        assert!(formatted.contains("4-letter (1)"));
        assert!(formatted.contains("RUST"));
        assert!(formatted.contains("Total words: 3"));
        assert!(formatted.contains("5-letter: 2 │ 4-letter: 1"));
    }

    #[test]
    fn test_format_puzzle_empty_words() {
        colored::control::set_override(false);
        let data = PuzzleData {
            date: "2026-01-01".to_string(),
            grid: "ab-cd".to_string(),
            words: vec![],
        };
        let formatted = format_puzzle(&data);
        assert!(formatted.contains("Words (0 total):"));
        assert!(formatted.contains("Total words: 0"));
    }

    #[test]
    fn test_format_puzzle_single_word() {
        colored::control::set_override(false);
        let data = PuzzleData {
            date: "2026-01-01".to_string(),
            grid: "ab-cd".to_string(),
            words: vec!["test".to_string()],
        };
        let formatted = format_puzzle(&data);
        assert!(formatted.contains("4-letter (1)"));
        assert!(formatted.contains("TEST"));
        assert!(formatted.contains("Total words: 1"));
    }

    #[test]
    fn test_format_puzzle_many_lengths() {
        colored::control::set_override(false);
        let data = PuzzleData {
            date: "2026-01-01".to_string(),
            grid: "ab-cd".to_string(),
            words: vec![
                "abcd".to_string(),
                "abcde".to_string(),
                "abcdef".to_string(),
                "abcdefg".to_string(),
                "abcdefgh".to_string(),
                "abcdefghi".to_string(),
                "abcdefghij".to_string(),
            ],
        };
        let formatted = format_puzzle(&data);
        assert!(formatted.contains("4-letter (1)"));
        assert!(formatted.contains("10-letter (1)"));
        assert!(formatted.contains("Total words: 7"));
    }
}

pub fn format_json(data: &PuzzleData) -> serde_json::Result<String> {
    let mut words_by_len: std::collections::BTreeMap<usize, Vec<String>> =
        std::collections::BTreeMap::new();
    for word in &data.words {
        words_by_len
            .entry(word.len())
            .or_default()
            .push(word.clone());
    }

    let output = serde_json::json!({
        "date": data.date,
        "grid": data.grid,
        "grid_rows": data.grid.split('-').collect::<Vec<_>>(),
        "words": data.words,
        "word_count": data.words.len(),
        "by_length": words_by_len,
    });
    
    serde_json::to_string_pretty(&output)
}

#[cfg(test)]
mod json_tests {
    use super::*;

    #[test]
    fn test_format_json() {
        let data = PuzzleData {
            date: "2026-02-28".to_string(),
            grid: "abcd-efgh".to_string(),
            words: vec!["hello".to_string(), "world".to_string(), "rust".to_string()],
        };
        let formatted = format_json(&data).unwrap();
        assert!(formatted.contains("\"date\": \"2026-02-28\""));
        assert!(formatted.contains("\"abcd\""));
        assert!(formatted.contains("\"word_count\": 3"));
    }

    #[test]
    fn test_format_json_empty_words() {
        let data = PuzzleData {
            date: "2026-01-01".to_string(),
            grid: "ab-cd".to_string(),
            words: vec![],
        };
        let formatted = format_json(&data).unwrap();
        let v: serde_json::Value = serde_json::from_str(&formatted).unwrap();
        assert_eq!(v["word_count"], 0);
        assert!(v["words"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_format_json_grid_rows() {
        let data = PuzzleData {
            date: "2026-01-01".to_string(),
            grid: "abc-def-ghi".to_string(),
            words: vec![],
        };
        let formatted = format_json(&data).unwrap();
        let v: serde_json::Value = serde_json::from_str(&formatted).unwrap();
        let rows = v["grid_rows"].as_array().unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], "abc");
        assert_eq!(rows[1], "def");
        assert_eq!(rows[2], "ghi");
    }

    #[test]
    fn test_format_json_by_length() {
        let data = PuzzleData {
            date: "2026-01-01".to_string(),
            grid: "ab-cd".to_string(),
            words: vec![
                "rust".to_string(),
                "code".to_string(),
                "hello".to_string(),
            ],
        };
        let formatted = format_json(&data).unwrap();
        let v: serde_json::Value = serde_json::from_str(&formatted).unwrap();
        let by_length = &v["by_length"];
        assert_eq!(by_length["4"].as_array().unwrap().len(), 2);
        assert_eq!(by_length["5"].as_array().unwrap().len(), 1);
    }
}

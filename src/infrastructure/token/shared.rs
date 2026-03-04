//! Simple heuristic tokenizer that works for all models.
//!
//! Uses character-based estimation with CJK (Chinese, Japanese, Korean) awareness.
//! Accuracy: ±10-15% for most text, sufficient for budget estimation.

/// Count tokens using a simple heuristic.
///
/// - Latin/code text: ~1 token per 4 characters
/// - CJK text: ~1 token per 1.5 characters (CJK characters are denser)
pub fn count_tokens(text: &str) -> u32 {
    let chars = text.chars().count();
    if chars == 0 {
        return 0;
    }

    let cjk_count = text.chars().filter(|c| is_cjk(*c)).count();
    let cjk_ratio = cjk_count as f32 / chars as f32;

    let estimate = if cjk_ratio > 0.3 {
        // CJK-heavy text: ~1 token per 1.5 characters
        (chars as f32 / 1.5).ceil() as u32
    } else {
        // Latin/code text: ~1 token per 4 characters
        (chars as f32 / 4.0).ceil() as u32
    };

    estimate.max(1)
}

/// Check if a character is CJK (Chinese, Japanese, Korean).
fn is_cjk(c: char) -> bool {
    matches!(
        c,
        '\u{4E00}'..='\u{9FFF}'     // Chinese (CJK Unified Ideographs)
        | '\u{3400}'..='\u{4DBF}'   // Chinese Extension A
        | '\u{20000}'..='\u{2A6DF}' // Chinese Extension B
        | '\u{2A700}'..='\u{2B73F}' // Chinese Extension C
        | '\u{2B740}'..='\u{2B81F}' // Chinese Extension D
        | '\u{2B820}'..='\u{2CEAF}' // Chinese Extension E
        | '\u{3040}'..='\u{309F}'   // Japanese Hiragana
        | '\u{30A0}'..='\u{30FF}'   // Japanese Katakana
        | '\u{31F0}'..='\u{31FF}'   // Katakana Phonetic Extensions
        | '\u{AC00}'..='\u{D7AF}'   // Korean Hangul Syllables
        | '\u{1100}'..='\u{11FF}'   // Korean Hangul Jamo
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(count_tokens(""), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(count_tokens("a"), 1);
    }

    #[test]
    fn test_latin_text() {
        // "Hello, world!" = 13 chars -> ~4 tokens
        let count = count_tokens("Hello, world!");
        assert!(
            (3..=5).contains(&count),
            "Expected ~4 tokens, got {}",
            count
        );
    }

    #[test]
    fn test_code_text() {
        // Code is roughly 1 token per 4 chars
        let code = "fn main() { println!(\"Hello\"); }";
        let count = count_tokens(code);
        // 32 chars -> ~8 tokens
        assert!(
            (6..=10).contains(&count),
            "Expected ~8 tokens, got {}",
            count
        );
    }

    #[test]
    fn test_cjk_text() {
        // Chinese text: 1 token per ~1.5 chars
        let text = "你好世界"; // 4 Chinese characters
        let count = count_tokens(text);
        // 4 chars / 1.5 = ~3 tokens
        assert!(
            (2..=4).contains(&count),
            "Expected ~3 tokens, got {}",
            count
        );
    }

    #[test]
    fn test_mixed_text() {
        // Mixed Latin and CJK
        let text = "Hello 你好 World 世界";
        let count = count_tokens(text);
        // Should be somewhere between pure Latin and pure CJK estimates
        assert!(count > 0, "Expected positive token count, got {}", count);
    }

    #[test]
    fn test_long_text() {
        let text = "a".repeat(100);
        let count = count_tokens(&text);
        // 100 chars / 4 = 25 tokens
        assert!(
            (23..=27).contains(&count),
            "Expected ~25 tokens, got {}",
            count
        );
    }
}

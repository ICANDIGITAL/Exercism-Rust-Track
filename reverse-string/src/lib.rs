use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    //input.chars().rev().collect::<String>() THIS CODE WORKS ON ONLY NON-GRAPHEME STRINGS.
    input.graphemes(true).rev().collect::<String>()
}

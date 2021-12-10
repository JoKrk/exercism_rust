use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {

    let mut reversed = String::new();
    let mut input_chars = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    input_chars.reverse();
    for c in input_chars.iter() {
        
        for char in c.chars() {
            reversed.push(char);
        }
    }

    reversed
}

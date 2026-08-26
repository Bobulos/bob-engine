use u4::{U4, U4x2};

// /// This can be used to translate a string into uv's of a font sprite sheet
// pub struct FontBinder {
    
// }

// impl FontBinder {
    
// }
pub enum FontSheetBinding {
    NonStandardTestPallate
    // 600 X 240
    // A B C D E F G H I J K L M
    // N O P Q R S T U V W X Y Z
    // 0 1 2 3 4 5 6 7 8 9
    // . : ; , / \ + - *
    // & e` " ' ( [ { e` _ c` a` } ] ) =
    // < > ? ! S u` $ $ * @ # ^2
    
}
pub fn convert_str_to_bound<const MAX_TEXT_LENGTH :usize>(text: &str, binding: FontSheetBinding) -> [U4x2; MAX_TEXT_LENGTH] {
    debug_assert!(text.len() < MAX_TEXT_LENGTH, "str too long {} max allowed {}", text.len(), MAX_TEXT_LENGTH);
    let mut packed = [U4x2::default(); MAX_TEXT_LENGTH];
    for (i, c) in text.chars().enumerate() {
        packed[i] = get_pos_from_char(c, &binding);
    }
    
    packed
}
fn get_pos_from_char(c: char, font_sheet_format: &FontSheetBinding) -> U4x2 {
    let mut bind: [u8; 2] = [0, 0];
    match font_sheet_format {
        FontSheetBinding::NonStandardTestPallate => {
            let c = c.to_ascii_lowercase();
            bind = match c {
                'a' => [0, 0],
                'b' => [1, 0],
                'c' => [2, 0],
                'd' => [3, 0],
                'e' => [4, 0],
                'f' => [5, 0],
                'g' => [6, 0],
                'h' => [7, 0],
                'i' => [8, 0],
                'j' => [9, 0],
                'k' => [10, 0],
                'l' => [11, 0],
                'm' => [12, 0],
                'n' => [0, 1],
                'o' => [1, 1],
                'p' => [2, 1],
                'q' => [3, 1],
                'r' => [4, 1],
                's' => [5, 1],
                't' => [6, 1],
                'u' => [7, 1],
                'v' => [8, 1],
                'w' => [9, 1],
                'x' => [10, 1],
                'y' => [11, 1],
                'z' => [12, 1],
                '0' => [0, 2],
                '1' => [1, 2],
                '2' => [2, 2],
                '3' => [3, 2],
                '4' => [4, 2],
                '5' => [5, 2],
                '6' => [6, 2],
                '7' => [7, 2],
                '8' => [8, 2],
                '9' => [9, 2],
                '.' => [0, 3],
                ':' => [1, 3],
                ';' => [2, 3],
                ',' => [3, 3],
                '/' => [4, 3],
                '\\' => [5, 3],
                '+' => [6, 3],
                '-' => [7, 3],
                '*' => [8, 3],
                '&' => [0, 4],
                // Weird e thing [1, 4]
                '"' => [2, 4],
                '\'' => [3, 4],
                '(' => [4, 4],
                '[' => [5, 4],
                '{' => [6, 4],
                // Other weird e [7, 4]
                '_' => [8, 4],
                // Weird c thing [9, 4]
                // Weird a [10, 4]
                '}' => [11, 4],
                ']' => [12, 4],
                ')' => [13, 4],
                '=' => [14, 4],
                '<' => [0, 5],
                '>' => [1, 5],
                '?' => [2, 5],
                '!' => [3, 5],
                // Weird s [4, 5]
                // Weird ua [5, 5]
                '$' => [6, 5],
                // Pounds [7, 5]
                // Degreas [8, 5]
                '@' => [9, 5],
                '#' => [10, 5],
                // ^2 [11, 5]
                _ =>  [0,0]
            };
        }
    };
    
    let left: U4 = U4::new(bind[0]).expect("font out of bounds x pos");
    let right: U4 = U4::new(bind[1]).expect("font out of bounds y pos");
    
    U4x2::new(left, right)
}
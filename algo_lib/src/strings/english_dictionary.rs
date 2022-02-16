use crate::io::input::Input;

pub struct Word {
    pub word: Vec<u8>,
    pub word_freq: u32,
}
///
/// Download dictionary from: https://en.lexipedia.org/
///
pub fn read_english_dict(path: &str) -> Vec<Word> {
    let mut res = vec![];
    let mut input = Input::new_file(path);
    while input.has_more_elements() {
        let line = input.read_line();
        let parts: Vec<_> = line.split(" ").collect();
        let word = parts[0].as_bytes().to_vec();
        // assert_eq!(word.len(), len, "str = {}", vec2str(&word));
        let word_freq = parts[2].parse().unwrap();
        res.push(Word { word, word_freq });
    }
    res
}

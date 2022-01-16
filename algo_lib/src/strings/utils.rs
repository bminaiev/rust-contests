use crate::misc::digits::char_from_digit;

pub trait VecToString {
    fn to_string(self) -> String;
}

impl VecToString for Vec<u8> {
    fn to_string(self) -> String {
        String::from_utf8(self).unwrap()
    }
}

impl VecToString for Vec<i32> {
    fn to_string(self) -> String {
        self.into_iter()
            .map(char_from_digit)
            .collect::<Vec<_>>()
            .to_string()
    }
}

pub fn vec2str(v: &[u8]) -> String {
    String::from_utf8(v.to_vec()).unwrap()
}

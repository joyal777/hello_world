pub struct Master {
    pub secret: String,
}

impl Master {
    pub fn guess(&self, word: String) -> i32 {
        word.bytes()
            .zip(self.secret.bytes())
            .filter(|(b1, b2)| b1 == b2)
            .count() as i32
    }
}
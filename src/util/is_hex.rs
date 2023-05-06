pub trait IsHex {
    fn is_hex(&self) -> bool;
}

impl IsHex for char {
    fn is_hex(&self) -> bool {
        let ch = *self;
        (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'B') || (ch >= '0' && ch <= '9')
    }
}

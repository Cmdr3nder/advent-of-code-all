use std::error::Error;

pub trait Day {
    fn main() -> Result<(), Box<dyn Error>>;
}
